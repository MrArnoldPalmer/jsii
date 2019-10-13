use crate::api::*;
use crate::runtime::{JsiiRuntime, JsiiRuntimeError};
use std::convert::{TryFrom, TryInto};

mod error;
pub use error::JsiiClientError;

pub struct JsiiClient {
    // Todo: Allow multiple clients referencing same runtime
    runtime: JsiiRuntime,
}

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

impl JsiiClient {
    pub fn new(runtime: Option<JsiiRuntime>) -> Result<Self, JsiiRuntimeError> {
        match runtime {
            Some(rt) => Ok(Self { runtime: rt }),
            None => Ok(Self {
                runtime: JsiiRuntime::new()?,
            }),
        }
    }

    // Todo: Don't require mutable reference to runtime
    pub fn load_module(&mut self, module: JsiiModule) -> Result<JsiiOkResponse, JsiiClientError> {
        self.runtime
            .request_response(JsiiRequest::Load(module))?
            .try_into()
    }

    pub fn create_object(&mut self, object: JsiiCreateObject) -> Result<ObjRef, JsiiClientError> {
        self.runtime
            .request_response(JsiiRequest::Create(object))?
            .try_into()
    }

    pub fn call_method(
        &mut self,
        method: JsiiInvokeRequest,
    ) -> Result<InvokeResponse, JsiiClientError> {
        self.runtime
            .request_response(JsiiRequest::Invoke(method))?
            .try_into()
    }
}
