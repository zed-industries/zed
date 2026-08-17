use crate::builder::{ConfigBuilder, WantsCipherSuites};
use crate::common_state::{CommonState, Protocol, Side};
use crate::conn::{ConnectionCommon, ConnectionCore};
use crate::dns_name::{DnsName, DnsNameRef, InvalidDnsNameError};
use crate::enums::{CipherSuite, ProtocolVersion, SignatureScheme};
use crate::error::Error;
use crate::kx::SupportedKxGroup;
#[cfg(feature = "logging")]
use crate::log::trace;
use crate::msgs::enums::NamedGroup;
use crate::msgs::handshake::ClientExtension;
use crate::msgs::persist;
use crate::sign;
use crate::suites::SupportedCipherSuite;
use crate::verify;
use crate::versions;
#[cfg(feature = "secret_extraction")]
use crate::ExtractedSecrets;
use crate::KeyLog;

use super::handy::{ClientSessionMemoryCache, NoClientSessionStorage};
use super::hs;

use std::marker::PhantomData;
use std::net::IpAddr;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::{fmt, io, mem};

/// A trait for the ability to store client session data, so that sessions
/// can be resumed in future connections.
///
/// Generally all data in this interface should be treated as
/// **highly sensitive**, containing enough key material to break all security
/// of the corresponding session.
///
/// `set_`, `insert_`, `remove_` and `take_` operations are mutating; this isn't
/// expressed in the type system to allow implementations freedom in
/// how to achieve interior mutability.  `Mutex` is a common choice.
pub trait ClientSessionStore: Send + Sync {
    /// Remember what `NamedGroup` the given server chose.
    fn set_kx_hint(&self, server_name: &ServerName, group: NamedGroup);

    /// This should return the value most recently passed to `set_kx_hint`
    /// for the given `server_name`.
    ///
    /// If `None` is returned, the caller chooses the first configured group,
    /// and an extra round trip might happen if that choice is unsatisfactory
    /// to the server.
    fn kx_hint(&self, server_name: &ServerName) -> Option<NamedGroup>;

    /// Remember a TLS1.2 session.
    ///
    /// At most one of these can be remembered at a time, per `server_name`.
    fn set_tls12_session(&self, server_name: &ServerName, value: persist::Tls12ClientSessionValue);

    /// Get the most recently saved TLS1.2 session for `server_name` provided to `set_tls12_session`.
    fn tls12_session(&self, server_name: &ServerName) -> Option<persist::Tls12ClientSessionValue>;

    /// Remove and forget any saved TLS1.2 session for `server_name`.
    fn remove_tls12_session(&self, server_name: &ServerName);

    /// Remember a TLS1.3 ticket that might be retrieved later from `take_tls13_ticket`, allowing
    /// resumption of this session.
    ///
    /// This can be called multiple times for a given session, allowing multiple independent tickets
    /// to be valid at once.  The number of times this is called is controlled by the server, so
    /// implementations of this trait should apply a reasonable bound of how many items are stored
    /// simultaneously.
    fn insert_tls13_ticket(
        &self,
        server_name: &ServerName,
        value: persist::Tls13ClientSessionValue,
    );

    /// Return a TLS1.3 ticket previously provided to `add_tls13_ticket`.
    ///
    /// Implementations of this trait must return each value provided to `add_tls13_ticket` _at most once_.
    fn take_tls13_ticket(
        &self,
        server_name: &ServerName,
    ) -> Option<persist::Tls13ClientSessionValue>;
}

/// A trait for the ability to choose a certificate chain and
/// private key for the purposes of client authentication.
pub trait ResolvesClientCert: Send + Sync {
    /// With the server-supplied acceptable issuers in `acceptable_issuers`,
    /// the server's supported signature schemes in `sigschemes`,
    /// return a certificate chain and signing key to authenticate.
    ///
    /// `acceptable_issuers` is undecoded and unverified by the rustls
    /// library, but it should be expected to contain a DER encodings
    /// of X501 NAMEs.
    ///
    /// Return None to continue the handshake without any client
    /// authentication.  The server may reject the handshake later
    /// if it requires authentication.
    fn resolve(
        &self,
        acceptable_issuers: &[&[u8]],
        sigschemes: &[SignatureScheme],
    ) -> Option<Arc<sign::CertifiedKey>>;

    /// Return true if any certificates at all are available.
    fn has_certs(&self) -> bool;
}

/// Common configuration for (typically) all connections made by a program.
///
/// Making one of these is cheap, though one of the inputs may be expensive: gathering trust roots
/// from the operating system to add to the [`RootCertStore`] passed to `with_root_certificates()`
/// (the rustls-native-certs crate is often used for this) may take on the order of a few hundred
/// milliseconds.
///
/// These must be created via the [`ClientConfig::builder()`] function.
///
/// # Defaults
///
/// * [`ClientConfig::max_fragment_size`]: the default is `None`: TLS packets are not fragmented to a specific size.
/// * [`ClientConfig::resumption`]: supports resumption with up to 256 server names, using session
///    ids or tickets, with a max of eight tickets per server.
/// * [`ClientConfig::alpn_protocols`]: the default is empty -- no ALPN protocol is negotiated.
/// * [`ClientConfig::key_log`]: key material is not logged.
///
/// [`RootCertStore`]: crate::RootCertStore
#[derive(Clone)]
pub struct ClientConfig {
    /// List of ciphersuites, in preference order.
    pub(super) cipher_suites: Vec<SupportedCipherSuite>,

    /// List of supported key exchange algorithms, in preference order -- the
    /// first element is the highest priority.
    ///
    /// The first element in this list is the _default key share algorithm_,
    /// and in TLS1.3 a key share for it is sent in the client hello.
    pub(super) kx_groups: Vec<&'static SupportedKxGroup>,

    /// Which ALPN protocols we include in our client hello.
    /// If empty, no ALPN extension is sent.
    pub alpn_protocols: Vec<Vec<u8>>,

    /// How and when the client can resume a previous session.
    pub resumption: Resumption,

    /// The maximum size of TLS message we'll emit.  If None, we don't limit TLS
    /// message lengths except to the 2**16 limit specified in the standard.
    ///
    /// rustls enforces an arbitrary minimum of 32 bytes for this field.
    /// Out of range values are reported as errors from ClientConnection::new.
    ///
    /// Setting this value to the TCP MSS may improve latency for stream-y workloads.
    pub max_fragment_size: Option<usize>,

    /// How to decide what client auth certificate/keys to use.
    pub client_auth_cert_resolver: Arc<dyn ResolvesClientCert>,

    /// Supported versions, in no particular order.  The default
    /// is all supported versions.
    pub(super) versions: versions::EnabledVersions,

    /// Whether to send the Server Name Indication (SNI) extension
    /// during the client handshake.
    ///
    /// The default is true.
    pub enable_sni: bool,

    /// How to verify the server certificate chain.
    pub(super) verifier: Arc<dyn verify::ServerCertVerifier>,

    /// How to output key material for debugging.  The default
    /// does nothing.
    pub key_log: Arc<dyn KeyLog>,

    /// Allows traffic secrets to be extracted after the handshake,
    /// e.g. for kTLS setup.
    #[cfg(feature = "secret_extraction")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secret_extraction")))]
    pub enable_secret_extraction: bool,

    /// Whether to send data on the first flight ("early data") in
    /// TLS 1.3 handshakes.
    ///
    /// The default is false.
    pub enable_early_data: bool,
}

/// What mechanisms to support for resuming a TLS 1.2 session.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tls12Resumption {
    /// Disable 1.2 resumption.
    Disabled,
    /// Support 1.2 resumption using session ids only.
    SessionIdOnly,
    /// Support 1.2 resumption using session ids or RFC 5077 tickets.
    ///
    /// See[^1] for why you might like to disable RFC 5077 by instead choosing the `SessionIdOnly`
    /// option. Note that TLS 1.3 tickets do not have those issues.
    ///
    /// [^1]: <https://words.filippo.io/we-need-to-talk-about-session-tickets/>
    SessionIdOrTickets,
}

impl fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientConfig")
            .field("alpn_protocols", &self.alpn_protocols)
            .field("resumption", &self.resumption)
            .field("max_fragment_size", &self.max_fragment_size)
            .field("enable_sni", &self.enable_sni)
            .field("enable_early_data", &self.enable_early_data)
            .finish_non_exhaustive()
    }
}

impl ClientConfig {
    /// Create a builder to build up the client configuration.
    ///
    /// For more information, see the [`ConfigBuilder`] documentation.
    pub fn builder() -> ConfigBuilder<Self, WantsCipherSuites> {
        ConfigBuilder {
            state: WantsCipherSuites(()),
            side: PhantomData,
        }
    }

    /// We support a given TLS version if it's quoted in the configured
    /// versions *and* at least one ciphersuite for this version is
    /// also configured.
    pub(crate) fn supports_version(&self, v: ProtocolVersion) -> bool {
        self.versions.contains(v)
            && self
                .cipher_suites
                .iter()
                .any(|cs| cs.version().version == v)
    }

    /// Access configuration options whose use is dangerous and requires
    /// extra care.
    #[cfg(feature = "dangerous_configuration")]
    pub fn dangerous(&mut self) -> danger::DangerousClientConfig {
        danger::DangerousClientConfig { cfg: self }
    }

    pub(super) fn find_cipher_suite(&self, suite: CipherSuite) -> Option<SupportedCipherSuite> {
        self.cipher_suites
            .iter()
            .copied()
            .find(|&scs| scs.suite() == suite)
    }
}

/// Configuration for how/when a client is allowed to resume a previous session.
#[derive(Clone)]
pub struct Resumption {
    /// How we store session data or tickets. The default is to use an in-memory
    /// [ClientSessionMemoryCache].
    pub(super) store: Arc<dyn ClientSessionStore>,

    /// What mechanism is used for resuming a TLS 1.2 session.
    pub(super) tls12_resumption: Tls12Resumption,
}

impl Resumption {
    /// Create a new `Resumption` that stores data for the given number of sessions in memory.
    ///
    /// This is the default `Resumption` choice, and enables resuming a TLS 1.2 session with
    /// a session id or RFC 5077 ticket.
    pub fn in_memory_sessions(num: usize) -> Self {
        Self {
            store: Arc::new(ClientSessionMemoryCache::new(num)),
            tls12_resumption: Tls12Resumption::SessionIdOrTickets,
        }
    }

    /// Use a custom [`ClientSessionStore`] implementation to store sessions.
    ///
    /// By default, enables resuming a TLS 1.2 session with a session id or RFC 5077 ticket.
    pub fn store(store: Arc<dyn ClientSessionStore>) -> Self {
        Self {
            store,
            tls12_resumption: Tls12Resumption::SessionIdOrTickets,
        }
    }

    /// Disable all use of session resumption.
    pub fn disabled() -> Self {
        Self {
            store: Arc::new(NoClientSessionStorage),
            tls12_resumption: Tls12Resumption::Disabled,
        }
    }

    /// Configure whether TLS 1.2 sessions may be resumed, and by what mechanism.
    ///
    /// This is meaningless if you've disabled resumption entirely.
    pub fn tls12_resumption(mut self, tls12: Tls12Resumption) -> Self {
        self.tls12_resumption = tls12;
        self
    }
}

impl fmt::Debug for Resumption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Resumption")
            .field("tls12_resumption", &self.tls12_resumption)
            .finish()
    }
}

impl Default for Resumption {
    /// Create an in-memory session store resumption with up to 256 server names, allowing
    /// a TLS 1.2 session to resume with a session id or RFC 5077 ticket.
    fn default() -> Self {
        Self::in_memory_sessions(256)
    }
}

/// Encodes ways a client can know the expected name of the server.
///
/// This currently covers knowing the DNS name of the server, but
/// will be extended in the future to supporting privacy-preserving names
/// for the server ("ECH").  For this reason this enum is `non_exhaustive`.
///
/// # Making one
///
/// If you have a DNS name as a `&str`, this type implements `TryFrom<&str>`,
/// so you can do:
///
/// ```
/// # use rustls::ServerName;
/// ServerName::try_from("example.com").expect("invalid DNS name");
///
/// // or, alternatively...
///
/// let x = "example.com".try_into().expect("invalid DNS name");
/// # let _: ServerName = x;
/// ```
#[non_exhaustive]
#[derive(Clone, Eq, Hash, PartialEq)]
pub enum ServerName {
    /// The server is identified by a DNS name.  The name
    /// is sent in the TLS Server Name Indication (SNI)
    /// extension.
    DnsName(DnsName),

    /// The server is identified by an IP address. SNI is not
    /// done.
    IpAddress(IpAddr),
}

impl fmt::Debug for ServerName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DnsName(d) => f
                .debug_tuple("DnsName")
                .field(&d.as_ref())
                .finish(),
            Self::IpAddress(i) => f
                .debug_tuple("IpAddress")
                .field(i)
                .finish(),
        }
    }
}

impl ServerName {
    /// Return the name that should go in the SNI extension.
    /// If [`None`] is returned, the SNI extension is not included
    /// in the handshake.
    pub(crate) fn for_sni(&self) -> Option<DnsNameRef> {
        match self {
            Self::DnsName(dns_name) => Some(dns_name.borrow()),
            Self::IpAddress(_) => None,
        }
    }
}

/// Attempt to make a ServerName from a string by parsing
/// it as a DNS name.
impl TryFrom<&str> for ServerName {
    type Error = InvalidDnsNameError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match DnsNameRef::try_from(s) {
            Ok(dns) => Ok(Self::DnsName(dns.to_owned())),
            Err(InvalidDnsNameError) => match s.parse() {
                Ok(ip) => Ok(Self::IpAddress(ip)),
                Err(_) => Err(InvalidDnsNameError),
            },
        }
    }
}

/// Container for unsafe APIs
#[cfg(feature = "dangerous_configuration")]
pub(super) mod danger {
    use std::sync::Arc;

    use super::verify::ServerCertVerifier;
    use super::ClientConfig;

    /// Accessor for dangerous configuration options.
    #[derive(Debug)]
    #[cfg_attr(docsrs, doc(cfg(feature = "dangerous_configuration")))]
    pub struct DangerousClientConfig<'a> {
        /// The underlying ClientConfig
        pub cfg: &'a mut ClientConfig,
    }

    impl<'a> DangerousClientConfig<'a> {
        /// Overrides the default `ServerCertVerifier` with something else.
        pub fn set_certificate_verifier(&mut self, verifier: Arc<dyn ServerCertVerifier>) {
            self.cfg.verifier = verifier;
        }
    }
}

#[derive(Debug, PartialEq)]
enum EarlyDataState {
    Disabled,
    Ready,
    Accepted,
    AcceptedFinished,
    Rejected,
}

pub(super) struct EarlyData {
    state: EarlyDataState,
    left: usize,
}

impl EarlyData {
    fn new() -> Self {
        Self {
            left: 0,
            state: EarlyDataState::Disabled,
        }
    }

    pub(super) fn is_enabled(&self) -> bool {
        matches!(self.state, EarlyDataState::Ready | EarlyDataState::Accepted)
    }

    fn is_accepted(&self) -> bool {
        matches!(
            self.state,
            EarlyDataState::Accepted | EarlyDataState::AcceptedFinished
        )
    }

    pub(super) fn enable(&mut self, max_data: usize) {
        assert_eq!(self.state, EarlyDataState::Disabled);
        self.state = EarlyDataState::Ready;
        self.left = max_data;
    }

    pub(super) fn rejected(&mut self) {
        trace!("EarlyData rejected");
        self.state = EarlyDataState::Rejected;
    }

    pub(super) fn accepted(&mut self) {
        trace!("EarlyData accepted");
        assert_eq!(self.state, EarlyDataState::Ready);
        self.state = EarlyDataState::Accepted;
    }

    pub(super) fn finished(&mut self) {
        trace!("EarlyData finished");
        self.state = match self.state {
            EarlyDataState::Accepted => EarlyDataState::AcceptedFinished,
            _ => panic!("bad EarlyData state"),
        }
    }

    fn check_write(&mut self, sz: usize) -> io::Result<usize> {
        match self.state {
            EarlyDataState::Disabled => unreachable!(),
            EarlyDataState::Ready | EarlyDataState::Accepted => {
                let take = if self.left < sz {
                    mem::replace(&mut self.left, 0)
                } else {
                    self.left -= sz;
                    sz
                };

                Ok(take)
            }
            EarlyDataState::Rejected | EarlyDataState::AcceptedFinished => {
                Err(io::Error::from(io::ErrorKind::InvalidInput))
            }
        }
    }

    fn bytes_left(&self) -> usize {
        self.left
    }
}

/// Stub that implements io::Write and dispatches to `write_early_data`.
pub struct WriteEarlyData<'a> {
    sess: &'a mut ClientConnection,
}

impl<'a> WriteEarlyData<'a> {
    fn new(sess: &'a mut ClientConnection) -> Self {
        WriteEarlyData { sess }
    }

    /// How many bytes you may send.  Writes will become short
    /// once this reaches zero.
    pub fn bytes_left(&self) -> usize {
        self.sess
            .inner
            .core
            .data
            .early_data
            .bytes_left()
    }
}

impl<'a> io::Write for WriteEarlyData<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.sess.write_early_data(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// This represents a single TLS client connection.
pub struct ClientConnection {
    inner: ConnectionCommon<ClientConnectionData>,
}

impl fmt::Debug for ClientConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ClientConnection")
            .finish()
    }
}

impl ClientConnection {
    /// Make a new ClientConnection.  `config` controls how
    /// we behave in the TLS protocol, `name` is the
    /// name of the server we want to talk to.
    pub fn new(config: Arc<ClientConfig>, name: ServerName) -> Result<Self, Error> {
        Ok(Self {
            inner: ConnectionCore::for_client(config, name, Vec::new(), Protocol::Tcp)?.into(),
        })
    }

    /// Returns an `io::Write` implementer you can write bytes to
    /// to send TLS1.3 early data (a.k.a. "0-RTT data") to the server.
    ///
    /// This returns None in many circumstances when the capability to
    /// send early data is not available, including but not limited to:
    ///
    /// - The server hasn't been talked to previously.
    /// - The server does not support resumption.
    /// - The server does not support early data.
    /// - The resumption data for the server has expired.
    ///
    /// The server specifies a maximum amount of early data.  You can
    /// learn this limit through the returned object, and writes through
    /// it will process only this many bytes.
    ///
    /// The server can choose not to accept any sent early data --
    /// in this case the data is lost but the connection continues.  You
    /// can tell this happened using `is_early_data_accepted`.
    pub fn early_data(&mut self) -> Option<WriteEarlyData> {
        if self
            .inner
            .core
            .data
            .early_data
            .is_enabled()
        {
            Some(WriteEarlyData::new(self))
        } else {
            None
        }
    }

    /// Returns True if the server signalled it will process early data.
    ///
    /// If you sent early data and this returns false at the end of the
    /// handshake then the server will not process the data.  This
    /// is not an error, but you may wish to resend the data.
    pub fn is_early_data_accepted(&self) -> bool {
        self.inner.core.is_early_data_accepted()
    }

    fn write_early_data(&mut self, data: &[u8]) -> io::Result<usize> {
        self.inner
            .core
            .data
            .early_data
            .check_write(data.len())
            .map(|sz| {
                self.inner
                    .send_early_plaintext(&data[..sz])
            })
    }

    /// Extract secrets, so they can be used when configuring kTLS, for example.
    #[cfg(feature = "secret_extraction")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secret_extraction")))]
    pub fn extract_secrets(self) -> Result<ExtractedSecrets, Error> {
        self.inner.extract_secrets()
    }
}

impl Deref for ClientConnection {
    type Target = ConnectionCommon<ClientConnectionData>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ClientConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[doc(hidden)]
impl<'a> TryFrom<&'a mut crate::Connection> for &'a mut ClientConnection {
    type Error = ();

    fn try_from(value: &'a mut crate::Connection) -> Result<Self, Self::Error> {
        use crate::Connection::*;
        match value {
            Client(conn) => Ok(conn),
            Server(_) => Err(()),
        }
    }
}

impl From<ClientConnection> for crate::Connection {
    fn from(conn: ClientConnection) -> Self {
        Self::Client(conn)
    }
}

impl ConnectionCore<ClientConnectionData> {
    pub(crate) fn for_client(
        config: Arc<ClientConfig>,
        name: ServerName,
        extra_exts: Vec<ClientExtension>,
        proto: Protocol,
    ) -> Result<Self, Error> {
        let mut common_state = CommonState::new(Side::Client);
        common_state.set_max_fragment_size(config.max_fragment_size)?;
        common_state.protocol = proto;
        #[cfg(feature = "secret_extraction")]
        {
            common_state.enable_secret_extraction = config.enable_secret_extraction;
        }
        let mut data = ClientConnectionData::new();

        let mut cx = hs::ClientContext {
            common: &mut common_state,
            data: &mut data,
        };

        let state = hs::start_handshake(name, extra_exts, config, &mut cx)?;
        Ok(Self::new(state, data, common_state))
    }

    pub(crate) fn is_early_data_accepted(&self) -> bool {
        self.data.early_data.is_accepted()
    }
}

/// State associated with a client connection.
pub struct ClientConnectionData {
    pub(super) early_data: EarlyData,
    pub(super) resumption_ciphersuite: Option<SupportedCipherSuite>,
}

impl ClientConnectionData {
    fn new() -> Self {
        Self {
            early_data: EarlyData::new(),
            resumption_ciphersuite: None,
        }
    }
}

impl crate::conn::SideData for ClientConnectionData {}
