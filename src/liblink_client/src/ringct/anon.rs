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

static MIN_RING_SIZE: usize = 3;
static MAX_RING_SIZE: usize = 32;
static MAX_ANON_INPUTS: usize = 32;
static ANON_FEE_MULTIPLIER: usize = 2;

// Note this should return a result
// MLSAG is a new ring signature protocol.
// Multilayered Linkable Spontaneous Anonymous Group
pub fn verify_mlsag(tx: &Transaction, &state: ValidationState) -> bool {}

// Note this should return a result
// Should move to TxMemPool
pub fn add_key_images_to_mempool(tx: &Transaction, &pool: TxMemPool) -> bool {}

// Note this should return a result
// Should mvoe to TxMempool
pub fn remove_key_images_to_mempool(hash: &[u8], txin: &TxIn, pool: &TxMemPool) -> bool {}

// Note this should return a result
pub fn all_anon_inputs_unknown(tx: &Transaction, state: &ValidationState) -> bool {}

// Note this should return a result
pub fn rollback_rct_inputs(tx: &Transaction, state: &ValidationState) -> bool {}

pub fn to_tx_rct_inputs(ptx: TransactionRef) -> Vec<Vec<OutPoint>> {}

pub fn to_rct_inputs(txin: Txin) -> Vec<OutPoint> {}

pub fn to_ringct_inputs(txin: TxIn, inputs: Vec<Vec<OutPoint>>) -> bool {}
