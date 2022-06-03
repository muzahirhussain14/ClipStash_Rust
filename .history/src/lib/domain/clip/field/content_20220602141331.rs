use super::ClipError;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content(String);


impl Content {
    pub fn new(content: &str) -> Result<Self, ClipError> {
        if !content.trim().is_empty() {
            Ok(Self(content.to_owned))
        } else {
            Err(ClipError::EmptyContent)
        }
    }

    pub into_inner(self) -> String {
        self.0
    }

    pub as_str(self) -> &str {
        self.0.as_str()
    } 
}