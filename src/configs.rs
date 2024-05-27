use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub statera: Statera,
    pub servers: Servers,
}

#[derive(Deserialize)]
pub struct Statera {
    pub port: String,
}
#[derive(Deserialize)]
pub struct Servers {
    pub ports: Vec<String>,
}

pub async fn load_config() -> Config {
    let contents = fs::read_to_string("statera.toml").unwrap();

    toml::from_str(&contents).unwrap()
}
