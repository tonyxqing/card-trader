use crate::model::card::*;
use futures::stream::{TryStreamExt};
use mongodb::{Database, bson::{doc, uuid::Uuid, Binary, to_bson}, error::Error};

pub async fn fetch_cards_for_player_from_db(db: &Database, player_id: Uuid) -> Result<Vec<Card>, Error>  {
    let collection = db.collection::<Card>("cards");
    let filter = doc! {"owner_id": player_id};
    let mut cursor = collection.find(filter, None).await?;
    let mut cards = Vec::<Card>::new();
    while let Some(c) = cursor.try_next().await? {
        cards.push(c);
    }
    Ok(cards)
}


pub async fn fetch_cards_from_db (db: &Database) -> Result<Vec<Card>, Error> {
    let collection = db.collection::<Card>("cards");
    let mut result = collection.find(None, None).await?;
    let mut cards = Vec::<Card>::new();

    while let Some(c) = result.try_next().await? {
        cards.push(c);
    } 
    Ok(cards)

}

pub async fn fetch_one_card_from_db(db: &Database, id: Uuid) -> Result<Option<Card>, Error> {
    let collection = db.collection::<Card>("cards");
    let filter = doc! { "id": id }; 
    let result = collection.find_one(filter, None).await?;
    Ok(result)

}

pub async fn add_card_to_db(db: &Database, c: Card) -> bool {
    println!("adding new card to the db: {:?}", c);
    let collection = db.collection::<Card>("cards");
    let result = collection.insert_one(c, None).await;
    match result {
        Ok(_) => true,
        Err(_) => false,
    }

}

pub async fn remove_card_from_db (db: &Database, id: Uuid) -> bool {
    let collection = db.collection::<Card>("cards");
    let filter = doc! {"id": id};
    let result = collection.find_one_and_delete(filter, None).await;
    match result {
        Ok(_) => true,
        Err(_) => false,
    }}

pub async fn update_card_in_db (db: &Database, id: Uuid, name: String, image: Vec<u8>, element: Element, skills: Skills, owner_id: Option<Uuid>) -> bool {
    let collection = db.collection::<Card>("cards");
    let filter = doc! {"id": id};
    let binary = Binary {
        subtype: mongodb::bson::spec::BinarySubtype::Generic,
        bytes: image
    };
    let bson_element = to_bson(&element).expect("Element not found when updating card");
    let bson_skills = to_bson(&skills).expect("Skills not found when updating player");
    let update = doc!{ "$set": {"name": name, "image": binary, "element": bson_element, "skills": bson_skills, "owner_id": owner_id }};
    let result = collection.find_one_and_update(filter, update, None).await;
    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}
