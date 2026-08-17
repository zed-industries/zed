// Copyright 2015 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use crate::verify_cert::Budget;
use crate::{der, public_values_eq, Error};
use ring::signature;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// X.509 certificates and related items that are signed are almost always
/// encoded in the format "tbs||signatureAlgorithm||signature". This structure
/// captures this pattern as an owned data type.
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
pub(crate) struct OwnedSignedData {
    /// The signed data. This would be `tbsCertificate` in the case of an X.509
    /// certificate, `tbsResponseData` in the case of an OCSP response, `tbsCertList`
    /// in the case of a CRL, and the data nested in the `digitally-signed` construct for
    /// TLS 1.2 signed data.
    pub(crate) data: Vec<u8>,

    /// The value of the `AlgorithmIdentifier`. This would be
    /// `signatureAlgorithm` in the case of an X.509 certificate, OCSP
    /// response or CRL. This would have to be synthesized in the case of TLS 1.2
    /// signed data, since TLS does not identify algorithms by ASN.1 OIDs.
    pub(crate) algorithm: Vec<u8>,

    /// The value of the signature. This would be `signature` in an X.509
    /// certificate, OCSP response or CRL. This would be the value of
    /// `DigitallySigned.signature` for TLS 1.2 signed data.
    pub(crate) signature: Vec<u8>,
}

#[cfg(feature = "alloc")]
impl OwnedSignedData {
    /// Return a borrowed [`SignedData`] from the owned representation.
    pub(crate) fn borrow(&self) -> SignedData<'_> {
        SignedData {
            data: untrusted::Input::from(&self.data),
            algorithm: untrusted::Input::from(&self.algorithm),
            signature: untrusted::Input::from(&self.signature),
        }
    }
}

/// X.509 certificates and related items that are signed are almost always
/// encoded in the format "tbs||signatureAlgorithm||signature". This structure
/// captures this pattern.
#[derive(Debug)]
pub(crate) struct SignedData<'a> {
    /// The signed data. This would be `tbsCertificate` in the case of an X.509
    /// certificate, `tbsResponseData` in the case of an OCSP response, `tbsCertList`
    /// in the case of a CRL, and the data nested in the `digitally-signed` construct for
    /// TLS 1.2 signed data.
    data: untrusted::Input<'a>,

    /// The value of the `AlgorithmIdentifier`. This would be
    /// `signatureAlgorithm` in the case of an X.509 certificate, OCSP
    /// response or CRL. This would have to be synthesized in the case of TLS 1.2
    /// signed data, since TLS does not identify algorithms by ASN.1 OIDs.
    pub(crate) algorithm: untrusted::Input<'a>,

    /// The value of the signature. This would be `signature` in an X.509
    /// certificate, OCSP response or CRL. This would be the value of
    /// `DigitallySigned.signature` for TLS 1.2 signed data.
    signature: untrusted::Input<'a>,
}

impl<'a> SignedData<'a> {
    /// Parses the concatenation of "tbs||signatureAlgorithm||signature" that
    /// is common in the X.509 certificate and OCSP response syntaxes.
    ///
    /// X.509 Certificates (RFC 5280) look like this:
    ///
    /// ```ASN.1
    /// Certificate (SEQUENCE) {
    ///     tbsCertificate TBSCertificate,
    ///     signatureAlgorithm AlgorithmIdentifier,
    ///     signatureValue BIT STRING
    /// }
    ///
    /// OCSP responses (RFC 6960) look like this:
    /// ```ASN.1
    /// BasicOCSPResponse {
    ///     tbsResponseData ResponseData,
    ///     signatureAlgorithm AlgorithmIdentifier,
    ///     signature BIT STRING,
    ///     certs [0] EXPLICIT SEQUENCE OF Certificate OPTIONAL
    /// }
    /// ```
    ///
    /// Note that this function does NOT parse the outermost `SEQUENCE` or the
    /// `certs` value.
    ///
    /// The return value's first component is the contents of
    /// `tbsCertificate`/`tbsResponseData`; the second component is a `SignedData`
    /// structure that can be passed to `verify_signed_data`.
    ///
    /// The provided size_limit will enforce the largest possible outermost `SEQUENCE` this
    /// function will read.
    pub(crate) fn from_der(
        der: &mut untrusted::Reader<'a>,
        size_limit: usize,
    ) -> Result<(untrusted::Input<'a>, Self), Error> {
        let (data, tbs) = der.read_partial(|input| {
            der::expect_tag_and_get_value_limited(input, der::Tag::Sequence, size_limit)
        })?;
        let algorithm = der::expect_tag_and_get_value(der, der::Tag::Sequence)?;
        let signature = der::bit_string_with_no_unused_bits(der)?;

        Ok((
            tbs,
            SignedData {
                data,
                algorithm,
                signature,
            },
        ))
    }

    /// Convert the borrowed signed data to an [`OwnedSignedData`].
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub(crate) fn to_owned(&self) -> OwnedSignedData {
        OwnedSignedData {
            data: self.data.as_slice_less_safe().to_vec(),
            algorithm: self.algorithm.as_slice_less_safe().to_vec(),
            signature: self.signature.as_slice_less_safe().to_vec(),
        }
    }
}

/// Verify `signed_data` using the public key in the DER-encoded
/// SubjectPublicKeyInfo `spki` using one of the algorithms in
/// `supported_algorithms`.
///
/// The algorithm is chosen based on the algorithm information encoded in the
/// algorithm identifiers in `public_key` and `signed_data.algorithm`. The
/// ordering of the algorithms in `supported_algorithms` does not really matter,
/// but generally more common algorithms should go first, as it is scanned
/// linearly for matches.
pub(crate) fn verify_signed_data(
    supported_algorithms: &[&SignatureAlgorithm],
    spki_value: untrusted::Input,
    signed_data: &SignedData,
    budget: &mut Budget,
) -> Result<(), Error> {
    budget.consume_signature()?;

    // We need to verify the signature in `signed_data` using the public key
    // in `public_key`. In order to know which *ring* signature verification
    // algorithm to use, we need to know the public key algorithm (ECDSA,
    // RSA PKCS#1, etc.), the curve (if applicable), and the digest algorithm.
    // `signed_data` identifies only the public key algorithm and the digest
    // algorithm, and `public_key` identifies only the public key algorithm and
    // the curve (if any). Thus, we have to combine information from both
    // inputs to figure out which `ring::signature::VerificationAlgorithm` to
    // use to verify the signature.
    //
    // This is all further complicated by the fact that we don't have any
    // implicit knowledge about any algorithms or identifiers, since all of
    // that information is encoded in `supported_algorithms.` In particular, we
    // avoid hard-coding any of that information so that (link-time) dead code
    // elimination will work effectively in eliminating code for unused
    // algorithms.

    // Parse the signature.
    //
    let mut found_signature_alg_match = false;
    for supported_alg in supported_algorithms.iter().filter(|alg| {
        alg.signature_alg_id
            .matches_algorithm_id_value(signed_data.algorithm)
    }) {
        match verify_signature(
            supported_alg,
            spki_value,
            signed_data.data,
            signed_data.signature,
        ) {
            Err(Error::UnsupportedSignatureAlgorithmForPublicKey) => {
                found_signature_alg_match = true;
                continue;
            }
            result => {
                return result;
            }
        }
    }

    if found_signature_alg_match {
        Err(Error::UnsupportedSignatureAlgorithmForPublicKey)
    } else {
        Err(Error::UnsupportedSignatureAlgorithm)
    }
}

pub(crate) fn verify_signature(
    signature_alg: &SignatureAlgorithm,
    spki_value: untrusted::Input,
    msg: untrusted::Input,
    signature: untrusted::Input,
) -> Result<(), Error> {
    let spki = SubjectPublicKeyInfo::from_der(spki_value)?;
    if !signature_alg
        .public_key_alg_id
        .matches_algorithm_id_value(spki.algorithm_id_value)
    {
        return Err(Error::UnsupportedSignatureAlgorithmForPublicKey);
    }
    signature::UnparsedPublicKey::new(
        signature_alg.verification_alg,
        spki.key_value.as_slice_less_safe(),
    )
    .verify(msg.as_slice_less_safe(), signature.as_slice_less_safe())
    .map_err(|_| Error::InvalidSignatureForPublicKey)
}

struct SubjectPublicKeyInfo<'a> {
    algorithm_id_value: untrusted::Input<'a>,
    key_value: untrusted::Input<'a>,
}

impl<'a> SubjectPublicKeyInfo<'a> {
    // Parse the public key into an algorithm OID, an optional curve OID, and the
    // key value. The caller needs to check whether these match the
    // `PublicKeyAlgorithm` for the `SignatureAlgorithm` that is matched when
    // parsing the signature.
    fn from_der(input: untrusted::Input<'a>) -> Result<Self, Error> {
        input.read_all(Error::BadDer, |input| {
            let algorithm_id_value = der::expect_tag_and_get_value(input, der::Tag::Sequence)?;
            let key_value = der::bit_string_with_no_unused_bits(input)?;
            Ok(SubjectPublicKeyInfo {
                algorithm_id_value,
                key_value,
            })
        })
    }
}

/// A signature algorithm.
pub struct SignatureAlgorithm {
    public_key_alg_id: AlgorithmIdentifier,
    signature_alg_id: AlgorithmIdentifier,
    verification_alg: &'static dyn signature::VerificationAlgorithm,
}

/// ECDSA signatures using the P-256 curve and SHA-256.
pub static ECDSA_P256_SHA256: SignatureAlgorithm = SignatureAlgorithm {
    public_key_alg_id: ECDSA_P256,
    signature_alg_id: ECDSA_SHA256,
    verification_alg: &signature::ECDSA_P256_SHA256_ASN1,
};

/// ECDSA signatures using the P-256 curve and SHA-384. Deprecated.
pub static ECDSA_P256_SHA384: SignatureAlgorithm = SignatureAlgorithm {
    public_key_alg_id: ECDSA_P256,
    signature_alg_id: ECDSA_SHA384,
    verification_alg: &signature::ECDSA_P256_SHA384_ASN1,
};

/// ECDSA signatures using the P-384 curve and SHA-256. Deprecated.
pub static ECDSA_P384_SHA256: SignatureAlgorithm = SignatureAlgorithm {
    public_key_alg_id: ECDSA_P384,
    signature_alg_id: ECDSA_SHA256,
    verification_alg: &signature::ECDSA_P384_SHA256_ASN1,
};

/// ECDSA signatures using the P-384 curve and SHA-384.
pub static ECDSA_P384_SHA384: SignatureAlgorithm = SignatureAlgorithm {
    public_key_alg_id: ECDSA_P384,
    signature_alg_id: ECDSA_SHA384,
    verification_alg: &signature::ECDSA_P384_SHA384_ASN1,
};

/// RSA PKCS#1 1.5 signatures using SHA-256 for keys of 2048-8192 bits.
///
/// Requires the `alloc` feature.
#[cfg(feature = "alloc")]
pub static RSA_PKCS1_2048_8192_SHA256: SignatureAlgorithm = SignatureAlgorithm {
    public_key_alg_id: RSA_ENCRYPTION,
    signature_alg_id: RSA_PKCS1_SHA256,
    verification_alg: &signature::RSA_PKCS1_2048_8192_SHA256,
};

/// RSA PKCS#1 1.5 signatures using SHA-384 for keys of 2048-8192 bits.
///
/// Requires the `alloc` feature.
#[cfg(feature = "alloc")]
pub static RSA_PKCS1_2048_8192_SHA384: SignatureAlgorithm = SignatureAlgorithm {
    public_key_alg_id: RSA_ENCRYPTION,
    signature_alg_id: RSA_PKCS1_SHA384,
    verification_alg: &signature::RSA_PKCS1_2048_8192_SHA384,
};

/// RSA PKCS#1 1.5 signatures using SHA-512 for keys of 2048-8192 bits.
///
/// Requires the `alloc` feature.
#[cfg(feature = "alloc")]
pub static RSA_PKCS1_2048_8192_SHA512: SignatureAlgorithm = SignatureAlgorithm {
    public_key_alg_id: RSA_ENCRYPTION,
    signature_alg_id: RSA_PKCS1_SHA512,
    verification_alg: &signature::RSA_PKCS1_2048_8192_SHA512,
};

/// RSA PKCS#1 1.5 signatures using SHA-384 for keys of 3072-8192 bits.
///
/// Requires the `alloc` feature.
#[cfg(feature = "alloc")]
pub static RSA_PKCS1_3072_8192_SHA384: SignatureAlgorithm = SignatureAlgorithm {
    public_key_alg_id: RSA_ENCRYPTION,
    signature_alg_id: RSA_PKCS1_SHA384,
    verification_alg: &signature::RSA_PKCS1_3072_8192_SHA384,
};

/// RSA PSS signatures using SHA-256 for keys of 2048-8192 bits and of
/// type rsaEncryption; see [RFC 4055 Section 1.2].
///
/// [RFC 4055 Section 1.2]: https://tools.ietf.org/html/rfc4055#section-1.2
///
/// Requires the `alloc` feature.
#[cfg(feature = "alloc")]
pub static RSA_PSS_2048_8192_SHA256_LEGACY_KEY: SignatureAlgorithm = SignatureAlgorithm {
    public_key_alg_id: RSA_ENCRYPTION,
    signature_alg_id: RSA_PSS_SHA256,
    verification_alg: &signature::RSA_PSS_2048_8192_SHA256,
};

/// RSA PSS signatures using SHA-384 for keys of 2048-8192 bits and of
/// type rsaEncryption; see [RFC 4055 Section 1.2].
///
/// [RFC 4055 Section 1.2]: https://tools.ietf.org/html/rfc4055#section-1.2
///
/// Requires the `alloc` feature.
#[cfg(feature = "alloc")]
pub static RSA_PSS_2048_8192_SHA384_LEGACY_KEY: SignatureAlgorithm = SignatureAlgorithm {
    public_key_alg_id: RSA_ENCRYPTION,
    signature_alg_id: RSA_PSS_SHA384,
    verification_alg: &signature::RSA_PSS_2048_8192_SHA384,
};

/// RSA PSS signatures using SHA-512 for keys of 2048-8192 bits and of
/// type rsaEncryption; see [RFC 4055 Section 1.2].
///
/// [RFC 4055 Section 1.2]: https://tools.ietf.org/html/rfc4055#section-1.2
///
/// Requires the `alloc` feature.
#[cfg(feature = "alloc")]
pub static RSA_PSS_2048_8192_SHA512_LEGACY_KEY: SignatureAlgorithm = SignatureAlgorithm {
    public_key_alg_id: RSA_ENCRYPTION,
    signature_alg_id: RSA_PSS_SHA512,
    verification_alg: &signature::RSA_PSS_2048_8192_SHA512,
};

/// ED25519 signatures according to RFC 8410
pub static ED25519: SignatureAlgorithm = SignatureAlgorithm {
    public_key_alg_id: ED_25519,
    signature_alg_id: ED_25519,
    verification_alg: &signature::ED25519,
};

struct AlgorithmIdentifier {
    asn1_id_value: untrusted::Input<'static>,
}

impl AlgorithmIdentifier {
    fn matches_algorithm_id_value(&self, encoded: untrusted::Input) -> bool {
        public_values_eq(encoded, self.asn1_id_value)
    }
}

// See src/data/README.md.

const ECDSA_P256: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("data/alg-ecdsa-p256.der")),
};

const ECDSA_P384: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("data/alg-ecdsa-p384.der")),
};

const ECDSA_SHA256: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("data/alg-ecdsa-sha256.der")),
};

const ECDSA_SHA384: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("data/alg-ecdsa-sha384.der")),
};

#[cfg(feature = "alloc")]
const RSA_ENCRYPTION: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("data/alg-rsa-encryption.der")),
};

#[cfg(feature = "alloc")]
const RSA_PKCS1_SHA256: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("data/alg-rsa-pkcs1-sha256.der")),
};

#[cfg(feature = "alloc")]
const RSA_PKCS1_SHA384: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("data/alg-rsa-pkcs1-sha384.der")),
};

#[cfg(feature = "alloc")]
const RSA_PKCS1_SHA512: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("data/alg-rsa-pkcs1-sha512.der")),
};

#[cfg(feature = "alloc")]
const RSA_PSS_SHA256: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("data/alg-rsa-pss-sha256.der")),
};

#[cfg(feature = "alloc")]
const RSA_PSS_SHA384: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("data/alg-rsa-pss-sha384.der")),
};

#[cfg(feature = "alloc")]
const RSA_PSS_SHA512: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("data/alg-rsa-pss-sha512.der")),
};

const ED_25519: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("data/alg-ed25519.der")),
};

#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose, Engine as _};

    use crate::{der, signed_data, Error};
    use alloc::{string::String, vec::Vec};

    macro_rules! test_file_bytes {
        ( $file_name:expr ) => {
            include_bytes!(concat!(
                "../third-party/chromium/data/verify_signed_data/",
                $file_name
            ))
        };
    }

    // TODO: The expected results need to be modified for SHA-1 deprecation.

    macro_rules! test_verify_signed_data {
        ($fn_name:ident, $file_name:expr, $expected_result:expr) => {
            #[test]
            fn $fn_name() {
                test_verify_signed_data(test_file_bytes!($file_name), $expected_result);
            }
        };
    }

    fn test_verify_signed_data(file_contents: &[u8], expected_result: Result<(), Error>) {
        let tsd = parse_test_signed_data(file_contents);
        let spki_value = untrusted::Input::from(&tsd.spki);
        let spki_value = spki_value
            .read_all(Error::BadDer, |input| {
                der::expect_tag_and_get_value(input, der::Tag::Sequence)
            })
            .unwrap();

        // we can't use `parse_signed_data` because it requires `data`
        // to be an ASN.1 SEQUENCE, and that isn't the case with
        // Chromium's test data. TODO: The test data set should be
        // expanded with SEQUENCE-wrapped data so that we can actually
        // test `parse_signed_data`.

        let algorithm = untrusted::Input::from(&tsd.algorithm);
        let algorithm = algorithm
            .read_all(Error::BadDer, |input| {
                der::expect_tag_and_get_value(input, der::Tag::Sequence)
            })
            .unwrap();

        let signature = untrusted::Input::from(&tsd.signature);
        let signature = signature
            .read_all(Error::BadDer, |input| {
                der::bit_string_with_no_unused_bits(input)
            })
            .unwrap();

        let signed_data = signed_data::SignedData {
            data: untrusted::Input::from(&tsd.data),
            algorithm,
            signature,
        };

        assert_eq!(
            expected_result,
            signed_data::verify_signed_data(
                SUPPORTED_ALGORITHMS_IN_TESTS,
                spki_value,
                &signed_data,
                &mut Budget::default()
            )
        );
    }

    // XXX: This is testing code that isn't even in this module.
    macro_rules! test_verify_signed_data_signature_outer {
        ($fn_name:ident, $file_name:expr, $expected_result:expr) => {
            #[test]
            fn $fn_name() {
                test_verify_signed_data_signature_outer(
                    test_file_bytes!($file_name),
                    $expected_result,
                );
            }
        };
    }

    fn test_verify_signed_data_signature_outer(file_contents: &[u8], expected_error: Error) {
        let tsd = parse_test_signed_data(file_contents);
        let signature = untrusted::Input::from(&tsd.signature);
        assert_eq!(
            signature
                .read_all(Error::BadDer, |input| {
                    der::bit_string_with_no_unused_bits(input)
                })
                .unwrap_err(),
            expected_error
        );
    }

    // XXX: This is testing code that is not even in this module.
    macro_rules! test_parse_spki_bad_outer {
        ($fn_name:ident, $file_name:expr, $error:expr) => {
            #[test]
            fn $fn_name() {
                test_parse_spki_bad_outer(test_file_bytes!($file_name), $error)
            }
        };
    }

    fn test_parse_spki_bad_outer(file_contents: &[u8], expected_error: Error) {
        let tsd = parse_test_signed_data(file_contents);
        let spki = untrusted::Input::from(&tsd.spki);
        assert_eq!(
            spki.read_all(Error::BadDer, |input| {
                der::expect_tag_and_get_value(input, der::Tag::Sequence)
            })
            .unwrap_err(),
            expected_error,
        );
    }

    const UNSUPPORTED_SIGNATURE_ALGORITHM_FOR_RSA_KEY: Error = if cfg!(feature = "alloc") {
        Error::UnsupportedSignatureAlgorithmForPublicKey
    } else {
        Error::UnsupportedSignatureAlgorithm
    };

    const INVALID_SIGNATURE_FOR_RSA_KEY: Error = if cfg!(feature = "alloc") {
        Error::InvalidSignatureForPublicKey
    } else {
        Error::UnsupportedSignatureAlgorithm
    };

    const OK_IF_RSA_AVAILABLE: Result<(), Error> = if cfg!(feature = "alloc") {
        Ok(())
    } else {
        Err(Error::UnsupportedSignatureAlgorithm)
    };

    // XXX: Some of the BadDer tests should have better error codes, maybe?

    // XXX: We should have a variant of this test with a SHA-256 digest that gives
    // `Error::UnsupportedSignatureAlgorithmForPublicKey`.
    test_verify_signed_data!(
        test_ecdsa_prime256v1_sha512_spki_params_null,
        "ecdsa-prime256v1-sha512-spki-params-null.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );
    test_verify_signed_data_signature_outer!(
        test_ecdsa_prime256v1_sha512_unused_bits_signature,
        "ecdsa-prime256v1-sha512-unused-bits-signature.pem",
        Error::BadDer
    );
    // XXX: We should have a variant of this test with a SHA-256 digest that gives
    // `Error::UnsupportedSignatureAlgorithmForPublicKey`.
    test_verify_signed_data!(
        test_ecdsa_prime256v1_sha512_using_ecdh_key,
        "ecdsa-prime256v1-sha512-using-ecdh-key.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );
    // XXX: We should have a variant of this test with a SHA-256 digest that gives
    // `Error::UnsupportedSignatureAlgorithmForPublicKey`.
    test_verify_signed_data!(
        test_ecdsa_prime256v1_sha512_using_ecmqv_key,
        "ecdsa-prime256v1-sha512-using-ecmqv-key.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );
    test_verify_signed_data!(
        test_ecdsa_prime256v1_sha512_using_rsa_algorithm,
        "ecdsa-prime256v1-sha512-using-rsa-algorithm.pem",
        Err(UNSUPPORTED_SIGNATURE_ALGORITHM_FOR_RSA_KEY)
    );
    // XXX: We should have a variant of this test with a SHA-256 digest that gives
    // `Error::InvalidSignatureForPublicKey`.
    test_verify_signed_data!(
        test_ecdsa_prime256v1_sha512_wrong_signature_format,
        "ecdsa-prime256v1-sha512-wrong-signature-format.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );
    // Differs from Chromium because we don't support P-256 with SHA-512.
    test_verify_signed_data!(
        test_ecdsa_prime256v1_sha512,
        "ecdsa-prime256v1-sha512.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );
    test_verify_signed_data!(
        test_ecdsa_secp384r1_sha256_corrupted_data,
        "ecdsa-secp384r1-sha256-corrupted-data.pem",
        Err(Error::InvalidSignatureForPublicKey)
    );
    test_verify_signed_data!(
        test_ecdsa_secp384r1_sha256,
        "ecdsa-secp384r1-sha256.pem",
        Ok(())
    );
    test_verify_signed_data!(
        test_ecdsa_using_rsa_key,
        "ecdsa-using-rsa-key.pem",
        Err(Error::UnsupportedSignatureAlgorithmForPublicKey)
    );

    test_parse_spki_bad_outer!(
        test_rsa_pkcs1_sha1_bad_key_der_length,
        "rsa-pkcs1-sha1-bad-key-der-length.pem",
        Error::BadDer
    );
    test_parse_spki_bad_outer!(
        test_rsa_pkcs1_sha1_bad_key_der_null,
        "rsa-pkcs1-sha1-bad-key-der-null.pem",
        Error::BadDer
    );
    test_verify_signed_data!(
        test_rsa_pkcs1_sha1_key_params_absent,
        "rsa-pkcs1-sha1-key-params-absent.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );
    test_verify_signed_data!(
        test_rsa_pkcs1_sha1_using_pss_key_no_params,
        "rsa-pkcs1-sha1-using-pss-key-no-params.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );
    test_verify_signed_data!(
        test_rsa_pkcs1_sha1_wrong_algorithm,
        "rsa-pkcs1-sha1-wrong-algorithm.pem",
        Err(INVALID_SIGNATURE_FOR_RSA_KEY)
    );
    test_verify_signed_data!(
        test_rsa_pkcs1_sha1,
        "rsa-pkcs1-sha1.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );
    // XXX: RSA PKCS#1 with SHA-1 is a supported algorithm, but we only accept
    // 2048-8192 bit keys, and this test file is using a 1024 bit key. Thus,
    // our results differ from Chromium's. TODO: this means we need a 2048+ bit
    // version of this test.
    test_verify_signed_data!(
        test_rsa_pkcs1_sha256,
        "rsa-pkcs1-sha256.pem",
        Err(INVALID_SIGNATURE_FOR_RSA_KEY)
    );
    test_parse_spki_bad_outer!(
        test_rsa_pkcs1_sha256_key_encoded_ber,
        "rsa-pkcs1-sha256-key-encoded-ber.pem",
        Error::BadDer
    );
    test_verify_signed_data!(
        test_rsa_pkcs1_sha256_spki_non_null_params,
        "rsa-pkcs1-sha256-spki-non-null-params.pem",
        Err(UNSUPPORTED_SIGNATURE_ALGORITHM_FOR_RSA_KEY)
    );
    test_verify_signed_data!(
        test_rsa_pkcs1_sha256_using_ecdsa_algorithm,
        "rsa-pkcs1-sha256-using-ecdsa-algorithm.pem",
        Err(Error::UnsupportedSignatureAlgorithmForPublicKey)
    );
    test_verify_signed_data!(
        test_rsa_pkcs1_sha256_using_id_ea_rsa,
        "rsa-pkcs1-sha256-using-id-ea-rsa.pem",
        Err(UNSUPPORTED_SIGNATURE_ALGORITHM_FOR_RSA_KEY)
    );

    // Chromium's PSS test are for parameter combinations we don't support.
    test_verify_signed_data!(
        test_rsa_pss_sha1_salt20_using_pss_key_no_params,
        "rsa-pss-sha1-salt20-using-pss-key-no-params.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );
    test_verify_signed_data!(
        test_rsa_pss_sha1_salt20_using_pss_key_with_null_params,
        "rsa-pss-sha1-salt20-using-pss-key-with-null-params.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );
    test_verify_signed_data!(
        test_rsa_pss_sha1_salt20,
        "rsa-pss-sha1-salt20.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );
    test_verify_signed_data!(
        test_rsa_pss_sha1_wrong_salt,
        "rsa-pss-sha1-wrong-salt.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );
    test_verify_signed_data!(
        test_rsa_pss_sha256_mgf1_sha512_salt33,
        "rsa-pss-sha256-mgf1-sha512-salt33.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );
    test_verify_signed_data!(
        test_rsa_pss_sha256_salt10_using_pss_key_with_params,
        "rsa-pss-sha256-salt10-using-pss-key-with-params.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );
    test_verify_signed_data!(
        test_rsa_pss_sha256_salt10_using_pss_key_with_wrong_params,
        "rsa-pss-sha256-salt10-using-pss-key-with-wrong-params.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );
    test_verify_signed_data!(
        test_rsa_pss_sha256_salt10,
        "rsa-pss-sha256-salt10.pem",
        Err(Error::UnsupportedSignatureAlgorithm)
    );

    // Our PSS tests that should work.
    test_verify_signed_data!(
        test_rsa_pss_sha256_salt32,
        "ours/rsa-pss-sha256-salt32.pem",
        OK_IF_RSA_AVAILABLE
    );
    test_verify_signed_data!(
        test_rsa_pss_sha384_salt48,
        "ours/rsa-pss-sha384-salt48.pem",
        OK_IF_RSA_AVAILABLE
    );
    test_verify_signed_data!(
        test_rsa_pss_sha512_salt64,
        "ours/rsa-pss-sha512-salt64.pem",
        OK_IF_RSA_AVAILABLE
    );
    test_verify_signed_data!(
        test_rsa_pss_sha256_salt32_corrupted_data,
        "ours/rsa-pss-sha256-salt32-corrupted-data.pem",
        Err(INVALID_SIGNATURE_FOR_RSA_KEY)
    );
    test_verify_signed_data!(
        test_rsa_pss_sha384_salt48_corrupted_data,
        "ours/rsa-pss-sha384-salt48-corrupted-data.pem",
        Err(INVALID_SIGNATURE_FOR_RSA_KEY)
    );
    test_verify_signed_data!(
        test_rsa_pss_sha512_salt64_corrupted_data,
        "ours/rsa-pss-sha512-salt64-corrupted-data.pem",
        Err(INVALID_SIGNATURE_FOR_RSA_KEY)
    );

    test_verify_signed_data!(
        test_rsa_using_ec_key,
        "rsa-using-ec-key.pem",
        Err(UNSUPPORTED_SIGNATURE_ALGORITHM_FOR_RSA_KEY)
    );
    test_verify_signed_data!(
        test_rsa2048_pkcs1_sha512,
        "rsa2048-pkcs1-sha512.pem",
        OK_IF_RSA_AVAILABLE
    );

    struct TestSignedData {
        spki: Vec<u8>,
        data: Vec<u8>,
        algorithm: Vec<u8>,
        signature: Vec<u8>,
    }

    fn parse_test_signed_data(file_contents: &[u8]) -> TestSignedData {
        let mut lines = core::str::from_utf8(file_contents).unwrap().lines();
        let spki = read_pem_section(&mut lines, "PUBLIC KEY");
        let algorithm = read_pem_section(&mut lines, "ALGORITHM");
        let data = read_pem_section(&mut lines, "DATA");
        let signature = read_pem_section(&mut lines, "SIGNATURE");

        TestSignedData {
            spki,
            data,
            algorithm,
            signature,
        }
    }

    use crate::verify_cert::Budget;
    use alloc::str::Lines;

    fn read_pem_section(lines: &mut Lines, section_name: &str) -> Vec<u8> {
        // Skip comments and header
        let begin_section = format!("-----BEGIN {}-----", section_name);
        loop {
            let line = lines.next().unwrap();
            if line == begin_section {
                break;
            }
        }

        let mut base64 = String::new();

        let end_section = format!("-----END {}-----", section_name);
        loop {
            let line = lines.next().unwrap();
            if line == end_section {
                break;
            }
            base64.push_str(line);
        }

        general_purpose::STANDARD.decode(&base64).unwrap()
    }

    static SUPPORTED_ALGORITHMS_IN_TESTS: &[&signed_data::SignatureAlgorithm] = &[
        // Reasonable algorithms.
        &signed_data::ECDSA_P256_SHA256,
        &signed_data::ECDSA_P384_SHA384,
        &signed_data::ED25519,
        #[cfg(feature = "alloc")]
        &signed_data::RSA_PKCS1_2048_8192_SHA256,
        #[cfg(feature = "alloc")]
        &signed_data::RSA_PKCS1_2048_8192_SHA384,
        #[cfg(feature = "alloc")]
        &signed_data::RSA_PKCS1_2048_8192_SHA512,
        #[cfg(feature = "alloc")]
        &signed_data::RSA_PKCS1_3072_8192_SHA384,
        #[cfg(feature = "alloc")]
        &signed_data::RSA_PSS_2048_8192_SHA256_LEGACY_KEY,
        #[cfg(feature = "alloc")]
        &signed_data::RSA_PSS_2048_8192_SHA384_LEGACY_KEY,
        #[cfg(feature = "alloc")]
        &signed_data::RSA_PSS_2048_8192_SHA512_LEGACY_KEY,
        // Algorithms deprecated because they are annoying (P-521) or because
        // they are nonsensical combinations.
        &signed_data::ECDSA_P256_SHA384, // Truncates digest.
        &signed_data::ECDSA_P384_SHA256, // Digest is unnecessarily short.
    ];
}
