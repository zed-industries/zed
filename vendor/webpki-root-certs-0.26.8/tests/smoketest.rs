#[test]
fn test_trust_anchor_der() {
    // Simple smoke-test that:
    //  a) parses each TLS server root DER w/ x509-parser.
    //  b) verifies the parsed cert is a CA certificate.
    //  c) verifies the DER can be converted to a webpki trust anchor.
    for root in webpki_root_certs::TLS_SERVER_ROOT_CERTS {
        let (rest, cert) = x509_parser::parse_x509_certificate(root.as_ref()).unwrap();
        assert!(rest.is_empty());
        assert!(cert.is_ca());
        webpki::anchor_from_trusted_cert(root).unwrap();
    }
}
