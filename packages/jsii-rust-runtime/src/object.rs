use crate::api::{InvokeResponse, JsiiInvokeRequest, ObjRef};
use crate::client::{JsiiClient, JsiiClientError};

pub trait JsiiObject<'a> {
    fn get_fqn() -> String;
    fn get_ref(&self) -> ObjRef;
    fn get_client(&'a mut self) -> &'a mut JsiiClient;
    fn call_method(
        &'a mut self,
        method: JsiiInvokeRequest,
    ) -> Result<InvokeResponse, JsiiClientError> {
        self.get_client().call_method(method)
    }
}
