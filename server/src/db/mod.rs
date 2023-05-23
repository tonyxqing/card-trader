use mongodb::{
    bson::{doc, oid::Error, Document},
    options::{ClientOptions, FindOptions},
    Client, Database,
};

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
    let doc = p;
    match collection.insert_one(doc, None).await {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn remove_player_from_db(db: &Database, name: String) -> Option<Player> {
    let collection = db.collection::<Player>("players");
    println!("Removing {} from database", name);
    // Query the books in the collection with a filter and an option.
    let filter = doc! { "name": name.as_str() };
    let did_delete = collection.find_one_and_delete(filter, None).await.ok().unwrap();
    println!("result was {:?} got deleted", did_delete);
    did_delete
}