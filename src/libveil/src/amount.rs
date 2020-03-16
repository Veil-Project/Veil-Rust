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

/// Amount in satoshis, can be negative.
pub type Amount = i64;

/// Constant full coin value.
pub const COIN: Amount = 100_000_000;
/// Constant decimal cent value.
pub const CENT: Amount = 1_000_000;

/// The absolute max amount of coin that can exist.
///
/// While this is the max amount of coin that can exist, this is not the total money supply
/// itself. This is simply for a sanity check. As this sanity check is used by consensus-critical
/// validation code, the exact value of the MAX_MONEY constant is consensus critical; in unusual
/// circumstances like a overflow bug. If the case, and a large amount of coins were created out
/// of thin air, this allows for some buffer to deal with the problem instead of leading to a
/// protocol fork at time of overflow of max money.
pub const MAX_MONEY: Amount = 300_000_000 * COIN;

pub fn money_range(value: &Amount) -> bool {
    (value >= &0 && value <= &MAX_MONEY)
}
