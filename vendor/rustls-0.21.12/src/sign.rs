use crate::enums::{SignatureAlgorithm, SignatureScheme};
use crate::error::Error;
use crate::key;
use crate::x509::{wrap_in_asn1_len, wrap_in_sequence};

use ring::io::der;
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{self, EcdsaKeyPair, Ed25519KeyPair, RsaKeyPair};

use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;

/// An abstract signing key.
pub trait SigningKey: Send + Sync {
    /// Choose a `SignatureScheme` from those offered.
    ///
    /// Expresses the choice by returning something that implements `Signer`,
    /// using the chosen scheme.
    fn choose_scheme(&self, offered: &[SignatureScheme]) -> Option<Box<dyn Signer>>;

    /// What kind of key we have.
    fn algorithm(&self) -> SignatureAlgorithm;
}

/// A thing that can sign a message.
pub trait Signer: Send + Sync {
    /// Signs `message` using the selected scheme.
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error>;

    /// Reveals which scheme will be used when you call `sign()`.
    fn scheme(&self) -> SignatureScheme;
}

/// A packaged-together certificate chain, matching `SigningKey` and
/// optional stapled OCSP response and/or SCT list.
#[derive(Clone)]
pub struct CertifiedKey {
    /// The certificate chain.
    pub cert: Vec<key::Certificate>,

    /// The certified key.
    pub key: Arc<dyn SigningKey>,

    /// An optional OCSP response from the certificate issuer,
    /// attesting to its continued validity.
    pub ocsp: Option<Vec<u8>>,

    /// An optional collection of SCTs from CT logs, proving the
    /// certificate is included on those logs.  This must be
    /// a `SignedCertificateTimestampList` encoding; see RFC6962.
    pub sct_list: Option<Vec<u8>>,
}

impl CertifiedKey {
    /// Make a new CertifiedKey, with the given chain and key.
    ///
    /// The cert chain must not be empty. The first certificate in the chain
    /// must be the end-entity certificate.
    pub fn new(cert: Vec<key::Certificate>, key: Arc<dyn SigningKey>) -> Self {
        Self {
            cert,
            key,
            ocsp: None,
            sct_list: None,
        }
    }

    /// The end-entity certificate.
    pub fn end_entity_cert(&self) -> Result<&key::Certificate, SignError> {
        self.cert.first().ok_or(SignError(()))
    }
}

/// Parse `der` as any supported key encoding/type, returning
/// the first which works.
pub fn any_supported_type(der: &key::PrivateKey) -> Result<Arc<dyn SigningKey>, SignError> {
    if let Ok(rsa) = RsaSigningKey::new(der) {
        Ok(Arc::new(rsa))
    } else if let Ok(ecdsa) = any_ecdsa_type(der) {
        Ok(ecdsa)
    } else {
        any_eddsa_type(der)
    }
}

/// Parse `der` as any ECDSA key type, returning the first which works.
///
/// Both SEC1 (PEM section starting with 'BEGIN EC PRIVATE KEY') and PKCS8
/// (PEM section starting with 'BEGIN PRIVATE KEY') encodings are supported.
pub fn any_ecdsa_type(der: &key::PrivateKey) -> Result<Arc<dyn SigningKey>, SignError> {
    if let Ok(ecdsa_p256) = EcdsaSigningKey::new(
        der,
        SignatureScheme::ECDSA_NISTP256_SHA256,
        &signature::ECDSA_P256_SHA256_ASN1_SIGNING,
    ) {
        return Ok(Arc::new(ecdsa_p256));
    }

    if let Ok(ecdsa_p384) = EcdsaSigningKey::new(
        der,
        SignatureScheme::ECDSA_NISTP384_SHA384,
        &signature::ECDSA_P384_SHA384_ASN1_SIGNING,
    ) {
        return Ok(Arc::new(ecdsa_p384));
    }

    Err(SignError(()))
}

/// Parse `der` as any EdDSA key type, returning the first which works.
pub fn any_eddsa_type(der: &key::PrivateKey) -> Result<Arc<dyn SigningKey>, SignError> {
    if let Ok(ed25519) = Ed25519SigningKey::new(der, SignatureScheme::ED25519) {
        return Ok(Arc::new(ed25519));
    }

    // TODO: Add support for Ed448

    Err(SignError(()))
}

/// A `SigningKey` for RSA-PKCS1 or RSA-PSS.
///
/// This is used by the test suite, so it must be `pub`, but it isn't part of
/// the public, stable, API.
#[doc(hidden)]
pub struct RsaSigningKey {
    key: Arc<RsaKeyPair>,
}

static ALL_RSA_SCHEMES: &[SignatureScheme] = &[
    SignatureScheme::RSA_PSS_SHA512,
    SignatureScheme::RSA_PSS_SHA384,
    SignatureScheme::RSA_PSS_SHA256,
    SignatureScheme::RSA_PKCS1_SHA512,
    SignatureScheme::RSA_PKCS1_SHA384,
    SignatureScheme::RSA_PKCS1_SHA256,
];

impl RsaSigningKey {
    /// Make a new `RsaSigningKey` from a DER encoding, in either
    /// PKCS#1 or PKCS#8 format.
    pub fn new(der: &key::PrivateKey) -> Result<Self, SignError> {
        RsaKeyPair::from_der(&der.0)
            .or_else(|_| RsaKeyPair::from_pkcs8(&der.0))
            .map(|s| Self { key: Arc::new(s) })
            .map_err(|_| SignError(()))
    }
}

impl SigningKey for RsaSigningKey {
    fn choose_scheme(&self, offered: &[SignatureScheme]) -> Option<Box<dyn Signer>> {
        ALL_RSA_SCHEMES
            .iter()
            .find(|scheme| offered.contains(scheme))
            .map(|scheme| RsaSigner::new(Arc::clone(&self.key), *scheme))
    }

    fn algorithm(&self) -> SignatureAlgorithm {
        SignatureAlgorithm::RSA
    }
}

struct RsaSigner {
    key: Arc<RsaKeyPair>,
    scheme: SignatureScheme,
    encoding: &'static dyn signature::RsaEncoding,
}

impl RsaSigner {
    fn new(key: Arc<RsaKeyPair>, scheme: SignatureScheme) -> Box<dyn Signer> {
        let encoding: &dyn signature::RsaEncoding = match scheme {
            SignatureScheme::RSA_PKCS1_SHA256 => &signature::RSA_PKCS1_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384 => &signature::RSA_PKCS1_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512 => &signature::RSA_PKCS1_SHA512,
            SignatureScheme::RSA_PSS_SHA256 => &signature::RSA_PSS_SHA256,
            SignatureScheme::RSA_PSS_SHA384 => &signature::RSA_PSS_SHA384,
            SignatureScheme::RSA_PSS_SHA512 => &signature::RSA_PSS_SHA512,
            _ => unreachable!(),
        };

        Box::new(Self {
            key,
            scheme,
            encoding,
        })
    }
}

impl Signer for RsaSigner {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        let mut sig = vec![0; self.key.public().modulus_len()];

        let rng = SystemRandom::new();
        self.key
            .sign(self.encoding, &rng, message, &mut sig)
            .map(|_| sig)
            .map_err(|_| Error::General("signing failed".to_string()))
    }

    fn scheme(&self) -> SignatureScheme {
        self.scheme
    }
}

/// A SigningKey that uses exactly one TLS-level SignatureScheme
/// and one ring-level signature::SigningAlgorithm.
///
/// Compare this to RsaSigningKey, which for a particular key is
/// willing to sign with several algorithms.  This is quite poor
/// cryptography practice, but is necessary because a given RSA key
/// is expected to work in TLS1.2 (PKCS#1 signatures) and TLS1.3
/// (PSS signatures) -- nobody is willing to obtain certificates for
/// different protocol versions.
///
/// Currently this is only implemented for ECDSA keys.
struct EcdsaSigningKey {
    key: Arc<EcdsaKeyPair>,
    scheme: SignatureScheme,
}

impl EcdsaSigningKey {
    /// Make a new `ECDSASigningKey` from a DER encoding in PKCS#8 or SEC1
    /// format, expecting a key usable with precisely the given signature
    /// scheme.
    fn new(
        der: &key::PrivateKey,
        scheme: SignatureScheme,
        sigalg: &'static signature::EcdsaSigningAlgorithm,
    ) -> Result<Self, ()> {
        let rng = SystemRandom::new();
        EcdsaKeyPair::from_pkcs8(sigalg, &der.0, &rng)
            .map_err(|_| ())
            .or_else(|_| Self::convert_sec1_to_pkcs8(scheme, sigalg, &der.0, &rng))
            .map(|kp| Self {
                key: Arc::new(kp),
                scheme,
            })
    }

    /// Convert a SEC1 encoding to PKCS8, and ask ring to parse it.  This
    /// can be removed once https://github.com/briansmith/ring/pull/1456
    /// (or equivalent) is landed.
    fn convert_sec1_to_pkcs8(
        scheme: SignatureScheme,
        sigalg: &'static signature::EcdsaSigningAlgorithm,
        maybe_sec1_der: &[u8],
        rng: &dyn SecureRandom,
    ) -> Result<EcdsaKeyPair, ()> {
        let pkcs8_prefix = match scheme {
            SignatureScheme::ECDSA_NISTP256_SHA256 => &PKCS8_PREFIX_ECDSA_NISTP256,
            SignatureScheme::ECDSA_NISTP384_SHA384 => &PKCS8_PREFIX_ECDSA_NISTP384,
            _ => unreachable!(), // all callers are in this file
        };

        // wrap sec1 encoding in an OCTET STRING
        let mut sec1_wrap = Vec::with_capacity(maybe_sec1_der.len() + 8);
        sec1_wrap.extend_from_slice(maybe_sec1_der);
        wrap_in_asn1_len(&mut sec1_wrap);
        sec1_wrap.insert(0, der::Tag::OctetString as u8);

        let mut pkcs8 = Vec::with_capacity(pkcs8_prefix.len() + sec1_wrap.len() + 4);
        pkcs8.extend_from_slice(pkcs8_prefix);
        pkcs8.extend_from_slice(&sec1_wrap);
        wrap_in_sequence(&mut pkcs8);

        EcdsaKeyPair::from_pkcs8(sigalg, &pkcs8, rng).map_err(|_| ())
    }
}

// This is (line-by-line):
// - INTEGER Version = 0
// - SEQUENCE (privateKeyAlgorithm)
//   - id-ecPublicKey OID
//   - prime256v1 OID
const PKCS8_PREFIX_ECDSA_NISTP256: &[u8] = b"\x02\x01\x00\
      \x30\x13\
      \x06\x07\x2a\x86\x48\xce\x3d\x02\x01\
      \x06\x08\x2a\x86\x48\xce\x3d\x03\x01\x07";

// This is (line-by-line):
// - INTEGER Version = 0
// - SEQUENCE (privateKeyAlgorithm)
//   - id-ecPublicKey OID
//   - secp384r1 OID
const PKCS8_PREFIX_ECDSA_NISTP384: &[u8] = b"\x02\x01\x00\
     \x30\x10\
     \x06\x07\x2a\x86\x48\xce\x3d\x02\x01\
     \x06\x05\x2b\x81\x04\x00\x22";

impl SigningKey for EcdsaSigningKey {
    fn choose_scheme(&self, offered: &[SignatureScheme]) -> Option<Box<dyn Signer>> {
        if offered.contains(&self.scheme) {
            Some(Box::new(EcdsaSigner {
                key: Arc::clone(&self.key),
                scheme: self.scheme,
            }))
        } else {
            None
        }
    }

    fn algorithm(&self) -> SignatureAlgorithm {
        self.scheme.sign()
    }
}

struct EcdsaSigner {
    key: Arc<EcdsaKeyPair>,
    scheme: SignatureScheme,
}

impl Signer for EcdsaSigner {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        let rng = SystemRandom::new();
        self.key
            .sign(&rng, message)
            .map_err(|_| Error::General("signing failed".into()))
            .map(|sig| sig.as_ref().into())
    }

    fn scheme(&self) -> SignatureScheme {
        self.scheme
    }
}

/// A SigningKey that uses exactly one TLS-level SignatureScheme
/// and one ring-level signature::SigningAlgorithm.
///
/// Compare this to RsaSigningKey, which for a particular key is
/// willing to sign with several algorithms.  This is quite poor
/// cryptography practice, but is necessary because a given RSA key
/// is expected to work in TLS1.2 (PKCS#1 signatures) and TLS1.3
/// (PSS signatures) -- nobody is willing to obtain certificates for
/// different protocol versions.
///
/// Currently this is only implemented for Ed25519 keys.
struct Ed25519SigningKey {
    key: Arc<Ed25519KeyPair>,
    scheme: SignatureScheme,
}

impl Ed25519SigningKey {
    /// Make a new `Ed25519SigningKey` from a DER encoding in PKCS#8 format,
    /// expecting a key usable with precisely the given signature scheme.
    fn new(der: &key::PrivateKey, scheme: SignatureScheme) -> Result<Self, SignError> {
        Ed25519KeyPair::from_pkcs8_maybe_unchecked(&der.0)
            .map(|kp| Self {
                key: Arc::new(kp),
                scheme,
            })
            .map_err(|_| SignError(()))
    }
}

impl SigningKey for Ed25519SigningKey {
    fn choose_scheme(&self, offered: &[SignatureScheme]) -> Option<Box<dyn Signer>> {
        if offered.contains(&self.scheme) {
            Some(Box::new(Ed25519Signer {
                key: Arc::clone(&self.key),
                scheme: self.scheme,
            }))
        } else {
            None
        }
    }

    fn algorithm(&self) -> SignatureAlgorithm {
        self.scheme.sign()
    }
}

struct Ed25519Signer {
    key: Arc<Ed25519KeyPair>,
    scheme: SignatureScheme,
}

impl Signer for Ed25519Signer {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(self.key.sign(message).as_ref().into())
    }

    fn scheme(&self) -> SignatureScheme {
        self.scheme
    }
}

/// The set of schemes we support for signatures and
/// that are allowed for TLS1.3.
pub fn supported_sign_tls13() -> &'static [SignatureScheme] {
    &[
        SignatureScheme::ECDSA_NISTP384_SHA384,
        SignatureScheme::ECDSA_NISTP256_SHA256,
        SignatureScheme::RSA_PSS_SHA512,
        SignatureScheme::RSA_PSS_SHA384,
        SignatureScheme::RSA_PSS_SHA256,
        SignatureScheme::ED25519,
    ]
}

/// Errors while signing
#[derive(Debug)]
pub struct SignError(());

impl fmt::Display for SignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("sign error")
    }
}

impl StdError for SignError {}

#[test]
fn can_load_ecdsa_nistp256_pkcs8() {
    let key = key::PrivateKey(include_bytes!("testdata/nistp256key.pkcs8.der").to_vec());
    assert!(any_supported_type(&key).is_ok());
    assert!(any_ecdsa_type(&key).is_ok());
    assert!(any_eddsa_type(&key).is_err());
}

#[test]
fn can_load_ecdsa_nistp256_sec1() {
    let key = key::PrivateKey(include_bytes!("testdata/nistp256key.der").to_vec());
    assert!(any_supported_type(&key).is_ok());
    assert!(any_ecdsa_type(&key).is_ok());
    assert!(any_eddsa_type(&key).is_err());
}

#[test]
fn can_load_ecdsa_nistp384_pkcs8() {
    let key = key::PrivateKey(include_bytes!("testdata/nistp384key.pkcs8.der").to_vec());
    assert!(any_supported_type(&key).is_ok());
    assert!(any_ecdsa_type(&key).is_ok());
    assert!(any_eddsa_type(&key).is_err());
}

#[test]
fn can_load_ecdsa_nistp384_sec1() {
    let key = key::PrivateKey(include_bytes!("testdata/nistp384key.der").to_vec());
    assert!(any_supported_type(&key).is_ok());
    assert!(any_ecdsa_type(&key).is_ok());
    assert!(any_eddsa_type(&key).is_err());
}

#[test]
fn can_load_eddsa_pkcs8() {
    let key = key::PrivateKey(include_bytes!("testdata/eddsakey.der").to_vec());
    assert!(any_supported_type(&key).is_ok());
    assert!(any_eddsa_type(&key).is_ok());
    assert!(any_ecdsa_type(&key).is_err());
}

#[test]
fn can_load_rsa2048_pkcs8() {
    let key = key::PrivateKey(include_bytes!("testdata/rsa2048key.pkcs8.der").to_vec());
    assert!(any_supported_type(&key).is_ok());
    assert!(any_eddsa_type(&key).is_err());
    assert!(any_ecdsa_type(&key).is_err());
}

#[test]
fn can_load_rsa2048_pkcs1() {
    let key = key::PrivateKey(include_bytes!("testdata/rsa2048key.pkcs1.der").to_vec());
    assert!(any_supported_type(&key).is_ok());
    assert!(any_eddsa_type(&key).is_err());
    assert!(any_ecdsa_type(&key).is_err());
}
