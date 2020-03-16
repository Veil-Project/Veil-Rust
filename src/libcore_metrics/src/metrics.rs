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

use std::convert::TryInto;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::{thread, time};

use prometheus::{self, Counter, Encoder, IntGauge, Opts, Registry, TextEncoder};
use tiny_http::{Request, Response, Server};
use veil_core_rpc_client::client::RPCClient;
use veil_core_rpc_client::veild_structs::*;

pub struct VeildMetricsConfig {
    addr: SocketAddr,
    blockchain_enabled: bool,
    network_enabled: bool,
    wallet_enabled: bool,
}

pub struct VeildMetrics {
    registry: Registry,
    addr: SocketAddr,
    blockchain_enabled: bool,
    network_enabled: bool,
    wallet_enabled: bool,
}

impl VeildMetrics {
    pub fn new(config: VeildMetricsConfig) -> Self {
        Self {
            registry: prometheus::Registry::new(),
            addr: config.addr,
            blockchain_enabled: config.blockchain_enabled,
            network_enabled: config.network_enabled,
            wallet_enabled: config.wallet_enabled,
        }
    }

    pub fn run(&self) {
        let server = Server::http("127.0.0.1:50184").unwrap();
        let metrics_thread = thread::Builder::new();
        metrics_thread.name("metrics server".to_string());
        let r = self.registry.clone();

        thread::spawn(move || loop {
            let req = server.recv().unwrap();
            Self::handle_request(&r, req);
        });
    }

    fn handle_request(r: &Registry, req: Request) {
        let mut buffer = vec![];
        TextEncoder::new().encode(&r.gather(), &mut buffer).unwrap();
        let response = Response::from_data(buffer);
        req.respond(response);
    }

    fn get_metrics(&self) {}
}
