//! PKCS#1 and PKCS#8 encoding support.
//!
//! Note: PKCS#1 support is achieved through a blanket impl of the
//! `pkcs1` crate's traits for types which impl the `pkcs8` crate's traits.

use crate::{
    traits::{PrivateKeyParts, PublicKeyParts},
    BigUint, RsaPrivateKey, RsaPublicKey,
};
use core::convert::{TryFrom, TryInto};
use pkcs8::{der::Encode, Document, EncodePrivateKey, EncodePublicKey, SecretDocument};
use zeroize::Zeroizing;

/// Verify that the `AlgorithmIdentifier` for a key is correct.
fn verify_algorithm_id(algorithm: &pkcs8::AlgorithmIdentifierRef) -> pkcs8::spki::Result<()> {
    algorithm.assert_algorithm_oid(pkcs1::ALGORITHM_OID)?;

    if algorithm.parameters_any()? != pkcs8::der::asn1::Null.into() {
        return Err(pkcs8::spki::Error::KeyMalformed);
    }

    Ok(())
}

impl TryFrom<pkcs8::PrivateKeyInfo<'_>> for RsaPrivateKey {
    type Error = pkcs8::Error;

    fn try_from(private_key_info: pkcs8::PrivateKeyInfo<'_>) -> pkcs8::Result<Self> {
        verify_algorithm_id(&private_key_info.algorithm)?;

        let pkcs1_key = pkcs1::RsaPrivateKey::try_from(private_key_info.private_key)?;

        // Multi-prime RSA keys not currently supported
        if pkcs1_key.version() != pkcs1::Version::TwoPrime {
            return Err(pkcs1::Error::Version.into());
        }

        let n = BigUint::from_bytes_be(pkcs1_key.modulus.as_bytes());
        let e = BigUint::from_bytes_be(pkcs1_key.public_exponent.as_bytes());
        let d = BigUint::from_bytes_be(pkcs1_key.private_exponent.as_bytes());
        let prime1 = BigUint::from_bytes_be(pkcs1_key.prime1.as_bytes());
        let prime2 = BigUint::from_bytes_be(pkcs1_key.prime2.as_bytes());
        let primes = vec![prime1, prime2];
        RsaPrivateKey::from_components(n, e, d, primes).map_err(|_| pkcs8::Error::KeyMalformed)
    }
}

impl TryFrom<pkcs8::SubjectPublicKeyInfoRef<'_>> for RsaPublicKey {
    type Error = pkcs8::spki::Error;

    fn try_from(spki: pkcs8::SubjectPublicKeyInfoRef<'_>) -> pkcs8::spki::Result<Self> {
        verify_algorithm_id(&spki.algorithm)?;

        let pkcs1_key = pkcs1::RsaPublicKey::try_from(
            spki.subject_public_key
                .as_bytes()
                .ok_or(pkcs8::spki::Error::KeyMalformed)?,
        )?;
        let n = BigUint::from_bytes_be(pkcs1_key.modulus.as_bytes());
        let e = BigUint::from_bytes_be(pkcs1_key.public_exponent.as_bytes());
        RsaPublicKey::new(n, e).map_err(|_| pkcs8::spki::Error::KeyMalformed)
    }
}

impl EncodePrivateKey for RsaPrivateKey {
    fn to_pkcs8_der(&self) -> pkcs8::Result<SecretDocument> {
        // Check if the key is multi prime
        if self.primes.len() > 2 {
            return Err(pkcs1::Error::Version.into());
        }

        let modulus = self.n().to_bytes_be();
        let public_exponent = self.e().to_bytes_be();
        let private_exponent = Zeroizing::new(self.d().to_bytes_be());
        let prime1 = Zeroizing::new(self.primes[0].to_bytes_be());
        let prime2 = Zeroizing::new(self.primes[1].to_bytes_be());
        let exponent1 = Zeroizing::new((self.d() % (&self.primes[0] - 1u8)).to_bytes_be());
        let exponent2 = Zeroizing::new((self.d() % (&self.primes[1] - 1u8)).to_bytes_be());
        let coefficient = Zeroizing::new(
            self.crt_coefficient()
                .ok_or(pkcs1::Error::Crypto)?
                .to_bytes_be(),
        );

        let private_key = pkcs1::RsaPrivateKey {
            modulus: pkcs1::UintRef::new(&modulus)?,
            public_exponent: pkcs1::UintRef::new(&public_exponent)?,
            private_exponent: pkcs1::UintRef::new(&private_exponent)?,
            prime1: pkcs1::UintRef::new(&prime1)?,
            prime2: pkcs1::UintRef::new(&prime2)?,
            exponent1: pkcs1::UintRef::new(&exponent1)?,
            exponent2: pkcs1::UintRef::new(&exponent2)?,
            coefficient: pkcs1::UintRef::new(&coefficient)?,
            other_prime_infos: None,
        }
        .to_der()?;

        pkcs8::PrivateKeyInfo::new(pkcs1::ALGORITHM_ID, private_key.as_ref()).try_into()
    }
}

impl EncodePublicKey for RsaPublicKey {
    fn to_public_key_der(&self) -> pkcs8::spki::Result<Document> {
        let modulus = self.n().to_bytes_be();
        let public_exponent = self.e().to_bytes_be();

        let subject_public_key = pkcs1::RsaPublicKey {
            modulus: pkcs1::UintRef::new(&modulus)?,
            public_exponent: pkcs1::UintRef::new(&public_exponent)?,
        }
        .to_der()?;

        pkcs8::SubjectPublicKeyInfoRef {
            algorithm: pkcs1::ALGORITHM_ID,
            subject_public_key: pkcs8::der::asn1::BitStringRef::new(
                0,
                subject_public_key.as_ref(),
            )?,
        }
        .try_into()
    }
}
