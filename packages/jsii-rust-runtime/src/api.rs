use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsiiObjRef {
    #[serde(rename = "$jsii.byref")]
    ref_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsiiHelloResponse {
    hello: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsiiLoadResponse {
    assembly: String,
    types: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsiiGetResponse {
    pub value: Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsiiInvokeResponse {
    pub result: Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsiiBeginResponse {
    pub promiseid: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsiiEndResponse {
    result: Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsiiCallbackResponse {
    cbid: String,
    cookie: Option<String>,
    invoke: Option<JsiiInvokeRequest>,
    get: Option<JsiiGetRequest>,
    set: Option<JsiiSetRequest>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsiiCallbacksResponse {
    callbacks: Vec<JsiiCallbackResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsiiCompleteResponse {
    cbid: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsiiNamingResponse {
    assemby: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsiiStatsResponse {
    object_count: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsiiSetResponse {}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsiiDelResponse {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum JsiiKernelResponse {
    Hello(JsiiHelloResponse),
    Load(JsiiLoadResponse),
    Create(JsiiObjRef),
    Get(JsiiGetResponse),
    Invoke(JsiiInvokeResponse),
    Begin(JsiiBeginResponse),
    End(JsiiEndResponse),
    Callback(JsiiCallbacksResponse),
    Complete(JsiiCompleteResponse),
    Naming(JsiiNamingResponse),
    Stats(JsiiStatsResponse),
    Set(JsiiSetResponse),
    Del(JsiiDelResponse),
}

/// JsiiErrorResponse
///
/// Error returned from jsii child process.
#[derive(Debug, Deserialize, Serialize)]
pub struct JsiiErrorResponse {
    pub error: String,
    pub stack: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsiiOkResponse {
    pub ok: JsiiKernelResponse,
}

/// JsiiResponse
///
/// Todo: format of responses should be known
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum JsiiResponse {
    Hello(JsiiHelloResponse),
    Ok(JsiiOkResponse),
    Callback { callback: JsiiCallbackResponse },
    Pending { pending: bool },
    Error(JsiiErrorResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsiiModule {
    pub name: String,
    pub version: String,
    pub tarball: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsiiCreateObject {
    pub fqn: String,
    pub args: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsiiInvokeRequest {
    #[serde(rename = "objref")]
    pub obj_ref: JsiiObjRef,
    pub method: String,
    pub args: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsiiGetRequest {
    #[serde(rename = "objref")]
    pub obj_ref: JsiiObjRef,
    pub property: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsiiSetRequest {
    #[serde(rename = "objref")]
    pub obj_ref: JsiiObjRef,
    pub property: String,
    pub value: Value,
}

/// JsiiRequest
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "api", rename_all = "camelCase")]
pub enum JsiiRequest {
    Invoke(JsiiInvokeRequest),
    Create(JsiiCreateObject),
    Load(JsiiModule),
    Get(JsiiGetRequest),
    Set(JsiiSetRequest),
}
