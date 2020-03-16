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

use std::collections::HashMap;
use std::fmt::Write;

use super::{StatusCode, Version};

#[derive(Default, Debug)]
pub struct Builder {
    version: Option<Version>,
    status: Option<StatusCode>,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Builder {
    pub fn new() -> Self {
        Builder::default()
    }

    pub fn version(&mut self, version: Version) -> &mut Self {
        self.version = Some(version);
        self
    }

    pub fn status(&mut self, status: StatusCode) -> &mut Self {
        self.status = Some(status);
        self
    }

    pub fn header<K: ToString, V: ToString>(&mut self, name: K, value: V) -> &mut Self {
        self.headers.insert(name.to_string(), value.to_string());
        self
    }

    pub fn headers<K: ToString, V: ToString>(&mut self, headers: HashMap<K, V>) -> &mut Self {
        for header in headers {
            self.headers
                .insert(header.0.to_string(), header.1.to_string());
        }
        self
    }

    pub fn body(&mut self, body: String) -> &mut Self {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> Response {
        let mut res = Response::default();
        if let Some(version) = self.version {
            res.version = version;
        }
        if let Some(status) = self.status {
            res.status = status;
        }
        res.headers = self.headers;
        res.body = self.body;
        res
    }
}

#[derive(Debug)]
pub struct Response {
    version: Version,
    status: StatusCode,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Response {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> Option<&String> {
        self.body.as_ref()
    }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            version: Version::V1_1,
            status: StatusCode(500),
            headers: HashMap::new(),
            body: None,
        }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut fmt = String::new();
        let version = self.version.to_str();
        let status = self.status.to_str();

        writeln!(&mut fmt, "{} {}", version, status).unwrap();
        for (header, value) in &self.headers {
            writeln!(&mut fmt, "{}: {}", header, value).unwrap();
        }

        writeln!(&mut fmt).unwrap();

        if let Some(body) = &self.body {
            write!(&mut fmt, "{}", body).unwrap();
        }
        fmt
    }
}
