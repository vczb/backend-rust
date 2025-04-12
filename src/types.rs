use serde::Serialize;

#[derive(Serialize)]
pub struct Person {
    pub id: String,
    pub nickname: String,
    pub name: Option<String>,
    pub birth_date: Option<String>,
    pub stack: Option<String>,
}
