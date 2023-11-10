use std::cmp::Ordering;

// use anyhow::{Ok, Result};
use axum::{
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Claims {
    pub id: u32,
    pub email: String,
    pub expire: Option<DateTime<Utc>>,
}

pub async fn create_token(mut claims: Claims) -> String {
    claims.expire = Some(Utc::now());
    let token = encode(&Header::default(), &claims, &KEYS.encoding);
    token.unwrap()
}

pub async fn verify_token<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // Ok(next.run(req).await)
    match req.headers().get(header::AUTHORIZATION) {
        Some(token) => {
            print!("{:?}", token);
            let token = token.to_str().unwrap_or("");
            let claims = decode::<Claims>(&token, &KEYS.decoding, &Validation::default());
            match claims {
                Ok(claims) => {
                    if Utc::now().cmp(&claims.claims.expire.unwrap()) == Ordering::Less {
                        Ok(next.run(req).await)
                    } else {
                        Err(StatusCode::UNAUTHORIZED)
                    }
                }
                _ => Err(StatusCode::UNAUTHORIZED),
            }
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
