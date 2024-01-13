use actix_web::web::Json;
use sqlx::{mysql::MySqlQueryResult, MySqlPool};

use crate::{
    models::note::{NoteDTO, NoteModel},
    repositories::note_repository,
    schemas::note::CreateNoteSchema,
};

#[derive(Debug)]
pub struct NoteService {
    pool: MySqlPool,
}

impl NoteService {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn create_note(
        &self,
        note_id: &String,
        body: Json<CreateNoteSchema>,
    ) -> Result<MySqlQueryResult, String> {
        let query_result = note_repository::insert_note(&note_id, &body, self.pool.clone()).await;
        Ok(query_result?)
    }

    pub async fn get_note_by_id(&self, note_id: &String) -> Result<NoteDTO, sqlx::Error> {
        let note = note_repository::get_note_by_id(&note_id, self.pool.clone()).await?;
        Ok(format_note_model(&note))
    }

    pub async fn get_list_note(
        &self,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<NoteDTO>, sqlx::Error> {
        let notes = note_repository::get_list_note(limit, offset, self.pool.clone()).await;

        let note_response = notes
            .unwrap()
            .iter()
            .map(|note| format_note_model(&note))
            .collect::<Vec<NoteDTO>>();

        Ok(note_response)
    }
}

fn format_note_model(note: &NoteModel) -> NoteDTO {
    NoteDTO {
        id: note.id.to_owned(),
        title: note.title.to_owned(),
        content: note.content.to_owned(),
        category: note.category.to_owned().unwrap(),
        published: note.published != 0,
        createdAt: note.created_at.unwrap(),
        updatedAt: note.updated_at.unwrap(),
    }
}
