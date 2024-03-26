mod helper;
mod entity;
mod controller;
mod repository;
use controller::create_reservation::hello_world;
use helper::fallback::api_fallback;
use std::net::SocketAddr;
use axum::{Router, Server};

#[tokio::main]
async fn main() {
    let api: Router = Router::new().fallback(api_fallback);
    let app: Router = Router::new().nest("/api", api);
    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let server = Server::bind(&address).serve(app.into_make_service());
    hello_world();
    server.await.unwrap();
}
