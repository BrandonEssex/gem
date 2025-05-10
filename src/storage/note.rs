use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub shard: String,
    pub tags: Vec<String>,
    pub created: String,
    pub modified: String,
}
