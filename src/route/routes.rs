use axum::Router;
use axum::routing::post;
use crate::controller::create_reservation::create_reservation;
use crate::helper::fallback::api_fallback;


pub fn routes () -> Router{
    Router::new()
        .route("/create_reservation",post(create_reservation))
        .fallback(api_fallback)
}