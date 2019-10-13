use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjRef {
    #[serde(rename = "$jsii.byref")]
    ref_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvokeRequest {
    fqn: String,
    method: String,
    args: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRequest {
    objref: Value,
    property: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRequest {
    objref: ObjRef,
    property: String,
    value: Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HelloResponse {
    hello: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadResponse {
    assembly: String,
    types: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResponse {
    pub value: Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvokeResponse {
    pub result: Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BeginResponse {
    pub promiseid: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndResponse {
    result: Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallbackResponse {
    cbid: String,
    cookie: Option<String>,
    invoke: Option<InvokeRequest>,
    get: Option<GetRequest>,
    set: Option<SetRequest>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallbacksResponse {
    callbacks: Vec<CallbackResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteResponse {
    cbid: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NamingResponse {
    assemby: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsResponse {
    object_count: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum KernelResponse {
    Hello(HelloResponse),
    Load(LoadResponse),
    Create(ObjRef),
    // Del {},
    Get(GetResponse),
    // Set {},
    Invoke(InvokeResponse),
    Begin(BeginResponse),
    End(EndResponse),
    Callback(CallbacksResponse),
    Complete(CompleteResponse),
    Naming(NamingResponse),
    Stats(StatsResponse),
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
    pub ok: KernelResponse,
}

/// JsiiResponse
///
/// Todo: format of responses should be known
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum JsiiResponse {
    Hello(HelloResponse),
    Ok(JsiiOkResponse),
    Callback { callback: CallbackResponse },
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
    pub obj_ref: ObjRef,
    pub method: String,
    pub args: Vec<serde_json::Value>,
}

/// JsiiRequest
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "api", rename_all = "camelCase")]
pub enum JsiiRequest {
    Invoke(JsiiInvokeRequest),
    Create(JsiiCreateObject),
    Load(JsiiModule),
}
