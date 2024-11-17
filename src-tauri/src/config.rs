use anyhow::{anyhow, Result};
use dirs::config_local_dir;
use serde::{Deserialize, Serialize};
use std::{fs::{create_dir, read_to_string}, path::PathBuf};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Config {
    host: String,
    token: String,
}

fn config_file() -> Result<PathBuf> {
    let mut config_file =
        config_local_dir().ok_or_else(|| anyhow!("Can't access a configuration directory"))?;
    config_file.extend(["gitlab-variables-explorer", "config.toml"]);
    Ok(config_file)
}

pub fn read_config() -> Result<Config> {
    Ok(toml::from_str(&read_to_string(config_file()?)?)?)
}

pub fn write_config(config: &Config) -> Result<()> {
    let config_file = config_file()?;
    create_dir(config_file.parent().ok_or_else(|| anyhow!("Invalid configuration directory"))?).unwrap_or_default();
    std::fs::write(config_file, toml::to_string(config)?)?;
    Ok(())
}
