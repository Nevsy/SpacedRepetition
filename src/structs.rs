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

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct TagsEase {
    pub tag: String,
    pub count: u32,
    pub ease: f64,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Data {
    pub todos: Vec<Todo>,
    pub spaced_repetition: Vec<SpacedRepetitionItem>,
    pub tags_ease: Vec<TagsEase>,
}
