use anyhow::Result;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::device_mapper::interface::*;

//----------------------------

pub struct ScopedDev {
    dm: Arc<Mutex<dyn Interface>>,
    name: DmNameBuf,
}

impl Drop for ScopedDev {
    fn drop(&mut self) {
        let mut dm = self.dm.lock().unwrap();

        // FIXME: the interface needs to track errors
        dm.remove(&self.name);
    }
}

impl ScopedDev {
    fn load(&mut self, id: &DmName, table: &Table) -> Result<()> {
        todo!();
    }

    fn suspend(&mut self, id: &DmName, flush: bool) -> Result<()> {
        todo!();
    }

    fn resume(&mut self, id: &DmName) -> Result<()> {
        todo!();
    }

    fn remove(&mut self, id: &DmName) -> Result<()> {
        todo!();
    }

    fn message(&mut self, id: &DmName, sector: Option<u64>, msg: &str) -> Result<()> {
        todo!();
    }

    fn status(&mut self, id: &DmName) -> Result<String> {
        todo!();
    }

    fn table(&mut self, id: &DmName) -> Result<Table> {
        todo!();
    }

    fn wait(&mut self, id: &DmName, event_nr: u64) -> Result<(u64, Vec<String>)> {
        todo!();
    }
}

//----------------------------

pub fn scoped_dev(dm: Arc<Mutex<dyn Interface>>, table: &Table) -> Result<ScopedDev> {
    let dm_cloned = dm.clone();

    let mut dm = dm.lock().unwrap();
    let name = dm.create_anonymous()?;

    dm.load(&name, table)?;
    dm.resume(&name)?;
    let mut path = PathBuf::new();
    path.push("/dev/mapper/");
    path.push(&name.to_string());

    Ok(ScopedDev {
        dm: dm_cloned,
        name,
    })
}

//----------------------------
