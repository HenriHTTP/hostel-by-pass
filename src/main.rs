mod helper;
mod entity;
mod controller;
mod repository;
mod route;

use route::routes::routes;
use axum::Router;
use axum::serve;
use tokio::net::TcpListener;
use dotenv::dotenv;
use std::env;
use axum::serve::Serve;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let host: String = env::var("HOST").unwrap_or_default();
    let port: String = env::var("PORT").unwrap_or_default();
    let addr: String = format!("{}:{}", host, port);
    let app: Router = Router::new().nest("/api", routes());
    let address: TcpListener = TcpListener::bind(&addr).await.unwrap();
    let server_app: Serve<Router, Router> = serve(address, app);
    println!("app listening at http://localhost:{}", &port);
    server_app.await.unwrap();
}
