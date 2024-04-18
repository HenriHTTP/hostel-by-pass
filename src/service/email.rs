use axum::http::StatusCode;
use axum::Json;
use regex::Regex;
use serde_json::Value;
use crate::helper::message_json;

pub async fn is_valid_email(email: &String) -> Result<(), (StatusCode, Json<Value>)> {
    let valid_email_pattern: Regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .unwrap();
    let email_is_valid: bool = valid_email_pattern.is_match(&email);
    if !email_is_valid {
        let error_message: Value = message_json::send_message_error(400, "this email is not valid try use a valid email!").await;
        return Err((StatusCode::BAD_REQUEST, Json(error_message)));
    }
    Ok(())
}
