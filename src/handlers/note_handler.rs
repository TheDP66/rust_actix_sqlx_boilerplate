use std::os::unix::raw::off_t;

use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{
    models::note::NoteDTO,
    schemas::{filter_options::FilterOptions, note::CreateNoteSchema},
    services::note_service::{self, NoteService},
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
        Ok(note_dto) => {
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
}

#[get("/notes")]
async fn note_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let note_service = NoteService::new(data.db.clone());

    match note_service.get_list_note(limit, offset).await {
        Ok(notes) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "data": serde_json::json!({
            "result": notes.len(),
            "notes": notes
            })
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": format!("{:?}", e)
        })),
    }
}

// #[get("/note/{id}")]
// async fn get_note_by_id(
//     opts: web::Query<FilterOptions>,
//     data: web::Data<AppState>,
// ) -> impl Responder {

//     match note_service.get_note_by_id(note_id) {}
// }
