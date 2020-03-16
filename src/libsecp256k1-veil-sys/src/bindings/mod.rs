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

//! Secp256k1-veil ffi
//!
//! These rules specify the order of arguments in API calls:
//!
//! 1. Context pointers go first, followed by output arguments, combined
//! output/input arguments, and finally input-only arguments.
//!
//! 2. Array lengths always immediately the follow the argument whose length
//! they describe, even if this violates rule 1.
//!
//! 3. Within the OUT/OUTIN/IN groups, pointers to data that is typically generated
//! later go first. This means: signatures, public nonces, private nonces,
//! messages, public keys, secret keys, tweaks.
//!
//! 4. Arguments that are not data pointers go last, from more complex to less
//! complex: function pointers, algorithm names, messages, void pointers,
//! counts, flags, booleans.
//!
//! 5. Opaque data pointers follow the function pointer they are to be passed to.
use crate::c_types::{c_int, c_uchar, c_uint, c_void, size_t};
use crate::{Context, PublicKey, Signature};

pub mod ecdh;
pub mod generator;
pub mod mlsag;
pub mod rangeproof;
pub mod recovery;
pub mod schnorr;
pub mod surjectionproof;
pub mod whitelist;

const FLAGS_TYPE_MASK: u32 = ((1 << 8) - 1);
const FLAGS_TYPE_CONTEXT: u32 = 1 << 0;
const FLAGS_TYPE_COMPRESSION: u32 = 1 << 1;

const FLAGS_BIT_CONTEXT_VERIFY: u32 = 1 << 8;
const FLAGS_BIT_CONTEXT_SIGN: u32 = 1 << 9;
const FLAGS_BIT_COMPRESSION: u32 = 1 << 8;

pub const CONTEXT_VERIFY: u32 = (FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_VERIFY);
pub const CONTEXT_SIGN: u32 = (FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_SIGN);
pub const CONTEXT_NONE: u32 = FLAGS_TYPE_CONTEXT;

pub const EC_COMPRESSED: u32 = (FLAGS_TYPE_COMPRESSION | FLAGS_BIT_COMPRESSION);
pub const EC_UNCOMPRESSED: u32 = FLAGS_TYPE_COMPRESSION;

const TAG_PUBKEY_EVEN: u32 = 0x02;
const TAG_PUBKEY_ODD: u32 = 0x03;
const TAG_PUBKEY_UNCOMPRESSED: u32 = 0x04;
const TAG_PUBKEY_HYBRID_EVEN: u32 = 0x06;
const TAG_PUBKEY_HYBRID_ODD: u32 = 0x07;

/// A pointer to a function to deterministically generate a nonce.
///
/// Except for test cases, this function should compute some cryptographic hash of
/// the message, the algorithm, the key and the attempt.
///
/// # Arguments
///
/// * `msg32` - The 32-byte message hash being verified (will not be NULL).
/// * `key32` - A pointer to a 32-byte secret key (will not be NULL).
/// * `algo16` -  A pointer to a 16-byte array describing the signature algorithm
/// (will be NULL for ECDSA for compatibility).
/// * `data` - Arbitrary data pointer that is passed through.
/// * `attempt` - How many iterations we have tried to find a nonce. This will
/// almost always be 0, but different attempt values are required to result in a
/// different nonce.
///
/// # Out
///
/// * `nonce32` - A pointer to a 32-byte array to be filled by the function.
///
/// # Returns
///
/// * `1` - If a nonce was successfully generated.
/// * `0` - Will cause signing to fail.
pub type NonceFunction = unsafe extern "C" fn(
    nonce32: *mut c_uchar,
    msg32: *const c_uchar,
    key32: *const c_uchar,
    algo16: *const c_uchar,
    data: *const c_void,
    attempt: c_uint,
) -> c_uint;

#[link(name = "secp256k1-veil")]
extern "C" {
    pub static secp256k1_nonce_function_rfc6979: NonceFunction;

    pub static secp256k1_nonce_function_default: NonceFunction;

    /// Create a secp256k1 context object.
    ///
    /// See also secp256k1_context_randomize.
    ///
    /// # Arguments
    ///
    /// * `flags` - Which parts of the context to initialize.
    ///
    /// # Returns
    ///
    /// * A newly created [`Context`] object.
    pub fn secp256k1_context_create(flag: c_uint) -> *mut Context;

    /// Copies a secp256k1 context object.
    ///
    /// # Arguments
    ///
    /// * `ctx` - An existing [`Context`] to copy.
    ///
    /// # Returns
    ///
    /// * A newly created [`Context`] object.
    pub fn secp256k1_context_clone(ctx: *mut Context) -> *mut Context;

    /// Destroy a secp256k1 context object.
    ///
    /// # Arguments
    ///
    /// * `ctx` - An existing [`Context`] object to destroy.
    pub fn secp256k1_context_destroy(ctx: *mut Context);

    /// Randomizes a secp256k1 context object.
    ///
    /// Updates the context randomization to protect against side-channel leakage.
    ///
    /// While secp256k1 code is written to be constant-time no matter what secret
    /// values are, it's possible that a future compiler may output code which
    /// isn't, and also that the CPU may not emit the same radio frequencies or
    /// draw the same amount power for all values.
    ///
    /// This function provides a seed which is combined into the blinding value: that
    /// blinding value is added before each multiplication (and removed afterwards) so
    /// that it does not affect function results, but shields against attacks which
    /// rely on any input-dependent behaviour.
    ///
    /// You should call this after secp256k1_context_create or
    ///  secp256k1_context_clone, and may call this repeatedly afterwards.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A pointer to a context object.
    /// * `seed32` - A 32-byte random seed.
    ///
    /// # Returns
    ///
    /// * `1` randomization successfully updated.
    /// * `0` error has occurred.
    pub fn secp256k1_context_randomize(ctx: *mut Context, seed32: *const c_uchar) -> c_uint;

    /// Parse a variable-length public key into the pubkey object.
    ///
    /// This function supports parsing compressed (33 bytes, header byte 0x02 or
    /// 0x03), uncompressed (65 bytes, header byte 0x04), or hybrid (65 bytes, header
    /// byte 0x06 or 0x07) format public keys.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A [`Context`] object.
    ///
    /// # Outs
    ///
    /// * `pub_key` - A pointer to a [`PublicKey`] object.
    ///
    /// # Ins
    ///
    /// * `input` - A pointer to a serialized public key.
    /// * `input_len` - The length of the array pointed to by input.
    ///
    /// # Returns
    ///
    /// * `1` - It is set to a parsed version of input.
    /// * `0` - Its value is undefined.
    pub fn secp256k1_ec_pubkey_parse(
        ctx: *const Context,
        public_key: *mut PublicKey,
        input: *const c_uchar,
        input_len: size_t,
    ) -> c_int;

    /// Serialize a pubkey object into a serialized byte sequence.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A [`Context'] object.
    ///
    /// # Outs
    ///
    /// * `output` - A pointer to a 65-byte (if compressed==0) or 33-byte (if
    /// compressed==1) byte array to place the serialized key in.
    ///
    /// # In/Out
    ///
    /// * `output_len` - A pointer to an integer which is initially set to the size
    /// of the output, and is overwritten with the written size.
    ///
    /// # Ins
    ///
    /// * `pub_key` - A pointer to a [`PublicKey`] containing a initialized
    /// public key.
    /// * `flags` - If serialization should be in compressed format, otherwise
    /// uncompressed.
    ///
    /// # Returns
    ///
    /// Always returns 1.
    pub fn secp256k1_ec_pubkey_serialize(
        ctx: *const Context,
        output: *mut c_uchar,
        output_len: *mut size_t,
        pub_key: *const PublicKey,
        flags: c_uint,
    ) -> c_int;

    /// Parse an ECDSA signature in compact (64 bytes) format.
    ///
    /// The signature must consist of a 32-byte big endian R value, followed by a
    /// 32-byte big endian S value. If R or S fall outside of [0..order-1], the
    /// encoding is invalid. R and S with value 0 are allowed in the encoding.
    ///
    /// After the call, sig will always be initialized. If parsing failed or R or
    /// S are zero, the resulting sig value is guaranteed to fail validation for any
    /// message and public key.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A [`Context`] object.
    ///
    /// # Out
    ///
    /// * `sig` - A pointer to a [`Signature`] object.
    ///
    /// # In
    ///
    /// * `input64` - A pointer to the 64-byte array to parse.
    ///
    /// # Result
    ///
    /// * `1` - Signature can be parsed.
    /// * `0` - Signature can not be parsed.
    pub fn secp256k1_ecdsa_signature_parse_compact(
        ctx: *const Context,
        sig: *mut Signature,
        input64: *const c_uchar,
    ) -> c_int;

    /// Parse a DER ECDSA signature.
    ///
    /// This function will accept any valid DER encoded signature, even if the
    /// encoded numbers are out of range.
    ///
    /// After the call, sig will always be initialized. If parsing failed or the
    /// encoded numbers are out of range, signature validation with it is
    /// guaranteed to fail for every message and public key.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A [`Context] object.
    ///
    /// # Out
    ///
    /// * `sig` - A pointer to a [`Signature`] object.
    ///
    /// # In
    ///
    /// * `input` - A pointer to the signature to be parsed.
    /// * `input_len` - The length of the array pointed to be input.
    ///
    /// Returns
    ///
    /// * `1` - Signature can be parsed.
    /// * `0` - Signature can not be parsed.
    pub fn secp256k1_ecdsa_signature_parse_der(
        ctx: *const Context,
        sig: *mut Signature,
        input: *const c_uchar,
        input_len: size_t,
    ) -> c_int;

    /// Serialize an ECDSA signature in DER format.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A [`Context`] object.
    ///
    /// # Out
    ///
    /// * `output` - A pointer to an array to store the DER serialization.
    ///
    /// # In/Out
    ///
    /// * `output_len` - A pointer to a length integer. Initially, this integer
    /// should be set to the length of output. After the call it will be set to the
    /// length of the serialization (even if 0 was returned).
    ///
    /// # In
    ///
    /// * `sig` - A pointer to the initialized [`Signature`] object.
    ///
    /// # Returns
    ///
    /// * `1` - If enough space was available to serialize.
    /// * `0` - If not enough space was available.
    pub fn secp256k1_ecdsa_signature_serialize_der(
        ctx: *const Context,
        output: *mut c_uchar,
        output_len: *mut size_t,
        sig: *const Signature,
    ) -> c_int;

    /// Serialize an ECDSA signature in compact (64 byte) format.
    ///
    /// See secp256k1_ecdsa_signature_parse_compact for details about the encoding.
    ///
    /// # Argument
    ///
    /// * `ctx` - A a secp256k1 context object.
    ///
    /// # Out
    ///
    /// * `output64` - A pointer to a 64-byte array to store the compact serialization.
    ///
    /// # In
    ///
    /// * `sig` - A pointer to an initialized signature object.
    pub fn secp256k1_ecdsa_signature_serialize_compact(
        ctx: *const Context,
        output64: *mut c_uchar,
        sig: *const Signature,
    ) -> c_int;

    /// Verify an ECDSA signature.
    ///
    /// To avoid accepting malleable signatures, only ECDSA signatures in lower-S
    /// form are accepted.
    ///
    /// If you need to accept ECDSA signatures from sources that do not obey this
    /// rule, apply secp256k1_ecdsa_signature_normalize to the signature prior to
    /// validation, but be aware that doing so results in malleable signatures.
    ///
    /// For details, see the comments for that function.
    ///
    /// # Argument
    ///
    /// * `ctx` - A secp256k1 context object, initialized for verification.
    ///
    /// # Ins
    ///
    /// * `sig` - The signature being verified (cannot be NULL).
    /// * `msg32` - The 32-byte message hash being verified (cannot be NULL).
    /// * `public_key` - A pointer to an initialized public key to verify with (cannot be NULL).
    ///
    /// # Returns
    ///
    /// * `1` - The signature is correct.
    /// * `0` - The signature is incorrect or not parsable.
    pub fn secp256k1_ecdsa_verify(
        ctx: *const Context,
        sig: *const Signature,
        msg32: *const c_uchar,
        public_key: *const PublicKey,
    ) -> c_int;

    /// Convert a signature to a normalized lower-S form.
    ///
    /// With ECDSA a third-party can forge a second distinct signature of the same
    /// message, given a single initial signature, but without knowing the key. This
    /// is done by negating the S value modulo the order of the curve, 'flipping'
    /// the sign of the random point R which is not included in the signature.
    ///
    /// Forgery of the same message isn't universally problematic, but in systems
    /// where message malleability or uniqueness of signatures is important this can
    /// cause issues. This forgery can be blocked by all verifiers forcing signers
    /// to use a normalized form.
    ///
    /// The lower-S form reduces the size of signatures slightly on average when
    /// variable length encodings (such as DER) are used and is cheap to verify,
    /// making it a good choice. Security of always using lower-S is assured because
    /// anyone can trivially modify a signature after the fact to enforce this
    /// property anyway.
    ///
    /// The lower S value is always between 0x1 and
    /// 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0,
    /// inclusive.
    ///
    /// No other forms of ECDSA malleability are known and none seem likely, but
    /// there is no formal proof that ECDSA, even with this additional restriction,
    /// is free of other malleability. Commonly used serialization schemes will also
    /// accept various non-unique encodings, so care should be taken when this
    /// property is required for an application.
    ///
    /// The secp256k1_ecdsa_sign function will by default create signatures in the
    /// lower-S form, and secp256k1_ecdsa_verify will not accept others. In case
    /// signatures come from a system that cannot enforce this property,
    /// secp256k1_ecdsa_signature_normalize must be called before verification.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A secp256k1 context object.
    ///
    /// # Out
    ///
    /// * `signature_out` - A pointer to a signature to fill with the normalized form,
    /// or copy if the input was already normalized. (can be NULL if you're only
    /// interested in whether the input was already normalized).
    ///
    /// # In
    ///
    /// * `signature_in` - A pointer to a signature to check/normalize (cannot be NULL,
    /// can be identical to sigout)
    ///
    /// # Returns
    ///
    /// * `1` - If `signature_in` was nor normalized.
    /// * `0` - If `signature_in` was already normalized.
    pub fn secp256k1_ecdsa_signature_normalize(
        ctx: *const Context,
        signature_out: *mut Signature,
        signature_in: *const Signature,
    ) -> c_int;

    /// Create an ECDSA signature.
    ///
    /// The created signature is always in lower-S form. See
    /// secp256k1_ecdsa_signature_normalize for more details.
    ///
    /// # Argument
    ///
    /// * `ctx` - A pointer to a context object, initialized for signing (cannot be NULL).
    ///
    /// # Out
    ///
    /// * `sig` - A pointer to an array where the signature will be placed (cannot be NULL).
    ///
    /// # In
    ///
    /// * `msg32` - The 32-byte message hash being signed (cannot be NULL).
    /// * `secret_key` - A pointer to a 32-byte secret key (cannot be NULL).
    /// * `nonce_fp` - A pointer to a nonce generation function. If NULL,
    /// secp256k1_nonce_function_default is used.
    /// * `n_data` - A pointer to arbitrary data used by the nonce generation function (can be
    /// NULL).
    ///
    /// # Returns
    /// * `1` - Signature created.
    /// * `0` - Nonce generation function failed or the private key was invalid.
    pub fn secp256k1_ecdsa_sign(
        ctx: *const Context,
        sig: *mut Signature,
        msg32: *const c_uchar,
        private_key: *const c_uchar,
        nonce_fp: NonceFunction,
        n_data: *const c_void,
    ) -> c_int;

    /// Verify an ECDSA secret key.
    ///
    /// # Argument
    ///
    /// * `ctx` - A pointer to a context object (cannot be NULL).
    ///
    /// # In
    ///
    /// * `private_key` - A pointer to a 32-byte secret key (cannot be NULL)
    ///
    /// # Returns
    ///
    /// * `1` - If valid.
    /// * `0` - If not valid.
    pub fn secp256k1_ec_seckey_verify(ctx: *const Context, private_key: *const c_uchar) -> c_int;

    /// Compute the public key for a private key.
    ///
    /// # Argument
    ///
    /// * `ctx` - A pointer to a context object, initialized for signing (cannot be NULL)
    ///
    /// # Out
    ///
    /// * `public_key` - A pointer to the created public key (cannot be NULL).
    ///
    /// # In
    ///
    /// * `private_key` - A pointer to a 32-byte private key (cannot be NULL).
    ///
    /// # Returns
    ///
    /// * `1` - Private key was valid.
    /// * `0` - Private key was invalid.
    pub fn secp256k1_ec_pubkey_create(
        ctx: *const Context,
        public_key: *mut PublicKey,
        private_key: *const c_uchar,
    ) -> c_int;

    /// Negates a private key in place.
    ///
    /// # Argument
    ///
    /// * `ctx` - A pointer to a context object.
    ///
    /// # In/Out
    ///
    /// * `private_key` - A pointer to the private key to be negated (cannot be NULL).
    ///
    /// # Returns
    ///
    /// Always returns 1.
    pub fn secp256k1_ec_privkey_negate(ctx: *const Context, private_key: *mut c_uchar) -> c_int;

    /// Negates a public key in place.
    ///
    /// # Argument
    ///
    /// * `ctx` - A pointer to a context object.
    ///
    /// # In/Out
    ///
    ///
    /// * `public_key` - A pointer to the public key to be negated (cannot be NULL).
    ///
    /// # Returns
    ///
    /// Always returns 1.
    pub fn secp256k1_ec_pubkey_negate(ctx: *const Context, public_key: *mut PublicKey) -> c_int;

    /// Tweak a private key by adding tweak to it.
    ///
    /// # Argument
    ///
    /// * `ctx` - A pointer to a context object (cannot be NULL).
    ///
    /// # In/Out
    ///
    /// * `private_key` - A pointer to a 32-byte private key.
    /// * `tweak` - A pointer to a 32-byte tweak.
    ///
    /// # Returns
    ///
    /// * `1` - If successful.
    /// * `0` - if the tweak was out of range (chance of around 1 in 2^128 for
    /// uniformly random 32-byte arrays, or if the resulting private key would be
    /// invalid (only when the tweak is the complement of the private key).
    pub fn secp256k1_ec_privkey_tweak_add(
        ctx: *const Context,
        private_key: *mut c_uchar,
        tweak: *const c_uchar,
    ) -> c_int;

    /// Tweak a public key by adding tweak times the generator to it.
    ///
    /// # Argument
    ///
    /// * `ctx` - A pointer to a context object initialized for validation (cannot
    /// be NULL).
    ///
    /// # In/Out
    ///
    /// * `public_key` - A pointer to a public key object.
    ///
    /// # In
    ///
    /// * `tweak` - A pointer to a 32-byte tweak.
    ///
    /// # Returns
    ///
    /// * `1` - If successful.
    /// * `0` - if the tweak was out of range (chance of around 1 in 2^128 for
    /// uniformly random 32-byte arrays, or if the resulting private key would be
    /// invalid (only when the tweak is the complement of the private key).
    pub fn secp256k1_ec_pubkey_tweak_add(
        ctx: *const Context,
        public_key: *mut PublicKey,
        tweak: *const c_uchar,
    ) -> c_int;

    /// Tweak a private key by adding tweak times the generator to it.
    ///
    /// # Argument
    ///
    /// * `ctx` - A pointer to a context object initialized for validation (cannot
    /// be NULL).
    ///
    /// # In/Out
    ///
    /// * `private_key` - A pointer to a 32-byte private key.
    ///
    /// # In
    ///
    /// * `tweak` - A pointer to a 32-byte tweak.
    ///
    /// # Returns
    ///
    /// * `1` - If successful.
    /// * `0` - if the tweak was out of range (chance of around 1 in 2^128 for
    /// uniformly random 32-byte arrays, or if the resulting private key would be
    /// invalid (only when the tweak is the complement of the private key).
    pub fn secp256k1_ec_privkey_tweak_mul(
        ctx: *const Context,
        private_key: *mut c_uchar,
        tweak: *const c_uchar,
    ) -> c_int;

    /// Tweak a public key by multiplying it by a tweak value.
    ///
    /// # Argument
    ///
    /// * `ctx` - A pointer to a context object (cannot be NULL).
    ///
    /// # In/Out
    ///
    /// * `public_key` - A pointer to a public key object.
    ///
    /// # In
    ///
    /// * `tweak` - A pointer to a 32-byte tweak.
    ///
    /// # Returns
    ///
    /// * `1` - If successful.
    /// * `0` - If the tweak was out of range (chance of around 1 in 2^128 for
    /// uniformly random 32-byte arrays, or equal to zero. 1 otherwise.
    pub fn secp256k1_ec_pubkey_tweak_mul(
        ctx: *const Context,
        public_key: *mut PublicKey,
        tweak: *const c_uchar,
    ) -> c_int;

    /// Add a number of public keys together.
    ///
    /// # Argument
    ///
    /// * `ctx` - A pointer to a context object.
    ///
    /// # Out
    ///
    /// * `out` - A pointer to a public key object for placing the resulting public
    /// key (cannot be NULL).
    ///
    /// # In
    ///
    /// * `ins` - A pointer to array of pointers to public keys (cannot be NULL).
    /// * `n` - The number of public keys to add together (must be at least 1).
    ///
    /// # Returns
    ///
    /// * `1` - The sum of the public keys is valid.
    /// * `0` - The sum of the public keys is not valid.
    pub fn secp256k1_ec_pubkey_combine(
        ctx: *const Context,
        out: *mut PublicKey,
        ins: *const *const PublicKey,
        n: size_t,
    ) -> c_int;
}
