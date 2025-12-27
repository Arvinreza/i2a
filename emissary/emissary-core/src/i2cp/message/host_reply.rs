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

use crate::i2cp::message::{MessageType, I2CP_HEADER_SIZE};

use bytes::{BufMut, Bytes, BytesMut};

/// Reply kind for host lookup.
pub enum HostReplyKind {
    /// Lookup succeeded.
    Success {
        /// Serialized destination.
        destination: Bytes,
    },

    /// Host lookup failed.
    Failure,

    /// Password required.
    #[allow(unused)]
    PasswordRequired,

    /// Private key required.
    #[allow(unused)]
    PrivateKeyRequired,

    /// Password and private key required.
    #[allow(unused)]
    PaswordAndPrivateKeyRequired,

    /// Failed to decrypt [`LeaseSet`].
    #[allow(unused)]
    LeasesetDecryptionFailed,
}

impl HostReplyKind {
    /// Serialize [`HostReplyKind`].
    fn as_u8(&self) -> u8 {
        match self {
            Self::Success { .. } => 0,
            Self::Failure => 1,
            Self::PasswordRequired => 2,
            Self::PrivateKeyRequired => 3,
            Self::PaswordAndPrivateKeyRequired => 4,
            Self::LeasesetDecryptionFailed => 5,
        }
    }

    /// Get serialized length of [`HostReplyKind`].
    ///
    /// [`HostReplyKind`] is at least one byte long (result code)
    fn serialized_len(&self) -> usize {
        match self {
            Self::Success { destination } => 1usize + destination.len(),
            _ => 1usize,
        }
    }
}

/// `HostReply` message.
///
/// https://geti2p.net/spec/i2cp#hostreplymessage
pub struct HostReply(());

impl HostReply {
    /// Create new `HostReply` message.
    pub fn new(session_id: u16, request_id: u32, kind: HostReplyKind) -> BytesMut {
        // session id + request id + serialized `kind`
        let payload_len = 2 + 3 + kind.serialized_len();
        let mut out = BytesMut::with_capacity(I2CP_HEADER_SIZE + payload_len);

        out.put_u32(payload_len as u32);
        out.put_u8(MessageType::HostReply.as_u8());
        out.put_u16(session_id);
        out.put_u32(request_id);
        out.put_u8(kind.as_u8());

        if let HostReplyKind::Success { destination } = kind {
            out.put_slice(&destination);
        }

        out
    }
}
