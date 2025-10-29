use axum::{Router, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod server;

#[tokio::main]
async fn main() {
    let app = server::routes::create_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);

    serve(listener, app).await.unwrap();
}