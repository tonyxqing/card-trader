mod db;
mod extractors;
mod handlers;
mod model;
use actix_cors::Cors;
use actix_web::Scope;
use actix_web::{
    error::ErrorBadRequest, get, middleware::Logger, web, App, Error, HttpResponse, HttpServer,
};
use db::Resolver;
use env_logger;
use extractors::authentication_token::*;
use handlers::{authenticationhandler::*, cardhandler::*, playerhandler::*, AppState};
use image::io::Reader as ImageReader;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;
use std::io::Cursor;
use std::sync::Mutex;
const IMG_WIDTH: u32 = 250;
const IMG_HEIGHT: u32 = 350;

#[get("/my-picture")]
async fn fetch_my_picture() -> Result<HttpResponse, Error> {
    let image = ImageReader::open("src/images/favicon.png")?
        .decode()
        .map_err(|e| ErrorBadRequest("error"))?;
    let mut picture_bytes = Vec::new();
    image
        .resize_exact(IMG_WIDTH, IMG_HEIGHT, image::imageops::FilterType::Nearest)
        .write_to(
            &mut Cursor::new(&mut picture_bytes),
            image::ImageOutputFormat::Png,
        )
        .map_err(|e| ErrorBadRequest("Could not load my picture"))?;

    Ok(HttpResponse::Ok()
        .content_type("image/jpeg")
        .body(picture_bytes))
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
        battle_queue: Vec::new(),
        secret: env::var("JWT_SECRET").unwrap_or("default".to_string()),
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
            .app_data(initial_state.clone())
            .service(fetch_my_picture)
            .service(login_scope())
            .service(players_scope())
            .service(cards_scope())
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

// .bind_openssl("127.0.0.1:5000", builder)?