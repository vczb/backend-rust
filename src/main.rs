use axum::{Router, routing::get};

mod types;

mod db;
use db::{connect, count_people, insert_person, query_people, query_person_by_id};

#[tokio::main]
async fn main() {
    let client = connect().await.expect("Failed to connect to DB");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/people", get(query_people).post(insert_person))
        .route("/people/:id", get(query_person_by_id))
        .route("/count-people", get(count_people))
        .with_state(client.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
