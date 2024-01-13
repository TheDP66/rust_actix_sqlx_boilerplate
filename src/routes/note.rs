use actix_web::web;

use crate::handlers::note_handler::{
    create_note_handler, delete_note, edit_note, get_note_by_id, note_list_handler,
};

pub fn note_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(create_note_handler)
        .service(note_list_handler)
        .service(get_note_by_id)
        .service(edit_note)
        .service(delete_note);

    conf.service(scope);
}
