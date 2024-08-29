//mod structs;

//pub use structs::Data;
//use crate::structs::Data;
use crate::structs::*;

//use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
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

	pub fn move_to_spaced_repetition(&mut self, todo_id: u32) -> Result<(), String> {
		if let Some(index) = self.todos.iter().position(|todo| todo.id == todo_id) {
			let todo = self.todos.remove(index);
			let new_item = SpacedRepetitionItem {
				id: todo.id,
				title: todo.title,
				created: todo.created.clone(),
				last_updated: todo.updated.clone(),
				last_reviewed: todo.updated.clone(),
				next_review: todo.due.clone(),
				ease: 1.0,  // Start with a default ease
				priority: todo.priority,
				tags: todo.tags.clone(),
			};
			self.spaced_repetition.push(new_item);
			Ok(())
		} else {
			Err(format!("Todo with id {} not found", todo_id))
		}
	}	
}