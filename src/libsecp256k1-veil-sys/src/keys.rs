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

use crate::{bindings, c_types::*, Context, Error, ErrorKind, Result};

/// Opaque data structure that holds a parsed and valid public key.
///
/// The exact representation of data inside is implementation defined and not
/// guaranteed to be portable between different platforms or versions. It is
/// however guaranteed to be 64 bytes in size, and can be safely copied/moved.
/// If you need to convert to a format suitable for storage, transmission, or
/// comparison, use secp256k1_ec_pubkey_serialize and secp256k1_ec_pubkey_parse.
#[derive(Clone)] // Todo implement debug
#[repr(C)]
pub struct PublicKey(pub [c_uchar; 64]);

impl PublicKey {
    pub unsafe fn new() -> *mut Self {
        &mut Self([0; 64]) as *mut Self
    }

    pub unsafe fn parse(
        ctx: *const Context,
        input: *const c_uchar,
        input_len: size_t,
    ) -> Result<*mut Self> {
        let public_key = Self::new();
        if bindings::secp256k1_ec_pubkey_parse(ctx, public_key, input, input_len) == 0 {
            Err(Error::new(ErrorKind::ParseKey))
        } else {
            Ok(public_key)
        }
    }

    pub unsafe fn combine(
        ctx: *const Context,
        ins: *const *const PublicKey,
        n: size_t,
    ) -> Result<*mut Self> {
        let public_key = Self::new();
        if bindings::secp256k1_ec_pubkey_combine(ctx, public_key, ins, n) == 0 {
            Err(Error::new(ErrorKind::CombinePublicKey))
        } else {
            Ok(public_key)
        }
    }

    pub unsafe fn serialize_compressed(&self, ctx: *const Context) -> Result<[u8; 33]> {
        let flags = bindings::EC_COMPRESSED;
        let output = &mut [0u8; 33];
        let output_len = &mut 0;

        bindings::secp256k1_ec_pubkey_serialize(
            ctx,
            output.as_mut_ptr(),
            output_len,
            self,
            flags as c_uint,
        );

        if *output_len != 33 {
            Err(Error::new(ErrorKind::SerializeCompressedPublicKey))
        } else {
            Ok(*output)
        }
    }

    pub unsafe fn serialize_uncompressed(&self, ctx: *const Context) -> Result<[u8; 65]> {
        let flags = bindings::EC_UNCOMPRESSED;
        let output = &mut [0u8; 65];
        let output_len: &mut size_t = &mut 0;

        bindings::secp256k1_ec_pubkey_serialize(
            ctx,
            output.as_mut_ptr(),
            output_len as *mut size_t,
            self,
            flags as c_uint,
        );

        if *output_len != 65 {
            Err(Error::new(ErrorKind::SerializeUncompressedPublicKey))
        } else {
            Ok(*output)
        }
    }

    pub unsafe fn create(ctx: *const Context, secret_key: *const c_uchar) -> Result<*mut Self> {
        let public_key = Self::new();
        if bindings::secp256k1_ec_pubkey_create(ctx, public_key as *mut PublicKey, secret_key) == 0
        {
            Err(Error::new(ErrorKind::CreatePublicKey))
        } else {
            Ok(public_key)
        }
    }

    pub unsafe fn negate(&mut self, ctx: *const Context) -> Result<()> {
        bindings::secp256k1_ec_pubkey_negate(ctx, self); // can not fail
        Ok(())
    }

    pub unsafe fn tweak_add(&mut self, ctx: *const Context, tweak: *const c_uchar) -> Result<()> {
        if bindings::secp256k1_ec_pubkey_tweak_add(ctx, self as *mut Self, tweak) == 0 {
            Err(Error::new(ErrorKind::TweakKey))
        } else {
            Ok(())
        }
    }

    pub unsafe fn tweak_mul(&mut self, ctx: *const Context, tweak: *const c_uchar) -> Result<()> {
        if bindings::secp256k1_ec_pubkey_tweak_mul(ctx, self as *mut Self, tweak) == 0 {
            Err(Error::new(ErrorKind::TweakKey))
        } else {
            Ok(())
        }
    }
}

pub struct PrivateKey;

impl PrivateKey {
    pub unsafe fn verify(ctx: *const Context, private_key: *const c_uchar) -> Result<()> {
        if bindings::secp256k1_ec_seckey_verify(ctx, private_key) == 0 {
            Err(Error::new(ErrorKind::VerifyPrivateKey))
        } else {
            Ok(())
        }
    }

    pub unsafe fn negate(ctx: *const Context, private_key: *mut c_uchar) -> Result<()> {
        bindings::secp256k1_ec_privkey_negate(ctx, private_key); // can not fail
        Ok(())
    }

    pub unsafe fn tweak_add(
        ctx: *const Context,
        private_key: *mut c_uchar,
        tweak: *const c_uchar,
    ) -> Result<()> {
        if bindings::secp256k1_ec_privkey_tweak_add(ctx, private_key, tweak) == 0 {
            Err(Error::new(ErrorKind::TweakKey))
        } else {
            Ok(())
        }
    }

    pub unsafe fn tweak_mul(
        ctx: *const Context,
        private_key: *mut c_uchar,
        tweak: *const c_uchar,
    ) -> Result<()> {
        if bindings::secp256k1_ec_privkey_tweak_mul(ctx, private_key, tweak) == 0 {
            Err(Error::new(ErrorKind::TweakKey))
        } else {
            Ok(())
        }
    }
}
