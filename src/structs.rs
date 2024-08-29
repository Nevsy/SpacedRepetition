use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
	pub id: u32,
    pub title: String,
    pub completed: bool,
    pub created: String,
    pub updated: String,
    pub due: String,
    pub priority: u32,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpacedRepetitionItem {
    pub id: u32,
    pub title: String,
    pub created: String,
    pub last_updated: String,
    pub last_reviewed: String,
    pub next_review: String,
    pub ease: f32,
    pub priority: u32,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagsEase {
    pub tag: String,
    pub count: u32,
    pub ease: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub todos: Vec<Todo>,
    pub spaced_repetition: Vec<SpacedRepetitionItem>,
    pub tags_ease: Vec<TagsEase>,
}
