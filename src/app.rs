extern crate clap;

use clap::{crate_authors, crate_version, App, Arg};
use log::info;

use crate::config::Config;

// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::Scheduler;
// Import week days and WeekDay

use std::thread;
use std::time::Duration;

pub fn run() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_env("GRIFFIN_LOG")
        .init();

    let matches = App::new("Griffin")
        .version(crate_version!())
        .author(crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Path to griffin config file")
                .default_value("griffin.yaml")
                .takes_value(true),
        )
        .get_matches();

    let config_path = matches.value_of("config").unwrap();
    info!("Loading config from {}", config_path);
    let config = Config::new_from_file(config_path).unwrap();

    let mut scheduler = Scheduler::new();

    for remote in &config.remotes {
        // for h in &remote.health {
        //     let interval = h.interval;
        //     let remote = remote.clone();
        //     scheduler
        //         .every(Interval::from(interval))
        //         .run(move || println!("{:?}", remote));
        // }
    }

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(10));
    }
}
