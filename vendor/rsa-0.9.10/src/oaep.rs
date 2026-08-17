//! Encryption and Decryption using [OAEP padding](https://datatracker.ietf.org/doc/html/rfc8017#section-7.1).
//!
//! # Usage
//!
//! See [code example in the toplevel rustdoc](../index.html#oaep-encryption).

mod decrypting_key;
mod encrypting_key;

pub use self::{decrypting_key::DecryptingKey, encrypting_key::EncryptingKey};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

use digest::{Digest, DynDigest, FixedOutputReset};
use num_bigint::BigUint;
use rand_core::CryptoRngCore;
use zeroize::Zeroizing;

use crate::algorithms::oaep::*;
use crate::algorithms::pad::{uint_to_be_pad, uint_to_zeroizing_be_pad};
use crate::algorithms::rsa::{rsa_decrypt_and_check, rsa_encrypt};
use crate::errors::{Error, Result};
use crate::key::{self, RsaPrivateKey, RsaPublicKey};
use crate::traits::{PaddingScheme, PublicKeyParts};

/// Encryption and Decryption using [OAEP padding](https://datatracker.ietf.org/doc/html/rfc8017#section-7.1).
///
/// - `digest` is used to hash the label. The maximum possible plaintext length is `m = k - 2 * h_len - 2`,
///   where `k` is the size of the RSA modulus.
/// - `mgf_digest` specifies the hash function that is used in the [MGF1](https://datatracker.ietf.org/doc/html/rfc8017#appendix-B.2).
/// - `label` is optional data that can be associated with the message.
///
/// The two hash functions can, but don't need to be the same.
///
/// A prominent example is the [`AndroidKeyStore`](https://developer.android.com/guide/topics/security/cryptography#oaep-mgf1-digest).
/// It uses SHA-1 for `mgf_digest` and a user-chosen SHA flavour for `digest`.
pub struct Oaep {
    /// Digest type to use.
    pub digest: Box<dyn DynDigest + Send + Sync>,

    /// Digest to use for Mask Generation Function (MGF).
    pub mgf_digest: Box<dyn DynDigest + Send + Sync>,

    /// Optional label.
    pub label: Option<String>,
}

impl Oaep {
    /// Create a new OAEP `PaddingScheme`, using `T` as the hash function for both the default (empty) label and for MGF1.
    ///
    /// # Example
    /// ```
    /// use sha1::Sha1;
    /// use sha2::Sha256;
    /// use rsa::{BigUint, RsaPublicKey, Oaep, };
    /// use base64ct::{Base64, Encoding};
    ///
    /// let n = Base64::decode_vec("ALHgDoZmBQIx+jTmgeeHW6KsPOrj11f6CvWsiRleJlQpW77AwSZhd21ZDmlTKfaIHBSUxRUsuYNh7E2SHx8rkFVCQA2/gXkZ5GK2IUbzSTio9qXA25MWHvVxjMfKSL8ZAxZyKbrG94FLLszFAFOaiLLY8ECs7g+dXOriYtBwLUJK+lppbd+El+8ZA/zH0bk7vbqph5pIoiWggxwdq3mEz4LnrUln7r6dagSQzYErKewY8GADVpXcq5mfHC1xF2DFBub7bFjMVM5fHq7RK+pG5xjNDiYITbhLYrbVv3X0z75OvN0dY49ITWjM7xyvMWJXVJS7sJlgmCCL6RwWgP8PhcE=").unwrap();
    /// let e = Base64::decode_vec("AQAB").unwrap();
    ///
    /// let mut rng = rand::thread_rng(); // rand@0.8
    /// let key = RsaPublicKey::new(BigUint::from_bytes_be(&n), BigUint::from_bytes_be(&e)).unwrap();
    /// let padding = Oaep::new::<Sha256>();
    /// let encrypted_data = key.encrypt(&mut rng, padding, b"secret").unwrap();
    /// ```
    pub fn new<T: 'static + Digest + DynDigest + Send + Sync>() -> Self {
        Self {
            digest: Box::new(T::new()),
            mgf_digest: Box::new(T::new()),
            label: None,
        }
    }

    /// Create a new OAEP `PaddingScheme` with an associated `label`, using `T` as the hash function for both the label and for MGF1.
    pub fn new_with_label<T: 'static + Digest + DynDigest + Send + Sync, S: AsRef<str>>(
        label: S,
    ) -> Self {
        Self {
            digest: Box::new(T::new()),
            mgf_digest: Box::new(T::new()),
            label: Some(label.as_ref().to_string()),
        }
    }

    /// Create a new OAEP `PaddingScheme`, using `T` as the hash function for the default (empty) label, and `U` as the hash function for MGF1.
    /// If a label is needed use `PaddingScheme::new_oaep_with_label` or `PaddingScheme::new_oaep_with_mgf_hash_with_label`.
    ///
    /// # Example
    /// ```
    /// use sha1::Sha1;
    /// use sha2::Sha256;
    /// use rsa::{BigUint, RsaPublicKey, Oaep, };
    /// use base64ct::{Base64, Encoding};
    ///
    /// let n = Base64::decode_vec("ALHgDoZmBQIx+jTmgeeHW6KsPOrj11f6CvWsiRleJlQpW77AwSZhd21ZDmlTKfaIHBSUxRUsuYNh7E2SHx8rkFVCQA2/gXkZ5GK2IUbzSTio9qXA25MWHvVxjMfKSL8ZAxZyKbrG94FLLszFAFOaiLLY8ECs7g+dXOriYtBwLUJK+lppbd+El+8ZA/zH0bk7vbqph5pIoiWggxwdq3mEz4LnrUln7r6dagSQzYErKewY8GADVpXcq5mfHC1xF2DFBub7bFjMVM5fHq7RK+pG5xjNDiYITbhLYrbVv3X0z75OvN0dY49ITWjM7xyvMWJXVJS7sJlgmCCL6RwWgP8PhcE=").unwrap();
    /// let e = Base64::decode_vec("AQAB").unwrap();
    ///
    /// let mut rng = rand::thread_rng(); // rand@0.8
    /// let key = RsaPublicKey::new(BigUint::from_bytes_be(&n), BigUint::from_bytes_be(&e)).unwrap();
    /// let padding = Oaep::new_with_mgf_hash::<Sha256, Sha1>();
    /// let encrypted_data = key.encrypt(&mut rng, padding, b"secret").unwrap();
    /// ```
    pub fn new_with_mgf_hash<
        T: 'static + Digest + DynDigest + Send + Sync,
        U: 'static + Digest + DynDigest + Send + Sync,
    >() -> Self {
        Self {
            digest: Box::new(T::new()),
            mgf_digest: Box::new(U::new()),
            label: None,
        }
    }

    /// Create a new OAEP `PaddingScheme` with an associated `label`, using `T` as the hash function for the label, and `U` as the hash function for MGF1.
    pub fn new_with_mgf_hash_and_label<
        T: 'static + Digest + DynDigest + Send + Sync,
        U: 'static + Digest + DynDigest + Send + Sync,
        S: AsRef<str>,
    >(
        label: S,
    ) -> Self {
        Self {
            digest: Box::new(T::new()),
            mgf_digest: Box::new(U::new()),
            label: Some(label.as_ref().to_string()),
        }
    }
}

impl PaddingScheme for Oaep {
    fn decrypt<Rng: CryptoRngCore>(
        mut self,
        rng: Option<&mut Rng>,
        priv_key: &RsaPrivateKey,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>> {
        decrypt(
            rng,
            priv_key,
            ciphertext,
            &mut *self.digest,
            &mut *self.mgf_digest,
            self.label,
        )
    }

    fn encrypt<Rng: CryptoRngCore>(
        mut self,
        rng: &mut Rng,
        pub_key: &RsaPublicKey,
        msg: &[u8],
    ) -> Result<Vec<u8>> {
        encrypt(
            rng,
            pub_key,
            msg,
            &mut *self.digest,
            &mut *self.mgf_digest,
            self.label,
        )
    }
}

impl fmt::Debug for Oaep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OAEP")
            .field("digest", &"...")
            .field("mgf_digest", &"...")
            .field("label", &self.label)
            .finish()
    }
}

/// Encrypts the given message with RSA and the padding scheme from
/// [PKCS#1 OAEP].
///
/// The message must be no longer than the length of the public modulus minus
/// `2 + (2 * hash.size())`.
///
/// [PKCS#1 OAEP]: https://datatracker.ietf.org/doc/html/rfc8017#section-7.1
#[inline]
fn encrypt<R: CryptoRngCore + ?Sized>(
    rng: &mut R,
    pub_key: &RsaPublicKey,
    msg: &[u8],
    digest: &mut dyn DynDigest,
    mgf_digest: &mut dyn DynDigest,
    label: Option<String>,
) -> Result<Vec<u8>> {
    key::check_public(pub_key)?;

    let em = oaep_encrypt(rng, msg, digest, mgf_digest, label, pub_key.size())?;

    let int = Zeroizing::new(BigUint::from_bytes_be(&em));
    uint_to_be_pad(rsa_encrypt(pub_key, &int)?, pub_key.size())
}

/// Encrypts the given message with RSA and the padding scheme from
/// [PKCS#1 OAEP].
///
/// The message must be no longer than the length of the public modulus minus
/// `2 + (2 * hash.size())`.
///
/// [PKCS#1 OAEP]: https://datatracker.ietf.org/doc/html/rfc8017#section-7.1
fn encrypt_digest<R: CryptoRngCore + ?Sized, D: Digest, MGD: Digest + FixedOutputReset>(
    rng: &mut R,
    pub_key: &RsaPublicKey,
    msg: &[u8],
    label: Option<String>,
) -> Result<Vec<u8>> {
    key::check_public(pub_key)?;

    let em = oaep_encrypt_digest::<_, D, MGD>(rng, msg, label, pub_key.size())?;

    let int = Zeroizing::new(BigUint::from_bytes_be(&em));
    uint_to_be_pad(rsa_encrypt(pub_key, &int)?, pub_key.size())
}

/// Decrypts a plaintext using RSA and the padding scheme from [PKCS#1 OAEP].
///
/// If an `rng` is passed, it uses RSA blinding to avoid timing side-channel attacks.
///
/// Note that whether this function returns an error or not discloses secret
/// information. If an attacker can cause this function to run repeatedly and
/// learn whether each instance returned an error then they can decrypt and
/// forge signatures as if they had the private key.
///
/// See `decrypt_session_key` for a way of solving this problem.
///
/// [PKCS#1 OAEP]: https://datatracker.ietf.org/doc/html/rfc8017#section-7.1
#[inline]
fn decrypt<R: CryptoRngCore + ?Sized>(
    rng: Option<&mut R>,
    priv_key: &RsaPrivateKey,
    ciphertext: &[u8],
    digest: &mut dyn DynDigest,
    mgf_digest: &mut dyn DynDigest,
    label: Option<String>,
) -> Result<Vec<u8>> {
    key::check_public(priv_key)?;

    if ciphertext.len() != priv_key.size() {
        return Err(Error::Decryption);
    }

    let em = rsa_decrypt_and_check(priv_key, rng, &BigUint::from_bytes_be(ciphertext))?;
    let mut em = uint_to_zeroizing_be_pad(em, priv_key.size())?;

    oaep_decrypt(&mut em, digest, mgf_digest, label, priv_key.size())
}

/// Decrypts a plaintext using RSA and the padding scheme from [PKCS#1 OAEP].
///
/// If an `rng` is passed, it uses RSA blinding to avoid timing side-channel attacks.
///
/// Note that whether this function returns an error or not discloses secret
/// information. If an attacker can cause this function to run repeatedly and
/// learn whether each instance returned an error then they can decrypt and
/// forge signatures as if they had the private key.
///
/// See `decrypt_session_key` for a way of solving this problem.
///
/// [PKCS#1 OAEP]: https://datatracker.ietf.org/doc/html/rfc8017#section-7.1
#[inline]
fn decrypt_digest<R: CryptoRngCore + ?Sized, D: Digest, MGD: Digest + FixedOutputReset>(
    rng: Option<&mut R>,
    priv_key: &RsaPrivateKey,
    ciphertext: &[u8],
    label: Option<String>,
) -> Result<Vec<u8>> {
    key::check_public(priv_key)?;

    if ciphertext.len() != priv_key.size() {
        return Err(Error::Decryption);
    }

    let em = rsa_decrypt_and_check(priv_key, rng, &BigUint::from_bytes_be(ciphertext))?;
    let mut em = uint_to_zeroizing_be_pad(em, priv_key.size())?;

    oaep_decrypt_digest::<D, MGD>(&mut em, label, priv_key.size())
}

#[cfg(test)]
mod tests {
    use crate::key::{RsaPrivateKey, RsaPublicKey};
    use crate::oaep::{DecryptingKey, EncryptingKey, Oaep};
    use crate::traits::PublicKeyParts;
    use crate::traits::{Decryptor, RandomizedDecryptor, RandomizedEncryptor};

    use alloc::string::String;
    use digest::{Digest, DynDigest, FixedOutputReset};
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;
    use rand_chacha::{
        rand_core::{RngCore, SeedableRng},
        ChaCha8Rng,
    };
    use sha1::Sha1;
    use sha2::{Sha224, Sha256, Sha384, Sha512};
    use sha3::{Sha3_256, Sha3_384, Sha3_512};

    fn get_private_key() -> RsaPrivateKey {
        // -----BEGIN RSA PRIVATE KEY-----
        // MIIEpAIBAAKCAQEA05e4TZikwmE47RtpWoEG6tkdVTvwYEG2LT/cUKBB4iK49FKW
        // icG4LF5xVU9d1p+i9LYVjPDb61eBGg/DJ+HyjnT+dNO8Fmweq9wbi1e5NMqL5bAL
        // TymXW8yZrK9BW1m7KKZ4K7QaLDwpdrPBjbre9i8AxrsiZkAJUJbAzGDSL+fvmH11
        // xqgbENlr8pICivEQ3HzBu8Q9Iq2rN5oM1dgHjMeA/1zWIJ3qNMkiz3hPdxfkKNdb
        // WuyP8w5fAUFRB2bi4KuNRzyE6HELK5gifD2wlTN600UvGeK5v7zN2BSKv2d2+lUn
        // debnWVbkUimuWpxGlJurHmIvDkj1ZSSoTtNIOwIDAQABAoIBAQDE5wxokWLJTGYI
        // KBkbUrTYOSEV30hqmtvoMeRY1zlYMg3Bt1VFbpNwHpcC12+wuS+Q4B0f4kgVMoH+
        // eaqXY6kvrmnY1+zRRN4p+hNb0U+Vc+NJ5FAx47dpgvWDADgmxVLomjl8Gga9IWNI
        // hjDZLowrtkPXq+9wDaldaFyUFImkb1S1MW9itdLDp/G70TTLNzU6RGg/3J2V02RY
        // 3iL2xEBX/nSgpDbEMI9z9NpC81xHrBanE41IOvyR5B3DoRJzguDA9RGbAiG0/GOd
        // a5w4F3pt6bUm69iMONeYLAf5ig79h31Qiq4nW5RpFcAuLhEG0XXXTsZ3f16A0SwF
        // PZx74eNBAoGBAPgnu/OkGHfHzFmuv0LtSynDLe/LjtloY9WwkKBaiTDdYkohydz5
        // g4Vo/foN9luEYqXyrJE9bFb5dVMr2OePsHvUBcqZpIS89Z8Bm73cs5M/K85wYwC0
        // 97EQEgxd+QGBWQZ8NdowYaVshjWlK1QnOzEnG0MR8Hld9gIeY1XhpC5hAoGBANpI
        // F84Aid028q3mo/9BDHPsNL8bT2vaOEMb/t4RzvH39u+nDl+AY6Ox9uFylv+xX+76
        // CRKgMluNH9ZaVZ5xe1uWHsNFBy4OxSA9A0QdKa9NZAVKBFB0EM8dp457YRnZCexm
        // 5q1iW/mVsnmks8W+fYlc18W5xMSX/ecwkW/NtOQbAoGAHabpz4AhKFbodSLrWbzv
        // CUt4NroVFKdjnoodjfujfwJFF2SYMV5jN9LG3lVCxca43ulzc1tqka33Nfv8TBcg
        // WHuKQZ5ASVgm5VwU1wgDMSoQOve07MWy/yZTccTc1zA0ihDXgn3bfR/NnaVh2wlh
        // CkuI92eyW1494hztc7qlmqECgYEA1zenyOQ9ChDIW/ABGIahaZamNxsNRrDFMl3j
        // AD+cxHSRU59qC32CQH8ShRy/huHzTaPX2DZ9EEln76fnrS4Ey7uLH0rrFl1XvT6K
        // /timJgLvMEvXTx/xBtUdRN2fUqXtI9odbSyCtOYFL+zVl44HJq2UzY4pVRDrNcxs
        // SUkQJqsCgYBSaNfPBzR5rrstLtTdZrjImRW1LRQeDEky9WsMDtCTYUGJTsTSfVO8
        // hkU82MpbRVBFIYx+GWIJwcZRcC7OCQoV48vMJllxMAAjqG/p00rVJ+nvA7et/nNu
        // BoB0er/UmDm4Ly/97EO9A0PKMOE5YbMq9s3t3RlWcsdrU7dvw+p2+A==
        // -----END RSA PRIVATE KEY-----

        RsaPrivateKey::from_components(
            BigUint::parse_bytes(b"00d397b84d98a4c26138ed1b695a8106ead91d553bf06041b62d3fdc50a041e222b8f4529689c1b82c5e71554f5dd69fa2f4b6158cf0dbeb57811a0fc327e1f28e74fe74d3bc166c1eabdc1b8b57b934ca8be5b00b4f29975bcc99acaf415b59bb28a6782bb41a2c3c2976b3c18dbadef62f00c6bb226640095096c0cc60d22fe7ef987d75c6a81b10d96bf292028af110dc7cc1bbc43d22adab379a0cd5d8078cc780ff5cd6209dea34c922cf784f7717e428d75b5aec8ff30e5f0141510766e2e0ab8d473c84e8710b2b98227c3db095337ad3452f19e2b9bfbccdd8148abf6776fa552775e6e75956e45229ae5a9c46949bab1e622f0e48f56524a84ed3483b", 16).unwrap(),
            BigUint::from_u64(65537).unwrap(),
            BigUint::parse_bytes(b"00c4e70c689162c94c660828191b52b4d8392115df486a9adbe831e458d73958320dc1b755456e93701e9702d76fb0b92f90e01d1fe248153281fe79aa9763a92fae69d8d7ecd144de29fa135bd14f9573e349e45031e3b76982f583003826c552e89a397c1a06bd2163488630d92e8c2bb643d7abef700da95d685c941489a46f54b5316f62b5d2c3a7f1bbd134cb37353a44683fdc9d95d36458de22f6c44057fe74a0a436c4308f73f4da42f35c47ac16a7138d483afc91e41dc3a1127382e0c0f5119b0221b4fc639d6b9c38177a6de9b526ebd88c38d7982c07f98a0efd877d508aae275b946915c02e2e1106d175d74ec6777f5e80d12c053d9c7be1e341", 16).unwrap(),
            vec![
                BigUint::parse_bytes(b"00f827bbf3a41877c7cc59aebf42ed4b29c32defcb8ed96863d5b090a05a8930dd624a21c9dcf9838568fdfa0df65b8462a5f2ac913d6c56f975532bd8e78fb07bd405ca99a484bcf59f019bbddcb3933f2bce706300b4f7b110120c5df9018159067c35da3061a56c8635a52b54273b31271b4311f0795df6021e6355e1a42e61",16).unwrap(),
                BigUint::parse_bytes(b"00da4817ce0089dd36f2ade6a3ff410c73ec34bf1b4f6bda38431bfede11cef1f7f6efa70e5f8063a3b1f6e17296ffb15feefa0912a0325b8d1fd65a559e717b5b961ec345072e0ec5203d03441d29af4d64054a04507410cf1da78e7b6119d909ec66e6ad625bf995b279a4b3c5be7d895cd7c5b9c4c497fde730916fcdb4e41b", 16).unwrap()
            ],
        ).unwrap()
    }

    #[test]
    fn test_encrypt_decrypt_oaep() {
        let priv_key = get_private_key();
        do_test_encrypt_decrypt_oaep::<Sha1>(&priv_key);
        do_test_encrypt_decrypt_oaep::<Sha224>(&priv_key);
        do_test_encrypt_decrypt_oaep::<Sha256>(&priv_key);
        do_test_encrypt_decrypt_oaep::<Sha384>(&priv_key);
        do_test_encrypt_decrypt_oaep::<Sha512>(&priv_key);
        do_test_encrypt_decrypt_oaep::<Sha3_256>(&priv_key);
        do_test_encrypt_decrypt_oaep::<Sha3_384>(&priv_key);
        do_test_encrypt_decrypt_oaep::<Sha3_512>(&priv_key);

        do_test_oaep_with_different_hashes::<Sha1, Sha1>(&priv_key);
        do_test_oaep_with_different_hashes::<Sha224, Sha1>(&priv_key);
        do_test_oaep_with_different_hashes::<Sha256, Sha1>(&priv_key);
        do_test_oaep_with_different_hashes::<Sha384, Sha1>(&priv_key);
        do_test_oaep_with_different_hashes::<Sha512, Sha1>(&priv_key);
        do_test_oaep_with_different_hashes::<Sha3_256, Sha1>(&priv_key);
        do_test_oaep_with_different_hashes::<Sha3_384, Sha1>(&priv_key);
        do_test_oaep_with_different_hashes::<Sha3_512, Sha1>(&priv_key);
    }

    fn get_label(rng: &mut ChaCha8Rng) -> Option<String> {
        const GEN_ASCII_STR_CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                0123456789=+";

        let mut buf = [0u8; 32];
        rng.fill_bytes(&mut buf);
        if buf[0] < (1 << 7) {
            for v in buf.iter_mut() {
                *v = GEN_ASCII_STR_CHARSET[(*v >> 2) as usize];
            }
            Some(core::str::from_utf8(&buf).unwrap().to_string())
        } else {
            None
        }
    }

    fn do_test_encrypt_decrypt_oaep<D: 'static + Digest + DynDigest + Send + Sync>(
        prk: &RsaPrivateKey,
    ) {
        let mut rng = ChaCha8Rng::from_seed([42; 32]);

        let k = prk.size();

        for i in 1..8 {
            let mut input = vec![0u8; i * 8];
            rng.fill_bytes(&mut input);

            if input.len() > k - 11 {
                input = input[0..k - 11].to_vec();
            }
            let label = get_label(&mut rng);

            let pub_key: RsaPublicKey = prk.into();

            let ciphertext = if let Some(ref label) = label {
                let padding = Oaep::new_with_label::<D, _>(label);
                pub_key.encrypt(&mut rng, padding, &input).unwrap()
            } else {
                let padding = Oaep::new::<D>();
                pub_key.encrypt(&mut rng, padding, &input).unwrap()
            };

            assert_ne!(input, ciphertext);
            let blind: bool = rng.next_u32() < (1 << 31);

            let padding = if let Some(ref label) = label {
                Oaep::new_with_label::<D, _>(label)
            } else {
                Oaep::new::<D>()
            };

            let plaintext = if blind {
                prk.decrypt(padding, &ciphertext).unwrap()
            } else {
                prk.decrypt_blinded(&mut rng, padding, &ciphertext).unwrap()
            };

            assert_eq!(input, plaintext);
        }
    }

    fn do_test_oaep_with_different_hashes<
        D: 'static + Digest + DynDigest + Send + Sync,
        U: 'static + Digest + DynDigest + Send + Sync,
    >(
        prk: &RsaPrivateKey,
    ) {
        let mut rng = ChaCha8Rng::from_seed([42; 32]);

        let k = prk.size();

        for i in 1..8 {
            let mut input = vec![0u8; i * 8];
            rng.fill_bytes(&mut input);

            if input.len() > k - 11 {
                input = input[0..k - 11].to_vec();
            }
            let label = get_label(&mut rng);

            let pub_key: RsaPublicKey = prk.into();

            let ciphertext = if let Some(ref label) = label {
                let padding = Oaep::new_with_mgf_hash_and_label::<D, U, _>(label);
                pub_key.encrypt(&mut rng, padding, &input).unwrap()
            } else {
                let padding = Oaep::new_with_mgf_hash::<D, U>();
                pub_key.encrypt(&mut rng, padding, &input).unwrap()
            };

            assert_ne!(input, ciphertext);
            let blind: bool = rng.next_u32() < (1 << 31);

            let padding = if let Some(ref label) = label {
                Oaep::new_with_mgf_hash_and_label::<D, U, _>(label)
            } else {
                Oaep::new_with_mgf_hash::<D, U>()
            };

            let plaintext = if blind {
                prk.decrypt(padding, &ciphertext).unwrap()
            } else {
                prk.decrypt_blinded(&mut rng, padding, &ciphertext).unwrap()
            };

            assert_eq!(input, plaintext);
        }
    }

    #[test]
    fn test_decrypt_oaep_invalid_hash() {
        let mut rng = ChaCha8Rng::from_seed([42; 32]);
        let priv_key = get_private_key();
        let pub_key: RsaPublicKey = (&priv_key).into();
        let ciphertext = pub_key
            .encrypt(&mut rng, Oaep::new::<Sha1>(), "a_plain_text".as_bytes())
            .unwrap();
        assert!(
            priv_key
                .decrypt_blinded(
                    &mut rng,
                    Oaep::new_with_label::<Sha1, _>("label"),
                    &ciphertext,
                )
                .is_err(),
            "decrypt should have failed on hash verification"
        );
    }

    #[test]
    fn test_encrypt_decrypt_oaep_traits() {
        let priv_key = get_private_key();
        do_test_encrypt_decrypt_oaep_traits::<Sha1>(&priv_key);
        do_test_encrypt_decrypt_oaep_traits::<Sha224>(&priv_key);
        do_test_encrypt_decrypt_oaep_traits::<Sha256>(&priv_key);
        do_test_encrypt_decrypt_oaep_traits::<Sha384>(&priv_key);
        do_test_encrypt_decrypt_oaep_traits::<Sha512>(&priv_key);
        do_test_encrypt_decrypt_oaep_traits::<Sha3_256>(&priv_key);
        do_test_encrypt_decrypt_oaep_traits::<Sha3_384>(&priv_key);
        do_test_encrypt_decrypt_oaep_traits::<Sha3_512>(&priv_key);

        do_test_oaep_with_different_hashes_traits::<Sha1, Sha1>(&priv_key);
        do_test_oaep_with_different_hashes_traits::<Sha224, Sha1>(&priv_key);
        do_test_oaep_with_different_hashes_traits::<Sha256, Sha1>(&priv_key);
        do_test_oaep_with_different_hashes_traits::<Sha384, Sha1>(&priv_key);
        do_test_oaep_with_different_hashes_traits::<Sha512, Sha1>(&priv_key);
        do_test_oaep_with_different_hashes_traits::<Sha3_256, Sha1>(&priv_key);
        do_test_oaep_with_different_hashes_traits::<Sha3_384, Sha1>(&priv_key);
        do_test_oaep_with_different_hashes_traits::<Sha3_512, Sha1>(&priv_key);
    }

    fn do_test_encrypt_decrypt_oaep_traits<D: Digest + FixedOutputReset>(prk: &RsaPrivateKey) {
        do_test_oaep_with_different_hashes_traits::<D, D>(prk);
    }

    fn do_test_oaep_with_different_hashes_traits<D: Digest, MGD: Digest + FixedOutputReset>(
        prk: &RsaPrivateKey,
    ) {
        let mut rng = ChaCha8Rng::from_seed([42; 32]);

        let k = prk.size();

        for i in 1..8 {
            let mut input = vec![0u8; i * 8];
            rng.fill_bytes(&mut input);

            if input.len() > k - 11 {
                input = input[0..k - 11].to_vec();
            }
            let label = get_label(&mut rng);

            let pub_key: RsaPublicKey = prk.into();

            let ciphertext = if let Some(ref label) = label {
                let encrypting_key =
                    EncryptingKey::<D, MGD>::new_with_label(pub_key, label.clone());
                encrypting_key.encrypt_with_rng(&mut rng, &input).unwrap()
            } else {
                let encrypting_key = EncryptingKey::<D, MGD>::new(pub_key);
                encrypting_key.encrypt_with_rng(&mut rng, &input).unwrap()
            };

            assert_ne!(input, ciphertext);
            let blind: bool = rng.next_u32() < (1 << 31);

            let decrypting_key = if let Some(ref label) = label {
                DecryptingKey::<D, MGD>::new_with_label(prk.clone(), label.clone())
            } else {
                DecryptingKey::<D, MGD>::new(prk.clone())
            };

            let plaintext = if blind {
                decrypting_key.decrypt(&ciphertext).unwrap()
            } else {
                decrypting_key
                    .decrypt_with_rng(&mut rng, &ciphertext)
                    .unwrap()
            };

            assert_eq!(input, plaintext);
        }
    }

    #[test]
    fn test_decrypt_oaep_invalid_hash_traits() {
        let mut rng = ChaCha8Rng::from_seed([42; 32]);
        let priv_key = get_private_key();
        let pub_key: RsaPublicKey = (&priv_key).into();
        let encrypting_key = EncryptingKey::<Sha1>::new(pub_key);
        let decrypting_key = DecryptingKey::<Sha1>::new_with_label(priv_key, "label");
        let ciphertext = encrypting_key
            .encrypt_with_rng(&mut rng, "a_plain_text".as_bytes())
            .unwrap();
        assert!(
            decrypting_key
                .decrypt_with_rng(&mut rng, &ciphertext)
                .is_err(),
            "decrypt should have failed on hash verification"
        );
    }
}
