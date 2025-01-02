use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::api::{Identity, Pagination, Registration, User};

pub async fn handler(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

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
                    password_hash: record.password_hash,
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
            password_hash: record.password_hash.clone(),
        }) {
        Ok(user) => Ok(Json(user)),
        Err(err) => {
            tracing::error!("{err:?}");
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn registration(
    State(pool): State<PgPool>,
    Json(payload): Json<Registration>,
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
