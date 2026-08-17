use std::fmt::Write;
use std::fs;

use ring::digest;

use webpki_ccadb::fetch_ccadb_roots;

#[tokio::test]
async fn new_generated_code_is_fresh() {
    let tls_roots_map = fetch_ccadb_roots().await;
    let mut code = String::with_capacity(256 * 1_024);
    code.push_str(HEADER);
    code.push_str("pub const TLS_SERVER_ROOT_CERTS: &[CertificateDer<'static>] = &[\n");

    let mut encoded_cert_der = String::new();
    for root in tls_roots_map.values() {
        // Verify the DER FP matches the metadata FP.
        let der = root.der();
        let calculated_fp = digest::digest(&digest::SHA256, &der);
        let metadata_fp = hex::decode(&root.sha256_fingerprint).expect("malformed fingerprint");
        assert_eq!(calculated_fp.as_ref(), metadata_fp.as_slice());

        write!(&mut encoded_cert_der, "b\"").unwrap();
        for &b in root.der().as_ref() {
            encoded_cert_der.push_str(&format!("\\x{:02X}", b));
        }
        encoded_cert_der.push('"');

        code.push_str(&format!(
            "   // {:?}\n",
            root.common_name_or_certificate_name
        ));
        code.push_str(&format!(
            "   CertificateDer::from_slice({encoded_cert_der}),\n"
        ));
        encoded_cert_der.clear();
    }
    code.push_str("];\n");

    // Check that the generated code matches the checked-in code
    let old = fs::read_to_string("src/lib.rs").unwrap();
    if old != code {
        fs::write("src/lib.rs", code).unwrap();
        panic!("generated code changed");
    }
}

const HEADER: &str = r#"//! A compiled-in copy of the full X.509 root certificates trusted by Mozilla.
//!
//! You should generally prefer to use [`webpki-roots`] when using [`rustls`] or [`webpki`] as it is
//! more space efficient and convenient for that use.
//!
//! This library is suitable for use in applications that can always be recompiled and instantly deployed.
//! For applications that are deployed to end-users and cannot be recompiled, or which need certification
//! before deployment, consider a library that uses the platform native certificate verifier such as
//! [`rustls-platform-verifier`]. This has the additional benefit of supporting OS provided CA constraints
//! and revocation data.
//!
//! [`webpki-roots`]: https://docs.rs/webpki-roots
//! [`webpki`]: https://docs.rs/rustls-webpki
//! [`rustls`]: https://docs.rs/rustls
//! [`rustls-platform-verifier`]: https://docs.rs/rustls-platform-verifier
//
// This library is automatically generated from the Mozilla
// IncludedCACertificateReportPEMCSV report via ccadb.org. Don't edit it.
//
// The generation is done deterministically so you can verify it
// yourself by inspecting and re-running the generation process.

#![no_std]
#![forbid(unsafe_code, unstable_features)]
#![deny(
    elided_lifetimes_in_paths,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_extern_crates,
    unused_qualifications
)]

use pki_types::CertificateDer;

"#;
