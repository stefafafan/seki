use std::io::{self, BufRead};

#[derive(serde::Deserialize, Debug)]
struct LogEntry {
    method: String,
    uri: String,
    status: String,
    response_time: String,
}

#[derive(Default)]
struct AggregatedLogEntry {
    method: String,
    uri: String,
    count: u64,
    status_code: StatusCode,
    response_time: ResponseTime,
}

#[derive(Default)]
struct StatusCode {
    status_1xx: u64,
    status_2xx: u64,
    status_3xx: u64,
    status_4xx: u64,
    status_5xx: u64,
}

#[derive(Default)]
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

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let entry: LogEntry = serde_json::from_str(&line).unwrap();
        println!("{:?}", entry);
    }
}
