use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;
use std::collections::HashMap;
use webpki::{KeyUsage, TrustAnchor};

#[test]
pub fn path_building() {
    let raw_json = include_bytes!("../third-party/bettertls/pathbuilding.tests.json");
    let better_tls: BetterTls = serde_json::from_slice(raw_json).expect("invalid test JSON");
    println!("Testing BetterTLS revision {:?}", better_tls.revision);

    let root_der = &better_tls.root_der();
    let roots = &[TrustAnchor::try_from_cert_der(root_der).expect("invalid trust anchor")];

    let path_building_suite = better_tls
        .suites
        .get("pathbuilding")
        .unwrap_or_else(|| panic!("missing pathbuilding suite"));

    for testcase in &path_building_suite.test_cases {
        println!("Testing path building test case {:?}", testcase.id);

        let certs_der = testcase.certs_der();
        let ee_der = &certs_der[0];
        let intermediates = &certs_der[1..]
            .iter()
            .map(|cert| cert.as_slice())
            .collect::<Vec<_>>();

        let ee_cert =
            webpki::EndEntityCert::try_from(ee_der.as_slice()).expect("invalid end entity cert");

        // Set the time to the time of test case generation. This ensures that the test case
        // certificates won't expire.
        let now = webpki::Time::from_seconds_since_unix_epoch(1_688_651_734);

        let result = ee_cert.verify_for_usage(
            &[&webpki::ECDSA_P256_SHA256], // All of the BetterTLS testcases use P256 keys.
            roots,
            intermediates,
            now,
            KeyUsage::server_auth(),
            &[],
        );

        match testcase.expected {
            ExpectedResult::Accept => assert!(result.is_ok(), "expected success, got {:?}", result),
            ExpectedResult::Reject => {
                assert!(result.is_err(), "expected failure, got {:?}", result)
            }
        }
    }
}

#[derive(Deserialize, Debug)]
struct BetterTls {
    #[serde(rename(deserialize = "betterTlsRevision"))]
    revision: String,
    #[serde(rename(deserialize = "trustRoot"))]
    root: String,
    suites: HashMap<String, BetterTlsSuite>,
}

impl BetterTls {
    fn root_der(&self) -> Vec<u8> {
        general_purpose::STANDARD
            .decode(&self.root)
            .expect("invalid trust anchor base64")
    }
}

#[derive(Deserialize, Debug)]
struct BetterTlsSuite {
    #[serde(rename(deserialize = "testCases"))]
    test_cases: Vec<BetterTlsTest>,
}

#[derive(Deserialize, Debug)]
struct BetterTlsTest {
    id: u32,
    certificates: Vec<String>,
    expected: ExpectedResult,
}

impl BetterTlsTest {
    fn certs_der(&self) -> Vec<Vec<u8>> {
        self.certificates
            .iter()
            .map(|cert| {
                general_purpose::STANDARD
                    .decode(cert)
                    .expect("invalid cert base64")
            })
            .collect()
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
enum ExpectedResult {
    Accept,
    Reject,
}
