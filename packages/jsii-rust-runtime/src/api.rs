use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// const TOKEN_REF: &str = "$jsii.byref";
// const TOKEN_DATE: &str = "$jsii.date";
// const TOKEN_ENUM: &str = "$jsii.enum";

type ObjRef = HashMap<String, String>;

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
pub struct CallbackResponse {
    cbid: String,
    cookie: Option<String>,
    invoke: Option<InvokeRequest>,
    get: Option<GetRequest>,
    set: Option<SetRequest>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum KernelResponse {
    Hello { hello: String },
    Load { assembly: String, types: i64 },
    Create(ObjRef),
    Del {},
    Get { value: Value },
    Set {},
    Invoke { result: Value },
    Begin { promiseid: String },
    End { result: Value },
    Callback { callbacks: Vec<CallbackResponse> },
    Complete { cbid: String },
    Naming { assembly: String },
    Stats { object_count: i64 },
}
