use anyhow::Result;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::device_mapper::interface::*;

//----------------------------

pub struct ScopedDev {
    pub dm: Arc<Mutex<dyn Interface>>,
    pub name: DmNameBuf,
}

impl Drop for ScopedDev {
    fn drop(&mut self) {
        let mut dm = self.dm.lock().unwrap();

        // FIXME: the interface needs to track errors
        dm.remove(&self.name).expect("dm.remove failed");
    }
}

impl ScopedDev {
    pub fn load(&mut self, table: &Table) -> Result<()> {
        let mut dm = self.dm.lock().unwrap();
        dm.load(&self.name, table)
    }

    pub fn suspend(&mut self, flush: bool) -> Result<()> {
        let mut dm = self.dm.lock().unwrap();
        dm.suspend(&self.name, flush)
    }

    pub fn resume(&mut self) -> Result<()> {
        let mut dm = self.dm.lock().unwrap();
        dm.resume(&self.name)
    }

    pub fn remove(&mut self) -> Result<()> {
        let mut dm = self.dm.lock().unwrap();
        dm.remove(&self.name)
    }

    pub fn message(&mut self, sector: Option<u64>, msg: &str) -> Result<()> {
        let mut dm = self.dm.lock().unwrap();
        dm.message(&self.name, sector, msg)
    }

    pub fn status(&mut self) -> Result<String> {
        let mut dm = self.dm.lock().unwrap();
        dm.status(&self.name)
    }

    pub fn table(&mut self) -> Result<Table> {
        let mut dm = self.dm.lock().unwrap();
        dm.table(&self.name)
    }

    pub fn wait(&mut self, event_nr: u64) -> Result<(u64, Vec<String>)> {
        let mut dm = self.dm.lock().unwrap();
        dm.wait(&self.name, event_nr)
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

// FIXME: move to ScopedDev
pub fn dev_to_ostr(dev: &ScopedDev) -> OsString {
    let mut s = OsString::new();
    s.push(std::str::from_utf8(dev.name.as_bytes()).expect("dm dev name is not utf8"));
    s
}

//----------------------------
