use std::net::SocketAddr;
use axum::{routing::Route, server};
use axum::http::StatusCode;
use serde_json::Value;
use serde_json::json;

#[tokio::main]
async fn main() {
    let api = Route::new().fallback(api_fallback);
    let app = Route::new().nest("/api", api);
    let address: SocketAddr =
        SocketAddr::from(([127, 0, 0, 1], 8080));

    let server = server::bind(&address).serve(app.into_make_service());

    server.await.unwrap();

}

fn api_fallback() -> (StatusCode, Json<Value>) {
    let body_response = json!({
        "status": 404,
        "message": "route is not found,try again more late"
    });

    (StatusCode::NOT_FOUND, Json(body))
}