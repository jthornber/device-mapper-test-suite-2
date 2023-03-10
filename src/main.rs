use anyhow::Result;

use dmtest::config;
use dmtest::suites::thin_provisioning;
use dmtest::test_runner::*;

//-------------------------------

fn register_tests(runner: &mut TestRunner) -> Result<()> {
    thin_provisioning::register_tests(runner)?;
    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();

    let config = config::read_config("config.toml")?;
    let mut runner = TestRunner::new(config);
    register_tests(&mut runner)?;
    runner.exec()?;

    Ok(())
}

//--------------------------------
