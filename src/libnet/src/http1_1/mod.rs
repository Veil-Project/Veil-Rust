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
use std::str::FromStr;

mod request;
mod response;
mod status_code;

pub use request::Request;
pub use response::Response;
pub use status_code::StatusCode;

pub trait ToMethod {
    fn to_method(&self) -> Method;
}

/// Possible HTTP request methods.
///
/// This enum contains all possible [`HTTP/1.0`] to [`HTTP/2.0`] methods.
#[derive(Clone, Debug, Deserialize, Copy)]
pub enum Method {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "CONNECT")]
    Connect,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "TRACE")]
    Trace,
    #[serde(rename = "PATCH")]
    Patch,
}

impl Method {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Get => "GET",
            Self::Head => "HEAD",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Connect => "CONNECT",
            Self::Options => "OPTIONS",
            Self::Trace => "TRACE",
            Self::Patch => "PATCH",
        }
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl FromStr for Method {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            s if s.eq_ignore_ascii_case("GET") => Method::Get,
            s if s.eq_ignore_ascii_case("HEAD") => Method::Head,
            s if s.eq_ignore_ascii_case("POST") => Method::Post,
            s if s.eq_ignore_ascii_case("PUT") => Method::Put,
            s if s.eq_ignore_ascii_case("DELETE") => Method::Delete,
            s if s.eq_ignore_ascii_case("CONNECT") => Method::Connect,
            s if s.eq_ignore_ascii_case("OPTIONS") => Method::Options,
            s if s.eq_ignore_ascii_case("TRACE") => Method::Trace,
            s if s.eq_ignore_ascii_case("PATCH") => Method::Patch,
            _ => Method::Get,
            // _ => println!("Throw unkown err"), // Note todo
        })
    }
}

impl ToMethod for Method {
    fn to_method(&self) -> Method {
        *self
    }
}

impl ToMethod for String {
    fn to_method(&self) -> Method {
        Method::from_str(self).unwrap()
    }
}

impl ToMethod for &str {
    fn to_method(&self) -> Method {
        Method::from_str(self).unwrap()
    }
}

pub trait ToVersion {
    fn to_version(&self) -> Version;
}

#[derive(Clone, Debug, Deserialize)]
pub enum Version {
    #[serde(rename = "HTTP/0.9")]
    V0_9,
    #[serde(rename = "HTTP/1.0")]
    V1_0,
    #[serde(rename = "HTTP/1.1")]
    V1_1,
    #[serde(rename = "HTTP/2.0")]
    V2_0,
    #[serde(rename = "HTTP/3.0")]
    V3_0,
}

impl Version {
    pub fn to_str(&self) -> &str {
        match self {
            Self::V0_9 => "HTTP/0.9",
            Self::V1_0 => "HTTP/1.0",
            Self::V1_1 => "HTTP/1.1",
            Self::V2_0 => "HTTP/2.0",
            Self::V3_0 => "HTTP/3.0",
        }
    }

    pub fn is_httpv09(&self) -> bool {
        match self {
            Version::V0_9 => true,
            _ => false,
        }
    }

    pub fn is_httpv10(&self) -> bool {
        match self {
            Version::V1_0 => true,
            _ => false,
        }
    }

    pub fn is_httpv11(&self) -> bool {
        match self {
            Version::V1_1 => true,
            _ => false,
        }
    }

    pub fn is_httpv20(&self) -> bool {
        match self {
            Version::V2_0 => true,
            _ => false,
        }
    }

    pub fn is_httpv30(&self) -> bool {
        match self {
            Version::V3_0 => true,
            _ => false,
        }
    }
}

impl FromStr for Version {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            s if s.eq_ignore_ascii_case("HTTP/0.9") => Version::V0_9,
            s if s.eq_ignore_ascii_case("HTTP/1.0") => Version::V1_0,
            s if s.eq_ignore_ascii_case("HTTP/1.1") => Version::V1_1,
            s if s.eq_ignore_ascii_case("HTTP/2.0") => Version::V2_0,
            s if s.eq_ignore_ascii_case("HTTP/3.0") => Version::V3_0,
            _ => Version::V1_1,
            // _ => println!("Throw unkown err"), // Note todo
        })
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl ToVersion for Version {
    fn to_version(&self) -> Version {
        self.to_owned()
    }
}

impl ToVersion for String {
    fn to_version(&self) -> Version {
        Version::from_str(self).unwrap()
    }
}

impl ToVersion for &str {
    fn to_version(&self) -> Version {
        Version::from_str(self).unwrap()
    }
}
