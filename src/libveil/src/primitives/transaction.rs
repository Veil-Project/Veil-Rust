use crate::amount::{self, Amount};
use crate::crypto;
use crate::u256;
use std::convert::TryFrom;

pub trait ToOutpoint {
    fn to_outpoint(&self) -> OutPoint;
}

#[derive(Debug, PartialEq)]
pub enum OutputKind {
    Null = 0,
    Standard = 1,
    Ct = 2,
    RingCt = 3,
    Data = 4,
}

#[derive(Debug, PartialEq)]
pub enum TransactionKind {
    Standard,
    Coinbase,
    CoinStake,
}

#[derive(Debug, PartialEq)]
pub enum DataOutputKind {
    Null,
    NarrPlain,
    NarrCrypt,
    Stealth,
    StealthPrefix,
    Vote,
    Fee,
    DevFund,
    FundMessage,
}

// TODO: Serialization to bytes.
// TODO: Partially eq, not equal, greater
// TODO: to_string() should be what the debug is
#[derive(Debug, PartialOrd, PartialEq)]
/// An outpoint
///
/// A combination of a transaction hash and an index sequence number into its out.
pub struct OutPoint {
    hash: u256,
    sequence: u32,
}

impl OutPoint {
    pub const ANON_MARKER: u32 = 0xffff_ffa0;

    pub fn new(hash: u256, sequence: u32) -> Self {
        Self { hash, sequence }
    }

    // TODO: hash then n
    //    pub fn serialize

    pub fn clear(&mut self) {
        self.hash = u256::from(0u32);
        self.sequence = 0;
    }

    pub fn set_anon(&mut self, input_len: u32, ring_len: u32) {
        self.hash.0[0] = ((input_len as u64) << 32) | ring_len as u64;
    }

    pub fn anon(&self) -> (u32, u32) {
        let word = self.hash.0[0];
        let lhs: u32 = u32::try_from(word >> 32).unwrap();
        let rhs = word as u32;
        (lhs, rhs)
    }

    pub fn is_empty(&self) -> bool {
        self.hash.is_zero() && self.sequence == 0
    }

    // Might be for Zerocoin only?
    pub fn is_anon(&self) -> bool {
        self.sequence == Self::ANON_MARKER
    }
}

// TODO: Actual script
pub struct TransactionIn {
    prev_out: OutPoint,
    script: Vec<u8>,
    sequence: u32,
    script_data: Vec<u8>,    // TODO: Actual script data struct
    script_witness: Vec<u8>, // TODO: Actual script witness struct
}

impl TransactionIn {
    /// Setting `sequence` to this value for every input in a transaction disables lock_time.
    pub const SEQUENCE_FINAL: u32 = 0xffff_ffff;

    // Below flags apply in the context of BIP 68.
    /// If this flag is set, `TransactionIn.sequence` is NOT interpreted as a relative lock-time.
    pub const SEQUENCE_LOCKTIME_DISABLE_FLAG: u32 = 1 << 31;

    /// If `TransactionIn,sequence` encodes a relative lock-time and this flag is set, the
    /// relative lock-time has units of 512 seconds, otherwise it specifies blocks with a
    /// granularity of 1.
    pub const SEQUENCE_LOCKTIME_TYPE_FLAG: u32 = 1 << 22;

    /// If `TransactionIn.sequence` encodes a relative lock-time, this mask is applied to extract
    /// that lock-time from the sequence field.
    pub const SEQUENCE_LOCKTIME_MASK: u32 = 0x0000_ffff;

    /// In order to use the same number of bits to encode roughly the same wall-clock duration,
    /// and because blocks are naturally limited to occur every 60s on average,the minimum
    /// granularity for time-based relative lock-time is fixed at 512 seconds. Converting from
    /// `TransactionIn.sequence` to seconds is performed by multiplying by 512 = 2 ^ 9, or
    /// equivalently shifting up by 9 bits.
    pub const SEQUENCE_LOCKTIME_GRANULARITY: u32 = 9;

    pub fn new(prev_out: OutPoint, script: Vec<u8>, sequence: u32) -> Self {
        Self {
            prev_out,
            script,
            sequence,
            script_data: Vec::new(),
            script_witness: Vec::new(),
        }
    }

    pub fn is_anon(&self) -> bool {
        self.prev_out.is_anon()
    }

    pub fn set_anon(&mut self, input_len: u32, ring_len: u32) {
        self.prev_out.set_anon(input_len, ring_len);
    }

    pub fn anon(&self) -> (u32, u32) {
        self.prev_out.anon()
    }
}

pub struct StandardOut {
    amount: Amount,
    script: Vec<u8>, // TODO: Actual script data struct
}

impl StandardOut {
    pub fn new(amount: Amount, script: Vec<u8>) -> Self {
        Self { amount, script }
    }

    pub fn set_value(&mut self, amount: Amount) {
        self.amount = amount;
    }

    pub fn set_script(&mut self, script: Vec<u8>) {
        self.script = script;
    }

    pub fn increase_value(&mut self, amount: Amount) {
        self.amount += amount;
    }

    pub fn decrease_value(&mut self, amount: Amount) {
        self.amount -= amount;
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn script(&self) -> &[u8] {
        &self.script
    }

    pub fn clear(&mut self) {
        self.amount = 0;
        self.script = Vec::new(); // TODO: change to method in script.
    }

    pub fn clear_value(&mut self) {
        self.amount = 0;
    }

    pub fn clear_script(&mut self) {
        self.script = Vec::new();
    }

    pub fn is_clear(&self) -> bool {
        self.amount == 0 && self.script == Vec::new()
    }
}

#[derive(Default)]
pub struct CtOut {
    ephemeral_public_key: Vec<u8>,
    data: Vec<u8>,
    commitment: Vec<u8>, // TOOD: Commitment
    script: Vec<u8>,     // TODO: Script
    range_proof: Vec<u8>,
}

impl CtOut {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_commitment(&mut self, commitment: Vec<u8>) {
        self.commitment = commitment
    }

    pub fn set_ephemeral_public_key(&mut self, public_key: Vec<u8>) {
        self.ephemeral_public_key = public_key
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data
    }

    pub fn set_script(&mut self, script: Vec<u8>) {
        self.script = script
    }

    pub fn set_range_proof(&mut self, range_proof: Vec<u8>) {
        self.range_proof = range_proof
    }

    pub fn commitment(&self) -> &[u8] {
        &self.commitment
    }

    pub fn ephemeral_public_key(&self) -> &[u8] {
        &self.ephemeral_public_key
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn script(&self) -> &[u8] {
        &self.script
    }

    pub fn range_proof(&self) -> &[u8] {
        &self.range_proof
    }

    pub fn clear(&mut self) {
        self.commitment.clear();
        self.ephemeral_public_key.clear();
        self.data.clear();
        self.script.clear();
        self.range_proof.clear();
    }

    pub fn is_clear(&self) -> bool {
        self.commitment.is_empty()
            && self.ephemeral_public_key.is_empty()
            && self.data.is_empty()
            && self.script.is_empty()
            && self.range_proof.is_empty()
    }
}

#[derive(Default)]
pub struct RingCtOut {
    public_key: Vec<u8>,           // TODO: CmpPubKey
    ephemeral_public_key: Vec<u8>, // From vData, first 33 bytes.
    data: Vec<u8>,
    commitment: Vec<u8>,
    range_proof: Vec<u8>,
}

impl RingCtOut {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_public_key(&mut self, public_key: Vec<u8>) {
        self.public_key = public_key
    }

    pub fn set_commitment(&mut self, commitment: Vec<u8>) {
        self.commitment = commitment
    }

    pub fn set_ephemeral_public_key(&mut self, public_key: Vec<u8>) {
        self.ephemeral_public_key = public_key
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data
    }

    pub fn set_range_proof(&mut self, range_proof: Vec<u8>) {
        self.range_proof = range_proof
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    pub fn commitment(&self) -> &[u8] {
        &self.commitment
    }

    pub fn ephemeral_public_key(&self) -> &[u8] {
        &self.ephemeral_public_key
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn range_proof(&self) -> &[u8] {
        &self.range_proof
    }

    pub fn clear(&mut self) {
        self.public_key.clear();
        self.commitment.clear();
        self.ephemeral_public_key.clear();
        self.data.clear();
        self.range_proof.clear();
    }

    pub fn is_clear(&self) -> bool {
        self.public_key.is_empty()
            && self.commitment.is_empty()
            && self.ephemeral_public_key.is_empty()
            && self.data.is_empty()
            && self.range_proof.is_empty()
    }
}

pub struct DataOut(pub i64);

impl DataOut {
    pub fn set_fee(&mut self, fee: Amount) {
        self.0 = fee;
    }

    pub fn fee(&self) -> &i64 {
        &self.0
    }
}

pub struct Transaction {
    transaction_in: Vec<TransactionIn>,
    standard_out: Option<Vec<StandardOut>>,
    ct_out: Option<Vec<CtOut>>,
    ring_ct_out: Option<Vec<RingCtOut>>,
    data_out: Option<Vec<DataOut>>,
    version: u32,
    lock_time: u32,
    hash: u256,
    witness_hash: u256,
}

impl Transaction {
    /// Default transaction version.
    pub const CURRENT_VERSION: u32 = 2;

    /// Max transaction version.
    ///
    /// Changing the default transaction version requires a two step process: first adapting
    /// relay policy by bumping MAX_VERSION, and then later date bumping the default
    /// CURRENT_VERSION at which point both CURRENT_VERSION and MAX_VERSION will be equal.
    pub const MAX_VERSION: u32 = 2;

    pub fn new() -> Self {
        Self {
            transaction_in: Vec::new(),
            standard_out: None,
            ct_out: None,
            ring_ct_out: None,
            data_out: None,
            version: Self::CURRENT_VERSION,
            lock_time: 0,
            hash: u256([0u64; 4]),
            witness_hash: u256([0u64; 4]),
        }
    }

    pub fn push_transaction_in(&mut self, tx_in: TransactionIn) {
        self.transaction_in.push(tx_in);
    }

    pub fn set_transaction_ins(&mut self, tx_ins: Vec<TransactionIn>) {
        self.transaction_in = tx_ins;
    }

    pub fn push_standard_out(&mut self, std_out: StandardOut) {
        self.standard_out.get_or_insert(Vec::new()).push(std_out);
    }

    pub fn set_standard_outs(&mut self, std_outs: Vec<StandardOut>) {
        self.standard_out = Some(std_outs);
    }

    pub fn push_ct_out(&mut self, ct_out: CtOut) {
        self.ct_out.get_or_insert(Vec::new()).push(ct_out);
    }

    pub fn set_ct_outs(&mut self, ct_outs: Vec<CtOut>) {
        self.ct_out = Some(ct_outs);
    }

    pub fn push_ring_ct_out(&mut self, ring_ct_out: RingCtOut) {
        self.ring_ct_out.get_or_insert(Vec::new()).push(ring_ct_out);
    }

    pub fn set_ring_ct_outs(&mut self, ring_ct_outs: Vec<RingCtOut>) {
        self.ring_ct_out = Some(ring_ct_outs);
    }

    pub fn push_data_out(&mut self, data_out: DataOut) {
        self.data_out.get_or_insert(Vec::new()).push(data_out);
    }

    pub fn set_data_outs(&mut self, data_outs: Vec<DataOut>) {
        self.data_out = Some(data_outs);
    }

    pub fn data_out(&self) -> Option<&[DataOut]> {
        self.data_out.as_ref().map(|x| x.as_ref())
    }

    pub fn set_lock_time(&mut self, lock_time: u32) {
        self.lock_time = lock_time
    }

    pub fn transaction_in(&self) -> &[TransactionIn] {
        &self.transaction_in
    }

    pub fn standard_out(&self) -> Option<&[StandardOut]> {
        self.standard_out.as_ref().map(|x| x.as_ref())
    }

    pub fn ct_out(&self) -> Option<&[CtOut]> {
        self.ct_out.as_ref().map(|x| x.as_ref())
    }

    pub fn ring_ct_out(&self) -> Option<&[RingCtOut]> {
        self.ring_ct_out.as_ref().map(|x| x.as_ref())
    }

    pub fn version(&self) -> &u32 {
        &self.version
    }

    pub fn lock_time(&self) -> &u32 {
        &self.lock_time
    }

    pub fn hash(&self) -> &u256 {
        &self.hash
    }

    pub fn witness_hash(&self) -> &u256 {
        &self.witness_hash
    }

    pub fn input_len(&self) -> usize {
        self.transaction_in.len()
    }

    pub fn output_len(&self) -> usize {
        let mut count = 0usize;
        if let Some(v) = &self.standard_out {
            count += v.len()
        }
        if let Some(v) = &self.ct_out {
            count += v.len()
        }
        if let Some(v) = &self.ring_ct_out {
            count += v.len()
        }
        if let Some(v) = &self.data_out {
            count += v.len()
        }
        count
    }

    pub fn is_txin_empty(&self) -> bool {
        self.transaction_in.is_empty()
    }

    pub fn is_txout_empty(&self) -> bool {
        self.standard_out.is_none()
            && self.ct_out.is_none()
            && self.ring_ct_out.is_none()
            && self.data_out.is_none()
    }

    pub fn is_stake(&self) -> bool {
        if self.is_txin_empty()
            && self.transaction_in.len() != 1
            && self.output_len() > 1
            && self.standard_out.is_some()
        {
            self.standard_out.as_ref().unwrap()[0].is_clear()
        } else {
            false
        }
    }

    pub fn is_coinbase(&self) -> bool {
        self.transaction_in.len() == 1 && self.transaction_in[0].prev_out.is_empty()
    }

    pub fn has_standard_out(&self) -> bool {
        self.standard_out.is_some()
    }

    pub fn has_ct_out(&self) -> bool {
        self.ct_out.is_some()
    }

    pub fn has_ring_ct_out(&self) -> bool {
        self.ring_ct_out.is_some()
    }

    pub fn has_data_out(&self) -> bool {
        self.data_out.is_some()
    }

    pub fn standard_out_amount(&self) -> Amount {
        let mut amount: Amount = 0;
        if let Some(v) = &self.standard_out {
            for out in v {
                amount += out.amount();
                if !amount::money_range(&out.amount()) || !amount::money_range(&amount) {
                    println!("TODO: Throw value out of range error")
                }
            }
        }
        amount
    }

    // TOOD: Need sha526
    //    fn hash_data()

    //    fn hash_witness_data()
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}
