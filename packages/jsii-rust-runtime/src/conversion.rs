use crate::api::*;
use crate::JsiiClientError;
use std::convert::{TryFrom, TryInto};

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

impl TryFrom<JsiiResponse> for JsiiObjRef {
    type Error = JsiiClientError;
    fn try_from(response: JsiiResponse) -> Result<Self, Self::Error> {
        let inner = JsiiOkResponse::try_from(response)?;
        match inner.ok {
            JsiiKernelResponse::Create(val) => Ok(val),
            rest => Err(JsiiClientError::UnexpectedKernelResponse(rest)),
        }
    }
}

impl TryFrom<JsiiResponse> for JsiiInvokeResponse {
    type Error = JsiiClientError;
    fn try_from(response: JsiiResponse) -> Result<Self, Self::Error> {
        let inner = JsiiOkResponse::try_from(response)?;
        match inner.ok {
            JsiiKernelResponse::Invoke(val) => Ok(val),
            rest => Err(JsiiClientError::UnexpectedKernelResponse(rest)),
        }
    }
}

impl TryFrom<JsiiResponse> for JsiiGetResponse {
    type Error = JsiiClientError;
    fn try_from(response: JsiiResponse) -> Result<Self, Self::Error> {
        let inner = JsiiOkResponse::try_from(response)?;
        match inner.ok {
            JsiiKernelResponse::Get(val) => Ok(val),
            rest => Err(JsiiClientError::UnexpectedKernelResponse(rest)),
        }
    }
}

impl TryFrom<JsiiResponse> for JsiiSetResponse {
    type Error = JsiiClientError;
    fn try_from(response: JsiiResponse) -> Result<Self, Self::Error> {
        let inner = JsiiOkResponse::try_from(response)?;
        match inner.ok {
            JsiiKernelResponse::Set(val) => Ok(val),
            rest => Err(JsiiClientError::UnexpectedKernelResponse(rest)),
        }
    }
}

// Methods for converting InvokeResponse to Rust types
impl TryFrom<JsiiInvokeResponse> for f64 {
    type Error = JsiiClientError;
    fn try_from(response: JsiiInvokeResponse) -> Result<Self, Self::Error> {
        serde_json::from_value(response.result).map_err(JsiiClientError::FormatError)
    }
}

// Methods for converting InvokeResponse to Rust types
impl TryFrom<JsiiInvokeResponse> for String {
    type Error = JsiiClientError;
    fn try_from(response: JsiiInvokeResponse) -> Result<Self, Self::Error> {
        serde_json::from_value(response.result).map_err(JsiiClientError::FormatError)
    }
}

// Methods for converting InvokeResponse to Rust types
impl TryFrom<JsiiInvokeResponse> for () {
    type Error = JsiiClientError;
    fn try_from(response: JsiiInvokeResponse) -> Result<Self, Self::Error> {
        serde_json::from_value(response.result).map_err(JsiiClientError::FormatError)
    }
}

// Methods for converting GetResponse to Rust types
impl TryFrom<JsiiGetResponse> for f64 {
    type Error = JsiiClientError;
    fn try_from(response: JsiiGetResponse) -> Result<Self, Self::Error> {
        serde_json::from_value(response.value).map_err(JsiiClientError::FormatError)
    }
}

// Methods for converting InvokeResponse to Rust types
impl TryFrom<JsiiGetResponse> for String {
    type Error = JsiiClientError;
    fn try_from(response: JsiiGetResponse) -> Result<Self, Self::Error> {
        serde_json::from_value(response.value).map_err(JsiiClientError::FormatError)
    }
}

// Todo: Generalize Option TryIntosto reduce repitition, this currently conflicts with an impl in
// std.
// impl<T> TryInto<Option<T>> for JsiiGetResponse
// where
//     T: serde::de::DeserializeOwned,
// {
//     type Error = JsiiClientError;
//     fn try_into(self) -> Result<Vec<T>, Self::Error> {
//         serde_json::from_value(self.value).map_err(JsiiClientError::FormatError)
//     }
// }

impl<T> TryInto<Vec<T>> for JsiiGetResponse
where
    T: serde::de::DeserializeOwned,
{
    type Error = JsiiClientError;
    fn try_into(self) -> Result<Vec<T>, Self::Error> {
        serde_json::from_value(self.value).map_err(JsiiClientError::FormatError)
    }
}

impl<T> TryInto<Option<Vec<T>>> for JsiiGetResponse
where
    T: serde::de::DeserializeOwned,
{
    type Error = JsiiClientError;
    fn try_into(self) -> Result<Option<Vec<T>>, Self::Error> {
        serde_json::from_value(self.value).map_err(JsiiClientError::FormatError)
    }
}

impl TryInto<Option<String>> for JsiiGetResponse {
    type Error = JsiiClientError;
    fn try_into(self) -> Result<Option<String>, Self::Error> {
        serde_json::from_value(self.value).map_err(JsiiClientError::FormatError)
    }
}
