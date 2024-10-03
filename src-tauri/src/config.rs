use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    host: String,
    token: String,
}

pub fn read_config() -> Config {
    // TODO: stub
    Config {
        host: "asdf".to_owned(),
        token: "qqq".to_owned(),
    }
}

pub fn write_config(config: &Config) {
    info!("{:?}", config);
    // TODO: stub
}
