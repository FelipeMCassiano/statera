use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub port: String,
    pub health_check: Option<HealthCheck>,
    pub servers: Vec<Server>,
}

#[derive(Deserialize)]
pub struct Server {
    pub name: String,
    pub host: String,
    pub port: String,
}

#[derive(Deserialize, Debug)]
pub struct HealthCheck {
    pub interval: u64,
    pub endpoint: String,
    pub max_failures: Option<u16>,
}

pub async fn load_config() -> Config {
    let contents = fs::read_to_string("statera.toml").unwrap();

    toml::from_str(&contents).unwrap()
}
