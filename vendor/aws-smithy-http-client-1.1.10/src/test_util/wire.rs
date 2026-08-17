/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Utilities for mocking at the socket level
//!
//! Other tools in this module actually operate at the `http::Request` / `http::Response` level. This
//! is useful, but it shortcuts the HTTP implementation (e.g. Hyper). [`WireMockServer`] binds
//! to an actual socket on the host.
//!
//! # Examples
//! ```no_run
//! use aws_smithy_runtime_api::client::http::HttpConnectorSettings;
//! use aws_smithy_http_client::test_util::wire::{check_matches, ReplayedEvent, WireMockServer};
//! use aws_smithy_http_client::{match_events, ev};
//! # async fn example() {
//!
//! // This connection binds to a local address
//! let mock = WireMockServer::start(vec![
//!     ReplayedEvent::status(503),
//!     ReplayedEvent::status(200)
//! ]).await;
//!
//! # /*
//! // Create a client using the wire mock
//! let config = my_generated_client::Config::builder()
//!     .http_client(mock.http_client())
//!     .build();
//! let client = Client::from_conf(config);
//!
//! // ... do something with <client>
//! # */
//!
//! // assert that you got the events you expected
//! match_events!(ev!(dns), ev!(connect), ev!(http(200)))(&mock.events());
//! # }
//! ```

#![allow(missing_docs)]

use aws_smithy_async::future::never::Never;
use aws_smithy_async::future::BoxFuture;
use aws_smithy_runtime_api::client::http::SharedHttpClient;
use bytes::Bytes;
use http_body_util::Full;
use hyper::service::service_fn;
use hyper_util::client::legacy::connect::dns::Name;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::graceful::{GracefulConnection, GracefulShutdown};
use std::collections::HashSet;
use std::convert::Infallible;
use std::error::Error;
use std::future::Future;
use std::iter::Once;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

/// An event recorded by [`WireMockServer`].
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum RecordedEvent {
    DnsLookup(String),
    NewConnection,
    Response(ReplayedEvent),
}

type Matcher = (
    Box<dyn Fn(&RecordedEvent) -> Result<(), Box<dyn Error>>>,
    &'static str,
);

/// This method should only be used by the macro
pub fn check_matches(events: &[RecordedEvent], matchers: &[Matcher]) {
    let mut events_iter = events.iter();
    let mut matcher_iter = matchers.iter();
    let mut idx = -1;
    loop {
        idx += 1;
        let bail = |err: Box<dyn Error>| {
            panic!("failed on event {idx}:\n  {err}\n  actual recorded events: {events:?}")
        };
        match (events_iter.next(), matcher_iter.next()) {
            (Some(event), Some((matcher, _msg))) => matcher(event).unwrap_or_else(bail),
            (None, None) => return,
            (Some(event), None) => {
                bail(format!("got {event:?} but no more events were expected").into())
            }
            (None, Some((_expect, msg))) => {
                bail(format!("expected {msg:?} but no more events were expected").into())
            }
        }
    }
}

#[macro_export]
macro_rules! matcher {
    ($expect:tt) => {
        (
            Box::new(|event: &$crate::test_util::wire::RecordedEvent| {
                if !matches!(event, $expect) {
                    return Err(
                        format!("expected `{}` but got {:?}", stringify!($expect), event).into(),
                    );
                }
                Ok(())
            }),
            stringify!($expect),
        )
    };
}

/// Helper macro to generate a series of test expectations
#[macro_export]
macro_rules! match_events {
        ($( $expect:pat),*) => {
            |events| {
                $crate::test_util::wire::check_matches(events, &[$( $crate::matcher!($expect) ),*]);
            }
        };
    }

/// Helper to generate match expressions for events
#[macro_export]
macro_rules! ev {
    (http($status:expr)) => {
        $crate::test_util::wire::RecordedEvent::Response(
            $crate::test_util::wire::ReplayedEvent::HttpResponse {
                status: $status,
                ..
            },
        )
    };
    (dns) => {
        $crate::test_util::wire::RecordedEvent::DnsLookup(_)
    };
    (connect) => {
        $crate::test_util::wire::RecordedEvent::NewConnection
    };
    (timeout) => {
        $crate::test_util::wire::RecordedEvent::Response(
            $crate::test_util::wire::ReplayedEvent::Timeout,
        )
    };
}

pub use {ev, match_events, matcher};

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReplayedEvent {
    Timeout,
    HttpResponse { status: u16, body: Bytes },
}

impl ReplayedEvent {
    pub fn ok() -> Self {
        Self::HttpResponse {
            status: 200,
            body: Bytes::new(),
        }
    }

    pub fn with_body(body: impl AsRef<[u8]>) -> Self {
        Self::HttpResponse {
            status: 200,
            body: Bytes::copy_from_slice(body.as_ref()),
        }
    }

    pub fn status(status: u16) -> Self {
        Self::HttpResponse {
            status,
            body: Bytes::new(),
        }
    }
}

/// Test server that binds to 127.0.0.1:0
///
/// See the [module docs](crate::test_util::wire) for a usage example.
///
/// Usage:
/// - Call [`WireMockServer::start`] to start the server
/// - Use [`WireMockServer::http_client`] or [`dns_resolver`](WireMockServer::dns_resolver) to configure your client.
/// - Make requests to [`endpoint_url`](WireMockServer::endpoint_url).
/// - Once the test is complete, retrieve a list of events from [`WireMockServer::events`]
#[derive(Debug)]
pub struct WireMockServer {
    event_log: Arc<Mutex<Vec<RecordedEvent>>>,
    bind_addr: SocketAddr,
    // when the sender is dropped, that stops the server
    shutdown_hook: oneshot::Sender<()>,
}

#[derive(Debug, Clone)]
struct SharedGraceful {
    graceful: Arc<Mutex<Option<hyper_util::server::graceful::GracefulShutdown>>>,
}

impl SharedGraceful {
    fn new() -> Self {
        Self {
            graceful: Arc::new(Mutex::new(Some(GracefulShutdown::new()))),
        }
    }

    fn watch<C: GracefulConnection>(&self, conn: C) -> impl Future<Output = C::Output> {
        let graceful = self.graceful.lock().unwrap();
        graceful
            .as_ref()
            .expect("graceful not shutdown")
            .watch(conn)
    }

    async fn shutdown(&self) {
        let graceful = { self.graceful.lock().unwrap().take() };

        if let Some(graceful) = graceful {
            graceful.shutdown().await;
        }
    }
}

impl WireMockServer {
    /// Start a wire mock server with the given events to replay.
    pub async fn start(mut response_events: Vec<ReplayedEvent>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let (tx, mut rx) = oneshot::channel();
        let listener_addr = listener.local_addr().unwrap();
        response_events.reverse();
        let response_events = Arc::new(Mutex::new(response_events));
        let handler_events = response_events;
        let wire_events = Arc::new(Mutex::new(vec![]));
        let wire_log_for_service = wire_events.clone();
        let poisoned_conns: Arc<Mutex<HashSet<SocketAddr>>> = Default::default();
        let graceful = SharedGraceful::new();
        let conn_builder = Arc::new(hyper_util::server::conn::auto::Builder::new(
            TokioExecutor::new(),
        ));

        let server = async move {
            let poisoned_conns = poisoned_conns.clone();
            let events = handler_events.clone();
            let wire_log = wire_log_for_service.clone();
            loop {
                tokio::select! {
                    Ok((stream, remote_addr)) = listener.accept() => {
                        tracing::info!("established connection: {:?}", remote_addr);
                        let poisoned_conns = poisoned_conns.clone();
                        let events = events.clone();
                        let wire_log = wire_log.clone();
                        wire_log.lock().unwrap().push(RecordedEvent::NewConnection);
                        let io = TokioIo::new(stream);

                        let svc = service_fn(move |_req| {
                            let poisoned_conns = poisoned_conns.clone();
                            let events = events.clone();
                            let wire_log = wire_log.clone();
                            if poisoned_conns.lock().unwrap().contains(&remote_addr) {
                                tracing::error!("poisoned connection {:?} was reused!", &remote_addr);
                                panic!("poisoned connection was reused!");
                            }
                            let next_event = events.clone().lock().unwrap().pop();
                            async move {
                                let next_event = next_event
                                    .unwrap_or_else(|| panic!("no more events! Log: {wire_log:?}"));

                                wire_log
                                    .lock()
                                    .unwrap()
                                    .push(RecordedEvent::Response(next_event.clone()));

                                if next_event == ReplayedEvent::Timeout {
                                    tracing::info!("{} is poisoned", remote_addr);
                                    poisoned_conns.lock().unwrap().insert(remote_addr);
                                }
                                tracing::debug!("replying with {:?}", next_event);
                                let event = generate_response_event(next_event).await;
                                dbg!(event)
                            }
                        });

                        let conn_builder = conn_builder.clone();
                        let graceful = graceful.clone();
                        tokio::spawn(async move {
                            let conn = conn_builder.serve_connection(io, svc);
                            let fut = graceful.watch(conn);
                            if let Err(e) = fut.await {
                                panic!("Error serving connection: {e:?}");
                            }
                        });
                    },
                    _ = &mut rx => {
                        tracing::info!("wire server: shutdown signalled");
                        graceful.shutdown().await;
                        tracing::info!("wire server: shutdown complete!");
                        break;
                    }
                }
            }
        };

        tokio::spawn(server);
        Self {
            event_log: wire_events,
            bind_addr: listener_addr,
            shutdown_hook: tx,
        }
    }

    /// Retrieve the events recorded by this connection
    pub fn events(&self) -> Vec<RecordedEvent> {
        self.event_log.lock().unwrap().clone()
    }

    fn bind_addr(&self) -> SocketAddr {
        self.bind_addr
    }

    pub fn dns_resolver(&self) -> LoggingDnsResolver {
        let event_log = self.event_log.clone();
        let bind_addr = self.bind_addr;
        LoggingDnsResolver(InnerDnsResolver {
            log: event_log,
            socket_addr: bind_addr,
        })
    }

    /// Prebuilt [`HttpClient`](aws_smithy_runtime_api::client::http::HttpClient) with correctly wired DNS resolver.
    ///
    /// **Note**: This must be used in tandem with [`Self::dns_resolver`]
    pub fn http_client(&self) -> SharedHttpClient {
        let resolver = self.dns_resolver();
        crate::client::build_with_tcp_conn_fn(None, None, move || {
            hyper_util::client::legacy::connect::HttpConnector::new_with_resolver(
                resolver.clone().0,
            )
        })
    }

    /// Endpoint to use when connecting
    ///
    /// This works in tandem with the [`Self::dns_resolver`] to bind to the correct local IP Address
    pub fn endpoint_url(&self) -> String {
        format!(
            "http://this-url-is-converted-to-localhost.com:{}",
            self.bind_addr().port()
        )
    }

    /// Shuts down the mock server.
    pub fn shutdown(self) {
        let _ = self.shutdown_hook.send(());
    }
}

async fn generate_response_event(
    event: ReplayedEvent,
) -> Result<http_1x::Response<Full<Bytes>>, Infallible> {
    let resp = match event {
        ReplayedEvent::HttpResponse { status, body } => http_1x::Response::builder()
            .status(status)
            .body(Full::new(body))
            .unwrap(),
        ReplayedEvent::Timeout => {
            Never::new().await;
            unreachable!()
        }
    };
    Ok::<_, Infallible>(resp)
}

/// DNS resolver that keeps a log of all lookups
///
/// Regardless of what hostname is requested, it will always return the same socket address.
#[derive(Clone, Debug)]
pub struct LoggingDnsResolver(InnerDnsResolver);

// internal implementation so we don't have to expose hyper_util
#[derive(Clone, Debug)]
struct InnerDnsResolver {
    log: Arc<Mutex<Vec<RecordedEvent>>>,
    socket_addr: SocketAddr,
}

impl tower::Service<Name> for InnerDnsResolver {
    type Response = Once<SocketAddr>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Self::Response, Self::Error>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Name) -> Self::Future {
        let socket_addr = self.socket_addr;
        let log = self.log.clone();
        Box::pin(async move {
            println!("looking up {req:?}, replying with {socket_addr:?}");
            log.lock()
                .unwrap()
                .push(RecordedEvent::DnsLookup(req.to_string()));
            Ok(std::iter::once(socket_addr))
        })
    }
}

#[cfg(all(feature = "legacy-test-util", feature = "hyper-014"))]
impl hyper_0_14::service::Service<hyper_0_14::client::connect::dns::Name> for LoggingDnsResolver {
    type Response = Once<SocketAddr>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: hyper_0_14::client::connect::dns::Name) -> Self::Future {
        use std::str::FromStr;
        let adapter = Name::from_str(req.as_str()).expect("valid conversion");
        self.0.call(adapter)
    }
}
