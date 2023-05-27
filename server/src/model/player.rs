use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use mongodb::bson::uuid::Uuid;
use crate::model::card::*;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub date_created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub cards: Vec<Uuid>,
    
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new(),
            name,
            date_created: Utc::now(),
            last_updated: Utc::now(),
            cards: Vec::new()
        }
    }

    pub fn add_card (&mut self, mut new_card: Card) {
        new_card.assign_owner(Some(self.id));
        self.cards.push(new_card.get_id());
    }
}