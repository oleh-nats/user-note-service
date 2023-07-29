use super::schema::{notes, users};
use chrono::NaiveDateTime;
use diesel::Insertable;

#[derive(Insertable, Clone)]
#[diesel(table_name=notes)]
pub struct NewNote {
    pub title: String,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Clone)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}
