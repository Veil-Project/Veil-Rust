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

use crate::{request::Request, response::Response};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{BufRead, BufReader, Lines};
use std::net;
use std::path;
use std::time::Duration;
//#[cfg(test)]
use crate::{test::test_server::TestStream, veild_structs::*, Result, RpcStream};
use mimir_net::http1_1;
use serde_json;
use veil;

pub const CORE_RPC_IP: &str = "127.0.0.1";
pub const CORE_RPC_PORT: u16 = 58812;

// TODO: Verbose needs to be a different method
// TODO: Expand serde_json to have from_response
// TODO: Implement this where needed: pub fn connect(addr:
// TODO: Config feature that uses Serde JSON or not. Internally will use Serde JSON. Will change
// outputs.

pub struct Config {
    pub username: Option<String>,
    pub password: Option<String>,
    pub addr: net::SocketAddr,
    pub cookie_path: Option<path::PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: None,
            password: None,
            addr: format!("{}:{}", CORE_RPC_IP.to_owned(), CORE_RPC_PORT)
                .parse()
                .unwrap(),
            cookie_path: Some(veil::paths::cookie_file()),
        }
    }
}

pub struct Client<R: RpcStream> {
    auth: Box<String>,
    stream: R,
    rx: Lines<BufReader<R>>,
    json_id: u64,
}

impl Client<net::TcpStream> {
    pub fn new(config: Config) -> Result<Self> {
        let stream = net::TcpStream::connect(config.addr)?;
        let rx = BufReader::new(stream.try_clone()?);
        let auth = format!(
            "Basic {}",
            base64::encode(&format!(
                "{}:{}",
                config.username.unwrap_or_else(|| "".to_string()),
                config.password.unwrap_or_else(|| "".to_string())
            ))
        );

        Ok(Self {
            auth: Box::new(auth),
            stream,
            rx: rx.lines(),
            json_id: 0,
        })
    }
}

//#[cfg(test)]
impl Client<TestStream> {
    pub fn new_test() -> Self {
        let stream = TestStream::new();
        Self {
            auth: Box::new("empty".to_string()),
            rx: BufReader::new(stream.clone()).lines(),
            stream,
            json_id: 0,
        }
    }
}

impl<R: RpcStream> Client<R> {
    pub fn send(&mut self, req: &Request) -> Result<()> {
        let req = req.to_string();
        let builder = http1_1::Request::builder()
            .version(http1_1::Version::V1_1)
            .method(http1_1::Method::Post)
            .resource("/")
            .header("Authorization", self.auth.as_ref())
            .header("Content-Length", req.len())
            .body(req);

        let http_req = builder.build().to_string();
        println!("{}", http_req);
        let bytes = http_req.as_bytes();
        self.stream.write_all(bytes)?;
        println!("Done writing");
        Ok(())
    }

    // TODO: Make into "TryFrom" for http1_1::Response or Response.
    pub fn receive(&mut self) -> Result<Response> {
        let mut iter = self.rx.by_ref().map(|l| l.unwrap());
        let mut res_builder = http1_1::Response::builder();
        // TODO: move into res_builder.
        match iter.next() {
            Some(res_line) => {
                let mut iter_http = res_line.split_whitespace();

                if let Some(ver) = iter_http.next() {
                    if ver != "HTTP/1.1" {
                        println!("Throw unexpected http1_1 version");
                    }
                } else {
                    println!("No response error");
                }

                if let Some(code) = iter_http.next() {
                    let status = code.parse::<u16>().unwrap();
                    res_builder.status(http1_1::StatusCode(status));
                } else {
                    println!("Error while receiving.");
                }
            }
            None => println!("No response"), // err (and this works!)
        }

        let mut header_end = false;
        let mut body = String::new();
        let mut headers = HashMap::new();
        for line in iter {
            if header_end {
                body = line;
                break;
            }

            if line.is_empty() {
                header_end = true;
            } else {
                let parts: Vec<&str> = line.splitn(2, ": ").collect();

                if parts.len() != 2 {
                    println!("http1_1 header error");
                } else {
                    headers.insert(parts[0].to_owned(), parts[1].to_owned());
                }
            }
        }
        res_builder.headers(headers);
        res_builder.body(body);

        Response::parse(res_builder.build())
    }

    pub fn request(&mut self, req: &Request) -> Result<Response> {
        self.send(&req)?;
        self.receive()
    }

    pub fn batch_request(&mut self, req: &[Request]) -> Result<()> {
        Ok(())
    }

    pub fn best_block_hash(&mut self) -> Result<String> {
        let req = Request::best_block_hash();
        let res = self.request(&req)?.verify()?;
        // Ok("temp".to_string())
        Ok(res.to_string())
    }

    // TODO: Verbosity option
    pub fn block(&mut self, hash: &str) -> Result<Block> {
        let req = Request::block(hash, Some(3));
        let res = self.request(&req)?.verify()?;
        Block::try_from(res)
    }

    pub fn blockchain_info(&mut self) -> Result<BlockchainInfo> {
        let req = Request::blockchain_info();
        let res = self.request(&req)?.verify()?;
        BlockchainInfo::try_from(res)
    }

    pub fn block_count(&mut self) -> Result<u64> {
        let req = Request::block_count();
        let res = self.request(&req)?.verify()?;
        Ok(res.as_u64().unwrap())
    }

    pub fn block_hash(&mut self, height: usize) -> Result<String> {
        let req = Request::block_hash(height);
        let res = self.request(&req)?.verify()?;
        Ok(res.to_string())
    }

    pub fn block_header(&mut self, hash: &str) -> Result<BlockHeader> {
        let req = Request::block_header(hash, Some(true));
        let res = self.request(&req)?.verify()?;
        BlockHeader::try_from(res)
    }

    pub fn block_header_serial(&mut self, hash: &str) -> Result<String> {
        let req = Request::block_header(hash, Some(false));
        let res = self.request(&req)?.verify()?;
        Ok(res.to_string())
    }

    pub fn block_stats<H: ToString>(
        &mut self,
        hash_or_height: H,
        stats: Option<String>,
    ) -> Result<BlockStats> {
        let req = Request::block_stats(hash_or_height, stats);
        let res = self.request(&req)?.verify()?;
        BlockStats::try_from(res)
    }

    pub fn chain_tips(&mut self) -> Result<Vec<ChainTip>> {
        let req = Request::chain_tips();
        let res = self.request(&req)?.verify()?;
        let chain_tips: Vec<ChainTip> = serde_json::from_value(res)?;
        Ok(chain_tips)
    }

    pub fn chain_tx_stats(
        &mut self,
        nblocks: Option<usize>,
        block_hash: Option<String>,
    ) -> Result<ChainTxStats> {
        let req = Request::chain_tx_stats(nblocks, block_hash);
        let res = self.request(&req)?.verify()?;
        ChainTxStats::try_from(res)
    }

    pub fn difficulty(&mut self) -> Result<f64> {
        let req = Request::difficulty();
        let res = self.request(&req)?.verify()?;
        Ok(res.as_f64().unwrap())
    }

    pub fn mempool_ancestors(
        &mut self,
        tx_id: String,
        verbose: Option<bool>,
    ) -> Result<Vec<MempoolTx>> {
        let req = Request::mempool_ancestors(tx_id, verbose);
        let res = self.request(&req)?.verify()?;
        let ancestors: Vec<MempoolTx> = serde_json::from_value(res)?;
        Ok(ancestors)
    }

    pub fn mempool_descendants(
        &mut self,
        tx_id: String,
        verbose: Option<bool>,
    ) -> Result<Vec<MempoolTx>> {
        let req = Request::mempool_ancestors(tx_id, verbose);
        let res = self.request(&req)?.verify()?;
        let descendants: Vec<MempoolTx> = serde_json::from_value(res)?;
        Ok(descendants)
    }

    pub fn mempool_entry(&mut self, tx_id: String) -> Result<MempoolTx> {
        let req = Request::mempool_entry(tx_id);
        let res = self.request(&req)?.verify()?;
        MempoolTx::try_from(res)
    }

    pub fn mempool_info(&mut self) -> Result<MempoolInfo> {
        let req = Request::mempool_info();
        let res = self.request(&req)?.verify()?;
        MempoolInfo::try_from(res)
    }

    pub fn raw_mempool(&mut self) -> Result<Vec<String>> {
        let req = Request::raw_mempool(None);
        let res = self.request(&req)?.verify()?;
        let txs: Vec<String> = serde_json::from_value(res)?;
        Ok(txs)
    }

    pub fn raw_mempool_verbose(&mut self) -> Result<Option<Vec<MempoolTx>>> {
        let req = Request::raw_mempool(Some(true));
        let res = self.request(&req)?.verify()?;
        let txs_result: Option<Vec<MempoolTx>> = serde_json::from_value(res).ok();
        Ok(txs_result)
    }

    pub fn tx_out(
        &mut self,
        tx_id: &str,
        v_out: u64,
        include_mempool: Option<bool>,
    ) -> Result<UnspentTransactionOut> {
        let req = Request::tx_out(tx_id, v_out, include_mempool);
        let res = self.request(&req)?.verify()?;
        UnspentTransactionOut::try_from(res)
    }

    pub fn tx_out_proof(&mut self, tx_ids: &[&str], block_hash: Option<&str>) -> Result<String> {
        let req = Request::tx_out_proof(tx_ids, block_hash);
        let res = self.request(&req)?.verify()?;
        let proof: String = serde_json::from_value(res)?;
        Ok(proof)
    }

    pub fn tx_out_set_info(&mut self) -> Result<TransactionOutSetInfo> {
        let req = Request::tx_out_set_info();
        let res = self.request(&req)?.verify()?;
        TransactionOutSetInfo::try_from(res)
    }

    pub fn precious_block(&mut self, hash: &str) -> Result<()> {
        let req = Request::precious_block(hash);
        self.request(&req)?.verify()?;
        Ok(())
    }

    pub fn save_mempool(&mut self) -> Result<()> {
        let req = Request::save_mempool();
        self.request(&req)?.verify()?;
        Ok(())
    }

    pub fn verify_chain(&mut self, check_level: Option<u8>, nblocks: Option<u8>) -> Result<bool> {
        let req = Request::verify_chain(check_level, nblocks);
        let res = self.request(&req)?.verify()?;
        Ok(res.as_bool().unwrap())
    }

    pub fn verify_tx_out_proof(&mut self, proof: &str) -> Result<Vec<String>> {
        let req = Request::verify_tx_out_proof(proof);
        let res = self.request(&req)?.verify()?;
        let tx_ids: Vec<String> = serde_json::from_value(res)?;
        Ok(tx_ids)
    }

    pub fn memory_info(&mut self) -> Result<MemoryInfo> {
        let req = Request::memory_info();
        let res = self.request(&req)?.verify()?;
        MemoryInfo::try_from(res)
    }

    pub fn uptime(&mut self) -> Result<Duration> {
        let req = Request::uptime();
        let res = self.request(&req)?.verify()?;
        let time: u64 = serde_json::from_value(res)?;
        Ok(Duration::new(time, 0))
    }

    pub fn generate_blocks(
        &mut self,
        nblocks: usize,
        max_tries: Option<usize>,
        address: Option<String>,
    ) -> Result<Vec<String>> {
        let req = if let Some(a) = address {
            Request::generate_blocks_to_address(nblocks, a, max_tries)
        } else {
            Request::generate_blocks(nblocks, max_tries)
        };
        let res = self.request(&req)?.verify()?;
        let hashes: Vec<String> = serde_json::from_value(res)?;
        Ok(hashes)
    }

    pub fn generate_blocks_continuous(
        &mut self,
        b: Option<bool>,
        threads: Option<usize>,
    ) -> Result<()> {
        let req = Request::generate_blocks_continuous(b, threads);
        self.send(&req)?;
        Ok(())
    }

    pub fn block_template(&mut self, template: Option<String>) -> Result<BlockTemplate> {
        let req = Request::block_template(template);
        let res = self.request(&req)?.verify()?;
        BlockTemplate::try_from(res)
    }

    pub fn mining_info(&mut self) -> Result<MiningInfo> {
        let req = Request::mining_info();
        let res = self.request(&req)?.verify()?;
        MiningInfo::try_from(res)
    }

    pub fn network_hashps(&mut self, nblocks: Option<usize>, height: Option<usize>) -> Result<f64> {
        let req = Request::network_hashps(nblocks, height);
        let res = self.request(&req)?.verify()?;
        Ok(res.as_f64().unwrap())
    }

    pub fn prioritize_transaction(&mut self, tx_id: &str, fee_delta: isize) -> Result<bool> {
        let req = Request::prioritize_transaction(tx_id, fee_delta);
        let res = self.request(&req)?.verify()?;
        Ok(res.as_bool().unwrap())
    }

    pub fn submit_block(&mut self, hex: String) -> Result<()> {
        let req = Request::submit_block(hex);
        self.request(&req)?.verify()?;
        Ok(())
    }

    pub fn add_node(&mut self, node: net::SocketAddr) -> Result<()> {
        let req = Request::add_node(node, "add");
        self.request(&req)?.verify()?;
        Ok(())
    }

    pub fn remove_node(&mut self, node: net::SocketAddr) -> Result<()> {
        let req = Request::add_node(node, "remove");
        self.request(&req)?.verify()?;
        Ok(())
    }

    pub fn try_node(&mut self, node: net::SocketAddr) -> Result<()> {
        let req = Request::add_node(node, "onetry");
        self.request(&req)?.verify()?;
        Ok(())
    }

    pub fn clear_banned(&mut self) -> Result<()> {
        let req = Request::clear_banned();
        self.request(&req)?.verify()?;
        Ok(())
    }

    pub fn disconnect_node(
        &mut self,
        address: Option<net::SocketAddr>,
        id: Option<usize>,
    ) -> Result<()> {
        let req = Request::disconnect_node(address, id);
        self.request(&req)?.verify()?;
        Ok(())
    }

    pub fn added_node_info(&mut self, ip: net::IpAddr) -> Result<AddedNodeInfo> {
        let req = Request::added_node_info(ip);
        let res = self.request(&req)?.verify()?;
        AddedNodeInfo::try_from(res)
    }

    pub fn connection_count(&mut self) -> Result<u64> {
        let req = Request::connection_count();
        let res = self.request(&req)?.verify()?;
        Ok(res.as_u64().unwrap())
    }

    pub fn net_totals(&mut self) -> Result<NetTotals> {
        let req = Request::net_totals();
        let res = self.request(&req)?.verify()?;
        NetTotals::try_from(res)
    }

    pub fn network_info(&mut self) -> Result<NetworkInfo> {
        let req = Request::network_info();
        let res = self.request(&req)?.verify()?;
        NetworkInfo::try_from(res)
    }

    pub fn peer_info(&mut self) -> Result<Vec<PeerInfo>> {
        let req = Request::peer_info();
        let res = self.request(&req)?.verify()?;
        let peer_info: Vec<PeerInfo> = serde_json::from_value(res)?;
        Ok(peer_info)
    }

    // NOTE: This might work
    pub fn banned_nodes(&mut self) -> Result<Vec<net::IpAddr>> {
        let req = Request::banned_nodes();
        let res = self.request(&req)?.verify()?;
        let banned: Vec<net::IpAddr> = serde_json::from_value(res)?;
        Ok(banned)
    }

    pub fn ping(&mut self) -> Result<()> {
        let req = Request::ping();
        self.send(&req)
    }

    pub fn add_ban(
        &mut self,
        ip: net::IpAddr,
        ban_time: Option<Duration>,
        absolute: Option<bool>,
    ) -> Result<()> {
        let req = Request::set_ban(ip, "add", ban_time, absolute);
        self.send(&req)
    }

    pub fn remove_ban(
        &mut self,
        ip: net::IpAddr,
        ban_time: Option<Duration>,
        absolute: Option<bool>,
    ) -> Result<()> {
        let req = Request::set_ban(ip, "remove", ban_time, absolute);
        self.send(&req)
    }

    pub fn set_network_active(&mut self, state: bool) -> Result<()> {
        let req = Request::set_network_active(state);
        self.send(&req)
    }

    pub fn combine_pst(&mut self, txs: &[Transaction]) -> Result<String> {
        let req = Request::combine_pst(txs)?;
        let res = self.request(&req)?.verify()?;
        Ok(res.to_string())
    }

    pub fn combine_raw_transactions(&mut self, txs: &[Transaction]) -> Result<String> {
        let req = Request::combine_raw_transactions(txs)?;
        let res = self.request(&req)?.verify()?;
        Ok(res.to_string())
    }

    pub fn convert_transaction_to_pst(
        &mut self,
        tx: &Transaction,
    ) -> Result<PartiallySignedTransaction> {
        let req = Request::convert_transaction_to_pst(tx)?;
        let res = self.request(&req)?.verify()?;
        PartiallySignedTransaction::try_from(res)
    }

    pub fn decode_pst(&mut self, pst: &str) -> Result<PartiallySignedTransaction> {
        let req = Request::decode_pst(pst);
        let res = self.request(&req)?.verify()?;
        PartiallySignedTransaction::try_from(res)
    }

    pub fn decode_raw_transaction(&mut self, raw_tx: &str) -> Result<Transaction> {
        let req = Request::decode_raw_transaction(raw_tx);
        let res = self.request(&req)?.verify()?;
        Transaction::try_from(res)
    }

    pub fn decode_script(&mut self, script: &str) -> Result<ScriptPubKey> {
        let req = Request::decode_script(script);
        let res = self.request(&req)?.verify()?;
        ScriptPubKey::try_from(res)
    }

    pub fn finalize_pst(
        &mut self,
        pst: &str,
        extract: Option<bool>,
    ) -> Result<FinalPartiallySignedTransaction> {
        let req = Request::finalize_pst(pst, extract);
        let res = self.request(&req)?.verify()?;
        FinalPartiallySignedTransaction::try_from(res)
    }

    pub fn fund_raw_transaction(
        &mut self,
        hex: &str,
        options: Option<&str>,
        is_witness: Option<bool>,
    ) -> Result<FundedTransaction> {
        let req = Request::fund_raw_transaction(hex, options, is_witness);
        let res = self.request(&req)?.verify()?;
        FundedTransaction::try_from(res)
    }

    pub fn raw_transaction(
        &mut self,
        tx_id: &str,
        block_hash: Option<&str>,
    ) -> Result<Transaction> {
        let req = Request::raw_transaction(tx_id, Some(true), block_hash);
        let res = self.request(&req)?.verify()?;
        Transaction::try_from(res)
    }

    pub fn raw_transaction_serialized(
        &mut self,
        tx_id: &str,
        block_hash: Option<&str>,
    ) -> Result<String> {
        let req = Request::raw_transaction(tx_id, Some(false), block_hash);
        let res = self.request(&req)?.verify()?;
        Ok(res.to_string())
    }

    pub fn send_raw_transaction(
        &mut self,
        hex: &str,
        allow_high_fees: Option<bool>,
    ) -> Result<String> {
        let req = Request::send_raw_transaction(hex, allow_high_fees);
        let res = self.request(&req)?.verify()?;
        Ok(res.to_string())
    }

    pub fn sign_raw_transaction(
        &mut self,
        hex: &str,
        private_keys: &[String],
        prev_transactions: Option<&[PreviousTransaction]>,
        sig_hash_type: Option<SigHashType>,
    ) -> Result<FinalSignedTransaction> {
        let req =
            Request::sign_raw_transaction(hex, private_keys, prev_transactions, sig_hash_type);
        let res = self.request(&req)?.verify()?;
        FinalSignedTransaction::try_from(res)
    }

    pub fn test_mempool_accept(
        &mut self,
        txs: &[String],
        allow_high_fees: Option<bool>,
    ) -> Result<MempoolAcceptTest> {
        let req = Request::test_mempool_accept(txs, allow_high_fees);
        let res = self.request(&req)?.verify()?;
        MempoolAcceptTest::try_from(res)
    }

    pub fn verify_commitment(
        &mut self,
        commitment: &str,
        blind: &str,
        amount: u64,
    ) -> Result<CommitmentResult> {
        let req = Request::verify_commitment(commitment, blind, amount);
        let res = self.request(&req)?.verify()?;
        CommitmentResult::try_from(res)
    }

    pub fn verify_raw_transaction(
        &mut self,
        hex: &str,
        prev_transactions: Option<&[PreviousTransaction]>,
    ) -> Result<FinalSignedTransaction> {
        let req = Request::verify_raw_transaction(hex, prev_transactions);
        let res = self.request(&req)?.verify()?;
        FinalSignedTransaction::try_from(res)
    }
}
