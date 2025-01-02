use crate::handler;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};

pub fn v1(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(handler::handler))
        .route("/users", get(handler::get_user_list))
        .route("/users/{id}", get(handler::get_user_by_id))
        .route("/registration", post(handler::registration))
        .with_state(pool)
}
