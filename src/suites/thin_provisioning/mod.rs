use anyhow::Result;
use scopeguard::defer;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::device_mapper::interface::*;
use crate::device_mapper::scoped::*;
use crate::device_mapper::*;
use crate::fixture::*;
use crate::segment::*;
use crate::test_runner::*;

//--------------------------------

fn mk_linear(s: &Segment) -> Box<dyn Target> {
    Box::new(Linear {
        dev: s.dev,
        offset: s.b_sector,
        sectors: s.len(),
    })
}

fn linear_create_remove(fix: &mut Fixture) -> Result<()> {
    let segs = fix.storage.alloc(1024)?;
    let targets: Vec<Box<dyn Target>> = segs.iter().map(mk_linear).collect();
    let table = Table { targets };

    {
        let _dev = scoped_dev(fix.dm.clone(), &table)?;
        // run mkfs :)
    }

    Ok(())
}

pub fn register_tests(runner: &mut TestRunner) -> Result<()> {
    let test = Test::new(Box::new(linear_create_remove), vec![]);
    runner.register("/linear/create-remove", test);
    Ok(())
}

//--------------------------------
