pub mod astros;
pub mod error;
pub mod iss_now;

mod response;

use astros::AstrosResponse;
use error::OpenNotifyError;
use iss_now::ISSNowResponse;
use response::OpenNotifyResponse;

const ISS_NOW_URL: &str = "http://api.open-notify.org/iss-now.json";
const ASTROS_URL: &str = "http://api.open-notify.org/astros.json";

pub async fn iss_now() -> Result<ISSNowResponse, OpenNotifyError> {
    get::<ISSNowResponse>(ISS_NOW_URL).await
}

pub async fn astros() -> Result<AstrosResponse, OpenNotifyError> {
    get::<AstrosResponse>(ASTROS_URL).await
}

async fn get<T: OpenNotifyResponse>(url: &str) -> Result<T, OpenNotifyError> {
    let response = reqwest::get(url).await;

    if let Ok(response) = response {
        if let Ok(body) = response.text().await {
            return Ok(T::deserialize(&body)?);
        }

        return Err(OpenNotifyError::DeserializationFailed);
    }

    Err(OpenNotifyError::RequestFailed)
}
