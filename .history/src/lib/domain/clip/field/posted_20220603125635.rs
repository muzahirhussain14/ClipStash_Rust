use crate::domain::Time;
use super::ClipError;
use serde::{Deserialize, Serialize};
use derive_more::Constructor;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Posted(Option<Time>);

