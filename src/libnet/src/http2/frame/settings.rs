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

use crate::http2::error::{ErrorKind, ProtocolErrorKind};
use crate::http2::{Error, Result};

use std::fmt;
use std::io;

// Traits
use std::io::{Read, Seek, Write};

// 0x1
/// Allows the sender to inform the remote endpoint of the maximum size of the header compression
/// table used to decode header blocks, in octets. The encoder can select any size equal to or less
/// than this value by using signaling specific to the header compression format inside a header
/// block. The initial value is 4,096 octets.
pub const DEFAULT_HEADER_TABLE_SIZE: u32 = 4_096;

// 0x2
/// This setting can be used to disable server push. An endpoint MUST NOT send a PUSH_PROMISE frame
/// if it receives this parameter set to a value of 0. An endpoint that has both set this parameter
/// to 0 and had it acknowledged MUST treat the receipt of a PUSH_PROMISE frame as a connection
/// error of type PROTOCOL_ERROR.
///
/// The initial value is true (or 1). If any other value that is received other than 0 or 1 MUST be
/// treated as a connection error of type PROTOCOL_ERROR.
pub const DEFAULT_ENABLE_PUSH: bool = true;

// 0x3
/// Indicates the maximum number of concurrent streams that the sender will allow. This limit is
/// directional: it applies to the number of streams that the sender permits the receiver to create.
/// Initially, there is no limit to this value. It is recommended that this value be no smaller than
/// 100, so as to not unnecessarily limit parallelism.
pub const DEFAULT_MAX_CONCURRENT_STREAMS: Option<u32> = None;
// 0x4
pub const DEFAULT_INITIAL_WINDOW_SIZE: u32 = 65_535;
// 0x5
pub const DEFAULT_MAX_FRAME_SIZE: u32 = 16_384;
// 0x6
pub const DEFAULT_MAX_HEADER_LIST_SIZE: Option<u32> = None;

/// Values smaller than this, while not an error, will unnecessarily limit parallelism.
pub const MIN_CONCURRENT_STREAMS: u32 = 100;

/// Values larger than this should be treated as a connection error of type FLOW_CONTROL_ERROR.
pub const MAX_INITIAL_WINDOW_SIZE: u32 = (1 << 31) - 1;

// Value advertised by an endpoint MUST be between this initial value and the maximum allowed
// frame size (2^24-1 or 16,777,215 octets), inclusive. Values outside this range MUST be
// treated as a connection error of type PROTOCOL_ERROR.
pub const MAX_FRAME_SIZE: u32 = (1 << 24) - 1;

pub const FLAGS_ACK: u8 = 0x1;

pub const FLAGS_ALL: u8 = FLAGS_ACK;

pub struct Flags(u8);

impl Flags {
    pub fn new() -> Self {
        Self(0u8)
    }

    pub fn set_ack(&mut self) {
        self.0 |= FLAGS_ACK;
    }

    pub fn set_all(&mut self) {
        self.0 |= FLAGS_ALL;
    }

    pub fn is_ack(&self) -> bool {
        self.0 & FLAGS_ACK == FLAGS_ACK
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

impl From<Flags> for u8 {
    fn from(src: Flags) -> Self {
        src.0
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Flags(");
        if self.0 & FLAGS_ACK == FLAGS_ACK {
            write!(f, "ACK)");
        } else {
            write!(f, "NONE)");
        }
        Ok(())
    }
}

impl fmt::Binary for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:b}", self.0);
        Ok(())
    }
}

impl fmt::LowerHex for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0);
        Ok(())
    }
}

impl fmt::UpperHex for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", self.0);
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
enum SettingKind {
    HeaderTableSize = 0x1,
    EnablePush = 0x2,
    MaxConcurrentStreams = 0x3,
    InitialWindowSize = 0x4,
    MaxFrameSize = 0x5,
    MaxHeaderListSize = 0x6,
}

impl SettingKind {
    fn maybe_from(i: u16) -> Option<Self> {
        use SettingKind::*;

        match i as isize {
            0x1 => Some(HeaderTableSize),
            0x2 => Some(EnablePush),
            0x3 => Some(MaxConcurrentStreams),
            0x4 => Some(InitialWindowSize),
            0x5 => Some(MaxFrameSize),
            0x6 => Some(MaxHeaderListSize),
            _ => None,
        }
    }
}

struct Setting {
    kind: SettingKind,
    value: u32,
}

impl Setting {
    fn new(kind: SettingKind, value: u32) -> Result<Self> {
        use SettingKind::*;

        Ok(match kind {
            HeaderTableSize => Self::new_header_table_size(value),
            EnablePush => Self::new_enable_push(value)?,
            MaxConcurrentStreams => Self::new_max_concurrent_streams(value),
            InitialWindowSize => Self::new_initial_window_size(value)?,
            MaxFrameSize => Self::new_max_frame_size(value)?,
            MaxHeaderListSize => Self::new_max_header_list_size(value),
        })
    }

    fn new_header_table_size(size: u32) -> Self {
        Self {
            kind: SettingKind::HeaderTableSize,
            value: size,
        }
    }

    fn new_enable_push<E: Into<u32>>(enabled: E) -> Result<Self> {
        let enabled: u32 = enabled.into();
        if enabled > 1 {
            return Err(Error::from(ProtocolErrorKind::Protocol));
        }

        Ok(Self {
            kind: SettingKind::EnablePush,
            value: enabled,
        })
    }

    fn new_max_concurrent_streams(max: u32) -> Self {
        Self {
            kind: SettingKind::MaxConcurrentStreams,
            value: max,
        }
    }

    fn new_initial_window_size(size: u32) -> Result<Self> {
        if size > MAX_INITIAL_WINDOW_SIZE {
            return Err(Error::from(ProtocolErrorKind::FlowControl));
        }

        Ok(Self {
            kind: SettingKind::InitialWindowSize,
            value: size,
        })
    }

    fn new_max_frame_size(size: u32) -> Result<Self> {
        if size > MAX_FRAME_SIZE {
            return Err(Error::from(ProtocolErrorKind::Protocol));
        }

        Ok(Self {
            kind: SettingKind::MaxFrameSize,
            value: size,
        })
    }

    fn new_max_header_list_size(max: u32) -> Self {
        // FIXME: Maybe not include limit it, but check it elsewhere.
        //        if let Some(l) = limit {
        //            if max < l {
        //                return Err(Error::from(ProtocolErrorKind::Protocol));
        //            }
        //        }

        Self {
            kind: SettingKind::MaxHeaderListSize,
            value: max,
        }
    }

    fn deserialize<R: Read + Seek>(reader: &mut R) -> Option<Self> {
        let mut kind_buf = [0u8; 2];
        let mut value_buf = [0u8; 4];

        reader.read_exact(&mut kind_buf).unwrap();
        reader.read_exact(&mut value_buf).unwrap();

        let kind = u16::from_be_bytes(kind_buf);
        let value = u32::from_be_bytes(value_buf);

        SettingKind::maybe_from(kind).map(|kind| Self { kind, value })
    }

    fn serialize<W: Write + Seek>(&self, writer: &mut W, dst: &mut [u8]) -> Result<()> {
        let kind = self.kind as u16;
        let value = self.value;

        writer.write(&kind.to_be_bytes())?;
        writer.write(&value.to_be_bytes())?;

        Ok(())
    }
}

pub struct Settings {
    stream_id: u32,
    flags: Flags,
    header_table_size: Option<u32>,
    enable_push: Option<u32>,
    max_concurrent_streams: Option<u32>,
    initial_window_size: Option<u32>,
    max_frame_size: Option<u32>,
    max_header_list_size: Option<u32>,
    max_header_list_size_limit: Option<u32>,
}

impl Settings {
    pub fn new(stream_id: u32) -> Self {
        Self {
            stream_id,
            ..Self::default()
        }
    }

    pub fn set_stream_id(&mut self, id: u32) {
        self.stream_id = id;
    }

    pub fn set_flag_ack(&mut self) {
        self.flags.set_ack();
    }

    pub fn set_flag_all(&mut self) {
        self.flags.set_all();
    }

    pub fn set_header_table_size(&mut self, size: Option<u32>) {
        self.initial_window_size = size;
    }

    pub fn set_enable_push<P: Into<u32>>(&mut self, push: Option<P>) {
        let push: Option<u32> = push.map(|p| p.into());
        if let Some(p) = push {
            if p > 1 {
                panic!(
                    "Http/2 protocol violation: only 0 or 1 are allowed but {} was set",
                    p
                );
            }
        }

        self.enable_push = push;
    }

    pub fn set_max_concurrent_streams(&mut self, max: Option<u32>) {
        self.max_concurrent_streams = max;
    }

    pub fn set_initial_window_size(&mut self, size: Option<u32>) {
        if let Some(s) = size {
            if s > MAX_INITIAL_WINDOW_SIZE {
                panic!(
                    "Http/2 flow control violation: max initial window size is {} but {} was \
                     set",
                    MAX_INITIAL_WINDOW_SIZE, s
                );
            }
        }

        self.initial_window_size = size;
    }

    pub fn set_max_frame_size(&mut self, size: Option<u32>) {
        if let Some(s) = size {
            if s > MAX_FRAME_SIZE {
                panic!(
                    "Http/2 protocol violation: max frame size is {} but {} was set",
                    MAX_FRAME_SIZE, s
                );
            }
        }

        self.max_frame_size = size;
    }

    pub fn set_max_header_list_size(&mut self, size: Option<u32>) {
        self.max_header_list_size = size;
    }

    pub fn set_max_header_list_size_limit(&mut self, limit: Option<u32>) {
        self.max_header_list_size_limit = limit;
    }

    pub fn stream_id(&self) -> u32 {
        self.stream_id
    }

    pub fn header_table_size(&self) -> Option<u32> {
        self.header_table_size
    }

    pub fn is_push_enabled(&self) -> Option<u32> {
        self.enable_push
    }

    pub fn max_concurrent_streams(&self) -> Option<u32> {
        self.max_concurrent_streams
    }

    pub fn initial_window_size(&self) -> Option<u32> {
        self.initial_window_size
    }

    pub fn max_frame_size(&self) -> Option<u32> {
        self.max_frame_size
    }

    pub fn max_header_list_size(&self) -> Option<u32> {
        self.max_header_list_size
    }

    pub fn max_header_list_size_limit(&self) -> Option<u32> {
        self.max_header_list_size_limit
    }

    pub fn empty_flags(&mut self) {
        self.flags.empty();
    }

    pub fn flag_is_ack(&self) -> bool {
        self.flags.is_ack()
    }

    pub fn flag_is_all(&self) -> bool {
        self.flags.is_all()
    }

    pub fn payload_len(&self) -> Result<usize> {
        let mut len = 0;
        self.for_each_setting(|_| len += 6)?;
        Ok(len)
    }

    pub fn deserialize(stream_id: u32, bytes: &[u8]) -> Result<Self> {
        let len = bytes.len() as u64;
        if len % 6 != 0 {
            return Err(Error::from(ErrorKind::DataLength));
        }

        let mut settings = Self::new(stream_id);
        let mut reader = io::Cursor::new(bytes);
        while reader.position() != len {
            Setting::deserialize(&mut reader).map(|setting| settings.set(setting));
        }

        Ok(settings)
    }

    pub fn serialize<W: Write + Seek>(&self, writer: &mut W, dst: &mut [u8]) -> Result<()> {
        let payload_len = self.payload_len();

        // FIXME: When std::ops::Try is stable, change `?` behaviour to replace the `unwrap`.
        self.for_each_setting(|s| s.serialize(writer, dst).unwrap())?;
        Ok(())
    }

    fn set(&mut self, setting: Setting) {
        use SettingKind::*;

        match setting.kind {
            HeaderTableSize => self.set_header_table_size(Some(setting.value)),
            EnablePush => self.set_enable_push(Some(setting.value)),
            MaxConcurrentStreams => self.set_max_concurrent_streams(Some(setting.value)),
            InitialWindowSize => self.set_initial_window_size(Some(setting.value)),
            MaxFrameSize => self.set_max_frame_size(Some(setting.value)),
            MaxHeaderListSize => self.set_max_header_list_size(Some(setting.value)),
        }
    }

    fn for_each_setting<F: FnMut(Setting)>(&self, mut f: F) -> Result<()> {
        if let Some(v) = self.header_table_size {
            f(Setting::new_header_table_size(v))
        }

        if let Some(v) = self.enable_push {
            f(Setting::new_enable_push(v)?);
        }

        if let Some(v) = self.max_concurrent_streams {
            f(Setting::new_max_concurrent_streams(v));
        }

        if let Some(v) = self.initial_window_size {
            f(Setting::new_initial_window_size(v)?);
        }

        if let Some(v) = self.max_frame_size {
            f(Setting::new_max_frame_size(v)?);
        }

        if let Some(v) = self.max_header_list_size {
            f(Setting::new_max_header_list_size(v));
        }

        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            stream_id: 0,
            flags: Flags::new(),
            header_table_size: None,
            enable_push: None,
            max_concurrent_streams: None,
            initial_window_size: None,
            max_frame_size: None,
            max_header_list_size: None,
            max_header_list_size_limit: None,
        }
    }
}
