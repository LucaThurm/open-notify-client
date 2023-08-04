use serde::Deserialize;

use crate::{error::OpenNotifyError, response::OpenNotifyResponse};

#[derive(Deserialize, Debug)]
pub struct ISSNowResponse {
    pub message: String,
    pub timestamp: u32,
    pub iss_position: ISSPosition,
}

#[derive(Deserialize, Debug)]
pub struct ISSPosition {
    pub latitude: String,
    pub longitude: String,
}

impl OpenNotifyResponse for ISSNowResponse {
    fn deserialize(s: &str) -> Result<Self, OpenNotifyError> {
        if let Ok(response) = serde_json::from_str(s) {
            return Ok(response);
        }

        Err(OpenNotifyError::DeserializationFailed)
    }
}
