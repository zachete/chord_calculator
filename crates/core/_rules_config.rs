use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub static CONFIG_YAML: &str = include_str!("./rules.yaml");

#[derive(Serialize, Deserialize, Debug)]
pub struct Scale {
    pub steps: Vec<u8>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RulesConfig {
    pub scales: HashMap<String, Scale>,
    pub chords: HashMap<String, Vec<u8>>,
    pub diatonic: HashMap<String, Vec<String>>,
}

pub fn load_rules() -> RulesConfig {
    return serde_yaml::from_str(&CONFIG_YAML).expect("Failed to parse YAML");
}
