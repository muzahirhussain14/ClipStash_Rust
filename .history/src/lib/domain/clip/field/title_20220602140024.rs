use super::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;


pub struct Ttile(Option<String>);