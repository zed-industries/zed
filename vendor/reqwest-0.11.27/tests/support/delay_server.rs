#![cfg(not(target_arch = "wasm32"))]
use std::convert::Infallible;
use std::future::Future;
use std::net;
use std::sync::Arc;
use std::time::Duration;

use futures_util::FutureExt;
use http::{Request, Response};
use hyper::service::service_fn;
use hyper::Body;
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::oneshot;

/// This server, unlike [`super::server::Server`], allows for delaying the
/// specified amount of time after each TCP connection is established. This is
/// useful for testing the behavior of the client when the server is slow.
///
/// For example, in case of HTTP/2, once the TCP/TLS connection is established,
/// both endpoints are supposed to send a preface and an initial `SETTINGS`
/// frame (See [RFC9113 3.4] for details). What if these frames are delayed for
/// whatever reason? This server allows for testing such scenarios.
///
/// [RFC9113 3.4]: https://www.rfc-editor.org/rfc/rfc9113.html#name-http-2-connection-preface
pub struct Server {
    addr: net::SocketAddr,
    shutdown_tx: Option<oneshot::Sender<()>>,
    server_terminated_rx: oneshot::Receiver<()>,
}

impl Server {
    pub async fn new<F1, Fut, F2>(func: F1, apply_config: F2, delay: Duration) -> Self
    where
        F1: Fn(Request<Body>) -> Fut + Clone + Send + 'static,
        Fut: Future<Output = Response<Body>> + Send + 'static,
        F2: FnOnce(hyper::server::conn::Http) -> hyper::server::conn::Http + Send + 'static,
    {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (server_terminated_tx, server_terminated_rx) = oneshot::channel();

        let tcp_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = tcp_listener.local_addr().unwrap();

        tokio::spawn(async move {
            let http = Arc::new(apply_config(hyper::server::conn::Http::new()));

            tokio::spawn(async move {
                let (connection_shutdown_tx, connection_shutdown_rx) = oneshot::channel();
                let connection_shutdown_rx = connection_shutdown_rx.shared();
                let mut shutdown_rx = std::pin::pin!(shutdown_rx);

                let mut handles = Vec::new();
                loop {
                    select! {
                        _ = shutdown_rx.as_mut() => {
                            connection_shutdown_tx.send(()).unwrap();
                            break;
                        }
                        res = tcp_listener.accept() => {
                            let (stream, _) = res.unwrap();


                            let handle = tokio::spawn({
                                let connection_shutdown_rx = connection_shutdown_rx.clone();
                                let http = http.clone();
                                let func = func.clone();

                                async move {
                                    tokio::time::sleep(delay).await;

                                    let mut conn = std::pin::pin!(http.serve_connection(
                                        stream,
                                        service_fn(move |req| {
                                            let fut = func(req);
                                            async move {
                                            Ok::<_, Infallible>(fut.await)
                                        }})
                                    ));

                                    select! {
                                        _ = conn.as_mut() => {}
                                        _ = connection_shutdown_rx => {
                                            conn.as_mut().graceful_shutdown();
                                            conn.await.unwrap();
                                        }
                                    }
                                }
                            });

                            handles.push(handle);
                        }
                    }
                }

                futures_util::future::join_all(handles).await;
                server_terminated_tx.send(()).unwrap();
            });
        });

        Self {
            addr,
            shutdown_tx: Some(shutdown_tx),
            server_terminated_rx,
        }
    }

    pub async fn shutdown(mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }

        self.server_terminated_rx.await.unwrap();
    }

    pub fn addr(&self) -> net::SocketAddr {
        self.addr
    }
}
