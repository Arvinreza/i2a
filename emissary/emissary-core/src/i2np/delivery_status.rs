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

use bytes::{BufMut, BytesMut};
use nom::{
    number::complete::{be_u32, be_u64},
    IResult,
};

use core::time::Duration;

/// Delivery status.
pub struct DeliveryStatus {
    /// Message ID.
    pub message_id: u32,

    /// Timestamp as duration since UNIX epoch.
    pub timestamp: Duration,
}

impl DeliveryStatus {
    /// Attempt to parse [`DeliveryStatus`] from `input`.
    ///
    /// Returns the parsed message and rest of `input` on success.
    pub fn parse_frame(input: &[u8]) -> IResult<&[u8], Self> {
        let (rest, message_id) = be_u32(input)?;
        let (rest, timestamp) = be_u64(rest)?;

        Ok((
            rest,
            Self {
                message_id,
                timestamp: Duration::from_millis(timestamp),
            },
        ))
    }

    /// Attempt to parse `input` into [`DeliveryStatus`].
    pub fn parse(input: &[u8]) -> Option<Self> {
        Self::parse_frame(input).ok().map(|(_, message)| message)
    }

    /// Serialize [`DeliveryStatus`] into a byte vector.
    pub fn serialize(self) -> BytesMut {
        let mut out = BytesMut::with_capacity(4 + 8);

        out.put_u32(self.message_id);
        out.put_u64(self.timestamp.as_millis() as u64);

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_deserialize() {
        let serialized = DeliveryStatus {
            message_id: 13371338u32,
            timestamp: Duration::from_millis(13391440),
        }
        .serialize();

        let DeliveryStatus {
            message_id,
            timestamp,
        } = DeliveryStatus::parse(&serialized).unwrap();
        assert_eq!(message_id, 13371338u32);
        assert_eq!(timestamp, Duration::from_millis(13391440));
    }
}
