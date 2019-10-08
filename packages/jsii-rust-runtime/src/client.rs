use crate::api::{JsiiModule, JsiiOkResponse, JsiiRequest, JsiiResponse};
use crate::runtime::{JsiiRuntime, JsiiRuntimeError};

mod error;
pub use error::JsiiClientError;

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
        let response = self.runtime.request_response(JsiiRequest::Load(module))?;

        match response {
            JsiiResponse::Ok(val) => Ok(val),
            rest => Err(JsiiClientError::UnexpectedResponse(rest)),
        }
    }
}
