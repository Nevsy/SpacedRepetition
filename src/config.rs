// config.rs
use serde::{Deserialize};
use std::fs;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
	fuzz: f32,
	minimum_ease: u32
	base_ease: u32 // min 130 best 250
	easy_bonus: u32 // min 100%
	max_interval: u32
	max_link_contribution: u32
	interval_change: u32 //newInterval = oldInterval * intervalChange / 100
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let config_str = fs::read_to_string(path)?;  // Read the file into a string
        let config: Config = serde_yaml::from_str(&config_str)?;  // Parse the YAML string into a Config struct
        Ok(config)  // Return the Config struct inside an Ok result
    }
}
