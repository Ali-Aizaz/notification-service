use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Message {
    pub id: i64,
    pub content: String,
    pub user_id: i64,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct NewMessage {
    pub content: String,
    pub user_id: i64,
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct UpdateMessage {
    pub content: String,
    pub user_id: i64,
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct Claims {
    pub name: String,
    pub email: String,
    pub password: String,
    pub exp: usize,
}
