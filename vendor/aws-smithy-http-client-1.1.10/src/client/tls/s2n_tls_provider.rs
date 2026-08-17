/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

pub(crate) mod build_connector {
    use crate::tls::TlsContext;
    use client::connect::HttpConnector;
    use hyper_util::client::legacy as client;
    use s2n_tls::security::Policy;
    use std::sync::LazyLock;

    // Default S2N security policy which sets protocol versions and cipher suites
    //  See https://aws.github.io/s2n-tls/usage-guide/ch06-security-policies.html
    const S2N_POLICY_VERSION: &str = "20230317";

    fn base_config() -> s2n_tls::config::Builder {
        let mut builder = s2n_tls::config::Config::builder();
        let policy = Policy::from_version(S2N_POLICY_VERSION).unwrap();
        builder
            .set_security_policy(&policy)
            .expect("valid s2n security policy");
        // default is true
        builder.with_system_certs(false).unwrap();
        builder
    }

    static CACHED_CONFIG: LazyLock<s2n_tls::config::Config> = LazyLock::new(|| {
        let mut config = base_config();
        config.with_system_certs(true).unwrap();
        // actually loads the system certs
        config.build().expect("valid s2n config")
    });

    impl TlsContext {
        fn s2n_config(&self) -> s2n_tls::config::Config {
            // TODO(s2n-tls): s2n does not support turning a config back into a builder or a way to load a trust store and re-use it
            // instead if we are only using the defaults then use a cached config, otherwise pay the cost to build a new one
            if self.trust_store.enable_native_roots && self.trust_store.custom_certs.is_empty() {
                CACHED_CONFIG.clone()
            } else {
                let mut config = base_config();
                config
                    .with_system_certs(self.trust_store.enable_native_roots)
                    .unwrap();
                for pem_cert in &self.trust_store.custom_certs {
                    config
                        .trust_pem(pem_cert.0.as_slice())
                        .expect("valid certificate");
                }
                config.build().expect("valid s2n config")
            }
        }
    }

    pub(crate) fn wrap_connector<R>(
        mut http_connector: HttpConnector<R>,
        tls_context: &TlsContext,
        proxy_config: crate::client::proxy::ProxyConfig,
    ) -> super::connect::S2nTlsConnector<R> {
        let config = tls_context.s2n_config();
        http_connector.enforce_http(false);
        let mut builder = s2n_tls_hyper::connector::HttpsConnector::builder_with_http(
            http_connector,
            config.clone(),
        );
        builder.with_plaintext_http(true);
        let https_connector = builder.build();

        super::connect::S2nTlsConnector::new(https_connector, config, proxy_config)
    }
}

pub(crate) mod connect {
    use crate::client::connect::{Conn, Connecting};
    use crate::client::proxy::ProxyConfig;
    use aws_smithy_runtime_api::box_error::BoxError;
    use http_1x::uri::Scheme;
    use http_1x::Uri;
    use hyper_util::client::legacy::connect::{Connected, Connection, HttpConnector};
    use hyper_util::client::proxy::matcher::Matcher;
    use hyper_util::rt::TokioIo;
    use std::error::Error;
    use std::sync::Arc;
    use std::{
        io::IoSlice,
        pin::Pin,
        task::{Context, Poll},
    };
    use tower::Service;

    #[derive(Clone)]
    pub(crate) struct S2nTlsConnector<R> {
        https: s2n_tls_hyper::connector::HttpsConnector<HttpConnector<R>>,
        tls_config: s2n_tls::config::Config,
        proxy_matcher: Option<Arc<Matcher>>, // Pre-computed for performance
    }

    impl<R> S2nTlsConnector<R> {
        pub(super) fn new(
            https: s2n_tls_hyper::connector::HttpsConnector<HttpConnector<R>>,
            tls_config: s2n_tls::config::Config,
            proxy_config: ProxyConfig,
        ) -> Self {
            // Pre-compute the proxy matcher once during construction
            let proxy_matcher = if proxy_config.is_disabled() {
                None
            } else {
                Some(Arc::new(proxy_config.into_hyper_util_matcher()))
            };

            Self {
                https,
                tls_config,
                proxy_matcher,
            }
        }
    }

    impl<R> Service<Uri> for S2nTlsConnector<R>
    where
        R: Clone + Send + Sync + 'static,
        R: Service<hyper_util::client::legacy::connect::dns::Name>,
        R::Response: Iterator<Item = std::net::SocketAddr>,
        R::Future: Send,
        R::Error: Into<Box<dyn Error + Send + Sync>>,
    {
        type Response = Conn;
        type Error = BoxError;
        type Future = Connecting;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.https.poll_ready(cx).map_err(Into::into)
        }

        fn call(&mut self, dst: Uri) -> Self::Future {
            // Check if this request should be proxied using pre-computed matcher
            let proxy_intercept = if let Some(ref matcher) = self.proxy_matcher {
                matcher.intercept(&dst)
            } else {
                None
            };

            if let Some(intercept) = proxy_intercept {
                if dst.scheme() == Some(&Scheme::HTTPS) {
                    // HTTPS through HTTP proxy: Use CONNECT tunneling + manual TLS
                    self.handle_https_through_proxy(dst, intercept)
                } else {
                    // HTTP through proxy: Direct connection to proxy
                    self.handle_http_through_proxy(dst, intercept)
                }
            } else {
                // Direct connection: Use the existing HTTPS connector
                self.handle_direct_connection(dst)
            }
        }
    }

    impl<R> S2nTlsConnector<R>
    where
        R: Clone + Send + Sync + 'static,
        R: Service<hyper_util::client::legacy::connect::dns::Name>,
        R::Response: Iterator<Item = std::net::SocketAddr>,
        R::Future: Send,
        R::Error: Into<Box<dyn Error + Send + Sync>>,
    {
        fn handle_direct_connection(&mut self, dst: Uri) -> Connecting {
            let fut = self.https.call(dst);
            Box::pin(async move {
                let conn = fut.await?;
                Ok(Conn {
                    inner: Box::new(conn),
                    is_proxy: false,
                })
            })
        }

        fn handle_http_through_proxy(
            &mut self,
            _dst: Uri,
            intercept: hyper_util::client::proxy::matcher::Intercept,
        ) -> Connecting {
            // For HTTP through proxy, connect to the proxy and let it handle the request
            let proxy_uri = intercept.uri().clone();
            let fut = self.https.call(proxy_uri);
            Box::pin(async move {
                let conn = fut.await?;
                Ok(Conn {
                    inner: Box::new(conn),
                    is_proxy: true,
                })
            })
        }

        fn handle_https_through_proxy(
            &mut self,
            dst: Uri,
            intercept: hyper_util::client::proxy::matcher::Intercept,
        ) -> Connecting {
            // For HTTPS through HTTP proxy, we need to:
            // 1. Establish CONNECT tunnel using the HTTPS connector
            // 2. Perform manual TLS handshake over the tunneled stream

            let tunnel = hyper_util::client::legacy::connect::proxy::Tunnel::new(
                intercept.uri().clone(),
                self.https.clone(),
            );

            // Configure tunnel with authentication if present
            let mut tunnel = if let Some(auth) = intercept.basic_auth() {
                tunnel.with_auth(auth.clone())
            } else {
                tunnel
            };

            let tls_config = self.tls_config.clone();
            let dst_clone = dst.clone();

            Box::pin(async move {
                // Stage 1: Establish CONNECT tunnel
                tracing::trace!("tunneling HTTPS over proxy using s2n-tls");
                let tunneled = tunnel
                    .call(dst_clone.clone())
                    .await
                    .map_err(|e| BoxError::from(format!("CONNECT tunnel failed: {e}")))?;

                // Stage 2: Manual TLS handshake over tunneled stream
                let host = dst_clone
                    .host()
                    .ok_or("missing host in URI for TLS handshake")?;

                // s2n-tls uses string server names (simpler than rustls ServerName)
                let tls_connector = s2n_tls_tokio::TlsConnector::new(tls_config);
                let tls_stream = tls_connector
                    .connect(host, TokioIo::new(tunneled))
                    .await
                    .map_err(|e| BoxError::from(format!("s2n-tls handshake failed: {e}")))?;

                Ok(Conn {
                    inner: Box::new(S2nTlsConn {
                        inner: TokioIo::new(tls_stream),
                    }),
                    is_proxy: true,
                })
            })
        }
    }

    // Simple wrapper that implements Connection for s2n-tls streams
    struct S2nTlsConn<T>
    where
        T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    {
        inner: TokioIo<s2n_tls_tokio::TlsStream<T>>,
    }

    impl<T> Connection for S2nTlsConn<T>
    where
        T: Connection + tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    {
        fn connected(&self) -> Connected {
            // For tunneled connections, we can't easily access the underlying connection info
            // from s2n-tls, so we'll return a basic Connected instance
            Connected::new()
        }
    }

    impl<T> hyper::rt::Read for S2nTlsConn<T>
    where
        T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    {
        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: hyper::rt::ReadBufCursor<'_>,
        ) -> Poll<tokio::io::Result<()>> {
            Pin::new(&mut self.get_mut().inner).poll_read(cx, buf)
        }
    }

    impl<T> hyper::rt::Write for S2nTlsConn<T>
    where
        T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    {
        fn poll_write(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<Result<usize, tokio::io::Error>> {
            Pin::new(&mut self.get_mut().inner).poll_write(cx, buf)
        }

        fn poll_flush(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Result<(), tokio::io::Error>> {
            Pin::new(&mut self.get_mut().inner).poll_flush(cx)
        }

        fn poll_shutdown(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Result<(), tokio::io::Error>> {
            Pin::new(&mut self.get_mut().inner).poll_shutdown(cx)
        }

        fn poll_write_vectored(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            bufs: &[IoSlice<'_>],
        ) -> Poll<Result<usize, tokio::io::Error>> {
            Pin::new(&mut self.get_mut().inner).poll_write_vectored(cx, bufs)
        }

        fn is_write_vectored(&self) -> bool {
            self.inner.is_write_vectored()
        }
    }
}
