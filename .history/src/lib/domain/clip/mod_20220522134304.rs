pub mod field;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClipError {
    #[error("invalid password: {}")]
    InvalidPassword(String),
    InvalidTitle(String),
    EmptyContent,
    InvalidData(String),
    DateParse(#[from] chrono::ParseError),
    Id(#[from] uuid::Error),
    Hits(#[from] std::num::TryFromIntError)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clip {
    pub clip_id: field::ClipId,
    pub shortcode: field::Shortcode,
    pub content: field::Content,
    pub title:  field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits
}