mod helper;
mod entity;
use helper::fallback::api_fallback;
mod controller;
use controller::create_reservation::hello_world;
mod repository;
use std::net::SocketAddr;
use axum::Router;
use axum::Server;
use axum::routing::post;

#[tokio::main]
async fn main() {
    let api: Router = Router::new().fallback(api_fallback);
    let app: Router = Router::new().nest("/api", api);
    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let server = Server::bind(&address).serve(app.into_make_service());
    hello_world();
    server.await.unwrap();
}
