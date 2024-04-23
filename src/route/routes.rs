use axum::Router;
use axum::routing::post;
use crate::controller::create_reservation::create_reservation;
use crate::controller::get_reservation_email::get_reservation_email;
use crate::controller::get_reservation_check_in_date::get_reservation_check_in_date;
use crate::controller::get_reservation_check_out_date::get_reservation_check_out_date;
use crate::helper::fallback::api_fallback;


pub fn routes () -> Router{
    Router::new()
        .route("/create_reservation",post(create_reservation))
        .route("/email", post(get_reservation_email))
        .route("/check_in_date", post(get_reservation_check_in_date))
        .route("/check_out_date", post(get_reservation_check_out_date))
        .fallback(api_fallback)
}