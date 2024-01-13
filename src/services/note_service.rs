use actix_web::web::Json;
use sqlx::{mysql::MySqlQueryResult, MySqlPool};

use crate::{
    models::note::{NoteDTO, NoteModel},
    repositories::note_repository,
    schemas::note::{CreateNoteSchema, UpdateNoteSchema},
    utils::dtos::format_note_model,
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

    pub async fn get_note_by_id(&self, note_id: &String) -> Result<NoteModel, sqlx::Error> {
        let note = note_repository::get_note_by_id(&note_id, self.pool.clone()).await?;

        Ok(note)
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

    pub async fn update_note(
        &self,
        body: Json<UpdateNoteSchema>,
        note: NoteModel,
        i8_published: i8,
        note_id: &String,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        let query_result =
            note_repository::update_note(body, note, i8_published, note_id, self.pool.clone())
                .await;

        Ok(query_result?)
    }

    pub async fn delete_note(&self, note_id: &String) -> Result<MySqlQueryResult, sqlx::Error> {
        let query_result = note_repository::delete_note(&note_id, self.pool.clone()).await;

        Ok(query_result?)
    }
}
