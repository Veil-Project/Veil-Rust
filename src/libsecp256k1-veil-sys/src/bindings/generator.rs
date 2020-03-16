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
use crate::{Context, Generator};

#[link(name = "secp256k1-veil")]
extern "C" {
    /// Parse a 33-byte generator byte sequence into a generator object.
    ///
    /// # Argument
    ///
    /// * `ctx` - A Secp256k1 context object.
    ///
    /// # Out
    ///
    /// * `commit` - A pointer to the output generator object.
    ///
    /// # In
    ///
    /// * `input` - A pointer to a 33-byte serialized generator.
    ///
    /// # Returns
    ///
    /// * `1` - If input contains a valid signature.
    /// * `0` - If input contains an invalid signature.
    pub fn secp256k1_generator_parse(
        ctx: *const Context,
        commit: *mut Generator,
        input: *const c_uchar,
    ) -> c_int;

    /// Serialize a 33-byte generator into a serialized byte sequence.
    ///
    /// # Argument
    ///
    /// * `ctx` - A secp256k1 context object.
    ///
    /// # Out
    ///
    /// * `output` - A pointer to a 33-byte array.
    ///
    /// # In
    ///
    /// * `commit` - A pointer to a generator.
    ///
    /// # Returns
    ///
    /// Always returns 1.
    pub fn secp256k1_generator_serialize(
        ctx: *const Context,
        output: *mut c_uchar,
        commit: *const Generator,
    ) -> c_int;

    /// Generate a generator for the curve.
    ///
    /// # Argument
    ///
    /// * `ctx` - A secp256k1 context object.
    ///
    /// # Out
    ///
    /// * `generator` - A generator object.
    ///
    /// # In
    ///
    /// * `seed32` - A 32-byte seed.
    ///
    /// # Returns
    ///
    /// * `1` - If the seed is acceptable.
    /// * `0` - In the highly unlikely case that the seed is not acceptable.
    pub fn secp256k1_generator_generate(
        ctx: *const Context,
        generator: *mut Generator,
        seed32: *const c_uchar,
    ) -> c_int;

    /// Generate a blinded generator for the curve.
    ///
    /// The result is equivalent to first calling secp256k1_generator_generate,
    /// converting the result to a public key, calling secp256k1_ec_pubkey_tweak_add,
    /// and then converting back to generator form.
    ///
    /// # Argument
    ///
    /// * `ctx` - A secp256k1 context object.
    ///
    /// # Out
    ///
    /// * `generator` - A generator object.
    ///
    /// # In
    ///
    /// * `seed32` - A 32-byte seed.
    /// * `blind32` - A 32-byte secret value to blind the generator with.
    ///
    /// # Returns
    ///
    /// * `1` - If the seed is acceptable.
    /// * `0` - In the highly unlikely case that the seed is not acceptable.
    pub fn secp256k1_generator_generate_blinded(
        ctx: *const Context,
        generator: *mut Generator,
        seed32: *const c_uchar, // double check this on recent secp256k1 libraries
        blind32: *const c_uchar,
    ) -> c_int;
}
