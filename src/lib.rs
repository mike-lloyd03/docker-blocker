use anyhow::{bail, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::net::Ipv4Addr;

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
    pub allow: Vec<String>,
}

pub fn load_config() -> Result<Config> {
    use std::fs::File;

    let config_path = "config.yaml";
    let f = File::open(config_path)?;
    if let Ok(c) = serde_yaml::from_reader::<_, Config>(f) {
        // Validate known_devices
        if c.known_devices.is_some() {
            for (_, v) in c.known_devices.as_ref().unwrap().iter() {
                if v.parse::<Ipv4Addr>().is_err() {
                    bail!("Unable to parse {} as an IPv4 address", v)
                }
            }
        }

        // Validate allow devices
        for r in &c.rules {
            for device in &r.allow {
                if device.parse::<Ipv4Addr>().is_ok()
                    || (c.known_devices.is_some()
                        && c.known_devices.as_ref().unwrap().contains_key(device))
                    || device == "any"
                {
                    continue;
                }
                bail!(
                    "Unable to parse '{}' for service rule: {}",
                    device,
                    r.service
                )
            }
        }
        return validate_devices(c);
    }

    bail!("No configuration file was found")
}

fn validate_devices(mut config: Config) -> Result<Config> {
    for mut r in config.rules.iter_mut() {
        let devices: Result<Vec<String>> = r
            .allow
            .iter()
            .map(|device| {
                if device.parse::<Ipv4Addr>().is_ok() {
                    Ok(device.to_string())
                } else if config.known_devices.is_some()
                    && config.known_devices.as_ref().unwrap().contains_key(device)
                {
                    Ok(config
                        .known_devices
                        .as_ref()
                        .unwrap()
                        .get(device)
                        .unwrap()
                        .to_string())
                } else if device == "any" {
                    Ok("0.0.0.0/0".to_string())
                } else {
                    bail!(
                        "Unable to parse '{}' for service rule: {}",
                        device,
                        r.service
                    )
                }
            })
            .collect();
        r.allow = devices?
        // for mut device in r.allow {
        //     if device.parse::<Ipv4Addr>().is_ok() {
        //         continue;
        //     } else if config.known_devices.is_some()
        //         && config.known_devices.as_ref().unwrap().contains_key(&device)
        //     {
        //         device = *config.known_devices.as_ref().unwrap().get(&device).unwrap()
        //         // continue;
        //     } else if device == "any" {
        //         device = "0.0.0.0/0".to_string();
        //         continue;
        //     } else {
        //         bail!(
        //             "Unable to parse '{}' for service rule: {}",
        //             device,
        //             r.service
        //         )
        //     }
        // }
    }
    Ok(config)
}
