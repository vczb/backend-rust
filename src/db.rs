use axum::{Json, extract::State, http::StatusCode};
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};

use crate::types::{NewPerson, Person};

pub async fn connect() -> Result<Arc<Client>, tokio_postgres::Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=127.0.0.1 user=rust_user password=rust_password dbname=rust_database",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(Arc::new(client))
}

pub async fn query_people(
    State(client): State<Arc<Client>>,
) -> Result<Json<Vec<Person>>, StatusCode> {
    let rows = client
        .query(
            "SELECT id::text, nickname, name, birth_date, stack FROM people",
            &[],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let people = rows
        .into_iter()
        .map(|row| Person {
            id: row.get::<_, &str>("id").to_string(),
            nickname: row.get("nickname"),
            name: row.get("name"),
            birth_date: row.get("birth_date"),
            stack: row.get("stack"),
        })
        .collect();

    Ok(Json(people))
}

pub async fn insert_person(
    State(client): State<Arc<Client>>,
    Json(payload): Json<NewPerson>,
) -> Result<StatusCode, StatusCode> {
    client
        .execute(
            "INSERT INTO people (id, nickname, name, birth_date, stack)
           VALUES (uuid_generate_v4(), $1, $2, $3, $4)",
            &[
                &payload.nickname,
                &payload.name,
                &payload.birth_date,
                &payload.stack,
            ],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}
