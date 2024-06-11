use std::{fs, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub port: String,
    pub health_check: Option<HealthCheck>,
    pub servers: Vec<Server>,
    pub ssl: Option<Ssl>,
}

#[derive(Deserialize)]
pub struct Server {
    pub name: String,
    pub host: String,
    pub port: String,
}

#[derive(Deserialize)]
pub struct HealthCheck {
    pub interval: u64,
    pub endpoint: String,
    pub max_failures: Option<u16>,
}

#[derive(Deserialize)]
pub struct Ssl {
    pub certificate: String,
    pub key: String,
}

pub async fn load_config() -> Config {
    let contents = fs::read_to_string("statera.toml").unwrap();

    toml::from_str(&contents).unwrap()
}

pub fn find_files_paths(path: &PathBuf, file_name: String) -> Option<PathBuf> {
    if path.is_dir() {
        for entry in fs::read_dir(path).expect("read_dir call failed") {
            let entry = entry.expect("read_dir call failed");
            let path = entry.path();
            if path.is_dir() {
                if let Some(found) = find_files_paths(&path, file_name.clone()) {
                    return Some(found);
                }
            } else if path.is_file() {
                if let Some(name) = path.file_name() {
                    if name.to_str() == Some(file_name.as_str()) {
                        return Some(path);
                    }
                }
            }
        }
    }
    None
}
