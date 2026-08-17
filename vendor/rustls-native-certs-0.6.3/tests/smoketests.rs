use std::convert::TryInto;
use std::sync::Arc;

use std::panic;

use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;

// #[serial] is used on all these tests to run them sequentially. If they're run in parallel,
// the global env var configuration in the env var test interferes with the others.
use serial_test::serial;

fn check_site(domain: &str) {
    let mut roots = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().unwrap() {
        roots
            .add(&rustls::Certificate(cert.0))
            .unwrap();
    }

    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(roots)
        .with_no_client_auth();

    let mut conn =
        rustls::ClientConnection::new(Arc::new(config), domain.try_into().unwrap()).unwrap();
    let mut sock = TcpStream::connect(format!("{}:443", domain)).unwrap();
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);
    tls.write_all(
        format!(
            "GET / HTTP/1.1\r\n\
                       Host: {}\r\n\
                       Connection: close\r\n\
                       Accept-Encoding: identity\r\n\
                       \r\n",
            domain
        )
        .as_bytes(),
    )
    .unwrap();
    let mut plaintext = [0u8; 1024];
    let len = tls.read(&mut plaintext).unwrap();
    assert!(plaintext[..len].starts_with(b"HTTP/1.1 ")); // or whatever
}

#[test]
#[serial]
fn google() {
    check_site("google.com");
}

#[test]
#[serial]
fn amazon() {
    check_site("amazon.com");
}

#[test]
#[serial]
fn facebook() {
    check_site("facebook.com");
}

#[test]
#[serial]
fn netflix() {
    check_site("netflix.com");
}

#[test]
#[serial]
fn ebay() {
    check_site("ebay.com");
}

#[test]
#[serial]
fn apple() {
    check_site("apple.com");
}

#[test]
#[serial]
fn badssl_with_env() {
    let result = panic::catch_unwind(|| check_site("self-signed.badssl.com"));
    // Self-signed certs should never be trusted by default:
    assert!(result.is_err());

    // But they should be trusted if SSL_CERT_FILE is set:
    env::set_var(
        "SSL_CERT_FILE",
        // The CA cert, downloaded directly from the site itself:
        PathBuf::from("./tests/badssl-com-chain.pem"),
    );
    check_site("self-signed.badssl.com");
    env::remove_var("SSL_CERT_FILE");
}
