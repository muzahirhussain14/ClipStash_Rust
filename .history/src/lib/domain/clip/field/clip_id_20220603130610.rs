use crate::data::DbId;
use derive_more::{Constructor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Constructor, Deserialize, Serialize)]
pub struct ClipId(DbId);

impl ClipId {
    pub fn into_inner(self) -> DbId {
        
    }
}