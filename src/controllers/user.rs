use crate::controllers::AUTH_TOKEN;
use crate::models::message::{Claims, LoginUser, NewUser, User};
use crate::{CustomError, Result};
use axum::body::Body;
use axum::http::{HeaderValue, StatusCode};
use axum::response::{self, IntoResponse};
use axum::{Extension, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use sqlx::PgPool;

use super::JWT_SECRET;

pub async fn signup(
    Extension(pool): Extension<PgPool>,
    Json(mut user): Json<NewUser>,
) -> Result<response::Response<Body>> {
    if user.email.is_empty() || user.password.is_empty() || user.name.is_empty() {
        return Err(CustomError::BadRequest);
    }

    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();

    let sql = r#"INSERT INTO "user"(name, email, password) values ($1, $2, $3)"#;

    sqlx::query(sql)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&hashed_password)
        .execute(&pool)
        .await
        .map_err(|op| {
            println!("{:?}", op);
            CustomError::InternalServerError
        })?;

    user.password = hashed_password.to_string();

    let token = create_jwt(&user.email, &user.name, &user.password).unwrap();

    // Create a response with the JWT in the Authorization header
    let response = response::Response::builder()
        .status(StatusCode::CREATED)
        .header(
            AUTH_TOKEN,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        )
        .body(Body::from("User created successfully"))
        .unwrap();

    Ok(response)
}

pub async fn login(
    Extension(pool): Extension<PgPool>,
    Json(user): Json<LoginUser>,
) -> impl IntoResponse {
    if user.email.is_empty() || user.password.is_empty() {
        return Err(CustomError::BadRequest);
    }

    let sql = r#"SELECT * FROM "user" WHERE email = $1"#;

    let selected_user = sqlx::query_as::<_, User>(sql)
        .bind(&user.email)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::BadRequest)
        .unwrap();

    let response = response::Response::builder();

    let token = create_jwt(
        &selected_user.email,
        &selected_user.name,
        &selected_user.password,
    )
    .unwrap();

    let response = match verify(&user.password, &selected_user.password) {
        Ok(true) => response
            .status(StatusCode::OK)
            .header(
                AUTH_TOKEN,
                HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            )
            .body(Body::from("Login Success")),
        Ok(false) => response
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Invalid Credentials")),
        Err(_) => response
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Server Error")),
    };

    Ok(response.unwrap())
}

pub fn create_jwt(email: &str, name: &str, password: &str) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        email: email.to_string(),
        name: name.to_string(),
        exp: expiration as usize,
        password: password.to_string(),
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| CustomError::JWTTokenCreationError)
}
