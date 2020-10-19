use regex::{Regex, RegexBuilder};
use serde::{de::Error, Deserialize, Deserializer};
use std::fmt;
use std::{error, fmt::Display, fs, io::BufReader, path::Path, str::FromStr};

#[derive(Debug)]
pub enum TimeUnit {
    Hours,
    Minutes,
    Seconds,
    Milliseconds,
}

#[derive(Debug, Clone)]
pub struct TimeUnitError {
    str: String,
}

impl Display for TimeUnitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid duration {}", self.str)
    }
}

impl FromStr for TimeUnit {
    type Err = TimeUnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "ms" => Ok(TimeUnit::Milliseconds),
            "s" => Ok(TimeUnit::Seconds),
            "min" => Ok(TimeUnit::Minutes),
            "h" => Ok(TimeUnit::Hours),
            _ => Err(TimeUnitError { str: s.to_owned() }),
        }
    }
}

#[derive(Debug)]
pub struct Interval {
    pub value: u32,
    pub unit: TimeUnit,
}

impl Interval {
    pub fn new(value: u32, unit: TimeUnit) -> Self {
        Self { value, unit }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            value: 30,
            unit: TimeUnit::Seconds,
        }
    }
}

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
    #[serde(deserialize_with = "duration_from_str")]
    pub interval: Interval,
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

lazy_static! {
    static ref RE: Regex = RegexBuilder::new(r"(\d+)(h|min|s|ms)")
        .case_insensitive(true)
        .build()
        .unwrap();
}

fn duration_from_str<'de, D>(deserializer: D) -> Result<Interval, D::Error>
where
    D: Deserializer<'de>,
{
    //println!("on dezer");
    let str = String::deserialize(deserializer)?;
    match RE.captures(&str) {
        Some(captures) => {
            let val = match captures.get(1) {
                Some(v) => v.as_str().parse::<u32>().unwrap(),
                None => 0,
            };
            if val == 0 {
                return Err(D::Error::custom("duration must be 0 or greater"));
            }
            let dur = captures.get(2).map_or("", |m| m.as_str());
            let tu = TimeUnit::from_str(dur).map_err(|e| D::Error::custom(e.to_string()))?;
            return Ok(Interval::new(val, tu));
        }
        None => return Err(D::Error::custom(format!("{} is invalid duration", &str))),
    };
    // Ok(Interval::new(10, TimeUnit::Hours))
}

impl Config {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn error::Error>> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let config: Config = serde_yaml::from_reader(reader).unwrap();
        Ok(config)
    }
}
