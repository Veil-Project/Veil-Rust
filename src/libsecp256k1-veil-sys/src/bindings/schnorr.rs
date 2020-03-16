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

use super::NonceFunction;
use crate::c_types::{c_int, c_uchar, c_void};
use crate::{Context, PublicKey};

#[link(name = "secp256k1-veil")]
extern "C" {
    pub fn secp256k1_schnorr_sign(
        ctx: *const Context,
        sig64: *mut c_uchar,
        msg32: *const c_uchar,
        sec_key: *const c_uchar,
        nonce_fp: NonceFunction,
        n_data: *const c_void,
    ) -> c_int;

    pub fn secp256k1_schnorr_verify(
        ctx: *const Context,
        sig64: *const c_uchar,
        msg32: *const c_uchar,
        pub_key: *const PublicKey,
    ) -> c_int;

    pub fn secp256k1_schnorr_recover(
        ctx: *const Context,
        pub_key: *mut PublicKey,
        sig64: *const c_uchar,
        msg32: *const c_uchar,
    ) -> c_int;

    pub fn secp256k1_schnorr_generate_nonce_pair(
        ctx: *const Context,
        pub_nonce: *mut PublicKey,
        priv_nonce32: *mut c_uchar,
        msg32: *const c_uchar,
        sec32: *const c_uchar,
        nonce_fp: NonceFunction,
        nonce_data: *const c_void,
    ) -> c_int;

    pub fn secp256k1_schnorr_partial_sign(
        ctx: *const Context,
        sig64: *mut c_uchar,
        msg32: *const c_uchar,
        sec32: *const c_uchar,
        pub_nonce_others: *const PublicKey,
        sec_nonce32: *const c_uchar,
    ) -> c_int;

    pub fn secp256k1_schnorr_partial_combine(
        ctx: *const Context,
        sig64: *mut c_uchar,
        sig64_sin: *const *const c_uchar,
        n: usize,
    ) -> c_int;
}
