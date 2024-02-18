use crate::errors::CustomError;
use crate::models::message::{LoginUser, NewUser, User};
use axum::body::Body;
use axum::http::{HeaderValue, StatusCode};
use axum::response::{self, IntoResponse};
use axum::{Extension, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;

pub async fn new_user(
    Extension(pool): Extension<PgPool>,
    Json(mut user): Json<NewUser>,
) -> Result<response::Response<Body>, CustomError> {
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

    let key = "secret".as_ref();
    let token = encode(&Header::default(), &user, &EncodingKey::from_secret(key)).unwrap();

    // Create a response with the JWT in the Authorization header
    let response = response::Response::builder()
        .status(StatusCode::CREATED)
        .header(
            "Authorization",
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

    let key = "secret".as_ref();
    let token = encode(
        &Header::default(),
        &selected_user,
        &EncodingKey::from_secret(key),
    )
    .unwrap();

    let response = match verify(&user.password, &selected_user.password) {
        Ok(true) => response
            .status(StatusCode::OK)
            .header(
                "Authorization",
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
