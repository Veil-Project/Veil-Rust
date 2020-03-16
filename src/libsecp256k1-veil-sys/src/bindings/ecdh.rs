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

use crate::c_types::{c_int, c_uchar};
use crate::{Context, PublicKey};

#[link(name = "secp256k1-veil")]
extern "C" {
    /// Compute an EC Diffie-Hellman secret in constant time.
    ///
    /// # Argument
    ///
    /// `ctx` - A pointer to a context object (cannot be NULL).
    ///
    /// # Out
    ///
    /// `result` - A 32--byte array which will be populated by an ECDH secret
    /// computed from the point and scalar.
    ///
    /// # In
    ///
    /// `public_key` - A pointer to a secp256k1_pubkey containing an initialized
    /// public key.
    /// `private_key` - A 32-byte scalar with which to multiply the point.
    ///
    /// # Returns
    ///
    /// `1` - Exponentiation was successful.
    /// `0` - Scalar was invalid (zero or overflow).
    pub fn secp256k1_ecdh(
        ctx: *const Context,
        result: *mut c_uchar,
        public_key: *const PublicKey,
        private_key: *const c_uchar,
    ) -> c_int;
}
