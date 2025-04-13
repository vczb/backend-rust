use axum::{Router, routing::get};

mod types;

mod db;
use db::{connect, insert_person, query_people};

#[tokio::main]
async fn main() {
    let client = connect().await.expect("Failed to connect to DB");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/people", get(query_people).post(insert_person))
        .with_state(client.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
