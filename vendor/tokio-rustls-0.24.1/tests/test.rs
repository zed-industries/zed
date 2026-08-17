use futures_util::future::TryFutureExt;
use lazy_static::lazy_static;
use rustls::{ClientConfig, OwnedTrustAnchor};
use rustls_pemfile::{certs, rsa_private_keys};
use std::convert::TryFrom;
use std::io::{BufReader, Cursor, ErrorKind};
use std::net::SocketAddr;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::Duration;
use std::{io, thread};
use tokio::io::{copy, split, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;
use tokio::{runtime, time};
use tokio_rustls::{LazyConfigAcceptor, TlsAcceptor, TlsConnector};

const CERT: &str = include_str!("end.cert");
const CHAIN: &[u8] = include_bytes!("end.chain");
const RSA: &str = include_str!("end.rsa");

lazy_static! {
    static ref TEST_SERVER: (SocketAddr, &'static str, &'static [u8]) = {
        let cert = certs(&mut BufReader::new(Cursor::new(CERT)))
            .unwrap()
            .drain(..)
            .map(rustls::Certificate)
            .collect();
        let mut keys = rsa_private_keys(&mut BufReader::new(Cursor::new(RSA))).unwrap();
        let mut keys = keys.drain(..).map(rustls::PrivateKey);

        let config = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert, keys.next().unwrap())
            .unwrap();
        let acceptor = TlsAcceptor::from(Arc::new(config));

        let (send, recv) = channel();

        thread::spawn(move || {
            let runtime = runtime::Builder::new_current_thread()
                .enable_io()
                .build()
                .unwrap();
            let runtime = Arc::new(runtime);
            let runtime2 = runtime.clone();

            let done = async move {
                let addr = SocketAddr::from(([127, 0, 0, 1], 0));
                let listener = TcpListener::bind(&addr).await?;

                send.send(listener.local_addr()?).unwrap();

                loop {
                    let (stream, _) = listener.accept().await?;

                    let acceptor = acceptor.clone();
                    let fut = async move {
                        let stream = acceptor.accept(stream).await?;

                        let (mut reader, mut writer) = split(stream);
                        copy(&mut reader, &mut writer).await?;

                        Ok(()) as io::Result<()>
                    }
                    .unwrap_or_else(|err| eprintln!("server: {:?}", err));

                    runtime2.spawn(fut);
                }
            }
            .unwrap_or_else(|err: io::Error| eprintln!("server: {:?}", err));

            runtime.block_on(done);
        });

        let addr = recv.recv().unwrap();
        (addr, "foobar.com", CHAIN)
    };
}

fn start_server() -> &'static (SocketAddr, &'static str, &'static [u8]) {
    &TEST_SERVER
}

async fn start_client(addr: SocketAddr, domain: &str, config: Arc<ClientConfig>) -> io::Result<()> {
    const FILE: &[u8] = include_bytes!("../README.md");

    let domain = rustls::ServerName::try_from(domain).unwrap();
    let config = TlsConnector::from(config);
    let mut buf = vec![0; FILE.len()];

    let stream = TcpStream::connect(&addr).await?;
    let mut stream = config.connect(domain, stream).await?;
    stream.write_all(FILE).await?;
    stream.flush().await?;
    stream.read_exact(&mut buf).await?;

    assert_eq!(buf, FILE);

    Ok(())
}

#[tokio::test]
async fn pass() -> io::Result<()> {
    let (addr, domain, chain) = start_server();

    // TODO: not sure how to resolve this right now but since
    // TcpStream::bind now returns a future it creates a race
    // condition until its ready sometimes.
    use std::time::*;
    tokio::time::sleep(Duration::from_secs(1)).await;

    let chain = certs(&mut std::io::Cursor::new(*chain)).unwrap();
    let mut root_store = rustls::RootCertStore::empty();
    root_store.add_server_trust_anchors(chain.iter().map(|cert| {
        let ta = webpki::TrustAnchor::try_from_cert_der(&cert[..]).unwrap();
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let config = Arc::new(config);

    start_client(*addr, domain, config).await?;

    Ok(())
}

#[tokio::test]
async fn fail() -> io::Result<()> {
    let (addr, domain, chain) = start_server();

    let chain = certs(&mut std::io::Cursor::new(*chain)).unwrap();
    let mut root_store = rustls::RootCertStore::empty();
    root_store.add_server_trust_anchors(chain.iter().map(|cert| {
        let ta = webpki::TrustAnchor::try_from_cert_der(&cert[..]).unwrap();
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let config = Arc::new(config);

    assert_ne!(domain, &"google.com");
    let ret = start_client(*addr, "google.com", config).await;
    assert!(ret.is_err());

    Ok(())
}

#[tokio::test]
async fn test_lazy_config_acceptor() -> io::Result<()> {
    let (sconfig, cconfig) = utils::make_configs();
    use std::convert::TryFrom;

    let (cstream, sstream) = tokio::io::duplex(1200);
    let domain = rustls::ServerName::try_from("foobar.com").unwrap();
    tokio::spawn(async move {
        let connector = crate::TlsConnector::from(cconfig);
        let mut client = connector.connect(domain, cstream).await.unwrap();
        client.write_all(b"hello, world!").await.unwrap();

        let mut buf = Vec::new();
        client.read_to_end(&mut buf).await.unwrap();
    });

    let acceptor = LazyConfigAcceptor::new(rustls::server::Acceptor::default(), sstream);
    let start = acceptor.await.unwrap();
    let ch = start.client_hello();

    assert_eq!(ch.server_name(), Some("foobar.com"));
    assert_eq!(
        ch.alpn()
            .map(|protos| protos.collect::<Vec<_>>())
            .unwrap_or_default(),
        Vec::<&[u8]>::new()
    );

    let mut stream = start.into_stream(sconfig).await.unwrap();
    let mut buf = [0; 13];
    stream.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf[..], b"hello, world!");

    stream.write_all(b"bye").await.unwrap();
    Ok(())
}

// This test is a follow-up from https://github.com/tokio-rs/tls/issues/85
#[tokio::test]
async fn lazy_config_acceptor_eof() {
    let buf = Cursor::new(Vec::new());
    let acceptor = LazyConfigAcceptor::new(rustls::server::Acceptor::default(), buf);

    let accept_result = match time::timeout(Duration::from_secs(3), acceptor).await {
        Ok(res) => res,
        Err(_elapsed) => panic!("timeout"),
    };

    match accept_result {
        Ok(_) => panic!("accepted a connection from zero bytes of data"),
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => {}
        Err(e) => panic!("unexpected error: {:?}", e),
    }
}

#[tokio::test]
async fn lazy_config_acceptor_take_io() -> Result<(), rustls::Error> {
    let (mut cstream, sstream) = tokio::io::duplex(1200);

    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        cstream.write_all(b"hello, world!").await.unwrap();

        let mut buf = Vec::new();
        cstream.read_to_end(&mut buf).await.unwrap();
        tx.send(buf).unwrap();
    });

    let acceptor = LazyConfigAcceptor::new(rustls::server::Acceptor::default(), sstream);
    futures_util::pin_mut!(acceptor);
    if (acceptor.as_mut().await).is_ok() {
        panic!("Expected Err(err)");
    }

    let server_msg = b"message from server";

    let some_io = acceptor.take_io();
    assert!(some_io.is_some(), "Expected Some(io)");
    some_io.unwrap().write_all(server_msg).await.unwrap();

    assert_eq!(rx.await.unwrap(), server_msg);

    assert!(
        acceptor.take_io().is_none(),
        "Should not be able to take twice"
    );
    Ok(())
}

// Include `utils` module
include!("utils.rs");
