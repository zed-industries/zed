// Copyright 2016 Joseph Birr-Pixton.
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

use webpki::KeyUsage;

static ALL_SIGALGS: &[&webpki::SignatureAlgorithm] = &[
    &webpki::ECDSA_P256_SHA256,
    &webpki::ECDSA_P256_SHA384,
    &webpki::ECDSA_P384_SHA256,
    &webpki::ECDSA_P384_SHA384,
    &webpki::ED25519,
    #[cfg(feature = "alloc")]
    &webpki::RSA_PKCS1_2048_8192_SHA256,
    #[cfg(feature = "alloc")]
    &webpki::RSA_PKCS1_2048_8192_SHA384,
    #[cfg(feature = "alloc")]
    &webpki::RSA_PKCS1_2048_8192_SHA512,
    #[cfg(feature = "alloc")]
    &webpki::RSA_PKCS1_3072_8192_SHA384,
];

/* Checks we can verify netflix's cert chain.  This is notable
 * because they're rooted at a Verisign v1 root. */
#[cfg(feature = "alloc")]
#[test]
pub fn netflix() {
    let ee: &[u8] = include_bytes!("netflix/ee.der");
    let inter = include_bytes!("netflix/inter.der");
    let ca = include_bytes!("netflix/ca.der");

    let anchors = [webpki::TrustAnchor::try_from_cert_der(ca).unwrap()];

    let time = webpki::Time::from_seconds_since_unix_epoch(1_492_441_716); // 2017-04-17T15:08:36Z

    let cert = webpki::EndEntityCert::try_from(ee).unwrap();
    assert_eq!(
        Ok(()),
        cert.verify_for_usage(
            ALL_SIGALGS,
            &anchors,
            &[inter],
            time,
            KeyUsage::server_auth(),
            &[]
        )
    );
}

/* This is notable because it is a popular use of IP address subjectAltNames. */
#[cfg(feature = "alloc")]
#[test]
pub fn cloudflare_dns() {
    let ee: &[u8] = include_bytes!("cloudflare_dns/ee.der");
    let inter = include_bytes!("cloudflare_dns/inter.der");
    let ca = include_bytes!("cloudflare_dns/ca.der");

    let anchors = [webpki::TrustAnchor::try_from_cert_der(ca).unwrap()];

    let time = webpki::Time::from_seconds_since_unix_epoch(1_663_495_771);

    let cert = webpki::EndEntityCert::try_from(ee).unwrap();
    assert_eq!(
        Ok(()),
        cert.verify_for_usage(
            ALL_SIGALGS,
            &anchors,
            &[inter],
            time,
            KeyUsage::server_auth(),
            &[]
        )
    );

    let check_name = |name: &str| {
        let subject_name_ref = webpki::SubjectNameRef::try_from_ascii_str(name).unwrap();
        assert_eq!(
            Ok(()),
            cert.verify_is_valid_for_subject_name(subject_name_ref)
        );
        println!("{:?} ok as name", name);
    };

    let check_addr = |addr: &str| {
        let subject_name_ref = webpki::SubjectNameRef::try_from_ascii(addr.as_bytes()).unwrap();
        assert_eq!(
            Ok(()),
            cert.verify_is_valid_for_subject_name(subject_name_ref)
        );
        println!("{:?} ok as address", addr);
    };

    check_name("cloudflare-dns.com");
    check_name("wildcard.cloudflare-dns.com");
    check_name("one.one.one.one");
    check_addr("1.1.1.1");
    check_addr("1.0.0.1");
    check_addr("162.159.36.1");
    check_addr("162.159.46.1");
    check_addr("2606:4700:4700:0000:0000:0000:0000:1111");
    check_addr("2606:4700:4700:0000:0000:0000:0000:1001");
    check_addr("2606:4700:4700:0000:0000:0000:0000:0064");
    check_addr("2606:4700:4700:0000:0000:0000:0000:6400");
}

#[cfg(feature = "alloc")]
#[test]
pub fn wpt() {
    let ee: &[u8] = include_bytes!("wpt/ee.der");
    let ca = include_bytes!("wpt/ca.der");

    let anchors = [webpki::TrustAnchor::try_from_cert_der(ca).unwrap()];

    let time = webpki::Time::from_seconds_since_unix_epoch(1_619_256_684); // 2021-04-24T09:31:24Z

    let cert = webpki::EndEntityCert::try_from(ee).unwrap();
    assert_eq!(
        Ok(()),
        cert.verify_for_usage(
            ALL_SIGALGS,
            &anchors,
            &[],
            time,
            KeyUsage::server_auth(),
            &[]
        )
    );
}

#[test]
pub fn ed25519() {
    let ee: &[u8] = include_bytes!("ed25519/ee.der");
    let ca = include_bytes!("ed25519/ca.der");

    let anchors = [webpki::TrustAnchor::try_from_cert_der(ca).unwrap()];

    let time = webpki::Time::from_seconds_since_unix_epoch(1_547_363_522); // 2019-01-13T07:12:02Z

    let cert = webpki::EndEntityCert::try_from(ee).unwrap();
    assert_eq!(
        Ok(()),
        cert.verify_for_usage(
            ALL_SIGALGS,
            &anchors,
            &[],
            time,
            KeyUsage::server_auth(),
            &[]
        )
    );
}

#[test]
#[cfg(feature = "alloc")]
fn critical_extensions() {
    let root = include_bytes!("critical_extensions/root-cert.der");
    let ca = include_bytes!("critical_extensions/ca-cert.der");

    let time = webpki::Time::from_seconds_since_unix_epoch(1_670_779_098);
    let anchors = [webpki::TrustAnchor::try_from_cert_der(root).unwrap()];

    let ee = include_bytes!("critical_extensions/ee-cert-noncrit-unknown-ext.der");
    let res = webpki::EndEntityCert::try_from(&ee[..]).and_then(|cert| {
        cert.verify_for_usage(
            ALL_SIGALGS,
            &anchors,
            &[ca],
            time,
            KeyUsage::server_auth(),
            &[],
        )
    });
    assert_eq!(res, Ok(()), "accept non-critical unknown extension");

    let ee = include_bytes!("critical_extensions/ee-cert-crit-unknown-ext.der");
    let res = webpki::EndEntityCert::try_from(&ee[..]).and_then(|cert| {
        cert.verify_for_usage(
            ALL_SIGALGS,
            &anchors,
            &[ca],
            time,
            KeyUsage::server_auth(),
            &[],
        )
    });
    assert_eq!(
        res,
        Err(webpki::Error::UnsupportedCriticalExtension),
        "reject critical unknown extension"
    );
}

#[test]
fn read_root_with_zero_serial() {
    let ca = include_bytes!("misc/serial_zero.der");
    let _ =
        webpki::TrustAnchor::try_from_cert_der(ca).expect("godaddy cert should parse as anchor");
}

#[test]
fn read_root_with_neg_serial() {
    let ca = include_bytes!("misc/serial_neg.der");
    let _ = webpki::TrustAnchor::try_from_cert_der(ca).expect("idcat cert should parse as anchor");
}

#[test]
#[cfg(feature = "alloc")]
fn read_ee_with_neg_serial() {
    let ca: &[u8] = include_bytes!("misc/serial_neg_ca.der");
    let ee: &[u8] = include_bytes!("misc/serial_neg_ee.der");

    let anchors = [webpki::TrustAnchor::try_from_cert_der(ca).unwrap()];

    let time = webpki::Time::from_seconds_since_unix_epoch(1_667_401_500); // 2022-11-02T15:05:00Z

    let cert = webpki::EndEntityCert::try_from(ee).unwrap();
    assert_eq!(
        Ok(()),
        cert.verify_for_usage(
            ALL_SIGALGS,
            &anchors,
            &[],
            time,
            KeyUsage::server_auth(),
            &[]
        )
    );
}

#[test]
#[cfg(feature = "alloc")]
fn read_ee_with_large_pos_serial() {
    let ee: &[u8] = include_bytes!("misc/serial_large_positive.der");

    webpki::EndEntityCert::try_from(ee).expect("should parse 20-octet positive serial number");
}

#[cfg(feature = "std")]
#[test]
fn time_constructor() {
    let _ =
        <webpki::Time as TryFrom<std::time::SystemTime>>::try_from(std::time::SystemTime::now())
            .unwrap();
}

#[cfg(feature = "alloc")]
#[test]
pub fn list_netflix_names() {
    let ee = include_bytes!("netflix/ee.der");

    expect_cert_dns_names(
        ee,
        &[
            "account.netflix.com",
            "ca.netflix.com",
            "netflix.ca",
            "netflix.com",
            "signup.netflix.com",
            "www.netflix.ca",
            "www1.netflix.com",
            "www2.netflix.com",
            "www3.netflix.com",
            "develop-stage.netflix.com",
            "release-stage.netflix.com",
            "www.netflix.com",
        ],
    );
}

#[cfg(feature = "alloc")]
#[test]
pub fn invalid_subject_alt_names() {
    // same as netflix ee certificate, but with the last name in the list
    // changed to 'www.netflix:com'
    let data = include_bytes!("misc/invalid_subject_alternative_name.der");

    expect_cert_dns_names(
        data,
        &[
            "account.netflix.com",
            "ca.netflix.com",
            "netflix.ca",
            "netflix.com",
            "signup.netflix.com",
            "www.netflix.ca",
            "www1.netflix.com",
            "www2.netflix.com",
            "www3.netflix.com",
            "develop-stage.netflix.com",
            "release-stage.netflix.com",
            // NOT 'www.netflix:com'
        ],
    );
}

#[cfg(feature = "alloc")]
#[test]
pub fn wildcard_subject_alternative_names() {
    // same as netflix ee certificate, but with the last name in the list
    // changed to 'ww*.netflix:com'
    let data = include_bytes!("misc/dns_names_and_wildcards.der");

    expect_cert_dns_names(
        data,
        &[
            "account.netflix.com",
            "*.netflix.com",
            "netflix.ca",
            "netflix.com",
            "signup.netflix.com",
            "www.netflix.ca",
            "www1.netflix.com",
            "www2.netflix.com",
            "www3.netflix.com",
            "develop-stage.netflix.com",
            "release-stage.netflix.com",
            "www.netflix.com",
        ],
    );
}

#[cfg(feature = "alloc")]
fn expect_cert_dns_names(data: &[u8], expected_names: &[&str]) {
    use std::collections::HashSet;

    let cert = webpki::EndEntityCert::try_from(data)
        .expect("should parse end entity certificate correctly");

    let expected_names: HashSet<_> = expected_names.iter().cloned().collect();

    let mut actual_names = cert
        .dns_names()
        .expect("should get all DNS names correctly for end entity cert")
        .collect::<Vec<_>>();

    // Ensure that converting the list to a set doesn't throw away
    // any duplicates that aren't supposed to be there
    assert_eq!(actual_names.len(), expected_names.len());

    let actual_names: std::collections::HashSet<&str> =
        actual_names.drain(..).map(|name| name.into()).collect();

    assert_eq!(actual_names, expected_names);
}

#[cfg(feature = "alloc")]
#[test]
pub fn no_subject_alt_names() {
    let data = include_bytes!("misc/no_subject_alternative_name.der");

    let cert = webpki::EndEntityCert::try_from(&data[..])
        .expect("should parse end entity certificate correctly");

    let names = cert
        .dns_names()
        .expect("we should get a result even without subjectAltNames");

    assert!(names.collect::<Vec<_>>().is_empty());
}
