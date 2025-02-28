use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{
        authorization::{Bearer, Credentials},
        Authorization,
    },
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;
use std::sync::LazyLock;

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = String::from("JWT_SECRET");

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

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: i64,
    pub nbf: i64,
    pub iat: i64,
    pub jti: String,
}

impl Display for AccessClaims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Issuer: {}\nRecipient: {}", self.iss, self.aud)
    }
}

impl<S> FromRequestParts<S> for AccessClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let mut validator = Validation::default();
        validator.set_audience(&["https://auth.patisson.com/v1/authorization"]);
        validator.set_issuer(&["https://token.patisson.com/v1/token"]);

        let token_data = decode::<AccessClaims>(bearer.token(), &KEYS.decoding, &validator)
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

// Route groupe /auth/*
#[derive(Deserialize)]
pub struct SignUp {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignIn {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SignInResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
}
impl SignInResponse {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token: access_token.clone(),
            token_type: Bearer::SCHEME.to_string(),
            expires_in: 0,
            refresh_token: access_token,
            scope: "update openid profile email".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct Profile {
    pub id: i64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct ProfileUpdate {
    pub username: String,
}

#[derive(Deserialize)]
pub struct ChangePassword {
    pub password: String,
}

// Route groupe /users/*
#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

// Common.
pub const DEFAULT_OFFSET: i64 = 0;
pub const DEFAULT_LIMIT: i64 = 10;

#[derive(Deserialize)]
pub struct Pagination {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl Pagination {
    pub fn get_offset(&self) -> i64 {
        self.offset.unwrap_or(DEFAULT_OFFSET)
    }
    pub fn get_limit(&self) -> i64 {
        self.limit.unwrap_or(DEFAULT_LIMIT)
    }
}

#[derive(Serialize)]
pub struct Identity {
    pub id: i64,
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
