#[cfg(feature = "ffi-testing")]
pub mod ffi;

use std::error::Error as StdError;
use std::time::Duration;

mod verification_real_world;

mod verification_mock;

use rustls::{pki_types, CertificateError, Error as TlsError, Error::InvalidCertificate};

struct TestCase<'a, E: StdError> {
    /// The name of the server we're connecting to.
    pub reference_id: &'a str,

    /// The certificates presented by the TLS server, in the same order.
    pub chain: &'a [&'a [u8]],

    /// The stapled OCSP response given to us by Rustls, if any.
    pub stapled_ocsp: Option<&'a [u8]>,

    /// The time to use as the current time for verification.
    pub verification_time: pki_types::UnixTime,

    pub expected_result: Result<(), TlsError>,

    /// An error that should be present inside an expected `CertificateError::Other` variant.
    ///
    /// Set this if the error being tested uses `CertificateError::Other` and not statically known
    /// variants in [TlsError]
    #[allow(dead_code)]
    pub other_error: Option<E>,
}

pub fn assert_cert_error_eq<E: StdError + PartialEq + 'static>(
    result: &Result<(), TlsError>,
    expected: &Result<(), TlsError>,
    expected_err: Option<&E>,
) {
    // If the expected error is an "Other" CertificateError we can't directly assert equality, we rely
    // on the test caller to provide the correct value to compare.
    if let Err(InvalidCertificate(CertificateError::Other(err))) = &expected {
        let expected_err = expected_err.expect("error not provided for `Other` case handling");
        let err: &E = err
            .0
            .downcast_ref()
            .expect("incorrect `Other` inner error kind");
        assert_eq!(err, expected_err);
    } else {
        assert_eq!(result, expected);
    }
}

/// Return a fixed [`pki_types::UnixTime`] for certificate validation purposes.
///
/// We fix the "now" value used for certificate validation to a fixed point in time at which
/// we know the test certificates are valid. This must be updated if the mock certificates
/// are regenerated.
pub(crate) fn verification_time() -> pki_types::UnixTime {
    // Thur, 06 Feb 2025 13:54:00 UTC
    pki_types::UnixTime::since_unix_epoch(Duration::from_secs(1_738_850_040))
}

fn ensure_global_state() {
    let _ = rustls::crypto::ring::default_provider().install_default();
}
