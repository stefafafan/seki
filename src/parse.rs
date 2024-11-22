use clap::Parser;
use std::{
    fs,
    io::{self, BufRead},
};

use crate::{
    aggregate::{compile_groupings, LogEntry},
    config::{self, Config},
};

/// A simple log aggregator that reads logs from stdin and outputs aggregated logs in JSON format.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to the config file
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

pub fn parse_config_and_logs() -> (Vec<config::Grouping>, Vec<LogEntry>) {
    let args = Args::parse();
    let groupings = match fs::metadata(&args.config).is_ok() {
        true => {
            let config_content = fs::read_to_string(args.config).unwrap();
            let mut config: Config = toml::from_str(&config_content).unwrap();
            compile_groupings(&mut config.grouping);
            config.grouping
        }
        false => Vec::new(),
    };

    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut logs: Vec<LogEntry> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let entry: LogEntry = serde_json::from_str(&line).unwrap();
        logs.push(entry);
    }
    (groupings, logs)
}
