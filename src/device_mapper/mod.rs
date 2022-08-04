pub mod dmsetup_interface;
pub mod interface;
pub mod ioctl;
pub mod logger;

use self::interface::*;
use self::ioctl::*;
use self::logger::*;
use anyhow::Result;

pub fn create_interface() -> Result<Box<dyn Interface>> {
    let inner = Ioctl::new()?;
    Ok(Box::new(Logger::new(inner)))
}
