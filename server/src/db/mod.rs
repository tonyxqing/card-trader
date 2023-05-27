pub mod dbplayer;
pub mod dbcard;
use mongodb::{Database, options::{ClientOptions}, Client};


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