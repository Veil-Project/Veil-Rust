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

// TODO: Json subscription notifications
// TODO: Version negotiation between server and client
// TODO: Json RPC 1.0 and 2.0
// TODO: Genesis compare

// TODO: ERROR: Must have it's own error wrapper.
// TODO: LEDGER: Must be able to store transactions from RPC
// TODO: MEMPOOL: Must have it's own mempool.
// TODO: METRICS: Must have metrics relating to itself.
// TODO: STORAGE: Must have an alternative blockchain storage method for RingCT.
// TODO: SERVER: Must be able to connect to a peer and communicate with JSON RPC
// 2.0
// TODO: Potential Into Inner to consume struct and return the inner?

// PHASE 1
// Connect to RPC server and parse all RingCT transactions into a key value
// storage

pub mod client;
pub mod config;
pub mod core_rpc;
pub mod daemon;
pub mod error;
pub mod methods;
pub mod paths;
pub mod server;
pub mod storage;
pub mod threads;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;
