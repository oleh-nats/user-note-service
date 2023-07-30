use actix_web::web::{delete, get, post, put, resource, ServiceConfig};

use self::{
    login::services::{basic_auth, create_user},
    note::services::{create_note, delete_note, fetch_notes, update_note},
};

pub mod login;
pub mod note;

pub fn auth_config(config: &mut ServiceConfig) {
    config
        .service(resource("/api/register").route(post().to(create_user)))
        .service(resource("/api/login").route(post().to(basic_auth)));
}

pub fn config(config: &mut ServiceConfig) {
    config
        .service(resource("/api/notes").route(post().to(create_note)))
        .service(resource("/api/notes").route(get().to(fetch_notes)))
        .service(resource("/api/notes/{id}").route(put().to(update_note)))
        .service(resource("/api/notes/{id}").route(delete().to(delete_note)));
}
