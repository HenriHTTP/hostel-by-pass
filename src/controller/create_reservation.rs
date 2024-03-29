use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::Extension;
use crate::entity::reservation::Reservation;
use crate::repository::reservation_repository::ReservationRepository;

pub async fn create_reservation(Json(reservation): Json<Reservation>, repository: Extension<ReservationRepository>, ) -> impl IntoResponse {
    match repository.insert(reservation).await {
        Ok(()) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
