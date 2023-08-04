use example::astros;
use example::error::OpenNotifyError;

#[tokio::main]
async fn main() {
    match astros().await {
        Ok(resp) => println!("{:?}", resp),
        Err(err) => match err {
            OpenNotifyError::RequestFailed => println!("Request Failed"),
            OpenNotifyError::DeserializationFailed => println!("Deserialization Failed"),
        },
    }
}
