use crate::model::card::*;
use mongodb::{Database, bson::uuid::Uuid};


pub async fn add_card_to_db(db: &Database, c: Card) {
    let collection = db.collection::<Card>("cards");
    let result = collection.insert_one(c, None);

}

pub async fn remove_card_from_db (db: &Database) {

}

pub async fn update_card_in_db (db: &Database) {

}

pub async fn fetch_cards_from_db (db: &Database) {

}

pub async fn fetch_one_card_from_db(db: &Database) {
    let collection = db.collection::<Card>("cards");
    let result = collection.find(None, None);
}