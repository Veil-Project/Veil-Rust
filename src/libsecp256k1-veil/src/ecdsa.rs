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

use crate::context::*;
use crate::error::{Error, ErrorKind};
use crate::keys::{PrivateKey, PublicKey};
use crate::message::Message;
use crate::signature::Signature;
use crate::Result;
use secp256k1_veil_sys;

#[derive(Debug)]
pub struct Secp256k1<C: Context> {
    ctx: C,
}

impl<C: Context> Secp256k1<C> {
    pub fn gen_context() -> Secp256k1<C> {
        Self { ctx: C::new() }
    }

    pub fn ctx(&self) -> &C {
        &self.ctx
    }
}

impl Secp256k1<ContextNone> {
    pub fn new() -> Secp256k1<ContextNone> {
        Self::gen_context()
    }

    pub fn with_context(ctx: ContextNone) -> Secp256k1<ContextNone> {
        Self { ctx }
    }
}

impl Secp256k1<ContextSign> {
    pub fn new_sign() -> Secp256k1<ContextSign> {
        Self::gen_context()
    }

    pub fn with_sign_context(ctx: ContextSign) -> Secp256k1<ContextSign> {
        Self { ctx }
    }

    pub fn sign(&self, msg: &Message, private_key: &PrivateKey) -> Result<Signature> {
        Ok(Signature::new(unsafe {
            secp256k1_veil_sys::Secp256k1::sign(
                self.ctx.as_ptr(),
                msg.as_ptr(),
                private_key.as_ptr(),
            )?
        }))
    }
}

impl Secp256k1<ContextVerify> {
    pub fn new_verify() -> Secp256k1<ContextVerify> {
        Self::gen_context()
    }

    pub fn with_verify(ctx: ContextVerify) -> Secp256k1<ContextVerify> {
        Self { ctx }
    }

    pub fn verify(&self, sig: &Signature, msg: &Message, public_key: &PublicKey) -> Result<()> {
        unsafe {
            secp256k1_veil_sys::Secp256k1::verify(
                self.ctx.as_ptr(),
                sig.as_ptr(),
                msg.as_ptr(),
                public_key.as_ptr(),
            )?;
            Ok(())
        }
    }
}

impl<C: Context> Drop for Secp256k1<C> {
    fn drop(&mut self) {
        self.ctx.destroy()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "rand_os")]
    #[test]
    fn test_context() {
        let mut ctx = ContextNone::new();
        let mut ctx_sign = ContextSign::new();
        ctx_sign.randomize();
        let private_key = PrivateKey::new(&ctx);
        let public_key = PublicKey::new(&ctx_sign, &private_key);

        let msg = [1; 32];
        let message = Message::new(&msg);

        let secp_sign = Secp256k1::new_sign();
        let secp_verify = Secp256k1::new_verify();
        let sig = secp_sign.sign(&message, &private_key);

        secp_verify
            .verify(&sig.unwrap(), &message, &public_key.unwrap())
            .unwrap();
    }
}
