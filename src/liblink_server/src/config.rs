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
use crate::paths;
use crate::server;
use crate::storage;
use serde;
use std::fs;
use std::net;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(serde::Deserialize)]
pub struct TomlStorageConfig {
    path: Option<PathBuf>,
}

// Check lightning network and maybe have colors and IDs?
#[derive(serde::Deserialize)]
pub struct TomlCoreRpcConfig {
    username: Option<String>,
    password: Option<String>,
    cookie_path: Option<PathBuf>,
    addr: Option<net::SocketAddr>,
    //    ip: Option<String>, // These should update addr
    //    port: Option<u16>,
    block_batch_size: Option<usize>,
}

#[derive(serde::Deserialize)]
pub struct TomlQuicConfig {
    cert_pem_path: Option<String>,
    key_pem_path: Option<String>,
    grease: Option<bool>,
    log_keys: Option<bool>,
    idle_timeout: Option<u64>,
    max_packet_size: Option<u64>,
    initial_max_data: Option<u64>,
    initial_max_stream_data_bidi_local: Option<u64>,
    initial_max_stream_data_bidi_remote: Option<u64>,
    initial_max_stream_data_uni: Option<u64>,
    initial_max_streams_bidi: Opion<u64>,
    initial_max_streams_uni: Option<u64>,
    ack_delay_exponent: Option<u64>,
    max_ack_delay: Option<u64>,
    disable_active_migration: Option<bool>,
    //    cc_algorithm_name: Option<String>,
    //    cc_algorithm: ...,
}

#[derive(serde::Deserialize)]
pub struct TomlServerConfig {
    agent: Option<String>,
    banner: Option<String>,
    donation_addr: Option<String>,
    addr: Option<net::SocketAddr>,
    threads: Option<usize>,
}

#[derive(serde::Deserialize)]
pub struct TomlConfig {
    verbosity: u8,
    dir: Option<PathBuf>,
    threads: Option<u8>,
    server: Option<TomlServerConfig>,
    db: Option<TomlStorageConfig>,
    rpc_server: Option<TomlCoreRpcConfig>,
    quic: Option<TomlQuicConfig>,
}

impl TomlConfig {
    pub fn new<P: AsRef<Path>>(f: P) -> Self {
        toml::from_str(fs::read_to_string(f).unwrap().as_ref()).unwrap()
    }
}

// TODO: Add path?
// TODO: Line by line parser of TOML config that is just better and less wonky. That way I can at
// least do "sub" sections.
pub struct Config {
    pub verbosity: u8,
    pub dir: PathBuf,
    pub server_cfg: server::Config,
    pub db_cfg: storage::Config,
}

impl Default for Config {
    fn default() -> Self {
        let db_cfg = storage::Config {
            path: paths::default_db_file(),
        };
        let server_cfg = server::Config::default();
        Self {
            verbosity: 0,
            dir: paths::default_data_dir(),
            server_cfg,
            db_cfg,
        }
    }
}

impl From<TomlConfig> for Config {
    fn from(toml: TomlConfig) -> Self {
        let mut core_rpc_conf = core_rpc::Config::default();
        if let Some(toml) = toml.rpc_server {
            core_rpc_conf.username = toml.username;
            core_rpc_conf.password = toml.password;
            if let Some(addr) = toml.addr {
                core_rpc_addr = addr;
            }
            core_rpc_conf.cookie_path = toml.cookie_path;
            if let Some(size) = toml.block_batch_size {
                core_rpc_addr = size;
            }
        };
        let mut quic_cfg = quiche::Config::new(quiche::PROTOCOL_VERSION)?;
        if let Some(quic) = toml.quic {
            quic.cert_pem_path
                .map(|path| quic_cfg.load_cert_chain_from_pem_file(&path));
            quic.key_pem_path
                .map(|path| quic_cfg.load_priv_key_from_pem_file(&path));
            quic.grease.map(|v| quic_cfg.grease(v));
            quic.log_keys.map(|v| {
                if v == true {
                    quic_cfg.log_keys()
                }
            });
            quic.idle_timeout
                .map(|time| quic_cfg.set_idle_timeout(time));
            quic.max_packet_size
                .map(|size| quic_cfg.set_max_packet_size(size));
            quic.initial_max_data
                .map(|max| quic_cfg.set_initial_max_data(max));
            quic.initial_max_stream_data_bidi_local
                .map(|max| quic_cfg.set_initial_max_stream_data_bidi_local(max));
            quic.initial_max_stream_data_bidi_remote
                .map(|max| quic_cfg.set_initial_max_stream_data_bidi_remote(max));
            quic.initial_max_stream_data_uni
                .map(|max| quic_cfg.set_initial_max_stream_data_uni(max));
            quic.initial_max_streams_bidi
                .map(|max| quic_cfg.set_initial_max_streams_bidi(max));
            quic.initial_max_streams_uni
                .map(|max| quic_cfg.set_initial_max_streams_uni(max));
            quic.ack_delay_exponent
                .map(|exponent| quic_cfg.set_ack_delay_exponent(exponent));
            quic.max_ack_delay
                .map(|max| quic_cfg.set_max_ack_delay(max));
            quic.disable_active_migration
                .map(|disabled| quic_cfg.set_disable_active_migration(disabled));
        }
        let mut server_conf = server::Config::default();
        if let Some(toml) = toml.server {
            if let Some(agent) = toml.agent {
                server_conf.agent = agent;
            }
            if let Some(banner) = toml.banner {
                server_conf.banner = banner;
            }
            if let Some(donation) = toml.donation_addr {
                server_conf.donation_address = donation;
            }
            if let Some(addr) = toml.addr {
                server_conf.addr = addr;
            }
            server_conf.thread_count = toml.threads;
        };
        let db_cfg = if let Some(db) = toml.db {
            storage::Config {
                path: db.path.unwrap_or(db_def.path),
            }
        } else {
            storage::Config::default()
        };

        server_conf.rpc_conf = core_rpc_conf;
        server_conf.quic_config = quic_cfg;

        Self {
            verbosity: 0,
            dir: Default::default(),
            server_cfg,
            db_cfg,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toml(&self) {}

    // Should move to binary.
    pub fn add_args(&mut self, matches: clap::ArgMatches) {
        // TODO: can do a "If args greater than 1 then parse args"? that is what gets this function.

        let verbosity = matches.occurrences_of("verbosity") as u8;
        if verbosity > 3 {
            // Error
        } else if verbosity >= 1 {
            self.verbosity = verbosity;
        }

        matches
            .value_of("dir")
            .map(|dir| self.dir = PathBuf::from_str(dir).unwrap());
        matches
            .value_of("banner")
            .map(|banner| self.server_cfg.banner = banner.to_string());
        matches
            .value_of("donation")
            .map(|addr| self.server_cfg.donation_address = donation.to_owned());
        matches
            .value_of("addr")
            .map(|addr| self.server_cfg.addr = addr.parse().unwrap());
        matches
            .value_of("ip")
            .map(|ip| self.server_cfg.addr.set_ip(ip.parse().unwrap()));
        matches
            .value_of("port")
            .map(|port| self.server_cfg.addr.set_port(u16::from_str(port).unwrap()));
        self.server_cfg.thread_count = matches.value_of("threads").map(|f| usize::from_str(f)?);
        self.server_cfg.rpc_cfg.username = matches.value_of("rpc_username").map(|f| f.to_owned());
        self.server_cfg.rpc_cfg.password = matches.value_of("rpc_password").map(|f| f.to_owned());
        matches
            .value_of("rpc_ip")
            .map(|ip| self.server_cfg.rpc_cfg.addr.set_ip(ip.parse().unwrap()));
        matches.value_of("rpc_port").map(|port| {
            self.server_cfg
                .rpc_cfg
                .addr
                .set_port(u16::from_str(port).unwrap())
        });
        matches
            .value_of("rpc_addr")
            .map(|addr| self.server_cfg.rpc_cfg.addr = addr.parse().unwrap());
        if let Some(size) = matches.value_of("rpc_batch_size") {
            self.server_cfg.rpc_cfg.block_batch_size = usize::from_str(size).unwrap();
        }
        matches
            .value_of("rpc_batch_size")
            .map(|size| self.server_cfg.rpc_cfg.block_batch_size = usize::from_str(size).unwrap());
    }
}
