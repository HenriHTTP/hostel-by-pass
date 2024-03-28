pub fn hello_world() {
    println!("hello world, create_reservation module!!");
}

use axum::{http::{StatusCode}, response::IntoResponse, Json, Extension};
use crate::entity::reservation::Reservation;
use crate::repository::reservation_repository::ReservationRepository;

pub async fn create_reservation(
    Json(reservation): Json<Reservation>,
    repository: Extension<ReservationRepository>,
) -> impl IntoResponse {
    match repository.insert(reservation).await {
        Ok(()) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}