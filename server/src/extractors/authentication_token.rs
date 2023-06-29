use crate::handlers::authenticationhandler::Claims;
use crate::handlers::AppState;
use actix_web::{
    dev::Payload, error::ErrorUnauthorized, http, web, Error as ActixWebError, FromRequest,
    HttpRequest,};

use jsonwebtoken::{
    decode, errors::Error as JwtError, DecodingKey,
    TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

#[derive(Serialize, Deserialize)]
pub struct AuthenticationToken {
    pub id: String,
}

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // get auth token from the authorization header
        let auth_header: Option<&http::header::HeaderValue> =
            req.headers().get(http::header::AUTHORIZATION);
        let auth_token = auth_header.unwrap().to_str().unwrap_or("");
        if auth_token.is_empty() {
            return ready(Err(ErrorUnauthorized("Invalid Auth Token")));
        }

        let secret = &req.app_data::<web::Data<AppState>>().unwrap().secret;
        // decode token with the secret
        let decode: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
            &auth_token,
            &DecodingKey::from_secret(secret.as_str().as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        );
        // return authentication token
        match decode {
            Ok(token) => ready(Ok(AuthenticationToken {
                id: token.claims.id,
            })),
            Err(_) => ready(Err(ErrorUnauthorized("Failed Decoding Token"))),
        }
    }
}
