extern crate clap;

use clap::{crate_authors, crate_version, App, Arg};
use log::info;

use crate::config::Config;

pub fn run() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
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
    println!("{:?}", config);
}
