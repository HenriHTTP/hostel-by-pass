use crate::helper::message_json;
use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

pub async fn api_fallback() -> (StatusCode, Json<Value>) {
    let error: Value = message_json::send_message_error(404, "Route not found. Please check the URL and try again later.").await;
    (StatusCode::NOT_FOUND, Json(error))
}