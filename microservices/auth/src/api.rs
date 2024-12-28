use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct Registration {
    pub username: String,
    pub password: String,
}

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
