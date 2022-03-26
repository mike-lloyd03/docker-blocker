use anyhow::{bail, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub known_devices: Option<HashMap<String, String>>,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct Device {
    pub name: String,
    pub ip: String,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub service: String,
    pub port: u16,
    pub allow_from: Vec<String>,
}

pub fn load_config() -> Result<Config> {
    use std::fs::File;
    use std::path::Path;

    let config_paths = vec!["config.yaml"];
    let f: File;

    for path in config_paths {
        if Path::new(path).exists() {
            f = File::open(path)?;
            return Ok(serde_yaml::from_reader(f)?);
        }
    }
    bail!("No configuration file was found")
}
