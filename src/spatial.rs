use axum::{
    response::IntoResponse,
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use validator::Validate;
use std::sync::Arc;

pub async fn get_spatial_handler(State(ctx): State<Arc<AppConfig>>, claim: Claim,) -> Result<IntoResponse, AppError> {
    claim
        .Validate()
        .map_err(|e| {});

    let result = query_as!(
        "SELECT user_id, username FROM users WHERE user_id = &1"
        &claim.sub
    )
        .fetch_optional(&ctx.db)
        .await?;

    match result {
        Some(data) => Ok(JSON(data)),
        None => {
            let new_result = query_as!(
                "INSERT INTO users (user_id, username)
                VALUES ($1, $2)
                RETURNING username",
                claim.sub,
                claim.username,
            )
                .fetch_one(&ctx.db)
                .await?;

                Ok(JSON(new_result))
        }
    }
}
