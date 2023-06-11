use actix_web::{
    delete,
    get,
    post, 
    put, 
    web, Error, HttpResponse, Responder, error,
};
use mongodb::{bson::{uuid::Uuid}};
use serde::Deserialize;
use super::AppState;
use crate::{
    db::{ dbplayer::*},
    model::{ player::*},
};

#[get("/players")]
pub async fn retreive_players(data: web::Data<AppState>) -> impl Responder {
    let players = fetch_players_from_db(&data.r.try_lock().unwrap().db)
        .await
        .unwrap_or(Vec::new());
    HttpResponse::Ok().json(players)
}

#[get("/players/{player_id}")]
pub async fn retreive_one_player(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> impl Responder {
    let player = fetch_one_player_from_db(
        &data.r.try_lock().unwrap().db,
        Uuid::parse_str(id.into_inner()).unwrap(),
    )
    .await;
    HttpResponse::Ok().json(player.unwrap())
}

#[derive(Deserialize)]
pub struct CreatePlayer {
    pub name: String,
}
#[post("/players")]
pub async fn create_player(
    data: web::Data<AppState>,
    response: web::Json<CreatePlayer>,
) -> Result<HttpResponse, Error> {
    println!("Welcome {}!", response.name);
    if add_player_to_db(
        &data.r.try_lock().unwrap().db,
        Player::new(response.name.clone()),
    )
    .await
    {
        Ok(HttpResponse::Ok().body("Added User".to_string()))
    } else {
        Err(error::ErrorConflict("Could not create player."))
    }
}
#[derive(Deserialize)]
pub struct UpdatePlayer {
    pub name: String,
}
#[put("/players/{player_id}")]
pub async fn edit_player(
    data: web::Data<AppState>,
    id: web::Path<String>,
    response: web::Json<UpdatePlayer>,
) -> impl Responder {
    println!("Editing {}!", response.name);
    update_player_to_db(
        &data.r.try_lock().unwrap().db,
        Uuid::parse_str(id.into_inner()).unwrap(),
        response.name.clone(),
    )
    .await;
    HttpResponse::Ok().body("Player Edited")
}

#[delete("/players/{player_id}")]
pub async fn delete_player(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    println!("About to remove player");
    let guard = data.r.try_lock().unwrap();
    let db = &guard.db;
    println!("Aquired Mutex");
    if remove_player_from_db(db, Uuid::parse_str(id.into_inner()).unwrap()).await {
        HttpResponse::Ok().body("Removed User".to_string())
    } else {
        HttpResponse::Ok().body("Not able to Remove User".to_string())
    }    
}
