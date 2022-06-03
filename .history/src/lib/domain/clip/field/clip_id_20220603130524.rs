use crate::data::DbId;
use derive_more::{Constructor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Constructor, Deserialize, Serialize)]