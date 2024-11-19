use std::io::{self, BufRead};

#[derive(serde::Deserialize, Debug)]
struct LogEntry {
    method: String,
    uri: String,
    status: String,
    response_time: String,
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
