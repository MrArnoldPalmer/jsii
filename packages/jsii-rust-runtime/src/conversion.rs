use crate::api::*;
use crate::JsiiClientError;
use std::convert::TryFrom;

// Convencience methods for converting responses from runtime into Results based on expected output
impl TryFrom<JsiiResponse> for JsiiOkResponse {
    type Error = JsiiClientError;
    fn try_from(response: JsiiResponse) -> Result<Self, Self::Error> {
        match response {
            JsiiResponse::Ok(val) => Ok(val),
            rest => Err(JsiiClientError::UnexpectedResponse(rest)),
        }
    }
}

impl TryFrom<JsiiResponse> for ObjRef {
    type Error = JsiiClientError;
    fn try_from(response: JsiiResponse) -> Result<Self, Self::Error> {
        let inner = JsiiOkResponse::try_from(response)?;
        match inner.ok {
            KernelResponse::Create(val) => Ok(val),
            rest => Err(JsiiClientError::UnexpectedKernelResponse(rest)),
        }
    }
}

impl TryFrom<JsiiResponse> for InvokeResponse {
    type Error = JsiiClientError;
    fn try_from(response: JsiiResponse) -> Result<Self, Self::Error> {
        let inner = JsiiOkResponse::try_from(response)?;
        match inner.ok {
            KernelResponse::Invoke(val) => Ok(val),
            rest => Err(JsiiClientError::UnexpectedKernelResponse(rest)),
        }
    }
}

// Methods for converting InvokeResponse to Rust types
impl TryFrom<InvokeResponse> for f64 {
    type Error = JsiiClientError;
    fn try_from(response: InvokeResponse) -> Result<Self, Self::Error> {
        serde_json::from_value(response.result).map_err(JsiiClientError::FormatError)
    }
}
