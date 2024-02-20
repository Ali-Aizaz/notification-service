use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Ctx {
    pub email: String,
    pub token: String,
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct Claims {
    pub name: String,
    pub email: String,
    pub password: String,
    pub exp: usize,
}
