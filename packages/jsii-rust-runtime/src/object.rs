use crate::api::*;
use crate::client::{JsiiClient, JsiiClientError};

pub trait JsiiObject<'a> {
    const FQN: &'a str;
    fn get_ref(&self) -> JsiiObjRef;
    fn get_client(&'a mut self) -> &'a mut JsiiClient;

    fn call_method(
        &'a mut self,
        method: String,
        args: Vec<serde_json::Value>,
    ) -> Result<JsiiInvokeResponse, JsiiClientError> {
        let obj_ref = self.get_ref();
        self.get_client().call_method(JsiiInvokeRequest {
            obj_ref,
            method,
            args,
        })
    }

    fn jsii_get_property(
        &'a mut self,
        property: String,
    ) -> Result<JsiiGetResponse, JsiiClientError> {
        let obj_ref = self.get_ref();
        self.get_client()
            .get_property(JsiiGetRequest { obj_ref, property })
    }

    fn jsii_set_property(
        &'a mut self,
        property: String,
        value: serde_json::Value,
    ) -> Result<JsiiSetResponse, JsiiClientError> {
        let obj_ref = self.get_ref();
        self.get_client().set_property(JsiiSetRequest {
            obj_ref,
            property,
            value,
        })
    }
}
