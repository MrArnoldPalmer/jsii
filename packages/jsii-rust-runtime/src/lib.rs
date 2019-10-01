use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    io::{BufRead, BufReader, Read, Write},
    process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, Stdio},
};

mod api;
use api::*;

/// JsiiRuntime
///
/// Manages the jsii-runtime child process.
#[derive(Debug)]
pub struct JsiiRuntime {
    pub process: Child,
    pub stdin: ChildStdin,
    pub stdout: BufReader<ChildStdout>,
    pub stderr: BufReader<ChildStderr>,
}

/// JsiiRequest
///
/// Todo: build enum of known/supported apis
/// Todo: don't allow arbitrary json fields if possible
#[derive(Debug, Deserialize, Serialize)]
pub struct JsiiRequest {
    pub api: String,
    #[serde(flatten)]
    pub fields: Value,
}

/// JsiiRuntime Errors
///
/// Todo: impl Error and From
/// Todo: separate process mgmt and proxy layer errors
#[derive(Debug)]
pub enum JsiiError {
    None,
    EmptyResponse,
    Handshake(JsiiResponse),
    Io(std::io::Error),
    FormatErr(serde_json::Error),
    ProcessErr(String),
}

/// JsiiResponse
///
/// Todo: format of responses should be known
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum JsiiResponse {
    Hello {
        hello: String,
    },
    Ok {
        ok: KernelResponse,
    },
    Callback {
        callback: CallbackResponse,
    },
    Pending {
        pending: bool,
    },
    Error {
        error: String,
        stack: Option<String>,
    },
}

impl JsiiRuntime {
    pub fn new() -> Result<Self, JsiiError> {
        let runtime_path = concat!(env!("OUT_DIR"), "/webpack/jsii-runtime.js");
        Command::new("node")
            .arg(runtime_path)
            // Todo: Make this configurable
            .env("JSII_DEBUG", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(JsiiError::Io)
            .map(|mut process| {
                let stdin = process.stdin.take().unwrap();
                let stdout = BufReader::new(process.stdout.take().unwrap());
                let stderr = BufReader::new(process.stderr.take().unwrap());

                Self {
                    process,
                    stdin,
                    stdout,
                    stderr,
                }
            })
            .and_then(|mut process| {
                process
                    .read_next_line()
                    .and_then(|response| match response {
                        JsiiResponse::Hello { .. } => Ok(process),
                        res => Err(JsiiError::Handshake(res)),
                    })
            })
    }

    pub fn request_response(&mut self, req: JsiiRequest) -> Result<JsiiResponse, JsiiError> {
        writeln!(
            self.stdin,
            "{}",
            serde_json::to_string(&req).map_err(JsiiError::FormatErr)?
        )
        .map_err(JsiiError::Io)?;

        self.read_next_line()
    }

    pub fn read_next_line(&mut self) -> Result<JsiiResponse, JsiiError> {
        let mut response = String::new();
        let mut err = String::new();
        self.stdout
            .read_line(&mut response)
            .map_err(JsiiError::Io)?;

        match response.as_ref() {
            "" => {
                self.stderr
                    .read_to_string(&mut err)
                    .map_err(JsiiError::Io)?;
                Err(JsiiError::ProcessErr("Some Process Error".into()))
            }
            response_str => serde_json::from_str(response_str).map_err(JsiiError::FormatErr),
        }
    }

    pub fn load_module(
        &mut self,
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
    // pub tarball: String,
}
