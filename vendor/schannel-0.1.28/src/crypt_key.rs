//! CryptoAPI private keys.

use windows_sys::Win32::Security::Cryptography;

/// A handle to a key.
pub struct CryptKey(usize);

impl Drop for CryptKey {
    fn drop(&mut self) {
        unsafe {
            Cryptography::CryptDestroyKey(self.0);
        }
    }
}

inner!(CryptKey, usize);
