pub mod cardhandler;
pub mod playerhandler;
use std::sync::Mutex;
use crate::db::Resolver;
pub struct AppState {
    pub r: Mutex<Resolver>,
}