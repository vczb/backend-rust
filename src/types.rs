use serde::{Deserialize, Serialize};

#[derive(Serialize)]
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
