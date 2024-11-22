mod aggregate;
mod config;
mod parse;

use aggregate::aggregate_logs;
use clap::Parser;
use parse::{parse_config, parse_logs};

/// A simple log aggregator that reads logs from stdin and outputs aggregated logs in JSON format.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to the config file
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

fn main() {
    let args = Args::parse();
    let groupings = parse_config(&args.config);
    let logs = parse_logs();
    let aggregated_logs = aggregate_logs(&logs, &groupings);
    let json_output = serde_json::to_string_pretty(&aggregated_logs).unwrap();
    println!("{}", json_output);
}
