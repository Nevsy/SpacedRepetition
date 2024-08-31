//mod structs;

//pub use structs::Data;
//use crate::structs::Data;
use crate::structs::*;
use crate::config::Config;

//use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::cmp::*;
use std::error::Error;
use rand::prelude::*;


impl Data {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let data_str = fs::read_to_string(path)?;  // Read the file into a string
        let data: Data = serde_yaml::from_str(&data_str)?;  // Parse the YAML string into a Data struct
        Ok(data)  // Return the Data struct inside an Ok result
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let data_str = serde_yaml::to_string(self)?;  // Serialize the Data struct into a YAML string
        fs::write(path, data_str)?;  // Write the string to the file specified by path
        Ok(())
    }

	pub fn remove_item(&mut self, id: u32) -> Result<(), String> {
		for (index, todo) in self.todos.iter().enumerate() {
			if todo.id == id {
				self.todos.remove(index);
				return Ok(());
			}
		}

		for (index, item) in self.spaced_repetition.iter().enumerate() {
			if item.id == id {
				self.spaced_repetition.remove(index);
				return Ok(());
			}
		}

		Err(format!("Todo with id {} not found", id))
	}

	pub fn list_spaced_repetition(&self) {
		if self.spaced_repetition.is_empty() {
			println!("No todos found");
		}
		else {
			println!("Listing todos...");
			for item in &self.spaced_repetition {
				println!("{}\t", item.title);
				println!("\t-> id: {}", item.id);
				println!("\t-> tags: {:?}", item.tags);
			}
		}
	}

	pub fn list_todos(&self) {
		if self.todos.is_empty() {
			println!("No todos found");
		}
		else {
			println!("Listing todos...");
			for todo in &self.todos {
				println!("{}\t", todo.title);
				println!("\t-> id: {}", todo.id);
				println!("\t-> tags: {:?}", todo.tags);
			}
		}
	}

	pub fn move_to_spaced_repetition(&mut self, todo_id: u32) -> Result<(), String> {
		if let Some(index) = self.todos.iter().position(|todo| todo.id == todo_id) {
			// Remove the todo from the todos list
			let todo = self.todos.remove(index);

			// Calculate the time until the next review
			let config = Config::load("./data/settings.yaml").expect("Failed to load config");
			let average_tag_ease = self.get_average_tag_ease(&todo.tags, config);
			
			// Make the new item
			let new_item = SpacedRepetitionItem {
				id: todo.id,
				title: todo.title,
				created: todo.created.clone(),
				updated: chrono::Local::now(),
				reviewed: chrono::Local::now(),
				next_review: todo.updated.clone(),
				ease: average_tag_ease,  // Start with a default ease
				interval: min(config.max_interval, average_tag_ease as u32),
				priority: todo.priority,
				tags: todo.tags.clone(),
			};
			// Push it to the spaced repetition list
			self.spaced_repetition.push(new_item);
			Ok(())
		} else {
			Err(format!("Todo with id {} not found", todo_id))
		}
	}

	pub fn check_to_revise(&self) {
		if self.spaced_repetition.is_empty() {
			println!("Nothing to revise");
		}
		else {
			println!("Checking to revise...");
			let current_time = chrono::Local::now();
			for item in &self.spaced_repetition {
				if (item.next_review - current_time).num_seconds() <= 0 {
					println!("{}\t", item.title);
					println!("\t-> id: {}", item.id);
					println!("\t-> tags: {:?}", item.tags);
				}
			}
		}
	}

	fn get_average_tag_ease(&self, tags: &Vec<String>, config: Config) -> f64 {
		let mut total_ease = 0.0;
		for tag in tags {
			for (_index, tag_ease) in self.tags_ease.iter().enumerate() {
				if tag == &tag_ease.tag {
					let random_float: f64 = rand::thread_rng().gen::<f64>() * 2.0 - 1.0; // generates a float between 0 and 1 -> -1.0 and 1.0
					total_ease += tag_ease.ease * (random_float * config.fuzz as f64);
				}
			}
		}
		if total_ease == 0.0 {
			return 50.0;
		}
		else {
			return total_ease / (tags.len() as f64);
		}
	}

	pub fn add_tag (&mut self, tag: String) {
		self.tags_ease.push(TagsEase {
			tag: tag.clone(),
			ease: 50.0,
			count: 1
		});
	}
}
