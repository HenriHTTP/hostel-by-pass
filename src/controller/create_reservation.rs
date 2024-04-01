use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::Extension;
use crate::entity::reservation::Reservation;
use crate::repository::reservation_repository::ReservationRepository;
use axum::extract::Request;
use serde_json::json;
use serde_json::Value;

pub async fn create_reservation(Json(req): Json<Reservation>) -> impl IntoResponse {
    let repository = ReservationRepository::new(
        "mongodb://localhost:27021",
        "hostel-by-pass",
        "reservations",
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
