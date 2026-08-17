/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![cfg(any(
    feature = "rustls-ring",
    feature = "rustls-aws-lc",
    feature = "rustls-aws-lc-fips",
    feature = "s2n-tls",
))]

use aws_smithy_async::time::SystemTimeSource;
use aws_smithy_http_client::tls;
use aws_smithy_http_client::tls::{TlsContext, TrustStore};
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::http::{HttpClient, HttpConnector, HttpConnectorSettings};
use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
use aws_smithy_types::byte_stream::ByteStream;
use http_1x::{Method, Request, Response, StatusCode};
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::service::service_fn;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use rustls::ServerConfig;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use std::{fs, io};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_rustls::TlsAcceptor;
use tracing::{debug, error};

struct TestServer {
    _handle: JoinHandle<()>,
    listen_addr: SocketAddr,
    conn_count: Arc<()>,
}

impl TestServer {
    /// Return the number of active connections to this server
    fn conn_count(&self) -> usize {
        // 1 reference for the struct MockProxyServer, 1 reference for the
        // socket task.
        Arc::strong_count(&self.conn_count)
            .checked_sub(2)
            .expect("de-count 2 refs")
    }
}

async fn server() -> Result<TestServer, BoxError> {
    // Set process wide crypto provider
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // load public certificate.
    let certs = load_certs("tests/server.pem")?;

    // load private key.
    let key = load_private_key("tests/server.rsa")?;

    debug!("Starting to serve on https://{}", addr);

    // TLS config
    let mut server_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| error(e.to_string()))?;

    server_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec(), b"http/1.0".to_vec()];
    let tls_acceptor = TlsAcceptor::from(Arc::new(server_config));
    let service = service_fn(echo);

    let conn_count = Arc::new(());
    let server_conn_count = conn_count.clone();

    let server = async move {
        loop {
            let (tcp_stream, remote_addr) = listener.accept().await.unwrap();
            debug!("accepted connection from: {}", remote_addr);

            let tls_acceptor = tls_acceptor.clone();
            let connection_conn_count = server_conn_count.clone();
            tokio::spawn(async move {
                let _connection_conn_count = connection_conn_count;
                let tls_stream = match tls_acceptor.accept(tcp_stream).await {
                    Ok(tls_stream) => tls_stream,
                    Err(err) => {
                        error!("failed to perform tls handshake: {err:#}");
                        return;
                    }
                };
                if let Err(err) = Builder::new(TokioExecutor::new())
                    .serve_connection(TokioIo::new(tls_stream), service)
                    .await
                {
                    error!("failed to serve connection: {err:#}");
                }
            });
        }
    };

    let server_task = tokio::spawn(server);

    Ok(TestServer {
        _handle: server_task,
        listen_addr: addr,
        conn_count,
    })
}

// Custom echo service, handling two different routes and a
// catch-all 404 responder.
async fn echo(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let mut response = Response::new(Full::default());
    match (req.method(), req.uri().path()) {
        // default route.
        (&Method::GET, "/") => {
            *response.body_mut() = Full::from("Hello TLS!");
        }
        // echo service route.
        (&Method::POST, "/echo") => {
            *response.body_mut() = Full::from(req.into_body().collect().await?.to_bytes());
        }
        // Catch-all 404.
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
    Ok(response)
}

fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

// Load public certificate from file.
fn load_certs(filename: &str) -> io::Result<Vec<CertificateDer<'static>>> {
    let certfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader).collect()
}

// Load private key from file.
fn load_private_key(filename: &str) -> io::Result<PrivateKeyDer<'static>> {
    // Open keyfile.
    let keyfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(keyfile);

    // Load and return a single private key.
    rustls_pemfile::private_key(&mut reader).map(|key| key.unwrap())
}

fn tls_context_from_pem(filename: &str) -> TlsContext {
    let pem_contents = fs::read(filename).unwrap();
    let trust_store = TrustStore::empty().with_pem_certificate(pem_contents);
    TlsContext::builder()
        .with_trust_store(trust_store)
        .build()
        .unwrap()
}

#[cfg(feature = "rustls-aws-lc")]
#[should_panic(expected = "InvalidCertificate(UnknownIssuer)")]
#[tokio::test]
async fn test_rustls_aws_lc_native_ca() {
    let client = aws_smithy_http_client::Builder::new()
        .tls_provider(tls::Provider::Rustls(
            tls::rustls_provider::CryptoMode::AwsLc,
        ))
        .build_https();

    run_tls_test(&client).await.unwrap()
}

#[cfg(feature = "rustls-aws-lc")]
#[tokio::test]
async fn test_rustls_aws_lc_custom_ca() {
    let client = aws_smithy_http_client::Builder::new()
        .tls_provider(tls::Provider::Rustls(
            tls::rustls_provider::CryptoMode::AwsLc,
        ))
        .tls_context(tls_context_from_pem("tests/server.pem"))
        .build_https();

    run_tls_test(&client).await.unwrap()
}

#[cfg(feature = "rustls-aws-lc")]
#[tokio::test(start_paused = false)]
// can't have paused clock due to <https://github.com/hyperium/hyper/issues/3950>
async fn test_rustls_aws_lc_custom_ca_with_timeout() {
    const TIMEOUT: Duration = Duration::from_secs(10);
    let client = aws_smithy_http_client::Builder::new()
        .pool_idle_timeout(TIMEOUT)
        .tls_provider(tls::Provider::Rustls(
            tls::rustls_provider::CryptoMode::AwsLc,
        ))
        .tls_context(tls_context_from_pem("tests/server.pem"))
        .build_https();

    run_tls_test_with_idle_timeout(&client, Some(TIMEOUT))
        .await
        .unwrap()
}

#[cfg(feature = "rustls-aws-lc-fips")]
#[should_panic(expected = "InvalidCertificate(UnknownIssuer)")]
#[tokio::test]
async fn test_rustls_aws_lc_fips_native_ca() {
    let client = aws_smithy_http_client::Builder::new()
        .tls_provider(tls::Provider::Rustls(
            tls::rustls_provider::CryptoMode::AwsLcFips,
        ))
        .build_https();

    run_tls_test(&client).await.unwrap()
}

#[cfg(feature = "rustls-aws-lc-fips")]
#[tokio::test]
async fn test_rustls_aws_lc_fips_custom_ca() {
    let client = aws_smithy_http_client::Builder::new()
        .tls_provider(tls::Provider::Rustls(
            tls::rustls_provider::CryptoMode::AwsLcFips,
        ))
        .tls_context(tls_context_from_pem("tests/server.pem"))
        .build_https();

    run_tls_test(&client).await.unwrap()
}

#[cfg(feature = "rustls-ring")]
#[should_panic(expected = "InvalidCertificate(UnknownIssuer)")]
#[tokio::test]
async fn test_rustls_ring_native_ca() {
    let client = aws_smithy_http_client::Builder::new()
        .tls_provider(tls::Provider::Rustls(
            tls::rustls_provider::CryptoMode::Ring,
        ))
        .build_https();

    run_tls_test(&client).await.unwrap()
}

#[cfg(feature = "rustls-ring")]
#[tokio::test]
async fn test_rustls_ring_custom_ca() {
    let client = aws_smithy_http_client::Builder::new()
        .tls_provider(tls::Provider::Rustls(
            tls::rustls_provider::CryptoMode::Ring,
        ))
        .tls_context(tls_context_from_pem("tests/server.pem"))
        .build_https();

    run_tls_test(&client).await.unwrap()
}

#[cfg(feature = "s2n-tls")]
#[should_panic(expected = "Certificate is untrusted")]
#[tokio::test]
async fn test_s2n_native_ca() {
    let client = aws_smithy_http_client::Builder::new()
        .tls_provider(tls::Provider::S2nTls)
        .build_https();

    run_tls_test(&client).await.unwrap()
}

#[cfg(feature = "s2n-tls")]
#[tokio::test]
async fn test_s2n_tls_custom_ca() {
    let client = aws_smithy_http_client::Builder::new()
        .tls_provider(tls::Provider::S2nTls)
        .tls_context(tls_context_from_pem("tests/server.pem"))
        .build_https();
    run_tls_test(&client).await.unwrap()
}

async fn run_tls_test(client: &dyn HttpClient) -> Result<(), BoxError> {
    run_tls_test_with_idle_timeout(client, None).await
}

async fn run_tls_test_with_idle_timeout(
    client: &dyn HttpClient,
    pool_timeout: Option<Duration>,
) -> Result<(), BoxError> {
    let server = server().await?;
    let start = tokio::time::Instant::now();
    assert_eq!(server.conn_count(), 0); // calibrate conn_count
    let endpoint = format!("https://localhost:{}/", server.listen_addr.port());

    let connector_settings = HttpConnectorSettings::builder().build();
    let runtime_components = RuntimeComponentsBuilder::for_tests()
        .with_time_source(Some(SystemTimeSource::new()))
        .build()
        .unwrap();
    let connector = client.http_connector(&connector_settings, &runtime_components);
    let mut response = connector.call(HttpRequest::get(endpoint).unwrap()).await?;

    let sdk_body = response.take_body();
    let body_stream = ByteStream::new(sdk_body);
    let resp_bytes = body_stream.collect().await?.into_bytes();
    assert_eq!(b"Hello TLS!", &resp_bytes[..]);

    if let Some(pool_timeout) = pool_timeout {
        assert_eq!(server.conn_count(), 1);
        tokio::time::sleep_until(start + pool_timeout - Duration::from_secs(1)).await;
        assert_eq!(server.conn_count(), 1);
        tokio::time::sleep(Duration::from_secs(2)).await;
        assert_eq!(server.conn_count(), 0);
    }
    Ok(())
}
