use chrono::{Local, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
	pub id: u32,
    pub title: String,
    pub created: DateTime<Local>,
    pub updated: DateTime<Local>,
    pub priority: Option<u32>,
    pub tags: Vec<String>,
}

impl Todo {
	pub fn print_todo(&self) {
		println!("{}\t", self.title);
		println!("\t-> id: {}", self.id);
		println!("\t-> tags: {:?}", self.tags);
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpacedRepetitionItem {
    pub id: u32,
    pub title: String,
    pub created: DateTime<Local>,
    pub updated: DateTime<Local>,
    pub reviewed: DateTime<Local>,
    pub next_review: DateTime<Local>,
	pub interval: u32,
    pub ease: f64,
    pub priority: Option<u32>,
    pub tags: Vec<String>,
}

impl SpacedRepetitionItem {
	pub fn print_item(&self) {
		println!("{}\t", self.title);
		println!("\t-> id: {}", self.id);
		println!("\t-> tags: {:?}", self.tags);
		println!("\t-> Next review in {}d", (self.next_review - Local::now()).num_days());
	}
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct TagsEase {
    pub tag: String,
	// Total number of items with this tag EVER, meaning it will never decrease (even if items are removed)
	// Helps make a weighted average, with a certain confidence level (not implemented)
    pub count: u64,
    pub ease: f64,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Data {
    pub todos: Vec<Todo>,
    pub spaced_repetition: Vec<SpacedRepetitionItem>,
    pub tags_ease: Vec<TagsEase>,
}
