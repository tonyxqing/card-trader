mod model;
mod db;
use futures::stream::StreamExt;
use actix_cors::Cors;
use actix_web::{get, post, delete, put, http, Error, error, web, App, HttpRequest, HttpResponse, HttpServer, Responder, middleware::Logger};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use mongodb::bson::{uuid::Uuid};
use succ_api::db::update_player_to_db;
use std::sync::Mutex;
use model::player::*;
use db::*;
use chrono::prelude::*;
use serde_json;
use env_logger;
use serde::Deserialize;


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
async fn retreive_one_player(data: web::Data<AppState>) -> impl Responder {
    let players = fetch_players_from_db(&data.r.lock().unwrap().db).await.unwrap_or(Vec::new());
    HttpResponse::Ok().json(players)
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
}
#[put("/players/{player_id}")]
async fn edit_player(data: web::Data<AppState>, id: web::Path<String>, response: web::Json<UpdatePlayer>) -> impl Responder { 
    println!("Editing {}!", response.name);
    update_player_to_db(&data.r.lock().unwrap().db, Uuid::parse_str(id.into_inner()).unwrap(), response.name.clone()).await;
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
            .app_data(initial_state.clone())
            .service(hello_world)
            .service(retreive_players)
            .service(create_player)
            .service(delete_player)
            .service(edit_player)
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}
