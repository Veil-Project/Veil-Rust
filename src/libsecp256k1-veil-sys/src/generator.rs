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

use crate::bindings::generator::*;
use crate::c_types::*;
use crate::{Context, Error, ErrorKind, Result};

/// Opaque data structure that stores a base point
///
/// The exact representation of data inside is implementation defined and not
/// guaranteed to be portable between different platforms or versions. It is
/// however guaranteed to be 33 bytes in size, and can be safely copied/moved.
/// If you need to convert to a format suitable for storage or transmission, use
/// the secp256k1_generator_serialize_*.
///
/// Furthermore, it is guaranteed to identical points will have identical
/// representation, so they can be memcmp'ed.
#[repr(C)]
pub struct Generator([c_uchar; 33]);

impl Generator {
    pub unsafe fn new() -> *mut Self {
        &mut Self([0; 33]) as *mut Self
    }

    pub unsafe fn parse(ctx: *const Context, input: *const c_uchar) -> Result<*mut Self> {
        let commit = Self::new();
        if secp256k1_generator_parse(ctx, commit, input) == 0 {
            Err(Error::new(ErrorKind::ParseGenerator))
        } else {
            Ok(commit)
        }
    }

    pub unsafe fn serialize(ctx: *const Context, commit: *const Generator) -> Result<[u8; 33]> {
        let mut output = [0 as c_uchar; 33];
        secp256k1_generator_serialize(ctx, output.as_mut_ptr(), commit);
        Ok(output)
    }

    pub unsafe fn generate(ctx: *const Context, seed32: *const c_uchar) -> Result<*mut Self> {
        let generator = Self::new();
        if secp256k1_generator_generate(ctx, generator, seed32) == 0 {
            Err(Error::new(ErrorKind::BadSeed))
        } else {
            Ok(generator)
        }
    }

    pub unsafe fn generate_blinded(
        ctx: *const Context,
        seed32: *const c_uchar,
        blind32: *const c_uchar,
    ) -> Result<*mut Self> {
        let generator = Self::new();
        if secp256k1_generator_generate_blinded(ctx, generator, seed32, blind32) == 0 {
            Err(Error::new(ErrorKind::BadSeed))
        } else {
            Ok(generator)
        }
    }
}
