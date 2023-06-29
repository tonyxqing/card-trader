pub mod cardhandler;
pub mod playerhandler;
pub mod authenticationhandler;
use std::sync::Mutex;
use mongodb::bson::Uuid;

use crate::db::Resolver;
pub struct AppState {
    pub r: Mutex<Resolver>,
    pub battle_queue: Vec<Uuid>,
    pub secret: String
}