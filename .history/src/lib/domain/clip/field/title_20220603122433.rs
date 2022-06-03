use super::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T: Into<Option<String>>>(title: T) -> Self {
        let title: Option<String> = title.into();

        match title {
            Some(title) => {
                if !title.trim().is_empty() {
                    Ok(Self(Some(title)))
                } else {
                    Ok(Self(None))
                }
            }
            None => Ok(Self(None))
        }
    }
}