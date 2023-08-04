use crate::error::OpenNotifyError;

pub trait OpenNotifyResponse: Sized {
    fn deserialize(s: &str) -> Result<Self, OpenNotifyError>;
}
