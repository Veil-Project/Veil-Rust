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

use crate::error::{Error, ErrorKind};
use crate::Result;
use core::convert;
use secp256k1_veil_sys::c_types::c_uchar;

pub struct Message([u8; 32]);

impl Message {
    // Maybe should have it by size.. and &[u8] input... ?
    pub fn new(bytes: &[u8; 32]) -> Self {
        Self::from(bytes)
    }

    pub fn as_ptr(&self) -> *const c_uchar {
        self.0.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut c_uchar {
        self.0.as_mut_ptr()
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        // Self::try_from(slice)
        let mut msg = [0u8; 32];

        if slice == [0u8; 32] {
            return Err(Error::new(ErrorKind::InvalidMessage));
        }

        match slice.len() {
            32 => {
                msg[..].copy_from_slice(slice);
                Ok(Self(msg))
            }
            _ => Err(Error::new(ErrorKind::InvalidMessage)),
        }
    }
}

impl From<&[u8; 32]> for Message {
    fn from(bytes: &[u8; 32]) -> Self {
        Self::new(bytes)
    }
}

impl convert::TryFrom<&[u8]> for Message {
    type Error = Error;
    fn try_from(slice: &[u8]) -> Result<Self> {
        Self::from_slice(slice)
    }
}
