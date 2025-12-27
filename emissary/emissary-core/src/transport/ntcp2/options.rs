// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

#![allow(unused)]

use bytes::{BufMut, BytesMut};
use nom::{
    number::complete::{be_u16, be_u32, be_u8},
    IResult,
};

/// Length of serialized [`InitiatorOptions`].
const INITIATOR_OPTIONS_SERIALIZED_LEN: usize = 16usize;

/// Length of serialized [`ResponderOptions`].
const RESPONDER_OPTIONS_SERIALIZED_LEN: usize = 16usize;

/// Initiator options.
#[derive(Debug)]
pub struct InitiatorOptions {
    /// Network ID
    pub network_id: u8,

    /// Version.
    pub version: u8,

    /// Padding length.
    pub padding_length: u16,

    /// Length of message 3 part 2.
    pub m3_p2_len: u16,

    // Timestamp.
    pub timestamp: u32,
}

impl InitiatorOptions {
    /// Attempt to parse [`InitiatorOptions`] from `input`.
    ///
    /// Returns the parsed options and rest of `input` on success.
    pub fn parse_frame(input: &[u8]) -> IResult<&[u8], Self> {
        let (rest, network_id) = be_u8(input)?;
        let (rest, version) = be_u8(rest)?;
        let (rest, padding_length) = be_u16(rest)?;
        let (rest, m3_p2_len) = be_u16(rest)?;
        let (rest, _) = be_u16(rest)?; // reserved1
        let (rest, timestamp) = be_u32(rest)?;
        let (rest, _) = be_u32(rest)?; // reserved2

        Ok((
            rest,
            Self {
                network_id,
                version,
                padding_length,
                m3_p2_len,
                timestamp,
            },
        ))
    }

    /// Attempt to parse `input` into [`InitiatorOptions`].
    pub fn parse(input: &[u8]) -> Option<Self> {
        Self::parse_frame(input).ok().map(|(_, message)| message)
    }

    /// Serialize [`InitiatorOptions`] into a byte vector.
    pub fn serialize(self) -> BytesMut {
        let mut out = BytesMut::with_capacity(INITIATOR_OPTIONS_SERIALIZED_LEN);

        out.put_u8(self.network_id);
        out.put_u8(self.version);
        out.put_u16(self.padding_length);
        out.put_u16(self.m3_p2_len);
        out.put_u16(0u16); // reserved1
        out.put_u32(self.timestamp);
        out.put_u32(0u32); // reserved2

        out
    }
}

/// Responder options.
#[derive(Debug)]
pub struct ResponderOptions {
    /// Padding length.
    pub padding_length: u16,

    /// Timestamp.
    pub timestamp: u32,
}

impl ResponderOptions {
    /// Attempt to parse [`ResponderOptions`] from `input`.
    ///
    /// Returns the parsed options and rest of `input` on success.
    pub fn parse_frame(input: &[u8]) -> IResult<&[u8], Self> {
        let (rest, _) = be_u16(input)?; // reserved1
        let (rest, padding_length) = be_u16(rest)?;
        let (rest, _) = be_u32(rest)?; // reserved2
        let (rest, timestamp) = be_u32(rest)?;
        let (rest, _) = be_u32(rest)?; // reserved3

        Ok((
            rest,
            Self {
                padding_length,
                timestamp,
            },
        ))
    }

    /// Attempt to parse `input` into [`ResponderOptions`].
    pub fn parse(input: &[u8]) -> Option<Self> {
        Self::parse_frame(input).ok().map(|(_, message)| message)
    }

    /// Serialize [`ResponderOptions`] into a byte vector.
    pub fn serialize(self) -> BytesMut {
        let mut out = BytesMut::with_capacity(RESPONDER_OPTIONS_SERIALIZED_LEN);

        out.put_u16(0u16); // reserved1
        out.put_u16(self.padding_length);
        out.put_u32(0u32); // reserved2
        out.put_u32(self.timestamp);
        out.put_u32(0u32); // reserved3

        out
    }
}
