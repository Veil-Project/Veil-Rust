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

use crate::paths;
use crate::Result;
use rocksdb;
use std::path;
use veil;

// TODO: Column families are like tables, key-values being the rows.
//////
// Tx Index
// Optional ?
// k: txid / v: confirmed height

//////
// KEYIMAGE DB
// k: keyimages / k: txid:8

//////
// INDEX: (Outpoint + TxOut data)
// Used for RingCT inputs
// k: index # / v: txid:8, vout, whole data of the transaction out

const KEYIMAGE_FAMILY: &str = "K";
const TRANSACTION_FAMILY: &str = "T";
const INDEX_FAMILY: &str = "I";

pub struct Config {
    pub path: path::PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: paths::default_db_file(),
        }
    }
}

pub struct DB(rocksdb::DB);

impl DB {
    pub fn new(config: Config) -> Result<Self> {
        let tx_cf =
            rocksdb::ColumnFamilyDescriptor::new(TRANSACTION_FAMILY, rocksdb::Options::default());
        let keyimage_cf =
            rocksdb::ColumnFamilyDescriptor::new(KEYIMAGE_FAMILY, rocksdb::Options::default());
        let index_cf =
            rocksdb::ColumnFamilyDescriptor::new(INDEX_FAMILY, rocksdb::Options::default());
        let mut db_cfg = rocksdb::Options::default();
        db_cfg.create_if_missing(true);
        //        db_cfg.create_missing_column_families(true); // Might reduce code

        let db = rocksdb::DB::open_cf_descriptors(
            &db_cfg,
            config.path,
            vec![tx_cf, keyimage_cf, index_cf],
        )?;
        Ok(Self(db))
    }

    fn keyimage_cf(&self) -> &rocksdb::ColumnFamily {
        self.0
            .cf_handle(KEYIMAGE_FAMILY)
            .expect("Keyimage column family missing")
    }

    fn transaction_cf(&self) -> &rocksdb::ColumnFamily {
        self.0
            .cf_handle(TRANSACTION_FAMILY)
            .expect("transaction column family missing")
    }

    fn index_cf(&self) -> &rocksdb::ColumnFamily {
        self.0
            .cf_handle(INDEX_FAMILY)
            .expect("index column family missing")
    }

    /// Places a `KeyImage` into the database.
    ///
    /// * `key` - `KeyImage`
    /// * `value` - `Txid`:8 bytes included in.
    ///
    /// Can get rest of the data from looking up that transaction ID.
    pub fn put_key_image(&mut self, key: veil::KeyImage, value: u64) -> Result<()> {
        Ok(self
            .0
            .put_cf(self.keyimage_cf(), key, value.to_be_bytes())?) // TODO: option
    }

    /// Places a transaction out into the database.
    ///
    /// * `key` - `Index #` as u32
    /// * `value` - `Txid`:8 bytes its from, vout as `u32`, whole tx out data.
    pub fn put_tx_out(&mut self, key: [u8; 32], value: &[u8]) -> Result<()> {
        Ok(self.0.put_cf(self.transaction_cf(), key, value)?)
    }

    /// Stores the txid and its confirmed block height. (Optional)
    ///
    /// * `key` - `TxId`
    /// * `value` - Confirmed height as `u32`.
    pub fn put_index(&mut self, key: [u8; 32], value: u64) -> Result<()> {
        Ok(self.0.put_cf(self.index_cf(), key, value.to_be_bytes())?)
    }
}
