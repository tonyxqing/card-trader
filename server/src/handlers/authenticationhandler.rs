use super::AppState;
use crate::{ AuthenticationToken};
use actix_web::{ web, HttpResponse, Responder, Scope};
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use mongodb::bson::{doc, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub phone: String,
    pub passcode: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub exp: i64,
    pub username: String,
    pub phone: String,
}

impl Default for Claims {
    fn default() -> Self {
        Self {
            id: Uuid::new().to_string(),
            exp: (Utc::now() + Duration::days(7)).timestamp(),
            username: "".to_string(),
            phone: "".to_string(),
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
}

#[derive(Serialize, Deserialize)]
struct PlayerRegistration {
    username: String,
    phone: String,
    passcode: String,
}
async fn register_player(
    data: web::Data<AppState>,
    registration: web::Json<PlayerRegistration>,
) -> impl Responder {
    // check if username or phone number already exists
    let db = &data.r.lock().unwrap().db;
    let users = db.collection::<User>("users");
    let filter_username = doc! { "name": registration.username.clone() };

    let query_username = users
        .find_one(filter_username, None)
        .await
        .is_ok_and(|x| x.is_none());

    let filter_phone = doc! { "phone": registration.phone.clone() };

    let query_phone = users
        .find_one(filter_phone, None)
        .await
        .is_ok_and(|x| x.is_none());

    if query_phone || query_username {
        let password = registration.passcode.as_bytes(); // Bad password; don't actually use!
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        if let Ok(password_hash) = argon2.hash_password(password, &salt) {
            // Verify password against PHC string.
            //
            // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
            // `Argon2` instance.
            if let Ok(parsed_hash) = PasswordHash::new(&password_hash.to_string()) {
                let registered_user = User {
                    username: registration.username.clone(),
                    phone: registration.phone.clone(),
                    passcode: parsed_hash.to_string(),
                };
                let added_user = users.insert_one(&registered_user, None).await;

                if added_user.is_ok() {
                    let claims: Claims = Claims {
                        username: registration.username.clone(),
                        phone: registration.phone.clone(),
                        ..Default::default()
                    };
                    let token: String = encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(data.secret.as_str().as_ref()),
                    )
                    .unwrap();
                    HttpResponse::Ok().json(EncodedResponse {
                        message: "success".to_string(),
                        token,
                    })
                } else {
                    HttpResponse::BadRequest().body("Unable to process registration".to_string())
                }
            } else {
                HttpResponse::BadRequest().body("Unable to process registration".to_string())
            }
        } else {
            HttpResponse::BadRequest().body("Unable to process registration".to_string())
        }
    } else {
        if !query_phone && !query_username {
            HttpResponse::BadRequest().body("Username and Phone Number already exists".to_string())
        } else if !query_phone && query_username {
            HttpResponse::BadRequest().body("Phone Number already exists".to_string())
        } else {
            HttpResponse::BadRequest().body("Username already exists".to_string())
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
        println!("authentication passcode {} {}", authentication.passcode.clone(), String::from(user_password));
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
