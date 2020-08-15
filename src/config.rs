use serde::Deserialize;
use std::{error, fs, io::BufReader, path::Path};

#[derive(Debug, Deserialize)]
pub enum HealthCheckMethod {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "ping")]
    Ping,
}

#[derive(Debug, Deserialize)]
pub struct HealthCheck {
    pub method: HealthCheckMethod,
    pub endpoint: Option<String>,
    pub interval: Option<String>,
    pub port: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Backend {
    pub name: String,
    pub host: String,
    pub health: Vec<HealthCheck>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub backends: Vec<Backend>,
}

impl Config {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn error::Error>> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let config: Config = serde_yaml::from_reader(reader).unwrap();
        Ok(config)
    }
}
