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

use crate::{bindings, c_types::*, Context, Error, ErrorKind, PublicKey, Result};

/// Opaque data structured that holds a parsed ECDSA signature.
///
/// The exact representation of data inside is implementation defined and not
/// guaranteed to be portable between different platforms or versions. It is
/// however guaranteed to be 64 bytes in size, and can be safely copied/moved.
/// If you need to convert to a format suitable for storage, transmission, or
/// comparison, use the secp256k1_ecdsa_signature_serialize_* and
/// secp256k1_ecdsa_signature_parse_* functions.
#[derive(Clone)] // Todo implement debug
#[repr(C)]
pub struct Signature(pub [c_uchar; 64]);

impl Signature {
    pub unsafe fn new() -> *mut Self {
        &mut Self([0; 64]) as *mut Self
    }

    pub unsafe fn parse_compact(ctx: *const Context, input64: *const c_uchar) -> Result<*mut Self> {
        let sig = Self::new();
        if bindings::secp256k1_ecdsa_signature_parse_compact(ctx, sig, input64) == 0 {
            Err(Error::new(ErrorKind::ParseKey))
        } else {
            Ok(sig)
        }
    }

    pub unsafe fn parse_der(
        ctx: *const Context,
        input: *const c_uchar,
        input_len: size_t,
    ) -> Result<*mut Self> {
        let sig = Self::new();
        if bindings::secp256k1_ecdsa_signature_parse_der(ctx, sig, input, input_len) == 0 {
            Err(Error::new(ErrorKind::ParseSignature))
        } else {
            Ok(sig)
        }
    }

    pub unsafe fn serialize_compact(&self, ctx: *const Context) -> Result<[u8; 64]> {
        let mut output64 = [0u8; 64];
        bindings::secp256k1_ecdsa_signature_serialize_compact(ctx, output64.as_mut_ptr(), self); // can not fail
        Ok(output64)
    }

    pub unsafe fn serialize_der(&self, ctx: *const Context) -> Result<(*mut c_uchar, *mut size_t)> {
        let output: *mut c_uchar = 0 as *mut c_uchar;
        let output_len: *mut size_t = 0 as *mut size_t;
        if bindings::secp256k1_ecdsa_signature_serialize_der(ctx, output, output_len, self) == 0 {
            Err(Error::new(ErrorKind::SerializeSignature))
        } else {
            Ok((output, output_len))
        }
    }

    pub unsafe fn normalize(&self, ctx: *const Context) -> Result<*mut Self> {
        let signature_out = Self::new();
        if bindings::secp256k1_ecdsa_signature_normalize(ctx, signature_out, self) == 0 {
            Err(Error::new(ErrorKind::NormalizeSignature))
        } else {
            Ok(signature_out)
        }
    }
}

pub struct Ecdsa;

impl Ecdsa {
    pub unsafe fn verify(
        ctx: *const Context,
        sig: *const Signature,
        msg32: *const c_uchar,
        public_key: *const PublicKey,
    ) -> Result<()> {
        if bindings::secp256k1_ecdsa_verify(ctx, sig, msg32, public_key) == 0 {
            Err(Error::new(ErrorKind::VerifySignature))
        } else {
            Ok(())
        }
    }

    pub unsafe fn sign(
        ctx: *const Context,
        msg32: *const c_uchar,
        private_key: *const c_uchar,
    ) -> Result<*mut Signature> {
        let sig = Signature::new();
        let nonce_fp = bindings::secp256k1_nonce_function_rfc6979;
        if bindings::secp256k1_ecdsa_sign(ctx, sig, msg32, private_key, nonce_fp, core::ptr::null())
            == 0
        {
            Err(Error::new(ErrorKind::SignMessage))
        } else {
            Ok(sig)
        }
    }
}
