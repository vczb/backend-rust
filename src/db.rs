use axum::{
    Json,
    extract::{FromRequest, Path, Query, State},
    http::{StatusCode, Uri},
};
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};
use uuid::Uuid;

use crate::types::{CountPerson, NewPerson, Person, PersonQueryParams};

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
    uri: Uri,
    State(client): State<Arc<Client>>,
) -> Result<Json<Vec<Person>>, StatusCode> {
    println!("URI {:?}", uri);

    let result: Query<PersonQueryParams> = Query::try_from_uri(&uri).unwrap();

    let mut query = String::from("SELECT id::text, nickname, name, birth_date, stack FROM people");
    let mut params = Vec::new();
    let mut param_idx = 1;
    let mut has_where = false;

    if result.nickname.is_some() || result.name.is_some() || result.stack.is_some() {
        query.push_str(" WHERE");
        has_where = true;
    }

    if let Some(nickname) = &result.nickname {
        query.push_str(&format!(" nickname = ${}", param_idx));
        params.push(nickname);
        param_idx += 1;
    }

    if let Some(name) = &result.name {
        if param_idx > 1 {
            query.push_str(" AND");
        }
        query.push_str(&format!(" name = ${}", param_idx));
        params.push(name);
        param_idx += 1;
    }

    let mut literals = Vec::new(); // New vector to store owned strings

    if let Some(stack) = &result.stack {
        if param_idx > 1 {
            query.push_str(" AND");
        }
        query.push_str(&format!(" stack LIKE ${}", param_idx));
        let wildcard_stack = format!("%{}%", stack);
        literals.push(wildcard_stack); // Store owned string in literals
        let last_lit = literals.last().unwrap(); // Get a reference to the last inserted item
        params.push(last_lit); // Push the reference to params
        // Don't need to increment param_idx here as it's the last parameter
    }

    println!("Query: {}", query);

    let rows = if has_where {
        let mut param_refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        for param in &params {
            param_refs.push(param as &(dyn tokio_postgres::types::ToSql + Sync));
        }

        client
            .query(&query, &param_refs[..])
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        client
            .query(
                "SELECT id::text, nickname, name, birth_date, stack FROM people",
                &[],
            )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

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

pub async fn count_people(
    State(client): State<Arc<Client>>,
) -> Result<Json<CountPerson>, StatusCode> {
    // Perform the query and get the first row
    let row = client
        .query_one("SELECT COUNT(*) AS count FROM people;", &[])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Extract the count from the row
    let count: i64 = row
        .try_get("count")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create a CountPerson instance with the extracted count
    let count_person = CountPerson { count };

    // Return the count as JSON
    Ok(Json(count_person))
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

pub async fn query_person_by_id(
    Path(id_str): Path<String>,
    State(client): State<Arc<Client>>,
) -> Result<Json<Person>, StatusCode> {
    // Parse the string as UUID
    let uuid = Uuid::parse_str(&id_str).map_err(|_| {
        eprintln!("Invalid UUID format: {}", id_str);
        StatusCode::BAD_REQUEST
    })?;

    let row = client
        .query_one(
            "SELECT id::text, nickname, name, birth_date, stack FROM people WHERE id = $1",
            &[&uuid],
        )
        .await
        .map_err(|err| {
            eprintln!("Database query error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let person = Person {
        id: row.get::<_, &str>("id").to_string(),
        nickname: row.get("nickname"),
        name: row.get("name"),
        birth_date: row.get("birth_date"),
        stack: row.get("stack"),
    };

    Ok(Json(person))
}
