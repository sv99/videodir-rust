use actix_web::{error, HttpResponse, Responder, web, web::Data};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web::{dev::ServiceRequest, Error};
use actix_web::http::header::ContentType;
use chrono::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::errors;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::htpasswd::Htpasswd;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
}

#[derive(Debug, Deserialize)]
pub struct Cred {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct Token {
    token: String,
}

pub async fn login(cred: web::Json<Cred>, conf: web::Data<Config>, passwd: web::Data<Htpasswd>) -> impl Responder {
    if passwd.check(&cred.username, &cred.password) {
        match create_jwt(&cred.username, &conf) {
            Ok(token) => {
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(serde_json::to_string(&Token { token }).unwrap())
            },
            Err(_err) => {
                HttpResponse::Unauthorized()
                    .body("Error create Jwt token")
            }
        }
    } else {
        HttpResponse::Unauthorized()
            .body("Unauthorized")
    }
}

fn create_jwt(uid: &str, conf: &Config) -> Result<String, errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_owned(),
        iat: Utc::now().timestamp() as u64,
        exp: expiration as u64,
    };
    let header = Header::new(Algorithm::HS384);
    encode(&header, &claims, &EncodingKey::from_secret(conf.jwt_secret.as_bytes()))
}

pub async fn bearer_jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req
        .app_data::<Data<Config>>()
        .unwrap();
    match validate_token(config, credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err((error::ErrorUnauthorized("token validation error"), req))
            }
        }
        Err(_) => Err((error::ErrorBadRequest("token error"), req)),
    }
}

fn validate_token(conf: &Data<Config>, token: &str) -> Result<bool, errors::Error>
{
    let validation = Validation::new(Algorithm::HS384);

    match decode::<Claims>(
        &token, &DecodingKey::from_secret(conf.jwt_secret.as_bytes()), &validation) {
        Ok(_) => Ok(true),
        Err(err) => Err(err),
    }
}