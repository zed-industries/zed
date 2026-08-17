//! Randomness support.

use security_framework_sys::random::{SecRandomCopyBytes, SecRandomRef, kSecRandomDefault};
use std::io;

/// A source of random data.
pub struct SecRandom(SecRandomRef);

unsafe impl Sync for SecRandom {}
unsafe impl Send for SecRandom {}

impl Default for SecRandom {
    #[inline(always)]
    fn default() -> Self {
        unsafe { Self(kSecRandomDefault) }
    }
}

impl SecRandom {
    /// Fills the buffer with cryptographically secure random bytes.
    pub fn copy_bytes(&self, buf: &mut [u8]) -> io::Result<()> {
        if unsafe { SecRandomCopyBytes(self.0, buf.len(), buf.as_mut_ptr().cast()) } == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut buf = [0; 10];
        SecRandom::default().copy_bytes(&mut buf).unwrap();
    }
}
