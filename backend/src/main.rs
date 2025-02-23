use axum::{http::StatusCode, routing::get, Router};

mod domain;
mod interface;

use interface::graphql;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route(
            "/",
            get(graphql::playground).post_service(graphql::service().await),
        )
        .route("/health", get(|| async { StatusCode::OK }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
