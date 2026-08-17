//! Tests of certificate verification that require our own test CA to be
//! trusted.
//!
//! # Re-generating the test data
//!
//! `cd src/tests/verification_mock && go run ca.go`
//!
//! # Repeatability and Self-containedness
//!
//! These tests are only supported on platforms where we have implemented the
//! ability to trust a CA (only) for the duration of a test, without modifying
//! the operating system's trust store--i.e. without affecting the security of
//! any parts of the system outside of these tests. See the `#![cfg(...)]`
//! immediately below to see which platforms run these tests.

#![cfg(all(
    any(windows, unix, target_os = "android"),
    // These OSes require a simulator runtime and bundle.
    not(target_os = "tvos"),
    not(target_os = "watchos"),
    not(target_os = "visionos")
))]

use std::convert::TryFrom;
use std::net::IpAddr;
#[cfg(not(any(target_vendor = "apple", windows)))]
use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::Arc;

use rustls::client::danger::ServerCertVerifier;
use rustls::pki_types;
#[cfg(not(any(target_vendor = "apple", windows)))]
use rustls::pki_types::{DnsName, ServerName};
use rustls::{CertificateError, Error as TlsError, OtherError};

use super::TestCase;
use crate::tests::{assert_cert_error_eq, ensure_global_state, verification_time};
use crate::verification::{EkuError, Verifier};

macro_rules! mock_root_test_cases {
    { $( $name:ident [ $target:meta ] => $test_case:expr ),+ , } => {
        mock_root_test_cases!(@ $($name [ $target ] => $test_case),+,);

        #[cfg(test)]
        mod tests {
            $(
                #[cfg($target)]
                #[test]
                pub fn $name() {
                    super::$name()
                }
            )+
        }

        #[cfg(feature = "ffi-testing")]
        pub static ALL_TEST_CASES: &'static [fn()] = &[
            $(
                #[cfg($target)]
                $name,
            )+

        ];
    };

    {@ $( $name:ident [ $target:meta ] => $test_case:expr ),+ , } => {
        $(
            #[cfg($target)]
            pub(super) fn $name() {
                test_with_mock_root(&$test_case, Roots::OnlyExtra);
                #[cfg(all($target, not(target_os = "android")))]
                test_with_mock_root(&$test_case, Roots::ExtraAndPlatform);
            }
        )+
    };
}

macro_rules! no_error {
    () => {
        None::<std::convert::Infallible>
    };
}

const ROOT1: &[u8] = include_bytes!("root1.crt");
const ROOT1_INT1: &[u8] = include_bytes!("root1-int1.crt");
const ROOT1_INT1_EXAMPLE_COM_GOOD: &[u8] = include_bytes!("root1-int1-ee_example.com-good.crt");
const ROOT1_INT1_LOCALHOST_IPV4_GOOD: &[u8] = include_bytes!("root1-int1-ee_127.0.0.1-good.crt");
const ROOT1_INT1_LOCALHOST_IPV6_GOOD: &[u8] = include_bytes!("root1-int1-ee_1-good.crt");

const EXAMPLE_COM: &str = "example.com";
const LOCALHOST_IPV4: &str = "127.0.0.1";
const LOCALHOST_IPV6: &str = "::1";

#[cfg(any(test, feature = "ffi-testing"))]
#[cfg_attr(feature = "ffi-testing", allow(dead_code))]
pub(super) fn verification_without_mock_root() {
    ensure_global_state();
    // Since Rustls 0.22 constructing a webpki verifier (like the one backing Verifier on unix
    // systems) without any roots produces `OtherError(NoRootAnchors)` - since our FreeBSD CI
    // runner fails to find any roots with openssl-probe we need to provide webpki-root-certs here
    // or the test will fail with the `OtherError` instead of the expected `CertificateError`.
    #[cfg(target_os = "freebsd")]
    let verifier =
        Verifier::new_with_extra_roots(webpki_root_certs::TLS_SERVER_ROOT_CERTS.iter().cloned())
            .unwrap();

    #[cfg(not(target_os = "freebsd"))]
    let verifier = Verifier::new();

    let server_name = pki_types::ServerName::try_from(EXAMPLE_COM).unwrap();
    let end_entity = pki_types::CertificateDer::from(ROOT1_INT1_EXAMPLE_COM_GOOD);
    let intermediates = [pki_types::CertificateDer::from(ROOT1_INT1)];

    // Fails because the server cert has no trust root in Windows, and can't since it uses a self-signed CA.
    // Similarly on UNIX platforms using the Webpki verifier, it can't fetch extra certificates through
    // AIA chasing or other mechanisms, and so we know this test will correctly verify an unknown
    // root in a chain fails validation.
    let result = verifier.verify_server_cert(
        &end_entity,
        &intermediates,
        &server_name,
        &[],
        verification_time(),
    );

    assert_eq!(
        result.map(|_| ()),
        Err(TlsError::InvalidCertificate(
            CertificateError::UnknownIssuer
        ))
    );
}

#[test]
fn test_verification_without_mock_root() {
    verification_without_mock_root()
}

// Note: Android does not currently support IP address hosts, so these tests are disabled for
// Android.
// Verifies that our test trust anchor(s) are not trusted when `Verifier::new()`
// is used.
mock_root_test_cases! {
    valid_no_stapling_dns [ any(windows, unix) ] => TestCase {
        reference_id: EXAMPLE_COM,
        chain: &[ROOT1_INT1_EXAMPLE_COM_GOOD, ROOT1_INT1],
        stapled_ocsp: None,
        verification_time: verification_time(),
        expected_result: Ok(()),
        other_error: no_error!(),
    },
    valid_no_stapling_ipv4 [ any(windows, unix) ] => TestCase {
        reference_id: LOCALHOST_IPV4,
        chain: &[ROOT1_INT1_LOCALHOST_IPV4_GOOD, ROOT1_INT1],
        stapled_ocsp: None,
        verification_time: verification_time(),
        expected_result: Ok(()),
        other_error: no_error!(),
    },
    valid_no_stapling_ipv6 [ any(windows, unix) ] => TestCase {
        reference_id: LOCALHOST_IPV6,
        chain: &[ROOT1_INT1_LOCALHOST_IPV6_GOOD, ROOT1_INT1],
        stapled_ocsp: None,
        verification_time: verification_time(),
        expected_result: Ok(()),
        other_error: no_error!(),
    },
    valid_stapled_good_dns [ any(windows, unix) ] => TestCase {
        reference_id: EXAMPLE_COM,
        chain: &[ROOT1_INT1_EXAMPLE_COM_GOOD, ROOT1_INT1],
        stapled_ocsp: Some(include_bytes!("root1-int1-ee_example.com-good.ocsp")),
        verification_time: verification_time(),
        expected_result: Ok(()),
        other_error: no_error!(),
    },
    valid_stapled_good_ipv4 [ any(windows, unix) ] => TestCase {
        reference_id: LOCALHOST_IPV4,
        chain: &[ROOT1_INT1_LOCALHOST_IPV4_GOOD, ROOT1_INT1],
        stapled_ocsp: Some(include_bytes!("root1-int1-ee_127.0.0.1-good.ocsp")),
        verification_time: verification_time(),
        expected_result: Ok(()),
        other_error: no_error!(),
    },
    valid_stapled_good_ipv6 [ any(windows, unix) ] => TestCase {
        reference_id: LOCALHOST_IPV6,
        chain: &[ROOT1_INT1_LOCALHOST_IPV6_GOOD, ROOT1_INT1],
        stapled_ocsp: Some(include_bytes!("root1-int1-ee_1-good.ocsp")),
        verification_time: verification_time(),
        expected_result: Ok(()),
        other_error: no_error!(),
    },

    // The revocation tests use a separate certificate from the one used in the "good" case to deal
    // with operating systems with validation data caches (e.g. Windows).
    // Linux is not included, since the webpki verifier does not presently support OCSP revocation
    // checking.

    // Check that self-signed certificates, which may or may not be revokved, do not return any
    // kind of revocation error. It is expected that non-public certificates without revocation information
    // have no revocation checking performed across platforms.
    revoked_dns [ any(windows, target_os = "android", target_vendor = "apple") ] => TestCase {
        reference_id: EXAMPLE_COM,
        chain: &[include_bytes!("root1-int1-ee_example.com-revoked.crt"), ROOT1_INT1],
        stapled_ocsp: None,
        verification_time: verification_time(),
        expected_result: Ok(()),
        other_error: no_error!(),
    },
    stapled_revoked_dns [ any(windows, target_os = "android", target_vendor = "apple") ] => TestCase {
        reference_id: EXAMPLE_COM,
        chain: &[include_bytes!("root1-int1-ee_example.com-revoked.crt"), ROOT1_INT1],
        stapled_ocsp: Some(include_bytes!("root1-int1-ee_example.com-revoked.ocsp")),
        verification_time: verification_time(),
        expected_result: Err(TlsError::InvalidCertificate(CertificateError::Revoked)),
        other_error: no_error!(),
    },
    stapled_revoked_ipv4 [ any(windows, target_os = "android", target_vendor = "apple") ] => TestCase {
        reference_id: LOCALHOST_IPV4,
        chain: &[include_bytes!("root1-int1-ee_127.0.0.1-revoked.crt"), ROOT1_INT1],
        stapled_ocsp: Some(include_bytes!("root1-int1-ee_127.0.0.1-revoked.ocsp")),
        verification_time: verification_time(),
        expected_result: Err(TlsError::InvalidCertificate(CertificateError::Revoked)),
        other_error: no_error!(),
    },
    stapled_revoked_ipv6 [ any(windows, target_os = "android", target_vendor = "apple") ] => TestCase {
        reference_id: LOCALHOST_IPV6,
        chain: &[include_bytes!("root1-int1-ee_1-revoked.crt"), ROOT1_INT1],
        stapled_ocsp: Some(include_bytes!("root1-int1-ee_1-revoked.ocsp")),
        verification_time: verification_time(),
        expected_result: Err(TlsError::InvalidCertificate(CertificateError::Revoked)),
        other_error: no_error!(),
    },
    // Validation fails with no intermediate (that can't be fetched
    // with AIA because there's no AIA issuer field in the certificate).
    // (AIA is an extension that allows downloading of missing data,
    // like missing certificates, during validation; see
    // https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.2.1).
    ee_only_dns [ any(windows, unix) ] => TestCase {
        reference_id: EXAMPLE_COM,
        chain: &[ROOT1_INT1_EXAMPLE_COM_GOOD],
        stapled_ocsp: None,
        verification_time: verification_time(),
        expected_result: Err(TlsError::InvalidCertificate(CertificateError::UnknownIssuer)),
        other_error: no_error!(),
    },
    ee_only_ipv4 [ any(windows, unix) ] => TestCase {
        reference_id: LOCALHOST_IPV4,
        chain: &[ROOT1_INT1_LOCALHOST_IPV4_GOOD],
        stapled_ocsp: None,
        verification_time: verification_time(),
        expected_result: Err(TlsError::InvalidCertificate(CertificateError::UnknownIssuer)),
        other_error: no_error!(),
    },
    ee_only_ipv6 [ any(windows, unix) ] => TestCase {
        reference_id: LOCALHOST_IPV6,
        chain: &[ROOT1_INT1_LOCALHOST_IPV6_GOOD],
        stapled_ocsp: None,
        verification_time: verification_time(),
        expected_result: Err(TlsError::InvalidCertificate(CertificateError::UnknownIssuer)),
        other_error: no_error!(),
    },
    // Validation fails when the certificate isn't valid for the reference ID.
    domain_mismatch_dns [ any(windows, unix) ] => TestCase {
        reference_id: "example.org",
        chain: &[ROOT1_INT1_EXAMPLE_COM_GOOD, ROOT1_INT1],
        stapled_ocsp: None,
        verification_time: verification_time(),
        #[cfg(not(any(target_vendor = "apple", windows)))]
        expected_result: Err(TlsError::InvalidCertificate(CertificateError::NotValidForNameContext {
            expected: ServerName::DnsName(DnsName::try_from("example.org").unwrap()),
            presented: vec!["DnsName(\"example.com\")".to_owned()]
        })),
        #[cfg(any(target_vendor = "apple", windows))]
        expected_result: Err(TlsError::InvalidCertificate(CertificateError::NotValidForName)),
        other_error: no_error!(),
    },
    domain_mismatch_ipv4 [ any(windows, unix) ] => TestCase {
        reference_id: "198.168.0.1",
        chain: &[ROOT1_INT1_LOCALHOST_IPV4_GOOD, ROOT1_INT1],
        stapled_ocsp: None,
        verification_time: verification_time(),
        #[cfg(not(any(target_vendor = "apple", windows)))]
        expected_result: Err(TlsError::InvalidCertificate(CertificateError::NotValidForNameContext {
            expected: ServerName::IpAddress(pki_types::IpAddr::V4(Ipv4Addr::from([198, 168, 0, 1]).into())),
            presented: vec!["IpAddress(127.0.0.1)".to_owned()],
        })),
        #[cfg(any(target_vendor = "apple", windows))]
        expected_result: Err(TlsError::InvalidCertificate(CertificateError::NotValidForName)),
        other_error: no_error!(),
    },
    domain_mismatch_ipv6 [ any(windows, unix) ] => TestCase {
        reference_id: "::ffff:c6a8:1",
        chain: &[ROOT1_INT1_LOCALHOST_IPV6_GOOD, ROOT1_INT1],
        stapled_ocsp: None,
        verification_time: verification_time(),
        #[cfg(not(any(target_vendor = "apple", windows)))]
        expected_result: Err(TlsError::InvalidCertificate(CertificateError::NotValidForNameContext {
            expected: ServerName::IpAddress(pki_types::IpAddr::V6(Ipv6Addr::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 198, 168, 0, 1]).into())),
            presented: vec!["IpAddress(0::1)".to_owned()],
        })),
        #[cfg(any(target_vendor = "apple", windows))]
        expected_result: Err(TlsError::InvalidCertificate(CertificateError::NotValidForName)),
        other_error: no_error!(),
    },
    wrong_eku_dns [ any(windows, unix) ] => TestCase {
        reference_id: EXAMPLE_COM,
        chain: &[include_bytes!("root1-int1-ee_example.com-wrong_eku.crt"), ROOT1_INT1],
        stapled_ocsp: None,
        verification_time: verification_time(),
        expected_result: Err(TlsError::InvalidCertificate(
            CertificateError::Other(OtherError(Arc::from(EkuError))))),
        other_error: Some(EkuError),
    },
    wrong_eku_ipv4 [ any(windows, unix) ] => TestCase {
        reference_id: LOCALHOST_IPV4,
        chain: &[include_bytes!("root1-int1-ee_127.0.0.1-wrong_eku.crt"), ROOT1_INT1],
        stapled_ocsp: None,
        verification_time: verification_time(),
        expected_result: Err(TlsError::InvalidCertificate(
            CertificateError::Other(OtherError(Arc::from(EkuError))))),
        other_error: Some(EkuError),
    },
    wrong_eku_ipv6 [ any(windows, unix) ] => TestCase {
        reference_id: LOCALHOST_IPV6,
        chain: &[include_bytes!("root1-int1-ee_1-wrong_eku.crt"), ROOT1_INT1],
        stapled_ocsp: None,
        verification_time: verification_time(),
        expected_result: Err(TlsError::InvalidCertificate(
            CertificateError::Other(OtherError(Arc::from(EkuError))))),
        other_error: Some(EkuError),
    },
}

fn test_with_mock_root<E: std::error::Error + PartialEq + 'static>(
    test_case: &TestCase<E>,
    root_src: Roots,
) {
    ensure_global_state();
    log::info!("verifying {:?}", test_case.expected_result);

    let verifier = match root_src {
        Roots::OnlyExtra => Verifier::new_with_fake_root(ROOT1), // TODO: time
        #[cfg(not(target_os = "android"))]
        Roots::ExtraAndPlatform => Verifier::new_with_extra_roots([ROOT1.into()]).unwrap(),
    };
    let mut chain = test_case
        .chain
        .iter()
        .map(|bytes| pki_types::CertificateDer::from(*bytes));

    let end_entity = chain.next().unwrap();
    let intermediates: Vec<pki_types::CertificateDer<'_>> = chain.collect();

    let server_name = pki_types::ServerName::try_from(test_case.reference_id).unwrap();

    if test_case.reference_id.parse::<IpAddr>().is_ok() {
        assert!(matches!(server_name, pki_types::ServerName::IpAddress(_)));
    } else {
        assert!(matches!(server_name, pki_types::ServerName::DnsName(_)));
    }

    let result = verifier.verify_server_cert(
        &end_entity,
        &intermediates,
        &server_name,
        test_case.stapled_ocsp.unwrap_or(&[]),
        test_case.verification_time,
    );

    assert_cert_error_eq(
        &result.map(|_| ()),
        &test_case.expected_result,
        test_case.other_error.as_ref(),
    );
    // TODO: get into specifics of errors returned when it fails.
}

enum Roots {
    /// Test with only extra roots, without loading the platform trust store.
    ///
    /// We want to keep things reproducible given the background-managed nature of trust roots on platforms.
    OnlyExtra,
    /// Test with loading the extra roots and the platform trust store.
    ///
    /// Right now, not all platforms are supported.
    #[cfg(not(target_os = "android"))]
    ExtraAndPlatform,
}
