use anyhow::Result;
use log::*;

use crate::device_mapper::interface::*;

//----------------------------------

pub struct Logger<I: Interface> {
    inner: I,
}

impl<I: Interface> Logger<I> {
    pub fn new(inner: I) -> Self {
        Self { inner }
    }
}

fn log_result<T>(v: Result<T>) -> Result<T> {
    v
}

impl<I: Interface> Interface for Logger<I> {
    fn create(&mut self, id: &DmName) -> Result<()> {
        debug!("create {}", id);
        log_result(self.inner.create(id))
    }

    fn load(&mut self, id: &DmName, table: &Table) -> Result<()> {
        debug!("load {}", id);
        self.inner.load(id, table)
    }

    fn suspend(&mut self, id: &DmName, flush: bool) -> Result<()> {
        debug!("suspend {}", id);
        self.inner.suspend(id, flush)
    }

    fn resume(&mut self, id: &DmName) -> Result<()> {
        debug!("resume {}", id);
        self.inner.resume(id)
    }

    fn remove(&mut self, id: &DmName) -> Result<()> {
        debug!("remove {}", id);
        self.inner.remove(id)
    }

    fn message(&mut self, id: &DmName, sector: Option<u64>, msg: &str) -> Result<()> {
        debug!("message {}", id);
        self.inner.message(id, sector, msg)
    }

    fn status(&mut self, id: &DmName) -> Result<String> {
        debug!("status {}", id);
        self.inner.status(id)
    }

    fn table(&mut self, id: &DmName) -> Result<Table> {
        debug!("table {}", id);
        self.inner.table(id)
    }

    fn wait(&mut self, id: &DmName, event_nr: u64) -> Result<(u64, Vec<String>)> {
        debug!("wait {}", id);
        self.inner.wait(id, event_nr)
    }
}

//----------------------------------
