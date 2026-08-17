//! # Rustls - a modern TLS library
//! Rustls is a TLS library that aims to provide a good level of cryptographic security,
//! requires no configuration to achieve that security, and provides no unsafe features or
//! obsolete cryptography.
//!
//! ## Current features
//!
//! * TLS1.2 and TLS1.3.
//! * ECDSA, Ed25519 or RSA server authentication by clients.
//! * ECDSA, Ed25519 or RSA server authentication by servers.
//! * Forward secrecy using ECDHE; with curve25519, nistp256 or nistp384 curves.
//! * AES128-GCM and AES256-GCM bulk encryption, with safe nonces.
//! * ChaCha20-Poly1305 bulk encryption ([RFC7905](https://tools.ietf.org/html/rfc7905)).
//! * ALPN support.
//! * SNI support.
//! * Tunable fragment size to make TLS messages match size of underlying transport.
//! * Optional use of vectored IO to minimise system calls.
//! * TLS1.2 session resumption.
//! * TLS1.2 resumption via tickets ([RFC5077](https://tools.ietf.org/html/rfc5077)).
//! * TLS1.3 resumption via tickets or session storage.
//! * TLS1.3 0-RTT data for clients.
//! * TLS1.3 0-RTT data for servers.
//! * Client authentication by clients.
//! * Client authentication by servers.
//! * Extended master secret support ([RFC7627](https://tools.ietf.org/html/rfc7627)).
//! * Exporters ([RFC5705](https://tools.ietf.org/html/rfc5705)).
//! * OCSP stapling by servers.
//! * SCT stapling by servers.
//! * SCT verification by clients.
//!
//! ## Possible future features
//!
//! * PSK support.
//! * OCSP verification by clients.
//! * Certificate pinning.
//!
//! ## Non-features
//!
//! For reasons [explained in the manual](manual),
//! rustls does not and will not support:
//!
//! * SSL1, SSL2, SSL3, TLS1 or TLS1.1.
//! * RC4.
//! * DES or triple DES.
//! * EXPORT ciphersuites.
//! * MAC-then-encrypt ciphersuites.
//! * Ciphersuites without forward secrecy.
//! * Renegotiation.
//! * Kerberos.
//! * Compression.
//! * Discrete-log Diffie-Hellman.
//! * Automatic protocol version downgrade.
//!
//! There are plenty of other libraries that provide these features should you
//! need them.
//!
//! ### Platform support
//!
//! While Rustls itself is platform independent it uses
//! [`ring`](https://crates.io/crates/ring) for implementing the cryptography in
//! TLS. As a result, rustls only runs on platforms
//! supported by `ring`. At the time of writing, this means 32-bit ARM, Aarch64 (64-bit ARM),
//! x86, x86-64, LoongArch64, 32-bit & 64-bit Little Endian MIPS, 32-bit PowerPC (Big Endian),
//! 64-bit PowerPC (Big and Little Endian), 64-bit RISC-V, and s390x. We do not presently
//! support WebAssembly.
//! For more information, see [the supported `ring` target platforms][ring-target-platforms].
//!
//! Rustls requires Rust 1.63 or later.
//!
//! [ring-target-platforms]: https://github.com/briansmith/ring/blob/2e8363b433fa3b3962c877d9ed2e9145612f3160/include/ring-core/target.h#L18-L64
//!
//! ## Design Overview
//! ### Rustls does not take care of network IO
//! It doesn't make or accept TCP connections, or do DNS, or read or write files.
//!
//! There's example client and server code which uses mio to do all needed network
//! IO.
//!
//! ### Rustls provides encrypted pipes
//! These are the [`ServerConnection`] and [`ClientConnection`] types.  You supply raw TLS traffic
//! on the left (via the [`read_tls()`] and [`write_tls()`] methods) and then read/write the
//! plaintext on the right:
//!
//! [`read_tls()`]: Connection::read_tls
//! [`write_tls()`]: Connection::read_tls
//!
//! ```text
//!          TLS                                   Plaintext
//!          ===                                   =========
//!     read_tls()      +-----------------------+      reader() as io::Read
//!                     |                       |
//!           +--------->   ClientConnection    +--------->
//!                     |          or           |
//!           <---------+   ServerConnection    <---------+
//!                     |                       |
//!     write_tls()     +-----------------------+      writer() as io::Write
//! ```
//!
//! ### Rustls takes care of server certificate verification
//! You do not need to provide anything other than a set of root certificates to trust.
//! Certificate verification cannot be turned off or disabled in the main API.
//!
//! ## Getting started
//! This is the minimum you need to do to make a TLS client connection.
//!
//! First we load some root certificates.  These are used to authenticate the server.
//! The recommended way is to depend on the `webpki_roots` crate which contains
//! the Mozilla set of root certificates.
//!
//! ```rust,no_run
//! let mut root_store = rustls::RootCertStore::empty();
//! root_store.add_trust_anchors(
//!     webpki_roots::TLS_SERVER_ROOTS
//!         .iter()
//!         .map(|ta| {
//!             rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
//!                 ta.subject,
//!                 ta.spki,
//!                 ta.name_constraints,
//!             )
//!         })
//! );
//! ```
//!
//! Next, we make a `ClientConfig`.  You're likely to make one of these per process,
//! and use it for all connections made by that process.
//!
//! ```rust,no_run
//! # let root_store: rustls::RootCertStore = panic!();
//! let config = rustls::ClientConfig::builder()
//!     .with_safe_defaults()
//!     .with_root_certificates(root_store)
//!     .with_no_client_auth();
//! ```
//!
//! Now we can make a connection.  You need to provide the server's hostname so we
//! know what to expect to find in the server's certificate.
//!
//! ```rust
//! # use rustls;
//! # use webpki;
//! # use std::sync::Arc;
//! # let mut root_store = rustls::RootCertStore::empty();
//! # root_store.add_trust_anchors(
//! #  webpki_roots::TLS_SERVER_ROOTS
//! #      .iter()
//! #      .map(|ta| {
//! #          rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
//! #              ta.subject,
//! #              ta.spki,
//! #              ta.name_constraints,
//! #          )
//! #      })
//! # );
//! # let config = rustls::ClientConfig::builder()
//! #     .with_safe_defaults()
//! #     .with_root_certificates(root_store)
//! #     .with_no_client_auth();
//! let rc_config = Arc::new(config);
//! let example_com = "example.com".try_into().unwrap();
//! let mut client = rustls::ClientConnection::new(rc_config, example_com);
//! ```
//!
//! Now you should do appropriate IO for the `client` object.  If `client.wants_read()` yields
//! true, you should call `client.read_tls()` when the underlying connection has data.
//! Likewise, if `client.wants_write()` yields true, you should call `client.write_tls()`
//! when the underlying connection is able to send data.  You should continue doing this
//! as long as the connection is valid.
//!
//! The return types of `read_tls()` and `write_tls()` only tell you if the IO worked.  No
//! parsing or processing of the TLS messages is done.  After each `read_tls()` you should
//! therefore call `client.process_new_packets()` which parses and processes the messages.
//! Any error returned from `process_new_packets` is fatal to the connection, and will tell you
//! why.  For example, if the server's certificate is expired `process_new_packets` will
//! return `Err(InvalidCertificate(Expired))`.  From this point on,
//! `process_new_packets` will not do any new work and will return that error continually.
//!
//! You can extract newly received data by calling `client.reader()` (which implements the
//! `io::Read` trait).  You can send data to the peer by calling `client.writer()` (which
//! implements `io::Write` trait).  Note that `client.writer().write()` buffers data you
//! send if the TLS connection is not yet established: this is useful for writing (say) a
//! HTTP request, but this is buffered so avoid large amounts of data.
//!
//! The following code uses a fictional socket IO API for illustration, and does not handle
//! errors.
//!
//! ```rust,no_run
//! # let mut client = rustls::ClientConnection::new(panic!(), panic!()).unwrap();
//! # struct Socket { }
//! # impl Socket {
//! #   fn ready_for_write(&self) -> bool { false }
//! #   fn ready_for_read(&self) -> bool { false }
//! #   fn wait_for_something_to_happen(&self) { }
//! # }
//! #
//! # use std::io::{Read, Write, Result};
//! # impl Read for Socket {
//! #   fn read(&mut self, buf: &mut [u8]) -> Result<usize> { panic!() }
//! # }
//! # impl Write for Socket {
//! #   fn write(&mut self, buf: &[u8]) -> Result<usize> { panic!() }
//! #   fn flush(&mut self) -> Result<()> { panic!() }
//! # }
//! #
//! # fn connect(_address: &str, _port: u16) -> Socket {
//! #   panic!();
//! # }
//! use std::io;
//! use rustls::Connection;
//!
//! client.writer().write(b"GET / HTTP/1.0\r\n\r\n").unwrap();
//! let mut socket = connect("example.com", 443);
//! loop {
//!   if client.wants_read() && socket.ready_for_read() {
//!     client.read_tls(&mut socket).unwrap();
//!     client.process_new_packets().unwrap();
//!
//!     let mut plaintext = Vec::new();
//!     client.reader().read_to_end(&mut plaintext).unwrap();
//!     io::stdout().write(&plaintext).unwrap();
//!   }
//!
//!   if client.wants_write() && socket.ready_for_write() {
//!     client.write_tls(&mut socket).unwrap();
//!   }
//!
//!   socket.wait_for_something_to_happen();
//! }
//! ```
//!
//! # Examples
//! [`tlsserver`](https://github.com/rustls/rustls/blob/main/examples/src/bin/tlsserver-mio.rs)
//! and [`tlsclient`](https://github.com/rustls/rustls/blob/main/examples/src/bin/tlsclient-mio.rs)
//! are full worked examples.  These both use mio.
//!
//! # Crate features
//! Here's a list of what features are exposed by the rustls crate and what
//! they mean.
//!
//! - `logging`: this makes the rustls crate depend on the `log` crate.
//!   rustls outputs interesting protocol-level messages at `trace!` and `debug!`
//!   level, and protocol-level errors at `warn!` and `error!` level.  The log
//!   messages do not contain secret key data, and so are safe to archive without
//!   affecting session security.  This feature is in the default set.
//!
//! - `dangerous_configuration`: this feature enables a `dangerous()` method on
//!   `ClientConfig` and `ServerConfig` that allows setting inadvisable options,
//!   such as replacing the certificate verification process.  Applications
//!   requesting this feature should be reviewed carefully.
//!
//! - `quic`: this feature exposes additional constructors and functions
//!   for using rustls as a TLS library for QUIC.  See the `quic` module for
//!   details of these.  You will only need this if you're writing a QUIC
//!   implementation.
//!
//! - `tls12`: enables support for TLS version 1.2. This feature is in the default
//!   set. Note that, due to the additive nature of Cargo features and because it
//!   is enabled by default, other crates in your dependency graph could re-enable
//!   it for your application. If you want to disable TLS 1.2 for security reasons,
//!   consider explicitly enabling TLS 1.3 only in the config builder API.
//!
//! - `read_buf`: When building with Rust Nightly, adds support for the unstable
//!   `std::io::ReadBuf` and related APIs. This reduces costs from initializing
//!   buffers. Will do nothing on non-Nightly releases.

// Require docs for public APIs, deny unsafe code, etc.
#![forbid(unsafe_code, unused_must_use)]
#![cfg_attr(not(any(read_buf, bench)), forbid(unstable_features))]
#![deny(
    clippy::clone_on_ref_ptr,
    clippy::use_self,
    trivial_casts,
    trivial_numeric_casts,
    missing_docs,
    unreachable_pub,
    unused_import_braces,
    unused_extern_crates,
    unused_qualifications
)]
// Relax these clippy lints:
// - ptr_arg: this triggers on references to type aliases that are Vec
//   underneath.
// - too_many_arguments: some things just need a lot of state, wrapping it
//   doesn't necessarily make it easier to follow what's going on
// - new_ret_no_self: we sometimes return `Arc<Self>`, which seems fine
// - single_component_path_imports: our top-level `use log` import causes
//   a false positive, https://github.com/rust-lang/rust-clippy/issues/5210
// - new_without_default: for internal constructors, the indirection is not
//   helpful
#![allow(
    clippy::too_many_arguments,
    clippy::new_ret_no_self,
    clippy::ptr_arg,
    clippy::single_component_path_imports,
    clippy::new_without_default
)]
// Enable documentation for all features on docs.rs
#![cfg_attr(docsrs, feature(doc_cfg))]
// XXX: Because of https://github.com/rust-lang/rust/issues/54726, we cannot
// write `#![rustversion::attr(nightly, feature(read_buf))]` here. Instead,
// build.rs set `read_buf` for (only) Rust Nightly to get the same effect.
//
// All the other conditional logic in the crate could use
// `#[rustversion::nightly]` instead of `#[cfg(read_buf)]`; `#[cfg(read_buf)]`
// is used to avoid needing `rustversion` to be compiled twice during
// cross-compiling.
#![cfg_attr(read_buf, feature(read_buf))]
#![cfg_attr(read_buf, feature(core_io_borrowed_buf))]
#![cfg_attr(bench, feature(test))]

// Import `test` sysroot crate for `Bencher` definitions.
#[cfg(bench)]
#[allow(unused_extern_crates)]
extern crate test;

// log for logging (optional).
#[cfg(feature = "logging")]
use log;

#[cfg(not(feature = "logging"))]
#[macro_use]
mod log {
    macro_rules! trace    ( ($($tt:tt)*) => {{}} );
    macro_rules! debug    ( ($($tt:tt)*) => {{}} );
    macro_rules! warn     ( ($($tt:tt)*) => {{}} );
}

#[macro_use]
mod msgs;
mod anchors;
mod cipher;
mod common_state;
mod conn;
mod dns_name;
mod error;
mod hash_hs;
mod limited_cache;
mod rand;
mod record_layer;
mod stream;
#[cfg(feature = "tls12")]
mod tls12;
mod tls13;
mod vecbuf;
mod verify;
#[cfg(test)]
mod verifybench;
mod x509;
#[macro_use]
mod check;
mod bs_debug;
mod builder;
mod enums;
mod key;
mod key_log;
mod key_log_file;
mod kx;
mod suites;
mod ticketer;
mod versions;

/// Internal classes which may be useful outside the library.
/// The contents of this section DO NOT form part of the stable interface.
pub mod internal {
    /// Low-level TLS message parsing and encoding functions.
    pub mod msgs {
        pub use crate::msgs::*;
    }
    /// Low-level TLS message decryption functions.
    pub mod cipher {
        pub use crate::cipher::MessageDecrypter;
    }
    /// Low-level TLS record layer functions.
    pub mod record_layer {
        pub use crate::record_layer::{Decrypted, RecordLayer};
    }
}

// The public interface is:
pub use crate::anchors::{OwnedTrustAnchor, RootCertStore};
pub use crate::builder::{
    ConfigBuilder, ConfigSide, WantsCipherSuites, WantsKxGroups, WantsVerifier, WantsVersions,
};
pub use crate::common_state::{CommonState, IoState, Side};
pub use crate::conn::{Connection, ConnectionCommon, Reader, SideData, Writer};
pub use crate::enums::{
    AlertDescription, CipherSuite, ContentType, HandshakeType, ProtocolVersion, SignatureAlgorithm,
    SignatureScheme,
};
pub use crate::error::{
    CertRevocationListError, CertificateError, Error, InvalidMessage, PeerIncompatible,
    PeerMisbehaved,
};
pub use crate::key::{Certificate, PrivateKey};
pub use crate::key_log::{KeyLog, NoKeyLog};
pub use crate::key_log_file::KeyLogFile;
pub use crate::kx::{SupportedKxGroup, ALL_KX_GROUPS};
pub use crate::msgs::enums::NamedGroup;
pub use crate::msgs::handshake::DistinguishedName;
pub use crate::stream::{Stream, StreamOwned};
pub use crate::suites::{
    BulkAlgorithm, SupportedCipherSuite, ALL_CIPHER_SUITES, DEFAULT_CIPHER_SUITES,
};
#[cfg(feature = "secret_extraction")]
#[cfg_attr(docsrs, doc(cfg(feature = "secret_extraction")))]
pub use crate::suites::{ConnectionTrafficSecrets, ExtractedSecrets};
pub use crate::ticketer::Ticketer;
#[cfg(feature = "tls12")]
pub use crate::tls12::Tls12CipherSuite;
pub use crate::tls13::Tls13CipherSuite;
pub use crate::verify::DigitallySignedStruct;
pub use crate::versions::{SupportedProtocolVersion, ALL_VERSIONS, DEFAULT_VERSIONS};

/// Items for use in a client.
pub mod client {
    pub(super) mod builder;
    mod client_conn;
    mod common;
    pub(super) mod handy;
    mod hs;
    #[cfg(feature = "tls12")]
    mod tls12;
    mod tls13;

    pub use crate::dns_name::InvalidDnsNameError;
    pub use builder::{WantsClientCert, WantsTransparencyPolicyOrClientCert};
    pub use client_conn::{
        ClientConfig, ClientConnection, ClientConnectionData, ClientSessionStore,
        ResolvesClientCert, Resumption, ServerName, Tls12Resumption, WriteEarlyData,
    };
    pub use handy::ClientSessionMemoryCache;

    #[cfg(feature = "dangerous_configuration")]
    pub use crate::verify::{
        verify_server_cert_signed_by_trust_anchor, verify_server_name,
        CertificateTransparencyPolicy, HandshakeSignatureValid, ServerCertVerified,
        ServerCertVerifier, WebPkiVerifier,
    };
    #[cfg(feature = "dangerous_configuration")]
    pub use client_conn::danger::DangerousClientConfig;

    pub use crate::msgs::persist::Tls12ClientSessionValue;
    pub use crate::msgs::persist::Tls13ClientSessionValue;
}

pub use client::{ClientConfig, ClientConnection, ServerName};

/// Items for use in a server.
pub mod server {
    pub(crate) mod builder;
    mod common;
    pub(crate) mod handy;
    mod hs;
    mod server_conn;
    #[cfg(feature = "tls12")]
    mod tls12;
    mod tls13;

    pub use crate::verify::{
        AllowAnyAnonymousOrAuthenticatedClient, AllowAnyAuthenticatedClient, NoClientAuth,
        UnparsedCertRevocationList,
    };
    pub use builder::WantsServerCert;
    pub use handy::ResolvesServerCertUsingSni;
    pub use handy::{NoServerSessionStorage, ServerSessionMemoryCache};
    pub use server_conn::StoresServerSessions;
    pub use server_conn::{
        Accepted, Acceptor, ReadEarlyData, ServerConfig, ServerConnection, ServerConnectionData,
    };
    pub use server_conn::{ClientHello, ProducesTickets, ResolvesServerCert};

    #[cfg(feature = "dangerous_configuration")]
    pub use crate::dns_name::DnsName;
    #[cfg(feature = "dangerous_configuration")]
    pub use crate::key::ParsedCertificate;
    #[cfg(feature = "dangerous_configuration")]
    pub use crate::verify::{ClientCertVerified, ClientCertVerifier};
}

pub use server::{ServerConfig, ServerConnection};

/// All defined ciphersuites appear in this module.
///
/// [`ALL_CIPHER_SUITES`] is provided as an array of all of these values.
pub mod cipher_suite {
    pub use crate::suites::CipherSuiteCommon;
    #[cfg(feature = "tls12")]
    pub use crate::tls12::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256;
    #[cfg(feature = "tls12")]
    pub use crate::tls12::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384;
    #[cfg(feature = "tls12")]
    pub use crate::tls12::TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256;
    #[cfg(feature = "tls12")]
    pub use crate::tls12::TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256;
    #[cfg(feature = "tls12")]
    pub use crate::tls12::TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384;
    #[cfg(feature = "tls12")]
    pub use crate::tls12::TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256;
    pub use crate::tls13::TLS13_AES_128_GCM_SHA256;
    pub use crate::tls13::TLS13_AES_256_GCM_SHA384;
    pub use crate::tls13::TLS13_CHACHA20_POLY1305_SHA256;
}

/// All defined protocol versions appear in this module.
///
/// ALL_VERSIONS is a provided as an array of all of these values.
pub mod version {
    #[cfg(feature = "tls12")]
    pub use crate::versions::TLS12;
    pub use crate::versions::TLS13;
}

/// All defined key exchange groups appear in this module.
///
/// ALL_KX_GROUPS is provided as an array of all of these values.
pub mod kx_group {
    pub use crate::kx::SECP256R1;
    pub use crate::kx::SECP384R1;
    pub use crate::kx::X25519;
}

/// Message signing interfaces and implementations.
pub mod sign;

#[cfg(feature = "quic")]
#[cfg_attr(docsrs, doc(cfg(feature = "quic")))]
/// APIs for implementing QUIC TLS
pub mod quic;

/// This is the rustls manual.
pub mod manual;
