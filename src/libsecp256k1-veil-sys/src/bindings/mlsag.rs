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

use crate::c_types::{c_int, size_t, uint8_t};
use crate::Context;

// typedef struct {
//     unsigned char data[33];
// } secp256k1_pedersen_commitment;
//
// int32_t nSigInputs, nSigRingSize;
// secp256k1_pedersen_commitment splitInputCommit;
//
// m = std::vector<uint8_t> vm(nCols * nRows * 33)
// sk = uint8_t blindSum[32];
// n_outs = size_t
// n_blinded = size_t // same number as n_outs?
// n_cols = size_t nCols = nSigRingSize
// n_rows = size_t nRows = nSigInputs + 1
// pcm_in = std::vector<const uint8_t*> vpInCommits(nCols * nSigInputs)
// pcm_out = std::vector<const uint8_t*> vpOutCommits;
// blinds = std::vector<const uint8_t*> vpBlinds
//
// secp256k1_prepare_mlsag(
//     m: &vm[0], // Input: Matrix of public keys, "ec_point"
//     sk: blindSum, // Input: Vector of secret keys
//     n_outs: 1, // Number of pcm_out
//     n_blinded: 1, // None of pcm_out?
//     n_cols: nCols, // Number of columns, ring size
//     n_rows: nRows, // Number of rows, signature inputs
//     pcm_in: &vpInCommits[0], // In commits?? matrix map?
//     pcm_out: &pSplitCommit, // The pedersen commit.
//     blinds: &vpBlinds[0] // The blinds ... ?
// )
//
// In/Out is the... m and sk, both the -last- of those rows are filled by the function

/// Add the commitments to the matrix containing the public keys (the messages
/// to sign) and the blinding values to the vector containing the private keys.
#[link(name = "secp256k1-veil")]
extern "C" {
    pub fn secp256k1_prepare_mlsag(
        m: *mut uint8_t,
        sk: *mut uint8_t,
        n_outs: size_t,
        n_blinded: size_t,
        n_cols: size_t,
        n_rows: size_t,
        pcm_in: *mut *const uint8_t,
        pcm_out: *mut *const uint8_t,
        blinds: *mut *const uint8_t,
    ) -> c_int;

    pub fn secp256k1_get_keyimage(
        ctx: *const Context,
        ki: *mut uint8_t,
        pk: *const uint8_t,
        sk: *const uint8_t,
    ) -> c_int;

    pub fn secp256k1_generate_mlsag(
        ctx: *const Context,
        ki: *mut uint8_t,
        pc: *mut uint8_t,
        ps: *mut uint8_t,
        nonce: *const uint8_t,
        pre_image: *const uint8_t,
        n_cols: size_t,
        n_rows: size_t,
        index: size_t,
        sk: *mut *const uint8_t,
        pk: *const uint8_t,
    ) -> c_int;

    pub fn secp256k1_verify_mlsag(
        ctx: *const Context,
        pre_image: *const uint8_t,
        n_cols: size_t,
        n_rows: size_t,
        pk: *const uint8_t,
        ki: *const uint8_t,
        pc: *const uint8_t,
        ps: *const uint8_t,
    ) -> c_int;
}
