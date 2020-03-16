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

use std::u64;

pub trait MulOverflow {
    fn mul_with_overflow(&self, rhs: Self) -> (Self, Self, bool)
    where
        Self: Sized;
}

//fn mul_with_overflow(l: u64, r: u64) -> (u64, u64) {
//    let l_lo = l & 0xFFFF_FFFF;
//    let l_hi = l >> 32;
//
//    let r_lo = r & 0xFFFF_FFFF;
//    let r_hi = r >> 32;
//
//    let mul_lo = l_lo * r_lo;
//    let mul_hi = (l_hi * r_lo) + (mul_lo >> 32);
//    let mul_carry = (l_lo * r_hi) + (mul_hi & 0xFFFF_FFFF);
//
//    let result_hi = (l_hi * r_hi) + (mul_hi >> 32) + (mul_carry >> 32);
//    let result_lo = (mul_carry << 32) + (mul_lo & 0xFFFF_FFFF);
//
//    (result_hi, result_lo)
//}

impl MulOverflow for u64 {
    fn mul_with_overflow(&self, rhs: Self) -> (Self, Self, bool) {
        let lhs_lo: u64 = self & 0xFFFF_FFFF;
        let lhs_hi: u64 = self >> 32;

        let rhs_lo: u64 = rhs & 0xFFFF_FFFF;
        let rhs_hi: u64 = rhs >> 32;

        let mul_lo: u64 = lhs_lo * rhs_lo;
        let mul_hi: u64 = (lhs_hi * rhs_lo) + (mul_lo >> 32);
        let mul_carry: u64 = (lhs_lo * rhs_hi) + (mul_hi & 0xFFFF_FFFF);

        let result_hi: u64 = (lhs_hi * rhs_hi) + (mul_hi >> 32) + (mul_carry >> 32);
        let result_lo: u64 = (mul_carry << 32) + (mul_lo & 0xFFFF_FFFF);

        let overflowed = result_hi > 0;

        (result_hi, result_lo, overflowed)
    }
}
