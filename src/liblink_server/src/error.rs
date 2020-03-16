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

use quiche;
use rocksdb;
use serde_json;
use std::{error, fmt, io, net, num, sync::mpsc};
use veil_core_rpc;

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
    ParseInt(num::ParseIntError),
    ParseFloat(num::ParseFloatError),
    AddrParse(net::AddrParseError),
    RocksDb(rocksdb::Error),
    SerdeJson(serde_json::Error),
    VeilCoreRpc(veil_core_rpc::Error),
    MpscRecvError(mpsc::RecvError),
    MpscSendError(mpsc::SendError<_>),
    Quic(quiche::Error),
}

impl From<&ErrorKind> for i32 {
    fn from(e: &ErrorKind) -> Self {
        match e {
            ErrorKind::Io(_) => 1,
            ErrorKind::ParseInt(_) => 2,
            ErrorKind::ParseFloat(_) => 3,
            ErrorKind::AddrParse(_) => 4,
            ErrorKind::RocksDb(_) => 5,
            ErrorKind::SerdeJson(_) => 6,
            ErrorKind::VeilCoreRpc(_) => 7,
            ErrorKind::MpscRecvError(_) => 8,
            ErrorKind::MpscSendError(_) => 9,
            ErrorKind::Quic(_) => 10,
        }
    }
}

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error(Box::new(kind))
    }

    pub fn source(&self) -> &ErrorKind {
        &self.0
    }

    pub fn exit_code(&self) -> i32 {
        i32::from(self.0.as_ref())
    }

    pub fn cause(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Io(ref e) => e.fmt(f),
            ErrorKind::ParseInt(ref e) => e.fmt(f),
            ErrorKind::ParseFloat(ref e) => e.fmt(f),
            ErrorKind::AddrParse(ref e) => e.fmt(f),
            ErrorKind::RocksDb(ref e) => e.fmt(f),
            ErrorKind::VeilCoreRpc(ref e) => e.fmt(f),
            ErrorKind::SerdeJson(ref e) => e.fmt(f),
            ErrorKind::MpscRecvError(ref e) => e.fmt(f),
            ErrorKind::MpscSendError(ref e) => e.fmt(f),
            ErrorKind::Quic(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self.0 {
            ErrorKind::Io(ref e) => Some(e),
            ErrorKind::ParseInt(ref e) => Some(e),
            ErrorKind::ParseFloat(ref e) => Some(e),
            ErrorKind::AddrParse(ref e) => Some(e),
            ErrorKind::RocksDb(ref e) => Some(e),
            ErrorKind::VeilCoreRpc(ref e) => Some(e),
            ErrorKind::SerdeJson(ref e) => Some(e),
            ErrorKind::MpscRecvError(ref e) => Some(e),
            ErrorKind::MpscSendError(ref e) => Some(e),
            ErrorKind::Quic(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::new(ErrorKind::Io(e))
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Error::new(ErrorKind::ParseInt(e))
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(e: num::ParseFloatError) -> Self {
        Error::new(ErrorKind::ParseFloat(e))
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(e: net::AddrParseError) -> Self {
        Error::new(ErrorKind::AddrParse(e))
    }
}

impl From<rocksdb::Error> for Error {
    fn from(e: rocksdb::Error) -> Self {
        Error::new(ErrorKind::RocksDb(e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::new(ErrorKind::SerdeJson(e))
    }
}

impl From<veil_core_rpc::Error> for Error {
    fn from(e: veil_core_rpc::Error) -> Self {
        Error::new(ErrorKind::VeilCoreRpc(e))
    }
}

impl From<mpsc::RecvError> for Error {
    fn from(e: mpsc::RecvError) -> Self {
        Error::new(ErrorKind::MpscRecvError(e))
    }
}

impl<T> From<mpsc::SendError<T>> for Error {
    fn from(e: mpsc::SendError<T>) -> Self {
        Error::new(ErrorKind::MpscSendError(e))
    }
}

impl From<quiche::Error> for Error {
    fn from(e: quiche::Error) -> Self {
        Error::new(ErrorKind::Quic(e))
    }
}
