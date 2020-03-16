// Copyright 2020 Veil Rust Developers
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

use serde::Deserialize;
use serde_json::Value;
use std::fmt::Write;

use super::Version;

// #[derive(Debug, Deserialize)]
// pub enum ErrorKind {
//     ParseError,
//     InvalidRequest,
//     MethodNotFound,
//     InvalidParams,
//     InternalError,
//     ServerError {
//         code: i16,
//         message: String,
//         data: Option<String>,
//     },
// }

// impl ErrorKind {
//     pub fn code(&self) -> i16 {
//         i16::from(self)
//     }

//     pub fn message(&self) -> &str {
//         match self {
//             ErrorKind::ParseError => "Parse error",
//             ErrorKind::InvalidRequest => "Invalid Request",
//             ErrorKind::MethodNotFound => "Method not found",
//             ErrorKind::InvalidParams => "Invalid params",
//             ErrorKind::InternalError => "Internal error",
//             ErrorKind::ServerError { message: s, .. } => &s,
//         }
//     }

//     pub fn data(&self) -> Option<&str> {
//         match &self {
//             ErrorKind::ParseError => Some("Invalid JSON was received by the server"),
//             ErrorKind::InvalidRequest => Some("The JSON sent is not a valid Request object"),
//             ErrorKind::MethodNotFound => Some("The method does not exist / is not available"),
//             ErrorKind::InvalidParams => Some("Invalid method parameter(s)"),
//             ErrorKind::InternalError => Some("Internal JSON-RPC error"),
//             ErrorKind::ServerError { data: d, .. } => d.as_ref().map(|x| &**x),
//         }
//     }

//     pub fn to_json_rpc(&self, version: &Version) -> String {
//         match version {
//             Version::V1 => self.json_v1(),
//             Version::V2 => self.json_v2(),
//         }
//     }

//     fn json_v1(&self) -> String {
//         let mut fmt = format!(
//             "{{\"code\": {}, \"message\": \"{}\"",
//             self.code(),
//             self.message()
//         );

//         write!(&mut fmt, ", \"data\": ").unwrap();
//         match self.data() {
//             Some(d) => write!(&mut fmt, "\"{}\" }}", d).unwrap(),
//             None => write!(&mut fmt, "null }}").unwrap(),
//         }
//         fmt
//     }

//     fn json_v2(&self) -> String {
//         let mut fmt = format!(
//             "{{\"code\": {}, \"message\": \"{}\"",
//             self.code(),
//             self.message()
//         );

//         match self {
//             ErrorKind::ServerError { data, .. } => match data {
//                 Some(d) => write!(&mut fmt, ", \"data\": \"{}\"}}", d).unwrap(),
//                 None => write!(&mut fmt, "}}").unwrap(),
//             },
//             _ => write!(
//                 &mut fmt,
//                 ", \"data\": \"{}\"}}",
//                 self.data().expect("JSON-RPC error data is None.")
//             )
//             .unwrap(),
//         }
//         fmt
//     }
// }

// impl ToString for ErrorKind {
//     fn to_string(&self) -> String {
//         match self {
//             ErrorKind::ParseError => "Parse error".to_string(),
//             ErrorKind::InvalidRequest => "Invalid Request".to_string(),
//             ErrorKind::MethodNotFound => "Method not found".to_string(),
//             ErrorKind::InvalidParams => "Invalid params".to_string(),
//             ErrorKind::InternalError => "Internal error".to_string(),
//             ErrorKind::ServerError { message: s, .. } => s.to_owned(),
//         }
//     }
// }

// impl From<&ErrorKind> for i16 {
//     fn from(ek: &ErrorKind) -> Self {
//         match ek {
//             ErrorKind::ParseError => -32_700,
//             ErrorKind::InvalidRequest => -32_600,
//             ErrorKind::MethodNotFound => -32_601,
//             ErrorKind::InvalidParams => -32_602,
//             ErrorKind::InternalError => -32_603,
//             ErrorKind::ServerError { code: i, .. } => i.to_owned(),
//         }
//     }
// }

#[derive(Debug, Deserialize)]
pub struct ResponseError {
    code: isize,
    message: String,
}

impl ResponseError {
    pub fn code(&self) -> &isize {
        &self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Default, Debug, Deserialize)]
pub struct Builder {
    // frame: Option<Version>,
    result: Option<Value>,
    error: Option<ResponseError>,
    id: Option<String>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn protocol(mut self, protocol: Version) -> Self {
    //     self.frame = Some(protocol);
    //     self
    // }

    // TODO: String -> Value
    pub fn result(mut self, result: Value) -> Self {
        self.result = Some(result);
        self
    }

    pub fn error(mut self, error: ResponseError) -> Self {
        self.error = Some(error);
        self
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn build(self) -> Response {
        let mut res = Response::default();
        // if let Some(version) = self.frame {
        //     res.frame = version;
        // }
        res.result = self.result;
        res.error = self.error;
        res.id = self.id;
        res
    }
}

// TODO: Optional protocol version
#[derive(Debug, Deserialize)]
pub struct Response {
    result: Option<Value>,
    error: Option<ResponseError>,
    id: Option<String>,
}

impl Response {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn result(&self) -> Option<&Value> {
        self.result.as_ref()
    }

    pub fn error(&self) -> Option<&ResponseError> {
        self.error.as_ref()
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.as_ref())
    }

    // pub fn to_v1_string(&self) -> String {
    //     let mut json = "{".to_string();

    //     write!(&mut json, "\"result: \"").unwrap();
    //     if let Some(result) = &self.result {
    //         write!(&mut json, "\"{}\"", result).unwrap();
    //     } else {
    //         write!(&mut json, "null").unwrap();
    //     }

    //     write!(&mut json, ", \"error: \"").unwrap();
    //     match &self.error {
    //         Some(error) => write!(&mut json, "{},", error.to_json_rpc(self.protocol())).unwrap(),
    //         None => write!(&mut json, "null",).unwrap(),
    //     }

    //     write!(&mut json, ", \"id\": ").unwrap();
    //     match &self.id {
    //         Some(id) => write!(&mut json, "{}", id).unwrap(),
    //         None => write!(&mut json, "null").unwrap(),
    //     }
    //     json
    // }

    // pub fn to_v2_string(&self) -> String {
    //     let mut json = "{\"jsonrpc\": \"2.0\"".to_string();

    //     if let Some(result) = &self.result {
    //         write!(&mut json, ", \"result\": \"{}\"", result).unwrap();
    //     }

    //     if let Some(error) = &self.error {
    //         write!(
    //             &mut json,
    //             ", \"error\": {}",
    //             error.to_json_rpc(self.protocol())
    //         )
    //         .unwrap();
    //     }

    //     if let Some(id) = &self.id {
    //         write!(&mut json, ", \"id\": {}", id).unwrap();
    //     }

    //     json
    // }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            result: None,
            error: None,
            id: None,
        }
    }
}

// impl ToString for Response {
//     fn to_string(&self) -> String {
//         if self.frame == Version::V1 {
//             self.to_v1_string()
//         } else {
//             self.to_v2_string()
//         }
//     }
// }
