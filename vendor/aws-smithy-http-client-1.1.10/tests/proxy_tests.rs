/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Integration tests for proxy functionality
//!
//! These tests verify that proxy configuration works end-to-end with real HTTP requests
//! using mock proxy servers.
#![cfg(feature = "default-client")]

use aws_smithy_async::time::SystemTimeSource;
use aws_smithy_http_client::{proxy::ProxyConfig, tls, Connector};
use aws_smithy_runtime_api::client::http::{
    http_client_fn, HttpClient, HttpConnector, HttpConnectorSettings, SharedHttpConnector,
};
use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
use base64::Engine;
use http_1x::{Request, Response, StatusCode};
use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

// ================================================================================================
// Test Utilities (Mock Proxy Server)
// ================================================================================================

/// Mock HTTP server that acts as a proxy endpoint for testing
#[derive(Debug)]
struct MockProxyServer {
    conn_count: Arc<()>,
    addr: SocketAddr,
    shutdown_tx: Option<oneshot::Sender<()>>,
    request_log: Arc<Mutex<Vec<RecordedRequest>>>,
}

/// A recorded request received by the mock proxy server
#[derive(Debug, Clone)]
struct RecordedRequest {
    method: String,
    uri: String,
    headers: HashMap<String, String>,
}

impl MockProxyServer {
    /// Create a new mock proxy server with a custom request handler
    async fn new<F>(handler: F) -> Self
    where
        F: Fn(RecordedRequest) -> Response<String> + Send + Sync + 'static,
    {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let request_log = Arc::new(Mutex::new(Vec::new()));
        let request_log_clone = request_log.clone();
        let conn_count = Arc::new(());
        let server_conn_count = conn_count.clone();

        let handler = Arc::new(handler);

        tokio::spawn(async move {
            let mut shutdown_rx = shutdown_rx;

            loop {
                tokio::select! {
                    result = listener.accept() => {
                        match result {
                            Ok((stream, _)) => {
                                let io = TokioIo::new(stream);
                                let handler = handler.clone();
                                let request_log = request_log_clone.clone();

                                let stream_conn_count = server_conn_count.clone();
                                tokio::spawn(async move {
                                    let _stream_conn_count = stream_conn_count;
                                    let service = service_fn(move |req: Request<Incoming>| {
                                        let handler = handler.clone();
                                        let request_log = request_log.clone();

                                        async move {
                                            // Record the request
                                            let recorded = RecordedRequest {
                                                method: req.method().to_string(),
                                                uri: req.uri().to_string(),
                                                headers: req.headers().iter()
                                                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                                                    .collect(),
                                            };

                                            request_log.lock().unwrap().push(recorded.clone());

                                            // Call the handler
                                            let response = handler(recorded);

                                            // Convert to hyper response
                                            let (parts, body) = response.into_parts();
                                            let hyper_response = Response::from_parts(parts, body);

                                            Ok::<_, Infallible>(hyper_response)
                                        }
                                    });

                                    if let Err(err) = hyper::server::conn::http1::Builder::new()
                                        .serve_connection(io, service)
                                        .await
                                    {
                                        eprintln!("Mock proxy server connection error: {}", err);
                                    }
                                });
                            }
                            Err(_) => break,
                        }
                    }
                    _ = &mut shutdown_rx => {
                        break;
                    }
                }
            }
        });

        Self {
            addr,
            shutdown_tx: Some(shutdown_tx),
            request_log,
            conn_count,
        }
    }

    /// Return the number of active connections to this server
    fn conn_count(&self) -> usize {
        // 1 reference for the struct MockProxyServer, 1 reference for the
        // socket task.
        Arc::strong_count(&self.conn_count)
            .checked_sub(2)
            .expect("de-count 2 refs")
    }

    /// Create a simple mock proxy that returns a fixed response
    async fn with_response(status: StatusCode, body: &str) -> Self {
        let body = body.to_string();
        Self::new(move |_req| {
            Response::builder()
                .status(status)
                .body(body.clone())
                .unwrap()
        })
        .await
    }

    /// Create a mock proxy that validates basic authentication
    async fn with_auth_validation(expected_user: &str, expected_pass: &str) -> Self {
        let expected_auth = format!(
            "Basic {}",
            base64::prelude::BASE64_STANDARD.encode(format!("{}:{}", expected_user, expected_pass))
        );

        Self::new(move |req| {
            if let Some(auth_header) = req.headers.get("proxy-authorization") {
                if auth_header == &expected_auth {
                    Response::builder()
                        .status(StatusCode::OK)
                        .body("authenticated".to_string())
                        .unwrap()
                } else {
                    Response::builder()
                        .status(StatusCode::PROXY_AUTHENTICATION_REQUIRED)
                        .body("invalid credentials".to_string())
                        .unwrap()
                }
            } else {
                Response::builder()
                    .status(StatusCode::PROXY_AUTHENTICATION_REQUIRED)
                    .header("proxy-authenticate", "Basic realm=\"proxy\"")
                    .body("authentication required".to_string())
                    .unwrap()
            }
        })
        .await
    }

    /// Get the address this server is listening on
    fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Get all requests received by this server
    fn requests(&self) -> Vec<RecordedRequest> {
        self.request_log.lock().unwrap().clone()
    }
}

impl Drop for MockProxyServer {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

/// Utility for running tests with specific environment variables
#[allow(clippy::await_holding_lock)]
async fn with_env_vars<F, Fut, R>(vars: &[(&str, &str)], test: F) -> R
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = R>,
{
    // Use a static mutex to serialize environment variable tests
    static ENV_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_MUTEX.lock().unwrap();

    // Save original environment
    let original_vars: Vec<_> = vars
        .iter()
        .map(|(key, _)| (*key, std::env::var(key)))
        .collect();

    // Set test environment variables
    for (key, value) in vars {
        std::env::set_var(key, value);
    }

    // Run the test
    let result = test().await;

    // Restore original environment
    for (key, original_value) in original_vars {
        match original_value {
            Ok(val) => std::env::set_var(key, val),
            Err(_) => std::env::remove_var(key),
        }
    }

    result
}

/// Helper function to make HTTP requests through a proxy-configured connector
async fn make_http_request_through_proxy(
    proxy_config: ProxyConfig,
    target_url: &str,
) -> Result<(StatusCode, String), Box<dyn std::error::Error + Send + Sync>> {
    make_http_request_through_proxy_with_pool_timeout(
        proxy_config,
        Some(Duration::from_secs(90)),
        target_url,
    )
    .await
    .map(|(status, res, _client)| (status, res))
}

/// Helper function to make HTTP requests through a proxy-configured connector
async fn make_http_request_through_proxy_with_pool_timeout(
    proxy_config: ProxyConfig,
    pool_idle_timeout: Option<Duration>,
    target_url: &str,
) -> Result<(StatusCode, String, SharedHttpConnector), Box<dyn std::error::Error + Send + Sync>> {
    // Create an HttpClient using http_client_fn with proxy-configured connector
    let http_client = http_client_fn(move |settings, _components| {
        let connector = Connector::builder()
            .proxy_config(proxy_config.clone())
            .pool_idle_timeout(pool_idle_timeout)
            .connector_settings(settings.clone())
            .build_http();

        aws_smithy_runtime_api::client::http::SharedHttpConnector::new(connector)
    });

    // Set up runtime components (following smoke_test_client pattern)
    let connector_settings = HttpConnectorSettings::builder().build();
    let runtime_components = RuntimeComponentsBuilder::for_tests()
        .with_time_source(Some(SystemTimeSource::new()))
        .build()
        .unwrap();

    // Get the HTTP connector from the client
    let http_connector = http_client.http_connector(&connector_settings, &runtime_components);

    // Create and make the HTTP request
    let request = HttpRequest::get(target_url)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    let response = http_connector.call(request).await?;

    // Extract status and body
    let status = response.status();
    let body_bytes = response.into_body().collect().await?.to_bytes();
    let body_string = String::from_utf8(body_bytes.to_vec())?;

    Ok((status.into(), body_string, http_connector))
}

// test the pool idle timeout. The test is in this file because it has a convenient
// infrastructure for making a server that answers smithy requests.
#[tokio::test(start_paused = false)]
// can't set start_paused due to <https://github.com/hyperium/hyper/issues/3950>
async fn test_http_proxy_connection_pool_timeout() {
    const TIMEOUT: Duration = Duration::from_secs(10);

    // Create a mock proxy server that validates the request was routed through it
    let mock_proxy = MockProxyServer::new(|req| {
        // Validate that this looks like a proxy request
        assert_eq!(req.method, "GET");
        // For HTTP proxy, the URI should be the full target URL
        assert_eq!(req.uri, "http://aws.amazon.com/api/data");

        // Return a successful response that we can identify
        Response::builder()
            .status(StatusCode::OK)
            .body("proxied response from mock server".to_string())
            .unwrap()
    })
    .await;
    // make sure that conn_count for an empty proxy is 0
    assert_eq!(mock_proxy.conn_count(), 0);
    tracing::info!("Start!");

    // Configure connector with HTTP proxy
    let proxy_config = ProxyConfig::http(format!("http://{}", mock_proxy.addr())).unwrap();

    // Make an HTTP request through the proxy - use safe domain
    let target_url = "http://aws.amazon.com/api/data";
    let start = tokio::time::Instant::now();
    let result =
        make_http_request_through_proxy_with_pool_timeout(proxy_config, Some(TIMEOUT), target_url)
            .await;
    // hold _connector to avoid the timer being dropped
    let (status, body, _connector) = result.expect("HTTP request through proxy should succeed");
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "proxied response from mock server");

    // Verify the mock proxy received the expected request
    let requests = mock_proxy.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, "GET");
    assert_eq!(requests[0].uri, target_url);

    // after making a request, conn count is 1
    assert_eq!(mock_proxy.conn_count(), 1);

    // sleep 1 second below the idle timeout, conn count is still 1
    tokio::time::sleep_until(start + TIMEOUT - Duration::from_secs(1)).await;
    assert_eq!(mock_proxy.conn_count(), 1);

    // sleep 2 more seconds, the connection should be disconnected, conn count should be 0
    tokio::time::sleep(Duration::from_secs(3)).await;
    assert_eq!(mock_proxy.conn_count(), 0);
}

#[tokio::test]
async fn test_http_proxy_basic_request() {
    // Create a mock proxy server that validates the request was routed through it
    let mock_proxy = MockProxyServer::new(|req| {
        // Validate that this looks like a proxy request
        assert_eq!(req.method, "GET");
        // For HTTP proxy, the URI should be the full target URL
        assert_eq!(req.uri, "http://aws.amazon.com/api/data");

        // Return a successful response that we can identify
        Response::builder()
            .status(StatusCode::OK)
            .body("proxied response from mock server".to_string())
            .unwrap()
    })
    .await;

    // Configure connector with HTTP proxy
    let proxy_config = ProxyConfig::http(format!("http://{}", mock_proxy.addr())).unwrap();

    // Make an HTTP request through the proxy - use safe domain
    let target_url = "http://aws.amazon.com/api/data";
    let result = make_http_request_through_proxy(proxy_config, target_url).await;

    let (status, body) = result.expect("HTTP request through proxy should succeed");

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "proxied response from mock server");

    // Verify the mock proxy received the expected request
    let requests = mock_proxy.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, "GET");
    assert_eq!(requests[0].uri, target_url);
}

#[tokio::test]
async fn test_proxy_authentication() {
    // Create a mock proxy that requires authentication
    let mock_proxy = MockProxyServer::with_auth_validation("testuser", "testpass").await;

    // Configure connector with authenticated proxy
    let proxy_config = ProxyConfig::http(format!("http://{}", mock_proxy.addr()))
        .unwrap()
        .with_basic_auth("testuser", "testpass");

    // Make request through authenticated proxy - use safe domain
    let target_url = "http://aws.amazon.com/protected/resource";
    let result = make_http_request_through_proxy(proxy_config, target_url).await;

    let (status, body) = result.expect("Authenticated proxy request should succeed");

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "authenticated");

    // Verify the proxy received the request with correct auth
    let requests = mock_proxy.requests();
    assert_eq!(requests.len(), 1);

    let expected_auth = format!(
        "Basic {}",
        base64::prelude::BASE64_STANDARD.encode("testuser:testpass")
    );
    assert_eq!(
        requests[0].headers.get("proxy-authorization"),
        Some(&expected_auth)
    );
}

/// Tests URL-embedded proxy authentication (http://user:pass@proxy.com format)
/// Verifies that credentials in the proxy URL are properly extracted and used
#[tokio::test]
async fn test_proxy_url_embedded_auth() {
    let mock_proxy = MockProxyServer::with_auth_validation("urluser", "urlpass").await;

    // Configure proxy with credentials embedded in URL
    let proxy_url = format!("http://urluser:urlpass@{}", mock_proxy.addr());
    let proxy_config = ProxyConfig::http(proxy_url).unwrap();

    // Make request through proxy with URL-embedded auth
    let target_url = "http://aws.amazon.com/api/test";
    let result = make_http_request_through_proxy(proxy_config, target_url).await;

    let (status, body) = result.expect("URL-embedded auth proxy request should succeed");
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "authenticated");

    // Verify the proxy received the request with correct auth
    let requests = mock_proxy.requests();
    assert_eq!(requests.len(), 1);

    let expected_auth = format!(
        "Basic {}",
        base64::prelude::BASE64_STANDARD.encode("urluser:urlpass")
    );
    assert_eq!(
        requests[0].headers.get("proxy-authorization"),
        Some(&expected_auth)
    );
}

/// Tests authentication precedence: URL-embedded credentials should take precedence over programmatic auth
/// Verifies that when both URL auth and with_basic_auth() are provided, URL auth wins
#[tokio::test]
async fn test_proxy_auth_precedence() {
    let mock_proxy = MockProxyServer::with_auth_validation("urluser", "urlpass").await;

    // Configure proxy with URL-embedded auth AND programmatic auth
    // URL auth should take precedence
    let proxy_url = format!("http://urluser:urlpass@{}", mock_proxy.addr());
    let proxy_config = ProxyConfig::http(proxy_url)
        .unwrap()
        .with_basic_auth("programmatic", "auth"); // This should be ignored

    // Make request - should use URL-embedded auth, not programmatic auth
    let target_url = "http://aws.amazon.com/precedence/test";
    let result = make_http_request_through_proxy(proxy_config, target_url).await;

    let (status, body) = result.expect("Auth precedence test should succeed");
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "authenticated");

    // Verify the proxy received the request with URL-embedded auth (not programmatic)
    let requests = mock_proxy.requests();
    assert_eq!(requests.len(), 1);

    let expected_auth = format!(
        "Basic {}",
        base64::prelude::BASE64_STANDARD.encode("urluser:urlpass")
    );
    assert_eq!(
        requests[0].headers.get("proxy-authorization"),
        Some(&expected_auth)
    );
}

#[tokio::test]
async fn test_proxy_from_environment_variables() {
    let mock_proxy = MockProxyServer::with_response(StatusCode::OK, "env proxy response").await;

    with_env_vars(
        &[
            ("HTTP_PROXY", &format!("http://{}", mock_proxy.addr())),
            ("NO_PROXY", "localhost,127.0.0.1"),
        ],
        || async {
            // Create connector with environment-based proxy config
            let proxy_config = ProxyConfig::from_env();

            // Make request through environment-configured proxy
            let target_url = "http://aws.amazon.com/v1/data";
            let result = make_http_request_through_proxy(proxy_config, target_url).await;

            let (status, body) = result.expect("Environment proxy request should succeed");

            assert_eq!(status, StatusCode::OK);
            assert_eq!(body, "env proxy response");

            // Verify the proxy received the request
            let requests = mock_proxy.requests();
            assert_eq!(requests.len(), 1);
            assert_eq!(requests[0].uri, target_url);
        },
    )
    .await;
}

/// Tests that NO_PROXY bypass rules work correctly
/// Verifies that requests to bypassed hosts do not go through the proxy
#[tokio::test]
async fn test_no_proxy_bypass_rules() {
    let mock_proxy = MockProxyServer::with_response(StatusCode::OK, "should not reach here").await;

    // Create a second mock server that will act as the "direct" target
    let direct_server = MockProxyServer::with_response(StatusCode::OK, "direct connection").await;

    // Configure proxy with NO_PROXY rules that include the direct server's address
    // Use just the IP address for the NO_PROXY rule
    let direct_ip = "127.0.0.1";
    let proxy_config = ProxyConfig::http(format!("http://{}", mock_proxy.addr()))
        .unwrap()
        .no_proxy(direct_ip);

    // Make request to the direct server (should bypass proxy due to NO_PROXY rule)
    let result = make_http_request_through_proxy(
        proxy_config,
        &format!("http://{}/test", direct_server.addr()),
    )
    .await;

    let (status, body) = result.expect("Direct connection should succeed");
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "direct connection");

    // Verify the mock proxy received no requests (bypassed)
    let proxy_requests = mock_proxy.requests();
    assert_eq!(
        proxy_requests.len(),
        0,
        "Proxy should not have received any requests due to NO_PROXY bypass"
    );

    // Verify the direct server received the request
    let direct_requests = direct_server.requests();
    assert_eq!(
        direct_requests.len(),
        1,
        "Direct server should have received the request"
    );
}

/// Tests that disabled proxy configuration results in direct connections
/// Verifies that ProxyConfig::disabled() bypasses all proxy logic
#[tokio::test]
async fn test_proxy_disabled() {
    // Create a direct target server
    let direct_server = MockProxyServer::with_response(StatusCode::OK, "direct connection").await;

    // Create a disabled proxy configuration
    let proxy_config = ProxyConfig::disabled();

    // Make request with disabled proxy (should go direct to our mock server)
    let result = make_http_request_through_proxy(
        proxy_config,
        &format!("http://{}/get", direct_server.addr()),
    )
    .await;

    let (status, body) = result.expect("Direct connection should succeed");
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "direct connection");

    // Verify the direct server received the request
    let requests = direct_server.requests();
    assert_eq!(
        requests.len(),
        1,
        "Direct server should have received the request"
    );
    assert_eq!(requests[0].method, "GET");
    // For direct connections, the URI might be just the path part
    assert!(
        requests[0].uri == format!("http://{}/get", direct_server.addr())
            || requests[0].uri == "/get",
        "URI should be either full URL or path, got: {}",
        requests[0].uri
    );
}

/// Tests HTTPS-only proxy configuration
/// Verifies that HTTP requests bypass HTTPS-only proxies
#[tokio::test]
async fn test_https_proxy_configuration() {
    let mock_proxy = MockProxyServer::with_response(StatusCode::OK, "https proxy response").await;

    // Create a direct target server for HTTP requests
    let direct_server =
        MockProxyServer::with_response(StatusCode::OK, "direct http connection").await;

    // Configure HTTPS-only proxy
    let proxy_config = ProxyConfig::https(format!("http://{}", mock_proxy.addr())).unwrap();

    // Test: HTTP request should NOT go through HTTPS-only proxy, should go direct
    let target_url = format!("http://{}/api", direct_server.addr());
    let result = make_http_request_through_proxy(proxy_config.clone(), &target_url).await;

    // The HTTP request should succeed by going directly to our mock server
    let (status, body) = result.expect("HTTP request should succeed via direct connection");
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "direct http connection");

    // Verify the HTTPS-only proxy received no requests
    let proxy_requests = mock_proxy.requests();
    assert_eq!(
        proxy_requests.len(),
        0,
        "HTTP request should not go through HTTPS-only proxy"
    );

    // Verify the direct server received the request
    let direct_requests = direct_server.requests();
    assert_eq!(
        direct_requests.len(),
        1,
        "Direct server should have received the HTTP request"
    );
}

/// Tests all-traffic proxy configuration
/// Verifies that both HTTP and HTTPS requests go through all-traffic proxies
#[tokio::test]
async fn test_all_traffic_proxy() {
    let mock_proxy = MockProxyServer::with_response(StatusCode::OK, "all traffic proxy").await;

    // Configure proxy for all traffic
    let proxy_config = ProxyConfig::all(format!("http://{}", mock_proxy.addr())).unwrap();

    // HTTP request should go through the proxy
    let target_url = "http://aws.amazon.com/api/endpoint";
    let result = make_http_request_through_proxy(proxy_config.clone(), target_url).await;

    let (status, body) = result.expect("HTTP request through all-traffic proxy should succeed");
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "all traffic proxy");

    // Verify the proxy received the HTTP request
    let requests = mock_proxy.requests();
    assert_eq!(
        requests.len(),
        1,
        "Proxy should have received exactly one request"
    );
    assert_eq!(requests[0].method, "GET");
    assert_eq!(requests[0].uri, target_url);
}

/// Tests proxy connection failure handling
/// Verifies that unreachable proxy servers result in appropriate connection errors
#[tokio::test]
async fn test_proxy_connection_failure() {
    // Configure proxy pointing to non-existent server
    let proxy_config = ProxyConfig::http("http://127.0.0.1:1").unwrap(); // Port 1 should be unavailable

    // Make request through non-existent proxy - use a safe domain that won't cause issues
    let target_url = "http://aws.amazon.com/api/test";
    let result = make_http_request_through_proxy(proxy_config, target_url).await;

    // The request should fail with a connection error
    assert!(
        result.is_err(),
        "Request should fail when proxy is unreachable"
    );

    let error = result.unwrap_err();
    let error_msg = error.to_string().to_lowercase();

    // Verify it's a connection-related error (not a different kind of error)
    assert!(
        error_msg.contains("connection")
            || error_msg.contains("refused")
            || error_msg.contains("unreachable")
            || error_msg.contains("timeout")
            || error_msg.contains("connect")
            || error_msg.contains("io error"), // Include generic IO errors
        "Error should be connection-related, got: {}",
        error
    );
}

/// Tests proxy authentication failure handling
/// Verifies that incorrect proxy credentials result in 407 Proxy Authentication Required
#[tokio::test]
async fn test_proxy_authentication_failure() {
    let mock_proxy = MockProxyServer::with_auth_validation("correct", "password").await;

    // Configure proxy with wrong credentials
    let proxy_config = ProxyConfig::http(format!("http://{}", mock_proxy.addr()))
        .unwrap()
        .with_basic_auth("wrong", "credentials");

    // Make request with wrong credentials - use safe domain
    let target_url = "http://aws.amazon.com/secure/api";
    let result = make_http_request_through_proxy(proxy_config, target_url).await;

    // The request should return 407 Proxy Authentication Required
    let (status, _body) = result.expect("Request should complete (even with auth failure)");
    assert_eq!(status, StatusCode::PROXY_AUTHENTICATION_REQUIRED);

    // Verify the proxy received the request (even though auth failed)
    let requests = mock_proxy.requests();
    assert_eq!(requests.len(), 1, "Proxy should have received the request");

    // Verify the wrong credentials were sent
    let expected_wrong_auth = format!(
        "Basic {}",
        base64::prelude::BASE64_STANDARD.encode("wrong:credentials")
    );
    assert_eq!(
        requests[0].headers.get("proxy-authorization"),
        Some(&expected_wrong_auth)
    );
}

/// Tests that ProxyConfig::disabled() overrides environment proxy settings
/// Verifies that explicit proxy disabling takes precedence over environment variables
#[tokio::test]
async fn test_explicit_proxy_disable_overrides_environment() {
    let mock_proxy = MockProxyServer::new(|_req| {
        panic!("Request should not reach proxy when explicitly disabled");
    })
    .await;

    // Create a direct target server
    let direct_server = MockProxyServer::with_response(StatusCode::OK, "direct connection").await;

    with_env_vars(
        &[("HTTP_PROXY", &format!("http://{}", mock_proxy.addr()))],
        || async {
            // Create connector with explicitly disabled proxy (should override environment)
            let proxy_config = ProxyConfig::disabled();

            // Make request - should go direct despite HTTP_PROXY environment variable
            let target_url = format!("http://{}/test", direct_server.addr());
            let result = make_http_request_through_proxy(proxy_config, &target_url).await;

            let (status, body) = result.expect("Direct connection should succeed");
            assert_eq!(status, StatusCode::OK);
            assert_eq!(body, "direct connection");

            // Verify the proxy received no requests (disabled)
            let proxy_requests = mock_proxy.requests();
            assert_eq!(
                proxy_requests.len(),
                0,
                "Proxy should not receive requests when explicitly disabled"
            );

            // Verify the direct server received the request
            let direct_requests = direct_server.requests();
            assert_eq!(
                direct_requests.len(),
                1,
                "Direct server should have received the request"
            );
        },
    )
    .await;
}

// ================================================================================================
// HTTPS/CONNECT Tunneling Tests
// ================================================================================================
//
// These tests are for HTTPS tunneling through HTTP proxies using the CONNECT method.

/// Helper function to make HTTPS requests through proxy using TLS providers
/// This is similar to make_http_request_through_proxy but uses TLS-enabled connectors
async fn make_https_request_through_proxy(
    proxy_config: ProxyConfig,
    target_url: &str,
    tls_provider: tls::Provider,
) -> Result<(StatusCode, String), Box<dyn std::error::Error + Send + Sync>> {
    let http_client = http_client_fn(move |settings, _components| {
        let connector = Connector::builder()
            .proxy_config(proxy_config.clone())
            .connector_settings(settings.clone())
            .tls_provider(tls_provider.clone())
            .build();

        aws_smithy_runtime_api::client::http::SharedHttpConnector::new(connector)
    });

    let connector_settings = HttpConnectorSettings::builder().build();
    let runtime_components = RuntimeComponentsBuilder::for_tests()
        .with_time_source(Some(SystemTimeSource::new()))
        .build()
        .unwrap();

    let http_connector = http_client.http_connector(&connector_settings, &runtime_components);

    let request = HttpRequest::get(target_url)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    let response = http_connector.call(request).await?;

    let status = response.status();
    let body_bytes = response.into_body().collect().await?.to_bytes();
    let body_string = String::from_utf8(body_bytes.to_vec())?;

    Ok((status.into(), body_string))
}

/// Generic test function for HTTPS CONNECT with authentication
/// Tests that HTTPS requests through HTTP proxy use CONNECT method with proper auth headers
async fn run_https_connect_with_auth_test(tls_provider: tls::Provider, provider_name: &str) {
    let mock_proxy = MockProxyServer::new(|req| {
        // For HTTPS through HTTP proxy, we should see a CONNECT request
        assert_eq!(req.method, "CONNECT");
        assert_eq!(req.uri, "secure.aws.amazon.com:443");

        // Verify authentication header is present
        let expected_auth = format!(
            "Basic {}",
            base64::prelude::BASE64_STANDARD.encode("connectuser:connectpass")
        );
        assert_eq!(req.headers.get("proxy-authorization"), Some(&expected_auth));

        // Return 400 to avoid dealing with actual TLS tunneling
        // The important part is that we got the CONNECT request with correct auth
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("CONNECT tunnel setup failed".to_string())
            .unwrap()
    })
    .await;

    // Configure proxy with authentication
    let proxy_config = ProxyConfig::all(format!("http://{}", mock_proxy.addr()))
        .unwrap()
        .with_basic_auth("connectuser", "connectpass");

    // Make HTTPS request - should trigger CONNECT method
    let target_url = "https://secure.aws.amazon.com/api/secure";
    let result = make_https_request_through_proxy(proxy_config, target_url, tls_provider).await;

    // We expect this to fail with a connection error since we returned 400
    // The important thing is that the CONNECT request was made correctly
    assert!(
        result.is_err(),
        "CONNECT tunnel should fail with 400 response for {}",
        provider_name
    );

    // Verify the proxy received the CONNECT request
    let requests = mock_proxy.requests();
    assert_eq!(
        requests.len(),
        1,
        "Proxy should have received exactly one CONNECT request for {}",
        provider_name
    );
}

/// Generic test function for CONNECT without authentication (should get 407)
/// Tests that HTTPS requests without auth get proper 407 response
async fn run_https_connect_auth_required_test(tls_provider: tls::Provider, provider_name: &str) {
    let mock_proxy = MockProxyServer::new(|req| {
        // For HTTPS through HTTP proxy, we should see a CONNECT request
        assert_eq!(req.method, "CONNECT");
        assert_eq!(req.uri, "secure.aws.amazon.com:443");

        // No auth header should be present
        assert!(!req.headers.contains_key("proxy-authorization"));

        // Return 407 Proxy Authentication Required
        Response::builder()
            .status(StatusCode::PROXY_AUTHENTICATION_REQUIRED)
            .body("Proxy authentication required for CONNECT".to_string())
            .unwrap()
    })
    .await;

    // Configure proxy without authentication
    let proxy_config = ProxyConfig::all(format!("http://{}", mock_proxy.addr())).unwrap();

    // Make HTTPS request - should trigger CONNECT method and get 407
    let target_url = "https://secure.aws.amazon.com/api/secure";
    let result = make_https_request_through_proxy(proxy_config, target_url, tls_provider).await;

    // We expect this to fail with a connection error since we returned 407
    assert!(
        result.is_err(),
        "CONNECT tunnel should fail with 407 response for {}",
        provider_name
    );

    let error_msg = result.unwrap_err().to_string();
    let error_msg_lower = error_msg.to_lowercase();

    // The important thing is that the request failed (which means CONNECT was attempted)
    // The specific error message format is less critical for this test
    // We accept either specific proxy auth errors OR generic connection errors
    // since both indicate the CONNECT tunnel attempt was made
    assert!(
        error_msg_lower.contains("407")
            || error_msg_lower.contains("proxy")
            || error_msg_lower.contains("auth")
            || error_msg_lower.contains("io error")
            || error_msg_lower.contains("connection"),
        "Error should be connection-related (indicating CONNECT was attempted) for {}, got: {}",
        provider_name,
        error_msg
    );

    // Verify the proxy received the CONNECT request
    let requests = mock_proxy.requests();
    assert_eq!(
        requests.len(),
        1,
        "Proxy should have received exactly one CONNECT request for {}",
        provider_name
    );
}

/// Tests HTTPS tunneling through HTTP proxy with CONNECT method (rustls provider)
/// Verifies that HTTPS requests through HTTP proxy use CONNECT method with authentication
#[cfg(feature = "rustls-ring")]
#[tokio::test]
async fn test_https_connect_with_auth_rustls() {
    run_https_connect_with_auth_test(
        tls::Provider::rustls(tls::rustls_provider::CryptoMode::Ring),
        "rustls",
    )
    .await;
}

/// Tests CONNECT method without authentication (should get 407) - rustls provider
/// Verifies that HTTPS requests without auth get proper 407 response
#[cfg(feature = "rustls-ring")]
#[tokio::test]
async fn test_https_connect_auth_required_rustls() {
    run_https_connect_auth_required_test(
        tls::Provider::rustls(tls::rustls_provider::CryptoMode::Ring),
        "rustls",
    )
    .await;
}

/// Tests HTTPS tunneling through HTTP proxy with CONNECT method (s2n-tls provider)
/// Verifies that HTTPS requests through HTTP proxy use CONNECT method with authentication
#[cfg(feature = "s2n-tls")]
#[tokio::test]
async fn test_https_connect_with_auth_s2n_tls() {
    run_https_connect_with_auth_test(tls::Provider::S2nTls, "s2n-tls").await;
}

/// Tests CONNECT method without authentication (should get 407) - s2n-tls provider
/// Verifies that HTTPS requests without auth get proper 407 response
#[cfg(feature = "s2n-tls")]
#[tokio::test]
async fn test_https_connect_auth_required_s2n_tls() {
    run_https_connect_auth_required_test(tls::Provider::S2nTls, "s2n-tls").await;
}

/// Tests that HTTP requests through proxy use absolute URI form
/// Verifies that the full URL (including hostname) is sent to the proxy
#[tokio::test]
async fn test_http_proxy_absolute_uri_form() {
    let target_host = "api.example.com";
    let target_path = "/v1/data";
    let expected_absolute_uri = format!("http://{}{}", target_host, target_path);

    // Clone for use in closure
    let expected_uri_clone = expected_absolute_uri.clone();
    let target_host_clone = target_host.to_string();

    let mock_proxy = MockProxyServer::new(move |req| {
        // For HTTP through proxy, we should see the full absolute URI
        assert_eq!(req.method, "GET");
        assert_eq!(req.uri, expected_uri_clone);

        // Host header should still be present
        assert_eq!(req.headers.get("host"), Some(&target_host_clone));

        Response::builder()
            .status(StatusCode::OK)
            .body("proxied response".to_string())
            .unwrap()
    })
    .await;

    let proxy_config = ProxyConfig::http(format!("http://{}", mock_proxy.addr())).unwrap();

    let result = make_http_request_through_proxy(proxy_config, &expected_absolute_uri).await;

    let (status, body) = result.expect("HTTP request through proxy should succeed");
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "proxied response");

    // Verify the proxy received the request with absolute URI
    let requests = mock_proxy.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].uri, expected_absolute_uri);
}

/// Tests that direct HTTP requests (no proxy) use origin form URI
/// Verifies that only the path is sent when connecting directly
#[tokio::test]
async fn test_direct_http_origin_uri_form() {
    let target_path = "/v1/data";

    // Create a direct target server (no proxy)
    let direct_server = MockProxyServer::new(move |req| {
        // For direct connections, we should see only the path (origin form)
        assert_eq!(req.method, "GET");
        // The URI should be just the path part, not the full URL
        assert!(
            req.uri == target_path || req.uri.ends_with(target_path),
            "Expected origin form URI ending with '{}', got '{}'",
            target_path,
            req.uri
        );

        Response::builder()
            .status(StatusCode::OK)
            .body("direct response".to_string())
            .unwrap()
    })
    .await;

    // Use disabled proxy to ensure direct connection
    let proxy_config = ProxyConfig::disabled();

    let target_url = format!("http://{}{}", direct_server.addr(), target_path);
    let result = make_http_request_through_proxy(proxy_config, &target_url).await;

    let (status, body) = result.expect("Direct HTTP request should succeed");
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "direct response");

    // Verify the server received the request
    let requests = direct_server.requests();
    assert_eq!(requests.len(), 1);
}

/// Tests URI form handling with different proxy configurations
/// Verifies that URI form changes based on proxy vs direct connection
#[tokio::test]
async fn test_uri_form_proxy_vs_direct() {
    let target_host = "test.example.com";
    let target_path = "/api/test";
    let full_url = format!("http://{}{}", target_host, target_path);

    // Test 1: Through proxy - should use absolute form
    {
        // Clone for use in closure
        let target_host_clone = target_host.to_string();
        let target_path_clone = target_path.to_string();

        let mock_proxy = MockProxyServer::new(move |req| {
            // Should receive absolute URI
            assert!(req.uri.starts_with("http://"));
            assert!(req.uri.contains(&target_host_clone));
            assert!(req.uri.contains(&target_path_clone));

            Response::builder()
                .status(StatusCode::OK)
                .body("proxy response".to_string())
                .unwrap()
        })
        .await;

        let proxy_config = ProxyConfig::http(format!("http://{}", mock_proxy.addr())).unwrap();
        let result = make_http_request_through_proxy(proxy_config, &full_url).await;

        assert!(result.is_ok(), "Proxy request should succeed");
        let requests = mock_proxy.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].uri, full_url);
    }

    // Test 2: Direct connection - should use origin form
    {
        let target_path_clone = target_path.to_string();

        let direct_server = MockProxyServer::new(move |req| {
            // Should receive only the path part
            assert!(!req.uri.starts_with("http://"));
            assert!(req.uri == target_path_clone || req.uri.ends_with(&target_path_clone));

            Response::builder()
                .status(StatusCode::OK)
                .body("direct response".to_string())
                .unwrap()
        })
        .await;

        let proxy_config = ProxyConfig::disabled();
        let direct_url = format!("http://{}{}", direct_server.addr(), target_path);
        let result = make_http_request_through_proxy(proxy_config, &direct_url).await;

        assert!(result.is_ok(), "Direct request should succeed");
        let requests = direct_server.requests();
        assert_eq!(requests.len(), 1);
    }
}

/// Generic test function for CONNECT URI form validation
/// Tests that CONNECT requests use the correct host:port format
async fn run_connect_uri_form_test(tls_provider: tls::Provider, provider_name: &str) {
    let target_host = "secure.example.com";
    let target_port = 443;
    let expected_connect_uri = format!("{}:{}", target_host, target_port);

    // Clone for use in closure
    let expected_uri_clone = expected_connect_uri.clone();

    let mock_proxy = MockProxyServer::new(move |req| {
        if req.method == "CONNECT" {
            // CONNECT should use host:port format
            assert_eq!(req.uri, expected_uri_clone);

            // CONNECT requests should not have a Host header in the CONNECT line
            // (the Host header is for the tunneled HTTP request, not the CONNECT)

            Response::builder()
                .status(StatusCode::OK)
                .body("Connection established".to_string())
                .unwrap()
        } else {
            // This shouldn't happen in our test, but handle it gracefully
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Unexpected non-CONNECT request".to_string())
                .unwrap()
        }
    })
    .await;

    let proxy_config = ProxyConfig::all(format!("http://{}", mock_proxy.addr())).unwrap();

    // Try to make an HTTPS request - this should trigger CONNECT
    let target_url = format!("https://{}/api/secure", target_host);

    let _result = make_https_request_through_proxy(proxy_config, &target_url, tls_provider).await;

    // The request will likely fail due to our mock setup, but that's OK
    // The important thing is that the CONNECT request was made with correct URI
    let requests = mock_proxy.requests();
    assert_eq!(
        requests.len(),
        1,
        "Should have received exactly one CONNECT request for {}",
        provider_name
    );
    assert_eq!(requests[0].method, "CONNECT");
    assert_eq!(requests[0].uri, expected_connect_uri);
}

/// Tests CONNECT method URI form for HTTPS tunneling - rustls provider
/// Verifies that CONNECT requests use the correct host:port format
#[cfg(feature = "rustls-ring")]
#[tokio::test]
async fn test_connect_uri_form_rustls() {
    run_connect_uri_form_test(
        tls::Provider::rustls(tls::rustls_provider::CryptoMode::Ring),
        "rustls",
    )
    .await;
}

/// Tests CONNECT method URI form for HTTPS tunneling - s2n-tls provider
/// Verifies that CONNECT requests use the correct host:port format
#[cfg(feature = "s2n-tls")]
#[tokio::test]
async fn test_connect_uri_form_s2n_tls() {
    run_connect_uri_form_test(tls::Provider::S2nTls, "s2n-tls").await;
}
