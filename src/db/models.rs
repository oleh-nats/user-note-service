#![allow(unused)]
#![allow(clippy::all)]

use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}
