use openssl::error::ErrorStack;
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkcs12::Pkcs12;
use openssl::pkey::{PKey, Private};
use openssl::ssl::{
    self, MidHandshakeSslStream, SslAcceptor, SslConnector, SslContextBuilder, SslMethod,
    SslVerifyMode,
};
use openssl::x509::store::X509StoreBuilder;
use openssl::x509::{X509VerifyResult, X509};
use openssl_probe::ProbeResult;
use std::sync::LazyLock;
use std::{error, fmt, io};

use crate::{Protocol, TlsAcceptorBuilder, TlsConnectorBuilder};
use log::debug;

static PROBE_RESULT: LazyLock<ProbeResult> = LazyLock::new(openssl_probe::probe);

#[cfg(have_min_max_version)]
fn supported_protocols(
    min: Option<Protocol>,
    max: Option<Protocol>,
    ctx: &mut SslContextBuilder,
) -> Result<(), ErrorStack> {
    use openssl::ssl::SslVersion;

    fn cvt(p: Protocol) -> SslVersion {
        match p {
            Protocol::Sslv3 => SslVersion::SSL3,
            Protocol::Tlsv10 => SslVersion::TLS1,
            Protocol::Tlsv11 => SslVersion::TLS1_1,
            Protocol::Tlsv12 => SslVersion::TLS1_2,
            Protocol::Tlsv13 => SslVersion::TLS1_3,
        }
    }

    ctx.set_min_proto_version(min.map(cvt))?;
    ctx.set_max_proto_version(max.map(cvt))?;

    Ok(())
}

#[cfg(not(have_min_max_version))]
fn supported_protocols(
    min: Option<Protocol>,
    max: Option<Protocol>,
    ctx: &mut SslContextBuilder,
) -> Result<(), ErrorStack> {
    use openssl::ssl::SslOptions;

    let no_ssl_mask = SslOptions::NO_SSLV2
        | SslOptions::NO_SSLV3
        | SslOptions::NO_TLSV1
        | SslOptions::NO_TLSV1_1
        | SslOptions::NO_TLSV1_2;

    ctx.clear_options(no_ssl_mask);
    let mut options = SslOptions::empty();
    options |= match min {
        None => SslOptions::empty(),
        Some(Protocol::Sslv3) => SslOptions::NO_SSLV2,
        Some(Protocol::Tlsv10) => SslOptions::NO_SSLV2 | SslOptions::NO_SSLV3,
        Some(Protocol::Tlsv11) => {
            SslOptions::NO_SSLV2 | SslOptions::NO_SSLV3 | SslOptions::NO_TLSV1
        }
        Some(Protocol::Tlsv12) => {
            SslOptions::NO_SSLV2
                | SslOptions::NO_SSLV3
                | SslOptions::NO_TLSV1
                | SslOptions::NO_TLSV1_1
        }
        Some(Protocol::Tlsv13) => {
            SslOptions::NO_SSLV2
                | SslOptions::NO_SSLV3
                | SslOptions::NO_TLSV1
                | SslOptions::NO_TLSV1_1
                | SslOptions::NO_TLSV1_2
        }
    };
    options |= match max {
        // NO_TLSV1_3 may be unavailalbe in the old versions
        None | Some(Protocol::Tlsv12 | Protocol::Tlsv13) => SslOptions::empty(),
        Some(Protocol::Tlsv11) => SslOptions::NO_TLSV1_2,
        Some(Protocol::Tlsv10) => SslOptions::NO_TLSV1_1 | SslOptions::NO_TLSV1_2,
        Some(Protocol::Sslv3) => {
            SslOptions::NO_TLSV1 | SslOptions::NO_TLSV1_1 | SslOptions::NO_TLSV1_2
        }
    };

    ctx.set_options(options);

    Ok(())
}

#[cfg(target_os = "android")]
fn load_android_root_certs(connector: &mut SslContextBuilder) -> Result<(), Error> {
    use std::fs;

    if let Ok(dir) = fs::read_dir("/system/etc/security/cacerts") {
        let certs = dir
            .filter_map(|r| r.ok())
            .filter_map(|e| fs::read(e.path()).ok())
            .filter_map(|b| X509::from_pem(&b).ok());
        for cert in certs {
            if let Err(err) = connector.cert_store_mut().add_cert(cert) {
                debug!("load_android_root_certs error: {:?}", err);
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Normal(ErrorStack),
    Ssl(ssl::Error, X509VerifyResult),
    EmptyChain,
    NotPkcs8,
    AlpnTooLong,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Normal(ref e) => error::Error::source(e),
            Error::Ssl(ref e, _) => error::Error::source(e),
            Error::EmptyChain => None,
            Error::NotPkcs8 => None,
            Error::AlpnTooLong => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Normal(ref e) => fmt::Display::fmt(e, fmt),
            Error::Ssl(ref e, X509VerifyResult::OK) => fmt::Display::fmt(e, fmt),
            Error::Ssl(ref e, v) => write!(fmt, "{} ({})", e, v),
            Error::EmptyChain => write!(
                fmt,
                "at least one certificate must be provided to create an identity"
            ),
            Error::NotPkcs8 => write!(fmt, "expected PKCS#8 PEM"),
            Error::AlpnTooLong => write!(fmt, "ALPN too long"),
        }
    }
}

impl From<ErrorStack> for Error {
    fn from(err: ErrorStack) -> Error {
        Error::Normal(err)
    }
}

#[derive(Clone)]
pub struct Identity {
    pkey: PKey<Private>,
    cert: X509,
    chain: Vec<X509>,
}

impl Identity {
    pub fn from_pkcs12(buf: &[u8], pass: &str) -> Result<Identity, Error> {
        let pkcs12 = Pkcs12::from_der(buf)?;
        let parsed = pkcs12.parse2(pass)?;
        Ok(Identity {
            pkey: parsed.pkey.ok_or_else(|| Error::EmptyChain)?,
            cert: parsed.cert.ok_or_else(|| Error::EmptyChain)?,
            // > The stack is the reverse of what you might expect due to the way
            // > PKCS12_parse is implemented, so we need to load it backwards.
            // > https://github.com/sfackler/rust-native-tls/commit/05fb5e583be589ab63d9f83d986d095639f8ec44
            chain: parsed.ca.into_iter().flatten().rev().collect(),
        })
    }

    pub fn from_pkcs8(buf: &[u8], key: &[u8]) -> Result<Identity, Error> {
        if !key.starts_with(b"-----BEGIN PRIVATE KEY-----") {
            return Err(Error::NotPkcs8);
        }

        let pkey = PKey::private_key_from_pem(key)?;
        let mut cert_chain = X509::stack_from_pem(buf)?.into_iter();
        let cert = cert_chain.next().ok_or(Error::EmptyChain)?;
        let chain = cert_chain.collect();
        Ok(Identity { pkey, cert, chain })
    }
}

#[derive(Clone)]
pub struct Certificate(X509);

impl Certificate {
    pub fn from_der(buf: &[u8]) -> Result<Certificate, Error> {
        let cert = X509::from_der(buf)?;
        Ok(Certificate(cert))
    }

    pub fn from_pem(buf: &[u8]) -> Result<Certificate, Error> {
        let cert = X509::from_pem(buf)?;
        Ok(Certificate(cert))
    }

    pub fn stack_from_pem(buf: &[u8]) -> Result<Vec<Certificate>, Error> {
        let certs = X509::stack_from_pem(buf)?;
        Ok(certs.into_iter().map(Certificate).collect())
    }

    pub fn to_der(&self) -> Result<Vec<u8>, Error> {
        let der = self.0.to_der()?;
        Ok(der)
    }
}

pub struct MidHandshakeTlsStream<S>(MidHandshakeSslStream<S>);

impl<S> fmt::Debug for MidHandshakeTlsStream<S>
where
    S: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}

impl<S> MidHandshakeTlsStream<S> {
    pub fn get_ref(&self) -> &S {
        self.0.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut S {
        self.0.get_mut()
    }
}

impl<S> MidHandshakeTlsStream<S>
where
    S: io::Read + io::Write,
{
    pub fn handshake(self) -> Result<TlsStream<S>, HandshakeError<S>> {
        match self.0.handshake() {
            Ok(s) => Ok(TlsStream(s)),
            Err(e) => Err(e.into()),
        }
    }
}

pub enum HandshakeError<S> {
    Failure(Error),
    WouldBlock(MidHandshakeTlsStream<S>),
}

impl<S> From<ssl::HandshakeError<S>> for HandshakeError<S> {
    fn from(e: ssl::HandshakeError<S>) -> HandshakeError<S> {
        match e {
            ssl::HandshakeError::SetupFailure(e) => HandshakeError::Failure(e.into()),
            ssl::HandshakeError::Failure(e) => {
                let v = e.ssl().verify_result();
                HandshakeError::Failure(Error::Ssl(e.into_error(), v))
            }
            ssl::HandshakeError::WouldBlock(s) => {
                HandshakeError::WouldBlock(MidHandshakeTlsStream(s))
            }
        }
    }
}

impl<S> From<ErrorStack> for HandshakeError<S> {
    fn from(e: ErrorStack) -> HandshakeError<S> {
        HandshakeError::Failure(e.into())
    }
}

#[derive(Clone)]
pub struct TlsConnector {
    connector: SslConnector,
    use_sni: bool,
    accept_invalid_hostnames: bool,
    accept_invalid_certs: bool,
}

impl TlsConnector {
    pub fn new(builder: &TlsConnectorBuilder) -> Result<TlsConnector, Error> {
        let mut connector = SslConnector::builder(SslMethod::tls())?;

        // We need to load these separately so an error on one doesn't prevent the other from loading.
        if let Some(cert_file) = &PROBE_RESULT.cert_file {
            if let Err(e) = connector.load_verify_locations(Some(cert_file), None) {
                debug!("load_verify_locations cert file error: {:?}", e);
            }
        }
        for cert_dir in &PROBE_RESULT.cert_dir {
            if let Err(e) = connector.load_verify_locations(None, Some(cert_dir)) {
                debug!("load_verify_locations cert dir error: {:?}", e);
            }
        }

        if let Some(ref identity) = builder.identity {
            connector.set_certificate(&identity.0.cert)?;
            connector.set_private_key(&identity.0.pkey)?;
            for cert in identity.0.chain.iter() {
                // https://www.openssl.org/docs/manmaster/man3/SSL_CTX_add_extra_chain_cert.html
                // specifies that "When sending a certificate chain, extra chain certificates are
                // sent in order following the end entity certificate."
                connector.add_extra_chain_cert(cert.to_owned())?;
            }
        }
        supported_protocols(builder.min_protocol, builder.max_protocol, &mut connector)?;

        if builder.disable_built_in_roots {
            connector.set_cert_store(X509StoreBuilder::new()?.build());
        }

        for cert in &builder.root_certificates {
            if let Err(err) = connector.cert_store_mut().add_cert((cert.0).0.clone()) {
                debug!("add_cert error: {:?}", err);
            }
        }

        #[cfg(feature = "alpn")]
        if !builder.alpn.is_empty() {
            connector.set_alpn_protos(&alpn_wire_format(&builder.alpn)?)?;
        }

        #[cfg(target_os = "android")]
        load_android_root_certs(&mut connector)?;

        Ok(TlsConnector {
            connector: connector.build(),
            use_sni: builder.use_sni,
            accept_invalid_hostnames: builder.accept_invalid_hostnames,
            accept_invalid_certs: builder.accept_invalid_certs,
        })
    }

    pub fn connect<S>(&self, domain: &str, stream: S) -> Result<TlsStream<S>, HandshakeError<S>>
    where
        S: io::Read + io::Write,
    {
        let mut ssl = self
            .connector
            .configure()?
            .use_server_name_indication(self.use_sni)
            .verify_hostname(!self.accept_invalid_hostnames);
        if self.accept_invalid_certs {
            ssl.set_verify(SslVerifyMode::NONE);
        }

        let s = ssl.connect(domain, stream)?;
        Ok(TlsStream(s))
    }
}

#[cfg(any(feature = "alpn", feature = "alpn-accept"))]
fn alpn_wire_format(alpn_list: &[Box<str>]) -> Result<Vec<u8>, Error> {
    // Wire format is each alpn preceded by its length as a byte.
    let mut alpn_wire_format =
        Vec::with_capacity(alpn_list.iter().map(|s| s.len()).sum::<usize>() + alpn_list.len());
    for alpn in alpn_list.iter().map(|s| s.as_bytes()) {
        let len_byte = alpn.len().try_into().map_err(|_| Error::AlpnTooLong)?;

        if alpn_wire_format.capacity() - alpn_wire_format.len() >= 1 {
            alpn_wire_format.push(len_byte);
        }
        if alpn_wire_format.capacity() - alpn_wire_format.len() >= alpn.len() {
            alpn_wire_format.extend(alpn);
        }
    }
    Ok(alpn_wire_format)
}

impl fmt::Debug for TlsConnector {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("TlsConnector")
            // n.b. SslConnector is a newtype on SslContext which implements a noop Debug so it's omitted
            .field("use_sni", &self.use_sni)
            .field("accept_invalid_hostnames", &self.accept_invalid_hostnames)
            .field("accept_invalid_certs", &self.accept_invalid_certs)
            .finish()
    }
}

#[derive(Clone)]
pub struct TlsAcceptor(SslAcceptor);

impl TlsAcceptor {
    pub fn new(builder: &TlsAcceptorBuilder) -> Result<TlsAcceptor, Error> {
        let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
        acceptor.set_private_key(&builder.identity.0.pkey)?;
        acceptor.set_certificate(&builder.identity.0.cert)?;
        #[cfg(feature = "alpn-accept")]
        if !builder.accept_alpn.is_empty() {
            let alpn_wire_format = alpn_wire_format(&builder.accept_alpn)?;
            acceptor.set_alpn_protos(&alpn_wire_format)?;
            // set up ALPN selection routine - as select_next_proto
            acceptor.set_alpn_select_callback(move |_: &mut openssl::ssl::SslRef, client_list: &[u8]| {
                openssl::ssl::select_next_proto(&alpn_wire_format, client_list).and_then(|selected| {
                    if selected.is_empty() || selected.len() > client_list.len() {
                        return None;
                    }
                    // return string from the client list to separate it from alpn_wire_format's lifetime
                    // https://github.com/rust-openssl/rust-openssl/pull/2360#issuecomment-2651522324
                    client_list.windows(selected.len()).find(|&item| item == selected)
                })
                .ok_or(openssl::ssl::AlpnError::NOACK)
            });
        }
        for cert in builder.identity.0.chain.iter() {
            // https://www.openssl.org/docs/manmaster/man3/SSL_CTX_add_extra_chain_cert.html
            // specifies that "When sending a certificate chain, extra chain certificates are
            // sent in order following the end entity certificate."
            acceptor.add_extra_chain_cert(cert.to_owned())?;
        }
        supported_protocols(builder.min_protocol, builder.max_protocol, &mut acceptor)?;

        Ok(TlsAcceptor(acceptor.build()))
    }

    pub fn accept<S>(&self, stream: S) -> Result<TlsStream<S>, HandshakeError<S>>
    where
        S: io::Read + io::Write,
    {
        let s = self.0.accept(stream)?;
        Ok(TlsStream(s))
    }
}

pub struct TlsStream<S>(ssl::SslStream<S>);

impl<S: fmt::Debug> fmt::Debug for TlsStream<S> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}

impl<S> TlsStream<S> {
    pub fn get_ref(&self) -> &S {
        self.0.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut S {
        self.0.get_mut()
    }
}

impl<S: io::Read + io::Write> TlsStream<S> {
    pub fn buffered_read_size(&self) -> Result<usize, Error> {
        Ok(self.0.ssl().pending())
    }

    pub fn peer_certificate(&self) -> Result<Option<Certificate>, Error> {
        Ok(self.0.ssl().peer_certificate().map(Certificate))
    }

    #[cfg(feature = "alpn")]
    pub fn negotiated_alpn(&self) -> Result<Option<Vec<u8>>, Error> {
        Ok(self
            .0
            .ssl()
            .selected_alpn_protocol()
            .map(|alpn| alpn.to_vec()))
    }

    pub fn tls_server_end_point(&self) -> Result<Option<Vec<u8>>, Error> {
        let cert = if self.0.ssl().is_server() {
            self.0.ssl().certificate().map(|x| x.to_owned())
        } else {
            self.0.ssl().peer_certificate()
        };

        let cert = match cert {
            Some(cert) => cert,
            None => return Ok(None),
        };

        let algo_nid = cert.signature_algorithm().object().nid();
        let signature_algorithms = match algo_nid.signature_algorithms() {
            Some(algs) => algs,
            None => return Ok(None),
        };

        let md = match signature_algorithms.digest {
            Nid::MD5 | Nid::SHA1 => MessageDigest::sha256(),
            nid => match MessageDigest::from_nid(nid) {
                Some(md) => md,
                None => return Ok(None),
            },
        };

        let digest = cert.digest(md)?;

        Ok(Some(digest.to_vec()))
    }

    pub fn shutdown(&mut self) -> io::Result<()> {
        match self.0.shutdown() {
            Ok(_) => Ok(()),
            Err(ref e) if e.code() == ssl::ErrorCode::ZERO_RETURN => Ok(()),
            Err(e) => Err(e.into_io_error().unwrap_or_else(io::Error::other)),
        }
    }
}

impl<S: io::Read + io::Write> io::Read for TlsStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<S: io::Read + io::Write> io::Write for TlsStream<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}
