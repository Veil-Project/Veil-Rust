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

use crate::context::{Context, ContextNone};
use crate::traits::AsNative;
use crate::Result;
use core::slice;
use secp256k1_veil_sys::c_types::size_t;

#[derive(Debug)]
pub struct Signature(*mut secp256k1_veil_sys::Signature);

impl Signature {
    pub fn new(sig: *mut secp256k1_veil_sys::Signature) -> Self {
        Self(sig)
    }

    pub fn parse_compact(ctx: &ContextNone, input: [u8; 64]) -> Result<Self> {
        Ok(Self(unsafe {
            secp256k1_veil_sys::Signature::parse_compact(ctx.as_ptr(), input.as_ptr())?
        }))
    }

    pub fn parse_der(input: &[u8], ctx: &ContextNone) -> Result<Self> {
        Ok(Self(unsafe {
            secp256k1_veil_sys::Signature::parse_der(
                ctx.as_ptr() as *const secp256k1_veil_sys::Context,
                input.as_ptr(),
                input.len() as size_t,
            )?
        }))
    }

    pub fn serialize_compact(&self, ctx: &ContextNone) -> Result<[u8; 64]> {
        unsafe {
            let sig = self.as_native_ref();
            Ok(sig.serialize_compact(ctx.as_ptr())?)
        }
    }

    // Might not work with no_std.
    pub fn serialize_der(&self, ctx: &ContextNone) -> Result<&[u8]> {
        unsafe {
            let sig = self.as_native_ref();
            let (output, output_len) = sig.serialize_der(ctx.as_ptr())?;

            Ok(slice::from_raw_parts(output, output_len as usize))
        }
    }

    pub fn as_ptr(&self) -> *const secp256k1_veil_sys::Signature {
        self.0 as *const secp256k1_veil_sys::Signature
    }

    pub fn as_mut_ptr(&mut self) -> *mut secp256k1_veil_sys::Signature {
        self.0
    }
}

impl AsNative<secp256k1_veil_sys::Signature> for Signature {
    unsafe fn as_native_ref(&self) -> &secp256k1_veil_sys::Signature {
        &*self.0
    }

    unsafe fn as_native_mut(&mut self) -> &mut secp256k1_veil_sys::Signature {
        &mut *self.0
    }
}

unsafe impl Send for Signature {}
unsafe impl Sync for Signature {}
