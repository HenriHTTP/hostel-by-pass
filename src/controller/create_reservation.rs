use crate::entity::reservation::Reservation;
use crate::repository::reservation_repository::ReservationRepository;
use crate::helper::message_json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;
use std::env;
use mongodb::error::Error;
use regex::Regex;


pub async fn create_reservation(Json(reservation): Json<Reservation>) -> impl IntoResponse {
    if let Err(error) = is_valid_reservation(&reservation).await {
        return error;
    }
    if let Err(error) = is_valid_email(&reservation).await {
        return error;
    }
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
            repository.insert(reservation).await.unwrap();
            let success: Value = message_json::send_message(200, "reservation create with success.").await;
            return (StatusCode::CREATED, Json(success));
        }
        Err(error) => {
            let error: Value = message_json::send_message_error(404, format!("failed:{}", &error.to_string())).await;
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(error));
        }
    }
}

async fn is_valid_reservation(reservation: &Reservation) -> Result<(), (StatusCode, Json<Value>)> {
    let required_fields: Vec<(&String, &str)> = vec![
        (&reservation.name, "name"),
        (&reservation.email, "email"),
        (&reservation.check_in_date, "check-in date"),
        (&reservation.check_out_date, "check-out date"),
    ];
    for (field, field_name) in required_fields {
        if field.is_empty() {
            let error_message: Value = message_json::send_message_error(400, format!("{} field is required.", field_name)).await;
            return Err((StatusCode::BAD_REQUEST, Json(error_message)));
        }
    }
    Ok(())
}

async fn is_valid_email(reservation: &Reservation) -> Result<(), (StatusCode, Json<Value>)> {
    let valid_email_pattern: Regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .unwrap();
    let email_is_valid: bool = valid_email_pattern.is_match(&reservation.email);
    if !email_is_valid {
        let error_message: Value = message_json::send_message_error(400, "this email is not valid try use a valid email!").await;
        return Err((StatusCode::BAD_REQUEST, Json(error_message)));
    }
    Ok(())
}
