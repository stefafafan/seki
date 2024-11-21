use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, BufRead},
};

/// A simple log aggregator that reads logs from stdin and outputs aggregated logs in JSON format.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to the config file
    #[arg(short, long)]
    config: Option<String>,
}

#[derive(Deserialize, Debug)]
struct LogEntry {
    method: String,
    uri: String,
    status: String,
    response_time: String,
}

#[derive(Default, Debug, Serialize)]
struct AggregatedLogEntry {
    method: String,
    uri: String,
    count: u64,
    status_code: StatusCode,
    response_time: ResponseTime,
}

#[derive(Default, Debug, Copy, Clone, Serialize)]
struct StatusCode {
    status_1xx: u64,
    status_2xx: u64,
    status_3xx: u64,
    status_4xx: u64,
    status_5xx: u64,
}

#[derive(Default, Debug, Serialize)]
struct ResponseTime {
    min: f64,
    max: f64,
    avg: f64,
    sum: f64,
    p50: f64,
    p75: f64,
    p90: f64,
    p95: f64,
    p99: f64,
}

#[derive(Debug, Deserialize)]
struct Config {
    grouping: Vec<Grouping>,
}

#[derive(Debug, Deserialize)]
struct Grouping {
    regexp: String,
    name: Option<String>,
    #[serde(skip)]
    compiled_regexp: Option<Regex>,
}

fn compile_groupings(groupings: &mut [Grouping]) {
    for grouping in groupings {
        grouping.compiled_regexp = Some(Regex::new(&grouping.regexp).unwrap());
    }
}

fn normalize_uri(uri: &str, groupings: &[Grouping]) -> String {
    let trimmed_uri = uri.split('?').next().unwrap_or(uri);

    for grouping in groupings {
        if let Some(ref regexp) = grouping.compiled_regexp {
            if regexp.is_match(uri) {
                let replacement = match &grouping.name {
                    Some(replacement) => replacement,
                    None => &grouping.regexp,
                };
                return regexp.replace(uri, replacement).to_string();
            }
        }
    }
    trimmed_uri.to_string()
}

fn aggregate_logs(logs: Vec<LogEntry>, groupings: &[Grouping]) -> Vec<AggregatedLogEntry> {
    let mut aggregated_logs: std::collections::HashMap<(String, String), AggregatedLogEntry> =
        std::collections::HashMap::new();

    for log in &logs {
        let normalized_uri = normalize_uri(&log.uri.clone(), &groupings);
        let key = (log.method.clone(), normalized_uri.clone());
        let current_log_aggregation = aggregated_logs.entry(key).or_insert(AggregatedLogEntry {
            method: log.method.clone(),
            uri: normalized_uri.clone(),
            ..Default::default()
        });

        current_log_aggregation.count += 1;
        current_log_aggregation.status_code = match log.status.chars().next() {
            Some('1') => StatusCode {
                status_1xx: current_log_aggregation.status_code.status_1xx + 1,
                ..current_log_aggregation.status_code
            },
            Some('2') => StatusCode {
                status_2xx: current_log_aggregation.status_code.status_2xx + 1,
                ..current_log_aggregation.status_code
            },
            Some('3') => StatusCode {
                status_3xx: current_log_aggregation.status_code.status_3xx + 1,
                ..current_log_aggregation.status_code
            },
            Some('4') => StatusCode {
                status_4xx: current_log_aggregation.status_code.status_4xx + 1,
                ..current_log_aggregation.status_code
            },
            Some('5') => StatusCode {
                status_5xx: current_log_aggregation.status_code.status_5xx + 1,
                ..current_log_aggregation.status_code
            },
            _ => current_log_aggregation.status_code,
        };

        current_log_aggregation.response_time = match log.response_time.parse::<f64>() {
            Ok(log_response_time) => ResponseTime {
                min: log_response_time.min(current_log_aggregation.response_time.min),
                max: log_response_time.max(current_log_aggregation.response_time.max),
                avg: (current_log_aggregation.response_time.sum + log_response_time)
                    / current_log_aggregation.count as f64,
                sum: current_log_aggregation.response_time.sum + log_response_time,
                ..Default::default()
            },
            Err(_) => ResponseTime {
                min: current_log_aggregation.response_time.min,
                max: current_log_aggregation.response_time.max,
                avg: current_log_aggregation.response_time.avg,
                sum: current_log_aggregation.response_time.sum,
                ..Default::default()
            },
        };
    }

    for entry in aggregated_logs.values_mut() {
        let mut response_times: Vec<f64> = Vec::new();
        for log in &logs {
            if log.method == entry.method && log.uri == entry.uri {
                if let Ok(time) = log.response_time.parse::<f64>() {
                    response_times.push(time);
                }
            }
        }
        response_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if !response_times.is_empty() {
            entry.response_time.p50 = response_times[response_times.len() / 2];
            entry.response_time.p75 = response_times[response_times.len() * 3 / 4];
            entry.response_time.p90 = response_times[response_times.len() * 9 / 10];
            entry.response_time.p95 = response_times[response_times.len() * 19 / 20];
            entry.response_time.p99 = response_times[response_times.len() * 99 / 100];
        }
    }

    let mut result: Vec<AggregatedLogEntry> = Vec::new();
    for (_, entry) in aggregated_logs {
        result.push(entry);
    }

    result
}

fn main() {
    let args = Args::parse();

    let default_config_path = "./config.toml";
    let config_path = match args.config {
        Some(path) => path,
        None => default_config_path.to_string(),
    };

    let groupings = match fs::metadata(&config_path).is_ok() {
        true => {
            let config_content = fs::read_to_string(config_path).unwrap();
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

    let aggregated_logs = aggregate_logs(logs, &groupings);
    let json_output = serde_json::to_string_pretty(&aggregated_logs).unwrap();
    println!("{}", json_output);
}
