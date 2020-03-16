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
use crate::{Context, PublicKey, Signature};

#[repr(C)]
pub struct RecoverableSignature([c_uchar; 65]);

#[link(name = "secp256k1-veil")]
extern "C" {
    pub fn secp256k1_ecdsa_recoverable_signature_parse_compact(
        ctx: *const Context,
        sig: *mut RecoverableSignature,
        input64: *const c_uchar,
        rec_id: c_int,
    ) -> c_int;

    pub fn secp256k1_ecdsa_recoverable_signature_convert(
        ctx: *const Context,
        sig: *mut Signature,
        sig_in: *const RecoverableSignature,
    ) -> c_int;

    pub fn secp256k1_ecdsa_recoverable_signature_serialize_compact(
        ctx: *const Context,
        output64: *mut c_uchar,
        rec_id: *mut c_int,
        sig: *const RecoverableSignature,
    ) -> c_int;

    pub fn secp256k1_ecdsa_sign_recoverable(
        ctx: *const Context,
        sig: *mut RecoverableSignature,
        msg32: *const c_uchar,
        sec_key: *const c_uchar,
        nonce_fp: NonceFunction,
        n_data: *const c_void,
    ) -> c_int;

    pub fn secp256k1_ecdsa_recover(
        ctx: *const Context,
        pub_key: *mut PublicKey,
        sig: *const RecoverableSignature,
        msg32: *const c_uchar,
    ) -> c_int;
}
