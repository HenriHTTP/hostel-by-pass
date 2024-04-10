use crate::entity::email_reservation::ReservationEmail;
use crate::repository::reservation_repository::ReservationRepository;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use serde_json::Value;
use std::env;
use mongodb::error::Error;

pub async fn get_reservation_by_email(Json(reservation_email): Json<ReservationEmail>) -> impl IntoResponse {
    let collection_name: String = env::var("COLLECTION_NAME").unwrap_or_default();
    let db_name: String = env::var("DATABASE_NAME").unwrap_or_default();
    let uri: String = env::var("MONGO_URL").unwrap_or_default();
    let repository: Result<ReservationRepository, Error> = ReservationRepository::new(
        &uri,
        &db_name,
        &collection_name,
    ).await;

    match repository {
        Ok(repository) => {
            let result_from_email: Vec<Value> = repository.get_reservation_from_email(reservation_email).await.unwrap();
            Ok((StatusCode::CREATED, Json(result_from_email)))
        }
        Err(error) => {
            let error_json: Value = json!({
                "error": {
                    "code": 404,
                    "message": error.to_string()
             }
            });
            Err((StatusCode::BAD_REQUEST, Json(error_json)))
        }
    }
}