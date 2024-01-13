use actix_web::{get, patch, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    schemas::{
        filter_options::FilterOptions,
        note::{CreateNoteSchema, UpdateNoteSchema},
    },
    services::note_service::NoteService,
    utils::dtos::format_note_model,
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

#[get("/note/{id}")]
async fn get_note_by_id(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let note_id = path.into_inner().to_string();

    let note_service = NoteService::new(data.db.clone());

    match note_service.get_note_by_id(&note_id).await {
        Ok(note) => HttpResponse::Ok().json(serde_json::json!({
            "status":"success",
            "data": note
        })),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "status":"fail",
            "message":format!("Note with ID {} not found", note_id)
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status":"error",
            "message": format!("{:?}",e)
        })),
    }
}

#[patch("/note/{id}")]
async fn edit_note(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateNoteSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let note_id = path.into_inner().to_string();

    let note_service = NoteService::new(data.db.clone());

    let note = match note_service.get_note_by_id(&note_id).await {
        Ok(note) => note,
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "status":"fail",
                "message": format!("Note with ID: {} not found", note_id)
            }))
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("{:?}", e)
            }))
        }
    };

    let published = body.published.unwrap_or(note.published != 0);
    let i8_published = published as i8;

    match note_service
        .update_note(body, note, i8_published, &note_id)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let message = format!("Note with ID: {} not found", note_id);
                return HttpResponse::NotFound().json(json!({
                    "status":"fail",
                    "message": message
                }));
            }
        }
        Err(e) => {
            let message = format!("Internal server error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status":"error",
                "message": message
            }));
        }
    };

    match note_service.get_note_by_id(&note_id).await {
        Ok(note) => {
            let note_response = serde_json::json!({"status":"success", "data":serde_json::json!({
                "note": format_note_model(&note)
            })});

            HttpResponse::Ok().json(note_response)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status":"error",
            "message":format!("{:?}", e)
        })),
    }
}
