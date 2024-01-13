use sqlx::{mysql::MySqlQueryResult, MySqlPool};

use crate::{models::note::NoteModel, schemas::note::CreateNoteSchema};

pub async fn insert_note(
    note_id: &String,
    body: &CreateNoteSchema,
    pool: MySqlPool,
) -> Result<MySqlQueryResult, String> {
    let query_result = sqlx::query(
        r#"
        INSERT INTO notes (id, title, content, category)
        VALUES (?, ?, ?, ?)
    "#,
    )
    .bind(note_id.clone())
    .bind(body.title.to_string())
    .bind(body.content.to_string())
    .bind(body.category.to_owned().unwrap_or_default())
    .execute(&pool)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    Ok(query_result?)
}

pub async fn get_note_by_id(note_id: &String, pool: MySqlPool) -> Result<NoteModel, sqlx::Error> {
    let note = sqlx::query_as!(
        NoteModel,
        r#"
            SELECT * 
            FROM notes 
            WHERE id = ?
        "#,
        note_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(note)
}

pub async fn get_list_note(
    limit: usize,
    offset: usize,
    pool: MySqlPool,
) -> Result<Vec<NoteModel>, sqlx::Error> {
    let notes = sqlx::query_as!(
        NoteModel,
        r#"
        SELECT * FROM notes ORDER by id LIMIT ? OFFSET ? 
    "#,
        limit as i32,
        offset as i32,
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Ok(notes)
}
