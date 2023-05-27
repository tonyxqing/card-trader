pub mod dbplayer;
use mongodb::{Database};
use dbplayer::*;
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