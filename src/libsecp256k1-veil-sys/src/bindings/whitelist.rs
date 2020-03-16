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
use crate::c_types::{c_int, c_uchar, c_void, size_t};
use crate::{Context, PublicKey};

const WHITELIST_MAX_N_KEYS: usize = 256;

#[repr(C)]
pub struct Secp256k1WhitelistSignature {
    n_keys: size_t,
    data: [c_uchar; 32 * (1 + WHITELIST_MAX_N_KEYS)],
}

#[link(name = "secp256k1-veil")]
extern "C" {
    pub fn secp256k1_whitelist_signature_parse(
        ctx: *const Context,
        sig: *mut Secp256k1WhitelistSignature,
        input: *const c_uchar,
    ) -> c_int;

    pub fn secp256k1_whitelist_signature_n_keys(sig: *const Secp256k1WhitelistSignature) -> usize;

    pub fn secp256k1_whitelist_signature_serialize(
        ctx: *const Context,
        output: *mut c_uchar,
        sig: *const Secp256k1WhitelistSignature,
    ) -> c_int;

    pub fn secp256k1_whitelist_sign(
        ctx: *const Context,
        sig: *mut Secp256k1WhitelistSignature,
        online_pub_keys: *const PublicKey,
        offline_pub_keys: *const PublicKey,
        n_keys: usize,
        sub_pub_key: *const PublicKey,
        online_sec_key: *const c_uchar,
        summed_sec_key: *const c_uchar,
        index: usize,
        nonce_fp: NonceFunction,
        nonce_data: *const c_void,
    ) -> c_int;

    pub fn secp256k1_whitelist_verify(
        ctx: *const Context,
        sig: *const Secp256k1WhitelistSignature,
        online_pubkeys: *const PublicKey,
        offline_pubkeys: *const PublicKey,
        sub_pubkey: *const PublicKey,
    ) -> c_int;
}
