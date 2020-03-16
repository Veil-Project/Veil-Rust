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
use std::fmt::Write;

use super::Version;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Builder {
    proto: Option<Version>,
    api_version: Option<String>,
    method: String,
    params: Option<String>,
    id: Option<String>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn protocol(mut self, protocol: Version) -> Self {
        self.proto = Some(protocol);
        self
    }

    pub fn api_version<A: ToString>(mut self, api_version: A) -> Self {
        self.api_version = Some(api_version.to_string());
        self
    }

    pub fn method<M: ToString>(mut self, method: M) -> Self {
        self.method = method.to_string();
        self
    }

    pub fn params<P: ToString>(mut self, params: P) -> Self {
        self.params = Some(params.to_string());
        self
    }

    pub fn id<I: ToString>(mut self, id: I) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn build(self) -> Request {
        Request {
            proto: self.proto,
            api_version: self.api_version,
            method: self.method,
            params: self.params,
            id: self.id,
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            proto: Some(Version::V2),
            api_version: None,
            method: "echo".to_owned(),
            params: None,
            id: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Request {
    proto: Option<Version>,
    api_version: Option<String>,
    method: String,
    params: Option<String>,
    id: Option<String>,
}

impl Request {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    pub fn protocol(&self) -> Option<&Version> {
        self.proto.as_ref()
    }

    pub fn api_version(&self) -> Option<&str> {
        self.api_version.as_deref()
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn params(&self) -> Option<&str> {
        self.params.as_ref().map(|x| x.as_ref())
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.as_ref())
    }

    fn v1_string(&self) -> String {
        let mut json = String::new();
        write!(
            &mut json,
            "{{\"jsonrpc\": \"1.0\", \"method\": \"{}\"",
            self.method
        )
        .unwrap();
        write!(&mut json, ", \"params\": ").unwrap();
        match &self.params {
            Some(params) => write!(&mut json, "{}", params).unwrap(),
            None => write!(&mut json, "[]",).unwrap(),
        }
        write!(&mut json, ", \"id\": ").unwrap();
        match &self.id {
            Some(id) => write!(&mut json, "\"{}\"", id).unwrap(),
            None => write!(&mut json, "null").unwrap(),
        }
        write!(&mut json, "}}").unwrap();
        json
    }

    fn v2_string(&self) -> String {
        let mut json = "{\"jsonrpc\": \"2.0\"".to_string();
        write!(&mut json, ", \"method\": \"{}\"", self.method).unwrap();
        if let Some(params) = &self.params {
            write!(&mut json, ", \"params\": {}", params).unwrap();
        }
        if let Some(id) = &self.id {
            write!(&mut json, ", \"id\": \"{}\"", id).unwrap();
        }
        json
    }
}

impl Default for Request {
    fn default() -> Self {
        Self {
            proto: Some(Version::V2),
            api_version: None,
            method: "echo".to_string(),
            params: None,
            id: None,
        }
    }
}

impl ToString for Request {
    fn to_string(&self) -> String {
        if self.proto == Some(Version::V1) || self.proto == None {
            self.v1_string()
        } else {
            self.v2_string()
        }
    }
}

impl TryFrom<&str> for Request {
    type Error = serde_json::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        let s = serde_json::from_str(str)?;
        Ok(s)
    }
}
