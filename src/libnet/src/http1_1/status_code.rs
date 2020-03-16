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

#[derive(Clone, Debug)]
pub struct StatusCode(pub u16);

impl StatusCode {
    pub fn to_str(&self) -> &str {
        match self.0 {
            100 => "100 Continue",
            101 => "101 Switching Protocols",
            102 => "102 Processing",
            103 => "103 Early Hints",
            200 => "200 OK",
            201 => "201 Created",
            202 => "202 Accepted",
            203 => "203 Non-Authoritative Information",
            204 => "204 No Content",
            205 => "205 Reset Content",
            206 => "206 Partial Content",
            207 => "207 Multi-Status",
            208 => "208 Already Reported",
            226 => "226 IM Used",
            300 => "300 Multiple Choices",
            301 => "301 Moved Permanently",
            302 => "302 Found",
            303 => "303 See Other",
            304 => "304 Not Modified",
            305 => "305 Use Proxy",
            306 => "306 Switch Proxy",
            307 => "307 Temporary Redirect",
            308 => "308 Permanent Redirect",
            400 => "400 Bad Request",
            401 => "401 Unauthorized",
            402 => "402 Payment Required",
            403 => "403 Forbidden",
            404 => "404 Not Found",
            405 => "405 Method Not Allowed",
            406 => "406 Not Acceptable",
            407 => "407 Proxy Authentication Required",
            408 => "408 Request Timeout",
            409 => "409 Conflict",
            410 => "410 Gone",
            411 => "411 Length Required",
            412 => "412 Precondition Failed",
            413 => "413 Payload Too Large",
            414 => "414 URI Too Long",
            415 => "415 Unsupported Media Type",
            416 => "416 Range Not Satisfiable",
            417 => "417 Expectation Failed",
            418 => "418 I'm a teapot",
            420 => "420 Blaze It",
            421 => "421 Misdirected Request",
            422 => "422 Unprocessable Entity",
            423 => "423 Locked",
            424 => "424 Failed Dependency",
            425 => "425 Too Early",
            426 => "426 Upgrade Required",
            428 => "428 Precondition Required",
            429 => "429 Too Many Requests",
            431 => "431 Request Header Fields Too Large",
            451 => "451 Unavailable For Legal Reasons",
            500 => "500 Internal Server Error",
            501 => "501 Not Implemented",
            502 => "502 Bad Gateway",
            503 => "503 Service Unavailable",
            504 => "504 Gateway Timeout",
            505 => "505 HTTP Version Not Supported",
            506 => "506 Variant Also Negotiates",
            507 => "507 Insufficient Storage",
            508 => "508 Loop Detected",
            510 => "510 Not Extended",
            511 => "511 NetworkAuthenticationRequired",
            _ => "999 Unknown",
        }
    }
}

impl ToString for StatusCode {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

macro_rules! from_int {
    ($($int: ident,)*) => {$(
        impl From<$int> for StatusCode {
            fn from(code: $int) -> StatusCode {
                StatusCode(code as u16)
            }
        }
    )*}
}

from_int! {
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,
}

impl AsRef<u16> for StatusCode {
    fn as_ref(&self) -> &u16 {
        &self.0
    }
}

impl PartialEq<u16> for StatusCode {
    fn eq(&self, eq_num: &u16) -> bool {
        &self.0 == eq_num
    }
}

impl PartialEq<StatusCode> for u16 {
    fn eq(&self, eq_num: &StatusCode) -> bool {
        self == &eq_num.0
    }
}
