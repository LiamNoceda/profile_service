use axum::{
    response::{IntoResponse, Response},
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::query_as;
use serde::Serialize;
use validator::Validate;
use std::sync::Arc;

#[derive(Serialize)]
pub struct UnitSpatinal {
    pub user_id: i64,
    pub username: String,
    pub object_count: i32,
}

pub async fn get_spatial_handler(State(ctx): State<Arc<AppConfig>>, claim: Claim,) -> Result<impl IntoResponse, AppError> {
    claim
        .Validate()
        .map_err(|e| {});

    let result = query_as!(
        UnitSpatinal,
        r#"
        INSERT INTO users (user_id, username, object_count)
        VALUES ($1, $2, 0)
        ON CONFLICT (user_id)
        DO UPDATE SET last_login = NOW()
        RETURNING user_id, username, object_count
        "#,
        claim.sub,
        claim.username
    )
    .fetch_one(&ctx.db)
    .await?;

    Ok((StatusCode::OK, Json(result)))
}
