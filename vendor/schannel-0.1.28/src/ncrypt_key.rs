//! CNG private keys.

use windows_sys::Win32::Security::Cryptography;

/// A CNG handle to a key.
pub struct NcryptKey(Cryptography::NCRYPT_KEY_HANDLE);

impl Drop for NcryptKey {
    fn drop(&mut self) {
        unsafe {
            Cryptography::NCryptFreeObject(self.0);
        }
    }
}

inner!(NcryptKey, Cryptography::NCRYPT_KEY_HANDLE);
