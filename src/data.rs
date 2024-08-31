//mod structs;

//pub use structs::Data;
//use crate::structs::Data;
use crate::structs::*;
use crate::Cli;
use crate::config::Config;

//use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::cmp::min;
use std::error::Error;

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

	pub fn add_todo(&mut self, title: &str, cli: &Cli) {
		let mut next_id = 0;
		
		// get next id
		for (_i, todo) in self.todos.iter().enumerate() {
			if todo.id >= next_id {
				next_id = todo.id + 1;
			}
		}
		for (_i, item) in self.spaced_repetition.iter().enumerate() {
			if item.id >= next_id {
				next_id = item.id + 1;
			}
		}
	
		// add new todo
		let new_todo = Todo {
			id: next_id,
			title: title.to_string(),
			created: chrono::Local::now(),
			updated: chrono::Local::now(),
			priority: Some(0),
			tags: cli.tags.clone(),
		};
		
		// check if tag exists
		for tag in &new_todo.tags {
			if !self.tags_ease.iter().any(|t: &TagsEase| t.tag == *tag) {
				println!("Tag '{}' not found in existing tags, adding it", tag);
				self.add_tag(tag.to_string());
				continue;
			}
		}
		
		// add to todos, save
		self.todos.push(new_todo);
		self.save("./data/data.yaml").expect("Failed to save todo");
		println!("Todo added: \"{}\" -> {}", title, next_id);
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
			println!("No revision found");
		}
		else {
			println!("Listing todos...");
			for item in &self.spaced_repetition {
				println!("{}\t", item.title);
				println!("\t-> id: {}", item.id);
				println!("\t-> tags: {:?}", item.tags);
				println!("\t-> Next review in {}d", (item.next_review - chrono::Local::now()).num_days());
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
				ease: average_tag_ease,  // Start with a default ease
				interval: average_tag_ease as u32,
				next_review: chrono::Local::now() + chrono::Duration::days(average_tag_ease as i64),
				priority: todo.priority,
				tags: todo.tags.clone(),
			};

			// Push it to the spaced repetition list
			if new_item.interval != 1 {
				println!("Moved todo \"{}\" to repetition, next revision in {} days", &new_item.title, &new_item.interval);
			}
			else {
				println!("Moved todo \"{}\" to repetition, next revision in 1 day", &new_item.title);
			}
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

	pub fn revised_item(&mut self, id: u32, ease: u8) {
		let mut did_revise = false;

		if let Some(index) = self.spaced_repetition.iter().position(|item| item.id == id) {
			// Check if the item is due for revision
			if (self.spaced_repetition[index].next_review - chrono::Local::now()).num_seconds() > 0 {
				println!("Item \"{}\" is not due for revision", self.spaced_repetition[index].title);
				println!("Next revision in {} days", (self.spaced_repetition[index].next_review - chrono::Local::now()).num_days());
				return;
			}

			let config = Config::load("./data/settings.yaml").expect("Failed to load config");
			self.spaced_repetition[index].reviewed = chrono::Local::now();
			if ease == 1 {
				self.spaced_repetition[index].ease = 3.0;
				self.spaced_repetition[index].interval = self.spaced_repetition[index].interval * config.interval_change + config.easy_bonus;
			}
			else if ease == 2 {
				self.spaced_repetition[index].ease = 2.0;
				self.spaced_repetition[index].interval = self.spaced_repetition[index].interval * config.interval_change;
			}
			else if ease == 3 {
				self.spaced_repetition[index].ease = 1.0;
				self.spaced_repetition[index].interval = config.hard_time;
			}
			self.spaced_repetition[index].interval = min(self.spaced_repetition[index].interval, config.max_interval);
			self.spaced_repetition[index].next_review = chrono::Local::now() + chrono::Duration::days(self.spaced_repetition[index].interval as i64);

			self.update_tags_ease(index);

			println!("Revised item \"{}\" with ease {}", self.spaced_repetition[index].title, ease);
			println!("\t-> next review in {} days", self.spaced_repetition[index].interval);
			
			did_revise = true;
		}
		if !did_revise {
			println!("Item with id {} not found", id);
			return;
		}

		self.save("./data/data.yaml").expect("Failed to save data");
	}

	fn get_average_tag_ease(&self, tags: &Vec<String>, config: Config) -> f64 {
		let mut total_ease = 0.0;
		
		// Calculate the average ease of the tags
		for tag in tags {
			for (_index, tag_ease) in self.tags_ease.iter().enumerate() {
				if tag == &tag_ease.tag {
					total_ease += tag_ease.ease / tag_ease.count as f64;				
				}
			}
		}

		println!("Average ease: {}", total_ease);
		if total_ease < 1.0 {
			return config.base_interval as f64;
		}
		else {
			let mut ease = total_ease / (tags.len() as f64);
			if ease > config.max_link_contribution as f64 {
				ease = config.max_link_contribution as f64;
			}
			return ease;
		}
	}

	fn update_tags_ease(&mut self, index: usize) {
		for (_index, tag) in self.spaced_repetition[index].tags.iter().enumerate() {
			for (_index, tag_ease) in self.tags_ease.iter_mut().enumerate() {
				if tag == &tag_ease.tag {
					tag_ease.count += 1;
					tag_ease.ease += self.spaced_repetition[index].ease;
				}
			}
		}
	}

	pub fn add_tag (&mut self, tag: String) {
		self.tags_ease.push(TagsEase {
			tag: tag.clone(),
			ease: 0.0,
			count: 0
		});
	}
}
