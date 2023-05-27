use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use mongodb::bson::uuid::Uuid;
use super::card::Card;
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub date_created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub cards: Vec<Card>,
    
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new(),
            name,
            date_created: Utc::now(),
            last_updated: Utc::now(),
            cards: Cards
        }
    }
}