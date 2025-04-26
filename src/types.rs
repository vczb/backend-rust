use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PersonQueryParams {
    pub id: Option<String>,
    pub nickname: Option<String>,
    pub stack: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Person {
    pub id: String,
    pub nickname: String,
    pub name: Option<String>,
    pub birth_date: Option<String>,
    pub stack: Option<String>,
}

#[derive(Deserialize)]
pub struct NewPerson {
    pub nickname: String,
    pub name: Option<String>,
    pub birth_date: Option<String>,
    pub stack: Option<String>,
}

#[derive(Serialize)]
pub struct CountPerson {
    pub count: i64,
}
