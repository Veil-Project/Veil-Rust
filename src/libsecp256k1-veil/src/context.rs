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

#[cfg(all(feature = "rand_os", any(feature = "std", test)))]
use rand::{rngs::OsRng, Rng};
use secp256k1_veil_sys::{self, bindings, c_types::*};

pub trait AsNative<T> {
    unsafe fn as_native_ref(&self) -> &T;
    unsafe fn as_native_mut(&mut self) -> &mut T;
}

pub trait Context {
    const FLAG: c_uint;

    fn new() -> Self;

    fn clone(&mut self) -> Self;

    fn destroy(&mut self);

    fn as_ptr(&self) -> *const secp256k1_veil_sys::Context;

    fn as_mut_ptr(&mut self) -> *mut secp256k1_veil_sys::Context;
}

#[cfg(all(feature = "rand_os", any(feature = "std", test)))]
pub trait ContextRandom {
    fn randomize(&mut self);

    fn randomize_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R);
}

macro_rules! impl_context {
    ($impl_name:ident, $flag:expr) => {
        #[derive(Debug)]
        pub struct $impl_name(*mut secp256k1_veil_sys::Context);

        impl Context for $impl_name {
            const FLAG: c_uint = $flag;

            fn new() -> Self {
                Self(unsafe { secp256k1_veil_sys::Context::new(Self::FLAG) })
            }

            fn clone(&mut self) -> Self {
                let ctx = unsafe { self.as_native_mut() };

                Self(unsafe { ctx.clone_data() })
            }

            fn destroy(&mut self) {
                let ctx = unsafe { self.as_native_mut() };
                unsafe { ctx.destroy() };
            }

            fn as_ptr(&self) -> *const secp256k1_veil_sys::Context {
                self.0 as *const secp256k1_veil_sys::Context
            }

            fn as_mut_ptr(&mut self) -> *mut secp256k1_veil_sys::Context {
                self.0 as *mut secp256k1_veil_sys::Context
            }
        }

        impl AsNative<secp256k1_veil_sys::Context> for $impl_name {
            unsafe fn as_native_ref(&self) -> &secp256k1_veil_sys::Context {
                &*self.0
            }

            unsafe fn as_native_mut(&mut self) -> &mut secp256k1_veil_sys::Context {
                &mut *self.0
            }
        }

        unsafe impl Send for $impl_name {}
        unsafe impl Sync for $impl_name {}
    };
}

impl_context!(ContextNone, bindings::CONTEXT_NONE);
impl_context!(ContextSign, bindings::CONTEXT_SIGN);
impl_context!(ContextVerify, bindings::CONTEXT_VERIFY);

#[cfg(all(feature = "rand_os", any(feature = "std", test)))]
impl ContextRandom for ContextVerify {
    fn randomize(&mut self) {
        let mut rng = OsRng;
        self.randomize_with_rng(&mut rng);
    }

    #[cfg(all(feature = "rand_os", any(feature = "std", test)))]
    fn randomize_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let mut seed: [c_uchar; 32] = rng.gen();
        unsafe { bindings::secp256k1_context_randomize(self.0, seed.as_mut_ptr()) };
    }
}

#[cfg(all(feature = "rand_os", any(feature = "std", test)))]
impl ContextRandom for ContextSign {
    fn randomize(&mut self) {
        let mut rng = OsRng;
        self.randomize_with_rng(&mut rng);
    }

    #[cfg(all(feature = "rand_os", any(feature = "std", test)))]
    fn randomize_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let mut seed: [c_uchar; 32] = rng.gen();
        unsafe { bindings::secp256k1_context_randomize(self.0, seed.as_mut_ptr()) };
    }
}
