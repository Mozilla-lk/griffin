use regex::{Regex, RegexBuilder};
use serde::{de::Error, Deserialize, Deserializer};
use std::{
    fmt::{self, Display},
    fs,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

#[derive(Debug, PartialEq, Clone, Copy)]
/// TimeUnit represents time duration's unit in hours, minutes, seconds, milliseconds
pub enum TimeUnit {
    Hours,
    Minutes,
    Seconds,
}

#[derive(Debug, Clone)]
/// Error when time unit is not valid
pub struct TimeUnitError {
    /// error message
    message: String,
}

impl Display for TimeUnitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid duration {}", self.message)
    }
}

impl FromStr for TimeUnit {
    type Err = TimeUnitError;

    /// Convert a string to TimeUnit
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "s" => Ok(TimeUnit::Seconds),
            "min" => Ok(TimeUnit::Minutes),
            "h" => Ok(TimeUnit::Hours),
            _ => Err(TimeUnitError {
                message: s.to_owned(),
            }),
        }
    }
}

/// Represents a time interval with a value and an unit
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Interval {
    pub value: u32,
    pub unit: TimeUnit,
}

impl Interval {
    /// Creates a new Interval from a value and TimeUnit
    pub fn new(value: u32, unit: TimeUnit) -> Self {
        Self { value, unit }
    }
}

impl From<Interval> for clokwerk::Interval {
    fn from(interval: Interval) -> Self {
        match interval.unit {
            TimeUnit::Hours => clokwerk::Interval::Hours(interval.value),
            TimeUnit::Minutes => clokwerk::Interval::Minutes(interval.value),
            TimeUnit::Seconds => clokwerk::Interval::Seconds(interval.value),
        }
    }
}

impl Default for Interval {
    /// Default time interval is 30s
    fn default() -> Self {
        Interval {
            value: 30,
            unit: TimeUnit::Seconds,
        }
    }
}

/// Health check config
#[derive(Clone, Debug, Deserialize)]
pub struct HealthCheckConfig {
    #[serde(default)]
    #[serde(deserialize_with = "interval_from_str")]
    /// Interval to check health
    pub interval: Interval,
}

/// Upstream remote
#[derive(Debug, Deserialize, Clone)]
pub struct Remote {
    /// Name of the upstream
    pub name: Option<String>,

    /// URL of the upstream
    pub url: String,

    /// Health check options
    pub health: Option<HealthCheckConfig>,
}

#[derive(Debug, Deserialize)]
/// Configuration
pub struct Config {
    /// remotes to check
    pub remotes: Vec<Remote>,
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Serde(serde_yaml::Error),
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(v: serde_yaml::Error) -> Self {
        ConfigError::Serde(v)
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(v: std::io::Error) -> Self {
        ConfigError::Io(v)
    }
}

lazy_static! {
    /// Regex expression to match time durations in string format
    static ref RE: Regex = RegexBuilder::new(r"^(\d+)(h|min|s)$")
        .case_insensitive(true)
        .build()
        .unwrap();
}

/// Get Interval from serde
fn interval_from_str<'de, D>(deserializer: D) -> Result<Interval, D::Error>
where
    D: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;
    match RE.captures(&str) {
        Some(captures) => {
            let val = match captures.get(1) {
                Some(v) => v.as_str().parse::<u32>().unwrap(),
                None => 0,
            };
            if val <= 0 {
                return Err(D::Error::custom("duration must be 0 or greater"));
            }
            let dur = captures.get(2).map_or("", |m| m.as_str());
            let tu = TimeUnit::from_str(dur).map_err(|e| D::Error::custom(e.to_string()))?;
            return Ok(Interval::new(val, tu));
        }
        None => return Err(D::Error::custom(format!("{} is invalid duration", &str))),
    }
}

impl Config {
    /// creates a new config from a file
    pub fn new_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        Self::new(reader)
    }

    /// creates a new config from a reader
    pub fn new<R: Read>(rdr: R) -> Result<Self, ConfigError> {
        let config: Config = serde_yaml::from_reader(rdr)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_correct_config() {
        let config = r###"
            remotes:
              - name: Foo Bar
                url: https://foo.bar
                health:
                  interval: 5min
        "###;

        match Config::new(config.as_bytes()) {
            Ok(config) => {
                assert_eq!(config.remotes.len(), 1);
                let remote = &config.remotes[0];

                assert_eq!(remote.name.as_deref(), Some("Foo Bar"));
                assert_eq!(remote.url, "https://foo.bar");
                assert_eq!(
                    remote.health.as_ref().unwrap().interval,
                    Interval::new(5, TimeUnit::Minutes)
                );
            }
            Err(e) => {
                assert!(false, "Error parsing config {:?}", e);
            }
        }
    }

    #[test]
    fn fail_on_invalid_config() {
        let config = r###"
            backends:
        "###;

        match Config::new(config.as_bytes()) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}
