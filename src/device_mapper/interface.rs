use anyhow::{anyhow, Result};
use nix::libc::dev_t;
use std::fmt::{self, Display};
use std::fs::metadata;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

//-------------------------------

pub use devicemapper::{Device, DmName, DmNameBuf};

pub fn dev_from_path<P: AsRef<Path>>(path: P) -> Result<Device> {
    let info = metadata(&path)?;
    let id = info.rdev();

    // the rdev method can't fail, so I'm assuming they're indicating error
    // by returning zero.
    if id == 0 {
        return Err(anyhow!(format!(
            "'{:?}' is not a device",
            path.as_ref().display()
        )));
    }

    Ok(Device::from(id as dev_t))
}

pub fn dev_from_name(name: &DmName) -> Result<Device> {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let osstr = OsStr::from_bytes(name.as_bytes());
    let mut path = PathBuf::new(); // FIXME: use from()?
    path.push("/dev/mapper");
    path.push(&osstr);
    dev_from_path(path)
}

/// A target is any type that has a type, length and an argument string
pub trait Target {
    fn target_type(&self) -> &'static str;
    fn sectors(&self) -> u64;
    fn args(&self) -> String;
}

//-------------------------------

#[derive(Debug)]
pub struct Linear {
    pub dev: Device,
    pub offset: u64,
    pub sectors: u64,
}

impl Linear {
    fn from_target_line(_sectors: u64, _args: &str) -> Result<Self> {
        todo!();
    }
}

impl Target for Linear {
    fn target_type(&self) -> &'static str {
        "linear"
    }

    fn sectors(&self) -> u64 {
        self.sectors
    }

    fn args(&self) -> String {
        format!("{}:{} {}", self.dev.major, self.dev.minor, self.offset)
    }
}

//-------------------------------

pub fn from_target_line(sectors: u64, ttype: &str, args: &str) -> Result<Box<dyn Target>> {
    match ttype {
        "linear" => Ok(Box::new(Linear::from_target_line(sectors, args)?)),
        _ => Err(anyhow!(format!("unknown target type '{}'", ttype))),
    }
}

//-------------------------------

pub struct Table {
    pub targets: Vec<Box<dyn Target>>,
}

impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Table").finish()
    }
}

pub trait Interface {
    fn create(&mut self, id: &DmName) -> Result<()>;

    fn create_anonymous(&mut self) -> Result<DmNameBuf> {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let name = format!("test-{:09}", rng.gen_range(0..1000_000_000));
            let name = DmNameBuf::new(name)?;
            if self.create(&name).is_ok() {
                return Ok(name);
            }
        }

        Err(anyhow!(
            "couldn't think of a unique name for temporary dm device"
        ))
    }

    fn load(&mut self, id: &DmName, table: &Table) -> Result<()>;
    fn suspend(&mut self, id: &DmName, flush: bool) -> Result<()>;
    fn resume(&mut self, id: &DmName) -> Result<()>;
    fn remove(&mut self, id: &DmName) -> Result<()>;
    fn message(&mut self, id: &DmName, sector: Option<u64>, msg: &str) -> Result<()>;
    fn status(&mut self, id: &DmName) -> Result<String>;
    fn table(&mut self, id: &DmName) -> Result<Table>;
    fn wait(&mut self, id: &DmName, event_nr: u64) -> Result<(u64, Vec<String>)>;
}

//-------------------------------
