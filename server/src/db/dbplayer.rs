use crate::{
    db::dbcard::update_card_in_db,
    model::{card::Card, player::*},
};
use chrono::prelude::*;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, uuid::Uuid},
    error::Error,
    options::FindOneAndDeleteOptions,
    Database,
};

pub async fn fetch_players_from_db(db: &Database) -> Result<Vec<Player>, Error> {
    let collection = db.collection::<Player>("players");
    let mut cursor = collection.find(None, None).await?;
    let mut players = Vec::<Player>::new();
    while let Some(p) = cursor.try_next().await? {
        players.push(p);
    }
    Ok(players)
}

pub async fn add_player_to_db(db: &Database, p: Player) -> bool {
    let collection = db.collection::<Player>("players");
    let result = collection.insert_one(p, None).await;
    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn update_player_to_db(
    db: &Database,
    id: Uuid,
    name: String,
    dink_coin: u32,
    social_credit: u32,
) -> bool {
    let collection = db.collection::<Player>("players");
    let filter = doc! { "id":  id };
    let result = collection.find_one_and_update(filter, doc! { "$set": { "name": name, "social_credit": social_credit, "dink_coin": dink_coin,  "last_updated": Utc::now().to_string()}}, None).await.unwrap_or(None);
    match result {
        Some(_) => true,
        None => false,
    }
}

pub async fn remove_player_from_db(db: &Database, id: Uuid) -> bool {
    let collection = db.collection::<Player>("players");
    let options = FindOneAndDeleteOptions::builder()
        .max_time(Some(std::time::Duration::from_secs(5)))
        .build();

    let filter = doc! { "id": id };
    let result = collection
        .find_one_and_delete(filter, options)
        .await
        .unwrap_or(None);
    match result {
        Some(_) => {
            let card_filter = doc! {"owner_id": id};
            let did_delete = db
                .collection::<Card>("cards")
                .delete_many(card_filter, None)
                .await
                .expect("could not find cards");

            println!("deleted player and cards {:?}, releasing mutex", did_delete);
            true
        }
        None => false,
    }
}

pub async fn fetch_one_player_from_db(db: &Database, id: Uuid) -> Option<Player> {
    let collection = db.collection::<Player>("players");
    let filter = doc! {"id": id};
    collection.find_one(filter, None).await.unwrap_or(None)
}
