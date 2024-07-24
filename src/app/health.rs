use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

pub async fn check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"message": "Healthy"})))
}
