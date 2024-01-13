use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct NoteModel {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub published: i8,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NoteDTO {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub published: bool,
    pub createdAt: chrono::DateTime<chrono::Utc>,
    pub updatedAt: chrono::DateTime<chrono::Utc>,
}

impl Into<NoteDTO> for NoteModel {
    fn into(self) -> NoteDTO {
        NoteDTO {
            id: self.id,
            title: self.title,
            content: self.content,
            category: self.category.unwrap(),
            published: self.published != 0,
            createdAt: self.created_at.unwrap(),
            updatedAt: self.updated_at.unwrap(),
        }
    }
}

impl Into<NoteModel> for NoteDTO {
    fn into(self) -> NoteModel {
        NoteModel {
            id: self.id,
            title: self.title,
            content: self.content,
            category: Some(self.category),
            published: if self.published { 1 } else { 0 },
            created_at: Some(self.createdAt),
            updated_at: Some(self.updatedAt),
        }
    }
}
