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

use crate::{bindings, c_types::*, Error, ErrorKind, Result};

/// Opaque data structure that holds context information (precomputed tables etc.).
///
/// The purpose of context structures is to cache large precomputed data tables
/// that are expensive to construct, and also to maintain the randomization data
/// for blinding.
///
/// Do not create a new context object for each operation, as construction is
/// far slower than all other API calls (~100 times slower than an ECDSA
/// verification).
///
/// A constructed context can safely be used from multiple threads
/// simultaneously, but API call that take a non-const pointer to a context
/// need exclusive access to it. In particular this is the case for
/// secp256k1_context_destroy and secp256k1_context_randomize.
///
/// Regarding randomization, either do it once at creation time (in which case
/// you do not need any locking for the other calls), or use a read-write lock.
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Context(pub c_int);

impl Context {
    pub unsafe fn new(flag: c_uint) -> *mut Self {
        bindings::secp256k1_context_create(flag)
    }

    pub unsafe fn clone_data(&mut self) -> *mut Self {
        bindings::secp256k1_context_clone(self as *mut Self)
    }

    pub unsafe fn destroy(&mut self) {
        bindings::secp256k1_context_destroy(self);
    }

    pub unsafe fn randomize(&mut self, seed: [c_uchar; 32]) -> Result<()> {
        if bindings::secp256k1_context_randomize(self as *mut Self, seed.as_ptr()) == 0 {
            Err(Error::new(ErrorKind::RandomizeContext))
        } else {
            Ok(())
        }
    }
}
