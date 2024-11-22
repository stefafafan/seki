mod aggregate;
mod config;
mod parse;

use aggregate::aggregate_logs;
use parse::parse_config_and_logs;

fn main() {
    let (groupings, logs) = parse_config_and_logs();
    let aggregated_logs = aggregate_logs(&logs, &groupings);
    let json_output = serde_json::to_string_pretty(&aggregated_logs).unwrap();
    println!("{}", json_output);
}
