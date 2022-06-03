use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::str::FromStr;

#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);


impl DbId {
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    pub fn nil() ->DbId {
        Self(Uuid::nil())
    }
}