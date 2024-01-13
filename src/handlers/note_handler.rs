use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    models::note::NoteDTO, schemas::note::CreateNoteSchema, services::note_service::NoteService,
    AppState,
};

#[post("/notes")]
async fn create_note_handler(
    body: web::Json<CreateNoteSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let note_service = NoteService::new(data.db.clone());

    let note_id = uuid::Uuid::new_v4().to_string();

    if let Err(err) = note_service.create_note(&note_id, body).await {
        if err.contains("Duplicate entry") {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "status": "fail",
                "message":"Note with that title already exists"
            }));
        }

        return HttpResponse::InternalServerError().json(serde_json::json!({
            "status":"error",
            "message": format!("{:?}", err)
        }));
    }

    match note_service.get_note_by_id(&note_id).await {
        Ok(note) => {
            let note_dto: NoteDTO = note.into();

            let note_response = serde_json::json!({
                "status": "success",
                "data" : serde_json::json!({
                    "note": note_dto
                })
            });

            HttpResponse::Ok().json(note_response)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": format!("{:?}", e)
        })),
    }

    // let query_result = insert_note(&body, &data);

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
    // }

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

// #[get("/note/{id}")]
// async fn get_note_by_id(data: web::Data<AppState>) -> impl Responder {}
