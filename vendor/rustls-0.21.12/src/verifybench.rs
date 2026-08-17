// This program does benchmarking of the functions in verify.rs,
// that do certificate chain validation and signature verification.
//
// Note: we don't use any of the standard 'cargo bench', 'test::Bencher',
// etc. because it's unstable at the time of writing.

use std::time::{Duration, Instant, SystemTime};

use crate::key;
use crate::verify;
use crate::verify::ServerCertVerifier;
use crate::{anchors, OwnedTrustAnchor};

fn duration_nanos(d: Duration) -> u64 {
    ((d.as_secs() as f64) * 1e9 + (d.subsec_nanos() as f64)) as u64
}

#[test]
fn test_reddit_cert() {
    Context::new(
        "reddit",
        "reddit.com",
        &[
            include_bytes!("testdata/cert-reddit.0.der"),
            include_bytes!("testdata/cert-reddit.1.der"),
        ],
    )
    .bench(100)
}

#[test]
fn test_github_cert() {
    Context::new(
        "github",
        "github.com",
        &[
            include_bytes!("testdata/cert-github.0.der"),
            include_bytes!("testdata/cert-github.1.der"),
        ],
    )
    .bench(100)
}

#[test]
fn test_arstechnica_cert() {
    Context::new(
        "arstechnica",
        "arstechnica.com",
        &[
            include_bytes!("testdata/cert-arstechnica.0.der"),
            include_bytes!("testdata/cert-arstechnica.1.der"),
            include_bytes!("testdata/cert-arstechnica.2.der"),
            include_bytes!("testdata/cert-arstechnica.3.der"),
        ],
    )
    .bench(100)
}

#[test]
fn test_servo_cert() {
    Context::new(
        "servo",
        "servo.org",
        &[
            include_bytes!("testdata/cert-servo.0.der"),
            include_bytes!("testdata/cert-servo.1.der"),
        ],
    )
    .bench(100)
}

#[test]
fn test_twitter_cert() {
    Context::new(
        "twitter",
        "twitter.com",
        &[
            include_bytes!("testdata/cert-twitter.0.der"),
            include_bytes!("testdata/cert-twitter.1.der"),
        ],
    )
    .bench(100)
}

#[test]
fn test_wikipedia_cert() {
    Context::new(
        "wikipedia",
        "wikipedia.org",
        &[
            include_bytes!("testdata/cert-wikipedia.0.der"),
            include_bytes!("testdata/cert-wikipedia.1.der"),
        ],
    )
    .bench(100)
}

#[test]
fn test_google_cert() {
    Context::new(
        "google",
        "www.google.com",
        &[
            include_bytes!("testdata/cert-google.0.der"),
            include_bytes!("testdata/cert-google.1.der"),
        ],
    )
    .bench(100)
}

#[test]
fn test_hn_cert() {
    Context::new(
        "hn",
        "news.ycombinator.com",
        &[
            include_bytes!("testdata/cert-hn.0.der"),
            include_bytes!("testdata/cert-hn.1.der"),
        ],
    )
    .bench(100)
}

#[test]
fn test_stackoverflow_cert() {
    Context::new(
        "stackoverflow",
        "stackoverflow.com",
        &[
            include_bytes!("testdata/cert-stackoverflow.0.der"),
            include_bytes!("testdata/cert-stackoverflow.1.der"),
        ],
    )
    .bench(100)
}

#[test]
fn test_duckduckgo_cert() {
    Context::new(
        "duckduckgo",
        "duckduckgo.com",
        &[
            include_bytes!("testdata/cert-duckduckgo.0.der"),
            include_bytes!("testdata/cert-duckduckgo.1.der"),
        ],
    )
    .bench(100)
}

#[test]
fn test_rustlang_cert() {
    Context::new(
        "rustlang",
        "www.rust-lang.org",
        &[
            include_bytes!("testdata/cert-rustlang.0.der"),
            include_bytes!("testdata/cert-rustlang.1.der"),
            include_bytes!("testdata/cert-rustlang.2.der"),
        ],
    )
    .bench(100)
}

#[test]
fn test_wapo_cert() {
    Context::new(
        "wapo",
        "www.washingtonpost.com",
        &[
            include_bytes!("testdata/cert-wapo.0.der"),
            include_bytes!("testdata/cert-wapo.1.der"),
        ],
    )
    .bench(100)
}

struct Context {
    name: &'static str,
    domain: &'static str,
    roots: anchors::RootCertStore,
    chain: Vec<key::Certificate>,
    now: SystemTime,
}

impl Context {
    fn new(name: &'static str, domain: &'static str, certs: &[&'static [u8]]) -> Self {
        let mut roots = anchors::RootCertStore::empty();
        roots.add_trust_anchors(
            webpki_roots::TLS_SERVER_ROOTS
                .iter()
                .map(|ta| {
                    OwnedTrustAnchor::from_subject_spki_name_constraints(
                        ta.subject,
                        ta.spki,
                        ta.name_constraints,
                    )
                }),
        );
        Self {
            name,
            domain,
            roots,
            chain: certs
                .iter()
                .copied()
                .map(|bytes| key::Certificate(bytes.to_vec()))
                .collect(),
            now: SystemTime::UNIX_EPOCH + Duration::from_secs(1640870720),
        }
    }

    fn bench(&self, count: usize) {
        let verifier = verify::WebPkiVerifier::new(self.roots.clone(), None);
        const SCTS: &[&[u8]] = &[];
        const OCSP_RESPONSE: &[u8] = &[];
        let mut times = Vec::new();

        let (end_entity, intermediates) = self.chain.split_first().unwrap();
        for _ in 0..count {
            let start = Instant::now();
            let server_name = self.domain.try_into().unwrap();
            verifier
                .verify_server_cert(
                    end_entity,
                    intermediates,
                    &server_name,
                    &mut SCTS.iter().copied(),
                    OCSP_RESPONSE,
                    self.now,
                )
                .unwrap();
            times.push(duration_nanos(Instant::now().duration_since(start)));
        }

        println!(
            "verify_server_cert({}): min {:?}us",
            self.name,
            times.iter().min().unwrap() / 1000
        );
    }
}
