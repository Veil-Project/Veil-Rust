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

use super::{Method, ToMethod, ToVersion, Version};
use serde::Deserialize;
use std::convert::TryFrom;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::{collections::HashMap, fmt::Write};

#[derive(Default, Debug, Clone)]
pub struct Builder {
    method: Option<Method>,
    resource: Option<String>,
    version: Option<Version>,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn method<M: ToMethod>(&mut self, method: M) -> &mut Self {
        self.method = Some(method.to_method());
        self
    }

    pub fn resource<R: ToString>(&mut self, resource: R) -> &mut Self {
        self.resource = Some(resource.to_string());
        self
    }

    pub fn version<V: ToVersion>(&mut self, version: V) -> &mut Self {
        self.version = Some(version.to_version());
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

    pub fn build(self) -> Request {
        let mut req = Request::default();
        if let Some(method) = self.method {
            req.method = method;
        }
        if let Some(resource) = self.resource {
            req.resource = resource;
        }
        if let Some(version) = self.version {
            req.version = version;
        }
        req.headers = self.headers;
        req.body = self.body;
        req
    }
}

#[derive(Debug, Deserialize)]
pub struct Request {
    method: Method,
    resource: String,
    version: Version,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Request {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn resources(&self) -> Vec<&str> {
        self.resource().split('/').collect()
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }
}

impl Default for Request {
    fn default() -> Self {
        Self {
            method: Method::Post,
            resource: "/".to_owned(),
            version: Version::V1_1,
            headers: HashMap::new(),
            body: None,
        }
    }
}

impl ToString for Request {
    fn to_string(&self) -> String {
        let mut fmt = String::new();
        let method = self.method.to_str();
        let version = self.version.to_str();

        writeln!(&mut fmt, "{} {} {}", method, self.resource, version).unwrap();
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

impl TryFrom<BufReader<TcpStream>> for Request {
    type Error = (); // TODO: Actual error

    fn try_from(reader: BufReader<TcpStream>) -> Result<Self, Self::Error> {
        let mut lines = reader.lines();
        let mut iter = lines.by_ref().map(|f| f.unwrap());
        let mut builder = Request::builder();
        match iter.next() {
            Some(line) => {
                let mut iter_http = line.split_whitespace();

                if let Some(method) = iter_http.next() {
                    builder.method(method);
                } else {
                    println!("TODO: response error");
                }

                if let Some(resource) = iter_http.next() {
                    builder.resource(resource);
                } else {
                    println!("TODO: http1_1 request resource error");
                }

                if let Some(version) = iter_http.next() {
                    builder.version(version);
                } else {
                    println!("TODO: http1_1 request version error");
                }
            }
            None => println!("TODO: No response error"),
        }

        let mut header_end = false;
        let mut body = String::new();
        let mut headers: HashMap<String, String> = HashMap::new();
        for line in iter {
            if header_end {
                body = line;
                break;
            }

            if line.is_empty() {
                header_end = true;
            } else {
                let parts: Vec<&str> = line.splitn(2, ": ").collect();

                if parts.len() != 2 {
                    println!("TODO: http1_1 header error");
                } else {
                    headers.insert(
                        parts[0].to_string().to_ascii_lowercase(),
                        parts[1].to_string().to_ascii_lowercase(),
                    );
                }
            }
        }
        builder.headers(headers);
        builder.body(body);

        Ok(builder.build())
    }
}

// TODO: HTTP error handling.
impl TryFrom<&TcpStream> for Request {
    type Error = ();

    fn try_from(stream: &TcpStream) -> Result<Self, Self::Error> {
        let reader = BufReader::new(stream.try_clone().unwrap());
        Self::try_from(reader)
    }
}
