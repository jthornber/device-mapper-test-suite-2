pub mod dmsetup_interface;
pub mod interface;
pub mod ioctl;
pub mod logger;
pub mod scoped;

use self::interface::*;
use self::ioctl::*;
use self::logger::*;
use anyhow::Result;
use std::sync::{Arc, Mutex};

pub fn create_interface() -> Result<Arc<Mutex<dyn Interface>>> {
    let inner = Ioctl::new()?;
    Ok(Arc::new(Mutex::new(Logger::new(inner))))
}
