use crate::{
    db::messages::{CreateNote, DeleteNote, FetchNotes, UpdateNote},
    db::utils::{AppState, DbActor},
    error::AppResult,
};
use actix::Addr;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
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
pub async fn create_note(
    state: Data<AppState>,
    body: Json<CreateArticleBody>,
) -> AppResult<HttpResponse> {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let res = db
        .send(CreateNote {
            title: body.title.to_string(),
            content: body.content.to_string(),
            created_at: Some(Utc::now().naive_utc()),
        })
        .await??;

    Ok(HttpResponse::Ok().json(res))
}

#[get("/api/notes")]
pub async fn fetch_notes(state: Data<AppState>) -> AppResult<HttpResponse> {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let res = db.send(FetchNotes).await??;

    Ok(HttpResponse::Ok().json(res))
}

#[put("/api/notes/{id}")]
pub async fn update_note(
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<UpdateArticleBody>,
) -> AppResult<HttpResponse> {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    let id: i32 = path.into_inner();

    let res = db
        .send(UpdateNote {
            id,
            title: body.title.to_string(),
            content: body.content.clone(),
            created_at: Some(Utc::now().naive_utc()),
        })
        .await??;

    Ok(HttpResponse::Ok().json(res))
}

#[delete("/api/notes/{id}")]
pub async fn delete_note(state: Data<AppState>, path: Path<i32>) -> AppResult<HttpResponse> {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    let note_id: i32 = path.into_inner();

    let res = db.send(DeleteNote { id: note_id }).await??;

    Ok(HttpResponse::Ok().json(res))
}
