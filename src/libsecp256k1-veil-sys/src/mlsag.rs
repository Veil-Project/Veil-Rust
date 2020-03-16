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

use crate::bindings::mlsag;
use crate::c_types::*;
use crate::Context;
use crate::{Error, ErrorKind, Result};

// pub fn combine(&self, ctx: &ContextNone, other: &PublicKey) -> Result<Self> {
//     let ins: *const *const secp256k1_veil_sys::PublicKey = [
//         self.0 as *const secp256k1_veil_sys::PublicKey,
//         other.0 as *const secp256k1_veil_sys::PublicKey,
//     ]
//     .as_ptr();

pub struct PublicKeyMatrix {
    components: *mut c_uchar,
    cols: size_t,
    rows: size_t,
}

pub struct Mlsag;

impl Mlsag {
    /// Prepares inputs for MLSAG algo
    ///
    /// Notes:
    /// * It takes in both a public key matrix and a private key vector.
    /// * It adds to the public_key matrix an extra "row" for the sum
    /// * It adds to the private_key vector an extra "row" for the sum
    pub unsafe fn prepare(
        pubkey_m: &PublicKeyMatrix,
        privkey_m: *mut c_uchar,
        pcm_in: *mut *const c_uchar, // &mut [*const c_uchar]
        pcm_out: *mut *const c_uchar,
        blinds: *mut *const c_uchar,
        outs: size_t,
    ) -> Result<()> {
        // TODO: Check if pubkey rows and cols is same size as private key's.
        if mlsag::secp256k1_prepare_mlsag(
            pubkey_m.components,
            privkey_m,
            outs,
            outs,
            pubkey_m.cols,
            pubkey_m.rows,
            pcm_in, // The pederson commitment
            pcm_out,
            blinds,
        ) == 0
        {
            Err(Error::new(ErrorKind::PrepareMlsag))
        } else {
            Ok(())
        }
    }

    pub unsafe fn get_keyimage(
        ctx: *const Context,
        public_key: *const c_uchar,
        private_key: *const c_uchar,
    ) -> Result<[u8; 33]> {
        let key_image = &mut [0u8; 33]; // FIXME: Check actual size
        if mlsag::secp256k1_get_keyimage(ctx, key_image.as_mut_ptr(), public_key, private_key) == 0
        {
            Err(Error::new(ErrorKind::KeyImageMlsag))
        } else {
            Ok(*key_image)
        }
    }

    pub unsafe fn generate(ctx: *const Context) {
        unimplemented!()
    }
}
