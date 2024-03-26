use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn api_fallback() -> (StatusCode, Json<Value>) {
    let body_response: Value = json!({
        "error": {
        "code": 404,
        "message": "Route not found. Please check the URL and try again later."
        }
    });

    (StatusCode::NOT_FOUND, Json(body_response))
}