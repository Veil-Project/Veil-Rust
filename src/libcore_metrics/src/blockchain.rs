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

use std::collections::HashMap;
use veil_core_rpc_client::client::RPCClient;

#[derive(Debug, Default)]
pub struct BlockchainStats {
    chain: String,
    blocks: u64,
    headers: u64,
    chain_tip_count: u64,
    pow_difficulty: f64,
    pos_difficulty: f64,
    network_hashps: f64,
    pooled_tx: u64,
    median_time: u64,
    money_supply: u64,
    denom_supply: HashMap<String, u64>,
    size_on_disk: u64,
    verification_progress: f64,
    veild_warnings: String,
    warnings: Vec<String>,
}

impl BlockchainStats {
    pub fn parse(rpc: &mut RPCClient) -> Self {
        // let stats = Self::default();
        let bc_info = rpc.get_blockchain_info().unwrap();
        let min_info = rpc.get_mining_info().unwrap();
        let chain_tips = rpc.get_chain_tips().unwrap();

        let mut denom_supply: HashMap<String, u64> = HashMap::new();
        for denom in bc_info.zerocoin_supply() {
            denom_supply.insert(denom.name().to_string(), denom.amount());
        }

        Self {
            chain: bc_info.chain().to_string(),
            blocks: bc_info.blocks(),
            headers: bc_info.headers(),
            chain_tip_count: chain_tips.len() as u64,
            pos_difficulty: bc_info.pos_difficulty(),
            pow_difficulty: bc_info.pow_difficulty(),
            network_hashps: min_info.network_hashps(),
            pooled_tx: min_info.pooled_tx(),
            median_time: bc_info.median_time(),
            money_supply: bc_info.money_supply(),
            denom_supply,
            size_on_disk: bc_info.size_on_disk(),
            verification_progress: bc_info.verification_progress(),
            veild_warnings: bc_info.warnings().to_string(),
            warnings: vec![],
        }
    }
}

pub struct BlockStats {
    feerate_percentiles: HashMap<String, f64>,
    height: u64,
    ins: u64,
    maxfee: u64,
    maxfeerate: u64,
    maxtxsize: u64,
    avgfee: u64,
    avgfeerate: u64,
    avgtxsize: u64,
    medianfee: u64,
    mediantime: u64,
    mediantxsize: u64,
    minfee: u64,
    minfeerate: u64,
    mintxsize: u64,
    outs: u64,
    subsidy: u64,
    swtxs: u64,
    time: u64,
    total_out: u64,
    total_size: u64,
    total_weight: u64,
    sw_total_size: u64,
    sw_total_weight: u64,
    txs: u64,
    utxo_increase: u64,
    utxo_size_inc: u64,
}

pub struct MempoolStats {
    size: u64,
    bytes: u64,
    usage: u64,
    maxmempool: u64,
    mempoolminfee: f64,
    minrelaytxfee: f64,
}
