use std::io::Cursor;

use super::AppState;
use crate::{
    db::{
        dbcard::{add_card_to_db, fetch_cards_for_player_from_db},
        dbplayer::*,
    },
    model::{card::Card, player::*},
    IMG_HEIGHT, IMG_WIDTH,
};
use actix_web::{
    delete,
    error::{self, ErrorBadRequest},
    get, post, put, web, Error, HttpResponse, Responder, Scope,
};
use image::DynamicImage;
use mongodb::bson::uuid::Uuid;
use serde::Deserialize;

pub fn players_scope() -> Scope {
    web::scope("")
        .service(retreive_players)
        .service(retreive_one_player)
        .service(create_player)
        .service(update_player)
        .service(delete_player)
        .service(create_card_for_player)
        .service(retreive_player_cards)
}

#[derive(Deserialize, Debug)]
pub struct CreatePlayerCard {
    pub name: String,
    pub image: Vec<u8>,
}

pub async fn get_random_dog() -> DynamicImage {
        let url = "https://dog.ceo/api/breeds/image/random";
        let response = reqwest::get(url)
            .await
            .map_err(|e| ErrorBadRequest("GET request to dog resource failed")).unwrap();
        let json = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| ErrorBadRequest("deserializing response from JSON failed")).unwrap();
        let picture_url = json["message"].as_str().unwrap();

        let picture_response = reqwest::get(picture_url)
            .await
            .map_err(|e| ErrorBadRequest("GET request to dog image failed")).unwrap();
        let picture_bytes = picture_response
            .bytes()
            .await
            .map_err(|e| ErrorBadRequest("Could not get picture response bytes")).unwrap();
        let cursor = std::io::Cursor::new(picture_bytes);

        image::load(
            cursor,
            image::ImageFormat::from_path(picture_url)
                .map_err(|e| ErrorBadRequest("Failed loading the image from the path")).unwrap(),
        ).unwrap()
}


#[post("/players/{player_id}/cards")]
async fn create_card_for_player(
    data: web::Data<AppState>,
    id: web::Path<String>,
    request: web::Json<CreatePlayerCard>,
) -> Result<HttpResponse, Error> {
    let db = &data.r.try_lock().unwrap().db;

    let player = fetch_one_player_from_db(db, Uuid::parse_str(id.into_inner()).unwrap()).await;

    if let Some(p) = player {
        let mut new_image = Vec::new();
        let image;
        if request.image.len() > 0 {
            image = image::load_from_memory(&request.image).unwrap();
        } else {
            image = get_random_dog().await;
        }

        image
            .resize_exact(IMG_WIDTH, IMG_HEIGHT, image::imageops::Nearest)
            .write_to(
                &mut Cursor::new(&mut new_image),
                image::ImageOutputFormat::Png,
            )
            .map_err(|e| ErrorBadRequest("Could not load my picture"))?;
        let mut card = Card::new(request.name.clone(), new_image.clone());
        card.assign_owner(Some(p.id));
        add_card_to_db(db, card.clone()).await;
        Ok(HttpResponse::Ok().json(card))
    } else {
        Err(ErrorBadRequest("err"))
    }
}

#[get("/players/{player_id}/cards")]
async fn retreive_player_cards(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let db = &data.r.try_lock().unwrap().db;
    let player = fetch_one_player_from_db(db, Uuid::parse_str(id.into_inner()).unwrap()).await;
    if let Some(p) = player {
        let cards = fetch_cards_for_player_from_db(db, p.id)
            .await
            .expect("Could not retrieve Cards for Player from Db");
        HttpResponse::Ok().json(cards)
    } else {
        HttpResponse::Ok().body("Could not fetch cards")
    }
}

#[get("/players")]
async fn retreive_players(data: web::Data<AppState>) -> impl Responder {
    let players = fetch_players_from_db(&data.r.try_lock().unwrap().db)
        .await
        .unwrap_or(Vec::new());
    HttpResponse::Ok().json(players)
}

#[get("/players/{player_id}")]
async fn retreive_one_player(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
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
async fn create_player(
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
    pub dink_coin: u32,
    pub social_credit: u32,
}
#[put("/players/{player_id}")]
async fn update_player(
    data: web::Data<AppState>,
    id: web::Path<String>,
    response: web::Json<UpdatePlayer>,
) -> impl Responder {
    println!("Editing {}!", response.name);
    update_player_to_db(
        &data.r.try_lock().unwrap().db,
        Uuid::parse_str(id.into_inner()).unwrap(),
        response.name.clone(),
        response.dink_coin.clone(),
        response.social_credit.clone(),
    )
    .await;
    HttpResponse::Ok().body("Player Edited")
}

#[delete("/players/{player_id}")]
async fn delete_player(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
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
