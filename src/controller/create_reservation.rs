use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::Extension;
use crate::entity::reservation::Reservation;
use crate::repository::reservation_repository::ReservationRepository;
use axum::extract::Request;
use serde_json::json;
use serde_json::Value;
use dotenv::dotenv;
use std::env;


pub async fn create_reservation(Json(req): Json<Reservation>) -> impl IntoResponse {
    let collection_name: String = env::var("COLLECTION_NAME").unwrap_or_default();
    let db_name: String = env::var("DATABASE_NAME").unwrap_or_default();
    let uri: String = env::var("MONGO_URL").unwrap_or_default();
    let repository = ReservationRepository::new(
        &uri,
        &db_name,
        &collection_name,
    ).await;
    let error: Value = json!({
        "error": {
        "code": 404,
        "message": "Route not found. Please check the URL and try again later."
        }
    });
    let success: Value = json!({
        "code":200,
        "message": "reservation create with success."
    });
    if let Ok(repository) = repository {
        if let Err(_) = repository.insert(req).await {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(error));
        }
        return (StatusCode::CREATED, Json(success));
    }
    (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
}
