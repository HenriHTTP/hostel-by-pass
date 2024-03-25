use std::net::SocketAddr;
use axum::{Router, Server};
use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;
use serde_json::json;

#[tokio::main]
async fn main() {
    let api: Router = Router::new().fallback(api_fallback());
    let app: Router = Router::new().nest("/api", api);
    let address: SocketAddr =
        SocketAddr::from(([127, 0, 0, 1], 8080));

    let server = Server::bind(&address).serve(app.into_make_service());

    server.await.unwrap();
}

fn api_fallback() -> (StatusCode, Json<Value>) {
    let body_response = json!({
        "status": 404,
        "message": "route is not found,try again more late"
    });

    (StatusCode::NOT_FOUND, Json(body_response))
}