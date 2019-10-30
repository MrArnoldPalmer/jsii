use crate::api::*;
use crate::runtime::JsiiRuntimeError;
use serde_json::Value;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum JsiiClientError {
    Runtime(JsiiRuntimeError),
    UnexpectedResponse(JsiiResponse),
    UnexpectedKernelResponse(JsiiKernelResponse),
    UnexpectedInvokeResponse(Value),
    FormatError(serde_json::Error),
}

impl From<JsiiRuntimeError> for JsiiClientError {
    fn from(err: JsiiRuntimeError) -> Self {
        Self::Runtime(err)
    }
}

impl fmt::Display for JsiiClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for JsiiClientError {
    fn description(&self) -> &str {
        match self {
            Self::Runtime(inner) => Error::description(inner),
            Self::UnexpectedResponse(_err) => "Unexpected response to request",
            Self::UnexpectedKernelResponse(_err) => "Unexpected kernel response",
            Self::UnexpectedInvokeResponse(_err) => "Unexpected invoke response",
            Self::FormatError(_err) => "Unexpected invoke response format",
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Runtime(inner) => Some(inner),
            _ => None,
        }
    }
}
