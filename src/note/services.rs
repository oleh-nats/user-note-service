use crate::{
    db::messages::{CreateNote, DeleteNote, FetchNotes, UpdateNote},
    db::utils::{AppState, DbActor},
};
use actix::Addr;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use chrono::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateArticleBody {
    pub title: String,
    pub content: Option<String>,
}

#[post("/api/notes")]
pub async fn create_note(state: Data<AppState>, body: Json<CreateArticleBody>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db
        .send(CreateNote {
            title: body.title.to_string(),
            content: body.content.to_string(),
            created_at: Some(Utc::now().naive_utc()),
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create note"),
    }
}

#[get("/api/notes")]
pub async fn fetch_notes(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchNotes).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No notes found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve notes"),
    }
}

#[put("/api/notes/{id}")]
pub async fn update_note(
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<UpdateArticleBody>,
) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    let id: i32 = path.into_inner();

    match db
        .send(UpdateNote {
            id,
            title: body.title.to_string(),
            content: body.content.clone(),
            created_at: Some(Utc::now().naive_utc()),
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to update note"),
    }
}

#[delete("/api/notes/{id}")]
pub async fn delete_note(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    let note_id: i32 = path.into_inner();

    match db.send(DeleteNote { id: note_id }).await {
        Ok(Ok(info)) => match info {
            0 => HttpResponse::InternalServerError().json("Nothing to delete"),
            _ => HttpResponse::Ok().json("Note deleted"),
        },
        _ => HttpResponse::InternalServerError().json("Failed to delete note"),
    }
}
