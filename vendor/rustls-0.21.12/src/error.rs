use crate::enums::{AlertDescription, ContentType, HandshakeType};
use crate::msgs::handshake::KeyExchangeAlgorithm;
use crate::rand;

use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;
use std::time::SystemTimeError;

/// rustls reports protocol errors using this type.
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    /// We received a TLS message that isn't valid right now.
    /// `expect_types` lists the message types we can expect right now.
    /// `got_type` is the type we found.  This error is typically
    /// caused by a buggy TLS stack (the peer or this one), a broken
    /// network, or an attack.
    InappropriateMessage {
        /// Which types we expected
        expect_types: Vec<ContentType>,
        /// What type we received
        got_type: ContentType,
    },

    /// We received a TLS handshake message that isn't valid right now.
    /// `expect_types` lists the handshake message types we can expect
    /// right now.  `got_type` is the type we found.
    InappropriateHandshakeMessage {
        /// Which handshake type we expected
        expect_types: Vec<HandshakeType>,
        /// What handshake type we received
        got_type: HandshakeType,
    },

    /// The peer sent us a TLS message with invalid contents.
    InvalidMessage(InvalidMessage),

    /// The peer didn't give us any certificates.
    NoCertificatesPresented,

    /// The certificate verifier doesn't support the given type of name.
    UnsupportedNameType,

    /// We couldn't decrypt a message.  This is invariably fatal.
    DecryptError,

    /// We couldn't encrypt a message because it was larger than the allowed message size.
    /// This should never happen if the application is using valid record sizes.
    EncryptError,

    /// The peer doesn't support a protocol version/feature we require.
    /// The parameter gives a hint as to what version/feature it is.
    PeerIncompatible(PeerIncompatible),

    /// The peer deviated from the standard TLS protocol.
    /// The parameter gives a hint where.
    PeerMisbehaved(PeerMisbehaved),

    /// We received a fatal alert.  This means the peer is unhappy.
    AlertReceived(AlertDescription),

    /// We saw an invalid certificate.
    ///
    /// The contained error is from the certificate validation trait
    /// implementation.
    InvalidCertificate(CertificateError),

    /// The presented SCT(s) were invalid.
    InvalidSct(sct::Error),

    /// A provided certificate revocation list (CRL) was invalid.
    InvalidCertRevocationList(CertRevocationListError),

    /// A catch-all error for unlikely errors.
    General(String),

    /// We failed to figure out what time it currently is.
    FailedToGetCurrentTime,

    /// We failed to acquire random bytes from the system.
    FailedToGetRandomBytes,

    /// This function doesn't work until the TLS handshake
    /// is complete.
    HandshakeNotComplete,

    /// The peer sent an oversized record/fragment.
    PeerSentOversizedRecord,

    /// An incoming connection did not support any known application protocol.
    NoApplicationProtocol,

    /// The `max_fragment_size` value supplied in configuration was too small,
    /// or too large.
    BadMaxFragmentSize,
}

/// A corrupt TLS message payload that resulted in an error.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]

pub enum InvalidMessage {
    /// An advertised message was larger then expected.
    HandshakePayloadTooLarge,
    /// The peer sent us a syntactically incorrect ChangeCipherSpec payload.
    InvalidCcs,
    /// An unknown content type was encountered during message decoding.
    InvalidContentType,
    /// A peer sent an invalid certificate status type
    InvalidCertificateStatusType,
    /// Context was incorrectly attached to a certificate request during a handshake.
    InvalidCertRequest,
    /// A peer's DH params could not be decoded
    InvalidDhParams,
    /// A message was zero-length when its record kind forbids it.
    InvalidEmptyPayload,
    /// A peer sent an unexpected key update request.
    InvalidKeyUpdate,
    /// A peer's server name could not be decoded
    InvalidServerName,
    /// A TLS message payload was larger then allowed by the specification.
    MessageTooLarge,
    /// Message is shorter than the expected length
    MessageTooShort,
    /// Missing data for the named handshake payload value
    MissingData(&'static str),
    /// A peer did not advertise its supported key exchange groups.
    MissingKeyExchange,
    /// A peer sent an empty list of signature schemes
    NoSignatureSchemes,
    /// Trailing data found for the named handshake payload value
    TrailingData(&'static str),
    /// A peer sent an unexpected message type.
    UnexpectedMessage(&'static str),
    /// An unknown TLS protocol was encountered during message decoding.
    UnknownProtocolVersion,
    /// A peer sent a non-null compression method.
    UnsupportedCompression,
    /// A peer sent an unknown elliptic curve type.
    UnsupportedCurveType,
    /// A peer sent an unsupported key exchange algorithm.
    UnsupportedKeyExchangeAlgorithm(KeyExchangeAlgorithm),
}

impl From<InvalidMessage> for Error {
    #[inline]
    fn from(e: InvalidMessage) -> Self {
        Self::InvalidMessage(e)
    }
}

#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
/// The set of cases where we failed to make a connection because we thought
/// the peer was misbehaving.
///
/// This is `non_exhaustive`: we might add or stop using items here in minor
/// versions.  We also don't document what they mean.  Generally a user of
/// rustls shouldn't vary its behaviour on these error codes, and there is
/// nothing it can do to improve matters.
///
/// Please file a bug against rustls if you see `Error::PeerMisbehaved` in
/// the wild.
pub enum PeerMisbehaved {
    AttemptedDowngradeToTls12WhenTls13IsSupported,
    BadCertChainExtensions,
    DisallowedEncryptedExtension,
    DuplicateClientHelloExtensions,
    DuplicateEncryptedExtensions,
    DuplicateHelloRetryRequestExtensions,
    DuplicateNewSessionTicketExtensions,
    DuplicateServerHelloExtensions,
    DuplicateServerNameTypes,
    EarlyDataAttemptedInSecondClientHello,
    EarlyDataExtensionWithoutResumption,
    EarlyDataOfferedWithVariedCipherSuite,
    HandshakeHashVariedAfterRetry,
    IllegalHelloRetryRequestWithEmptyCookie,
    IllegalHelloRetryRequestWithNoChanges,
    IllegalHelloRetryRequestWithOfferedGroup,
    IllegalHelloRetryRequestWithUnofferedCipherSuite,
    IllegalHelloRetryRequestWithUnofferedNamedGroup,
    IllegalHelloRetryRequestWithUnsupportedVersion,
    IllegalHelloRetryRequestWithWrongSessionId,
    IllegalMiddleboxChangeCipherSpec,
    IllegalTlsInnerPlaintext,
    IncorrectBinder,
    InvalidMaxEarlyDataSize,
    InvalidKeyShare,
    InvalidSctList,
    KeyEpochWithPendingFragment,
    KeyUpdateReceivedInQuicConnection,
    MessageInterleavedWithHandshakeMessage,
    MissingBinderInPskExtension,
    MissingKeyShare,
    MissingPskModesExtension,
    MissingQuicTransportParameters,
    OfferedDuplicateKeyShares,
    OfferedEarlyDataWithOldProtocolVersion,
    OfferedEmptyApplicationProtocol,
    OfferedIncorrectCompressions,
    PskExtensionMustBeLast,
    PskExtensionWithMismatchedIdsAndBinders,
    RefusedToFollowHelloRetryRequest,
    RejectedEarlyDataInterleavedWithHandshakeMessage,
    ResumptionAttemptedWithVariedEms,
    ResumptionOfferedWithVariedCipherSuite,
    ResumptionOfferedWithVariedEms,
    ResumptionOfferedWithIncompatibleCipherSuite,
    SelectedDifferentCipherSuiteAfterRetry,
    SelectedInvalidPsk,
    SelectedTls12UsingTls13VersionExtension,
    SelectedUnofferedApplicationProtocol,
    SelectedUnofferedCipherSuite,
    SelectedUnofferedCompression,
    SelectedUnofferedKxGroup,
    SelectedUnofferedPsk,
    SelectedUnusableCipherSuiteForVersion,
    ServerHelloMustOfferUncompressedEcPoints,
    ServerNameDifferedOnRetry,
    ServerNameMustContainOneHostName,
    SignedKxWithWrongAlgorithm,
    SignedHandshakeWithUnadvertisedSigScheme,
    TooMuchEarlyDataReceived,
    UnexpectedCleartextExtension,
    UnsolicitedCertExtension,
    UnsolicitedEncryptedExtension,
    UnsolicitedSctList,
    UnsolicitedServerHelloExtension,
    WrongGroupForKeyShare,
}

impl From<PeerMisbehaved> for Error {
    #[inline]
    fn from(e: PeerMisbehaved) -> Self {
        Self::PeerMisbehaved(e)
    }
}

#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
/// The set of cases where we failed to make a connection because a peer
/// doesn't support a TLS version/feature we require.
///
/// This is `non_exhaustive`: we might add or stop using items here in minor
/// versions.
pub enum PeerIncompatible {
    EcPointsExtensionRequired,
    KeyShareExtensionRequired,
    NamedGroupsExtensionRequired,
    NoCertificateRequestSignatureSchemesInCommon,
    NoCipherSuitesInCommon,
    NoEcPointFormatsInCommon,
    NoKxGroupsInCommon,
    NoSignatureSchemesInCommon,
    NullCompressionRequired,
    ServerDoesNotSupportTls12Or13,
    ServerSentHelloRetryRequestWithUnknownExtension,
    ServerTlsVersionIsDisabledByOurConfig,
    SignatureAlgorithmsExtensionRequired,
    SupportedVersionsExtensionRequired,
    Tls12NotOffered,
    Tls12NotOfferedOrEnabled,
    Tls13RequiredForQuic,
    UncompressedEcPointsRequired,
}

impl From<PeerIncompatible> for Error {
    #[inline]
    fn from(e: PeerIncompatible) -> Self {
        Self::PeerIncompatible(e)
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
/// The ways in which certificate validators can express errors.
///
/// Note that the rustls TLS protocol code interprets specifically these
/// error codes to send specific TLS alerts.  Therefore, if a
/// custom certificate validator uses incorrect errors the library as
/// a whole will send alerts that do not match the standard (this is usually
/// a minor issue, but could be misleading).
pub enum CertificateError {
    /// The certificate is not correctly encoded.
    BadEncoding,

    /// The current time is after the `notAfter` time in the certificate.
    Expired,

    /// The current time is before the `notBefore` time in the certificate.
    NotValidYet,

    /// The certificate has been revoked.
    Revoked,

    /// The certificate contains an extension marked critical, but it was
    /// not processed by the certificate validator.
    UnhandledCriticalExtension,

    /// The certificate chain is not issued by a known root certificate.
    UnknownIssuer,

    /// A certificate is not correctly signed by the key of its alleged
    /// issuer.
    BadSignature,

    /// The subject names in an end-entity certificate do not include
    /// the expected name.
    NotValidForName,

    /// The certificate is being used for a different purpose than allowed.
    InvalidPurpose,

    /// The certificate is valid, but the handshake is rejected for other
    /// reasons.
    ApplicationVerificationFailure,

    /// Any other error.
    ///
    /// This can be used by custom verifiers to expose the underlying error
    /// (where they are not better described by the more specific errors
    /// above).
    ///
    /// It is also used by the default verifier in case its error is
    /// not covered by the above common cases.
    ///
    /// Enums holding this variant will never compare equal to each other.
    Other(Arc<dyn StdError + Send + Sync>),
}

impl PartialEq<Self> for CertificateError {
    fn eq(&self, other: &Self) -> bool {
        use CertificateError::*;
        #[allow(clippy::match_like_matches_macro)]
        match (self, other) {
            (BadEncoding, BadEncoding) => true,
            (Expired, Expired) => true,
            (NotValidYet, NotValidYet) => true,
            (Revoked, Revoked) => true,
            (UnhandledCriticalExtension, UnhandledCriticalExtension) => true,
            (UnknownIssuer, UnknownIssuer) => true,
            (BadSignature, BadSignature) => true,
            (NotValidForName, NotValidForName) => true,
            (InvalidPurpose, InvalidPurpose) => true,
            (ApplicationVerificationFailure, ApplicationVerificationFailure) => true,
            _ => false,
        }
    }
}

// The following mapping are heavily referenced in:
// * [OpenSSL Implementation](https://github.com/openssl/openssl/blob/45bb98bfa223efd3258f445ad443f878011450f0/ssl/statem/statem_lib.c#L1434)
// * [BoringSSL Implementation](https://github.com/google/boringssl/blob/583c60bd4bf76d61b2634a58bcda99a92de106cb/ssl/ssl_x509.cc#L1323)
impl From<CertificateError> for AlertDescription {
    fn from(e: CertificateError) -> Self {
        use CertificateError::*;
        match e {
            BadEncoding | UnhandledCriticalExtension | NotValidForName => Self::BadCertificate,
            // RFC 5246/RFC 8446
            // certificate_expired
            //  A certificate has expired or **is not currently valid**.
            Expired | NotValidYet => Self::CertificateExpired,
            Revoked => Self::CertificateRevoked,
            UnknownIssuer => Self::UnknownCA,
            BadSignature => Self::DecryptError,
            InvalidPurpose => Self::UnsupportedCertificate,
            ApplicationVerificationFailure => Self::AccessDenied,
            // RFC 5246/RFC 8446
            // certificate_unknown
            //  Some other (unspecified) issue arose in processing the
            //  certificate, rendering it unacceptable.
            Other(_) => Self::CertificateUnknown,
        }
    }
}

impl From<CertificateError> for Error {
    #[inline]
    fn from(e: CertificateError) -> Self {
        Self::InvalidCertificate(e)
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
/// The ways in which a certificate revocation list (CRL) can be invalid.
pub enum CertRevocationListError {
    /// The CRL had a bad, or unsupported signature from its issuer.
    BadSignature,

    /// The CRL contained an invalid CRL number.
    InvalidCrlNumber,

    /// The CRL contained a revoked certificate with an invalid serial number.
    InvalidRevokedCertSerialNumber,

    /// The CRL issuer does not specify the cRLSign key usage.
    IssuerInvalidForCrl,

    /// The CRL is invalid for some other reason.
    ///
    /// Enums holding this variant will never compare equal to each other.
    Other(Arc<dyn StdError + Send + Sync>),

    /// The CRL is not correctly encoded.
    ParseError,

    /// The CRL is not a v2 X.509 CRL.
    UnsupportedCrlVersion,

    /// The CRL, or a revoked certificate in the CRL, contained an unsupported critical extension.
    UnsupportedCriticalExtension,

    /// The CRL is an unsupported delta CRL, containing only changes relative to another CRL.
    UnsupportedDeltaCrl,

    /// The CRL is an unsupported indirect CRL, containing revoked certificates issued by a CA
    /// other than the issuer of the CRL.
    UnsupportedIndirectCrl,

    /// The CRL contained a revoked certificate with an unsupported revocation reason.
    /// See RFC 5280 Section 5.3.1[^1] for a list of supported revocation reasons.
    ///
    /// [^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5.3.1>
    UnsupportedRevocationReason,
}

impl PartialEq<Self> for CertRevocationListError {
    fn eq(&self, other: &Self) -> bool {
        use CertRevocationListError::*;
        #[allow(clippy::match_like_matches_macro)]
        match (self, other) {
            (BadSignature, BadSignature) => true,
            (InvalidCrlNumber, InvalidCrlNumber) => true,
            (InvalidRevokedCertSerialNumber, InvalidRevokedCertSerialNumber) => true,
            (IssuerInvalidForCrl, IssuerInvalidForCrl) => true,
            (ParseError, ParseError) => true,
            (UnsupportedCrlVersion, UnsupportedCrlVersion) => true,
            (UnsupportedCriticalExtension, UnsupportedCriticalExtension) => true,
            (UnsupportedDeltaCrl, UnsupportedDeltaCrl) => true,
            (UnsupportedIndirectCrl, UnsupportedIndirectCrl) => true,
            (UnsupportedRevocationReason, UnsupportedRevocationReason) => true,
            _ => false,
        }
    }
}

impl From<webpki::Error> for CertRevocationListError {
    fn from(e: webpki::Error) -> Self {
        use webpki::Error::*;
        match e {
            InvalidCrlSignatureForPublicKey
            | UnsupportedCrlSignatureAlgorithm
            | UnsupportedCrlSignatureAlgorithmForPublicKey => Self::BadSignature,
            InvalidCrlNumber => Self::InvalidCrlNumber,
            InvalidSerialNumber => Self::InvalidRevokedCertSerialNumber,
            IssuerNotCrlSigner => Self::IssuerInvalidForCrl,
            MalformedExtensions | BadDer | BadDerTime => Self::ParseError,
            UnsupportedCriticalExtension => Self::UnsupportedCriticalExtension,
            UnsupportedCrlVersion => Self::UnsupportedCrlVersion,
            UnsupportedDeltaCrl => Self::UnsupportedDeltaCrl,
            UnsupportedIndirectCrl => Self::UnsupportedIndirectCrl,
            UnsupportedRevocationReason => Self::UnsupportedRevocationReason,

            _ => Self::Other(Arc::new(e)),
        }
    }
}

impl From<CertRevocationListError> for Error {
    #[inline]
    fn from(e: CertRevocationListError) -> Self {
        Self::InvalidCertRevocationList(e)
    }
}

fn join<T: fmt::Debug>(items: &[T]) -> String {
    items
        .iter()
        .map(|x| format!("{:?}", x))
        .collect::<Vec<String>>()
        .join(" or ")
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InappropriateMessage {
                ref expect_types,
                ref got_type,
            } => write!(
                f,
                "received unexpected message: got {:?} when expecting {}",
                got_type,
                join::<ContentType>(expect_types)
            ),
            Self::InappropriateHandshakeMessage {
                ref expect_types,
                ref got_type,
            } => write!(
                f,
                "received unexpected handshake message: got {:?} when expecting {}",
                got_type,
                join::<HandshakeType>(expect_types)
            ),
            Self::InvalidMessage(ref typ) => {
                write!(f, "received corrupt message of type {:?}", typ)
            }
            Self::PeerIncompatible(ref why) => write!(f, "peer is incompatible: {:?}", why),
            Self::PeerMisbehaved(ref why) => write!(f, "peer misbehaved: {:?}", why),
            Self::AlertReceived(ref alert) => write!(f, "received fatal alert: {:?}", alert),
            Self::InvalidCertificate(ref err) => {
                write!(f, "invalid peer certificate: {:?}", err)
            }
            Self::InvalidCertRevocationList(ref err) => {
                write!(f, "invalid certificate revocation list: {:?}", err)
            }
            Self::NoCertificatesPresented => write!(f, "peer sent no certificates"),
            Self::UnsupportedNameType => write!(f, "presented server name type wasn't supported"),
            Self::DecryptError => write!(f, "cannot decrypt peer's message"),
            Self::EncryptError => write!(f, "cannot encrypt message"),
            Self::PeerSentOversizedRecord => write!(f, "peer sent excess record size"),
            Self::HandshakeNotComplete => write!(f, "handshake not complete"),
            Self::NoApplicationProtocol => write!(f, "peer doesn't support any known protocol"),
            Self::InvalidSct(ref err) => write!(f, "invalid certificate timestamp: {:?}", err),
            Self::FailedToGetCurrentTime => write!(f, "failed to get current time"),
            Self::FailedToGetRandomBytes => write!(f, "failed to get random bytes"),
            Self::BadMaxFragmentSize => {
                write!(f, "the supplied max_fragment_size was too small or large")
            }
            Self::General(ref err) => write!(f, "unexpected error: {}", err),
        }
    }
}

impl From<SystemTimeError> for Error {
    #[inline]
    fn from(_: SystemTimeError) -> Self {
        Self::FailedToGetCurrentTime
    }
}

impl StdError for Error {}

impl From<rand::GetRandomFailed> for Error {
    fn from(_: rand::GetRandomFailed) -> Self {
        Self::FailedToGetRandomBytes
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, InvalidMessage};
    use crate::error::CertRevocationListError;

    #[test]
    fn certificate_error_equality() {
        use super::CertificateError::*;
        assert_eq!(BadEncoding, BadEncoding);
        assert_eq!(Expired, Expired);
        assert_eq!(NotValidYet, NotValidYet);
        assert_eq!(Revoked, Revoked);
        assert_eq!(UnhandledCriticalExtension, UnhandledCriticalExtension);
        assert_eq!(UnknownIssuer, UnknownIssuer);
        assert_eq!(BadSignature, BadSignature);
        assert_eq!(NotValidForName, NotValidForName);
        assert_eq!(InvalidPurpose, InvalidPurpose);
        assert_eq!(
            ApplicationVerificationFailure,
            ApplicationVerificationFailure
        );
        let other = Other(std::sync::Arc::from(Box::from("")));
        assert_ne!(other, other);
        assert_ne!(BadEncoding, Expired);
    }

    #[test]
    fn crl_error_equality() {
        use super::CertRevocationListError::*;
        assert_eq!(BadSignature, BadSignature);
        assert_eq!(InvalidCrlNumber, InvalidCrlNumber);
        assert_eq!(
            InvalidRevokedCertSerialNumber,
            InvalidRevokedCertSerialNumber
        );
        assert_eq!(IssuerInvalidForCrl, IssuerInvalidForCrl);
        assert_eq!(ParseError, ParseError);
        assert_eq!(UnsupportedCriticalExtension, UnsupportedCriticalExtension);
        assert_eq!(UnsupportedCrlVersion, UnsupportedCrlVersion);
        assert_eq!(UnsupportedDeltaCrl, UnsupportedDeltaCrl);
        assert_eq!(UnsupportedIndirectCrl, UnsupportedIndirectCrl);
        assert_eq!(UnsupportedRevocationReason, UnsupportedRevocationReason);
        let other = Other(std::sync::Arc::from(Box::from("")));
        assert_ne!(other, other);
        assert_ne!(BadSignature, InvalidCrlNumber);
    }

    #[test]
    fn crl_error_from_webpki() {
        use super::CertRevocationListError::*;
        let testcases = &[
            (webpki::Error::InvalidCrlSignatureForPublicKey, BadSignature),
            (
                webpki::Error::UnsupportedCrlSignatureAlgorithm,
                BadSignature,
            ),
            (
                webpki::Error::UnsupportedCrlSignatureAlgorithmForPublicKey,
                BadSignature,
            ),
            (webpki::Error::InvalidCrlNumber, InvalidCrlNumber),
            (
                webpki::Error::InvalidSerialNumber,
                InvalidRevokedCertSerialNumber,
            ),
            (webpki::Error::IssuerNotCrlSigner, IssuerInvalidForCrl),
            (webpki::Error::MalformedExtensions, ParseError),
            (webpki::Error::BadDer, ParseError),
            (webpki::Error::BadDerTime, ParseError),
            (
                webpki::Error::UnsupportedCriticalExtension,
                UnsupportedCriticalExtension,
            ),
            (webpki::Error::UnsupportedCrlVersion, UnsupportedCrlVersion),
            (webpki::Error::UnsupportedDeltaCrl, UnsupportedDeltaCrl),
            (
                webpki::Error::UnsupportedIndirectCrl,
                UnsupportedIndirectCrl,
            ),
            (
                webpki::Error::UnsupportedRevocationReason,
                UnsupportedRevocationReason,
            ),
        ];
        for t in testcases {
            assert_eq!(
                <webpki::Error as Into<CertRevocationListError>>::into(t.0),
                t.1
            );
        }

        assert!(matches!(
            <webpki::Error as Into<CertRevocationListError>>::into(
                webpki::Error::NameConstraintViolation
            ),
            Other(_)
        ));
    }

    #[test]
    fn smoke() {
        use crate::enums::{AlertDescription, ContentType, HandshakeType};
        use sct;

        let all = vec![
            Error::InappropriateMessage {
                expect_types: vec![ContentType::Alert],
                got_type: ContentType::Handshake,
            },
            Error::InappropriateHandshakeMessage {
                expect_types: vec![HandshakeType::ClientHello, HandshakeType::Finished],
                got_type: HandshakeType::ServerHello,
            },
            Error::InvalidMessage(InvalidMessage::InvalidCcs),
            Error::NoCertificatesPresented,
            Error::DecryptError,
            super::PeerIncompatible::Tls12NotOffered.into(),
            super::PeerMisbehaved::UnsolicitedCertExtension.into(),
            Error::AlertReceived(AlertDescription::ExportRestriction),
            super::CertificateError::Expired.into(),
            Error::InvalidSct(sct::Error::MalformedSct),
            Error::General("undocumented error".to_string()),
            Error::FailedToGetCurrentTime,
            Error::FailedToGetRandomBytes,
            Error::HandshakeNotComplete,
            Error::PeerSentOversizedRecord,
            Error::NoApplicationProtocol,
            Error::BadMaxFragmentSize,
            Error::InvalidCertRevocationList(CertRevocationListError::BadSignature),
        ];

        for err in all {
            println!("{:?}:", err);
            println!("  fmt '{}'", err);
        }
    }

    #[test]
    fn rand_error_mapping() {
        use super::rand;
        let err: Error = rand::GetRandomFailed.into();
        assert_eq!(err, Error::FailedToGetRandomBytes);
    }

    #[test]
    fn time_error_mapping() {
        use std::time::SystemTime;

        let time_error = SystemTime::UNIX_EPOCH
            .duration_since(SystemTime::now())
            .unwrap_err();
        let err: Error = time_error.into();
        assert_eq!(err, Error::FailedToGetCurrentTime);
    }
}
