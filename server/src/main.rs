mod model;
mod db;
use actix_cors::Cors;
use actix_web::{get, post, delete, put, http, Error, error::{self, ErrorBadRequest}, web, App, HttpRequest, HttpResponse, HttpServer, Responder, middleware::Logger};
use imageproc::definitions::Image;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use mongodb::bson::{uuid::Uuid};
use std::sync::Mutex;
use model::{player::*, card::*};
use db::{Resolver, dbplayer::*, dbcard::*};
use env_logger;
use serde::Deserialize;
use image::{io::Reader as ImageReader, DynamicImage, Rgb, RgbImage};
use reqwest;
use std::io::Cursor;

const IMG_WIDTH: u32 = 250;
const IMG_HEIGHT: u32 = 350;

#[get("/dog-picture")]
async fn fetch_dog_picture(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let mut selected_player = data.selected_player.lock().unwrap();
    let url = "https://dog.ceo/api/breeds/image/random";

    let response = reqwest::get(url).await.map_err(|e|ErrorBadRequest("err"))?;
    let json = response.json::<serde_json::Value>().await.map_err(|e|ErrorBadRequest("err"))?;
    let picture_url = json["message"].as_str().unwrap();

    let picture_response = reqwest::get(picture_url).await.map_err(|e|ErrorBadRequest("err"))?;
    let picture_bytes = picture_response.bytes().await.map_err(|e|ErrorBadRequest("err"))?;
    let cursor = std::io::Cursor::new(picture_bytes);
    let mut picture_bytes = Vec::new();

    let image: Result<DynamicImage, image::ImageError> = image::load(cursor, image::ImageFormat::from_path(picture_url).map_err(|e|ErrorBadRequest("err"))?);
    
    
    match image {
        Ok(picture) => {
            picture.resize_exact(IMG_WIDTH, IMG_HEIGHT, image::imageops::Nearest).write_to(&mut Cursor::new(&mut picture_bytes), image::ImageOutputFormat::Png).map_err(|e| ErrorBadRequest("Could not load my picture"))?;
            
            let card = Card::new("card_name".to_string(), picture_bytes.clone());
            let player = selected_player.as_mut().unwrap();
            player.add_card(card);
            add_card_to_db(&data.r.lock().unwrap().db, card);
            println!("Added card to Player: {:?}", player);
            Ok(HttpResponse::Ok()
            .content_type("image/jpeg")
            .body(picture_bytes))

        },
        Err(_) => Err(ErrorBadRequest("err"))
    }
}

#[get("/my-picture")]
async fn fetch_my_picture() -> Result<HttpResponse, Error> {
    let image = ImageReader::open("src/images/favicon.png")?.decode().map_err(|e| ErrorBadRequest("error"))?;
    let mut picture_bytes = Vec::new();
    image.resize_exact(IMG_WIDTH, IMG_HEIGHT, image::imageops::FilterType::Nearest).write_to(&mut Cursor::new(&mut picture_bytes), image::ImageOutputFormat::Png).map_err(|e| ErrorBadRequest("Could not load my picture"))?;
    
    
    
    Ok(HttpResponse::Ok()
        .content_type("image/jpeg")
        .body(picture_bytes))
}

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/players")]
async fn retreive_players(data: web::Data<AppState>) -> impl Responder {
    let players = fetch_players_from_db(&data.r.lock().unwrap().db).await.unwrap_or(Vec::new());
    HttpResponse::Ok().json(players)
}

#[get("/players/{player_id}")]
async fn retreive_one_player(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let mut selected_player = data.selected_player.lock().unwrap();
    println!("Retrieving player {}", id);
    let player = fetch_one_player_from_db(&data.r.lock().unwrap().db, Uuid::parse_str(id.into_inner()).unwrap()).await;
    *selected_player = player.clone();
    println!("Selected Player: {:?}", selected_player);
    HttpResponse::Ok().json(player.unwrap())
}

#[derive(Deserialize)]
struct CreatePlayer {
    name: String,
}
#[post("/players")]
async fn create_player(data: web::Data<AppState>, response: web::Json<CreatePlayer>) -> Result<HttpResponse, Error> {
    println!("Welcome {}!", response.name);
    if add_player_to_db(&data.r.lock().unwrap().db, Player::new(response.name.clone())).await {
        Ok(HttpResponse::Ok().body("Added User".to_string()))
    } else {
        Err(error::ErrorConflict("Could not create player."))
    }
}
#[derive(Deserialize)]
struct UpdatePlayer {
    name: String,
    cards: Vec<Uuid>
}
#[put("/players/{player_id}")]
async fn edit_player(data: web::Data<AppState>, id: web::Path<String>, response: web::Json<UpdatePlayer>) -> impl Responder { 
    println!("Editing {}!", response.name);
    update_player_to_db(&data.r.lock().unwrap().db, Uuid::parse_str(id.into_inner()).unwrap(), response.name.clone(), response.cards.clone()).await;
    HttpResponse::Ok().body("Player Edited")
}

#[delete("/players/{player_id}")]
async fn delete_player(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    println!("Goodbye {}", id);
    remove_player_from_db(&data.r.lock().unwrap().db, Uuid::parse_str(id.into_inner()).unwrap()).await;
    HttpResponse::Ok().body("Removed User".to_string())
}

struct AppState {
    r: Mutex<Resolver>,
    selected_player: Mutex<Option<Player>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {   
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();


    let initial_state = web::Data::new(AppState {
        r: Mutex::new(Resolver::new().await),
        selected_player: Mutex::new(None)
    });
    HttpServer::new(move || {
        let cors = Cors::default()
       .allow_any_origin()
       .allow_any_method()
       .allow_any_header()
       .expose_any_header()
       .send_wildcard();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(initial_state.clone())
            .service(hello_world)
            .service(retreive_players)
            .service(retreive_one_player)
            .service(create_player)
            .service(delete_player)
            .service(edit_player)
            .service(fetch_dog_picture)
            .service(fetch_my_picture)

    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}
