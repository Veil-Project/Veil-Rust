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

use crate::util::MulOverflow;
use crate::Result;
use std::convert::TryFrom;
use std::ops::{Div, Rem};
use std::u32;
use std::u64;

#[derive(Debug, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct u256(pub [u64; 4]);

impl u256 {
    pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let iter = self.0.iter().zip(rhs.0.iter());

        let mut arr = [0u64; 4];
        let mut carry = 0u64;
        let mut overflow = false;
        for (i, (word_l, word_r)) in iter.enumerate() {
            let (n, o) = (word_l + carry).overflowing_add(*word_r);
            overflow = o;
            arr[i] = n;

            if i == 3 && overflow {
                arr = [n, 0u64, 0u64, 0u64];
            } else if overflow {
                carry = 1;
            } else {
                carry = 0;
            }
        }

        (Self(arr), overflow)
    }

    pub fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let iter = self.0.iter().zip(rhs.0.iter());

        let mut arr = [0u64; 4];
        let mut sub = 0u64;
        let mut overflow = false;
        for (i, (word_l, word_r)) in iter.enumerate() {
            let (n, o) = (word_l - sub).overflowing_sub(*word_r);
            overflow = o;
            arr[i] = n;

            if i == 3 && overflow {
                arr = [n, 0u64, 0u64, 0u64];
            } else if overflow {
                sub = 1;
            } else {
                sub = 0;
            }
        }

        (Self(arr), overflow)
    }

    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let mut ret_arr = [0u64; 8];
        let mut arrs = [[0u64; 8]; 4];

        let mut carry_outer = 0u64;
        let mut overflow_outer = false;
        for (i, a) in self.0.iter().enumerate() {
            let mut carry_inner = 0u64;
            let mut overflow_inner = false;
            for (j, b) in rhs.0.iter().enumerate() {
                // Multiply a by b.
                let (high, low, o) = a.mul_with_overflow(*b);
                overflow_inner = o;

                // Add the carry from previous column, if any.
                let low = low + carry_inner;

                // Place returned result to the "i + j" position.
                arrs[j][i + j] = low;

                // If the result overflowed, carry it over.
                if overflow_inner {
                    carry_inner = high;
                } else {
                    carry_inner = 0;
                }
            }
        }

        for (i, arr) in arrs.iter().enumerate() {
            let mut carry = 0;
            for (j, word) in arr.iter().enumerate() {
                let (n, o) = (carry + ret_arr[j]).overflowing_add(*word);
                if o {
                    carry = 1;
                } else {
                    carry = 0;
                }
                ret_arr[j] = n;
            }
        }

        let (low_slice, high_slice) = ret_arr.split_at(4);
        let mut low = [0u64; 4];
        let mut high = [0u64; 4];
        low.copy_from_slice(low_slice);
        high.copy_from_slice(high_slice);
        (Self(low), Self(high) != Self::min_value())
    }

    pub fn min_value() -> Self {
        Self([0u64; 4])
    }

    pub fn max_value() -> Self {
        Self([0xFFFFFFFF_FFFFFFFF; 4])
    }

    pub fn is_zero(&self) -> bool {
        self.0 == [0u64, 0u64, 0u64, 0u64]
    }

    pub fn from_u32(digit: u32) -> Self {
        Self::from(digit)
    }
}

impl From<u8> for u256 {
    fn from(digit: u8) -> u256 {
        From::from(digit as u64)
    }
}

impl From<u16> for u256 {
    fn from(digit: u16) -> u256 {
        From::from(digit as u64)
    }
}

impl From<u32> for u256 {
    fn from(digit: u32) -> u256 {
        From::from(digit as u64)
    }
}

impl From<u64> for u256 {
    fn from(digit: u64) -> u256 {
        let mut ret = [0u64; 4];
        ret[0] = digit;
        Self(ret)
    }
}

impl From<[u64; 4]> for u256 {
    fn from(arr: [u64; 4]) -> u256 {
        Self(arr)
    }
}

//impl From<[u8; 32]> for u256 {
//    fn from(bytes: [u8; 32]) -> Self {
//        Self(bytes)
//    }
//}
//
//impl From<&[u8]> for u256 {
//    fn from(bytes: &[u8]) -> Self {
//        Self(bytes[..32])
//    }
//}

#[cfg(test)]
mod tests {
    use crate::u256::u256;

    #[test]
    fn overflowing_arith() {
        let mut num1 = u256::from([0xFFFFFFFF_FFFFFFFF, 0x0, 0x0, 0x0]);
        let mut num2 = u256::from([0x1, 0x0, 0x0, 0x0]);
        let mut ans = num1.overflowing_add(num2);
        println!("{:?}", ans);

        num1 = u256::from([0x0, 0x1, 0x0, 0x0]);
        num2 = u256::from([0xFFFFFFFF_FFFFFFFF, 0x0, 0x0, 0x0]);
        ans = num1.overflowing_sub(num2);

        println!("{:?}", ans);

        num1 = u256::from([
            0xFFFFFFFF_FFFFFFFF,
            0xFFFFFFFF_FFFFFFFF,
            0xFFFFFFFF_FFFFFFFF,
            0xFFFFFFFF_FFFFFFFF,
        ]);
        num2 = u256::from([
            0xFFFFFFFF_FFFFFFFF,
            0xFFFFFFFF_FFFFFFFF,
            0xFFFFFFFF_FFFFFFFF,
            0xFFFFFFFF_FFFFFFFF,
        ]);
        num1.overflowing_mul(num2);
    }
}
