mod index;

use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(index::index));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
