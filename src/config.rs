use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};
use std::path::Path;

//-----------------------------------------

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub test_volumes: Vec<String>,
}

pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Config> {
    let input = std::fs::read_to_string(path).context("couldn't read config file")?;
    let cfg: Config = toml::from_str(&input).context("couldn't parse config file")?;
    Ok(cfg)
}

//-----------------------------------------
