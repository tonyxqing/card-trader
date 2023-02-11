use std::{sync::Arc};
use futures_util::{lock::Mutex};
use slab::Slab;

pub mod modelcard;
pub use modelcard::*;
pub mod modelcardholder;
pub use modelcardholder::*;

pub type Storage = Arc<Mutex<Slab<CardHolder>>>;