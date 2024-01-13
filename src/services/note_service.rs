use actix_web::web::Json;
use sqlx::{mysql::MySqlQueryResult, MySqlPool};

use crate::{
    models::note::NoteDTO, repositories::note_repository, schemas::note::CreateNoteSchema,
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

        // if let Err(err) = query_result.await {
        //     if err.contains("Duplicate entry") {
        //         return HttpResponse::BadRequest().json(serde_json::json!({
        //             "status": "fail",
        //             "message":"Note with that title already exists"
        //         }));
        //     }

        //     return HttpResponse::InternalServerError().json(serde_json::json!({
        //         "status":"error",
        //         "message": format!("{:?}", err)
        //     }));
        // };

        // let query_result = note_repository::get_note_by_id(note_id, self.pool.clone());
        // match query_result.await {
        //     Ok(note) => {
        //         let note_response = serde_json::json!({
        //             "status": "success",
        //             "data" : serde_json::json!({
        //                 "note": note
        //             })
        //         });

        //         HttpResponse::Ok().json(note_response)
        //     }
        //     Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
        //         "status": "error",
        //         "message": format!("{:?}", e)
        //     })),
        // }
    }

    pub async fn get_note_by_id(&self, note_id: &String) -> Result<NoteDTO, sqlx::Error> {
        let note = note_repository::get_note_by_id(&note_id, self.pool.clone()).await?;

        Ok(note.into())

        // let query_result = get_note_by_id(&note_id, &data);

        // match query_result.await {
        //     Ok(note) => {
        //         let note_dto: NoteDTO = note.into();

        //         let note_response = serde_json::json!({
        //             "status": "success",
        //             "data" : serde_json::json!({
        //                 "note": note_dto
        //             })
        //         });

        //         HttpResponse::Ok().json(note_response)
        //     }
        //     Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
        //         "status": "error",
        //         "message": format!("{:?}", e)
        //     })),
        // }
    }
}
