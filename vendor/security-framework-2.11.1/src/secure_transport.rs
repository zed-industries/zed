//! SSL/TLS encryption support using Secure Transport.
//!
//! # Examples
//!
//! To connect as a client to a server with a certificate trusted by the system:
//!
//! ```rust
//! use std::io::prelude::*;
//! use std::net::TcpStream;
//! use security_framework::secure_transport::ClientBuilder;
//!
//! let stream = TcpStream::connect("google.com:443").unwrap();
//! let mut stream = ClientBuilder::new().handshake("google.com", stream).unwrap();
//!
//! stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
//! let mut page = vec![];
//! stream.read_to_end(&mut page).unwrap();
//! println!("{}", String::from_utf8_lossy(&page));
//! ```
//!
//! To connect to a server with a certificate that's *not* trusted by the
//! system, specify the root certificates for the server's chain to the
//! `ClientBuilder`:
//!
//! ```rust,no_run
//! use std::io::prelude::*;
//! use std::net::TcpStream;
//! use security_framework::secure_transport::ClientBuilder;
//!
//! # let root_cert = unsafe { std::mem::zeroed() };
//! let stream = TcpStream::connect("my_server.com:443").unwrap();
//! let mut stream = ClientBuilder::new()
//!                      .anchor_certificates(&[root_cert])
//!                      .handshake("my_server.com", stream)
//!                      .unwrap();
//!
//! stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
//! let mut page = vec![];
//! stream.read_to_end(&mut page).unwrap();
//! println!("{}", String::from_utf8_lossy(&page));
//! ```
//!
//! For more advanced configuration, the `SslContext` type can be used directly.
//!
//! To run a server:
//!
//! ```rust,no_run
//! use std::net::TcpListener;
//! use std::thread;
//! use security_framework::secure_transport::{SslContext, SslProtocolSide, SslConnectionType};
//!
//! // Create a TCP listener and start accepting on it.
//! let mut listener = TcpListener::bind("0.0.0.0:443").unwrap();
//!
//! for stream in listener.incoming() {
//!     let stream = stream.unwrap();
//!     thread::spawn(move || {
//!         // Create a new context configured to operate on the server side of
//!         // a traditional SSL/TLS session.
//!         let mut ctx = SslContext::new(SslProtocolSide::SERVER, SslConnectionType::STREAM)
//!                           .unwrap();
//!
//!         // Install the certificate chain that we will be using.
//!         # let identity = unsafe { std::mem::zeroed() };
//!         # let intermediate_cert = unsafe { std::mem::zeroed() };
//!         # let root_cert = unsafe { std::mem::zeroed() };
//!         ctx.set_certificate(identity, &[intermediate_cert, root_cert]).unwrap();
//!
//!         // Perform the SSL/TLS handshake and get our stream.
//!         let mut stream = ctx.handshake(stream).unwrap();
//!     });
//! }
//!
//! ```
#[allow(unused_imports)]
use core_foundation::array::{CFArray, CFArrayRef};

use core_foundation::base::{Boolean, TCFType};
#[cfg(feature = "alpn")]
use core_foundation::string::CFString;
use core_foundation_sys::base::{kCFAllocatorDefault, OSStatus};
use std::os::raw::c_void;

#[allow(unused_imports)]
use security_framework_sys::base::{
    errSecBadReq, errSecIO, errSecNotTrusted, errSecSuccess, errSecTrustSettingDeny,
    errSecUnimplemented,
};

use security_framework_sys::secure_transport::*;
use std::any::Any;
use std::cmp;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::marker::PhantomData;
use std::panic::{self, AssertUnwindSafe};
use std::ptr;
use std::result;
use std::slice;

use crate::base::{Error, Result};
use crate::certificate::SecCertificate;
use crate::cipher_suite::CipherSuite;
use crate::identity::SecIdentity;
use crate::import_export::Pkcs12ImportOptions;
use crate::policy::SecPolicy;
use crate::trust::SecTrust;
use crate::{cvt, AsInner};
use security_framework_sys::base::errSecParam;

/// Specifies a side of a TLS session.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SslProtocolSide(SSLProtocolSide);

impl SslProtocolSide {
    /// The server side of the session.
    pub const SERVER: Self = Self(kSSLServerSide);

    /// The client side of the session.
    pub const CLIENT: Self = Self(kSSLClientSide);
}

/// Specifies the type of TLS session.
#[derive(Debug, Copy, Clone)]
pub struct SslConnectionType(SSLConnectionType);

impl SslConnectionType {
    /// A traditional TLS stream.
    pub const STREAM: Self = Self(kSSLStreamType);

    /// A DTLS session.
    pub const DATAGRAM: Self = Self(kSSLDatagramType);
}

/// An error or intermediate state after a TLS handshake attempt.
#[derive(Debug)]
pub enum HandshakeError<S> {
    /// The handshake failed.
    Failure(Error),
    /// The handshake was interrupted midway through.
    Interrupted(MidHandshakeSslStream<S>),
}

impl<S> From<Error> for HandshakeError<S> {
    #[inline(always)]
    fn from(err: Error) -> Self {
        Self::Failure(err)
    }
}

/// An error or intermediate state after a TLS handshake attempt.
#[derive(Debug)]
pub enum ClientHandshakeError<S> {
    /// The handshake failed.
    Failure(Error),
    /// The handshake was interrupted midway through.
    Interrupted(MidHandshakeClientBuilder<S>),
}

impl<S> From<Error> for ClientHandshakeError<S> {
    #[inline(always)]
    fn from(err: Error) -> Self {
        Self::Failure(err)
    }
}

/// An SSL stream midway through the handshake process.
#[derive(Debug)]
pub struct MidHandshakeSslStream<S> {
    stream: SslStream<S>,
    error: Error,
}

impl<S> MidHandshakeSslStream<S> {
    /// Returns a shared reference to the inner stream.
    #[inline(always)]
    #[must_use]
    pub fn get_ref(&self) -> &S {
        self.stream.get_ref()
    }

    /// Returns a mutable reference to the inner stream.
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut S {
        self.stream.get_mut()
    }

    /// Returns a shared reference to the `SslContext` of the stream.
    #[inline(always)]
    #[must_use]
    pub fn context(&self) -> &SslContext {
        self.stream.context()
    }

    /// Returns a mutable reference to the `SslContext` of the stream.
    #[inline(always)]
    pub fn context_mut(&mut self) -> &mut SslContext {
        self.stream.context_mut()
    }

    /// Returns `true` iff `break_on_server_auth` was set and the handshake has
    /// progressed to that point.
    #[inline(always)]
    #[must_use]
    pub fn server_auth_completed(&self) -> bool {
        self.error.code() == errSSLPeerAuthCompleted
    }

    /// Returns `true` iff `break_on_cert_requested` was set and the handshake
    /// has progressed to that point.
    #[inline(always)]
    #[must_use]
    pub fn client_cert_requested(&self) -> bool {
        self.error.code() == errSSLClientCertRequested
    }

    /// Returns `true` iff the underlying stream returned an error with the
    /// `WouldBlock` kind.
    #[inline(always)]
    #[must_use]
    pub fn would_block(&self) -> bool {
        self.error.code() == errSSLWouldBlock
    }

    /// Returns the error which caused the handshake interruption.
    #[inline(always)]
    #[must_use]
    pub fn error(&self) -> &Error {
        &self.error
    }

    /// Restarts the handshake process.
    #[inline(always)]
    pub fn handshake(self) -> result::Result<SslStream<S>, HandshakeError<S>> {
        self.stream.handshake()
    }
}

/// An SSL stream midway through the handshake process.
#[derive(Debug)]
pub struct MidHandshakeClientBuilder<S> {
    stream: MidHandshakeSslStream<S>,
    domain: Option<String>,
    certs: Vec<SecCertificate>,
    trust_certs_only: bool,
    danger_accept_invalid_certs: bool,
}

impl<S> MidHandshakeClientBuilder<S> {
    /// Returns a shared reference to the inner stream.
    #[inline(always)]
    #[must_use]
    pub fn get_ref(&self) -> &S {
        self.stream.get_ref()
    }

    /// Returns a mutable reference to the inner stream.
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut S {
        self.stream.get_mut()
    }

    /// Returns the error which caused the handshake interruption.
    #[inline(always)]
    #[must_use]
    pub fn error(&self) -> &Error {
        self.stream.error()
    }

    /// Restarts the handshake process.
    pub fn handshake(self) -> result::Result<SslStream<S>, ClientHandshakeError<S>> {
        let MidHandshakeClientBuilder {
            stream,
            domain,
            certs,
            trust_certs_only,
            danger_accept_invalid_certs,
        } = self;

        let mut result = stream.handshake();
        loop {
            let stream = match result {
                Ok(stream) => return Ok(stream),
                Err(HandshakeError::Interrupted(stream)) => stream,
                Err(HandshakeError::Failure(err)) => {
                    return Err(ClientHandshakeError::Failure(err))
                }
            };

            if stream.would_block() {
                let ret = MidHandshakeClientBuilder {
                    stream,
                    domain,
                    certs,
                    trust_certs_only,
                    danger_accept_invalid_certs,
                };
                return Err(ClientHandshakeError::Interrupted(ret));
            }

            if stream.server_auth_completed() {
                if danger_accept_invalid_certs {
                    result = stream.handshake();
                    continue;
                }
                let mut trust = match stream.context().peer_trust2()? {
                    Some(trust) => trust,
                    None => {
                        result = stream.handshake();
                        continue;
                    }
                };
                trust.set_anchor_certificates(&certs)?;
                trust.set_trust_anchor_certificates_only(self.trust_certs_only)?;
                let policy = SecPolicy::create_ssl(SslProtocolSide::SERVER, domain.as_deref());
                trust.set_policy(&policy)?;
                trust.evaluate_with_error().map_err(|error| {
                    #[cfg(feature = "log")]
                    log::warn!("SecTrustEvaluateWithError: {}", error.to_string());
                    Error::from_code(error.code() as _)
                })?;
                result = stream.handshake();
                continue;
            }

            let err = Error::from_code(stream.error().code());
            return Err(ClientHandshakeError::Failure(err));
        }
    }
}

/// Specifies the state of a TLS session.
#[derive(Debug, PartialEq, Eq)]
pub struct SessionState(SSLSessionState);

impl SessionState {
    /// The session has not yet started.
    pub const IDLE: Self = Self(kSSLIdle);

    /// The session is in the handshake process.
    pub const HANDSHAKE: Self = Self(kSSLHandshake);

    /// The session is connected.
    pub const CONNECTED: Self = Self(kSSLConnected);

    /// The session has been terminated.
    pub const CLOSED: Self = Self(kSSLClosed);

    /// The session has been aborted due to an error.
    pub const ABORTED: Self = Self(kSSLAborted);
}

/// Specifies a server's requirement for client certificates.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SslAuthenticate(SSLAuthenticate);

impl SslAuthenticate {
    /// Do not request a client certificate.
    pub const NEVER: Self = Self(kNeverAuthenticate);

    /// Require a client certificate.
    pub const ALWAYS: Self = Self(kAlwaysAuthenticate);

    /// Request but do not require a client certificate.
    pub const TRY: Self = Self(kTryAuthenticate);
}

/// Specifies the state of client certificate processing.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SslClientCertificateState(SSLClientCertificateState);

impl SslClientCertificateState {
    /// A client certificate has not been requested or sent.
    pub const NONE: Self = Self(kSSLClientCertNone);

    /// A client certificate has been requested but not recieved.
    pub const REQUESTED: Self = Self(kSSLClientCertRequested);
    /// A client certificate has been received and successfully validated.
    pub const SENT: Self = Self(kSSLClientCertSent);

    /// A client certificate has been received but has failed to validate.
    pub const REJECTED: Self = Self(kSSLClientCertRejected);
}

/// Specifies protocol versions.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SslProtocol(SSLProtocol);

impl SslProtocol {
    /// No protocol has been or should be negotiated or specified; use the default.
    pub const UNKNOWN: Self = Self(kSSLProtocolUnknown);

    /// The SSL 3.0 protocol is preferred, though SSL 2.0 may be used if the peer does not support
    /// SSL 3.0.
    pub const SSL3: Self = Self(kSSLProtocol3);

    /// The TLS 1.0 protocol is preferred, though lower versions may be used
    /// if the peer does not support TLS 1.0.
    pub const TLS1: Self = Self(kTLSProtocol1);

    /// The TLS 1.1 protocol is preferred, though lower versions may be used
    /// if the peer does not support TLS 1.1.
    pub const TLS11: Self = Self(kTLSProtocol11);

    /// The TLS 1.2 protocol is preferred, though lower versions may be used
    /// if the peer does not support TLS 1.2.
    pub const TLS12: Self = Self(kTLSProtocol12);

    /// The TLS 1.3 protocol is preferred, though lower versions may be used
    /// if the peer does not support TLS 1.3.
    pub const TLS13: Self = Self(kTLSProtocol13);

    /// Only the SSL 2.0 protocol is accepted.
    pub const SSL2: Self = Self(kSSLProtocol2);

    /// The `DTLSv1` protocol is preferred.
    pub const DTLS1: Self = Self(kDTLSProtocol1);

    /// Only the SSL 3.0 protocol is accepted.
    pub const SSL3_ONLY: Self = Self(kSSLProtocol3Only);

    /// Only the TLS 1.0 protocol is accepted.
    pub const TLS1_ONLY: Self = Self(kTLSProtocol1Only);

    /// All supported TLS/SSL versions are accepted.
    pub const ALL: Self = Self(kSSLProtocolAll);
}

declare_TCFType! {
    /// A Secure Transport SSL/TLS context object.
    SslContext, SSLContextRef
}

impl_TCFType!(SslContext, SSLContextRef, SSLContextGetTypeID);

impl fmt::Debug for SslContext {
    #[cold]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = fmt.debug_struct("SslContext");
        if let Ok(state) = self.state() {
            builder.field("state", &state);
        }
        builder.finish()
    }
}

unsafe impl Sync for SslContext {}
unsafe impl Send for SslContext {}

impl AsInner for SslContext {
    type Inner = SSLContextRef;

    #[inline(always)]
    fn as_inner(&self) -> SSLContextRef {
        self.0
    }
}

macro_rules! impl_options {
    ($($(#[$a:meta])* const $opt:ident: $get:ident & $set:ident,)*) => {
        $(
            $(#[$a])*
            #[inline(always)]
            pub fn $set(&mut self, value: bool) -> Result<()> {
                unsafe { cvt(SSLSetSessionOption(self.0, $opt, value as Boolean)) }
            }

            $(#[$a])*
            #[inline]
            pub fn $get(&self) -> Result<bool> {
                let mut value = 0;
                unsafe { cvt(SSLGetSessionOption(self.0, $opt, &mut value))?; }
                Ok(value != 0)
            }
        )*
    }
}

impl SslContext {
    /// Creates a new `SslContext` for the specified side and type of SSL
    /// connection.
    #[inline]
    pub fn new(side: SslProtocolSide, type_: SslConnectionType) -> Result<Self> {
        unsafe {
            let ctx = SSLCreateContext(kCFAllocatorDefault, side.0, type_.0);
            Ok(Self(ctx))
        }
    }

    /// Sets the fully qualified domain name of the peer.
    ///
    /// This will be used on the client side of a session to validate the
    /// common name field of the server's certificate. It has no effect if
    /// called on a server-side `SslContext`.
    ///
    /// It is *highly* recommended to call this method before starting the
    /// handshake process.
    #[inline]
    pub fn set_peer_domain_name(&mut self, peer_name: &str) -> Result<()> {
        unsafe {
            // SSLSetPeerDomainName doesn't need a null terminated string
            cvt(SSLSetPeerDomainName(
                self.0,
                peer_name.as_ptr().cast(),
                peer_name.len(),
            ))
        }
    }

    /// Returns the peer domain name set by `set_peer_domain_name`.
    pub fn peer_domain_name(&self) -> Result<String> {
        unsafe {
            let mut len = 0;
            cvt(SSLGetPeerDomainNameLength(self.0, &mut len))?;
            let mut buf = vec![0; len];
            cvt(SSLGetPeerDomainName(
                self.0,
                buf.as_mut_ptr().cast(),
                &mut len,
            ))?;
            Ok(String::from_utf8(buf).unwrap())
        }
    }

    /// Sets the certificate to be used by this side of the SSL session.
    ///
    /// This must be called before the handshake for server-side connections,
    /// and can be used on the client-side to specify a client certificate.
    ///
    /// The `identity` corresponds to the leaf certificate and private
    /// key, and the `certs` correspond to extra certificates in the chain.
    pub fn set_certificate(
        &mut self,
        identity: &SecIdentity,
        certs: &[SecCertificate],
    ) -> Result<()> {
        let mut arr = vec![identity.as_CFType()];
        arr.extend(certs.iter().map(|c| c.as_CFType()));
        let certs = CFArray::from_CFTypes(&arr);

        unsafe { cvt(SSLSetCertificate(self.0, certs.as_concrete_TypeRef())) }
    }

    /// Sets the peer ID of this session.
    ///
    /// A peer ID is an opaque sequence of bytes that will be used by Secure
    /// Transport to identify the peer of an SSL session. If the peer ID of
    /// this session matches that of a previously terminated session, the
    /// previous session can be resumed without requiring a full handshake.
    #[inline]
    pub fn set_peer_id(&mut self, peer_id: &[u8]) -> Result<()> {
        unsafe { cvt(SSLSetPeerID(self.0, peer_id.as_ptr().cast(), peer_id.len())) }
    }

    /// Returns the peer ID of this session.
    pub fn peer_id(&self) -> Result<Option<&[u8]>> {
        unsafe {
            let mut ptr = ptr::null();
            let mut len = 0;
            cvt(SSLGetPeerID(self.0, &mut ptr, &mut len))?;
            if ptr.is_null() {
                Ok(None)
            } else {
                Ok(Some(slice::from_raw_parts(ptr.cast(), len)))
            }
        }
    }

    /// Returns the list of ciphers that are supported by Secure Transport.
    pub fn supported_ciphers(&self) -> Result<Vec<CipherSuite>> {
        unsafe {
            let mut num_ciphers = 0;
            cvt(SSLGetNumberSupportedCiphers(self.0, &mut num_ciphers))?;
            let mut ciphers = vec![0; num_ciphers];
            cvt(SSLGetSupportedCiphers(
                self.0,
                ciphers.as_mut_ptr(),
                &mut num_ciphers,
            ))?;
            Ok(ciphers.iter().map(|c| CipherSuite::from_raw(*c)).collect())
        }
    }

    /// Returns the list of ciphers that are eligible to be used for
    /// negotiation.
    pub fn enabled_ciphers(&self) -> Result<Vec<CipherSuite>> {
        unsafe {
            let mut num_ciphers = 0;
            cvt(SSLGetNumberEnabledCiphers(self.0, &mut num_ciphers))?;
            let mut ciphers = vec![0; num_ciphers];
            cvt(SSLGetEnabledCiphers(
                self.0,
                ciphers.as_mut_ptr(),
                &mut num_ciphers,
            ))?;
            Ok(ciphers.iter().map(|c| CipherSuite::from_raw(*c)).collect())
        }
    }

    /// Sets the list of ciphers that are eligible to be used for negotiation.
    pub fn set_enabled_ciphers(&mut self, ciphers: &[CipherSuite]) -> Result<()> {
        let ciphers = ciphers.iter().map(|c| c.to_raw()).collect::<Vec<_>>();
        unsafe {
            cvt(SSLSetEnabledCiphers(
                self.0,
                ciphers.as_ptr(),
                ciphers.len(),
            ))
        }
    }

    /// Returns the cipher being used by the session.
    #[inline]
    pub fn negotiated_cipher(&self) -> Result<CipherSuite> {
        unsafe {
            let mut cipher = 0;
            cvt(SSLGetNegotiatedCipher(self.0, &mut cipher))?;
            Ok(CipherSuite::from_raw(cipher))
        }
    }

    /// Sets the requirements for client certificates.
    ///
    /// Should only be called on server-side sessions.
    #[inline]
    pub fn set_client_side_authenticate(&mut self, auth: SslAuthenticate) -> Result<()> {
        unsafe { cvt(SSLSetClientSideAuthenticate(self.0, auth.0)) }
    }

    /// Returns the state of client certificate processing.
    #[inline]
    pub fn client_certificate_state(&self) -> Result<SslClientCertificateState> {
        let mut state = 0;

        unsafe {
            cvt(SSLGetClientCertificateState(self.0, &mut state))?;
        }
        Ok(SslClientCertificateState(state))
    }

    /// Returns the `SecTrust` object corresponding to the peer.
    ///
    /// This can be used in conjunction with `set_break_on_server_auth` to
    /// validate certificates which do not have roots in the default set.
    pub fn peer_trust2(&self) -> Result<Option<SecTrust>> {
        // Calling SSLCopyPeerTrust on an idle connection does not seem to be well defined,
        // so explicitly check for that
        if self.state()? == SessionState::IDLE {
            return Err(Error::from_code(errSecBadReq));
        }

        unsafe {
            let mut trust = ptr::null_mut();
            cvt(SSLCopyPeerTrust(self.0, &mut trust))?;
            if trust.is_null() {
                Ok(None)
            } else {
                Ok(Some(SecTrust::wrap_under_create_rule(trust)))
            }
        }
    }

    /// Returns the state of the session.
    #[inline]
    pub fn state(&self) -> Result<SessionState> {
        unsafe {
            let mut state = 0;
            cvt(SSLGetSessionState(self.0, &mut state))?;
            Ok(SessionState(state))
        }
    }

    /// Returns the protocol version being used by the session.
    #[inline]
    pub fn negotiated_protocol_version(&self) -> Result<SslProtocol> {
        unsafe {
            let mut version = 0;
            cvt(SSLGetNegotiatedProtocolVersion(self.0, &mut version))?;
            Ok(SslProtocol(version))
        }
    }

    /// Returns the maximum protocol version allowed by the session.
    #[inline]
    pub fn protocol_version_max(&self) -> Result<SslProtocol> {
        unsafe {
            let mut version = 0;
            cvt(SSLGetProtocolVersionMax(self.0, &mut version))?;
            Ok(SslProtocol(version))
        }
    }

    /// Sets the maximum protocol version allowed by the session.
    #[inline]
    pub fn set_protocol_version_max(&mut self, max_version: SslProtocol) -> Result<()> {
        unsafe { cvt(SSLSetProtocolVersionMax(self.0, max_version.0)) }
    }

    /// Returns the minimum protocol version allowed by the session.
    #[inline]
    pub fn protocol_version_min(&self) -> Result<SslProtocol> {
        unsafe {
            let mut version = 0;
            cvt(SSLGetProtocolVersionMin(self.0, &mut version))?;
            Ok(SslProtocol(version))
        }
    }

    /// Sets the minimum protocol version allowed by the session.
    #[inline]
    pub fn set_protocol_version_min(&mut self, min_version: SslProtocol) -> Result<()> {
        unsafe { cvt(SSLSetProtocolVersionMin(self.0, min_version.0)) }
    }

    /// Returns the set of protocols selected via ALPN if it succeeded.
    #[cfg(feature = "alpn")]
    pub fn alpn_protocols(&self) -> Result<Vec<String>> {
        let mut array: CFArrayRef = ptr::null();
        unsafe {
            #[cfg(feature = "OSX_10_13")]
            {
                cvt(SSLCopyALPNProtocols(self.0, &mut array))?;
            }

            #[cfg(not(feature = "OSX_10_13"))]
            {
                dlsym! { fn SSLCopyALPNProtocols(SSLContextRef, *mut CFArrayRef) -> OSStatus }
                if let Some(f) = SSLCopyALPNProtocols.get() {
                    cvt(f(self.0, &mut array))?;
                } else {
                    return Err(Error::from_code(errSecUnimplemented));
                }
            }

            if array.is_null() {
                return Ok(vec![]);
            }

            let array = CFArray::<CFString>::wrap_under_create_rule(array);
            Ok(array.into_iter().map(|p| p.to_string()).collect())
        }
    }

    /// Configures the set of protocols use for ALPN.
    ///
    /// This is only used for client-side connections.
    #[cfg(feature = "alpn")]
    pub fn set_alpn_protocols(&mut self, protocols: &[&str]) -> Result<()> {
        // When CFMutableArray is added to core-foundation and IntoIterator trait
        // is implemented for CFMutableArray, the code below should directly collect
        // into a CFMutableArray.
        let protocols = CFArray::from_CFTypes(
            &protocols
                .iter()
                .map(|proto| CFString::new(proto))
                .collect::<Vec<_>>(),
        );

        #[cfg(feature = "OSX_10_13")]
        {
            unsafe { cvt(SSLSetALPNProtocols(self.0, protocols.as_concrete_TypeRef())) }
        }
        #[cfg(not(feature = "OSX_10_13"))]
        {
            dlsym! { fn SSLSetALPNProtocols(SSLContextRef, CFArrayRef) -> OSStatus }
            if let Some(f) = SSLSetALPNProtocols.get() {
                unsafe { cvt(f(self.0, protocols.as_concrete_TypeRef())) }
            } else {
                Err(Error::from_code(errSecUnimplemented))
            }
        }
    }

    /// Sets whether the client sends the `SessionTicket` extension in its `ClientHello`.
    ///
    /// On its own, this will just cause the client to send an empty `SessionTicket` extension on
    /// every connection. [`SslContext::set_peer_id`] must also be used to key the session
    /// ticket returned by the server.
    ///
    /// [`SslContext::set_peer_id`]: #method.set_peer_id
    #[cfg(feature = "session-tickets")]
    pub fn set_session_tickets_enabled(&mut self, enabled: bool) -> Result<()> {
        #[cfg(feature = "OSX_10_13")]
        {
            unsafe { cvt(SSLSetSessionTicketsEnabled(self.0, enabled as Boolean)) }
        }
        #[cfg(not(feature = "OSX_10_13"))]
        {
            dlsym! { fn SSLSetSessionTicketsEnabled(SSLContextRef, Boolean) -> OSStatus }
            if let Some(f) = SSLSetSessionTicketsEnabled.get() {
                unsafe { cvt(f(self.0, enabled as Boolean)) }
            } else {
                Err(Error::from_code(errSecUnimplemented))
            }
        }
    }

    /// Sets whether a protocol is enabled or not.
    ///
    /// # Note
    ///
    /// On OSX this is a deprecated API in favor of `set_protocol_version_max` and
    /// `set_protocol_version_min`, although if you're working with OSX 10.8 or before you may have
    /// to use this API instead.
    #[cfg(target_os = "macos")]
    #[deprecated(note = "use `set_protocol_version_max`")]
    pub fn set_protocol_version_enabled(
        &mut self,
        protocol: SslProtocol,
        enabled: bool,
    ) -> Result<()> {
        unsafe {
            cvt(SSLSetProtocolVersionEnabled(
                self.0,
                protocol.0,
                Boolean::from(enabled),
            ))
        }
    }

    /// Returns the number of bytes which can be read without triggering a
    /// `read` call in the underlying stream.
    #[inline]
    pub fn buffered_read_size(&self) -> Result<usize> {
        unsafe {
            let mut size = 0;
            cvt(SSLGetBufferedReadSize(self.0, &mut size))?;
            Ok(size)
        }
    }

    impl_options! {
        /// If enabled, the handshake process will pause and return instead of
        /// automatically validating a server's certificate.
        const kSSLSessionOptionBreakOnServerAuth: break_on_server_auth & set_break_on_server_auth,
        /// If enabled, the handshake process will pause and return after
        /// the server requests a certificate from the client.
        const kSSLSessionOptionBreakOnCertRequested: break_on_cert_requested & set_break_on_cert_requested,
        /// If enabled, the handshake process will pause and return instead of
        /// automatically validating a client's certificate.
        const kSSLSessionOptionBreakOnClientAuth: break_on_client_auth & set_break_on_client_auth,
        /// If enabled, TLS false start will be performed if an appropriate
        /// cipher suite is negotiated.
        ///
        const kSSLSessionOptionFalseStart: false_start & set_false_start,
        /// If enabled, 1/n-1 record splitting will be enabled for TLS 1.0
        /// connections using block ciphers to mitigate the BEAST attack.
        ///
        const kSSLSessionOptionSendOneByteRecord: send_one_byte_record & set_send_one_byte_record,
    }

    fn into_stream<S>(self, stream: S) -> Result<SslStream<S>>
    where
        S: Read + Write,
    {
        unsafe {
            let ret = SSLSetIOFuncs(self.0, read_func::<S>, write_func::<S>);
            if ret != errSecSuccess {
                return Err(Error::from_code(ret));
            }

            let stream = Connection {
                stream,
                err: None,
                panic: None,
            };
            let stream = Box::into_raw(Box::new(stream));
            let ret = SSLSetConnection(self.0, stream.cast());
            if ret != errSecSuccess {
                let _conn = Box::from_raw(stream);
                return Err(Error::from_code(ret));
            }

            Ok(SslStream {
                ctx: self,
                _m: PhantomData,
            })
        }
    }

    /// Performs the SSL/TLS handshake.
    pub fn handshake<S>(self, stream: S) -> result::Result<SslStream<S>, HandshakeError<S>>
    where
        S: Read + Write,
    {
        self.into_stream(stream)
            .map_err(HandshakeError::Failure)
            .and_then(SslStream::handshake)
    }
}

struct Connection<S> {
    stream: S,
    err: Option<io::Error>,
    panic: Option<Box<dyn Any + Send>>,
}

// the logic here is based off of libcurl's
#[cold]
fn translate_err(e: &io::Error) -> OSStatus {
    match e.kind() {
        io::ErrorKind::NotFound => errSSLClosedGraceful,
        io::ErrorKind::ConnectionReset => errSSLClosedAbort,
        io::ErrorKind::WouldBlock |
        io::ErrorKind::NotConnected => errSSLWouldBlock,
        _ => errSecIO,
    }
}

unsafe extern "C" fn read_func<S>(
    connection: SSLConnectionRef,
    data: *mut c_void,
    data_length: *mut usize,
) -> OSStatus
where
    S: Read,
{
    let conn: &mut Connection<S> = &mut *(connection as *mut _);
    let data = slice::from_raw_parts_mut(data.cast::<u8>(), *data_length);
    let mut start = 0;
    let mut ret = errSecSuccess;

    while start < data.len() {
        match panic::catch_unwind(AssertUnwindSafe(|| conn.stream.read(&mut data[start..]))) {
            Ok(Ok(0)) => {
                ret = errSSLClosedNoNotify;
                break;
            }
            Ok(Ok(len)) => start += len,
            Ok(Err(e)) => {
                ret = translate_err(&e);
                conn.err = Some(e);
                break;
            }
            Err(e) => {
                ret = errSecIO;
                conn.panic = Some(e);
                break;
            }
        }
    }

    *data_length = start;
    ret
}

unsafe extern "C" fn write_func<S>(
    connection: SSLConnectionRef,
    data: *const c_void,
    data_length: *mut usize,
) -> OSStatus
where
    S: Write,
{
    let conn: &mut Connection<S> = &mut *(connection as *mut _);
    let data = slice::from_raw_parts(data as *mut u8, *data_length);
    let mut start = 0;
    let mut ret = errSecSuccess;

    while start < data.len() {
        match panic::catch_unwind(AssertUnwindSafe(|| conn.stream.write(&data[start..]))) {
            Ok(Ok(0)) => {
                ret = errSSLClosedNoNotify;
                break;
            }
            Ok(Ok(len)) => start += len,
            Ok(Err(e)) => {
                ret = translate_err(&e);
                conn.err = Some(e);
                break;
            }
            Err(e) => {
                ret = errSecIO;
                conn.panic = Some(e);
                break;
            }
        }
    }

    *data_length = start;
    ret
}

/// A type implementing SSL/TLS encryption over an underlying stream.
pub struct SslStream<S> {
    ctx: SslContext,
    _m: PhantomData<S>,
}

impl<S: fmt::Debug> fmt::Debug for SslStream<S> {
    #[cold]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("SslStream")
            .field("context", &self.ctx)
            .field("stream", self.get_ref())
            .finish()
    }
}

impl<S> Drop for SslStream<S> {
    fn drop(&mut self) {
        unsafe {
            let mut conn = ptr::null();
            let ret = SSLGetConnection(self.ctx.0, &mut conn);
            assert!(ret == errSecSuccess);
            let _ = Box::<Connection<S>>::from_raw(conn as *mut _);
        }
    }
}

impl<S> SslStream<S> {
    fn handshake(mut self) -> result::Result<Self, HandshakeError<S>> {
        match unsafe { SSLHandshake(self.ctx.0) } {
            errSecSuccess => Ok(self),
            reason @ errSSLPeerAuthCompleted
            | reason @ errSSLClientCertRequested
            | reason @ errSSLWouldBlock
            | reason @ errSSLClientHelloReceived => {
                Err(HandshakeError::Interrupted(MidHandshakeSslStream {
                    stream: self,
                    error: Error::from_code(reason),
                }))
            }
            err => {
                self.check_panic();
                Err(HandshakeError::Failure(Error::from_code(err)))
            }
        }
    }

    /// Returns a shared reference to the inner stream.
    #[inline(always)]
    #[must_use]
    pub fn get_ref(&self) -> &S {
        &self.connection().stream
    }

    /// Returns a mutable reference to the underlying stream.
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut S {
        &mut self.connection_mut().stream
    }

    /// Returns a shared reference to the `SslContext` of the stream.
    #[inline(always)]
    #[must_use]
    pub fn context(&self) -> &SslContext {
        &self.ctx
    }

    /// Returns a mutable reference to the `SslContext` of the stream.
    #[inline(always)]
    pub fn context_mut(&mut self) -> &mut SslContext {
        &mut self.ctx
    }

    /// Shuts down the connection.
    pub fn close(&mut self) -> result::Result<(), io::Error> {
        unsafe {
            let ret = SSLClose(self.ctx.0);
            if ret == errSecSuccess {
                Ok(())
            } else {
                Err(self.get_error(ret))
            }
        }
    }

    fn connection(&self) -> &Connection<S> {
        unsafe {
            let mut conn = ptr::null();
            let ret = SSLGetConnection(self.ctx.0, &mut conn);
            assert!(ret == errSecSuccess);

            &mut *(conn as *mut Connection<S>)
        }
    }

    fn connection_mut(&mut self) -> &mut Connection<S> {
        unsafe {
            let mut conn = ptr::null();
            let ret = SSLGetConnection(self.ctx.0, &mut conn);
            assert!(ret == errSecSuccess);

            &mut *(conn as *mut Connection<S>)
        }
    }

    #[cold]
    fn check_panic(&mut self) {
        let conn = self.connection_mut();
        if let Some(err) = conn.panic.take() {
            panic::resume_unwind(err);
        }
    }

    #[cold]
    fn get_error(&mut self, ret: OSStatus) -> io::Error {
        self.check_panic();

        if let Some(err) = self.connection_mut().err.take() {
            err
        } else {
            io::Error::new(io::ErrorKind::Other, Error::from_code(ret))
        }
    }
}

impl<S: Read + Write> Read for SslStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Below we base our return value off the amount of data read, so a
        // zero-length buffer might cause us to erroneously interpret this
        // request as an error. Instead short-circuit that logic and return
        // `Ok(0)` instead.
        if buf.is_empty() {
            return Ok(0);
        }

        // If some data was buffered but not enough to fill `buf`, SSLRead
        // will try to read a new packet. This is bad because there may be
        // no more data but the socket is remaining open (e.g HTTPS with
        // Connection: keep-alive).
        let buffered = self.context().buffered_read_size().unwrap_or(0);
        let to_read = if buffered > 0 {
            cmp::min(buffered, buf.len())
        } else {
            buf.len()
        };

        unsafe {
            let mut nread = 0;
            let ret = SSLRead(self.ctx.0, buf.as_mut_ptr().cast(), to_read, &mut nread);
            // SSLRead can return an error at the same time it returns the last
            // chunk of data (!)
            if nread > 0 {
                return Ok(nread);
            }

            match ret {
                errSSLClosedGraceful | errSSLClosedAbort | errSSLClosedNoNotify => Ok(0),
                // this error isn't fatal
                errSSLPeerAuthCompleted => self.read(buf),
                _ => Err(self.get_error(ret)),
            }
        }
    }
}

impl<S: Read + Write> Write for SslStream<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Like above in read, short circuit a 0-length write
        if buf.is_empty() {
            return Ok(0);
        }
        unsafe {
            let mut nwritten = 0;
            let ret = SSLWrite(
                self.ctx.0,
                buf.as_ptr().cast(),
                buf.len(),
                &mut nwritten,
            );
            // just to be safe, base success off of nwritten rather than ret
            // for the same reason as in read
            if nwritten > 0 {
                Ok(nwritten)
            } else {
                Err(self.get_error(ret))
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.connection_mut().stream.flush()
    }
}

/// A builder type to simplify the creation of client side `SslStream`s.
#[derive(Debug)]
pub struct ClientBuilder {
    identity: Option<SecIdentity>,
    certs: Vec<SecCertificate>,
    chain: Vec<SecCertificate>,
    protocol_min: Option<SslProtocol>,
    protocol_max: Option<SslProtocol>,
    trust_certs_only: bool,
    use_sni: bool,
    danger_accept_invalid_certs: bool,
    danger_accept_invalid_hostnames: bool,
    whitelisted_ciphers: Vec<CipherSuite>,
    blacklisted_ciphers: Vec<CipherSuite>,
    #[cfg(feature = "alpn")]
    alpn: Option<Vec<String>>,
    #[cfg(feature = "session-tickets")]
    enable_session_tickets: bool,
}

impl Default for ClientBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl ClientBuilder {
    /// Creates a new builder with default options.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            identity: None,
            certs: Vec::new(),
            chain: Vec::new(),
            protocol_min: None,
            protocol_max: None,
            trust_certs_only: false,
            use_sni: true,
            danger_accept_invalid_certs: false,
            danger_accept_invalid_hostnames: false,
            whitelisted_ciphers: Vec::new(),
            blacklisted_ciphers: Vec::new(),
            #[cfg(feature = "alpn")]
            alpn: None,
            #[cfg(feature = "session-tickets")]
            enable_session_tickets: false,
        }
    }

    /// Specifies the set of root certificates to trust when
    /// verifying the server's certificate.
    #[inline]
    pub fn anchor_certificates(&mut self, certs: &[SecCertificate]) -> &mut Self {
        self.certs = certs.to_owned();
        self
    }

    /// Add the certificate the set of root certificates to trust
    /// when verifying the server's certificate.
    #[inline]
    pub fn add_anchor_certificate(&mut self, certs: &SecCertificate) -> &mut Self {
        self.certs.push(certs.to_owned());
        self
    }

    /// Specifies whether to trust the built-in certificates in addition
    /// to specified anchor certificates.
    #[inline(always)]
    pub fn trust_anchor_certificates_only(&mut self, only: bool) -> &mut Self {
        self.trust_certs_only = only;
        self
    }

    /// Specifies whether to trust invalid certificates.
    ///
    /// # Warning
    ///
    /// You should think very carefully before using this method. If invalid
    /// certificates are trusted, *any* certificate for *any* site will be
    /// trusted for use. This includes expired certificates. This introduces
    /// significant vulnerabilities, and should only be used as a last resort.
    #[inline(always)]
    pub fn danger_accept_invalid_certs(&mut self, noverify: bool) -> &mut Self {
        self.danger_accept_invalid_certs = noverify;
        self
    }

    /// Specifies whether to use Server Name Indication (SNI).
    #[inline(always)]
    pub fn use_sni(&mut self, use_sni: bool) -> &mut Self {
        self.use_sni = use_sni;
        self
    }

    /// Specifies whether to verify that the server's hostname matches its certificate.
    ///
    /// # Warning
    ///
    /// You should think very carefully before using this method. If hostnames are not verified,
    /// *any* valid certificate for *any* site will be trusted for use. This introduces significant
    /// vulnerabilities, and should only be used as a last resort.
    #[inline(always)]
    pub fn danger_accept_invalid_hostnames(
        &mut self,
        danger_accept_invalid_hostnames: bool,
    ) -> &mut Self {
        self.danger_accept_invalid_hostnames = danger_accept_invalid_hostnames;
        self
    }

    /// Set a whitelist of enabled ciphers. Any ciphers not whitelisted will be disabled.
    pub fn whitelist_ciphers(&mut self, whitelisted_ciphers: &[CipherSuite]) -> &mut Self {
        self.whitelisted_ciphers = whitelisted_ciphers.to_owned();
        self
    }

    /// Set a blacklist of disabled ciphers. Blacklisted ciphers will be disabled.
    pub fn blacklist_ciphers(&mut self, blacklisted_ciphers: &[CipherSuite]) -> &mut Self {
        self.blacklisted_ciphers = blacklisted_ciphers.to_owned();
        self
    }

    /// Use the specified identity as a SSL/TLS client certificate.
    pub fn identity(&mut self, identity: &SecIdentity, chain: &[SecCertificate]) -> &mut Self {
        self.identity = Some(identity.clone());
        self.chain = chain.to_owned();
        self
    }

    /// Configure the minimum protocol that this client will support.
    #[inline(always)]
    pub fn protocol_min(&mut self, min: SslProtocol) -> &mut Self {
        self.protocol_min = Some(min);
        self
    }

    /// Configure the minimum protocol that this client will support.
    #[inline(always)]
    pub fn protocol_max(&mut self, max: SslProtocol) -> &mut Self {
        self.protocol_max = Some(max);
        self
    }

    /// Configures the set of protocols used for ALPN.
    #[cfg(feature = "alpn")]
    pub fn alpn_protocols(&mut self, protocols: &[&str]) -> &mut Self {
        self.alpn = Some(protocols.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Configures the use of the RFC 5077 `SessionTicket` extension.
    ///
    /// Defaults to `false`.
    #[cfg(feature = "session-tickets")]
    #[inline(always)]
    pub fn enable_session_tickets(&mut self, enable: bool) -> &mut Self {
        self.enable_session_tickets = enable;
        self
    }

    /// Initiates a new SSL/TLS session over a stream connected to the specified domain.
    ///
    /// If both SNI and hostname verification are disabled, the value of `domain` will be ignored.
    pub fn handshake<S>(
        &self,
        domain: &str,
        stream: S,
    ) -> result::Result<SslStream<S>, ClientHandshakeError<S>>
    where
        S: Read + Write,
    {
        // the logic for trust validation is in MidHandshakeClientBuilder::connect, so run all
        // of the handshake logic through that.
        let stream = MidHandshakeSslStream {
            stream: self.ctx_into_stream(domain, stream)?,
            error: Error::from(errSecSuccess),
        };

        let certs = self.certs.clone();
        let stream = MidHandshakeClientBuilder {
            stream,
            domain: if self.danger_accept_invalid_hostnames {
                None
            } else {
                Some(domain.to_string())
            },
            certs,
            trust_certs_only: self.trust_certs_only,
            danger_accept_invalid_certs: self.danger_accept_invalid_certs,
        };
        stream.handshake()
    }

    fn ctx_into_stream<S>(&self, domain: &str, stream: S) -> Result<SslStream<S>>
    where
        S: Read + Write,
    {
        let mut ctx = SslContext::new(SslProtocolSide::CLIENT, SslConnectionType::STREAM)?;

        if self.use_sni {
            ctx.set_peer_domain_name(domain)?;
        }
        if let Some(ref identity) = self.identity {
            ctx.set_certificate(identity, &self.chain)?;
        }
        #[cfg(feature = "alpn")]
        {
            if let Some(ref alpn) = self.alpn {
                ctx.set_alpn_protocols(&alpn.iter().map(|s| &**s).collect::<Vec<_>>())?;
            }
        }
        #[cfg(feature = "session-tickets")]
        {
            if self.enable_session_tickets {
                // We must use the domain here to ensure that we go through certificate validation
                // again rather than resuming the session if the domain changes.
                ctx.set_peer_id(domain.as_bytes())?;
                ctx.set_session_tickets_enabled(true)?;
            }
        }
        ctx.set_break_on_server_auth(true)?;
        self.configure_protocols(&mut ctx)?;
        self.configure_ciphers(&mut ctx)?;

        ctx.into_stream(stream)
    }

    fn configure_protocols(&self, ctx: &mut SslContext) -> Result<()> {
        if let Some(min) = self.protocol_min {
            ctx.set_protocol_version_min(min)?;
        }
        if let Some(max) = self.protocol_max {
            ctx.set_protocol_version_max(max)?;
        }
        Ok(())
    }

    fn configure_ciphers(&self, ctx: &mut SslContext) -> Result<()> {
        let mut ciphers = if self.whitelisted_ciphers.is_empty() {
            ctx.enabled_ciphers()?
        } else {
            self.whitelisted_ciphers.clone()
        };

        if !self.blacklisted_ciphers.is_empty() {
            ciphers.retain(|cipher| !self.blacklisted_ciphers.contains(cipher));
        }

        ctx.set_enabled_ciphers(&ciphers)?;
        Ok(())
    }
}

/// A builder type to simplify the creation of server-side `SslStream`s.
#[derive(Debug)]
pub struct ServerBuilder {
    identity: SecIdentity,
    certs: Vec<SecCertificate>,
}

impl ServerBuilder {
    /// Creates a new `ServerBuilder` which will use the specified identity
    /// and certificate chain for handshakes.
    #[must_use]
    pub fn new(identity: &SecIdentity, certs: &[SecCertificate]) -> Self {
        Self {
            identity: identity.clone(),
            certs: certs.to_owned(),
        }
    }

    /// Creates a new `ServerBuilder` which will use the identity
    /// from the given PKCS #12 data.
    ///
    /// This operation fails if PKCS #12 file contains zero or more than one identity.
    ///
    /// This is a shortcut for the most common operation.
    pub fn from_pkcs12(pkcs12_der: &[u8], passphrase: &str) -> Result<Self> {
        let mut identities: Vec<(SecIdentity, Vec<SecCertificate>)> = Pkcs12ImportOptions::new()
            .passphrase(passphrase)
            .import(pkcs12_der)?
            .into_iter()
            .filter_map(|idendity| {
                let certs = idendity.cert_chain.unwrap_or_default();
                idendity.identity.map(|identity| (identity, certs))
            })
            .collect();
        if identities.len() == 1 {
            let (identity, certs) = identities.pop().unwrap();
            Ok(ServerBuilder::new(&identity, &certs))
        } else {
            // This error code is not really helpful
            Err(Error::from_code(errSecParam))
        }
    }

    /// Create a SSL context for lower-level stream initialization.
    pub fn new_ssl_context(&self) -> Result<SslContext> {
        let mut ctx = SslContext::new(SslProtocolSide::SERVER, SslConnectionType::STREAM)?;
        ctx.set_certificate(&self.identity, &self.certs)?;
        Ok(ctx)
    }

    /// Initiates a new SSL/TLS session over a stream.
    pub fn handshake<S>(&self, stream: S) -> Result<SslStream<S>>
    where
        S: Read + Write,
    {
        match self.new_ssl_context()?.handshake(stream) {
            Ok(stream) => Ok(stream),
            Err(HandshakeError::Interrupted(stream)) => Err(*stream.error()),
            Err(HandshakeError::Failure(err)) => Err(err),
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::prelude::*;
    use std::net::TcpStream;

    use super::*;

    #[test]
    fn server_builder_from_pkcs12() {
        let pkcs12_der = include_bytes!("../test/server.p12");
        ServerBuilder::from_pkcs12(pkcs12_der, "password123").unwrap();
    }

    #[test]
    fn connect() {
        let mut ctx = p!(SslContext::new(
            SslProtocolSide::CLIENT,
            SslConnectionType::STREAM
        ));
        p!(ctx.set_peer_domain_name("google.com"));
        let stream = p!(TcpStream::connect("google.com:443"));
        p!(ctx.handshake(stream));
    }

    #[test]
    fn connect_bad_domain() {
        let mut ctx = p!(SslContext::new(
            SslProtocolSide::CLIENT,
            SslConnectionType::STREAM
        ));
        p!(ctx.set_peer_domain_name("foobar.com"));
        let stream = p!(TcpStream::connect("google.com:443"));
        match ctx.handshake(stream) {
            Ok(_) => panic!("expected failure"),
            Err(_) => {}
        }
    }

    #[test]
    fn load_page() {
        let mut ctx = p!(SslContext::new(
            SslProtocolSide::CLIENT,
            SslConnectionType::STREAM
        ));
        p!(ctx.set_peer_domain_name("google.com"));
        let stream = p!(TcpStream::connect("google.com:443"));
        let mut stream = p!(ctx.handshake(stream));
        p!(stream.write_all(b"GET / HTTP/1.0\r\n\r\n"));
        p!(stream.flush());
        let mut buf = vec![];
        p!(stream.read_to_end(&mut buf));
        println!("{}", String::from_utf8_lossy(&buf));
    }

    #[test]
    fn client_no_session_ticket_resumption() {
        for _ in 0..2 {
            let stream = p!(TcpStream::connect("google.com:443"));

            // Manually handshake here.
            let stream = MidHandshakeSslStream {
                stream: ClientBuilder::new()
                    .ctx_into_stream("google.com", stream)
                    .unwrap(),
                error: Error::from(errSecSuccess),
            };

            let mut result = stream.handshake();

            if let Err(HandshakeError::Interrupted(stream)) = result {
                assert!(stream.server_auth_completed());
                result = stream.handshake();
            } else {
                panic!("Unexpectedly skipped server auth");
            }

            assert!(result.is_ok());
        }
    }

    #[test]
    #[cfg(feature = "session-tickets")]
    fn client_session_ticket_resumption() {
        // The first time through this loop, we should do a full handshake. The second time, we
        // should immediately finish the handshake without breaking on server auth.
        for i in 0..2 {
            let stream = p!(TcpStream::connect("google.com:443"));
            let mut builder = ClientBuilder::new();
            builder.enable_session_tickets(true);

            // Manually handshake here.
            let stream = MidHandshakeSslStream {
                stream: builder.ctx_into_stream("google.com", stream).unwrap(),
                error: Error::from(errSecSuccess),
            };

            let mut result = stream.handshake();

            if let Err(HandshakeError::Interrupted(stream)) = result {
                assert!(stream.server_auth_completed());
                assert_eq!(
                    i, 0,
                    "Session ticket resumption did not work, server auth was not skipped"
                );
                result = stream.handshake();
            } else {
                assert_eq!(i, 1, "Unexpectedly skipped server auth");
            }

            assert!(result.is_ok());
        }
    }

    #[test]
    #[cfg(feature = "alpn")]
    fn client_alpn_accept() {
        let mut ctx = p!(SslContext::new(
            SslProtocolSide::CLIENT,
            SslConnectionType::STREAM
        ));
        p!(ctx.set_peer_domain_name("google.com"));
        p!(ctx.set_alpn_protocols(&vec!["h2"]));
        let stream = p!(TcpStream::connect("google.com:443"));
        let stream = ctx.handshake(stream).unwrap();
        assert_eq!(vec!["h2"], stream.context().alpn_protocols().unwrap());
    }

    #[test]
    #[cfg(feature = "alpn")]
    fn client_alpn_reject() {
        let mut ctx = p!(SslContext::new(
            SslProtocolSide::CLIENT,
            SslConnectionType::STREAM
        ));
        p!(ctx.set_peer_domain_name("google.com"));
        p!(ctx.set_alpn_protocols(&vec!["h2c"]));
        let stream = p!(TcpStream::connect("google.com:443"));
        let stream = ctx.handshake(stream).unwrap();
        assert!(stream.context().alpn_protocols().is_err());
    }

    #[test]
    fn client_no_anchor_certs() {
        let stream = p!(TcpStream::connect("google.com:443"));
        assert!(ClientBuilder::new()
            .trust_anchor_certificates_only(true)
            .handshake("google.com", stream)
            .is_err());
    }

    #[test]
    fn client_bad_domain() {
        let stream = p!(TcpStream::connect("google.com:443"));
        assert!(ClientBuilder::new()
            .handshake("foobar.com", stream)
            .is_err());
    }

    #[test]
    fn client_bad_domain_ignored() {
        let stream = p!(TcpStream::connect("google.com:443"));
        ClientBuilder::new()
            .danger_accept_invalid_hostnames(true)
            .handshake("foobar.com", stream)
            .unwrap();
    }

    #[test]
    fn connect_no_verify_ssl() {
        let stream = p!(TcpStream::connect("expired.badssl.com:443"));
        let mut builder = ClientBuilder::new();
        builder.danger_accept_invalid_certs(true);
        builder.handshake("expired.badssl.com", stream).unwrap();
    }

    #[test]
    fn load_page_client() {
        let stream = p!(TcpStream::connect("google.com:443"));
        let mut stream = p!(ClientBuilder::new().handshake("google.com", stream));
        p!(stream.write_all(b"GET / HTTP/1.0\r\n\r\n"));
        p!(stream.flush());
        let mut buf = vec![];
        p!(stream.read_to_end(&mut buf));
        println!("{}", String::from_utf8_lossy(&buf));
    }

    #[test]
    #[cfg_attr(any(target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"), ignore)] // FIXME what's going on with ios?
    fn cipher_configuration() {
        let mut ctx = p!(SslContext::new(
            SslProtocolSide::SERVER,
            SslConnectionType::STREAM
        ));
        let ciphers = p!(ctx.enabled_ciphers());
        let ciphers = ciphers
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if i % 2 == 0 { Some(*c) } else { None })
            .collect::<Vec<_>>();
        p!(ctx.set_enabled_ciphers(&ciphers));
        assert_eq!(ciphers, p!(ctx.enabled_ciphers()));
    }

    #[test]
    fn test_builder_whitelist_ciphers() {
        let stream = p!(TcpStream::connect("google.com:443"));

        let ctx = p!(SslContext::new(
            SslProtocolSide::CLIENT,
            SslConnectionType::STREAM
        ));
        assert!(p!(ctx.enabled_ciphers()).len() > 1);

        let ciphers = p!(ctx.enabled_ciphers());
        let cipher = ciphers.first().unwrap();
        let stream = p!(ClientBuilder::new()
            .whitelist_ciphers(&[*cipher])
            .ctx_into_stream("google.com", stream));

        assert_eq!(1, p!(stream.context().enabled_ciphers()).len());
    }

    #[test]
    #[cfg_attr(any(target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"), ignore)] // FIXME same issue as cipher_configuration
    fn test_builder_blacklist_ciphers() {
        let stream = p!(TcpStream::connect("google.com:443"));

        let ctx = p!(SslContext::new(
            SslProtocolSide::CLIENT,
            SslConnectionType::STREAM
        ));
        let num = p!(ctx.enabled_ciphers()).len();
        assert!(num > 1);

        let ciphers = p!(ctx.enabled_ciphers());
        let cipher = ciphers.first().unwrap();
        let stream = p!(ClientBuilder::new()
            .blacklist_ciphers(&[*cipher])
            .ctx_into_stream("google.com", stream));

        assert_eq!(num - 1, p!(stream.context().enabled_ciphers()).len());
    }

    #[test]
    fn idle_context_peer_trust() {
        let ctx = p!(SslContext::new(
            SslProtocolSide::SERVER,
            SslConnectionType::STREAM
        ));
        assert!(ctx.peer_trust2().is_err());
    }

    #[test]
    fn peer_id() {
        let mut ctx = p!(SslContext::new(
            SslProtocolSide::SERVER,
            SslConnectionType::STREAM
        ));
        assert!(p!(ctx.peer_id()).is_none());
        p!(ctx.set_peer_id(b"foobar"));
        assert_eq!(p!(ctx.peer_id()), Some(&b"foobar"[..]));
    }

    #[test]
    fn peer_domain_name() {
        let mut ctx = p!(SslContext::new(
            SslProtocolSide::CLIENT,
            SslConnectionType::STREAM
        ));
        assert_eq!("", p!(ctx.peer_domain_name()));
        p!(ctx.set_peer_domain_name("foobar.com"));
        assert_eq!("foobar.com", p!(ctx.peer_domain_name()));
    }

    #[test]
    #[should_panic(expected = "blammo")]
    fn write_panic() {
        struct ExplodingStream(TcpStream);

        impl Read for ExplodingStream {
            fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                self.0.read(buf)
            }
        }

        impl Write for ExplodingStream {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                panic!("blammo");
            }

            fn flush(&mut self) -> io::Result<()> {
                self.0.flush()
            }
        }

        let mut ctx = p!(SslContext::new(
            SslProtocolSide::CLIENT,
            SslConnectionType::STREAM
        ));
        p!(ctx.set_peer_domain_name("google.com"));
        let stream = p!(TcpStream::connect("google.com:443"));
        let _ = ctx.handshake(ExplodingStream(stream));
    }

    #[test]
    #[should_panic(expected = "blammo")]
    fn read_panic() {
        struct ExplodingStream(TcpStream);

        impl Read for ExplodingStream {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                panic!("blammo");
            }
        }

        impl Write for ExplodingStream {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.0.write(buf)
            }

            fn flush(&mut self) -> io::Result<()> {
                self.0.flush()
            }
        }

        let mut ctx = p!(SslContext::new(
            SslProtocolSide::CLIENT,
            SslConnectionType::STREAM
        ));
        p!(ctx.set_peer_domain_name("google.com"));
        let stream = p!(TcpStream::connect("google.com:443"));
        let _ = ctx.handshake(ExplodingStream(stream));
    }

    #[test]
    fn zero_length_buffers() {
        let mut ctx = p!(SslContext::new(
            SslProtocolSide::CLIENT,
            SslConnectionType::STREAM
        ));
        p!(ctx.set_peer_domain_name("google.com"));
        let stream = p!(TcpStream::connect("google.com:443"));
        let mut stream = ctx.handshake(stream).unwrap();
        assert_eq!(stream.write(b"").unwrap(), 0);
        assert_eq!(stream.read(&mut []).unwrap(), 0);
    }
}
