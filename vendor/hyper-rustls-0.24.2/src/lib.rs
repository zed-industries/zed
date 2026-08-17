//! # hyper-rustls
//!
//! A pure-Rust HTTPS connector for [hyper](https://hyper.rs), based on
//! [Rustls](https://github.com/rustls/rustls).
//!
//! ## Example client
//!
//! ```no_run
//! # #[cfg(all(feature = "rustls-native-certs", feature = "tokio-runtime", feature = "http1"))]
//! # fn main() {
//! use hyper::{Body, Client, StatusCode, Uri};
//!
//! let mut rt = tokio::runtime::Runtime::new().unwrap();
//! let url = ("https://hyper.rs").parse().unwrap();
//! let https = hyper_rustls::HttpsConnectorBuilder::new()
//!     .with_native_roots()
//!     .https_only()
//!     .enable_http1()
//!     .build();
//!
//! let client: Client<_, hyper::Body> = Client::builder().build(https);
//!
//! let res = rt.block_on(client.get(url)).unwrap();
//! assert_eq!(res.status(), StatusCode::OK);
//! # }
//! # #[cfg(not(all(feature = "rustls-native-certs", feature = "tokio-runtime", feature = "http1")))]
//! # fn main() {}
//! ```
//!
//! ## Example server
//!
//! ```no_run
//! # #[cfg(all(feature = "rustls-native-certs", feature = "tokio-runtime", feature = "http1", feature = "acceptor"))]
//! # fn main() {
//! use hyper::server::conn::AddrIncoming;
//! use hyper::service::{make_service_fn, service_fn};
//! use hyper::{Body, Method, Request, Response, Server, StatusCode};
//! use hyper_rustls::TlsAcceptor;
//! use std::io;
//! use std::fs::File;
//!
//! let mut rt = tokio::runtime::Runtime::new().unwrap();
//! let addr = "127.0.0.1:1337".parse().unwrap();
//!
//! // Load public certificate.
//! let certfile = File::open("examples/sample.pem").unwrap();
//! let mut reader = io::BufReader::new(certfile);
//!
//! // Load and return certificate.
//! let certs = rustls_pemfile::certs(&mut reader).unwrap();
//! let certs = certs.into_iter().map(rustls::Certificate).collect();
//!
//! // Load private key. (see `examples/server.rs`)
//! let keyfile = File::open("examples/sample.rsa").unwrap();
//! let mut reader = io::BufReader::new(keyfile);
//!
//! // Load and return a single private key.
//! let keys = rustls_pemfile::rsa_private_keys(&mut reader).unwrap();
//! let key = rustls::PrivateKey(keys[0].clone());
//! let https = hyper_rustls::HttpsConnectorBuilder::new()
//!     .with_native_roots()
//!     .https_only()
//!     .enable_http1()
//!     .build();
//!
//! let incoming = AddrIncoming::bind(&addr).unwrap();
//! let acceptor = TlsAcceptor::builder()
//!     .with_single_cert(certs, key).unwrap()
//!     .with_all_versions_alpn()
//!     .with_incoming(incoming);
//! let service = make_service_fn(|_| async { Ok::<_, io::Error>(service_fn(|_req|async {Ok::<_, io::Error>(Response::new(Body::empty()))})) });
//! let server = Server::builder(acceptor).serve(service);
//! // server.await.unwrap();
//! # }
//! # #[cfg(not(all(feature = "rustls-native-certs", feature = "tokio-runtime", feature = "http1")))]
//! # fn main() {}
//! ```

#![warn(missing_docs, unreachable_pub, clippy::use_self)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "acceptor")]
/// TLS acceptor implementing hyper's `Accept` trait.
pub mod acceptor;
mod config;
mod connector;
mod stream;

#[cfg(feature = "logging")]
mod log {
    pub(crate) use log::{debug, trace};
}

#[cfg(not(feature = "logging"))]
mod log {
    macro_rules! trace    ( ($($tt:tt)*) => {{}} );
    macro_rules! debug    ( ($($tt:tt)*) => {{}} );
    pub(crate) use {debug, trace};
}

#[cfg(feature = "acceptor")]
pub use crate::acceptor::{AcceptorBuilder, TlsAcceptor};
pub use crate::config::ConfigBuilderExt;
pub use crate::connector::builder::ConnectorBuilder as HttpsConnectorBuilder;
pub use crate::connector::HttpsConnector;
pub use crate::stream::MaybeHttpsStream;

/// The various states of the [`HttpsConnectorBuilder`]
pub mod builderstates {
    #[cfg(feature = "http2")]
    pub use crate::connector::builder::WantsProtocols3;
    pub use crate::connector::builder::{
        WantsProtocols1, WantsProtocols2, WantsSchemes, WantsTlsConfig,
    };
}
