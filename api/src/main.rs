mod handlers;
mod utils;

use handlers::news;
use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_hello))
        .route("/news", get(news::get_financial_news))
        .route("/disclosure", get(news::get_today_disclosure))
        .route("/ticker", get(news::get_ticker));
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
