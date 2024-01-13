use actix_web::web;

use crate::handlers::note_handler::create_note_handler;

pub fn note_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(create_note_handler);

    conf.service(scope);
}
