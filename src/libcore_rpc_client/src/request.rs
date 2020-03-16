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

use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use mimir_net::{http1_1, json_rpc};
use serde_json::json;

use crate::error::{Error, ErrorKind};
use crate::veild_structs::{
    PartiallySignedTransaction, PreviousTransaction, SigHashType, Transaction,
};
use crate::Result;

#[derive(Debug)]
pub struct Request(json_rpc::Request);

impl Request {
    pub fn parse(http: http1_1::Request) -> Result<Self> {
        let json: json_rpc::Request = serde_json::from_str(http.body().unwrap())?;
        Ok(Self(json))
    }

    // Blockchain
    pub fn best_block_hash() -> Self {
        Self(
            json_rpc::Request::builder()
                .method("getbestblockhash")
                .build(),
        )
    }

    pub fn block(hash: &str, verbosity: Option<usize>) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("getblock")
                .params(json!([hash, verbosity]))
                .build(),
        )
    }

    pub fn blockchain_info() -> Self {
        Self(
            json_rpc::Request::builder()
                .method("getblockchaininfo")
                .build(),
        )
    }

    pub fn block_count() -> Self {
        Self(json_rpc::Request::builder().method("getblockcount").build())
    }

    pub fn block_hash(hash: usize) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("getblockhash")
                .params(json!([hash]))
                .build(),
        )
    }

    pub fn block_header(hash: &str, verbose: Option<bool>) -> Self {
        let json = json!([hash, verbose]).to_string();
        Self(
            json_rpc::Request::builder()
                .method("getblockheader")
                .params(json)
                .build(),
        )
    }

    pub fn block_stats<H: ToString>(hash_or_height: H, stats: Option<String>) -> Self {
        let json = json!([hash_or_height.to_string(), stats]).to_string();
        Self(
            json_rpc::Request::builder()
                .method("getblockstats")
                .params(json)
                .build(),
        )
    }

    pub fn chain_tips() -> Self {
        Self(json_rpc::Request::builder().method("getchaintips").build())
    }

    pub fn chain_tx_stats(nblocks: Option<usize>, block_hash: Option<String>) -> Self {
        let json = json!([nblocks, block_hash]);
        Self(
            json_rpc::Request::builder()
                .method("getchaintxstats")
                .params(json)
                .build(),
        )
    }

    pub fn difficulty() -> Self {
        Self(json_rpc::Request::builder().method("getdifficulty").build())
    }

    pub fn mempool_ancestors(tx_id: String, verbose: Option<bool>) -> Self {
        let json = json!([tx_id, verbose]);
        Self(
            json_rpc::Request::builder()
                .method("getmempoolancestors")
                .params(json)
                .build(),
        )
    }

    pub fn mempool_descendants(tx_id: String, verbose: Option<bool>) -> Self {
        let json = json!([tx_id, verbose]);
        Self(
            json_rpc::Request::builder()
                .method("getmempoolancestors")
                .params(json)
                .build(),
        )
    }

    pub fn mempool_entry(tx_id: String) -> Self {
        let json = json!([tx_id]);
        Self(
            json_rpc::Request::builder()
                .method("getmempoolentry")
                .params(json)
                .build(),
        )
    }

    pub fn mempool_info() -> Self {
        Self(
            json_rpc::Request::builder()
                .method("getmempoolinfo")
                .build(),
        )
    }

    pub fn raw_mempool(verbose: Option<bool>) -> Self {
        let params = json!([verbose]);
        Self(
            json_rpc::Request::builder()
                .method("getrawmempool")
                .params(params)
                .build(),
        )
    }

    pub fn tx_out(tx_id: &str, v_out: u64, include_mempool: Option<bool>) -> Self {
        let params = json!([tx_id, v_out, include_mempool]);
        Self(
            json_rpc::Request::builder()
                .method("gettxout")
                .params(params)
                .build(),
        )
    }

    pub fn tx_out_proof(tx_ids: &[&str], block_hash: Option<&str>) -> Self {
        let params = json!([tx_ids, block_hash]);
        Self(
            json_rpc::Request::builder()
                .method("gettxoutproof")
                .params(params)
                .build(),
        )
    }

    pub fn tx_out_set_info() -> Self {
        Self(
            json_rpc::Request::builder()
                .method("gettxoutsetinfo")
                .build(),
        )
    }

    pub fn precious_block(hash: &str) -> Self {
        let params = json!([hash]);
        Self(
            json_rpc::Request::builder()
                .method("preciousblock")
                .params(params)
                .build(),
        )
    }

    pub fn save_mempool() -> Self {
        Self(json_rpc::Request::builder().method("savemempool").build())
    }

    pub fn verify_chain(check_level: Option<u8>, nblocks: Option<u8>) -> Self {
        let params = json!([check_level, nblocks]);
        Self(
            json_rpc::Request::builder()
                .method("verifychain")
                .params(params)
                .build(),
        )
    }

    pub fn verify_tx_out_proof(proof: &str) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("verifytxoutproof")
                .params(json!([proof]))
                .build(),
        )
    }

    // Control
    pub fn memory_info() -> Self {
        Self(json_rpc::Request::builder().method("getmemoryinfo").build())
    }

    pub fn uptime() -> Self {
        Self(json_rpc::Request::builder().method("uptime").build())
    }

    // Generating
    pub fn generate_blocks(nblocks: usize, max_tries: Option<usize>) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("generate")
                .params(json!([nblocks, max_tries]))
                .build(),
        )
    }

    pub fn generate_blocks_continuous(b: Option<bool>, threads: Option<usize>) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("generatecontinuous")
                .params(json!([b, threads]))
                .build(),
        )
    }

    pub fn generate_blocks_to_address(
        nblocks: usize,
        address: String,
        max_tries: Option<usize>,
    ) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("generatetoaddress")
                .params(json!([nblocks, address, max_tries]))
                .build(),
        )
    }

    // Mining
    pub fn block_template(template: Option<String>) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("getblocktemplate")
                .params(json!([template]))
                .build(),
        )
    }

    pub fn mining_info() -> Self {
        Self(json_rpc::Request::builder().method("getmininginfo").build())
    }

    pub fn network_hashps(nblocks: Option<usize>, height: Option<usize>) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("getnetworkhashps")
                .params(json!([nblocks, height]))
                .build(),
        )
    }

    pub fn prioritize_transaction(tx_id: &str, fee_delta: isize) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("prioritisetransaction")
                .params(json!([tx_id, "null", fee_delta]))
                .build(),
        )
    }

    pub fn submit_block(hex: String) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("submitblock")
                .params(json![hex])
                .build(),
        )
    }

    // Network
    pub fn add_node(node: SocketAddr, command: &str) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("addnode")
                .params(json!([node, command]))
                .build(),
        )
    }

    pub fn clear_banned() -> Self {
        Self(json_rpc::Request::builder().method("clearbanned").build())
    }

    pub fn disconnect_node(address: Option<SocketAddr>, id: Option<usize>) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("disconnectnode")
                .params(json!([address, id]))
                .build(),
        )
    }

    pub fn added_node_info(ip: IpAddr) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("getaddednodeinfo")
                .params(json!([ip]))
                .build(),
        )
    }

    pub fn connection_count() -> Self {
        Self(
            json_rpc::Request::builder()
                .method("getconnectioncount")
                .build(),
        )
    }

    pub fn net_totals() -> Self {
        Self(json_rpc::Request::builder().method("getnettotals").build())
    }

    pub fn network_info() -> Self {
        Self(
            json_rpc::Request::builder()
                .method("getnetworkinfo")
                .build(),
        )
    }

    pub fn peer_info() -> Self {
        Self(json_rpc::Request::builder().method("getpeerinfo").build())
    }

    pub fn banned_nodes() -> Self {
        Self(json_rpc::Request::builder().method("listbanned").build())
    }

    pub fn ping() -> Self {
        Self(json_rpc::Request::builder().method("ping").build())
    }

    pub fn set_ban(
        ip: IpAddr,
        command: &str,
        ban_time: Option<Duration>,
        absolute: Option<bool>,
    ) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("setban")
                .params(json!([ip, command, ban_time, absolute]))
                .build(),
        )
    }

    pub fn set_network_active(state: bool) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("setnetworkactive")
                .params(json!([state]))
                .build(),
        )
    }

    pub fn combine_pst(txs: &[Transaction]) -> Result<Self> {
        let txs_json = serde_json::to_string(txs)?;
        Ok(Self(
            json_rpc::Request::builder()
                .method("combinepsbt")
                .params(json!([txs_json]))
                .build(),
        ))
    }

    pub fn combine_raw_transactions(txs: &[Transaction]) -> Result<Self> {
        let txs_json = serde_json::to_string(txs)?;

        Ok(Self(
            json_rpc::Request::builder()
                .method("combinerawtransaction")
                .params(json!([txs_json]))
                .build(),
        ))
    }

    pub fn convert_transaction_to_pst(tx: &Transaction) -> Result<Self> {
        let tx_json = serde_json::to_string(tx)?;

        Ok(Self(
            json_rpc::Request::builder()
                .method("converttopsbt")
                .params(json!([tx_json]))
                .build(),
        ))
    }

    // TODO: This actually doesn't yet exist for RingCT in core library.
    // pub fn create_raw_transaction() {}

    pub fn decode_pst(pst: &str) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("decodepsbt")
                .params(json!([pst]))
                .build(),
        )
    }

    pub fn decode_raw_transaction(raw_tx: &str) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("decoderawtransaction")
                .params(json!([raw_tx]))
                .build(),
        )
    }

    pub fn decode_script(script: &str) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("decodescript")
                .params(json!([script]))
                .build(),
        )
    }

    pub fn finalize_pst(pst: &str, extract: Option<bool>) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("finalizepsbt")
                .params(json!([pst, extract]))
                .build(),
        )
    }

    pub fn fund_raw_transaction(
        hex: &str,
        options: Option<&str>,
        is_witness: Option<bool>,
    ) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("fundrawtransaction")
                .params(json!([hex, options, is_witness]))
                .build(),
        )
    }

    // TODO: fundrawtransactionfrom "input_type" "hexstring" input_amounts output_amounts ( options iswitness )

    pub fn raw_transaction(tx_id: &str, verbose: Option<bool>, block_hash: Option<&str>) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("getrawtransaction")
                .params(json!([tx_id, verbose, block_hash]))
                .build(),
        )
    }

    pub fn send_raw_transaction(hex: &str, allow_high_fees: Option<bool>) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("sendrawtransaction")
                .params(json!([hex, allow_high_fees]))
                .build(),
        )
    }

    pub fn sign_raw_transaction(
        hex: &str,
        private_keys: &[String],
        prev_transactions: Option<&[PreviousTransaction]>,
        sig_hash_type: Option<SigHashType>,
    ) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("signrawtransactionwithkey")
                .params(json!([hex, private_keys, prev_transactions, sig_hash_type]))
                .build(),
        )
    }

    pub fn test_mempool_accept(txs: &[String], allow_high_fees: Option<bool>) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("testmempoolaccept")
                .params(json!([txs, allow_high_fees]))
                .build(),
        )
    }

    pub fn verify_commitment(commitment: &str, blind: &str, amount: u64) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("verifycommitment")
                .params(json!([commitment, blind, amount]))
                .build(),
        )
    }

    // FIXME: Test what `returndecoded` does. Check help information for the method in CLI.
    pub fn verify_raw_transaction(
        hex: &str,
        prev_transactions: Option<&[PreviousTransaction]>,
    ) -> Self {
        Self(
            json_rpc::Request::builder()
                .method("verifyrawtransaction")
                .params(json!([hex, prev_transactions]))
                .build(),
        )
    }

    pub fn set_id(&mut self, id: String) {
        self.0.set_id(id);
    }

    pub fn protocol(&self) -> &json_rpc::Version {
        self.0.protocol()
    }

    pub fn method(&self) -> &str {
        self.0.method()
    }

    pub fn params(&self) -> Option<&str> {
        self.0.params()
    }

    pub fn id(&self) -> Option<&str> {
        self.0.id()
    }
}

impl ToString for Request {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

// // NOTE get rid of all argument lines when I hard code all the requests into the RPC.
// #[derive(Debug)]
// pub enum RequestKind {
//     /// Returns the hash of the best (tip) block in the longest blockchain.
//     BestBlockHash,
//     /// Returns the block associated to the hash provided with all tx information.
//     ///
//     /// ### Arguments
//     /// 1. **Required** block hash `String` of exactly 32 bytes in length else it will error.
//     Block(String),
//     /// Returns the state of the blockchain.
//     BlockchainInfo,

//     /// Returns the total count of the blocks in the blockchain.
//     BlockCount,

//     /// Returns the hash of the block.
//     BlockHash(usize),

//     /// Returns information about all known tips in the block tree.
//     ChainTips,

//     /// Compute statistics about the total number and rate of txs in the chain.
//     ChainTxStats(Option<usize>, Option<String>),

//     /// If tx is in the mempool, returns all in-mempool ancestors.
//     MempoolAncestors(String),

//     /// If tx is in the mempool, returns all in-mempool descendants.
//     MempoolDescendants(String),

//     /// Returns mempool entry data.
//     MempoolEntry(String),

//     /// Returns details on the state of the memory pool.
//     MempoolInfo,

//     /// Returns all transaction IDs in the memory pool.
//     RawMempool,

//     /// Returns all details about an unspent transaction output.
//     ///
//     /// *Warning* It currently only works on basecoin transactions.
//     // NOTE may extend this and bring this feature back with the Electrum-like server.
//     TxOut(String, u64),

//     /// Returns a hex-encoded proof that "txid" was included in a block.
//     TxOutProof(Vec<String>, Option<String>),

//     /// Returns statistics about the unspent transactions output set.
//     TxOutSetInfo,

//     /// Returns the Veil supply in satoshis held in zerocoins at a specified block height.
//     ZerocoinSupply(Option<u64>),

//     /// Returns an object containing information about RAM memory usage.
//     MemoryInfo,

//     /// Returns the data required to contract a new block to work on.
//     // Should make a builder for this
//     BlockTemplate,

//     /// Returns mining related information.
//     MiningInfo,

//     /// Returns various state info regarding the P2P network.
//     NetworkInfo,

//     /// Returns data about each connected network node.
//     PeerInfo,

//     /// Returns all banned peers.
//     Banned,

//     /// Returns the raw transaction data.
//     RawTransaction(String, Option<String>),
//     //
//     // /// Returns the list of addresses assigned to specified label.
//     // Not sure if this actually returns something that I can parse.
//     // AddressesByLabel(String),
//     //
//     /// Return information about the given veil address. Some information requires the address
//     /// to be in the wallet.
//     AddressInfo(String),
// }

// #[derive(Debug)]
// pub struct Request {
//     request_kind: RequestKind,
//     method: String,
//     params: Option<String>,
// }

// impl Request {
//     pub fn new(request_kind: RequestKind) -> Result<Self> {
//         // let rq_tp = request_type.clone();
//         let (req, params) = match &request_kind {
//             RequestKind::BestBlockHash => ("getbestblockhash", None),
//             RequestKind::Block(h) => ("getblock", Some(Self::format_block_params(h)?)),
//             RequestKind::BlockchainInfo => ("getblockchaininfo", None),
//             RequestKind::BlockCount => ("getblockcount", None),
//             RequestKind::BlockHash(h) => ("getblockhash", Some(Self::format_block_hash(h)?)),
//             RequestKind::ChainTips => ("getchaintips", None),
//             RequestKind::ChainTxStats(w, b) => {
//                 ("getchaintxstats", Some(Self::format_chain_tx_stats(w, b)))
//             }
//             RequestKind::MempoolAncestors(t) => {
//                 ("getmempoolancestors", Some(json!([t, true]).to_string()))
//             }
//             RequestKind::MempoolDescendants(t) => {
//                 ("getmempooldescendants", Some(json!([t, true]).to_string()))
//             }
//             RequestKind::MempoolEntry(t) => ("getmempoolentry", Some(json!([t, true]).to_string())),
//             RequestKind::MempoolInfo => ("getmempoolinfo", None),
//             RequestKind::RawMempool => ("getrawmempool", Some(json!([true]).to_string())),
//             RequestKind::TxOut(t, n) => ("gettxout", Some(json!([t, n, true]).to_string())),
//             RequestKind::TxOutProof(t, b) => (
//                 "gettxoutproof",
//                 Some(Self::format_tx_out_proof(t, b.as_ref())),
//             ),
//             RequestKind::TxOutSetInfo => ("gettxoutsetinfo", None),
//             RequestKind::ZerocoinSupply(h) => (
//                 "getzerocoinsupply",
//                 Some(Self::format_zerocoin_supply(h.as_ref())),
//             ),
//             RequestKind::MemoryInfo => ("getmemoryinfo", None),
//             RequestKind::BlockTemplate => ("getblocktemplate", None),
//             RequestKind::MiningInfo => ("getmininginfo", None),
//             RequestKind::NetworkInfo => ("getnetworkinfo", None),
//             RequestKind::PeerInfo => ("getpeerinfo", None),
//             RequestKind::Banned => ("listbanned", None),
//             RequestKind::RawTransaction(t, b) => (
//                 "getrawtransaction",
//                 Some(Self::format_raw_transaction(t, b)),
//             ),
//             // RequestKind::AddressesByLabel(l) => {
//             //     ("getaddressesbylabel", Some(json!([l]).to_string()))
//             // }
//             RequestKind::AddressInfo(a) => ("getaddressinfo", Some(json!([a]).to_string())),
//         };

//         Ok(Self {
//             request_kind,
//             method: req.to_owned(),
//             // params: params.map(|p| p.to_owned()),
//             params,
//         })
//     }

//     fn format_block_params(block_hash: &str) -> Result<String> {
//         if block_hash.len() != 64 {
//             let e = Error::new(ErrorKind::Params(
//                 "block hash size is not 32 bytes".to_owned(),
//             ));

//             return Err(e);
//         }

//         let verbosity = 2;
//         let json = json!([block_hash, verbosity]);

//         Ok(json.to_string())
//     }

//     fn format_block_hash(block_height: &usize) -> Result<String> {
//         Ok(json!([block_height]).to_string())
//     }

//     fn format_chain_tx_stats(window_size: &Option<usize>, block_hash: &Option<String>) -> String {
//         match window_size {
//             Some(w) => match block_hash {
//                 Some(b) => return json!([w, b]).to_string(),
//                 None => return json!([w]).to_string(),
//             },
//             None => match block_hash {
//                 Some(b) => return json!([43200, b]).to_string(),
//                 None => return json!([43200]).to_string(),
//             },
//         }
//     }

//     fn format_tx_out_proof(tx_ids: &Vec<String>, height: Option<&String>) -> String {
//         match height {
//             Some(h) => return json!([tx_ids, h]).to_string(),
//             None => return json!([tx_ids]).to_string(),
//         }
//     }

//     fn format_zerocoin_supply(height: Option<&u64>) -> String {
//         match height {
//             Some(h) => return json!([h]).to_string(),
//             None => return json!([]).to_string(),
//         }
//     }

//     fn format_raw_transaction(tx_id: &String, block_hash: &Option<String>) -> String {
//         let verbose = true;

//         match block_hash {
//             Some(b) => return json!([tx_id, verbose, b]).to_string(),
//             None => return json!([tx_id, verbose]).to_string(),
//         }
//     }

//     pub fn request_type(&self) -> &RequestKind {
//         &self.request_kind
//     }

//     pub fn method(&self) -> &str {
//         &self.method
//     }

//     pub fn params(&self) -> Option<&String> {
//         self.params.as_ref()
//     }
// }
