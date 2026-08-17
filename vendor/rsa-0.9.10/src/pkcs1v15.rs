//! PKCS#1 v1.5 support as described in [RFC8017 § 8.2].
//!
//! # Usage
//!
//! See [code example in the toplevel rustdoc](../index.html#pkcs1-v15-signatures).
//!
//! [RFC8017 § 8.2]: https://datatracker.ietf.org/doc/html/rfc8017#section-8.2

mod decrypting_key;
mod encrypting_key;
mod signature;
mod signing_key;
mod verifying_key;

pub use self::{
    decrypting_key::DecryptingKey, encrypting_key::EncryptingKey, signature::Signature,
    signing_key::SigningKey, verifying_key::VerifyingKey,
};

use alloc::{boxed::Box, vec::Vec};
use core::fmt::Debug;
use digest::Digest;
use num_bigint::BigUint;
use pkcs8::AssociatedOid;
use rand_core::CryptoRngCore;
use zeroize::Zeroizing;

use crate::algorithms::pad::{uint_to_be_pad, uint_to_zeroizing_be_pad};
use crate::algorithms::pkcs1v15::*;
use crate::algorithms::rsa::{rsa_decrypt_and_check, rsa_encrypt};
use crate::errors::{Error, Result};
use crate::key::{self, RsaPrivateKey, RsaPublicKey};
use crate::traits::{PaddingScheme, PublicKeyParts, SignatureScheme};

/// Encryption using PKCS#1 v1.5 padding.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Pkcs1v15Encrypt;

impl PaddingScheme for Pkcs1v15Encrypt {
    fn decrypt<Rng: CryptoRngCore>(
        self,
        rng: Option<&mut Rng>,
        priv_key: &RsaPrivateKey,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>> {
        decrypt(rng, priv_key, ciphertext)
    }

    fn encrypt<Rng: CryptoRngCore>(
        self,
        rng: &mut Rng,
        pub_key: &RsaPublicKey,
        msg: &[u8],
    ) -> Result<Vec<u8>> {
        encrypt(rng, pub_key, msg)
    }
}

/// `RSASSA-PKCS1-v1_5`: digital signatures using PKCS#1 v1.5 padding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pkcs1v15Sign {
    /// Length of hash to use.
    pub hash_len: Option<usize>,

    /// Prefix.
    pub prefix: Box<[u8]>,
}

impl Pkcs1v15Sign {
    /// Create new PKCS#1 v1.5 padding for the given digest.
    ///
    /// The digest must have an [`AssociatedOid`]. Make sure to enable the `oid`
    /// feature of the relevant digest crate.
    pub fn new<D>() -> Self
    where
        D: Digest + AssociatedOid,
    {
        Self {
            hash_len: Some(<D as Digest>::output_size()),
            prefix: pkcs1v15_generate_prefix::<D>().into_boxed_slice(),
        }
    }

    /// Create new PKCS#1 v1.5 padding for computing an unprefixed signature.
    ///
    /// This sets `hash_len` to `None` and uses an empty `prefix`.
    pub fn new_unprefixed() -> Self {
        Self {
            hash_len: None,
            prefix: Box::new([]),
        }
    }

    /// Create new PKCS#1 v1.5 padding for computing an unprefixed signature.
    ///
    /// This sets `hash_len` to `None` and uses an empty `prefix`.
    #[deprecated(since = "0.9.0", note = "use Pkcs1v15Sign::new_unprefixed instead")]
    pub fn new_raw() -> Self {
        Self::new_unprefixed()
    }
}

impl SignatureScheme for Pkcs1v15Sign {
    fn sign<Rng: CryptoRngCore>(
        self,
        rng: Option<&mut Rng>,
        priv_key: &RsaPrivateKey,
        hashed: &[u8],
    ) -> Result<Vec<u8>> {
        if let Some(hash_len) = self.hash_len {
            if hashed.len() != hash_len {
                return Err(Error::InputNotHashed);
            }
        }

        sign(rng, priv_key, &self.prefix, hashed)
    }

    fn verify(self, pub_key: &RsaPublicKey, hashed: &[u8], sig: &[u8]) -> Result<()> {
        if let Some(hash_len) = self.hash_len {
            if hashed.len() != hash_len {
                return Err(Error::InputNotHashed);
            }
        }

        verify(
            pub_key,
            self.prefix.as_ref(),
            hashed,
            &BigUint::from_bytes_be(sig),
            sig.len(),
        )
    }
}

/// Encrypts the given message with RSA and the padding
/// scheme from PKCS#1 v1.5.  The message must be no longer than the
/// length of the public modulus minus 11 bytes.
#[inline]
fn encrypt<R: CryptoRngCore + ?Sized>(
    rng: &mut R,
    pub_key: &RsaPublicKey,
    msg: &[u8],
) -> Result<Vec<u8>> {
    key::check_public(pub_key)?;

    let em = pkcs1v15_encrypt_pad(rng, msg, pub_key.size())?;
    let int = Zeroizing::new(BigUint::from_bytes_be(&em));
    uint_to_be_pad(rsa_encrypt(pub_key, &int)?, pub_key.size())
}

/// Decrypts a plaintext using RSA and the padding scheme from PKCS#1 v1.5.
///
/// If an `rng` is passed, it uses RSA blinding to avoid timing side-channel attacks.
///
/// Note that whether this function returns an error or not discloses secret
/// information. If an attacker can cause this function to run repeatedly and
/// learn whether each instance returned an error then they can decrypt and
/// forge signatures as if they had the private key. See
/// `decrypt_session_key` for a way of solving this problem.
#[inline]
fn decrypt<R: CryptoRngCore + ?Sized>(
    rng: Option<&mut R>,
    priv_key: &RsaPrivateKey,
    ciphertext: &[u8],
) -> Result<Vec<u8>> {
    key::check_public(priv_key)?;

    let em = rsa_decrypt_and_check(priv_key, rng, &BigUint::from_bytes_be(ciphertext))?;
    let em = uint_to_zeroizing_be_pad(em, priv_key.size())?;

    pkcs1v15_encrypt_unpad(em, priv_key.size())
}

/// Calculates the signature of hashed using
/// RSASSA-PKCS1-V1_5-SIGN from RSA PKCS#1 v1.5. Note that `hashed` must
/// be the result of hashing the input message using the given hash
/// function. If hash is `None`, hashed is signed directly. This isn't
/// advisable except for interoperability.
///
/// If `rng` is not `None` then RSA blinding will be used to avoid timing
/// side-channel attacks.
///
/// This function is deterministic. Thus, if the set of possible
/// messages is small, an attacker may be able to build a map from
/// messages to signatures and identify the signed messages. As ever,
/// signatures provide authenticity, not confidentiality.
#[inline]
fn sign<R: CryptoRngCore + ?Sized>(
    rng: Option<&mut R>,
    priv_key: &RsaPrivateKey,
    prefix: &[u8],
    hashed: &[u8],
) -> Result<Vec<u8>> {
    let em = pkcs1v15_sign_pad(prefix, hashed, priv_key.size())?;

    uint_to_zeroizing_be_pad(
        rsa_decrypt_and_check(priv_key, rng, &BigUint::from_bytes_be(&em))?,
        priv_key.size(),
    )
}

/// Verifies an RSA PKCS#1 v1.5 signature.
#[inline]
fn verify(
    pub_key: &RsaPublicKey,
    prefix: &[u8],
    hashed: &[u8],
    sig: &BigUint,
    sig_len: usize,
) -> Result<()> {
    if sig >= pub_key.n() || sig_len != pub_key.size() {
        return Err(Error::Verification);
    }

    let em = uint_to_be_pad(rsa_encrypt(pub_key, sig)?, pub_key.size())?;

    pkcs1v15_sign_unpad(prefix, hashed, &em, pub_key.size())
}

mod oid {
    use const_oid::ObjectIdentifier;

    /// A trait which associates an RSA-specific OID with a type.
    pub trait RsaSignatureAssociatedOid {
        /// The OID associated with this type.
        const OID: ObjectIdentifier;
    }

    #[cfg(feature = "sha1")]
    impl RsaSignatureAssociatedOid for sha1::Sha1 {
        const OID: ObjectIdentifier =
            const_oid::ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.5");
    }

    #[cfg(feature = "sha2")]
    impl RsaSignatureAssociatedOid for sha2::Sha224 {
        const OID: ObjectIdentifier =
            const_oid::ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.14");
    }

    #[cfg(feature = "sha2")]
    impl RsaSignatureAssociatedOid for sha2::Sha256 {
        const OID: ObjectIdentifier =
            const_oid::ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.11");
    }

    #[cfg(feature = "sha2")]
    impl RsaSignatureAssociatedOid for sha2::Sha384 {
        const OID: ObjectIdentifier =
            const_oid::ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.12");
    }

    #[cfg(feature = "sha2")]
    impl RsaSignatureAssociatedOid for sha2::Sha512 {
        const OID: ObjectIdentifier =
            const_oid::ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.13");
    }
}

pub use oid::RsaSignatureAssociatedOid;

#[cfg(test)]
mod tests {
    use super::*;
    use ::signature::{
        hazmat::{PrehashSigner, PrehashVerifier},
        DigestSigner, DigestVerifier, Keypair, RandomizedDigestSigner, RandomizedSigner,
        SignatureEncoding, Signer, Verifier,
    };
    use base64ct::{Base64, Encoding};
    use hex_literal::hex;
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;
    use num_traits::Num;
    use rand_chacha::{
        rand_core::{RngCore, SeedableRng},
        ChaCha8Rng,
    };
    use sha1::{Digest, Sha1};
    use sha2::Sha256;
    use sha3::Sha3_256;

    use crate::traits::{
        Decryptor, EncryptingKeypair, PublicKeyParts, RandomizedDecryptor, RandomizedEncryptor,
    };
    use crate::{RsaPrivateKey, RsaPublicKey};

    fn get_private_key() -> RsaPrivateKey {
        // In order to generate new test vectors you'll need the PEM form of this key:
        // -----BEGIN RSA PRIVATE KEY-----
        // MIIBOgIBAAJBALKZD0nEffqM1ACuak0bijtqE2QrI/KLADv7l3kK3ppMyCuLKoF0
        // fd7Ai2KW5ToIwzFofvJcS/STa6HA5gQenRUCAwEAAQJBAIq9amn00aS0h/CrjXqu
        // /ThglAXJmZhOMPVn4eiu7/ROixi9sex436MaVeMqSNf7Ex9a8fRNfWss7Sqd9eWu
        // RTUCIQDasvGASLqmjeffBNLTXV2A5g4t+kLVCpsEIZAycV5GswIhANEPLmax0ME/
        // EO+ZJ79TJKN5yiGBRsv5yvx5UiHxajEXAiAhAol5N4EUyq6I9w1rYdhPMGpLfk7A
        // IU2snfRJ6Nq2CQIgFrPsWRCkV+gOYcajD17rEqmuLrdIRexpg8N1DOSXoJ8CIGlS
        // tAboUGBxTDq3ZroNism3DaMIbKPyYrAqhKov1h5V
        // -----END RSA PRIVATE KEY-----

        RsaPrivateKey::from_components(
            BigUint::from_str_radix("9353930466774385905609975137998169297361893554149986716853295022578535724979677252958524466350471210367835187480748268864277464700638583474144061408845077", 10).unwrap(),
            BigUint::from_u64(65537).unwrap(),
            BigUint::from_str_radix("7266398431328116344057699379749222532279343923819063639497049039389899328538543087657733766554155839834519529439851673014800261285757759040931985506583861", 10).unwrap(),
            vec![
                BigUint::from_str_radix("98920366548084643601728869055592650835572950932266967461790948584315647051443",10).unwrap(),
                BigUint::from_str_radix("94560208308847015747498523884063394671606671904944666360068158221458669711639", 10).unwrap()
            ],
        ).unwrap()
    }

    #[test]
    fn test_decrypt_pkcs1v15() {
        let priv_key = get_private_key();

        let tests = [[
	    "gIcUIoVkD6ATMBk/u/nlCZCCWRKdkfjCgFdo35VpRXLduiKXhNz1XupLLzTXAybEq15juc+EgY5o0DHv/nt3yg==",
	    "x",
	], [
	    "Y7TOCSqofGhkRb+jaVRLzK8xw2cSo1IVES19utzv6hwvx+M8kFsoWQm5DzBeJCZTCVDPkTpavUuEbgp8hnUGDw==",
	    "testing.",
	], [
	    "arReP9DJtEVyV2Dg3dDp4c/PSk1O6lxkoJ8HcFupoRorBZG+7+1fDAwT1olNddFnQMjmkb8vxwmNMoTAT/BFjQ==",
	    "testing.\n",
	], [
	"WtaBXIoGC54+vH0NH0CHHE+dRDOsMc/6BrfFu2lEqcKL9+uDuWaf+Xj9mrbQCjjZcpQuX733zyok/jsnqe/Ftw==",
		"01234567890123456789012345678901234567890123456789012",
	]];

        for test in &tests {
            let out = priv_key
                .decrypt(Pkcs1v15Encrypt, &Base64::decode_vec(test[0]).unwrap())
                .unwrap();
            assert_eq!(out, test[1].as_bytes());
        }
    }

    #[test]
    fn test_encrypt_decrypt_pkcs1v15() {
        let mut rng = ChaCha8Rng::from_seed([42; 32]);
        let priv_key = get_private_key();
        let k = priv_key.size();

        for i in 1..100 {
            let mut input = vec![0u8; i * 8];
            rng.fill_bytes(&mut input);
            if input.len() > k - 11 {
                input = input[0..k - 11].to_vec();
            }

            let pub_key: RsaPublicKey = priv_key.clone().into();
            let ciphertext = encrypt(&mut rng, &pub_key, &input).unwrap();
            assert_ne!(input, ciphertext);

            let blind: bool = rng.next_u32() < (1u32 << 31);
            let blinder = if blind { Some(&mut rng) } else { None };
            let plaintext = decrypt(blinder, &priv_key, &ciphertext).unwrap();
            assert_eq!(input, plaintext);
        }
    }

    #[test]
    fn test_decrypt_pkcs1v15_traits() {
        let priv_key = get_private_key();
        let decrypting_key = DecryptingKey::new(priv_key);

        let tests = [[
	    "gIcUIoVkD6ATMBk/u/nlCZCCWRKdkfjCgFdo35VpRXLduiKXhNz1XupLLzTXAybEq15juc+EgY5o0DHv/nt3yg==",
	    "x",
	], [
	    "Y7TOCSqofGhkRb+jaVRLzK8xw2cSo1IVES19utzv6hwvx+M8kFsoWQm5DzBeJCZTCVDPkTpavUuEbgp8hnUGDw==",
	    "testing.",
	], [
	    "arReP9DJtEVyV2Dg3dDp4c/PSk1O6lxkoJ8HcFupoRorBZG+7+1fDAwT1olNddFnQMjmkb8vxwmNMoTAT/BFjQ==",
	    "testing.\n",
	], [
	"WtaBXIoGC54+vH0NH0CHHE+dRDOsMc/6BrfFu2lEqcKL9+uDuWaf+Xj9mrbQCjjZcpQuX733zyok/jsnqe/Ftw==",
		"01234567890123456789012345678901234567890123456789012",
	]];

        for test in &tests {
            let out = decrypting_key
                .decrypt(&Base64::decode_vec(test[0]).unwrap())
                .unwrap();
            assert_eq!(out, test[1].as_bytes());
        }
    }

    #[test]
    fn test_encrypt_decrypt_pkcs1v15_traits() {
        let mut rng = ChaCha8Rng::from_seed([42; 32]);
        let priv_key = get_private_key();
        let k = priv_key.size();
        let decrypting_key = DecryptingKey::new(priv_key);

        for i in 1..100 {
            let mut input = vec![0u8; i * 8];
            rng.fill_bytes(&mut input);
            if input.len() > k - 11 {
                input = input[0..k - 11].to_vec();
            }

            let encrypting_key = decrypting_key.encrypting_key();
            let ciphertext = encrypting_key.encrypt_with_rng(&mut rng, &input).unwrap();
            assert_ne!(input, ciphertext);

            let blind: bool = rng.next_u32() < (1u32 << 31);
            let plaintext = if blind {
                decrypting_key
                    .decrypt_with_rng(&mut rng, &ciphertext)
                    .unwrap()
            } else {
                decrypting_key.decrypt(&ciphertext).unwrap()
            };
            assert_eq!(input, plaintext);
        }
    }

    #[test]
    fn test_sign_pkcs1v15() {
        let priv_key = get_private_key();

        let tests = [(
            "Test.\n",
            hex!(
                "a4f3fa6ea93bcdd0c57be020c1193ecbfd6f200a3d95c409769b029578fa0e33"
                "6ad9a347600e40d3ae823b8c7e6bad88cc07c1d54c3a1523cbbb6d58efc362ae"
            ),
        )];

        for (text, expected) in &tests {
            let digest = Sha1::digest(text.as_bytes()).to_vec();

            let out = priv_key.sign(Pkcs1v15Sign::new::<Sha1>(), &digest).unwrap();
            assert_ne!(out, digest);
            assert_eq!(out, expected);

            let mut rng = ChaCha8Rng::from_seed([42; 32]);
            let out2 = priv_key
                .sign_with_rng(&mut rng, Pkcs1v15Sign::new::<Sha1>(), &digest)
                .unwrap();
            assert_eq!(out2, expected);
        }
    }

    #[test]
    fn test_sign_pkcs1v15_signer() {
        let priv_key = get_private_key();

        let tests = [(
            "Test.\n",
            hex!(
                "a4f3fa6ea93bcdd0c57be020c1193ecbfd6f200a3d95c409769b029578fa0e33"
                "6ad9a347600e40d3ae823b8c7e6bad88cc07c1d54c3a1523cbbb6d58efc362ae"
            ),
        )];

        let signing_key = SigningKey::<Sha1>::new(priv_key);

        for (text, expected) in &tests {
            let out = signing_key.sign(text.as_bytes()).to_bytes();
            assert_ne!(out.as_ref(), text.as_bytes());
            assert_ne!(out.as_ref(), &Sha1::digest(text.as_bytes()).to_vec());
            assert_eq!(out.as_ref(), expected);

            let mut rng = ChaCha8Rng::from_seed([42; 32]);
            let out2 = signing_key
                .sign_with_rng(&mut rng, text.as_bytes())
                .to_bytes();
            assert_eq!(out2.as_ref(), expected);
        }
    }

    #[test]
    fn test_sign_pkcs1v15_signer_sha2_256() {
        let priv_key = get_private_key();

        let tests = [(
            "Test.\n",
            hex!(
                "2ffae3f3e130287b3a1dcb320e46f52e8f3f7969b646932273a7e3a6f2a182ea"
                "02d42875a7ffa4a148aa311f9e4b562e4e13a2223fb15f4e5bf5f2b206d9451b"
            ),
        )];

        let signing_key = SigningKey::<Sha256>::new(priv_key);

        for (text, expected) in &tests {
            let out = signing_key.sign(text.as_bytes()).to_bytes();
            assert_ne!(out.as_ref(), text.as_bytes());
            assert_eq!(out.as_ref(), expected);

            let mut rng = ChaCha8Rng::from_seed([42; 32]);
            let out2 = signing_key
                .sign_with_rng(&mut rng, text.as_bytes())
                .to_bytes();
            assert_eq!(out2.as_ref(), expected);
        }
    }

    #[test]
    fn test_sign_pkcs1v15_signer_sha3_256() {
        let priv_key = get_private_key();

        let tests = [(
            "Test.\n",
            hex!(
                "55e9fba3354dfb51d2c8111794ea552c86afc2cab154652c03324df8c2c51ba7"
                "2ff7c14de59a6f9ba50d90c13a7537cc3011948369f1f0ec4a49d21eb7e723f9"
            ),
        )];

        let signing_key = SigningKey::<Sha3_256>::new(priv_key);

        for (text, expected) in &tests {
            let out = signing_key.sign(text.as_bytes()).to_bytes();
            assert_ne!(out.as_ref(), text.as_bytes());
            assert_eq!(out.as_ref(), expected);

            let mut rng = ChaCha8Rng::from_seed([42; 32]);
            let out2 = signing_key
                .sign_with_rng(&mut rng, text.as_bytes())
                .to_bytes();
            assert_eq!(out2.as_ref(), expected);
        }
    }

    #[test]
    fn test_sign_pkcs1v15_digest_signer() {
        let priv_key = get_private_key();

        let tests = [(
            "Test.\n",
            hex!(
                "a4f3fa6ea93bcdd0c57be020c1193ecbfd6f200a3d95c409769b029578fa0e33"
                "6ad9a347600e40d3ae823b8c7e6bad88cc07c1d54c3a1523cbbb6d58efc362ae"
            ),
        )];

        let signing_key = SigningKey::new(priv_key);

        for (text, expected) in &tests {
            let mut digest = Sha1::new();
            digest.update(text.as_bytes());
            let out = signing_key.sign_digest(digest).to_bytes();
            assert_ne!(out.as_ref(), text.as_bytes());
            assert_ne!(out.as_ref(), &Sha1::digest(text.as_bytes()).to_vec());
            assert_eq!(out.as_ref(), expected);

            let mut rng = ChaCha8Rng::from_seed([42; 32]);
            let mut digest = Sha1::new();
            digest.update(text.as_bytes());
            let out2 = signing_key
                .sign_digest_with_rng(&mut rng, digest)
                .to_bytes();
            assert_eq!(out2.as_ref(), expected);
        }
    }

    #[test]
    fn test_verify_pkcs1v15() {
        let priv_key = get_private_key();

        let tests = [
            (
                "Test.\n",
                hex!(
                    "a4f3fa6ea93bcdd0c57be020c1193ecbfd6f200a3d95c409769b029578fa0e33"
                    "6ad9a347600e40d3ae823b8c7e6bad88cc07c1d54c3a1523cbbb6d58efc362ae"
                ),
                true,
            ),
            (
                "Test.\n",
                hex!(
                    "a4f3fa6ea93bcdd0c57be020c1193ecbfd6f200a3d95c409769b029578fa0e33"
                    "6ad9a347600e40d3ae823b8c7e6bad88cc07c1d54c3a1523cbbb6d58efc362af"
                ),
                false,
            ),
        ];
        let pub_key: RsaPublicKey = priv_key.into();

        for (text, sig, expected) in &tests {
            let digest = Sha1::digest(text.as_bytes()).to_vec();

            let result = pub_key.verify(Pkcs1v15Sign::new::<Sha1>(), &digest, sig);
            match expected {
                true => result.expect("failed to verify"),
                false => {
                    result.expect_err("expected verifying error");
                }
            }
        }
    }

    #[test]
    fn test_verify_pkcs1v15_signer() {
        let priv_key = get_private_key();

        let tests = [
            (
                "Test.\n",
                hex!(
                    "a4f3fa6ea93bcdd0c57be020c1193ecbfd6f200a3d95c409769b029578fa0e33"
                    "6ad9a347600e40d3ae823b8c7e6bad88cc07c1d54c3a1523cbbb6d58efc362ae"
                ),
                true,
            ),
            (
                "Test.\n",
                hex!(
                    "a4f3fa6ea93bcdd0c57be020c1193ecbfd6f200a3d95c409769b029578fa0e33"
                    "6ad9a347600e40d3ae823b8c7e6bad88cc07c1d54c3a1523cbbb6d58efc362af"
                ),
                false,
            ),
        ];
        let pub_key: RsaPublicKey = priv_key.into();
        let verifying_key = VerifyingKey::<Sha1>::new(pub_key);

        for (text, sig, expected) in &tests {
            let result = verifying_key.verify(
                text.as_bytes(),
                &Signature::try_from(sig.as_slice()).unwrap(),
            );
            match expected {
                true => result.expect("failed to verify"),
                false => {
                    result.expect_err("expected verifying error");
                }
            }
        }
    }

    #[test]
    fn test_verify_pkcs1v15_digest_signer() {
        let priv_key = get_private_key();

        let tests = [
            (
                "Test.\n",
                hex!(
                    "a4f3fa6ea93bcdd0c57be020c1193ecbfd6f200a3d95c409769b029578fa0e33"
                    "6ad9a347600e40d3ae823b8c7e6bad88cc07c1d54c3a1523cbbb6d58efc362ae"
                ),
                true,
            ),
            (
                "Test.\n",
                hex!(
                    "a4f3fa6ea93bcdd0c57be020c1193ecbfd6f200a3d95c409769b029578fa0e33"
                    "6ad9a347600e40d3ae823b8c7e6bad88cc07c1d54c3a1523cbbb6d58efc362af"
                ),
                false,
            ),
        ];
        let pub_key: RsaPublicKey = priv_key.into();
        let verifying_key = VerifyingKey::new(pub_key);

        for (text, sig, expected) in &tests {
            let mut digest = Sha1::new();
            digest.update(text.as_bytes());
            let result =
                verifying_key.verify_digest(digest, &Signature::try_from(sig.as_slice()).unwrap());
            match expected {
                true => result.expect("failed to verify"),
                false => {
                    result.expect_err("expected verifying error");
                }
            }
        }
    }

    #[test]
    fn test_unpadded_signature() {
        let msg = b"Thu Dec 19 18:06:16 EST 2013\n";
        let expected_sig = Base64::decode_vec("pX4DR8azytjdQ1rtUiC040FjkepuQut5q2ZFX1pTjBrOVKNjgsCDyiJDGZTCNoh9qpXYbhl7iEym30BWWwuiZg==").unwrap();
        let priv_key = get_private_key();

        let sig = priv_key.sign(Pkcs1v15Sign::new_unprefixed(), msg).unwrap();
        assert_eq!(expected_sig, sig);

        let pub_key: RsaPublicKey = priv_key.into();
        pub_key
            .verify(Pkcs1v15Sign::new_unprefixed(), msg, &sig)
            .expect("failed to verify");
    }

    #[test]
    fn test_unpadded_signature_hazmat() {
        let msg = b"Thu Dec 19 18:06:16 EST 2013\n";
        let expected_sig = Base64::decode_vec("pX4DR8azytjdQ1rtUiC040FjkepuQut5q2ZFX1pTjBrOVKNjgsCDyiJDGZTCNoh9qpXYbhl7iEym30BWWwuiZg==").unwrap();
        let priv_key = get_private_key();

        let signing_key = SigningKey::<Sha1>::new_unprefixed(priv_key);
        let sig = signing_key
            .sign_prehash(msg)
            .expect("Failure during sign")
            .to_bytes();
        assert_eq!(sig.as_ref(), expected_sig);

        let verifying_key = signing_key.verifying_key();
        verifying_key
            .verify_prehash(msg, &Signature::try_from(expected_sig.as_slice()).unwrap())
            .expect("failed to verify");
    }
}
