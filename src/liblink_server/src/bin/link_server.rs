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

// The whole purpose of this is to create a new faster database using data pulled through RPC.
// It will be using Electrum's protocol with changes to accommodate for RingCT as well as being
// delivered using HTTP 2.0, JSON RPC 2.0, and Noise Protocol.
// TODO: Noise protocol
// TODO: Json RPC 2.0
// TODO: HTTP 2.0
// TODO: Database storage
// TODO: Daemon
// TODO: CLI
// TODO: RPC

// Data will come either from it's own database, or through the RPC. Depending on the
// configuration, features can be turned on or off.
use clap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;
use toml;
use veil_link_server::{config, error, paths, storage, Result};

fn config() -> Result<config::Config> {
    let matches = clap::App::new("Veil Link Server")
        .version(clap::crate_version!())
        .author("Mimir <mimirmim@pm.me>")
        .about("Low resource, fast and safe server for interfacing with Veil Core.")
        .arg(
            clap::Arg::with_name("network")
                .long("network")
                .help("Sets the network to use ('mainnet', 'testnet', 'regtest'"),
        )
        .arg(
            clap::Arg::with_name("config")
                .long("config")
                .short("c")
                .value_name("CONF")
                .help("Sets the configuration file path.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("verbosity")
                .long("verbose")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity."),
        )
        .arg(
            clap::Arg::with_name("dir")
                .long("dir")
                .short("d")
                .help("Sets the main file directory.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("db_path")
                .long("db-path")
                .help("Sets the database path.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("addr")
                .long("address")
                .short("a")
                .help("Sets the address of the server.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("ip")
                .long("i")
                .help("Sets the server IP address.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("port")
                .long("port")
                .short("p")
                .help("Sets the server port address.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("banner")
                .long("banner")
                .help("Sets the server banner welcome message.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("donation")
                .long("donation-addr")
                .help("Sets the server donation address people can view.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("rpc_cookie")
                .long("rpc-cookie")
                .help("Sets the core RPC cookie file path.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("rpc_addr")
                .long("rpc-addr")
                .help("Sets the core RPC address.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("rpc_ip")
                .long("rpc-ip")
                .help("Sets the core RPC IP.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("rpc_port")
                .long("rpc-port")
                .help("Sets the core RPC port.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("rpc_username")
                .long("rpc-username")
                .help("Sets the core RPC username.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("rpc_password")
                .long("rpc-password")
                .help("Sets the core RPC password")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("rpc_batch_size")
                .long("rpc-batch-size")
                .help(
                    "Sets the batch size in blocks, or other structures, to be called at a \
                     time from the Core RPC.",
                )
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("reindex")
                .long("reindex")
                .help("Reindex the core database through RPC using the block height provided.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("threads")
                .long("threads")
                .help(
                    "Set the number of threads used for indexing. Default: 0 - Uses all available.",
                )
                .takes_value(true),
        )
        .get_matches();
    let cfg_path = matches
        .value_of("config")
        .map_or(paths::default_config_file(), |f| {
            PathBuf::from_str(f).unwrap()
        });
    let mut cfg: config::Config = if cfg_path.exists() {
        let toml_cfg: config::TomlConfig =
            toml::from_str(&fs::read_to_string(cfg_path).unwrap()).unwrap();
        config::Config::from(toml_cfg)
    } else {
        config::Config::default()
    };

    // I believe its 2?
    if env::args().len() > 2 {
        cfg.add_args(matches);
    }
    Ok(cfg)
}

fn run() -> Result<()> {
    let cfg = config()?;
    // Should DB run in it's own thread with a tx / rx system?
    let db = storage::DB::new(cfg.db_cfg)?;

    Ok(())
}

fn main() {
    run().unwrap_or_else(|e| {
        let cause = e.cause();
        let exit_code: i32 = e.exit_code();
        let source = e.source();
        eprintln!("Error occurred while {}: {:?}", cause, source);
        process::exit(exit_code);
    });
}
