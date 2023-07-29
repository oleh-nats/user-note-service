use super::insertables::{NewNote, NewUser};
use super::messages::{
    CreateNote, CreateUser, DeleteNote, FetchNotes, GetNote, GetUser, UpdateNote,
};
use super::models::{Note, User};
use super::schema::notes::{dsl::*, id as note_id};
use super::schema::users::{dsl::*, id as user_id};
use super::utils::DbActor;
use actix::Handler;
use anyhow::anyhow;
use diesel::{self, prelude::*};

impl Handler<FetchNotes> for DbActor {
    type Result = anyhow::Result<Vec<Note>>;

    fn handle(&mut self, _: FetchNotes, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;

        Ok(notes.get_results::<Note>(&mut conn)?)
    }
}

impl Handler<CreateNote> for DbActor {
    type Result = anyhow::Result<Note>;

    fn handle(&mut self, msg: CreateNote, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;
        let new_article = NewNote {
            title: msg.title,
            content: msg.content,
            created_at: msg.created_at,
        };

        let res = diesel::insert_into(notes)
            .values(new_article)
            .returning((note_id, title, content, created_at))
            .get_result::<Note>(&mut conn)?;

        Ok(res)
    }
}

impl Handler<CreateUser> for DbActor {
    type Result = anyhow::Result<User>;

    fn handle(&mut self, msg: CreateUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;

        let existing_user = users
            .filter(username.eq(&msg.username))
            .first::<User>(&mut conn)
            .optional()?;

        if existing_user.is_some() {
            return Err(anyhow!("User already exists"));
        }

        let new_user = NewUser {
            username: msg.username,
            password: msg.password,
        };

        Ok(diesel::insert_into(users)
            .values(new_user)
            .returning((user_id, username, password))
            .get_result::<User>(&mut conn)?)
    }
}

impl Handler<GetUser> for DbActor {
    type Result = anyhow::Result<User>;

    fn handle(&mut self, msg: GetUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;

        Ok(users
            .filter(username.eq(msg.username))
            .select((user_id, username, password))
            .first::<User>(&mut conn)?)
    }
}

impl Handler<GetNote> for DbActor {
    type Result = anyhow::Result<Note>;

    fn handle(&mut self, msg: GetNote, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;

        Ok(notes
            .filter(note_id.eq(msg.id))
            .select((note_id, title, content, created_at))
            .first::<Note>(&mut conn)?)
    }
}

impl Handler<DeleteNote> for DbActor {
    type Result = anyhow::Result<usize>;

    fn handle(&mut self, msg: DeleteNote, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;

        Ok(diesel::delete(notes.filter(note_id.eq(msg.id))).execute(&mut conn)?)
    }
}

impl Handler<UpdateNote> for DbActor {
    type Result = anyhow::Result<Note>;

    fn handle(&mut self, msg: UpdateNote, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;

        let res = match msg.content {
            Some(info) => diesel::update(notes.filter(note_id.eq(msg.id)))
                .set((
                    title.eq(msg.title),
                    content.eq(info),
                    created_at.eq(msg.created_at),
                ))
                .get_result::<Note>(&mut conn)?,
            None => diesel::update(notes.filter(note_id.eq(msg.id)))
                .set((title.eq(msg.title), created_at.eq(msg.created_at)))
                .get_result::<Note>(&mut conn)?,
        };

        Ok(res)
    }
}
