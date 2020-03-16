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

use std::{error, fmt, io, net, num};

#[derive(Debug)]
pub enum ErrorKind {
    Standard(String),
    Io(io::Error),
    ParseInt(num::ParseIntError),
    ParseFloat(num::ParseFloatError),
    AddrParse(net::AddrParseError),
    SerdeJson(serde_json::Error),
    Veild(String),
    Params(String),
}

/// An error which can be retured when using the veil client library.
#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error(Box::new(kind))
    }

    /// Outputs a detailed cause of the error.
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Standard(ref s) => write!(f, "{}", s),
            ErrorKind::Io(ref e) => e.fmt(f),
            ErrorKind::ParseInt(ref e) => e.fmt(f),
            ErrorKind::ParseFloat(ref e) => e.fmt(f),
            ErrorKind::AddrParse(ref e) => e.fmt(f),
            ErrorKind::SerdeJson(ref e) => e.fmt(f),
            ErrorKind::Veild(ref s) => write!(f, "{}", s),
            ErrorKind::Params(ref s) => write!(f, "{}", s),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self.0 {
            ErrorKind::Standard(_) => None,
            ErrorKind::Io(ref e) => Some(e),
            ErrorKind::ParseInt(ref e) => Some(e),
            ErrorKind::ParseFloat(ref e) => Some(e),
            ErrorKind::AddrParse(ref e) => Some(e),
            ErrorKind::SerdeJson(ref e) => Some(e),
            ErrorKind::Veild(_) => None,
            ErrorKind::Params(_) => None,
        }
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::new(ErrorKind::Standard(s))
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::new(ErrorKind::Io(e))
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Error::new(ErrorKind::ParseInt(e))
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(e: num::ParseFloatError) -> Self {
        Error::new(ErrorKind::ParseFloat(e))
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(e: net::AddrParseError) -> Self {
        Error::new(ErrorKind::AddrParse(e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::new(ErrorKind::SerdeJson(e))
    }
}
