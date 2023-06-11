use actix_web::{get, post, delete, put, Error, error::{ErrorBadRequest}, web, HttpResponse, Responder};
use mongodb::bson::{uuid::Uuid, Document};
use serde::Deserialize;
use image::{DynamicImage};
use reqwest;
use std::io::Cursor;
use crate::{db::{dbcard::*, dbplayer::*}, model::{card::*}, IMG_HEIGHT, IMG_WIDTH};

use super::AppState;

#[get("/cards")]
pub async fn retrieve_cards(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder{
    let db = &data.r.try_lock().unwrap().db;
    let cards = fetch_cards_from_db(db).await.unwrap_or(Vec::new());
    
    HttpResponse::Ok().json(cards)
}

#[derive(Deserialize)] 
pub struct CreateCard {
    pub name: String,
    pub image: Vec<u8>,
}
#[post("/cards")]
pub async fn create_card(data: web::Data<AppState>, request: web::Json<CreateCard>) -> impl Responder{
    let db = &data.r.try_lock().unwrap().db;
    println!("string is empty {}", request.name.is_empty());
    let card = Card::new(request.name.clone(), request.image.clone());
    let result = add_card_to_db(db, &card).await;
    
    match result {
        true => HttpResponse::Ok().json(card),
        false=> HttpResponse::Ok().body("Card was not created")
    }
}


#[get("/players/{player_id}/cards")]
pub async fn retreive_player_cards(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let db = &data.r.try_lock().unwrap().db;
    let player = fetch_one_player_from_db(db, Uuid::parse_str(id.into_inner()).unwrap()).await;
    if let Some(p) = player {
        let cards = fetch_cards_for_player_from_db(db, p.id).await.expect("Could not retrieve Cards for Player from Db");
        HttpResponse::Ok().json(cards)

    } else {
        HttpResponse::Ok().body("Could not fetch cards")
    }

}
#[derive(Deserialize, Debug)]
pub struct CreatePlayerCard {
    pub name: String,
    pub image: Vec<u8>,   
}

#[post("/players/{player_id}/cards")]
pub async fn create_card_for_player (data: web::Data<AppState>, id: web::Path<String>, request: web::Json<CreatePlayerCard>) -> Result<HttpResponse, Error> {
    let db = &data.r.try_lock().unwrap().db;
    
    let player = fetch_one_player_from_db(db, Uuid::parse_str(id.into_inner()).unwrap()).await;
    
    
    if let Some(mut p) = player {
    let mut new_image = Vec::new();
    let image  =  if request.image.len() > 0 {
        image::load_from_memory(&request.image)
    } else {
        let url = "https://dog.ceo/api/breeds/image/random";
        let response = reqwest::get(url).await.map_err(|e|ErrorBadRequest("GET request to dog resource failed"))?;
        let json = response.json::<serde_json::Value>().await.map_err(|e|ErrorBadRequest("deserializing response from JSON failed"))?;
        let picture_url = json["message"].as_str().unwrap();
        
        let picture_response = reqwest::get(picture_url).await.map_err(|e|ErrorBadRequest("GET request to dog image failed"))?;
        let picture_bytes = picture_response.bytes().await.map_err(|e|ErrorBadRequest("Could not get picture response bytes"))?;
        let cursor = std::io::Cursor::new(picture_bytes);
        
        image::load(cursor, image::ImageFormat::from_path(picture_url).map_err(|e|ErrorBadRequest("Failed loading the image from the path"))?)
    };
    match image {
        Ok(picture) => {
            picture.resize_exact(IMG_WIDTH, IMG_HEIGHT, image::imageops::Nearest).write_to(&mut Cursor::new(&mut new_image), image::ImageOutputFormat::Png).map_err(|e| ErrorBadRequest("Could not load my picture"))?;
                let mut card = Card::new(request.name.clone(), new_image.clone());
                card.assign_owner(Some(p.id));
                add_card_to_db(db, &card).await;
                Ok(HttpResponse::Ok().json(card))
            },
            Err(_) => Err(ErrorBadRequest("err"))
        }
    } 
    else {
        Err(ErrorBadRequest("err"))
    }
}

#[get("/cards/{card_id}")]
pub async fn retrieve_one_card(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder{
    let db = &data.r.try_lock().unwrap().db;
    let card = fetch_one_card_from_db(db, Uuid::parse_str(id.into_inner()).unwrap()).await.unwrap_or(None);

    HttpResponse::Ok().json(card)
}
#[derive(Deserialize)]
pub struct UpdateCard {
    pub name: String,
    pub image: Vec<u8>,
    pub skills: Skills,
    pub element: Element,
    pub owner_id: Option<Uuid>,
}
#[put("/cards/{card_id}")]
pub async fn update_card(data: web::Data<AppState>, id: web::Path<String>, request: web::Json<UpdateCard>) -> impl Responder{
    let db = &data.r.try_lock().unwrap().db;
    update_card_in_db(db, Uuid::parse_str(id.into_inner()).unwrap(), request.name.clone(), request.image.clone(), request.element.clone(), request.skills.clone(), request.owner_id.clone()).await;

    HttpResponse::Ok().body("Updated Card")
}
#[delete("/cards/{card_id}")]
pub async fn delete_card(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder{
    let db = &data.r.try_lock().unwrap().db;
    remove_card_from_db(db, Uuid::parse_str(id.into_inner()).unwrap()).await;
    HttpResponse::Ok().body("Deleted Card")
}