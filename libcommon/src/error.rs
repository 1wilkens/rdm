use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum IpcError {
    IO(io::Error),
    HeaderTooShort,
    InvalidMagic,
    UnknownMessageType,
}

impl Error for IpcError {
    fn description(&self) -> &str {
        "test"
    }

    fn cause(&self) -> Option<&(dyn Error)> {
        match *self {
            IpcError::IO(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for IpcError {
    fn from(error: io::Error) -> Self {
        IpcError::IO(error)
    }
}

impl fmt::Display for IpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test")
    }
}
