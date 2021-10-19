use core::time;

use chrono::{DateTime, Local};

pub struct HealthCheckResult {
    status_code: u8,
    response: String,
    time: DateTime<Local>,
}

pub fn check_url(url: &str) {}
