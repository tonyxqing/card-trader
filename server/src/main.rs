mod model;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::Mutex;
use model::player::*;

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

async fn manual_hi(data: web::Data<AppState>) -> impl Responder {
    let response = format!("{:?}", data.users.lock().unwrap());
    HttpResponse::Ok().body(response)
}
struct AppState {
    users: Mutex<Vec<Player>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    let initial_state = web::Data::new(AppState {
        users: Mutex::new(vec![]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(initial_state.clone())
            .service(hello_world)
            .service(web::scope("/app").route("/hi", web::get().to(manual_hi)))
            .route("/hi", web::get().to(manual_hi))
            .route("/hello", web::get().to(manual_hi))
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}
