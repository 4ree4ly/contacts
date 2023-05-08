use std::fs::File;

use serde::Deserialize;
use serde_yaml::{self};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub contacts_path: String,
}

pub fn get_config() -> Config {
    let file = File::open("config.yml").unwrap();
    serde_yaml::from_reader(file).unwrap()
}
