use serde::Deserialize;
use std::{error, fs, io::BufReader, path::Path};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum HealthCheckKind {
    Http { method: String, endpoint: String },
    Ping { method: String },
}

#[derive(Debug, Deserialize)]
pub struct HealthCheck {
    #[serde(flatten)]
    pub kind: HealthCheckKind,
    pub interval: String,
    #[serde(default = "default_port")]
    pub port: u32,
}

#[derive(Debug, Deserialize)]
pub struct Backend {
    pub name: String,
    pub host: String,
    pub health: Vec<HealthCheck>,
}

fn default_port() -> u32 {
    80
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
