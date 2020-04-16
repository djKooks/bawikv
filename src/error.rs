use failure::Fail;
use std::io;

#[derive(Fail, Debug)]
pub enum InternalError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),

    #[fail(display = "Key not found")]
    KeyNotFound,

    #[fail(display = "Unexpected")]
    Unexpected,
}

impl From<io::Error> for InternalError {
    fn from(err: io::Error) -> InternalError {
        InternalError::Io(err)
    }
}

impl From<serde_json::Error> for InternalError {
    fn from(err: serde_json::Error) -> InternalError {
        InternalError::Serde(err)
    }
}

pub type Result<T> = std::result::Result<T, InternalError>;
