use crate::{
    aggregate::{compile_groupings, LogEntry},
    config::{self, Config},
};
use std::{
    fs,
    io::{self, BufRead},
};

pub fn parse_config(config_path: &str) -> Vec<config::Grouping> {
    match fs::metadata(config_path).is_ok() {
        true => {
            let config_content = fs::read_to_string(config_path).unwrap();
            let mut config: Config = toml::from_str(&config_content).unwrap();
            compile_groupings(&mut config.grouping);
            config.grouping
        }
        false => Vec::new(),
    }
}

pub fn parse_logs() -> Vec<LogEntry> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut logs: Vec<LogEntry> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let entry: LogEntry = serde_json::from_str(&line).unwrap();
        logs.push(entry);
    }
    logs
}
