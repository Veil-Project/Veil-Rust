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

use crate::c_types::{c_int, c_uchar, size_t, uint64_t};
use crate::{Context, Generator};

#[repr(C)]
pub struct PedersenCommitment([c_uchar; 33]);

#[link(name = "secp256k1-veil")]
extern "C" {
    // Not sure if this is needed.
    pub static mut SECP256K1_GENERATOR_H: *const Generator;

    pub fn secp256k1_pedersen_commitment_parse(
        ctx: *const Context,
        commit: *mut PedersenCommitment,
        input: *const c_uchar,
    ) -> c_int;

    pub fn secp256k1_pedersen_commitment_serialize(
        ctx: *const Context,
        output: *mut c_uchar,
        commit: *const PedersenCommitment,
    ) -> c_int;

    pub fn secp256k1_pedersen_commitment_initialize(ctx: *mut Context);

    pub fn secp256k1_pedersen_commit(
        ctx: *const Context,
        commit: *mut PedersenCommitment,
        blind: *const c_uchar,
        value: uint64_t,
        gen: *const Generator,
    ) -> c_int;

    pub fn secp256k1_pedersen_blind_sum(
        ctx: *const Context,
        blind_out: *mut c_uchar,
        blinds: *const *const c_uchar,
        n: size_t,
        n_positive: size_t,
    ) -> c_int;

    pub fn secp256k1_pedersen_commitment_sum(
        ctx: *const Context,
        sum_out: *mut PedersenCommitment,
        commits: *const *const PedersenCommitment,
        n: size_t,
    ) -> c_int;

    pub fn secp256k1_pedersen_verify_tally(
        ctx: *const Context,
        commits: *const *const PedersenCommitment,
        p_cnt: size_t,
        n_commits: *const *const PedersenCommitment,
        n_cnt: size_t,
    ) -> c_int;

    pub fn secp256k1_pedersen_blind_generator_blind_sum(
        ctx: *const Context,
        value: *const uint64_t,
        Generator_blind: *const *const c_uchar,
        blinding_factor: *const *mut c_uchar,
        n_total: size_t,
        n_inputs: size_t,
    ) -> c_int;

    pub fn secp256k1_rangeproof_context_initialize(ctx: *mut Context);

    pub fn secp256k1_rangeproof_verify(
        ctx: *const Context,
        min_value: *mut uint64_t,
        max_value: *mut uint64_t,
        commit: *const PedersenCommitment,
        proof: *const c_uchar,
        p_len: size_t,
        extra_commit: *const c_uchar,
        extra_commit_len: size_t,
        gen: *const Generator,
    ) -> c_int;

    pub fn secp256k1_rangeproof_rewind(
        ctx: *const Context,
        blind_out: *mut c_uchar,
        value_out: *mut uint64_t,
        message_out: *mut c_uchar,
        out_len: *mut size_t,
        nonce: *const c_uchar,
        min_value: *mut uint64_t,
        max_value: *mut uint64_t,
        commit: *const PedersenCommitment,
        proof: *const c_uchar,
        p_len: size_t,
        extra_commit: *const c_uchar,
        extra_commit_len: size_t,
        gen: *const Generator,
    ) -> c_int;

    pub fn secp256k1_rangeproof_sign(
        ctx: *const Context,
        proof: *mut c_uchar,
        p_len: *mut size_t,
        min_value: uint64_t,
        commit: *const PedersenCommitment,
        blind: *const c_uchar,
        nonce: *const c_uchar,
        exp: c_int,
        min_bits: c_int,
        value: uint64_t,
        message: *const c_uchar,
        msg_len: size_t,
        extra_commit: *const c_uchar,
        extra_commit_len: size_t,
        gen: *const Generator,
    ) -> c_int;

    pub fn secp256k1_rangeproof_info(
        ctx: *const Context,
        exp: *mut c_int,
        mantissa: *mut c_int,
        min_value: *mut uint64_t,
        max_value: *mut uint64_t,
        proof: *const c_uchar,
        p_len: size_t,
    ) -> c_int;
}
