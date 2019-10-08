use crate::api::JsiiResponse;
use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum JsiiRuntimeError {
    Crash(String),
    Handshake(JsiiResponse),
    Io(io::Error),
    BadInput(serde_json::Error),
    BadOutput(serde_json::Error),
}

impl From<io::Error> for JsiiRuntimeError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl std::fmt::Display for JsiiRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime Error: {}", self)
    }
}

impl Error for JsiiRuntimeError {
    fn description(&self) -> &str {
        match self {
            Self::Io(inner) => Error::description(inner),
            Self::BadInput(inner) => Error::description(inner),
            Self::BadOutput(inner) => Error::description(inner),
            // Todo: Add _err str to description output
            Self::Crash(_err) => "Runtime child process error",
            Self::Handshake(_err) => "Unexpected handshake response",
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(inner) => Some(inner),
            Self::BadInput(inner) => Some(inner),
            Self::BadOutput(inner) => Some(inner),
            _ => None,
        }
    }
}
