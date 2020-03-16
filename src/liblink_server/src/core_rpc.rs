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
use crate::Result;
use std::net;
use std::path;
use veil_core_rpc::{client, Request};

pub static BLOCK_BATCH_SIZE: usize = 100;

#[derive(Clone)]
pub struct Config {
    pub username: Option<String>,
    pub password: Option<String>,
    pub addr: net::SocketAddr,
    pub cookie_path: Option<path::PathBuf>,
    pub block_batch_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: None,
            password: None,
            addr: format!(
                "{}:{}",
                client::CORE_RPC_IP.to_owned(),
                client::CORE_RPC_PORT
            )
            .parse()
            .unwrap(),
            cookie_path: Some(veil::paths::cookie_file()),
            block_batch_size: 100,
        }
    }
}

impl From<&Config> for client::Config {
    fn from(cfg: &Config) -> Self {
        let cfg: Config = cfg.clone();
        Self {
            username: cfg.username,
            password: cfg.password,
            addr: cfg.addr,
            cookie_path: cfg.cookie_path,
        }
    }
}

pub struct Client {
    inner: veil_core_rpc::Client<net::TcpStream>,
    block_batch_size: usize,
}

impl Client {
    pub fn new(cfg: Config) -> Result<Self> {
        let inner_cfg = client::Config::from(&cfg);
        let inner = client::Client::new(inner_cfg)?;
        Ok(Self {
            inner,
            block_batch_size: cfg.block_batch_size,
        })
    }

    fn request(&mut self, req: &veil_core_rpc::Request) -> Result<serde_json::Value> {
        self.inner.send(&req)?;
        Ok(self.inner.receive()?.verify()?)
    }

    fn blockchain_headers(&mut self) -> Result<serde_json::Value> {
        self.request(&veil_core_rpc::Request::block_count())
    }

    fn blockchain_transaction_broadcast(
        &mut self,
        raw_tx: &str,
        high_fees: bool,
    ) -> Result<serde_json::Value> {
        self.request(&veil_core_rpc::Request::send_raw_transaction(
            raw_tx,
            Some(high_fees),
        ))
    }

    // TODO: Should grab from it's own DB and put into it's own data structure.
    fn blockchain_transaction(
        &mut self,
        hash: &str,
        verbose: Option<bool>,
        block_hash: Option<&str>,
    ) -> Result<serde_json::Value> {
        self.request(&veil_core_rpc::Request::raw_transaction(
            hash, verbose, block_hash,
        ))
    }

    // TODO: When data structure is sorted.
    //    fn blockchain_transaction_merkle()

    // TODO: Get from storage. Returns the transaction + vout at that index.
    //    fn blockchain_transaction_index(&mut self, index: usize) -> Result<serde_json::Value>

    // TODO: Get keyimage from storage, returning height confirmed in, commitments, and txid.
    //    fn blockcahin_keyimage()

    fn blockchain_block_header(&mut self, height: usize) -> Result<serde_json::Value> {
        let hash = self.request(&Request::block_hash(height))?.to_string();
        self.request(&Request::block_header(&hash, None))
    }

    // Returns port of ZMQ PUB port to subscribe to.
    //    fn subscribe_blockchain_block_count(&mut self) ->

    //    fn subscribe_blockchain_headers()

    //    fn subscribe_block

    //    fn subscribe_transaction
}
