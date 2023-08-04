use example::error::OpenNotifyError;
use example::iss_now;

#[tokio::main]
async fn main() {
    match iss_now().await {
        Ok(resp) => println!("{:?}", resp),
        Err(err) => match err {
            OpenNotifyError::RequestFailed => println!("Request Failed"),
            OpenNotifyError::DeserializationFailed => println!("Deserialization Failed"),
        },
    }
}
