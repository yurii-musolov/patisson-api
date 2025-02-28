use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, Header};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::api::{
    AccessClaims, AuthError, ChangePassword, Identity, Pagination, Profile, ProfileUpdate, SignIn,
    SignInResponse, SignUp, User, KEYS,
};

/// Hash function for SHA256.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let str = "password";
/// let expected = "5E884898DA28047151D0E56F8DC6292773603D0D6AABBDD62A11EF721D1542D8";
///
/// let hash = hash_sha256(str);
///
/// assert_eq!(hash, expected);
/// ```
pub fn hash_sha256(str: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(str);
    format!("{:X}", hasher.finalize())
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    tracing::error!("{err:?}");

    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub fn create_tokens(username: String) -> Result<SignInResponse, Box<dyn std::error::Error>> {
    let now = Utc::now();
    let exp = (now + Duration::minutes(5)).timestamp();
    let now = now.timestamp();


    let claims = AccessClaims {
        iss: "https://token.patisson.com/v1/token".to_owned(), // Issuer
        sub: username,                                         // client_id
        aud: "https://auth.patisson.com/v1/authorization".to_owned(), // Recipient
        exp,
        nbf: now,
        iat: now,
        jti: "JWT-ID".to_owned(),
    };
    let access_token = encode(&Header::default(), &claims, &KEYS.encoding)?;

    Ok(SignInResponse::new(access_token))
}

pub async fn sign_in(Json(dto): Json<SignIn>) -> Result<Json<SignInResponse>, AuthError> {
    if dto.username.is_empty() || dto.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    if dto.username != "nemo" || dto.password != "password" {
        return Err(AuthError::WrongCredentials);
    }

    let response = create_tokens(dto.username).map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(response))
}

pub async fn sign_out(claims: AccessClaims) -> (StatusCode, String) {
    // TODO: Add implementation.
    tracing::debug!("Sign out. claims.sub: {}", claims.sub);
    (StatusCode::NOT_IMPLEMENTED, "Not implemented".to_string())
}

pub async fn get_profile(claims: AccessClaims) -> Result<Json<Profile>, (StatusCode, String)> {
    // TODO: Add implementation.
    tracing::debug!("Get profile. claims.sub: {}", claims.sub);

    let response = Profile {
        id: 123456789,
        username: "NOT_IMPLEMENTED".to_string(),
    };
    Ok(Json(response))
}

pub async fn update_profile(
    claims: AccessClaims,
    Json(dto): Json<ProfileUpdate>,
) -> (StatusCode, String) {
    // TODO: Add implementation.
    tracing::debug!(
        "Update profile. claims.sub: {}, username: {:?}",
        claims.sub,
        dto.username
    );

    (StatusCode::NOT_IMPLEMENTED, "Not implemented".to_string())
}

pub async fn change_password(
    claims: AccessClaims,
    Json(dto): Json<ChangePassword>,
) -> (StatusCode, String) {
    // TODO: Add implementation.
    tracing::debug!("CHange password. claims.sub: {}", claims.sub);

    (StatusCode::NOT_IMPLEMENTED, "Not implemented".to_string())
}

pub async fn get_user_list(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let offset = pagination.get_offset();
    let limit = pagination.get_limit();

    match sqlx::query_file!("./sql/get-user-list.sql", limit, offset)
        .fetch_all(&pool)
        .await
        .map(|records| {
            records
                .into_iter()
                .map(|record| User {
                    id: record.id,
                    username: record.username,
                })
                .collect()
        })
        .map_err(internal_error)
    {
        Ok(users) => Ok(Json(users)),
        Err((status, _reason)) => Err(status),
    }
}

pub async fn get_user_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<User>, StatusCode> {
    match sqlx::query_file!("./sql/get-user-by-id.sql", id)
        .fetch_one(&pool)
        .await
        .map(|record| User {
            id: record.id,
            username: record.username.clone(),
        }) {
        Ok(user) => Ok(Json(user)),
        Err(err) => {
            tracing::error!("{err:?}");
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn sign_up(
    State(pool): State<PgPool>,
    Json(payload): Json<SignUp>,
) -> Result<Json<Identity>, (StatusCode, String)> {
    let password_hash = hash_sha256(&payload.password);

    match sqlx::query_file!("./sql/create-user.sql", payload.username, password_hash)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => {
            // TODO: StatusCode::CREATED
            Ok(Json(Identity { id: row.id }))
        }
        Err(err) => {
            let reason = err.to_string();
            let pattern = "duplicate key value violates unique constraint";
            let code = if reason.contains(pattern) {
                StatusCode::CONFLICT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };

            Err((code, reason))
        }
    }
}
