//! Cryptographic primitives using either native crates or openssl.
#[cfg(feature = "native_crypto")]
mod native;
#[cfg(all(feature = "native_crypto", not(feature = "unstable")))]
pub(crate) use native::*;
#[cfg(all(feature = "native_crypto", feature = "unstable"))]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub use native::*;

mod error;
pub use error::Error;

#[cfg(feature = "openssl_crypto")]
mod openssl;
#[cfg(all(feature = "openssl_crypto", not(feature = "unstable")))]
pub(crate) use self::openssl::*;
#[cfg(all(feature = "openssl_crypto", feature = "unstable"))]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub use self::openssl::*;

#[cfg(test)]
mod test {
    use super::*;
    use crate::Key;

    #[test]
    fn test_encrypt() {
        let data = b"some data";
        let expected_encrypted = &[
            241, 233, 175, 173, 142, 44, 63, 240, 77, 154, 211, 233, 217, 170, 49, 142,
        ];
        let aes_key = Key::new(vec![
            132, 3, 113, 222, 81, 209, 49, 43, 81, 232, 243, 46, 1, 103, 184, 42,
        ]);
        let aes_iv = &[
            78, 82, 67, 158, 214, 102, 48, 109, 84, 107, 94, 54, 225, 29, 186, 246,
        ];

        let encrypted = encrypt(data, &aes_key, aes_iv).unwrap();
        assert_eq!(encrypted, expected_encrypted);

        let decrypted = decrypt(&encrypted, &aes_key, aes_iv).unwrap();
        assert_eq!(decrypted.to_vec(), data);
    }

    #[test]
    fn test_legacy_derive_key_and_iv() {
        let expected_key = &[
            0x1f, 0x35, 0x38, 0x40, 0xf2, 0x95, 0x73, 0x30, 0xa6, 0xcb, 0x01, 0xf9, 0x53, 0xba,
            0x22, 0x12,
        ];
        let expected_iv = &[
            0x7f, 0xf5, 0x65, 0xb2, 0x31, 0xa5, 0x77, 0x32, 0xf8, 0xd3, 0xd0, 0xa6, 0x45, 0x1c,
            0x39, 0x97,
        ];
        let salt = &[0x92, 0xf4, 0xc0, 0x34, 0x0f, 0x5f, 0x36, 0xf9];
        let iteration_count = 1782;
        let password = b"test";
        let (key, iv) = legacy_derive_key_and_iv(password, Ok(()), salt, iteration_count).unwrap();
        assert_eq!(key.as_ref(), &expected_key[..]);
        assert_eq!(iv, &expected_iv[..]);
    }
}
