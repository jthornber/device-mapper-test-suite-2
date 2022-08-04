use anyhow::Result;
use std::path::Path;

use crate::device_mapper::interface::*;
use crate::device_mapper::*;
use crate::test_runner::*;

fn linear_create_remove(_fix: &mut Fixture) -> Result<()> {
    let mut dm = create_interface()?;

    let dev = DmNameBuf::new("test-1".to_string())?;
    dm.create(&dev)?;

    let vda = dev_from_path(Path::new("/dev/vda"))?;
    let targets: Vec<Box<dyn Target>> = vec![Box::new(Linear {
        dev: vda,
        offset: 0,
        sectors: 1024,
    })];
    let table = Table { targets };
    dm.load(&dev, &table)?;
    dm.resume(&dev)?;

    // run mkfs :)

    dm.remove(&dev)?;
    Ok(())
}

pub fn register_tests(runner: &mut TestRunner) -> Result<()> {
    let test = Test::new(Box::new(linear_create_remove));
    runner.register("/linear/create-remove", test);
    Ok(())
}
