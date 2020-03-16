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

use mimir_net::http1_1;
use mimir_net::json_rpc;

use crate::error::ErrorKind;
use crate::{Error, Result};

#[derive(Debug)]
pub struct Response(json_rpc::Response);

impl Response {
    pub fn new(res: json_rpc::Response) -> Self {
        Self(res)
    }

    pub fn parse(http: http1_1::Response) -> Result<Self> {
        let json: json_rpc::Response = serde_json::from_str(http.body().unwrap())?;
        Ok(Self(json))
    }

    pub fn verify(self) -> Result<serde_json::Value> {
        if let Some(e) = &self.0.error() {
            Err(Error::new(ErrorKind::Veild(e.message().to_owned())))
        } else {
            Ok(self.0.result().unwrap().to_owned())
        }
    }

    // TODO: serde as feature
    pub fn result(&self) -> Option<&serde_json::Value> {
        self.0.result()
    }

    pub fn error(&self) -> Option<&json_rpc::ResponseError> {
        if let Some(e) = &self.0.error() {
            Some(e)
        } else {
            None
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.0.id()
    }

    pub fn is_error(&self) -> bool {
        self.0.error().is_some()
    }

    pub fn is_result(&self) -> bool {
        self.0.result().is_some()
    }
}
