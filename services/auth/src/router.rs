use crate::handler;
use axum::{
    routing::{get, patch, post},
    Router,
};
use sqlx::{Pool, Postgres};

pub fn v1(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/auth/sign-up", post(handler::sign_up))
        .route("/auth/sign-in", post(handler::sign_in))
        .route("/auth/sign-out", post(handler::sign_out))
        .route("/auth/profile", get(handler::get_profile))
        .route("/auth/profile", patch(handler::update_profile))
        .route("/auth/change-password", post(handler::change_password))
        .route("/users", get(handler::get_user_list))
        .route("/users/{id}", get(handler::get_user_by_id))
        .with_state(pool)
}
