use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, Header};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Pool, Postgres};

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

pub fn create_tokens(id: i64) -> Result<SignInResponse, Box<dyn std::error::Error>> {
    let now = Utc::now();
    let exp = (now + Duration::minutes(5)).timestamp();
    let now = now.timestamp();

    let claims = AccessClaims {
        iss: "https://token.patisson.com/v1/token".to_owned(), // Issuer
        sub: id.to_string(),                                   // client_id
        aud: "https://auth.patisson.com/v1/authorization".to_owned(), // Recipient
        exp,
        nbf: now,
        iat: now,
        jti: "JWT-ID".to_owned(),
    };
    let access_token = encode(&Header::default(), &claims, &KEYS.encoding)?;

    Ok(SignInResponse::new(access_token))
}

pub async fn sign_in(
    State(pool): State<PgPool>,
    Json(dto): Json<SignIn>,
) -> Result<Json<SignInResponse>, AuthError> {
    if dto.username.is_empty() || dto.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let (id, _username, password_hash) = fetch_user_by_username(&pool, &dto.username)
        .await
        .map_err(|_| AuthError::WrongCredentials)?;
    if hash_sha256(&dto.password) != password_hash {
        return Err(AuthError::WrongCredentials);
    }

    let response = create_tokens(id).map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(response))
}

pub async fn sign_out(claims: AccessClaims) -> (StatusCode, String) {
    // TODO: Add implementation.
    tracing::debug!("Sign out. claims.sub: {}", claims.sub);
    (StatusCode::NOT_IMPLEMENTED, "Not implemented".to_string())
}

pub async fn get_profile(
    State(pool): State<PgPool>,
    claims: AccessClaims,
) -> Result<Json<Profile>, (StatusCode, String)> {
    // TODO: Add implementation.
    tracing::debug!("Get profile. claims.sub: {}", claims.sub);

    let user_id = claims.sub.parse().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("claims.sub is not integer"),
        )
    })?;

    let (id, username, _) = fetch_user_by_id(&pool, user_id).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("User not found."),
        )
    })?;

    Ok(Json(Profile { id, username }))
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
    tracing::debug!(
        "CHange password. claims.sub: {}, dto.password: {}",
        claims.sub,
        dto.password
    );

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
    fetch_user_by_id(&pool, id)
        .await
        .map(|(id, username, _password)| Json(User { id, username }))
        .map_err(|err| {
            tracing::error!("{err:?}");
            StatusCode::NOT_FOUND
        })
}

pub async fn fetch_user_by_id(
    pool: &Pool<Postgres>,
    id: i64,
) -> Result<(i64, String, String), Box<dyn std::error::Error>> {
    let row = sqlx::query_file!("./sql/get-user-by-id.sql", id)
        .fetch_one(pool)
        .await?;

    Ok((row.id, row.username, row.password_hash))
}

pub async fn fetch_user_by_username(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<(i64, String, String), Box<dyn std::error::Error>> {
    let row = sqlx::query_file!("./sql/get-user-by-username.sql", username)
        .fetch_one(pool)
        .await?;

    Ok((row.id, row.username, row.password_hash))
}

pub async fn sign_up(
    State(pool): State<PgPool>,
    Json(payload): Json<SignUp>,
) -> Result<(StatusCode, Json<Identity>), (StatusCode, String)> {
    let password_hash = hash_sha256(&payload.password);

    // TODO: Check 'username' in DB. StatusCode::CONFLICT

    sqlx::query_file!("./sql/create-user.sql", payload.username, password_hash)
        .fetch_one(&pool)
        .await
        .map(|row| (StatusCode::CREATED, Json(Identity { id: row.id })))
        .map_err(|err| {
            let reason = err.to_string();
            let pattern = "duplicate key value violates unique constraint";
            let code = if reason.contains(pattern) {
                StatusCode::CONFLICT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };

            (code, reason)
        })
}
