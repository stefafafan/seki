use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub grouping: Vec<Grouping>,
}

#[derive(Debug, Deserialize)]
pub struct Grouping {
    pub regexp: String,
    pub name: Option<String>,
    #[serde(skip)]
    pub compiled_regexp: Option<Regex>,
}
