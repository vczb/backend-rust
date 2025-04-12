use axum::{Router, routing::get};

mod db;
use db::connect;

#[tokio::main]
async fn main() {
    // initialize DB connection
    let client = connect().await.expect("Failed to connect to DB");

    // you could store `client` in app state if needed later

    // build the app
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run the app
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
