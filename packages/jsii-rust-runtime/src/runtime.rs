use std::{
    io::{BufRead, BufReader, Read, Write},
    process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, Stdio},
};

mod error;
pub use self::error::JsiiRuntimeError;
use crate::api::*;

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

impl JsiiRuntime {
    pub fn new() -> Result<Self, JsiiRuntimeError> {
        let runtime_path = concat!(env!("OUT_DIR"), "/webpack/jsii-runtime.js");
        Command::new("node")
            .arg(runtime_path)
            // Todo: Make this configurable
            .env("JSII_DEBUG", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(JsiiRuntimeError::from)
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
            .and_then(|runtime| runtime.handshake())
    }

    fn handshake(mut self) -> Result<Self, JsiiRuntimeError> {
        self.read_next_line().and_then(|response| match response {
            JsiiResponse::Hello { .. } => Ok(self),
            res => Err(JsiiRuntimeError::Handshake(res)),
        })
    }

    pub fn request_response(&mut self, req: JsiiRequest) -> Result<JsiiResponse, JsiiRuntimeError> {
        let req_str = serde_json::to_string(&req).map_err(JsiiRuntimeError::BadInput)?;
        writeln!(self.stdin, "{}", req_str)?;

        self.read_next_line()
    }

    pub fn read_next_line(&mut self) -> Result<JsiiResponse, JsiiRuntimeError> {
        let mut response = String::new();
        let mut err = String::new();
        self.stdout.read_line(&mut response)?;

        match response.as_ref() {
            "" => {
                self.stderr.read_to_string(&mut err)?;
                Err(JsiiRuntimeError::Crash(err))
            }
            response_str => serde_json::from_str(response_str).map_err(JsiiRuntimeError::BadOutput),
        }
    }

    pub fn kill(mut self) -> Result<(), JsiiRuntimeError> {
        self.process.kill().map_err(JsiiRuntimeError::Io)
    }
}
