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

use std::collections;

pub const FLAGS_END_STREAM: u8 = 1 << 0;

pub const FLAGS_END_HEADERS: u8 = 1 << 2;

pub const FLAGS_PADDED: u8 = 1 << 3;

pub const FLAGS_PRIORITY: u8 = 1 << 5;

pub const FLAGS_ALL: u8 = FLAGS_END_STREAM | FLAGS_END_HEADERS | FLAGS_PADDED | FLAGS_PRIORITY;

pub struct Flags(u8);

impl Flags {
    pub fn new() -> Self {
        Self(0u8)
    }

    pub fn set_end_stream(&mut self) {
        self.0 |= FLAGS_END_STREAM;
    }

    pub fn set_end_headers(&mut self) {
        self.0 |= FLAGS_END_HEADERS;
    }

    pub fn set_padded(&mut self) {
        self.0 |= FLAGS_PADDED;
    }

    pub fn set_priority(&mut self) {
        self.0 |= FLAGS_PRIORITY;
    }

    pub fn set_all(&mut self) {
        self.0 |= FLAGS_ALL;
    }

    pub fn is_end_stream(&self) -> bool {
        self.0 & FLAGS_END_STREAM == FLAGS_END_STREAM
    }

    pub fn is_end_headers(&self) -> bool {
        self.0 & FLAGS_END_HEADERS == FLAGS_END_HEADERS
    }

    pub fn is_padded(&self) -> bool {
        self.0 & FLAGS_PADDED == FLAGS_PADDED
    }

    pub fn is_priority(&self) -> bool {
        self.0 & FLAGS_PRIORITY == FLAGS_PRIORITY
    }

    pub fn is_all(&self) -> bool {
        self.0 & FLAGS_ALL == FLAGS_ALL
    }

    pub fn into_inner(self) -> u8 {
        self.0
    }

    pub fn empty(&mut self) {
        self.0 = 0;
    }
}

pub struct HeaderBlock {
    //    fields: collections::HashMap,
}

pub struct Headers {
    stream_id: u32,
    flags: Flags,
    pad_length: Option<u8>,
    stream_dependency: Option<u32>,
    weight: Option<u8>,
    //    header_block:
}
