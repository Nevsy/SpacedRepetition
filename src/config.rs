use std::fs;
use std::error::Error;
use serde_yaml;
use serde;

#[derive(Debug, serde::Deserialize, Clone, Copy)]
#[warn(dead_code)]
pub struct Config {
	pub fuzz: f32,
	pub minimum_ease: u32,
	pub base_ease: u32, // min 130 best 250
	pub easy_bonus: u32, // min 100%
	pub max_interval: u32,
	pub max_link_contribution: u32,
	pub interval_change: u32 //newInterval = oldInterval * intervalChange / 100
}

impl Config {
	pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
		let config_str = fs::read_to_string(path)?; // Read the file into a string
		let config: Config = serde_yaml::from_str(&config_str)?; // Parse the YAML string into a Config struct
		Ok(config) // Return the Config struct inside an Ok result
	}
}
