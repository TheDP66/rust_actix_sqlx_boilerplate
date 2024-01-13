use crate::models::note::{NoteDTO, NoteModel};

pub fn format_note_model(note: &NoteModel) -> NoteDTO {
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
