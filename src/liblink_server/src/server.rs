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

use crate::core_rpc;
use crate::error::{Error, ErrorKind};
use crate::threads;
use crate::Result;
use clap;
use mimir_net::{http1_1, json_rpc};
use quiche;
use serde_json;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{self, BufRead, BufReader};
use std::net;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time;

// Note: https://spdk.io/doc/jsonrpc.html

pub const DEFAULT_SERVER_AGENT: &str = "VeilLink";
pub const DEFAULT_SERVER_BANNER: &str = "Welcome to a standard Link service!";
pub const DEFAULT_SERVER_DONATION: &str = "This operator has not set a donation address :(.";
pub const DEFAULT_ADDRESS: &str = "127.0.0.1:4635";
pub const QUIC_PROTOCOL_VERSION: u32 = quiche::PROTOCOL_VERSION;

pub enum Message {
    Terminate,
}

pub struct Config {
    pub agent: String,
    pub banner: String,
    pub donation_address: String,
    pub addr: net::SocketAddr,
    pub rpc_cfg: core_rpc::Config,
    pub thread_count: Option<usize>,
    pub quic_config: quiche::Config,
}

impl Config {
    pub fn new() -> Result<Self> {
        //        let mut quic_config = quiche::Config::new(QUIC_PROTOCOL_VERSION)?;
        // Server gets ALPN from client, then returns what we will be communicating on.
        //        quic_config.set_application_protos(b"\x08h3\x08jsonrpc/2\x08link/1")?;

        Ok(Self {
            agent: DEFAULT_SERVER_AGENT.to_owned(),
            banner: DEFAULT_SERVER_BANNER.to_owned(),
            donation_address: DEFAULT_SERVER_DONATION.to_owned(),
            addr: DEFAULT_ADDRESS.parse().unwrap(),
            rpc_cfg: core_rpc::Config::default(),
            thread_count: Some(1),
            quic_config: quiche::Config::new(QUIC_PROTOCOL_VERSION)?,
        })
    }

    pub fn set_agent(&mut self, agent: String) {
        self.agent = agent;
    }

    pub fn set_banner(&mut self, banner: String) {
        self.banner = banner;
    }

    pub fn set_donation_address(&mut self, address: String) {
        self.donation_address = address;
    }

    pub fn set_addr(&mut self, addr: net::SocketAddr) {
        self.addr = addr;
    }

    pub fn set_thread_count(&mut self, count: Option<usize>) {
        self.thread_count = count;
    }

    pub fn load_cert_chain_from_pem_file(&mut self, file: &str) -> Result<()> {
        self.quic_config.load_cert_chain_from_pem_file(file)?;
        Ok(())
    }

    pub fn load_priv_key_from_pem_file(&mut self, file: &str) -> Result<()> {
        self.quic_config.load_priv_key_from_pem_file(file)?;
        Ok(())
    }

    pub fn set_grease(&mut self, grease: bool) {
        self.quic_config.grease(grease)
    }

    pub fn enable_log_keys(&mut self) {
        self.quic_config.log_keys();
    }

    pub fn set_max_idle_timeout(&mut self, v: u64) {
        self.quic_config.set_idle_timeout(v);
    }

    pub fn set_max_packet_size(&mut self, v: u64) {
        self.quic_config.set_max_packet_size(v);
    }
}

pub struct Server {
    addr: net::SocketAddr,
    agent: String,
    banner: String,
    donation_address: String,
    rpc: core_rpc::Client,
    pool: threads::WorkerPool,
    client_count: AtomicUsize,
    version: String,
}

impl Server {
    pub fn new(cfg: Config) -> Result<Self> {
        let mut pool_cfg = threads::WorkerPoolConfig {
            name: "Server".to_string(),
            size: cfg.thread_count,
            stack_size: None,
        };

        Ok(Server {
            addr: cfg.addr,
            agent: cfg.agent,
            banner: cfg.banner,
            donation_address: cfg.donation_address,
            rpc: core_rpc::Client::new(cfg.rpc_cfg)?,
            pool: threads::WorkerPool::new(pool_cfg)?,
            client_count: AtomicUsize::new(0),
            version: clap::crate_version!().to_string(),
        })
    }

    pub fn run(&mut self, rx: mpsc::Receiver<Message>) -> Result<()> {
        let listener = net::TcpListener::bind(self.addr)?;
        listener
            .set_nonblocking(true)
            .expect("Server::run cannot set non-blocking");

        for stream in listener.incoming() {
            match stream {
                Ok(s) => self.pool.execute(|| {
                    // Have states locked with pool of jobs?
                    self.handle_stream(s);
                }),
                Err(ref e) if e.kind == io::ErrorKind::WouldBlock => {
                    let recv = rx.try_recv();
                    match recv {
                        Ok(msg) => match msg {
                            Message::Terminate => break,
                        },
                        Err(e) => match e {
                            mpsc::TryRecvError::Empty => {
                                thread::sleep(time::Duration::from_millis(250));
                                continue;
                            }
                            mpsc::TryRecvError::Disconnected => {
                                break;
                            }
                        },
                    }
                }
                Err(e) => Err(Error::from(e)),
            }
        }

        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<()> {
        self.pool.shutdown()
    }

    pub fn agent(&self) -> Result<serde_json::Value> {
        Ok(serde_json::Value::from_str(&self.agent)?)
    }

    pub fn banner(&self) -> Result<serde_json::Value> {
        Ok(serde_json::Value::from_str(&self.banner)?)
    }

    pub fn donation(&self) -> Result<serde_json::Value> {
        Ok(serde_json::Value::from_str(&self.donation_address)?)
    }

    pub fn is_working(&self) -> bool {
        self.pool.is_working()
    }

    pub fn has_work(&self) -> bool {
        self.pool.has_work()
    }

    // TODO: Shared data needs to be atomics. Anything read.
    fn handle_stream(&self, stream: net::TcpStream) -> Result<()> {
        self.client_count.fetch_add(1, Ordering::SeqCst);

        let req = http1_1::Request::try_from(stream)?;

        if req.resource() != "/" {
            todo!(); // 404 Not found error
        }

        match req.version() {
            http1_1::Version::V1_1 => self.http1_1(&req),
            http1_1::Version::V2_0 => self.http2(&req),
            _ => todo!(), // 505 HTTP Version Not Supported
        }

        //        let body = match req.body() {
        //            Some(b) => b,
        //            None => todo!(), // 400 Bad request, no body, or empty.
        //        };
        //        let content_type = if let Some(ct) = req.headers().get("content-type") {
        //            if ct != "application/json" {
        //                todo!() // 415 Bad media type, others not supported.
        //            }
        //            ct.to_owned()
        //        } else {
        //            todo!() // 400 Bad request, requires Content-Type.
        //                    // Exit with error
        //        };
        //        let content_length = if let Some(cl) = req.headers().get("content-length") {
        //            let cl = usize::from_str(cl);
        //            if let Ok(cl) = cl {
        //                cl
        //            } else if cl != req.body()?.len() {
        //                todo!()
        //            } else {
        //                todo!() // 400 Bad request, can't parse into usize.
        //                        // Exit with error
        //            }
        //        } else {
        //            todo!() // 411 length required
        //                    // Exit with error
        //        };
        //        let resources: Vec<&str> = req.resource().split('/').collect();
        //        // We assume that if there are more than 2 resources, as we only handle two as no
        //        // sub-services currently exist, it'll be a 404 error.
        //        if resources[0] != "v1" || resources.len() != 2 {
        //            todo!() // 404 not found error
        //                    // Exit with error
        //        } else if req.method() != http1_1::Method::Post {
        //            todo!() // HTTP 405 Method not allow error
        //                    // Exit with error
        //        }
        //
        //        match resources[1] {
        //            "server" => todo!(),
        //            "blockchain" => todo!(),
        //            "wallet" => todo!(),
        //            "transaction" => todo!(),
        //            "network" => todo!(),
        //            "util" => todo!(),
        //            _ => todo!(), // 404 not found error.
        //        }
        //
        //        let method = match request.method() {
        //            http1_1::Method::Post => self.post(body),
        //            _ => todo!(), // TODO: 405 method not allowed error.
        //        };
        //
        //        let body = request
        //            .body()
        //            .expect("Server::handle_stream expected body in the http1_1 request.");

        Ok(())
    }

    fn http1_1(&self, req: &http1_1::Request) -> Result<()> {
        // TODO: HTTP 2.0 upgrade with empty settings, not yet supported, will be ignored.
        // TODO: Handle as HTTP 1.1
        // TODO: ONLY clear text "h2c" upgrade allowed. No TLS support for now, Noise Protocol in
        // the future.

        match req.method() {
            // For now, we only expect a GET request to be used for an upgrade.
            http1_1::Method::Get => {
                // TODO: check host if its correct. For now, just 400 if none.
                if !req.headers().contains_key("host") {
                    todo!(); // 400 Bad request
                }

                let connection = req.headers().get("connection");
                if connection != Some("upgrade, http2-settings") || connection.is_none() {
                    todo!() // 400 Bad request
                }

                let upgrade = req.headers().get("upgrade");
                if upgrade != Some("h2c") || upgrade.is_none() {
                    todo!() // 400 Bad request
                }

                let http2_settings = req.headers().get("http2-settings");
                // TODO: Http2 settings, for now ignore.
                if http2_settings.is_none() {
                    todo!() // 400 Bad request
                }

                todo!() // Send "HTTP/1.1 101 Switching Protocols upgrade
            }
            http1_1::Method::Post => {}
            _ => todo!(), // 405 method not allowed error.
        }
        Ok(())
    }

    fn http2(&self, req: &http1_1::Request) -> Result<()> {
        Ok(())
    }

    fn resource_server(&self) -> Result<()> {
        //        match
        Ok(())
    }

    fn v1_json_rpc(&self) -> Result<()> {}

    fn post(&self, body: &str) -> Result<&str> {
        let json_req = json_rpc::Request::try_from(body)?;
        Ok(match json_req.method() {
            "server.agent" => &self.agent,
            "server.version" => &self.version,
            "server.banner" => &self.banner,
            "server.donation_address" => &self.donation_address,
            "block.get_header" => unimplemented!(),
            //            "block."
            _ => unimplemented!(), // TODO: error if none or unknown,
        })
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.shutdown().unwrap();
    }
}
