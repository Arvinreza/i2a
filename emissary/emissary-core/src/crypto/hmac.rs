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

use hmac::Mac;
use sha2::Sha256;

use alloc::vec::Vec;

pub struct Hmac {
    hmac: hmac::Hmac<Sha256>,
}

impl Hmac {
    /// Create new [`Hmac`].
    pub fn new(key: &[u8]) -> Self {
        Self {
            hmac: hmac::Hmac::new_from_slice(key).expect("valid key size"),
        }
    }

    /// Update state.
    pub fn update(mut self, bytes: impl AsRef<[u8]>) -> Self {
        self.hmac.update(bytes.as_ref());
        self
    }

    /// Finalize and return MAC.
    //
    // TODO: remove
    pub fn finalize(self) -> Vec<u8> {
        self.hmac.finalize().into_bytes().to_vec()
    }

    /// Finalize and return MAC.
    //
    // TODO: rename
    pub fn finalize_new(self) -> [u8; 32] {
        self.hmac.finalize().into_bytes().into()
    }
}
