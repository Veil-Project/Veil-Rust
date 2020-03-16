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

use core::fmt;
use secp256k1_veil_sys;
#[cfg(any(feature = "std", test))]
use std::error;

#[derive(Debug)]
pub enum ErrorKind {
    Secp256k1Binding(secp256k1_veil_sys::Error),
    InvalidPublicKey,
    InvalidPrivateKey,
    InvalidMessage,
    InvalidSignature,
}

#[derive(Debug)]
pub struct Error(ErrorKind);

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self(kind)
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ErrorKind::*;
        match self.0 {
            Secp256k1Binding(ref e) => e.fmt(f),
            InvalidPublicKey => unimplemented!(),
            InvalidPrivateKey => unimplemented!(),
            InvalidMessage => write!(f, "message is empty or not 32 bytes in length"),
            InvalidSignature => unimplemented!(),
        }
    }
}

#[cfg(any(feature = "std", test))]
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use ErrorKind::*;
        match self.0 {
            Secp256k1Binding(_) => None,
            InvalidPublicKey => None,
            InvalidPrivateKey => None,
            InvalidMessage => None,
            InvalidSignature => None,
        }
    }
}

impl From<secp256k1_veil_sys::Error> for Error {
    fn from(s: secp256k1_veil_sys::Error) -> Self {
        Error::new(ErrorKind::Secp256k1Binding(s.into()))
    }
}
