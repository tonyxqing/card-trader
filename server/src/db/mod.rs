use mongodb::{
    bson::{doc, oid::Error, Document, uuid::Uuid},
    options::{ClientOptions, FindOptions},
    Client, Database,
};
use futures::stream::TryStreamExt;
use crate::model::player::*;
use chrono::prelude::*;
use actix_web::{Responder, HttpResponse};
pub struct Resolver {
    pub db: Database,
}

impl Resolver {
    pub async fn new() -> Self {
        let db = connect_db().await.expect("Check database connection");

        Self {
            db
        }
    }
}
pub async fn fetch_players_from_db(db: &Database) -> Result<Vec<Player>, mongodb::error::Error> {
    let collection = db.collection::<Player>("players");
    let mut cursor = collection.find(None, None).await?;
    let mut players = Vec::<Player>::new();
    while let Some(p) = cursor.try_next().await? {
        players.push(p);
    }
    Ok(players)
}

pub async fn connect_db() -> Result<Database, mongodb::error::Error> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    // Manually set an option.
    client_options.app_name = Some("Super Ultimate Card Collector".to_string());
    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;
    // List the names of the databases in that deployment.
    // for db_name in client.list_database_names(None, None).await? {
    //     println!("{}", db_name);
    // }
    // Get a handle to a database.
    let db = client.database("succdb");
    Ok(db)
}

pub async fn add_player_to_db(db: &Database, p: Player) -> bool {
    let collection = db.collection::<Player>("players");
    let result =  collection.insert_one(p, None).await;
    match result {
        Ok(_) => true,
        Err(_) => false,
    }
    
}

pub async fn update_player_to_db(db: &Database, id: Uuid, name: String) -> bool{
    let collection = db.collection::<Player>("players");
    let filter = doc! { "id":  id };
    let result = collection.find_one_and_update(filter, doc! { "$set": { "name": name, "last_updated": Utc::now().to_string()}}, None).await.unwrap_or(None);
    println!("result was {:?} got edited", result);

    match result {
        Some(_) => true,
        None => false,
    }
}   

pub async fn remove_player_from_db(db: &Database, id: Uuid) -> Option<Player> {
    let collection = db.collection::<Player>("players");
    println!("Removing {} from database", id);
    // Query the books in the collection with a filter and an option.
    let filter = doc! { "id": id };
    let did_delete = collection.find_one_and_delete(filter, None).await.unwrap_or(None);
    println!("result was {:?} got deleted", did_delete);
    did_delete
}