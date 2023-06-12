mod db;
mod handlers;
mod model;
use actix_cors::Cors;
use actix_web::{
    error::{ErrorBadRequest},
    get, 
    middleware::Logger,
    web, App, Error, HttpResponse, HttpRequest, HttpServer,
};
use db::Resolver;
use env_logger;
use handlers::{cardhandler::*, playerhandler::*, AppState};
use image::io::Reader as ImageReader;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::io::Cursor;
use std::sync::Mutex;
use actix_web_actors::ws;
use actix::{Actor, StreamHandler};

const IMG_WIDTH: u32 = 250;
const IMG_HEIGHT: u32 = 350;

struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}


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
            .service(retreive_players)
            .service(retreive_one_player)
            .service(create_player)
            .service(delete_player)
            .service(edit_player)
            .service(fetch_my_picture)
            .service(create_card_for_player)
            .service(retreive_player_cards)
            .service(retrieve_cards)
            .service(retrieve_one_card)
            .service(delete_card)
            .service(update_card)
            .route("/ws/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
