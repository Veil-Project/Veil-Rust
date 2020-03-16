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

use crate::response::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// TODO: change all &Option outs, no as_ref
// TODO: Serialize and deserialize method?
use crate::{Error, Result};

pub enum ServiceFlags {
    Network = 1 << 0,
    GetUtxo = 1 << 1,
    Bloom = 1 << 2,
    Witness = 1 << 3,
    NetworkLimited = 1 << 10,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SigHashType {
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "ALL|ANYONECANPAY")]
    AllAnyoneCanPay,
    #[serde(rename = "NONE|ANYONECANPAY")]
    NoneAnyoneCanPay,
    #[serde(rename = "SINGLE|ANYONECANPAY")]
    SingleAnyoneCanPay,
}

pub type FlagHex = String;

pub trait FlagMethods {
    fn to_names(&self) -> Vec<String>;
}

impl FlagMethods for FlagHex {
    fn to_names(&self) -> Vec<String> {
        let flags: u64 = u64::from_str_radix(&self, 16).unwrap();
        let mut flag_vec = vec![];

        if flags & ServiceFlags::Network as u64 == ServiceFlags::Network as u64 {
            flag_vec.push("NETWORK".to_owned());
        }

        if flags & ServiceFlags::GetUtxo as u64 == ServiceFlags::GetUtxo as u64 {
            flag_vec.push("GETUTXO".to_owned());
        }

        if flags & ServiceFlags::Bloom as u64 == ServiceFlags::Bloom as u64 {
            flag_vec.push("BLOOM".to_owned());
        }

        if flags & ServiceFlags::Witness as u64 == ServiceFlags::Witness as u64 {
            flag_vec.push("WITNESS".to_owned());
        }

        if flags & ServiceFlags::NetworkLimited as u64 == ServiceFlags::NetworkLimited as u64 {
            flag_vec.push("NETWORK_LIMITED".to_owned());
        }

        flag_vec
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TxKind {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "anon")]
    CT,
    #[serde(rename = "ringct")]
    RingCT,
    #[serde(rename = "zerocoin")]
    Zcoin,
    None,
}

// TODO: Enum for kind
#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ScriptPubKey {
    asm: String,
    hex: String,
    #[serde(rename = "reqsigs")]
    req_sigs: Option<u64>,
    #[serde(rename = "type")]
    kind: String,
    addresses: Option<Vec<String>>,
    p2sh: Option<String>,
}

impl ScriptPubKey {
    pub fn asm(&self) -> &str {
        &self.asm
    }

    pub fn as_hex_string(&self) -> &str {
        &self.hex
    }

    pub fn req_sigs(&self) -> Option<&u64> {
        self.req_sigs.as_ref()
    }

    pub fn kind(&self) -> &TxKind {
        match self.kind.as_ref() {
            "standard" => &TxKind::Standard,
            "ringct" => &TxKind::RingCT,
            _ => &TxKind::None,
        }
    }

    pub fn addresses(&self) -> Option<&Vec<String>> {
        self.addresses.as_ref()
    }
}

impl TryFrom<Value> for ScriptPubKey {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for ScriptPubKey {
    type Error = Error;
    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ScriptSignature {
    asm: String,
    hex: String,
    #[serde(rename = "type")]
    kind: Option<TxKind>,
}

impl ScriptSignature {
    pub fn asm(&self) -> &str {
        &self.asm
    }

    pub fn as_hex_string(&self) -> &str {
        &self.hex
    }

    pub fn kind(&self) -> Option<&TxKind> {
        self.kind.as_ref()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RingCTInput {
    #[serde(rename = "txid")]
    tx_id: String,
    #[serde(rename = "vout.n")]
    v_out: u64,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TransactionIn {
    coinbase: Option<String>,
    #[serde(rename = "txid")]
    tx_id: Option<String>,
    #[serde(rename = "type")]
    kind: String,
    num_inputs: Option<u64>,
    ring_size: Option<u64>,
    ringct_inputs: Option<Vec<RingCTInput>>,
    key_images: Option<Vec<String>>,
    denomination: Option<String>,
    serial: Option<String>,
    #[serde(rename = "pubcoin")]
    pub_coin: Option<String>,
    #[serde(rename = "vout")]
    v_out: Option<u64>,
    #[serde(rename = "scriptSig")]
    script_sig: Option<ScriptSignature>,
    sequence: Option<u64>,
    #[serde(rename = "txinwitness")]
    tx_in_witnesses: Option<Vec<String>>,
}

impl TransactionIn {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn is_coinbase(&self) -> bool {
        self.coinbase.is_some()
    }

    // All these Options should be only returning if correct type, or error if not.
    // I don't think any of these should be returning an option.
    pub fn coinbase(&self) -> Option<&String> {
        self.coinbase.as_ref()
    }

    pub fn tx_id(&self) -> Option<&String> {
        self.tx_id.as_ref()
    }

    pub fn kind(&self) -> &TxKind {
        match self.kind.as_ref() {
            "standard" => &TxKind::Standard,
            "ringct" => &TxKind::RingCT,
            _ => &TxKind::None,
        }
    }

    pub fn is_standard(&self) -> bool {
        match self.kind.as_ref() {
            "standard" => true,
            "ringct" => false,
            _ => false,
        }
    }

    pub fn is_ringct(&self) -> bool {
        match self.kind.as_ref() {
            "standard" => false,
            "ringct" => true,
            _ => false,
        }
    }

    pub fn denomination(&self) -> Option<&String> {
        self.denomination.as_ref()
    }

    pub fn serial(&self) -> Option<&String> {
        self.serial.as_ref()
    }

    pub fn pub_coin(&self) -> Option<&String> {
        self.pub_coin.as_ref()
    }

    pub fn v_out(&self) -> &Option<u64> {
        &self.v_out
    }

    pub fn script_sig(&self) -> &Option<ScriptSignature> {
        &self.script_sig
    }

    pub fn sequence(&self) -> &Option<u64> {
        &self.sequence
    }

    // NOTE todo err
    // pub fn tx_in_witnesses(&self) -> Result<Vec<String>> {
    //     match self.tx_in_witnesses {
    //         Some(w) => Ok(w),
    //         None => eprint!("err?"), // Needs error.
    //     }
    // }
}

impl TryFrom<Value> for TransactionIn {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for TransactionIn {
    type Error = Error;
    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct UnspentTransactionOut {
    #[serde(rename = "bestblock")]
    best_block: String,
    confirmations: u64,
    value: f64,
    #[serde(rename = "scriptPubKey")]
    script_pub_key: Option<ScriptPubKey>,
    coinbase: bool,
}

impl UnspentTransactionOut {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn best_block(&self) -> &str {
        &self.best_block
    }

    pub fn confirmations(&self) -> &u64 {
        &self.confirmations
    }

    pub fn value(&self) -> &f64 {
        &self.value
    }

    pub fn script_pub_key(&self) -> Option<&ScriptPubKey> {
        self.script_pub_key.as_ref()
    }

    pub fn is_coinbase(&self) -> bool {
        self.coinbase
    }
}

impl TryFrom<Value> for UnspentTransactionOut {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for UnspentTransactionOut {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TransactionOut {
    #[serde(rename = "type")]
    kind: String,
    value: Option<f64>,
    #[serde(rename = "valueSat")]
    value_sat: Option<u64>,
    #[serde(rename = "scriptPubKey")]
    script_pub_key: Option<ScriptPubKey>,
    #[serde(rename = "ephemeral_pubkey")]
    ephemeral_pub_key: Option<String>,
    #[serde(rename = "pubkey")]
    pub_key: Option<String>,
    #[serde(rename = "valueCommitment")]
    value_commitment: Option<String>,
    data_hex: Option<String>,
    ct_fee: Option<f64>,
    rangeproof: Option<String>,
    rp_exponent: Option<u64>,
    rp_mantissa: Option<u64>,
    rp_min_value: Option<f64>,
    rp_max_value: Option<f64>,
}

impl TransactionOut {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn kind(&self) -> &TxKind {
        match self.kind.as_ref() {
            "standard" => &TxKind::Standard,
            "ringct" => &TxKind::RingCT,
            _ => &TxKind::None,
        }
    }

    pub fn value(&self) -> &Option<f64> {
        &self.value
    }

    pub fn value_sat(&self) -> &Option<u64> {
        &self.value_sat
    }

    pub fn script_pub_key(&self) -> &Option<ScriptPubKey> {
        &self.script_pub_key
    }
}

impl TryFrom<Value> for TransactionOut {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for TransactionOut {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Transaction {
    // #[serde(rename = "inactivechain")]
    // in_active_chain: bool, // Doesn't exist in Veil?
    #[serde(rename = "txid")]
    tx_id: String,
    hash: String,
    version: u64,
    size: u64,
    #[serde(rename = "vsize")]
    v_size: u64,
    weight: u64,
    #[serde(rename = "locktime")]
    lock_time: u64,
    #[serde(rename = "vin")]
    v_in: Vec<TransactionIn>,
    #[serde(rename = "vout")]
    v_out: Vec<TransactionOut>,
    #[serde(rename = "blockhash")]
    block_hash: Option<String>, // Doesn't exist
    confirmations: Option<u64>, // Doesn't exist in requestblock, exists in rawtransaction
    #[serde(rename = "blocktime")]
    block_time: Option<u64>, // Doesn't exist
    time: Option<u64>,          // Doesn't exist in requestblock, exists in rawtransaction
    hex: String,
}

impl Transaction {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn tx_id(&self) -> &str {
        &self.tx_id
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn version(&self) -> &u64 {
        &self.version
    }

    pub fn size(&self) -> &u64 {
        &self.size
    }

    pub fn v_size(&self) -> &u64 {
        &self.v_size
    }

    pub fn weight(&self) -> &u64 {
        &self.weight
    }

    pub fn lock_time(&self) -> &u64 {
        &self.lock_time
    }

    pub fn v_in(&self) -> &Vec<TransactionIn> {
        &self.v_in
    }

    pub fn v_out(&self) -> &Vec<TransactionOut> {
        &self.v_out
    }

    pub fn as_hex_string(&self) -> &str {
        &self.hex
    }
}

impl TryFrom<Value> for Transaction {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for Transaction {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FundedTransaction {
    hex: String,
    fee: f64,
    #[serde(rename = "changepos")]
    change_pos: i64,
}

impl FundedTransaction {
    pub fn hex(&self) -> &str {
        &self.hex
    }

    pub fn fee(&self) -> &f64 {
        &self.fee
    }

    pub fn change_pos(&self) -> &i64 {
        &self.change_pos
    }
}

impl TryFrom<Value> for FundedTransaction {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for FundedTransaction {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct WitnessUnspentTransactionOutput {
    amount: f64,
    #[serde(rename = "scriptPubKey")]
    script_pub_key: ScriptPubKey,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PartiallySignedTransactionInput {
    // non_witness_utxo: Option<> // FIXME: I have no idea what this looks like from the given documentation.
    witness_utxo: WitnessUnspentTransactionOutput,
    partial_signatures: Option<HashMap<String, String>>,
    sig_hash: Option<String>,
    redeem_script: Option<ScriptPubKey>,
    witness_script: Option<ScriptPubKey>,
    // bip32_derivs:  // FIXME: I have no idea what this looks like from the given documentation.
    #[serde(rename = "final_scriptsig")]
    final_script_sig: Option<ScriptPubKey>,
    #[serde(rename = "final_scriptwitness")]
    final_script_witness: Option<Vec<String>>,
    unknown: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PartiallySignedTransactionOutput {
    redeem_script: Option<ScriptPubKey>,
    witness_script: Option<ScriptPubKey>,
    // bip32_derivs:  // FIXME: I have no idea what this looks like from the given documentation.
    unknown: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PartiallySignedTransaction {
    #[serde(rename = "tx")]
    txs: Vec<Transaction>,
    unknown: HashMap<String, String>,
    inputs: Vec<PartiallySignedTransactionInput>,
    outputs: Vec<PartiallySignedTransactionOutput>,
    fee: f64,
}

impl TryFrom<Value> for PartiallySignedTransaction {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for PartiallySignedTransaction {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FinalPartiallySignedTransaction {
    pst: Option<String>,
    hex: Option<String>,
    complete: bool,
}

impl TryFrom<Value> for FinalPartiallySignedTransaction {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for FinalPartiallySignedTransaction {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PreviousTransaction {
    #[serde(rename = "txid")]
    tx_id: String,
    #[serde(rename = "vout")]
    v_out: u64,
    #[serde(rename = "scriptPubKey")]
    script_pub_key: String,
    #[serde(rename = "redeemScript")]
    redeem_script: Option<String>,
    amount: f64,
    amount_commitment: Option<f64>,
}

impl PreviousTransaction {
    pub fn tx_id(&self) -> &str {
        &self.tx_id
    }

    pub fn v_out(&self) -> &u64 {
        &self.v_out
    }

    pub fn script_pub_key(&self) -> &str {
        &self.script_pub_key
    }

    pub fn redeem_script(&self) -> Option<&str> {
        self.redeem_script.as_ref().map(|x| x.as_ref())
    }

    pub fn amount(&self) -> &f64 {
        &self.amount
    }
}

impl TryFrom<Value> for PreviousTransaction {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for PreviousTransaction {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SignedTransactionError {
    #[serde(rename = "txid")]
    tx_id: String,
    #[serde(rename = "vout")]
    v_out: u64,
    #[serde(rename = "scriptSig")]
    script_sig: String,
    sequence: u64,
    error: String,
}

impl SignedTransactionError {
    pub fn tx_id(&self) -> &str {
        &self.tx_id
    }

    pub fn v_out(&self) -> &u64 {
        &self.v_out
    }

    pub fn script_sig(&self) -> &str {
        &self.script_sig
    }

    pub fn sequence(&self) -> &u64 {
        &self.sequence
    }

    pub fn error(&self) -> &str {
        &self.error
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FinalSignedTransaction {
    hex: Option<String>,
    complete: bool,
    errors: Vec<SignedTransactionError>,
}

impl FinalSignedTransaction {
    pub fn hex(&self) -> Option<&str> {
        self.hex.as_ref().map(|x| x.as_ref())
    }

    pub fn complete(&self) -> &bool {
        &self.complete
    }

    pub fn errors(&self) -> &[SignedTransactionError] {
        &self.errors
    }
}

impl TryFrom<Value> for FinalSignedTransaction {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for FinalSignedTransaction {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CommitmentResult {
    result: bool,
}

impl TryFrom<Value> for CommitmentResult {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;
        Ok(s)
    }
}

impl TryFrom<Response> for CommitmentResult {
    type Error = Error;
    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BlockStats {
    #[serde(rename = "avgfee")]
    avg_fee: u64,
    #[serde(rename = "avgfeerate")]
    avg_fee_rate: u64,
    #[serde(rename = "avgtxsize")]
    avg_tx_size: u64,
    #[serde(rename = "blockhash")]
    block_hash: String,
    #[serde(rename = "feerate_percentiles")]
    fee_rate_percentiles: Vec<u64>,
    height: u64,
    ins: u64,
    #[serde(rename = "maxfee")]
    max_fee: u64,
    #[serde(rename = "maxfeerate")]
    max_fee_rate: u64,
    #[serde(rename = "maxtxsize")]
    max_tx_size: u64,
    #[serde(rename = "medianfee")]
    median_fee: u64,
    #[serde(rename = "mediantime")]
    median_time: u64,
    #[serde(rename = "mediantxsize")]
    median_tx_size: u64,
    #[serde(rename = "minfee")]
    min_fee: u64,
    #[serde(rename = "minfeerate")]
    min_fee_rate: u64,
    #[serde(rename = "mintxsize")]
    min_tx_size: u64,
    outs: u64,
    subsidy: u64,
    #[serde(rename = "segwit_total_size")]
    segwit_total_size: u64,
    #[serde(rename = "segwit_total_weight")]
    segwit_total_weight: u64,
    #[serde(rename = "swtxs")]
    segwit_txs: u64,
    time: u64,
    total_out: u64,
    total_size: u64,
    total_weight: u64,
    #[serde(rename = "totalfee")]
    total_fee: u64,
    txs: u64,
    utxo_increase: i64,
    utxo_size_inc: u64,
}

impl BlockStats {
    pub fn new_from_response(res: Response) -> Result<Self> {
        Self::new_from_value(res.result().unwrap().to_owned())
    }

    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn avg_fee(&self) -> &u64 {
        &self.avg_fee
    }

    pub fn avg_fee_rate(&self) -> &u64 {
        &self.avg_fee_rate
    }

    pub fn avg_tx_size(&self) -> &u64 {
        &self.avg_tx_size
    }

    pub fn block_hash(&self) -> &str {
        &self.block_hash
    }

    pub fn fee_rate_percentiles(&self) -> &[u64] {
        &self.fee_rate_percentiles
    }

    pub fn height(&self) -> &u64 {
        &self.height
    }

    pub fn ins(&self) -> &u64 {
        &self.ins
    }

    pub fn max_fee(&self) -> &u64 {
        &self.max_fee
    }

    pub fn max_fee_rate(&self) -> &u64 {
        &self.max_fee_rate
    }

    pub fn max_tx_size(&self) -> &u64 {
        &self.max_tx_size
    }

    pub fn median_fee(&self) -> &u64 {
        &self.median_fee
    }

    pub fn median_time(&self) -> &u64 {
        &self.median_time
    }

    pub fn median_tx_size(&self) -> &u64 {
        &self.median_tx_size
    }

    pub fn min_fee(&self) -> &u64 {
        &self.min_fee
    }

    pub fn min_fee_rate(&self) -> &u64 {
        &self.min_fee_rate
    }

    pub fn min_tx_size(&self) -> &u64 {
        &self.min_tx_size
    }

    pub fn outs(&self) -> &u64 {
        &self.outs
    }

    pub fn subsidy(&self) -> &u64 {
        &self.subsidy
    }

    pub fn segwit_total_size(&self) -> &u64 {
        &self.segwit_total_size
    }

    pub fn segwit_total_weight(&self) -> &u64 {
        &self.segwit_total_weight
    }

    pub fn segwit_txs(&self) -> &u64 {
        &self.segwit_txs
    }

    pub fn time(&self) -> &u64 {
        &self.time
    }

    pub fn total_out(&self) -> &u64 {
        &self.total_out
    }

    pub fn total_size(&self) -> &u64 {
        &self.total_size
    }

    pub fn total_weight(&self) -> &u64 {
        &self.total_weight
    }

    pub fn total_fee(&self) -> &u64 {
        &self.total_fee
    }

    pub fn txs(&self) -> &u64 {
        &self.txs
    }

    pub fn utxo_increase(&self) -> &i64 {
        &self.utxo_increase
    }

    pub fn utxo_size_inc(&self) -> &u64 {
        &self.utxo_size_inc
    }
}

impl TryFrom<Value> for BlockStats {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for BlockStats {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BlockHeader {
    hash: String,
    confirmations: Option<u64>,
    height: u64,
    // weight: u64, // Missing?
    // money_supply: u64, // Missing?
    version: u64,
    #[serde(rename = "versionHex")]
    version_hex: String,
    #[serde(rename = "merkleroot")]
    merkle_root: String,
    #[serde(rename = "time")]
    timestamp: u64,
    #[serde(rename = "mediantime")]
    median_time: u64,
    nonce: u64,
    bits: String,
    difficulty: f64,
    #[serde(rename = "chainwork")]
    chain_work: String,
    #[serde(rename = "nTx")]
    tx_len: u64,
    #[serde(rename = "previousblockhash")]
    previous_block_hash: String,
    #[serde(rename = "nextblockhash")]
    next_block_hash: String,
}

impl BlockHeader {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn new_from_block(b: &Block) -> Self {
        Self {
            hash: b.hash.to_owned(),
            confirmations: b.confirmations,
            height: b.height,
            // weight: b.weight,
            version: b.version,
            version_hex: b.version_hex.to_owned(),
            merkle_root: b.merkle_root.to_owned(),
            timestamp: b.timestamp,
            median_time: b.median_time,
            nonce: b.nonce,
            bits: b.bits.to_owned(),
            difficulty: b.difficulty,
            chain_work: b.chain_work.to_owned(),
            tx_len: b.txs.len() as u64,
            previous_block_hash: b.previous_block_hash.to_owned(),
            next_block_hash: b.next_block_hash.to_owned(),
        }
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn confirmations(&self) -> &Option<u64> {
        &self.confirmations
    }

    pub fn height(&self) -> &u64 {
        &self.height
    }

    // pub fn money_supply(&self) -> &u64 {
    //     &self.money_supply
    // }

    pub fn version(&self) -> &u64 {
        &self.version
    }

    pub fn version_hex(&self) -> &str {
        &self.version_hex
    }

    pub fn merkle_root(&self) -> &str {
        &self.merkle_root
    }

    pub fn timestamp(&self) -> &u64 {
        &self.timestamp
    }

    pub fn median_time(&self) -> &u64 {
        &self.median_time
    }

    pub fn nonce(&self) -> &u64 {
        &self.nonce
    }

    pub fn bits(&self) -> &str {
        &self.bits
    }

    pub fn difficutly(&self) -> &f64 {
        &self.difficulty
    }

    pub fn chain_work(&self) -> &str {
        &self.chain_work
    }

    pub fn previous_block_hash(&self) -> &str {
        &self.previous_block_hash
    }

    pub fn next_block_hash(&self) -> &str {
        &self.next_block_hash
    }
}

impl TryFrom<Value> for BlockHeader {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for BlockHeader {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

// I believe Block is missing the "money supply portion"
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Block {
    hash: String,
    confirmations: Option<u64>,
    size: u64,
    #[serde(rename = "strippedsize")]
    stripped_size: u64,
    weight: u64,
    height: u64,
    proof_type: String,
    #[serde(rename = "proofofstakehash")]
    stake_hash: String,
    version: u64,
    #[serde(rename = "versionHex")]
    version_hex: String,
    #[serde(rename = "merkleroot")]
    merkle_root: String,
    #[serde(rename = "tx")]
    txs: Vec<Transaction>,
    // #[serde(skip_deserializing)]
    #[serde(rename = "time")]
    timestamp: u64,
    #[serde(rename = "mediantime")]
    median_time: u64,
    nonce: u64,
    bits: String,
    difficulty: f64,
    #[serde(rename = "chainwork")]
    chain_work: String,
    // #[serde(rename = "nTx")]
    // tx_count: u64, // Removed, can always to `.len()` on txs
    anon_index: u64,
    #[serde(rename = "previousblockhash")]
    previous_block_hash: String,
    #[serde(rename = "nextblockhash")]
    next_block_hash: String,
}

impl Block {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn header(&self) -> BlockHeader {
        BlockHeader::new_from_block(&self)
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn confirmations(&self) -> &Option<u64> {
        &self.confirmations
    }

    pub fn size(&self) -> &u64 {
        &self.size
    }

    pub fn stripped_size(&self) -> &u64 {
        &self.stripped_size
    }

    pub fn weight(&self) -> &u64 {
        &self.weight
    }

    pub fn height(&self) -> &u64 {
        &self.height
    }

    pub fn proof_type(&self) -> &str {
        &self.proof_type
    }

    pub fn stake_hash(&self) -> &str {
        &self.stake_hash
    }

    pub fn version(&self) -> &u64 {
        &self.version
    }

    pub fn version_hex(&self) -> &str {
        &self.version_hex
    }

    pub fn merkle_root(&self) -> &str {
        &self.merkle_root
    }

    pub fn txs(&self) -> &Vec<Transaction> {
        &self.txs
    }

    pub fn timestamp(&self) -> &u64 {
        &self.timestamp
    }

    pub fn median_time(&self) -> &u64 {
        &self.median_time
    }

    pub fn duration_since(&self) -> Result<Duration> {
        let timestamp = Duration::from_secs(self.timestamp);
        let now = SystemTime::now();
        let now_unix = now.duration_since(UNIX_EPOCH).unwrap();

        Ok(now_unix.checked_sub(timestamp).unwrap())
    }

    pub fn nonce(&self) -> &u64 {
        &self.nonce
    }

    pub fn bits(&self) -> &str {
        &self.bits
    }

    pub fn difficutly(&self) -> &f64 {
        &self.difficulty
    }

    pub fn chain_work(&self) -> &str {
        &self.chain_work
    }

    pub fn anon_index(&self) -> &u64 {
        &self.anon_index
    }

    pub fn previous_block_hash(&self) -> &str {
        &self.previous_block_hash
    }

    pub fn next_block_hash(&self) -> &str {
        &self.next_block_hash
    }
}

impl TryFrom<Value> for Block {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for Block {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ZerocashLimp {
    status: String,
    #[serde(rename = "startTime")]
    start_time: u64,
    timeout: u64,
    since: u64,
}

impl ZerocashLimp {
    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn start_time(&self) -> &u64 {
        &self.start_time
    }

    pub fn timeout(&self) -> &u64 {
        &self.timeout
    }

    pub fn since(&self) -> &u64 {
        &self.since
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Bip9 {
    zc_limp: ZerocashLimp,
}

impl Bip9 {
    pub fn kind(&self) -> &ZerocashLimp {
        &self.zc_limp
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DenomInfo {
    #[serde(rename = "denom")]
    kind: String,
    amount: u64,
    amount_formatted: Option<String>,
    percent: f64,
}

impl DenomInfo {
    pub fn kind(&self) -> &str {
        &self.kind // Maybe enum
    }

    pub fn amount(&self) -> &u64 {
        &self.amount
    }

    pub fn amount_formatted(&self) -> Option<&str> {
        self.amount_formatted.as_ref().map(|x| x.as_ref())
    }

    pub fn percent_of_all(&self) -> &f64 {
        &self.percent
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BlockchainInfo {
    chain: String,
    blocks: u64,
    #[serde(rename = "moneysupply")]
    money_supply: u64,
    #[serde(rename = "moneysupply_formatted")]
    money_supply_formatted: String,
    #[serde(rename = "zerocoinsupply")]
    zerocoin_supply: Vec<DenomInfo>,
    headers: u64,
    #[serde(rename = "bestblockhash")]
    best_block_hash: String,
    #[serde(rename = "difficulty_pow")]
    pow_difficulty: f64,
    #[serde(rename = "difficulty_pos")]
    pos_difficulty: f64,
    #[serde(rename = "mediantime")]
    median_time: u64,
    #[serde(rename = "verificationprogress")]
    verification_progress: f64,
    #[serde(rename = "initialblockdownload")]
    initial_block_download: bool,
    #[serde(rename = "chainwork")]
    chain_work: String,
    #[serde(rename = "chainpow")]
    chain_pow: String,
    size_on_disk: u64,
    pruned: bool,
    bip9_softforks: Bip9,
    warnings: String,
}

impl BlockchainInfo {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn chain(&self) -> &str {
        // Could be improved with an enum
        &self.chain
    }

    pub fn blocks(&self) -> &u64 {
        &self.blocks
    }

    pub fn money_supply(&self) -> &u64 {
        &self.money_supply
    }

    pub fn money_supply_formatted(&self) -> &String {
        &self.money_supply_formatted
    }

    pub fn zerocoin_supply(&self) -> &Vec<DenomInfo> {
        &self.zerocoin_supply
    }

    pub fn headers(&self) -> &u64 {
        &self.headers
    }

    pub fn best_block_hash(&self) -> &str {
        &self.best_block_hash
    }

    pub fn pow_difficulty(&self) -> &f64 {
        &self.pow_difficulty
    }

    pub fn pos_difficulty(&self) -> &f64 {
        &self.pos_difficulty
    }

    pub fn median_time(&self) -> &u64 {
        &self.median_time
    }

    pub fn verification_progress(&self) -> &f64 {
        &self.verification_progress
    }

    pub fn is_ibd(&self) -> bool {
        self.initial_block_download
    }

    pub fn chain_work(&self) -> &str {
        &self.chain_work
    }

    pub fn chain_pow(&self) -> &str {
        &self.chain_pow
    }

    pub fn size_on_disk(&self) -> &u64 {
        &self.size_on_disk
    }

    pub fn softforks(&self) -> &Bip9 {
        &self.bip9_softforks
    }

    pub fn warnings(&self) -> &str {
        &self.warnings
    }
}

impl TryFrom<Value> for BlockchainInfo {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for BlockchainInfo {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ChainTip {
    height: u64,
    hash: String,
    #[serde(rename = "branchlen")]
    branch_len: u64,
    status: String,
}

impl ChainTip {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn height(&self) -> &u64 {
        &self.height
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn branch_len(&self) -> &u64 {
        &self.branch_len
    }
}

impl TryFrom<Value> for ChainTip {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for ChainTip {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ChainTxStats {
    time: u64,
    #[serde(rename = "txcount")]
    tx_count: u64,
    window_final_block_hash: String,
    window_block_count: u64,
    #[serde(rename = "window_tx_count")]
    window_tx_count: u64,
    #[serde(rename = "txrate")]
    tx_rate: f64,
}

impl ChainTxStats {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn time(&self) -> &u64 {
        &self.time
    }

    pub fn tx_count(&self) -> &u64 {
        &self.tx_count
    }

    pub fn window_final_block_hash(&self) -> &str {
        &self.window_final_block_hash
    }

    pub fn window_block_count(&self) -> &u64 {
        &self.window_block_count
    }

    pub fn window_tx_count(&self) -> &u64 {
        &self.window_tx_count
    }
}

impl TryFrom<Value> for ChainTxStats {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for ChainTxStats {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MempoolInfo {
    size: u64,
    bytes: u64,
    usage: u64,
    #[serde(rename = "maxmempool")]
    max_mempool: u64,
    #[serde(rename = "mempoolminfee")]
    mempool_min_fee: f64,
    #[serde(rename = "minrelaytxfee")]
    min_relay_tx_fee: f64,
}

impl MempoolInfo {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn size(&self) -> &u64 {
        &self.size
    }

    pub fn bytes(&self) -> &u64 {
        &self.bytes
    }

    pub fn usage(&self) -> &u64 {
        &self.usage
    }

    pub fn max_mempool(&self) -> &u64 {
        &self.max_mempool
    }

    pub fn mempool_min_fee(&self) -> &f64 {
        &self.mempool_min_fee
    }

    pub fn min_relay_tx_fee(&self) -> &f64 {
        &self.min_relay_tx_fee
    }
}

impl TryFrom<Value> for MempoolInfo {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for MempoolInfo {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MempoolFee {
    base: f64,
    modified: f64,
    ancestor: f64,
    descendant: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MempoolTx {
    size: u64,
    fee: f64,
    #[serde(rename = "modifiedfee")]
    modified_fee: f64,
    #[serde(rename = "time")]
    timestamp: u64,
    height: u64,
    #[serde(rename = "descendantcount")]
    descendant_len: u64,
    #[serde(rename = "descendantsize")]
    descendant_size: u64,
    #[serde(rename = "descendantfees")]
    descendant_fees: u64,
    #[serde(rename = "ancestorcount")]
    ancestor_len: u64,
    #[serde(rename = "ancestorsize")]
    ancestor_size: u64,
    #[serde(rename = "ancestorfees")]
    ancestor_fees: u64,
    #[serde(rename = "wtxid")]
    witness_tx_id: String,
    fees: MempoolFee,
    depends: Vec<String>,
    #[serde(rename = "spentby")]
    spent_by: Vec<String>,
}

impl MempoolTx {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn size(&self) -> &u64 {
        &self.size
    }

    pub fn fee(&self) -> &f64 {
        &self.fee
    }

    pub fn modified_fee(&self) -> &f64 {
        &self.modified_fee
    }

    pub fn timestamp(&self) -> &u64 {
        &self.timestamp
    }

    pub fn height(&self) -> &u64 {
        &self.height
    }

    pub fn descendant_len(&self) -> &u64 {
        &self.descendant_len
    }

    pub fn descendant_size(&self) -> &u64 {
        &self.descendant_size
    }

    pub fn descendant_fees(&self) -> &u64 {
        &self.descendant_fees
    }

    pub fn ancestor_len(&self) -> &u64 {
        &self.ancestor_len
    }

    pub fn ancestor_size(&self) -> &u64 {
        &self.ancestor_size
    }

    pub fn ancestor_fees(&self) -> &u64 {
        &self.ancestor_fees
    }

    pub fn witness_tx_id(&self) -> &str {
        &self.witness_tx_id
    }

    pub fn fees(&self) -> &MempoolFee {
        &self.fees
    }

    pub fn depends(&self) -> &Vec<String> {
        &self.depends
    }

    pub fn spent_by(&self) -> &Vec<String> {
        &self.spent_by
    }
}

impl TryFrom<Value> for MempoolTx {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for MempoolTx {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MempoolAcceptTest {
    #[serde(rename = "txid")]
    tx_id: String,
    allowed: bool,
    #[serde(rename = "reject-reason")]
    reject_reason: String,
}

impl MempoolAcceptTest {
    pub fn tx_id(&self) -> &str {
        &self.tx_id
    }

    pub fn allowed(&self) -> &bool {
        &self.allowed
    }

    pub fn reject_reason(&self) -> &str {
        &self.reject_reason
    }
}

impl TryFrom<Value> for MempoolAcceptTest {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for MempoolAcceptTest {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TransactionOutSetInfo {
    height: u64,
    #[serde(rename = "bestblock")]
    best_block: String,
    #[serde(rename = "transactions")]
    transactions_len: u64,
    #[serde(rename = "txouts")]
    transaction_outs_len: u64,
    #[serde(rename = "bogosize")]
    bogo_size: u64,
    #[serde(rename = "hash_serialized_2")]
    hash_serialized: String,
    disk_size: u64,
    #[serde(rename = "total_amount")]
    coin_total: f64,
}

impl TransactionOutSetInfo {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn height(&self) -> &u64 {
        &self.height
    }

    pub fn best_block(&self) -> &str {
        &self.best_block
    }

    pub fn txs(&self) -> &u64 {
        &self.transactions_len
    }

    pub fn tx_outs_len(&self) -> &u64 {
        &self.transaction_outs_len
    }

    pub fn bogo_size(&self) -> &u64 {
        &self.bogo_size
    }

    pub fn hash_serialized(&self) -> &str {
        &self.hash_serialized
    }

    pub fn disk_size(&self) -> &u64 {
        &self.disk_size
    }

    pub fn coin_total(&self) -> &f64 {
        &self.coin_total
    }
}

impl TryFrom<Value> for TransactionOutSetInfo {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for TransactionOutSetInfo {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct LockedMemoryManager {
    used: u64,
    free: u64,
    total: u64,
    locked: u64,
    chunks_used: u64,
    chunks_free: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MemoryInfo {
    locked: LockedMemoryManager,
}

impl MemoryInfo {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn used(&self) -> &u64 {
        &self.locked.used
    }

    pub fn free(&self) -> &u64 {
        &self.locked.free
    }

    pub fn total(&self) -> &u64 {
        &self.locked.total
    }

    pub fn locked(&self) -> &u64 {
        &self.locked.locked
    }

    pub fn chunks_used(&self) -> &u64 {
        &self.locked.chunks_used
    }

    pub fn chunks_free(&self) -> &u64 {
        &self.locked.chunks_free
    }
}

impl TryFrom<Value> for MemoryInfo {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for MemoryInfo {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BlockTransaction {
    data: String,
    #[serde(rename = "tx_id")]
    tx_id: String,
    hash: String,
    depends: Vec<u64>,
    fee: u64,
    #[serde(rename = "sigops")]
    sig_ops: u64,
    weight: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BlockTemplate {
    capabilities: Vec<String>,
    version: u64,
    rules: Vec<String>,
    #[serde(rename = "vbavailable")]
    vb_available: HashMap<String, u64>,
    #[serde(rename = "vbrequired")]
    vb_required: u64,
    #[serde(rename = "previousblockhash")]
    previous_blockhash: String,
    transactions: Vec<BlockTransaction>,
    #[serde(rename = "coinbaseaux")]
    coinbase_aux: HashMap<String, String>,
    #[serde(rename = "coinbasevalue")]
    coinbase_value: u64,
    // #[serde(rename = "coinbase_txn")]
    // coinbase_txn: // Not in Veil? Might be for PoW .. ?
    #[serde(rename = "longpollid")]
    long_poll_id: String,
    #[serde(rename = "target")]
    target: String,
    #[serde(rename = "mintime")]
    min_time: u64,
    mutable: Vec<String>,
    #[serde(rename = "noncerange")]
    nonce_range: String,
    #[serde(rename = "sigoplimit")]
    sig_op_limit: u64,
    #[serde(rename = "sizelimit")]
    size_limit: u64,
    #[serde(rename = "weightlimit")]
    weight_limit: u64,
    #[serde(rename = "curtime")]
    current_time: u64,
    bits: String,
    height: u64,
    #[serde(rename = "accumulatorhashes")]
    accumulator_hashes: HashMap<String, String>,
    #[serde(rename = "proofoffullnodehash")]
    proof_of_fullnode_hash: String,
}

impl BlockTemplate {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn capabilities(&self) -> &[String] {
        &self.capabilities
    }

    pub fn version(&self) -> &u64 {
        &self.version
    }

    pub fn rules(&self) -> &[String] {
        &self.rules
    }

    pub fn vb_available(&self) -> &HashMap<String, u64> {
        &self.vb_available
    }

    pub fn vb_required(&self) -> &u64 {
        &self.vb_required
    }

    pub fn previous_blockhash(&self) -> &str {
        &self.previous_blockhash
    }

    pub fn transactions(&self) -> &[BlockTransaction] {
        &self.transactions
    }

    pub fn coinbase_aux(&self) -> &HashMap<String, String> {
        &self.coinbase_aux
    }

    pub fn coinbase_value(&self) -> &u64 {
        &self.coinbase_value
    }

    pub fn long_poll_id(&self) -> &str {
        &self.long_poll_id
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn min_time(&self) -> &u64 {
        &self.min_time
    }

    pub fn mutable(&self) -> &[String] {
        &self.mutable
    }

    pub fn nonce_range(&self) -> &str {
        &self.nonce_range
    }

    pub fn sig_op_limit(&self) -> &u64 {
        &self.sig_op_limit
    }

    pub fn size_limit(&self) -> &u64 {
        &self.size_limit
    }

    pub fn weight_limit(&self) -> &u64 {
        &self.weight_limit
    }

    pub fn current_time(&self) -> &u64 {
        &self.current_time
    }

    pub fn bits(&self) -> &str {
        &self.bits
    }

    pub fn height(&self) -> &u64 {
        &self.height
    }

    pub fn accumulator_hashes(&self) -> &HashMap<String, String> {
        &self.accumulator_hashes
    }

    pub fn proof_of_fullnode_hash(&self) -> &str {
        &self.proof_of_fullnode_hash
    }
}

impl TryFrom<Value> for BlockTemplate {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for BlockTemplate {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MiningInfo {
    chain: String,
    blocks: u64,
    #[serde(rename = "currentblockweight")]
    block_weight: u64,
    #[serde(rename = "currentblocktx")]
    block_tx_len: u64, // I think this is the length.
    difficulty: f64,
    #[serde(rename = "networkhashps")]
    network_hashps: f64, // NOTE In Veil, this metric is actually wrong as mining is different.
    #[serde(rename = "pooledtx")]
    pooled_tx: u64,
    warnings: String,
}

impl MiningInfo {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn chain(&self) -> &str {
        &self.chain
    }

    pub fn blocks(&self) -> &u64 {
        &self.blocks
    }

    pub fn block_weight(&self) -> &u64 {
        &self.block_weight
    }

    pub fn block_tx_len(&self) -> &u64 {
        &self.block_tx_len
    }

    pub fn difficulty(&self) -> &f64 {
        &self.difficulty
    }

    pub fn network_hashps(&self) -> &f64 {
        &self.network_hashps
    }

    pub fn pooled_tx(&self) -> &u64 {
        &self.pooled_tx
    }

    pub fn warnings(&self) -> &str {
        &self.warnings
    }
}

impl TryFrom<Value> for MiningInfo {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for MiningInfo {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LocalAddress {
    address: String,
    port: u16,
    score: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Networks {
    name: String,
    limited: bool,
    reachable: bool,
    proxy: String,
    proxy_randomize_credentials: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct UploadTarget {
    #[serde(rename = "timeframe")]
    time_frame: u64,
    target: u64,
    target_reached: bool,
    serve_historical_blocks: bool,
    bytes_left_in_cycle: u64,
    time_left_in_cycle: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct NetTotals {
    #[serde(rename = "totalbytesrecv")]
    total_bytes_recv: u64,
    #[serde(rename = "totalbytessent")]
    total_bytes_sent: u64,
    #[serde(rename = "timemillis")]
    time_millis: u64,
    #[serde(rename = "uploadtarget")]
    upload_target: UploadTarget,
}

impl NetTotals {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn total_bytes_recv(&self) -> &u64 {
        &self.total_bytes_recv
    }

    pub fn total_bytes_sent(&self) -> &u64 {
        &self.total_bytes_sent
    }

    pub fn time_millis(&self) -> &u64 {
        &self.time_millis
    }

    pub fn upload_target(&self) -> &UploadTarget {
        &self.upload_target
    }
}

impl TryFrom<Value> for NetTotals {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for NetTotals {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct NetworkInfo {
    version: u64,
    #[serde(rename = "subversion")]
    sub_version: String,
    #[serde(rename = "protocolversion")]
    protocol_version: u64,
    #[serde(rename = "localservices")]
    // https://github.com/bitcoin/bitcoin/pull/15511
    services: FlagHex,
    #[serde(rename = "localrelay")]
    relay: bool,
    #[serde(rename = "timeoffset")]
    time_offset: i64,
    #[serde(rename = "networkactive")]
    network_active: bool,
    connections: u64,
    networks: Vec<Networks>,
    #[serde(rename = "relayfee")]
    relay_fee: f64,
    #[serde(rename = "incrementalfee")]
    incremental_fee: f64,
    #[serde(rename = "localaddresses")]
    addresses: Vec<LocalAddress>,
    warnings: String,
}

impl NetworkInfo {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn version(&self) -> &u64 {
        &self.version
    }

    pub fn sub_version(&self) -> &str {
        &self.sub_version
    }

    pub fn protocol_version(&self) -> &u64 {
        &self.protocol_version
    }

    pub fn services(&self) -> &str {
        &self.services
    }

    pub fn relay(&self) -> bool {
        self.relay
    }

    pub fn time_offset(&self) -> &i64 {
        &self.time_offset
    }

    pub fn network_active(&self) -> bool {
        self.network_active
    }

    pub fn connections(&self) -> &u64 {
        &self.connections
    }

    pub fn networks(&self) -> &Vec<Networks> {
        &self.networks
    }

    pub fn relay_fee(&self) -> &f64 {
        &self.relay_fee
    }

    pub fn incremental_fee(&self) -> &f64 {
        &self.incremental_fee
    }

    pub fn addresses(&self) -> &Vec<LocalAddress> {
        &self.addresses
    }

    pub fn warnings(&self) -> &str {
        &self.warnings
    }
}

impl TryFrom<Value> for NetworkInfo {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for NetworkInfo {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct NodeAddr {
    address: SocketAddr,
    connect: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AddedNodeInfo {
    #[serde(rename = "addednode")]
    ip: IpAddr,
    connected: bool,
    addresses: Option<Vec<NodeAddr>>,
}

impl AddedNodeInfo {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn ip(&self) -> &IpAddr {
        &self.ip
    }

    pub fn connected(&self) -> bool {
        self.connected
    }

    pub fn addresses(&self) -> Option<&Vec<NodeAddr>> {
        self.addresses.as_ref()
    }
}

impl TryFrom<Value> for AddedNodeInfo {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for AddedNodeInfo {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PeerInfo {
    id: u64,
    #[serde(rename = "addr")]
    address: String,
    #[serde(rename = "addrbind")]
    address_bind: String,
    #[serde(rename = "addrlocal")]
    address_local: String,
    services: FlagHex,
    #[serde(rename = "relaytxes")]
    relay_txs: bool,
    #[serde(rename = "lastsend")]
    last_send: u64,
    #[serde(rename = "lastrecv")]
    last_recv: u64,
    #[serde(rename = "bytessent")]
    bytes_sent: u64,
    #[serde(rename = "bytesrecv")]
    bytes_recv: u64,
    #[serde(rename = "conntime")]
    connection_time: u64,
    #[serde(rename = "timeoffset")]
    time_offset: i64,
    #[serde(rename = "pingtime")]
    ping_time: f64,
    #[serde(rename = "pingwait")]
    ping_wait: f64, // Doesn't exist on all
    #[serde(rename = "pingmin")]
    min_ping: f64,
    version: u64,
    #[serde(rename = "subver")]
    sub_version: String,
    inbound: bool,
    #[serde(rename = "addednode")]
    added_node: bool,
    #[serde(rename = "startingheight")]
    starting_height: i64,
    #[serde(rename = "banscore")]
    ban_score: u64,
    #[serde(rename = "syncedheaders")]
    synced_headers: i64,
    #[serde(rename = "syncedblocks")]
    synced_blocks: i64,
    #[serde(rename = "inflight")]
    blocks_in_flight: Vec<u64>,
    whitelisted: bool,
    #[serde(rename = "bytessent_per_msg")]
    bytes_sent_per_msg: HashMap<String, u64>,
    #[serde(rename = "bytesrecv_per_msg")]
    bytes_recv_per_msg: HashMap<String, u64>,
}

impl PeerInfo {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn id(&self) -> &u64 {
        &self.id
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn address_bind(&self) -> &str {
        &self.address_bind
    }

    pub fn address_local(&self) -> &str {
        &self.address_local
    }

    pub fn services(&self) -> &str {
        &self.services
    }

    pub fn relay_txs(&self) -> bool {
        self.relay_txs
    }

    pub fn last_send(&self) -> &u64 {
        &self.last_send
    }

    pub fn last_recv(&self) -> &u64 {
        &self.last_recv
    }

    pub fn bytes_sent(&self) -> &u64 {
        &self.bytes_sent
    }

    pub fn bytes_recv(&self) -> &u64 {
        &self.bytes_recv
    }

    pub fn connection_time(&self) -> &u64 {
        &self.connection_time
    }

    pub fn time_offset(&self) -> &i64 {
        &self.time_offset
    }

    pub fn ping_time(&self) -> &f64 {
        &self.ping_time
    }

    pub fn ping_wait(&self) -> &f64 {
        &self.ping_wait
    }

    pub fn min_ping(&self) -> &f64 {
        &self.min_ping
    }

    pub fn version(&self) -> &u64 {
        &self.version
    }

    pub fn sub_version(&self) -> &str {
        &self.sub_version
    }

    pub fn inbound(&self) -> bool {
        self.inbound
    }

    pub fn added_node(&self) -> bool {
        self.added_node
    }

    pub fn starting_height(&self) -> &i64 {
        &self.starting_height
    }

    pub fn ban_score(&self) -> &u64 {
        &self.ban_score
    }

    pub fn synced_headers(&self) -> &i64 {
        &self.synced_headers
    }

    pub fn synced_blocks(&self) -> &i64 {
        &self.synced_blocks
    }

    pub fn blocks_in_flight(&self) -> &Vec<u64> {
        &self.blocks_in_flight
    }

    pub fn whitelisted(&self) -> bool {
        self.whitelisted
    }

    pub fn bytes_sent_per_msg(&self) -> &HashMap<String, u64> {
        &self.bytes_sent_per_msg
    }

    pub fn bytes_recv_per_msg(&self) -> &HashMap<String, u64> {
        &self.bytes_recv_per_msg
    }
}

impl TryFrom<Value> for PeerInfo {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for PeerInfo {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Banned {
    address: String,
    banned_until: u64,
    ban_created: u64,
    ban_reason: String,
}

impl Banned {
    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn banned_until(&self) -> &u64 {
        &self.banned_until
    }

    pub fn ban_created(&self) -> &u64 {
        &self.ban_created
    }

    pub fn ban_reason(&self) -> &str {
        &self.ban_reason
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Label {
    name: String,
    purpose: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AddressInfo {
    address: String,
    #[serde(rename = "scriptPubKey")]
    script_pub_key: String,
    #[serde(rename = "ismine")]
    is_mine: bool,
    #[serde(rename = "iswatchonly")]
    is_watch_only: bool,
    #[serde(rename = "isscript")]
    is_script: bool,
    #[serde(rename = "iswitness")]
    is_witness: bool,
    witness_version: Option<u64>,
    witness_program: Option<String>,
    script: Option<String>,
    hex: Option<String>,
    #[serde(rename = "pubkeys")]
    pub_keys: Option<Vec<String>>,
    #[serde(rename = "sigsrequired")]
    sigs_required: Option<u64>,
    pub_key: Option<String>,
    // embedded: // need to find example
    #[serde(rename = "iscompressed")]
    is_compressed: bool,
    label: String,
    timestamp: Option<u64>,
    #[serde(rename = "hdkeypath")]
    hd_key_path: Option<String>,
    #[serde(rename = "hdseedid")]
    hd_seed_id: Option<String>,
    #[serde(rename = "hdmasterkeyid")]
    hd_master_key_id: Option<String>,
    labels: Vec<Label>,
}

impl AddressInfo {
    pub fn new_from_value(v: Value) -> Result<Self> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn script_pub_key(&self) -> &str {
        &self.script_pub_key
    }

    pub fn is_mine(&self) -> bool {
        self.is_mine
    }

    pub fn is_watch_only(&self) -> bool {
        self.is_watch_only
    }

    pub fn is_script(&self) -> bool {
        self.is_script
    }

    pub fn is_witness(&self) -> bool {
        self.is_witness
    }

    pub fn witness_version(&self) -> Option<&u64> {
        self.witness_version.as_ref()
    }

    pub fn witness_program(&self) -> Option<&String> {
        self.witness_program.as_ref()
    }

    pub fn script(&self) -> Option<&String> {
        self.script.as_ref()
    }

    pub fn hex(&self) -> Option<&String> {
        self.hex.as_ref()
    }

    pub fn sigs_required(&self) -> Option<&u64> {
        self.sigs_required.as_ref()
    }

    pub fn pub_key(&self) -> Option<&String> {
        self.pub_key.as_ref()
    }

    pub fn is_compressed(&self) -> bool {
        self.is_compressed
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn timestamp(&self) -> Option<&u64> {
        self.timestamp.as_ref()
    }

    pub fn hd_key_path(&self) -> Option<&String> {
        self.hd_key_path.as_ref()
    }

    pub fn hd_seed_id(&self) -> Option<&String> {
        self.hd_seed_id.as_ref()
    }

    pub fn hd_master_key_id(&self) -> Option<&String> {
        self.hd_master_key_id.as_ref()
    }

    pub fn labels(&self) -> &Vec<Label> {
        &self.labels
    }
}

impl TryFrom<Value> for AddressInfo {
    type Error = Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(v)?;

        Ok(s)
    }
}

impl TryFrom<Response> for AddressInfo {
    type Error = Error;

    fn try_from(res: Response) -> Result<Self, Self::Error> {
        let s: Self = serde_json::from_value(res.result().unwrap().to_owned())?;
        Ok(s)
    }
}
