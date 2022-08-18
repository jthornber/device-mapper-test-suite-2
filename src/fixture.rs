use std::sync::{Arc, Mutex};

use crate::device_mapper::interface::*;
use crate::segment;

//---------------------------------

// artifact directory, chdir for duration of test
// create temp files
// record log file
// shared volume manager so we can run multiple tests at once
// archive artifacts to database after test
// destroy artifact dir
pub struct Fixture {
    pub dm: Arc<Mutex<dyn Interface>>,
    pub storage: segment::Allocator,
}

impl Fixture {
    pub fn new(dm: Arc<Mutex<dyn Interface>>, storage: segment::Allocator) -> Self {
        Self { dm, storage }
    }
}

//---------------------------------
