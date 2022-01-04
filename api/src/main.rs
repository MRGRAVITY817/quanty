mod handlers;
use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_hello))
        .route("/news", get(handlers::news::get_news));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
