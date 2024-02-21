use axum::extract::Path;
use axum::http::StatusCode;

use axum::{Extension, Json};
use sqlx::PgPool;

use crate::ctx::Ctx;
use crate::models::message::{Message, NewMessage, UpdateMessage};
use crate::{CustomError, Result};

pub async fn all_messages(
    Extension(ctx): Extension<Ctx>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Message>>> {
    let sql = "SELECT * FROM message WHERE id = $1";

    let task = sqlx::query_as::<_, Message>(&sql)
        .bind(ctx.id)
        .fetch_all(&pool)
        .await
        .map_err(|_| CustomError::BadRequest)
        .unwrap();

    Ok(Json(task))
}

pub async fn new_message(
    Extension(ctx): Extension<Ctx>,
    Extension(pool): Extension<PgPool>,
    Json(message): Json<NewMessage>,
) -> Result<Json<NewMessage>> {
    if message.content.is_empty() {
        return Err(CustomError::BadRequest);
    }

    let sql = "INSERT INTO message (content, user_id) values ($1, $2)";

    sqlx::query(&sql)
        .bind(&message.content)
        .bind(ctx.id)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok(Json(message))
}

pub async fn update_message(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(message): Json<UpdateMessage>,
) -> Result<(StatusCode, Json<UpdateMessage>)> {
    let sql = "SELECT * FROM message where id=$1";

    let _find: Message = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::TaskNotFound)?;

    let _ = sqlx::query("UPDATE message SET content=$1 WHERE id=$2")
        .bind(&message.content)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::BadRequest)?;

    Ok((StatusCode::OK, Json(message)))
}
