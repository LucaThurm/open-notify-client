use serde::Deserialize;

use crate::{error::OpenNotifyError, response::OpenNotifyResponse};

#[derive(Deserialize, Debug)]
pub struct AstrosResponse {
    pub message: String,
    pub number: u32,
    pub people: Box<Vec<Astro>>,
}

#[derive(Deserialize, Debug)]
pub struct Astro {
    pub name: String,
    pub craft: String,
}

impl OpenNotifyResponse for AstrosResponse {
    fn deserialize(s: &str) -> Result<Self, OpenNotifyError> {
        if let Ok(response) = serde_json::from_str(s) {
            return Ok(response);
        }

        Err(OpenNotifyError::DeserializationFailed)
    }
}
