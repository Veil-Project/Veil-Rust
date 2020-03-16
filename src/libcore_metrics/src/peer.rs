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

use crate::util::{VecMathSigned, VecMathUnsigned};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use veil_core_rpc_client::client::RPCClient;
use veil_core_rpc_client::veild_structs::{BannedPeer, PeerInfo};

#[derive(Debug)]
struct PeerBan {
    banned_until: u64,
    ban_created: u64,
    ban_reason: String,
}

impl PeerBan {
    pub fn new(ban: &BannedPeer) -> Self {
        Self {
            banned_until: ban.until(),
            ban_created: ban.created(),
            ban_reason: ban.reason().to_string(),
        }
    }
}

#[derive(Debug)]
struct PeerMiniStat {
    version: String,
    protocol: u64,
    banscore: u64,
    duration: Duration,
    last_updated: SystemTime,
}

impl PeerMiniStat {
    pub fn new(info: &PeerInfo) -> Self {
        Self {
            version: info.sub_version().to_string(),
            protocol: info.version(),
            banscore: info.ban_score(),
            duration: Duration::new(0, 0),
            last_updated: SystemTime::now(),
        }
    }

    pub fn update(&mut self, info: &PeerInfo) {
        self.version = info.sub_version().to_string();
        self.protocol = info.version();
        self.banscore = info.ban_score();
        self.duration = self
            .last_updated
            .duration_since(self.last_updated)
            .expect("Problem with system clock; possibly changed.");
        self.last_updated = SystemTime::now();
    }
}

#[derive(Debug, Default)]
pub struct PeerStats {
    peers: u64,
    banned: u64,
    peer_list: HashMap<String, PeerMiniStat>, // Should be on its own
    ban_list: HashMap<String, PeerBan>,       // Should be on its own
    versions: HashMap<String, u64>,
    subversions: HashMap<String, u64>,
    block_height: HashMap<String, u64>,
    inbound: u64,
    outbound: u64,
    max_time_offset: i64,
    max_time_conn: u64,
    max_synced_blocks: u64,
    max_synced_headers: u64,
    min_time_offset: i64,
    min_time_conn: u64,
    min_synced_blocks: u64,
    min_synced_headers: u64,
    avg_time_offset: f64,
    avg_time_conn: f64,
    avg_synced_blocks: f64,
    avg_synced_headers: f64,
    median_time_offset: f64,
    median_time_conn: f64,
    median_synced_blocks: f64,
    median_synced_headers: f64,
    total_bytes_recv: u64,
    total_bytes_sent: u64,
    time_millis: u64,
    warnings: Vec<String>,
}

impl PeerStats {
    pub fn parse(rpc: &mut RPCClient) -> Self {
        let mut stats: Self = Self::default();

        let peer_info = rpc.get_peer_info().unwrap();
        let banned_list = rpc.list_banned().unwrap();
        let net_totals = rpc.get_net_totals().unwrap();

        stats.peers = peer_info.len() as u64;
        stats.banned = banned_list.len() as u64;
        stats.total_bytes_recv = net_totals.total_bytes_recv();
        stats.total_bytes_sent = net_totals.total_bytes_sent();
        stats.time_millis = net_totals.time_millis();

        let mut time_offset: Vec<i64> = vec![];
        let mut time_conn: Vec<u64> = vec![];
        let mut synced_blocks: Vec<u64> = vec![];
        let mut synced_headers: Vec<u64> = vec![];

        stats.ban_list.clear();
        for ban in &banned_list {
            stats
                .ban_list
                .insert(ban.address().to_string(), PeerBan::new(ban));
        }

        for peer in &peer_info {
            if peer.is_inbound() {
                stats.inbound += 1;
            } else {
                stats.outbound += 1;
            }

            if stats.peer_list.contains_key(peer.addr()) {
                let existing_peer = stats.peer_list.get_mut(peer.addr()).unwrap();
                existing_peer.update(peer); // Need to test this. No idea if this actually updates it or not. Might have to toss it back.
            } else {
                stats
                    .peer_list
                    .insert(peer.addr().to_string(), PeerMiniStat::new(peer));
            }

            let peer_version = format!("{}", peer.version());
            let counter_version = stats.versions.entry(peer_version).or_insert(0);
            *counter_version += 1;
            let counter_subversion = stats
                .subversions
                .entry(peer.sub_version().to_string())
                .or_insert(0);
            *counter_subversion += 1;
            let counter_block_height = stats
                .block_height
                .entry(peer.synced_blocks().unwrap_or(0).to_string())
                .or_insert(0);
            *counter_block_height += 1;

            time_offset.push(peer.time_offset());
            time_conn.push(peer.connection_time());

            match peer.synced_blocks() {
                Some(x) => synced_blocks.push(x),
                None => {}
            }

            match peer.synced_headers() {
                Some(x) => synced_headers.push(x),
                None => {}
            }
        }

        stats.max_time_offset = time_offset.largest();
        stats.max_time_conn = time_conn.largest();
        stats.max_synced_blocks = synced_blocks.largest();
        stats.max_synced_headers = synced_headers.largest();
        stats.min_time_offset = time_offset.smallest();
        stats.min_time_conn = time_conn.smallest();
        stats.min_synced_blocks = synced_blocks.smallest();
        stats.min_synced_headers = synced_headers.smallest();
        stats.avg_time_offset = time_offset.average();
        stats.avg_time_conn = time_conn.average();
        stats.avg_synced_blocks = synced_blocks.average();
        stats.avg_synced_headers = synced_headers.average();
        stats.median_time_offset = time_offset.median();
        stats.median_time_conn = time_conn.median();
        stats.median_synced_blocks = synced_blocks.median();
        stats.median_synced_headers = synced_headers.median();

        stats.warnings();

        stats
    }

    fn warnings(&mut self) {
        let th = self.peers as f64 * 0.1f64;
        let th_u = th as u64;

        if self.peers == 0u64 {
            self.warnings
                .push("There are no peers connected. Check your connection!".to_string());
        }

        if self.banned > 5u64 {
            self.warnings
                .push("Banned count unusually high.".to_string());
        }

        if self.block_height.get("0").unwrap() > &th_u {
            self.warnings
                .push("More than threshold of peers have an unknown block height.".to_string());
        }
    }
}
