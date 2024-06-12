mod index;
mod args;

use axum::routing::get;
use axum::Router;
use clap::Parser;

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    let opts = args::Cli::parse();

    println!("Starting server at {:?}", &opts.address);

    let app = Router::new().route("/", get(index::index));
    let listener = tokio::net::TcpListener::bind(&opts.address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
