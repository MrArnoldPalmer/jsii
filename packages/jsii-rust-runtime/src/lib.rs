use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    io::{Read, Write},
    process::{Child, Command, Stdio},
};

/// JsiiRuntime
///
/// Manages the jsii-runtime child process.
#[derive(Debug)]
pub struct JsiiRuntime {
    pub process: Child,
}

/// JsiiRequest
///
/// Todo: build enum of known/supported apis
/// Todo: don't allow arbitrary json fields if possible
#[derive(Debug, Deserialize, Serialize)]
pub struct JsiiRequest {
    api: String,
    #[serde(flatten)]
    fields: Value,
}

/// JsiiRuntime Errors
///
/// Todo: impl Error and From
/// Todo: separate process mgmt and proxy layer errors
#[derive(Debug)]
pub enum JsiiError {
    None,
    EmptyResponse,
    Io(std::io::Error),
    FormatErr(serde_json::Error),
    ProcessErr(String),
}

/// JsiiResponse
///
/// Todo: format of responses should be known
type JsiiResponse = Value;

impl JsiiRuntime {
    pub fn new() -> Result<Self, std::io::Error> {
        let runtime_path = concat!(env!("OUT_DIR"), "/webpack/jsii-runtime.js");
        Command::new("node")
            .arg(runtime_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map(|process| Self { process })
    }

    fn request_response(self, req: JsiiRequest) -> Result<JsiiResponse, JsiiError> {
        self.process
            .stdin
            .ok_or_else(|| JsiiError::None)?
            .write_all(
                serde_json::to_string(&req)
                    .map_err(JsiiError::FormatErr)?
                    .as_bytes(),
            )
            .expect("Can't write to stdin");

        let mut response = String::new();
        self.process
            .stdout
            .ok_or_else(|| JsiiError::None)?
            .read_to_string(&mut response)
            .map_err(JsiiError::Io)?;
        dbg!(&response);

        let mut err = String::new();
        self.process
            .stderr
            .ok_or_else(|| JsiiError::None)?
            .read_to_string(&mut err)
            .map_err(JsiiError::Io)?;
        dbg!(&err);

        match response.as_ref() {
            "" => Err(JsiiError::ProcessErr(err)),
            response_str => serde_json::from_str(response_str).map_err(JsiiError::FormatErr),
        }
    }

    pub fn load_module(
        self,
        JsiiModule { name, version }: JsiiModule,
    ) -> Result<JsiiResponse, JsiiError> {
        self.request_response(JsiiRequest {
            api: "load".into(),
            fields: json!({
                "name": name,
                "version": version
            }),
        })
    }

    pub fn kill(mut self) -> Result<(), std::io::Error> {
        self.process.kill()
    }
}

#[derive(Debug, Deserialize)]
pub struct JsiiModule {
    pub name: String,
    pub version: String,
}
