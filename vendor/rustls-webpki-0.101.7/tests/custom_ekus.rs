#![cfg(feature = "alloc")]

use webpki::KeyUsage;

fn check_cert(
    ee: &[u8],
    ca: &[u8],
    eku: KeyUsage,
    time: webpki::Time,
    result: Result<(), webpki::Error>,
) {
    let anchors = [webpki::TrustAnchor::try_from_cert_der(ca).unwrap()];
    let algs = &[
        &webpki::RSA_PKCS1_2048_8192_SHA256,
        &webpki::ECDSA_P256_SHA256,
    ];

    let cert = webpki::EndEntityCert::try_from(ee).unwrap();

    assert_eq!(
        cert.verify_for_usage(algs, &anchors, &[], time, eku, &[]),
        result
    );
}

#[test]
pub fn verify_custom_eku_mdoc() {
    let err = Err(webpki::Error::RequiredEkuNotFound);
    let time = webpki::Time::from_seconds_since_unix_epoch(1609459200); //  Jan 1 01:00:00 CET 2021

    let ee = include_bytes!("misc/mdoc_eku.ee.der");
    let ca = include_bytes!("misc/mdoc_eku.ca.der");

    let eku_mdoc = KeyUsage::required(&[40, 129, 140, 93, 5, 1, 2]);
    check_cert(ee, ca, eku_mdoc, time, Ok(()));
    check_cert(ee, ca, KeyUsage::server_auth(), time, err);
    check_cert(ee, ca, eku_mdoc, time, Ok(()));
    check_cert(ee, ca, KeyUsage::server_auth(), time, err);
}

#[test]
pub fn verify_custom_eku_client() {
    let time = webpki::Time::from_seconds_since_unix_epoch(0x1fed_f00d);

    let ee = include_bytes!("client_auth/cert_with_no_eku_accepted_for_client_auth.ee.der");
    let ca = include_bytes!("client_auth/cert_with_no_eku_accepted_for_client_auth.ca.der");
    check_cert(ee, ca, KeyUsage::client_auth(), time, Ok(()));

    let ee = include_bytes!("client_auth/cert_with_both_ekus_accepted_for_client_auth.ee.der");
    let ca = include_bytes!("client_auth/cert_with_both_ekus_accepted_for_client_auth.ca.der");
    check_cert(ee, ca, KeyUsage::client_auth(), time, Ok(()));
    check_cert(ee, ca, KeyUsage::server_auth(), time, Ok(()));
}
