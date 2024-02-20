use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub type Result<T> = core::result::Result<T, CustomError>;

#[derive(Debug)]
pub enum CustomError {
    BadRequest,
    TaskNotFound,
    InternalServerError,
    AuthFailNoAuthToken,
    JWTTokenCreationError,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request"),
            Self::TaskNotFound => (StatusCode::NOT_FOUND, "Task Not Found"),
            Self::AuthFailNoAuthToken => (StatusCode::UNAUTHORIZED, "Auth Fail invalid Auth Token"),
            Self::JWTTokenCreationError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Jwt Creation Error")
            }
        };
        (status, Json(json!({"error": error_message}))).into_response()
    }
}
