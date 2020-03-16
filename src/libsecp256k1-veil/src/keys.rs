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

#[cfg(any(feature = "alloc"))]
use alloc::boxed::Box;
#[cfg(all(feature = "rand_os", any(feature = "std", test)))]
use rand::{rngs::OsRng, Rng};
use secp256k1_veil_sys::c_types::*;
#[cfg(any(feature = "std", test))]
use std::boxed::Box;

use crate::context::{Context, ContextNone, ContextSign, ContextVerify};
use crate::traits::AsNative;
use crate::Result;

#[derive(Debug)]
pub struct PublicKey(*mut secp256k1_veil_sys::PublicKey);

impl PublicKey {
    pub fn new(ctx: &ContextSign, private_key: &PrivateKey) -> Result<Self> {
        Ok(Self(unsafe {
            secp256k1_veil_sys::PublicKey::create(ctx.as_ptr(), private_key.data.as_ptr())?
        }))
    }

    pub fn parse(ctx: &ContextNone, input: &[u8]) -> Result<Self> {
        if input.len() != 33 || input.len() != 65 {
            // TODO: error
        }

        Ok(Self(unsafe {
            secp256k1_veil_sys::PublicKey::parse(ctx.as_ptr(), input.as_ptr(), input.len())?
        }))
    }

    pub fn combine(&self, ctx: &ContextNone, other: &PublicKey) -> Result<Self> {
        let ins: *const *const secp256k1_veil_sys::PublicKey = [
            self.0 as *const secp256k1_veil_sys::PublicKey,
            other.0 as *const secp256k1_veil_sys::PublicKey,
        ]
        .as_ptr();

        Ok(Self(unsafe {
            secp256k1_veil_sys::PublicKey::combine(ctx.as_ptr(), ins, 2 as size_t)?
        }))
    }

    pub fn serialize_compressed(&self, ctx: &ContextNone) -> Result<[u8; 33]> {
        unsafe {
            let public_key = self.as_native_ref();
            Ok(public_key.serialize_compressed(ctx.as_ptr())?)
        }
    }

    pub fn serialize_uncompressed(&self, ctx: &ContextNone) -> Result<[u8; 65]> {
        unsafe {
            let public_key = self.as_native_ref();
            Ok(public_key.serialize_uncompressed(ctx.as_ptr())?)
        }
    }

    pub fn neg(&mut self, ctx: &ContextNone) -> Result<()> {
        let public_key = unsafe { self.as_native_mut() };
        unsafe { public_key.negate(ctx.as_ptr())? };
        Ok(())
    }

    pub fn add_assign(&mut self, ctx: &ContextVerify, other: [u8; 32]) -> Result<()> {
        let public_key = unsafe { self.as_native_mut() };
        unsafe { public_key.tweak_add(ctx.as_ptr(), other.as_ptr())? };
        Ok(())
    }

    pub fn mul_assign(&mut self, ctx: &ContextVerify, other: [u8; 32]) -> Result<()> {
        let public_key = unsafe { self.as_native_mut() };
        unsafe { public_key.tweak_mul(ctx.as_ptr(), other.as_ptr())? };
        Ok(())
    }

    pub fn as_ptr(&self) -> *const secp256k1_veil_sys::PublicKey {
        self.0 as *const secp256k1_veil_sys::PublicKey
    }

    pub fn as_mut_ptr(&mut self) -> *mut secp256k1_veil_sys::PublicKey {
        self.0
    }

    pub fn as_slice(&self) -> [u8; 64] {
        let s = unsafe { self.as_native_ref() };
        s.0
    }
}

impl AsNative<secp256k1_veil_sys::PublicKey> for PublicKey {
    unsafe fn as_native_ref(&self) -> &secp256k1_veil_sys::PublicKey {
        &*self.0
    }

    unsafe fn as_native_mut(&mut self) -> &mut secp256k1_veil_sys::PublicKey {
        &mut *self.0
    }
}

impl From<PrivateKey> for PublicKey {
    fn from(private_key: PrivateKey) -> Self {
        unimplemented!()
    }
}

unsafe impl Send for PublicKey {}
unsafe impl Sync for PublicKey {}

#[derive(Debug)]
pub struct PrivateKey {
    #[cfg(any(feature = "std", feature = "alloc", test))]
    data: Box<[u8; 32]>,
    #[cfg(all(not(feature = "std"), not(feature = "alloc"), not(test)))]
    data: [u8; 32],
}

impl PrivateKey {
    #[cfg(all(feature = "rand_os", any(feature = "std", feature = "alloc", test)))]
    pub fn new(ctx: &ContextNone) -> Self {
        let mut rng = OsRng;
        Self::new_with_rng(&mut rng, ctx)
    }

    #[cfg(all(feature = "rand_os", any(feature = "std", feature = "alloc", test)))]
    pub fn new_with_rng<R: Rng + ?Sized>(rng: &mut R, ctx: &ContextNone) -> Self {
        let mut secret_key: Box<[c_uchar; 32]> = Box::new(rng.gen());

        unsafe {
            while !secp256k1_veil_sys::PrivateKey::verify(ctx.as_ptr(), secret_key.as_ptr()).is_ok()
            {
                *secret_key = rng.gen();
            }
        }

        Self { data: secret_key }
    }

    pub fn neg(&mut self, ctx: ContextNone) -> Result<()> {
        unsafe {
            secp256k1_veil_sys::PrivateKey::negate(ctx.as_ptr(), self.data.as_mut_ptr())?;
            Ok(())
        }
    }

    pub fn add_assign(&mut self, ctx: ContextVerify, other: [u8; 32]) -> Result<()> {
        unsafe {
            secp256k1_veil_sys::PrivateKey::tweak_add(
                ctx.as_ptr(),
                self.data.as_mut_ptr(),
                other.as_ptr(),
            )?;
            Ok(())
        }
    }

    pub fn mul_assign(&mut self, ctx: ContextVerify, other: [u8; 32]) -> Result<()> {
        unsafe {
            secp256k1_veil_sys::PrivateKey::tweak_mul(
                ctx.as_ptr(),
                self.data.as_mut_ptr(),
                other.as_ptr(),
            )?;
            Ok(())
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }

    #[cfg(all(any(feature = "std", feature = "alloc", test)))]
    pub fn drop_key(&mut self) {
        *self.data = [0; 32];
    }
}

#[cfg(all(any(feature = "std", feature = "alloc", test)))]
impl Drop for PrivateKey {
    fn drop(&mut self) {
        self.drop_key();
    }
}
