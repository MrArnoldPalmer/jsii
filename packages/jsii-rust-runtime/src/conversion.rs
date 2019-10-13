use crate::api::*;
use crate::JsiiClientError;
use std::convert::TryFrom;

// Methods for converting InvokeResponse to Rust types
impl TryFrom<InvokeResponse> for f64 {
    type Error = JsiiClientError;
    fn try_from(response: InvokeResponse) -> Result<Self, Self::Error> {
        serde_json::from_value(response.result).map_err(JsiiClientError::FormatError)
    }
}
