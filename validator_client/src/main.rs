mod attester_service;
mod block_producer_service;
mod config;
mod duties;
pub mod error;
mod service;

use crate::config::Config as ValidatorConfig;
use clap::{App, Arg};
use service::Service as ValidatorService;
use slog::{error, info, o, Drain};

fn main() {
    // Logging
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!());

    // CLI
    let matches = App::new("Lighthouse Validator Client")
        .version("0.0.1")
        .author("Sigma Prime <contact@sigmaprime.io>")
        .about("Eth 2.0 Validator Client")
        .arg(
            Arg::with_name("datadir")
                .long("datadir")
                .value_name("DIR")
                .help("Data directory for keys and databases.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("server")
                .long("server")
                .value_name("server")
                .help("Address to connect to BeaconNode.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("spec")
                .long("spec")
                .value_name("spec")
                .short("s")
                .help("Configuration of Beacon Chain")
                .takes_value(true)
                .possible_values(&["foundation", "few_validators"])
                .default_value("foundation"),
        )
        .get_matches();

    let config = ValidatorConfig::parse_args(matches, &log).unwrap();

    // start the validator service.
    match ValidatorService::start(config, log.clone()) {
        Ok(_) => info!(log, "Validator client shutdown successfully."),
        Err(e) => error!(log, "Validator exited due to {:?}", e),
    }
}
