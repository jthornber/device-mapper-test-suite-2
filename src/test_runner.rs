use anyhow::Result;
use log::*;
use regex::Regex;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

use crate::config::*;
use crate::device_mapper::interface::*;
use crate::device_mapper::*;
use crate::fixture::*;
use crate::segment::*;

//-------------------------------

fn path_components(path: &str) -> Vec<String> {
    path.trim_start_matches('/')
        .split('/')
        .map(|s| s.to_string())
        .collect()
}

struct PathFormatter {
    last_path: Vec<String>,
}

impl PathFormatter {
    fn new() -> Self {
        PathFormatter {
            last_path: Vec::new(),
        }
    }

    fn indent(&self, count: usize) {
        let mut space = String::new();
        for _ in 0..count {
            space.push_str("  ");
        }
        print!("{}", space);
    }

    fn dots(&self, count: usize) {
        let mut space = String::new();
        for _ in 0..count {
            space.push('.');
        }
        print!("{}", space);
    }

    fn print(&mut self, components: &[String]) {
        let mut last_path = Vec::new();
        let mut common = true;
        let mut width = 0;
        for (index, c) in components.iter().enumerate() {
            let last = self.last_path.get(index);
            if last.is_none() || last.unwrap() != c {
                common = false;
            }

            if !common {
                self.indent(index);
                if index == components.len() - 1 {
                    print!("{} ", c);
                } else {
                    println!("{} ", c);
                }
            }

            last_path.push(c.clone());
            width = (index * 2) + c.len();
        }
        self.dots(60 - width);

        // Inefficient, but I don't think it will be significant.
        self.last_path = last_path;
    }
}

//--------------------------------

type StorageReq = Vec<(Tags, u64)>;

//--------------------------------

trait TestFn_ = FnOnce(&mut Fixture) -> Result<()> + Send + 'static;
pub type TestFn = Box<dyn TestFn_>;

pub struct Test {
    func: TestFn,
    storage: StorageReq,
}

impl Test {
    pub fn new(func: TestFn, storage: StorageReq) -> Self {
        Test {
            func: Box::new(func),
            storage,
        }
    }
}

/// Returns sectors
fn get_dev_size(vol: &str) -> u64 {
    1024
}

#[allow(dead_code)]
pub struct TestRunner<'a> {
    config: Config,
    filter_fn: Box<dyn Fn(&str) -> bool + 'a>,
    tests: BTreeMap<String, Test>,
    jobs: usize,
}

/// Wraps a test so we can run it in a thread.
fn run_test(mut fix: Fixture, t: Test) -> Result<()> {
    (t.func)(&mut fix)
}

impl<'a> TestRunner<'a> {
    pub fn new(config: Config) -> Self {
        let filter_fn = Box::new(move |_: &str| true);

        TestRunner {
            config,
            filter_fn,
            tests: BTreeMap::new(),
            jobs: 1,
        }
    }

    pub fn set_filter(&mut self, filter: Regex) {
        self.filter_fn = Box::new(move |p| filter.is_match(p));
    }

    pub fn set_jobs(&mut self, jobs: usize) {
        self.jobs = jobs;
    }

    pub fn register(&mut self, path: &str, t: Test) {
        self.tests.insert(path.to_string(), t);
    }

    pub fn exec(self) -> Result<(usize, usize)> {
        let mut pass = 0;
        let mut fail = 0;
        let mut formatter = PathFormatter::new();

        let pool = ThreadPool::new(self.jobs);

        let results: Arc<Mutex<BTreeMap<String, Result<()>>>> =
            Arc::new(Mutex::new(BTreeMap::new()));

        for (p, t) in self.tests {
            if !(*self.filter_fn)(&p) {
                continue;
            }

            let results = results.clone();
            let dm = create_interface()?;
            let mut allocator = Allocator::default();
            for vol in &self.config.test_volumes {
                allocator.add_allocation_seg(Segment {
                    dev: dev_from_path(vol)?,
                    b_sector: 0,
                    e_sector: get_dev_size(&vol),
                    tags: Tags::empty(),
                });
            }
            let fix = Fixture::new(dm, allocator);

            // pool.execute(move || {
            let res = run_test(fix, t);

            let mut results = results.lock().unwrap();
            results.insert(p.clone(), res);
            drop(results);
            // });
        }

        pool.join();

        let results = Arc::try_unwrap(results).unwrap().into_inner()?;

        for (p, res) in results.into_iter() {
            let components = path_components(&p);
            formatter.print(&components);

            match res {
                Err(e) => {
                    fail += 1;
                    println!(" FAIL");
                    info!("{}", e);
                }
                Ok(()) => {
                    pass += 1;
                    println!(" PASS");
                }
            }
        }

        Ok((pass, fail))
    }
}

//-------------------------------
