use crate::api::*;
use crate::runtime::{JsiiRuntime, JsiiRuntimeError};
use std::convert::TryInto;

mod error;
pub use error::JsiiClientError;

#[derive(Debug)]
pub struct JsiiClient {
    // Todo: Allow multiple clients referencing same runtime
    runtime: JsiiRuntime,
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
