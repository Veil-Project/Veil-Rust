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

use core::fmt;
#[cfg(any(feature = "std", test))]
use std::error;

#[derive(Debug)]
pub enum ErrorKind {
    RandomizeContext,
    CombinePublicKey,
    SerializeCompressedPublicKey,
    SerializeUncompressedPublicKey,
    CreatePublicKey,
    VerifyPrivateKey,
    ParseKey,
    TweakKey,
    ParseSignature,
    SerializeSignature,
    NormalizeSignature,
    VerifySignature,
    SignMessage,
    Ecdh,
    ParseGenerator,
    BadSeed,
    PrepareMlsag,
    KeyImageMlsag,
}

#[derive(Debug)]
pub struct Error(ErrorKind);

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self(kind)
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ErrorKind::*;
        match self.0 {
            RandomizeContext => write!(f, "failed to randomize, possibly fully initialized context object not for use with signing or verification"),
            CombinePublicKey => write!(f, "sum of the public keys is not valid"),
            SerializeCompressedPublicKey => write!(f, "output length is not 33 bytes"),
            SerializeUncompressedPublicKey => write!(f, "output length is not 65 bytes"),
            CreatePublicKey => write!(f, "can not create public key from supplied private key"),
            VerifyPrivateKey => write!(f, "private key is not valid"),
            ParseKey => write!(f, "key format undefined or can not be parsed"),
            TweakKey => write!(f, "tweak out of range (chance of around 1 in 2^128 for uniformly random 32-byte arrays, or equal to zero"),
            ParseSignature => write!(f, "signature undefined or can not be parse"),
            SerializeSignature => write!(f, "not enough space was available to serialize"),
            NormalizeSignature => write!(f, "signature is already normalized"),
            VerifySignature => write!(f, "the signature is incorrect or not parsable"),
            SignMessage => write!(f, "nonce generation function failed or the private key was invalid"),
            // Ecdh
            Ecdh => write!(f, "scalar was invalid (zero or overflow)"),
            // Generator
            ParseGenerator => write!(f, "input is an invalid signature"),
            BadSeed => write!(f, "seed supplied it not acceptable"),
            // Mlsag
            PrepareMlsag => write!(f, "could not prepare MLSAG, wrong size matrix or inputs"),
            KeyImageMlsag => write!(f, "key image error"),
        }
    }
}

#[cfg(any(feature = "std", test))]
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use ErrorKind::*;
        match self.0 {
            RandomizeContext => None,
            CombinePublicKey => None,
            SerializeCompressedPublicKey => None,
            SerializeUncompressedPublicKey => None,
            CreatePublicKey => None,
            VerifyPrivateKey => None,
            ParseKey => None,
            TweakKey => None,
            ParseSignature => None,
            SerializeSignature => None,
            NormalizeSignature => None,
            VerifySignature => None,
            SignMessage => None,
            Ecdh => None,
            ParseGenerator => None,
            BadSeed => None,
            PrepareMlsag => None,
            KeyImageMlsag => None,
        }
    }
}
