use crate::entity::email_reservation::ReservationEmail;
use crate::repository::reservation_repository::ReservationRepository;
use crate::service::email;
use crate::helper::message_json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};
use std::env;
use mongodb::error::Error;

pub async fn get_reservation_by_email(Json(reservation_email): Json<ReservationEmail>) -> impl IntoResponse {
    let collection_name: String = env::var("COLLECTION_NAME").unwrap_or_default();
    let db_name: String = env::var("DATABASE_NAME").unwrap_or_default();
    let uri: String = env::var("MONGO_URL").unwrap_or_default();
    let repository = match ReservationRepository::new(
        &uri,
        &db_name,
        &collection_name,
    ).await {
        Ok(repository) => repository,
        Err(error) => {
            let error_json: Value = message_json::send_message_error(400, &error.to_string().as_str()).await;
            return Err((StatusCode::BAD_REQUEST, Json(error_json)));
        }
    };
    let email: &String = &reservation_email.email;
    if let Err(error) = email::is_valid_email(&email).await {
        return Err(error);
    }
    let result_from_email: Vec<Value> = match repository.get_reservation_from_email(reservation_email).await {
        Ok(content_from_query) => content_from_query,
        Err(error) => {
            let error_json: Value = message_json::send_message_error(500, &error.to_string().as_str()).await;
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_json)));
        }
    };
    Ok((StatusCode::CREATED, Json(json!({"content": result_from_email}))))
}