use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub shard: String,
    pub tags: Vec<String>,
    pub created: String,
    pub updated: String,
}
