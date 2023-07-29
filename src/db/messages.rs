use super::models::{Note, User};
use actix::Message;
use chrono::NaiveDateTime;

#[derive(Message)]
#[rtype(result = "anyhow::Result<Vec<Note>>")]
pub struct FetchNotes;

#[derive(Message)]
#[rtype(result = "anyhow::Result<Note>")]
pub struct CreateNote {
    pub title: String,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<User>")]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<User>")]
pub struct GetUser {
    pub username: String,
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<Note>")]
pub struct GetNote {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<Note>")]
pub struct UpdateNote {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<usize>")]
pub struct DeleteNote {
    pub id: i32,
}
