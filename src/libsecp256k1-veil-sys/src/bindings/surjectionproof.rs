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

use crate::c_types::{c_int, c_uchar, size_t};
use crate::{Context, Generator};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurjectionProof {
    n_inputs: size_t,
    used_inputs: [c_uchar; 32],
    data: [c_uchar; 8224],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FixedAssetTag([c_uchar; 32]);

#[link(name = "secp256k1-veil")]
extern "C" {
    pub fn secp256k1_surjectionproof_parse(
        ctx: *const Context,
        proof: *mut SurjectionProof,
        input: *const c_uchar,
        input_len: size_t,
    ) -> c_int;

    pub fn secp256k1_surjectionproof_serialize(
        ctx: *const Context,
        output: *mut c_uchar,
        output_len: *mut size_t,
        proof: *const SurjectionProof,
    ) -> c_int;

    pub fn secp256k1_surjectionproof_n_total_inputs(
        ctx: *const Context,
        proof: *const SurjectionProof,
    ) -> size_t;

    pub fn secp256k1_surjectionproof_n_used_inputs(
        ctx: *const Context,
        proof: *const SurjectionProof,
    ) -> size_t;

    pub fn secp256k1_surjectionproof_serialized_size(
        ctx: *const Context,
        proof: *const SurjectionProof,
    ) -> size_t;

    pub fn secp256k1_surjectionproof_initialize(
        ctx: *const Context,
        proof: *mut SurjectionProof,
        input_index: *mut size_t,
        fixed_input_tags: *const FixedAssetTag,
        n_input_tags: size_t,
        n_input_tags_to_use: size_t,
        fixed_output_tag: *const FixedAssetTag,
        n_max_iterations: size_t,
        random_seed32: *const c_uchar,
    ) -> c_int;

    pub fn secp256k1_surjectionproof_generate(
        ctx: *const Context,
        proof: *mut SurjectionProof,
        ephemeral_input_tags: *const Generator,
        n_ephemeral_input_tags: usize,
        ephemeral_output_tag: *const Generator,
        input_index: usize,
        input_blinding_key: *const c_uchar,
        output_blinding_key: *const c_uchar,
    ) -> c_int;

    pub fn secp256k1_surjectionproof_verify(
        ctx: *const Context,
        proof: *const SurjectionProof,
        ephemeral_input_tags: *const Generator,
        n_ephemeral_input_tags: size_t,
        ephemeral_output_tag: *const Generator,
    ) -> c_int;
}
