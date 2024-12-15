-- Add up migration script here
CREATE TABLE IF NOT EXISTS users
(
    id            BIGSERIAL PRIMARY KEY NOT NULL,
    username      VARCHAR(30) UNIQUE NOT NULL,
    password_hash VARCHAR(64)        NOT NULL
);
