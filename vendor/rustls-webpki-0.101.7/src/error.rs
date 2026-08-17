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

use core::fmt;
use core::ops::ControlFlow;

/// An error that occurs during certificate validation or name validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The encoding of some ASN.1 DER-encoded item is invalid.
    BadDer,

    /// The encoding of an ASN.1 DER-encoded time is invalid.
    BadDerTime,

    /// A CA certificate is being used as an end-entity certificate.
    CaUsedAsEndEntity,

    /// The certificate is expired; i.e. the time it is being validated for is
    /// later than the certificate's notAfter time.
    CertExpired,

    /// The certificate is not valid for the name it is being validated for.
    CertNotValidForName,

    /// The certificate is not valid yet; i.e. the time it is being validated
    /// for is earlier than the certificate's notBefore time.
    CertNotValidYet,

    /// The certificate, or one of its issuers, has been revoked.
    CertRevoked,

    /// An end-entity certificate is being used as a CA certificate.
    EndEntityUsedAsCa,

    /// An X.509 extension is invalid.
    ExtensionValueInvalid,

    /// The certificate validity period (notBefore, notAfter) is invalid; e.g.
    /// the notAfter time is earlier than the notBefore time.
    InvalidCertValidity,

    /// A CRL number extension was invalid:
    ///  - it was mis-encoded
    ///  - it was negative
    ///  - it was too long
    InvalidCrlNumber,

    /// A iPAddress name constraint was invalid:
    /// - it had a sparse network mask (ie, cannot be written in CIDR form).
    /// - it was too long or short
    InvalidNetworkMaskConstraint,

    /// A serial number was invalid:
    ///  - it was misencoded
    ///  - it was negative
    ///  - it was too long
    InvalidSerialNumber,

    /// The CRL signature is invalid for the issuer's public key.
    InvalidCrlSignatureForPublicKey,

    /// The signature is invalid for the given public key.
    InvalidSignatureForPublicKey,

    /// A CRL was signed by an issuer that has a KeyUsage bitstring that does not include
    /// the cRLSign key usage bit.
    IssuerNotCrlSigner,

    /// A presented or reference DNS identifier was malformed, potentially
    /// containing invalid characters or invalid labels.
    MalformedDnsIdentifier,

    /// The certificate extensions are malformed.
    ///
    /// In particular, webpki requires the DNS name(s) be in the subjectAltName
    /// extension as required by the CA/Browser Forum Baseline Requirements
    /// and as recommended by RFC6125.
    MalformedExtensions,

    /// A name constraint was malformed, potentially containing invalid characters or
    /// invalid labels.
    MalformedNameConstraint,

    /// The maximum number of name constraint comparisons has been reached.
    MaximumNameConstraintComparisonsExceeded,

    /// The maximum number of internal path building calls has been reached. Path complexity is too great.
    MaximumPathBuildCallsExceeded,

    /// The path search was terminated because it became too deep.
    MaximumPathDepthExceeded,

    /// The maximum number of signature checks has been reached. Path complexity is too great.
    MaximumSignatureChecksExceeded,

    /// The certificate violates one or more name constraints.
    NameConstraintViolation,

    /// The certificate violates one or more path length constraints.
    PathLenConstraintViolated,

    /// The certificate is not valid for the Extended Key Usage for which it is
    /// being validated.
    RequiredEkuNotFound,

    /// The algorithm in the TBSCertificate "signature" field of a certificate
    /// does not match the algorithm in the signature of the certificate.
    SignatureAlgorithmMismatch,

    /// A valid issuer for the certificate could not be found.
    UnknownIssuer,

    /// The certificate is not a v3 X.509 certificate.
    ///
    /// This error may be also reported if the certificate version field
    /// is malformed.
    UnsupportedCertVersion,

    /// The certificate contains an unsupported critical extension.
    UnsupportedCriticalExtension,

    /// The CRL is not a v2 X.509 CRL.
    ///
    /// The RFC 5280 web PKI profile mandates only version 2 be used. See section
    /// 5.1.2.1 for more information.
    ///
    /// This error may also be reported if the CRL version field is malformed.
    UnsupportedCrlVersion,

    /// The CRL is an unsupported "delta" CRL.
    UnsupportedDeltaCrl,

    /// The CRL contains unsupported "indirect" entries.
    UnsupportedIndirectCrl,

    /// The revocation reason is not in the set of supported revocation reasons.
    UnsupportedRevocationReason,

    /// The signature algorithm for a signature over a CRL is not in the set of supported
    /// signature algorithms given.
    UnsupportedCrlSignatureAlgorithm,

    /// The signature algorithm for a signature is not in the set of supported
    /// signature algorithms given.
    UnsupportedSignatureAlgorithm,

    /// The CRL signature's algorithm does not match the algorithm of the issuer
    /// public key it is being validated for. This may be because the public key
    /// algorithm's OID isn't recognized (e.g. DSA), or the public key
    /// algorithm's parameters don't match the supported parameters for that
    /// algorithm (e.g. ECC keys for unsupported curves), or the public key
    /// algorithm and the signature algorithm simply don't match (e.g.
    /// verifying an RSA signature with an ECC public key).
    UnsupportedCrlSignatureAlgorithmForPublicKey,

    /// The signature's algorithm does not match the algorithm of the public
    /// key it is being validated for. This may be because the public key
    /// algorithm's OID isn't recognized (e.g. DSA), or the public key
    /// algorithm's parameters don't match the supported parameters for that
    /// algorithm (e.g. ECC keys for unsupported curves), or the public key
    /// algorithm and the signature algorithm simply don't match (e.g.
    /// verifying an RSA signature with an ECC public key).
    UnsupportedSignatureAlgorithmForPublicKey,
}

impl Error {
    // Compare the Error with the new error by rank, returning the higher rank of the two as
    // the most specific error.
    pub(crate) fn most_specific(self, new: Error) -> Error {
        // Assign an error a numeric value ranking it by specificity.
        if self.rank() >= new.rank() {
            self
        } else {
            new
        }
    }

    // Return a numeric indication of how specific the error is, where an error with a higher rank
    // is considered more useful to an end user than an error with a lower rank. This is used by
    // Error::most_specific to compare two errors in order to return which is more specific.
    #[allow(clippy::as_conversions)] // We won't exceed u32 errors.
    pub(crate) fn rank(&self) -> u32 {
        match &self {
            // Errors related to certificate validity
            Error::CertNotValidYet | Error::CertExpired => 290,
            Error::CertNotValidForName => 280,
            Error::CertRevoked => 270,
            Error::InvalidCrlSignatureForPublicKey | Error::InvalidSignatureForPublicKey => 260,
            Error::SignatureAlgorithmMismatch => 250,
            Error::RequiredEkuNotFound => 240,
            Error::NameConstraintViolation => 230,
            Error::PathLenConstraintViolated => 220,
            Error::CaUsedAsEndEntity | Error::EndEntityUsedAsCa => 210,
            Error::IssuerNotCrlSigner => 200,

            // Errors related to supported features used in an invalid way.
            Error::InvalidCertValidity => 190,
            Error::InvalidNetworkMaskConstraint => 180,
            Error::InvalidSerialNumber => 170,
            Error::InvalidCrlNumber => 160,

            // Errors related to unsupported features.
            Error::UnsupportedCrlSignatureAlgorithmForPublicKey
            | Error::UnsupportedSignatureAlgorithmForPublicKey => 150,
            Error::UnsupportedCrlSignatureAlgorithm | Error::UnsupportedSignatureAlgorithm => 140,
            Error::UnsupportedCriticalExtension => 130,
            Error::UnsupportedCertVersion => 130,
            Error::UnsupportedCrlVersion => 120,
            Error::UnsupportedDeltaCrl => 110,
            Error::UnsupportedIndirectCrl => 100,
            Error::UnsupportedRevocationReason => 90,
            // Reserved for webpki 0.102.0+ usages:
            // Error::UnsupportedRevocationReasonsPartitioning => 80,
            // Error::UnsupportedCrlIssuingDistributionPoint => 70,
            Error::MaximumPathDepthExceeded => 61,

            // Errors related to malformed data.
            Error::MalformedDnsIdentifier => 60,
            Error::MalformedNameConstraint => 50,
            Error::MalformedExtensions => 40,
            Error::ExtensionValueInvalid => 30,

            // Generic DER errors.
            Error::BadDerTime => 20,
            Error::BadDer => 10,

            // Special case errors - not subject to ranking.
            Error::MaximumSignatureChecksExceeded => 0,
            Error::MaximumPathBuildCallsExceeded => 0,
            Error::MaximumNameConstraintComparisonsExceeded => 0,

            // Default catch all error - should be renamed in the future.
            Error::UnknownIssuer => 0,
        }
    }

    /// Returns true for errors that should be considered fatal during path building. Errors of
    /// this class should halt any further path building and be returned immediately.
    #[inline]
    pub(crate) fn is_fatal(&self) -> bool {
        matches!(
            self,
            Error::MaximumSignatureChecksExceeded
                | Error::MaximumPathBuildCallsExceeded
                | Error::MaximumNameConstraintComparisonsExceeded
        )
    }
}

impl From<Error> for ControlFlow<Error, Error> {
    fn from(value: Error) -> Self {
        match value {
            // If an error is fatal, we've exhausted the potential for continued search.
            err if err.is_fatal() => Self::Break(err),
            // Otherwise we've rejected one candidate chain, but may continue to search for others.
            err => Self::Continue(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl ::std::error::Error for Error {}

impl From<untrusted::EndOfInput> for Error {
    fn from(_: untrusted::EndOfInput) -> Self {
        Error::BadDer
    }
}
