use std::env;

use super::AppState;
use crate::{
    db::{dbcard::add_card_to_db, dbplayer::add_player_to_db},
    model::{card::Card, player::Player},
    AuthenticationToken,
};
use actix_web::{web, HttpResponse, Responder, Scope};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use mongodb::bson::{doc, Uuid};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub passcode: String,
    pub role: Roles,
    pub verified: bool,
    pub verification_code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Roles {
    Owner,
    Admin,
    Subscriber,
    Basic,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub exp: i64,
    pub role: Roles,
    pub username: String,
    pub email: String,
}

impl Default for Claims {
    fn default() -> Self {
        Self {
            id: Uuid::new().to_string(),
            exp: (Utc::now() + Duration::days(7)).timestamp(),
            username: "".to_string(),
            email: "".to_string(),
            role: Roles::Basic,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

pub fn login_scope() -> Scope {
    web::scope("/api")
        .route("/register-player", web::post().to(register_player))
        .route("/authenticate-player", web::post().to(authenticate_player))
        .route("/encode-token/{id}", web::get().to(encode_token))
        .route("/decode-token", web::post().to(decode_token))
        .route("/protected", web::get().to(protected))
        .route("/verify/{user}/{token}", web::get().to(verify_email))
}

pub async fn verify_email(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let db = &data.r.lock().unwrap().db;
    let user_filter = doc! {
        "username": path.0.clone(),
        "verification_code": path.1.clone(),
        "verified": false
    };
    let users = db.collection::<User>("users");
    let verify_user = users.find_one_and_update(user_filter, doc! { "$set": {"verified": true} }, None).await;
    match verify_user {
        Ok(x) => HttpResponse::Ok().body("verified player"),
        Err(_) => HttpResponse::Ok().body("did not verified player")
    }   
}

#[derive(Serialize, Deserialize)]
struct PlayerRegistration {
    username: String,
    email: String,
    passcode: String,
    role: Roles,
}
async fn register_player(
    data: web::Data<AppState>,
    registration: web::Json<PlayerRegistration>,
) -> impl Responder {
    // check if username or email number already exists
    let db = &data.r.lock().unwrap().db;
    let users = db.collection::<User>("users");
    let filter_username = doc! { "username": registration.username.clone() };

    let username_is_unique = users
        .find_one(filter_username, None)
        .await
        .unwrap()
        .is_none();

    let filter_email = doc! { "email": registration.email.clone() };

    let email_is_unique = users.find_one(filter_email, None).await.unwrap().is_none();

    if email_is_unique && username_is_unique {
        let password = registration.passcode.as_bytes();
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2.hash_password(password, &salt).unwrap();
        let password_str = password_hash.to_string();
        let parsed_hash = PasswordHash::new(&password_str).unwrap();

        dotenv().ok();
        // Retrieve SMTP credentials from environment variables
        let mut rng = thread_rng();
        let random_string = (0..32)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect::<String>();

        let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set");
        let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");

        let body = format!(
            "This is the email body. <a href='http://localhost:5000/api/verify/{}/{}'>click here to verify</a>", registration.username,
            random_string
        );
        let email = Message::builder()
            .from(
                format!("Super Ultimate Card Collection <{}>", smtp_username)
                    .parse()
                    .unwrap(),
            )
            .to(registration.email.clone().parse().unwrap())
            .subject("Verify Your Account")
            .header(ContentType::TEXT_HTML)
            .body(body)
            .unwrap();

        let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => {
                println!("Email sent successfully!")
            }
            Err(e) => println!("Email failed, {}", e),
        }
        let new_user = User {
            username: registration.username.clone(),
            email: registration.email.clone(),
            passcode: parsed_hash.to_string(),
            role: registration.role.clone(),
            verified: false,
            verification_code: random_string,
        };
        let insert_user = users.insert_one(&new_user, None).await;

        if insert_user.is_ok() {
            let claims: Claims = Claims {
                username: registration.username.clone(),
                role: registration.role.clone(),
                ..Default::default()
            };
            let token: String = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(data.secret.as_str().as_ref()),
            )
            .unwrap();
            let new_player = Player::new(registration.username.clone());
            let new_card = Card::new("".to_string(), Vec::new());
            add_player_to_db(db, new_player).await;
            add_card_to_db(db, new_card).await;
            HttpResponse::Ok().json(EncodedResponse {
                message: "success".to_string(),
                token,
            })
        } else {
            HttpResponse::BadRequest().body("Unable to process registration".to_string())
        }
    } else {
        match (email_is_unique, username_is_unique) {
            (false, false) => {
                HttpResponse::BadRequest().body("Username and Email already exist".to_string())
            }
            (false, true) => HttpResponse::BadRequest().body("Username already exists".to_string()),
            (true, false) => HttpResponse::BadRequest().body("Email already exists".to_string()),
            _ => HttpResponse::InternalServerError().body("Unexpected error".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize)]

struct PlayerAuthentication {
    username: String,
    passcode: String,
}
async fn authenticate_player(
    data: web::Data<AppState>,
    authentication: web::Json<PlayerAuthentication>,
) -> impl Responder {
    let db = &data.r.lock().unwrap().db;
    let users = db.collection::<User>("users");
    let user_filter = doc! {"username": authentication.username.clone() };
    let authenticate_user = users
        .find_one(user_filter, None)
        .await
        .expect("Could not find user in db");
    if let Some(user) = authenticate_user {
        let user_password = user.passcode.as_ref();
        let password = authentication.passcode.as_bytes();
        println!(
            "authentication passcode {} {}",
            authentication.passcode.clone(),
            String::from(user_password)
        );
        let verified = Argon2::default()
            .verify_password(password, &PasswordHash::new(user_password).expect("msg"))
            .is_ok();
        if verified {
            HttpResponse::Ok().body("Login Passed".to_string())
        } else {
            HttpResponse::BadRequest().body("Login Failed".to_string())
        }
    } else {
        HttpResponse::BadRequest().body("Login Failed".to_string())
    }
}

#[derive(Serialize, Deserialize)]

struct EncodedResponse {
    message: String,
    token: String,
}

async fn encode_token(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let id: String = id.into_inner();
    let claims: Claims = Claims {
        id,
        ..Default::default()
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.secret.as_str().as_ref()),
    );

    match token {
        Ok(token) => HttpResponse::Ok().json(EncodedResponse {
            message: "success".to_string(),
            token,
        }),
        Err(e) => HttpResponse::BadRequest().json(Response {
            message: e.to_string(),
        }),
    }
}

#[derive(Serialize, Deserialize)]
struct DecodeBody {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct DecodeResponse {
    message: String,
    id: String,
}
async fn decode_token(data: web::Data<AppState>, body: web::Json<DecodeBody>) -> impl Responder {
    let decoded: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
        &body.token,
        &DecodingKey::from_secret(data.secret.as_str().as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    match decoded {
        Ok(token) => HttpResponse::Ok().json(DecodeResponse {
            message: "Authorized!".to_string(),
            id: token.claims.id,
        }),
        Err(e) => HttpResponse::BadRequest().json(Response {
            message: e.to_string(),
        }),
    }
}

async fn protected(token: AuthenticationToken) -> impl Responder {
    HttpResponse::Ok().json(Response {
        message: "protected".to_string(),
    })
}
