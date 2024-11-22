use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub grouping: Vec<Grouping>,
}

#[derive(Deserialize)]
pub struct Grouping {
    pub regexp: String,
    pub name: Option<String>,
    #[serde(skip)]
    pub compiled_regexp: Option<Regex>,
}
