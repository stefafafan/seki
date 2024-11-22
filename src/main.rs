mod aggregate;
mod config;

use crate::config::Config;
use aggregate::{aggregate_logs, compile_groupings, LogEntry};

use clap::Parser;
use std::{
    fs,
    io::{self, BufRead},
};

/// A simple log aggregator that reads logs from stdin and outputs aggregated logs in JSON format.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to the config file
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

fn parse_config_and_logs(args: Args) -> (Vec<config::Grouping>, Vec<LogEntry>) {
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

fn main() {
    let args = Args::parse();
    let (groupings, logs) = parse_config_and_logs(args);
    let aggregated_logs = aggregate_logs(logs, &groupings);
    let json_output = serde_json::to_string_pretty(&aggregated_logs).unwrap();
    println!("{}", json_output);
}
