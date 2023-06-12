use chrono::prelude::*;
use mongodb::bson::uuid::Uuid;
use rnglib::{Language, RNG};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: Uuid,
    pub social_credit: u32,
    pub dink_coin: u32,
    pub name: String,
    pub date_created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl Player {
    pub fn new(mut name: String) -> Self {
        if name.is_empty() {
            let rng = RNG::try_from(&Language::Elven).unwrap();
            let rng2 = RNG::try_from(&Language::Demonic).unwrap();
            name = format!("{} {}", rng.generate_name(), rng2.generate_name());
        }
        Self {
            id: Uuid::new(),
            name,
            social_credit: 0,
            dink_coin: 0,
            date_created: Utc::now(),
            last_updated: Utc::now(),
        }
    }
}
