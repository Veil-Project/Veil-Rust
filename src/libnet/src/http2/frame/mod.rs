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

/*
TODO:

3. Starting HTTP/2
3.2
- "h2c" or HTTP/2 Clear text implementation only
- HTTP/2 first frame MUST be a server connection preface consisting of a SETTINGS frame. Upon
receiving the 101 response, the client MUST send a connection preface, which includes a SETTINGS
frame.
- The HTTP/1.1 request that is sent prior to the upgrade is assigned a stream identifier of 1
with default priority values. Stream 1 is implicitly "half-closed" from the client toward the
server, since the request is completed as an HTTP/1.1 request. After commencing the HTTP/2
connection, stream 1 is used for the response.
- 3.3?
- 3.4?
- 3.5 HTTP/2 connection preface

*/

//mod continuation;
//mod data;
//mod goaway;
mod headers;
//mod ping;
//mod priority;
//mod push_promise;
//mod rst_stream;
mod settings;
//mod window_update;

pub use super::error::Error;
pub use settings::Settings;

enum Kind {
    Data = 0,
    Headers = 1,
    Priority = 2,
    Reset = 3,
    Settings = 4,
    PushPromise = 5,
    Ping = 6,
    GoAway = 7,
    WindowUpdate = 8,
    Continuation = 9,
}

pub const 

pub enum Type {
    Settings(Settings),
}

//impl Frame<Settings> {
//    pub fn identifier(&self) -> u32 {
//        self.inner.identifier()
//    }
//    // methods specific to the frame Settings
//}
