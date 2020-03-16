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

use crate::http2::Error;
use crate::http2::Result;

use std::ops;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum ErrorKind {
    /// Unknown number index provided for lookup on the static table.
    StaticTableUnknownIndex,
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &str {
        use ErrorKind::*;

        match *self {
            StaticTableUnknownIndex => "index provided is unknown",
        }
    }
}

static STATIC_TABLE: [(&str, Option<&str>); 62] = [
    // This first entry added simply as a dummy. Not part of HPACK protocol.
    ("", None),
    (":authority", None),
    (":method", Some("GET")),
    (":method", Some("POST")),
    (":path", Some("/")),
    (":path", Some("/index.html")),
    (":scheme", Some("http")),
    (":scheme", Some("https")),
    (":status", Some("200")),
    (":status", Some("204")),
    (":status", Some("206")),
    (":status", Some("304")),
    (":status", Some("400")),
    (":status", Some("404")),
    (":status", Some("500")),
    ("accept-charset", None),
    ("accept-encoding", Some("gzip, deflate")),
    ("accept-language", None),
    ("accept-ranges", None),
    ("accept", None),
    ("access-control-allow-origin", None),
    ("age", None),
    ("allow", None),
    ("authorization", None),
    ("cache-control", None),
    ("content-disposition", None),
    ("content-encoding", None),
    ("content-language", None),
    ("content-length", None),
    ("content-location", None),
    ("content-range", None),
    ("content-type", None),
    ("cooke", None),
    ("date", None),
    ("etag", None),
    ("expect", None),
    ("expires", None),
    ("from", None),
    ("host", None),
    ("if-match", None),
    ("if-modified-since", None),
    ("if-none-match", None),
    ("if-range", None),
    ("if-unmodified-since", None),
    ("last-modified", None),
    ("link", None),
    ("location", None),
    ("max-forwards", None),
    ("proxy-authenticate", None),
    ("proxy-authorization", None),
    ("range", None),
    ("referer", None),
    ("refresh", None),
    ("retry-after", None),
    ("server", None),
    ("set-cookie", None),
    ("strict-transport-security", None),
    ("transfer-encoding", None),
    ("user-agent", None),
    ("vary", None),
    ("via", None),
    ("www-authenticate", None),
];

type HeaderName = String;
type HeaderValue = String;
type HeaderEntry = (HeaderName, Option<HeaderValue>);

struct DynamicTable {
    inner: Vec<HeaderEntry>,
    /// The absolute max size a table can be, set during setting negotiation.
    max_size: usize,
}

impl DynamicTable {
    fn new(max_size: usize) -> Self {
        Self {
            inner: Vec::new(),
            max_size,
        }
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    /// The size of the table is the sum of the size of all it's entries. This is done before any
    /// Huffman encoding is applied.
    fn size(&self) -> usize {
        // The size of an entry is the sum of its name's length in octets, its value's length in
        // octets, and 32.
        let mut size = 0usize;
        for indexed_entry in self.inner {
            let (name, value): (HeaderName, Option<HeaderValue>) = indexed_entry;
            // Estimated overhead associated with an entry in case it is 64 bit.
            let overhead = 32usize;
            let value_len = value.unwrap_or_default().len();
            size += name.len() + value_len + overhead;
        }
        size
    }

    /// Resize the `DynamicTable` in bytes.
    ///
    /// If the new max size is smaller than the current max size and if the current size is
    /// bigger than the new max size, entries will be evicted from the table until there are as
    /// much or less than the new max size.
    fn resize(&mut self, new_max: usize) {
        while self.size() > new_max {
            self.inner.pop();
        }
        assert!(
            self.size() < new_max,
            "DynamicTable::resize size of the table isn't smaller than the new size."
        );
        self.max_size = new_max;
    }
}

impl ops::Index<usize> for DynamicTable {
    type Output = (String, Option<String>);

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index > 61);
        &self.inner[index - 61]
    }
}

pub struct Table {
    dynamic_table: DynamicTable,
}

impl Table {
    pub fn new(max_size: usize) -> Self {
        Self {
            dynamic_table: DynamicTable {
                inner: Vec::new(),
                max_size,
            },
        }
    }

    pub fn resize_dynamic_table(&mut self, new_max: usize) {
        self.dynamic_table.resize(new_max)
    }

    pub fn dynamic_table_size(&self) -> usize {
        self.dynamic_table.size()
    }

    pub fn dynamic_table_len(&self) -> usize {
        self.dynamic_table.len()
    }

    pub fn len(&self) -> usize {
        self.dynamic_table.len() + STATIC_TABLE.len()
    }
}

impl ops::Index<usize> for Table {
    type Output = (HeaderName, Option<HeaderValue>);

    fn index(&self, index: usize) -> &Self::Output {
        // Kind of hate this, but panic catch to decode error or do alternative...?
        if index == 0 {
            panic!("index of 0 for HPACK out of range, starts from 1")
        }
        if index < 61 {
            let (k, v) = STATIC_TABLE[index];
            &(k.to_string(), v.map(|v| v.to_string()))
        } else {
            &self.dynamic_table[index]
        }
    }
}

pub struct Hpack {
    /// Starts at index 62, following the static table.
    table: Table,
}

//impl Hpack {
//    /// Creates a new HPACK.
//    ///
//    /// It takes in an initial max dynamic table size which is from SETTINGS_HEADER_TABLE_SIZE
//    /// setting. This number can be updated by doing a dynamic table size update which must occur
//    /// at the beginning of the first header block following the change to the dynamic table size.
//    ///
//    /// It is possible to set this as 0 which will clear this dynamic table entirely then
//    /// restored to another value.
//    pub fn new(max_dynamic_table_size: usize) -> Self {
//        Self {
//            dynamic_table: DynamicTable::new(max_dynamic_table_size),
//        }
//    }
//}
