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

use crate::crypto::hmac::Hmac;

use siphasher::sip::SipHasher24;
use zeroize::Zeroize;

use alloc::vec::Vec;

/// Key context for SipHash-2-4 hasher.
struct KeyContext {
    /// First SipHash key.
    sip_key1: u64,

    /// Second SipHash key.
    sip_key2: u64,

    /// IV.
    sip_iv: Vec<u8>,
}

/// SipHasher for (de)obfuscating frame sizes.
pub struct SipHash {
    /// Key context for obfuscating frame sizes that are sent.
    sender: KeyContext,

    /// Key context for deobfuscating frame sizes that are received.
    receiver: KeyContext,
}

impl SipHash {
    /// Derive SipHash (de)obfuscation keys.
    fn derive_keys(key: &[u8], h: &[u8]) -> (KeyContext, KeyContext) {
        // from specification, generation additional symmetric key for SipHash
        let mut temp_key = {
            let mut ask_master = Hmac::new(key).update(&b"ask"[..]).update([0x01]).finalize();

            let temp_key = Hmac::new(&ask_master).update(h).update(&b"siphash"[..]).finalize();

            let mut sip_master = Hmac::new(&temp_key).update([0x01]).finalize();
            let temp_key = Hmac::new(&sip_master).update([]).finalize();

            ask_master.zeroize();
            sip_master.zeroize();

            temp_key
        };

        // initiator's SipHash keys
        let sipkeys_ab = Hmac::new(&temp_key).update([0x01]).finalize();

        // responder's SipHash keys
        let sipkeys_ba = Hmac::new(&temp_key).update(&sipkeys_ab).update([0x02]).finalize();

        temp_key.zeroize();

        let (sipk1_initiator, sipk2_initiator, sipiv_initiator) = {
            (
                u64::from_le_bytes(
                    TryInto::<[u8; 8]>::try_into(&sipkeys_ab[..8]).expect("to succeed"),
                ),
                u64::from_le_bytes(
                    TryInto::<[u8; 8]>::try_into(&sipkeys_ab[8..16]).expect("to succeed"),
                ),
                sipkeys_ab[16..24].to_vec(),
            )
        };

        let (sipk1_responder, sipk2_responder, sipiv_responder) = {
            (
                u64::from_le_bytes(
                    TryInto::<[u8; 8]>::try_into(&sipkeys_ba[..8]).expect("to succeed"),
                ),
                u64::from_le_bytes(
                    TryInto::<[u8; 8]>::try_into(&sipkeys_ba[8..16]).expect("to succeed"),
                ),
                sipkeys_ba[16..24].to_vec(),
            )
        };

        (
            KeyContext {
                sip_key1: sipk1_initiator,
                sip_key2: sipk2_initiator,
                sip_iv: sipiv_initiator,
            },
            KeyContext {
                sip_key1: sipk1_responder,
                sip_key2: sipk2_responder,
                sip_iv: sipiv_responder,
            },
        )
    }

    /// Derive SipHash keys and return new [`SipHash`] instance.
    pub fn new_initiator(key: &[u8], h: &[u8]) -> Self {
        let (initiator, responder) = Self::derive_keys(key, h);

        Self {
            sender: initiator,
            receiver: responder,
        }
    }

    /// Derive SipHash keys and return new [`SipHash`] instance for responder.
    pub fn new_responder(key: &[u8], h: &[u8]) -> Self {
        let (initiator, responder) = Self::derive_keys(key, h);

        Self {
            sender: responder,
            receiver: initiator,
        }
    }

    /// Deobfuscate `size`.
    pub fn deobfuscate(&mut self, size: u16) -> u16 {
        let hasher = SipHasher24::new_with_keys(self.receiver.sip_key1, self.receiver.sip_key2);
        let hash: u64 = hasher.hash(&self.receiver.sip_iv);
        self.receiver.sip_iv = hash.to_le_bytes().to_vec();

        size ^ ((hash & 0xffff) as u16)
    }

    /// Obfuscate `size`.
    pub fn obfuscate(&mut self, size: u16) -> u16 {
        let hasher = SipHasher24::new_with_keys(self.sender.sip_key1, self.sender.sip_key2);
        let hash: u64 = hasher.hash(&self.sender.sip_iv);
        self.sender.sip_iv = hash.to_le_bytes().to_vec();

        size ^ ((hash & 0xffff) as u16)
    }
}
