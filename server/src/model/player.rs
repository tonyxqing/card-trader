use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub date_created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            date_created: Utc::now(),
            last_updated: Utc::now(),
        }
    }
}