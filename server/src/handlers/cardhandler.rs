use crate::{
    db::{dbcard::*, dbplayer::*},
    model::card::*,
    IMG_HEIGHT, IMG_WIDTH,
};
use actix_web::{
    delete, error::ErrorBadRequest, get, post, put, web, Error, HttpResponse, Responder, Scope,
};
use image::DynamicImage;
use mongodb::bson::{uuid::Uuid, Document};
use reqwest;
use serde::Deserialize;
use std::io::Cursor;

pub fn cards_scope() -> Scope {
    web::scope("/")
        .service(create_card)
        .service(retrieve_cards)
        .service(retrieve_one_card)
        .service(update_card)
        .service(delete_card)
}

use super::AppState;
#[derive(Deserialize)]
pub struct CreateCard {
    pub name: String,
    pub image: Vec<u8>,
}

#[post("/cards")]
async fn create_card(data: web::Data<AppState>, request: web::Json<CreateCard>) -> impl Responder {
    let db = &data.r.try_lock().unwrap().db;
    println!("string is empty {}", request.name.is_empty());
    let card = Card::new(request.name.clone(), request.image.clone());
    let result = add_card_to_db(db, &card).await;

    match result {
        true => HttpResponse::Ok().json(card),
        false => HttpResponse::Ok().body("Card was not created"),
    }
}

#[get("/cards")]
async fn retrieve_cards(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let db = &data.r.try_lock().unwrap().db;
    let cards = fetch_cards_from_db(db).await.unwrap_or(Vec::new());

    HttpResponse::Ok().json(cards)
}

#[get("/cards/{card_id}")]
async fn retrieve_one_card(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let db = &data.r.try_lock().unwrap().db;
    let card = fetch_one_card_from_db(db, Uuid::parse_str(id.into_inner()).unwrap())
        .await
        .unwrap_or(None);

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
async fn update_card(
    data: web::Data<AppState>,
    id: web::Path<String>,
    request: web::Json<UpdateCard>,
) -> impl Responder {
    let db = &data.r.try_lock().unwrap().db;
    update_card_in_db(
        db,
        Uuid::parse_str(id.into_inner()).unwrap(),
        request.name.clone(),
        request.image.clone(),
        request.element.clone(),
        request.skills.clone(),
        request.owner_id.clone(),
    )
    .await;

    HttpResponse::Ok().body("Updated Card")
}
#[delete("/cards/{card_id}")]
async fn delete_card(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let db = &data.r.try_lock().unwrap().db;
    remove_card_from_db(db, Uuid::parse_str(id.into_inner()).unwrap()).await;
    HttpResponse::Ok().body("Deleted Card")
}
