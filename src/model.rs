use chrono::Utc;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Baja,
    Media,
    Alta,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub id: String,
    pub timestamp: String,
    pub task: String,
    pub level: Level,
    pub source: String,
}

impl Entry {
    pub fn new(task:String, level: Level, source: String) -> Self {
        Entry {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().to_utc().to_string(),
            task,
            level,
            source,
        }
    }
}