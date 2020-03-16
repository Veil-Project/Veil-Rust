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

use std::error;
use std::fmt;
use std::io;
use std::result;

use super::hpack;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub enum ProtocolErrorKind {
    /// The associated condition is not a result of an error. For example, a GOAWAY might include
    /// this code to indicate graceful shutdown of a connection.
    None = 0x0,
    /// The endpoint detected an unspecific protocol error. This error is for use when a more
    /// specific error code is not available.
    Protocol = 0x1,
    /// The endpoint encountered an unexpected internal error.
    Internal = 0x2,
    /// The endpoint detected that its peer violated the flow-control protocol.
    FlowControl = 0x3,
    /// The endpoint sent a SETTINGS frame but did not receive a response in a timely manner.
    SettingsTimeout = 0x4,
    /// The endpoint received a frame after a stream was half-closed.
    StreamClosed = 0x5,
    /// The endpoint received a frame with an invalid size.
    FrameSize = 0x6,
    /// The endpoint refused the stream prior to performing any application processing.
    RefusedStream = 0x7,
    /// Used by the endpoint to indicate that the stream is no longer needed.
    Cancel = 0x8,
    /// The endpoint is unable to maintain the header compression context for the connection.
    Compression = 0x9,
    /// The connection established in response to a CONNECT request was reset or abnormally closed.
    Connect = 0xA,
    /// The endpoint detected that its peer is exhibiting a behavior that might be generating excessive load.
    EnhanceYourCalm = 0xB,
    /// The underlying transport has properties that do not meet minimum security requirements.
    InadequateSecurity = 0xC,
    /// The endpoint requires that HTTP/1.1 be used instead of HTTP/2.
    Http11Required = 0xD,
    /// The endpoint detected an unknown error code.
    Unknown = 0xE,
}

impl ProtocolErrorKind {
    pub fn from_code(code: isize) -> Self {
        use ProtocolErrorKind::*;

        match code {
            0x0 => None,
            0x1 => Protocol,
            0x2 => Internal,
            0x3 => FlowControl,
            0x4 => SettingsTimeout,
            0x5 => StreamClosed,
            0x6 => FrameSize,
            0x7 => RefusedStream,
            0x8 => Cancel,
            0x9 => Compression,
            0xA => Connect,
            0xB => EnhanceYourCalm,
            0xC => InadequateSecurity,
            0xD => Http11Required,
            0xE => Unknown,
            _ => Unknown,
        }
    }

    pub(crate) fn as_code(&self) -> isize {
        *self as isize
    }

    pub(crate) fn as_id_str(&self) -> &str {
        use ProtocolErrorKind::*;

        match self {
            None => "NO_ERROR",
            Protocol => "PROTOCOL_ERROR",
            Internal => "INTERNAL_ERROR",
            FlowControl => "FLOW_CONTROL_ERROR",
            SettingsTimeout => "SETTINGS_TIMEOUT",
            StreamClosed => "STREAM_CLOSED",
            FrameSize => "FRAME_SIZE_ERROR",
            RefusedStream => "REFUSED_STREAM",
            Cancel => "CANCEL",
            Compression => "COMPRESSION_ERROR",
            Connect => "CONNECT_ERROR",
            EnhanceYourCalm => "ENHANCE_YOUR_CALM",
            InadequateSecurity => "INADEQUATE_SECURITY",
            Http11Required => "HTTP_1_1_REQUIRED",
            Unknown => "INTERNAL_ERROR",
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        use ProtocolErrorKind::*;

        match self {
            None => "condition is not a result of an error",
            Protocol => "detected an unspecific protocol error",
            Internal => "encountered an unexpected internal error",
            FlowControl => "detected that its peer violated the flow-control protocol",
            SettingsTimeout => "sent a SETTINGS frame but did not receive a response in a timely manner",
            StreamClosed => "received a frame after a stream was half-closed",
            FrameSize => "received a frame with an invalid size",
            RefusedStream => "refused the stream prior to performing any application processing",
            Cancel => "indicates that the stream is no longer needed",
            Compression => "unable to maintain the header compression context for the connection",
            Connect => "connection established in response to a CONNECT request was reset or abnormally closed",
            EnhanceYourCalm => "detected that its peer is exhibiting a behavior that might be generating excessive load",
            InadequateSecurity => "underlying transport has properties that do not meet minimum security requirements",
            Http11Required => "requires that HTTP/1.1 be used instead of HTTP/2",
            Unknown => "encountered an unexpected error code",
        }
    }
}

impl fmt::Display for ProtocolErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.as_id_str(), self.as_str())
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    /// The setting identifier provided is invalid.
    InvalidSetting(String),
    /// The length of the data in bytes bytes is too short or too long.
    DataLength,
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &str {
        use ErrorKind::*;

        match *self {
            InvalidSetting(ref e) => e,
            DataLength => "Data length is too long or too short",
        }
    }
}

pub enum Repr {
    Io(io::Error),
    Protocol(ProtocolErrorKind),
    Simple(ErrorKind),
    Hpack(hpack::ErrorKind),
}

impl fmt::Debug for Repr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Repr::*;

        match self {
            Io(ref e) => e.fmt(f),
            Protocol(kind) => f
                .debug_struct("Protocol")
                .field("code", &kind.as_code())
                .field("id", &kind.as_id_str())
                .field("description", &kind.as_str())
                .finish(),
            Simple(kind) => f.debug_tuple("Kind").field(&kind).finish(),
            Hpack(kind) => f.debug_tuple("HpackKind").field(&kind).finish(),
        }
    }
}

pub struct Error(Box<Repr>);

impl Error {
    pub fn new(kind: Repr) -> Error {
        Error(Box::new(kind))
    }

    pub fn from_raw_protocol_error(code: isize) -> Error {
        Error(Box::new(Repr::Protocol(ProtocolErrorKind::from_code(code))))
    }

    pub fn raw_protocol_error(&self) -> Option<isize> {
        use Repr::*;

        match *self.0 {
            Io(..) => None,
            Protocol(i) => Some(i as isize),
        }
    }

    pub fn kind(&self) -> &Repr {
        &self.0
    }
}

impl From<ProtocolErrorKind> for Error {
    fn from(kind: ProtocolErrorKind) -> Self {
        Error::new(Repr::Protocol(kind))
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error::new(Repr::Simple(kind))
    }
}

impl From<hpack::ErrorKind> for Error {
    fn from(kind: hpack::ErrorKind) -> Self {
        Error::new(Repr::Hpack(kind))
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Repr::*;

        match *self.0 {
            Io(ref e) => e.fmt(f),
            Protocol(kind) => write!(f, "{}: {}", kind.as_id_str(), kind.as_str()),
            Simple(kind) => kind.fmt(f),
            Hpack(kind) => kind.fmt(f),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Repr::*;

        match *self.0 {
            Io(ref e) => e.fmt(f),
            Protocol(kind) => write!(f, "{}", kind.as_str()),
            Simple(kind) => write!(f, "{}", kind.as_str()),
            Hpack(kind) => write!(f, "{}", kind.as_str()),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use Repr::*;

        match *self.0 {
            Io(e) => e.description(),
            Protocol(kind) => kind.as_str(),
            Simple(kind) => kind.as_str(),
            Hpack(kind) => kind.as_str(),
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Repr::*;

        match *self.0 {
            Io(e) => e.source(),
            Protocol(..) => None,
            Simple(..) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::new(Repr::Io(e))
    }
}

// TODO: Test like std::io::error test.
