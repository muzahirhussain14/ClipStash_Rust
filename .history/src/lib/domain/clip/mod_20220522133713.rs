pub mod field;

use serde::{Deserialize, Serialize};
use thiserror::Error;



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