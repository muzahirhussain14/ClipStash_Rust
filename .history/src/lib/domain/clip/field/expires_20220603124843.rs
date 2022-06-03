use crate::domain::Time;
use super::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Time(Option<String>);
