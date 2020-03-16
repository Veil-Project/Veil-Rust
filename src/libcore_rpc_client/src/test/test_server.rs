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

use crate::{Client, Request, Response, Result, RpcStream};
use mimir_net::{http1_1, json_rpc};
use serde_json;
use std::{
    collections::HashMap,
    convert::TryInto,
    io::{BufRead, BufReader, BufWriter, Read, Write},
    str::FromStr,
};

use std::fs;

#[derive(Clone)]
pub struct TestStream {
    buf: Vec<u8>,
    // json: HashMap<String, String>,
}

impl TestStream {
    pub fn new() -> Self {
        // let hm = HashMap::new();

        Self { buf: Vec::new() }
    }

    fn receive(&mut self, buf: &[u8]) -> Result<json_rpc::Request> {
        let lines = buf.lines();
        let mut iter = lines.map(|l| l.unwrap());
        let mut req_builder = http1_1::Request::builder();

        match iter.next() {
            Some(l) => {
                let mut iter_http = l.split_whitespace();
                req_builder = req_builder
                    .method(iter_http.next().unwrap())
                    .resource(iter_http.next().unwrap())
                    .version(iter_http.next().unwrap());
            }
            None => println!("No response"), // err (and this works!)
        }

        let mut header_end = false;
        let mut body = String::new();
        let mut headers = HashMap::new();
        for line in iter {
            if header_end {
                body = line;
                break;
            }
            if line.is_empty() {
                header_end = true;
            } else {
                let parts: Vec<&str> = line.splitn(2, ": ").collect();

                if parts.len() != 2 {
                    println!("http1_1 header error");
                } else {
                    headers.insert(parts[0].to_owned(), parts[1].to_owned());
                }
            }
        }

        let req = req_builder.headers(headers).body(body).build();
        let json: json_rpc::Request = serde_json::from_str(req.body().unwrap())?;
        Ok(json)
    }

    // fn send(&mut self, req: json_rpc::Request) -> Result<()> {
    //     // TODO: All JSON to a file as key_value seperated by delimiter.
    //     match req.method() {
    //        "getbestblockhash" =>
    //     }
    //     Ok(())
    // }
}

impl Read for TestStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let len = buf.len();
        let mut writer = BufWriter::new(buf);
        writer.write_all(&self.buf)?;
        Ok(len)
    }
}

impl Write for TestStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        let mut len = 0;
        let mut in_buf = vec![0u8];
        for byte in buf {
            in_buf.push(*byte);
            len += 1;
        }
        self.receive(&in_buf).unwrap();
        Ok(len)
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        // self.buf = [0; 100_000];
        Ok(())
    }
}

impl RpcStream for TestStream {}
