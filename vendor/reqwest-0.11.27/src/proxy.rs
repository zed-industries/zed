use std::fmt;
#[cfg(feature = "socks")]
use std::net::SocketAddr;
use std::sync::Arc;

use crate::into_url::{IntoUrl, IntoUrlSealed};
use crate::Url;
use http::{header::HeaderValue, Uri};
use ipnet::IpNet;
use once_cell::sync::Lazy;
use percent_encoding::percent_decode;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::net::IpAddr;
#[cfg(target_os = "macos")]
use system_configuration::{
    core_foundation::{
        base::CFType,
        dictionary::CFDictionary,
        number::CFNumber,
        string::{CFString, CFStringRef},
    },
    dynamic_store::SCDynamicStoreBuilder,
    sys::schema_definitions::kSCPropNetProxiesHTTPEnable,
    sys::schema_definitions::kSCPropNetProxiesHTTPPort,
    sys::schema_definitions::kSCPropNetProxiesHTTPProxy,
    sys::schema_definitions::kSCPropNetProxiesHTTPSEnable,
    sys::schema_definitions::kSCPropNetProxiesHTTPSPort,
    sys::schema_definitions::kSCPropNetProxiesHTTPSProxy,
};
#[cfg(target_os = "windows")]
use winreg::enums::HKEY_CURRENT_USER;
#[cfg(target_os = "windows")]
use winreg::RegKey;

/// Configuration of a proxy that a `Client` should pass requests to.
///
/// A `Proxy` has a couple pieces to it:
///
/// - a URL of how to talk to the proxy
/// - rules on what `Client` requests should be directed to the proxy
///
/// For instance, let's look at `Proxy::http`:
///
/// ```rust
/// # fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let proxy = reqwest::Proxy::http("https://secure.example")?;
/// # Ok(())
/// # }
/// ```
///
/// This proxy will intercept all HTTP requests, and make use of the proxy
/// at `https://secure.example`. A request to `http://hyper.rs` will talk
/// to your proxy. A request to `https://hyper.rs` will not.
///
/// Multiple `Proxy` rules can be configured for a `Client`. The `Client` will
/// check each `Proxy` in the order it was added. This could mean that a
/// `Proxy` added first with eager intercept rules, such as `Proxy::all`,
/// would prevent a `Proxy` later in the list from ever working, so take care.
///
/// By enabling the `"socks"` feature it is possible to use a socks proxy:
/// ```rust
/// # fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let proxy = reqwest::Proxy::http("socks5://192.168.1.1:9000")?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct Proxy {
    intercept: Intercept,
    no_proxy: Option<NoProxy>,
}

/// Represents a possible matching entry for an IP address
#[derive(Clone, Debug)]
enum Ip {
    Address(IpAddr),
    Network(IpNet),
}

/// A wrapper around a list of IP cidr blocks or addresses with a [IpMatcher::contains] method for
/// checking if an IP address is contained within the matcher
#[derive(Clone, Debug, Default)]
struct IpMatcher(Vec<Ip>);

/// A wrapper around a list of domains with a [DomainMatcher::contains] method for checking if a
/// domain is contained within the matcher
#[derive(Clone, Debug, Default)]
struct DomainMatcher(Vec<String>);

/// A configuration for filtering out requests that shouldn't be proxied
#[derive(Clone, Debug, Default)]
pub struct NoProxy {
    ips: IpMatcher,
    domains: DomainMatcher,
}

/// A particular scheme used for proxying requests.
///
/// For example, HTTP vs SOCKS5
#[derive(Clone)]
pub enum ProxyScheme {
    Http {
        auth: Option<HeaderValue>,
        host: http::uri::Authority,
    },
    Https {
        auth: Option<HeaderValue>,
        host: http::uri::Authority,
    },
    #[cfg(feature = "socks")]
    Socks5 {
        addr: SocketAddr,
        auth: Option<(String, String)>,
        remote_dns: bool,
    },
}

impl ProxyScheme {
    fn maybe_http_auth(&self) -> Option<&HeaderValue> {
        match self {
            ProxyScheme::Http { auth, .. } | ProxyScheme::Https { auth, .. } => auth.as_ref(),
            #[cfg(feature = "socks")]
            _ => None,
        }
    }
}

/// Trait used for converting into a proxy scheme. This trait supports
/// parsing from a URL-like type, whilst also supporting proxy schemes
/// built directly using the factory methods.
pub trait IntoProxyScheme {
    fn into_proxy_scheme(self) -> crate::Result<ProxyScheme>;
}

impl<S: IntoUrl> IntoProxyScheme for S {
    fn into_proxy_scheme(self) -> crate::Result<ProxyScheme> {
        // validate the URL
        let url = match self.as_str().into_url() {
            Ok(ok) => ok,
            Err(e) => {
                let mut presumed_to_have_scheme = true;
                let mut source = e.source();
                while let Some(err) = source {
                    if let Some(parse_error) = err.downcast_ref::<url::ParseError>() {
                        match parse_error {
                            url::ParseError::RelativeUrlWithoutBase => {
                                presumed_to_have_scheme = false;
                                break;
                            }
                            _ => {}
                        }
                    } else if let Some(_) = err.downcast_ref::<crate::error::BadScheme>() {
                        presumed_to_have_scheme = false;
                        break;
                    }
                    source = err.source();
                }
                if presumed_to_have_scheme {
                    return Err(crate::error::builder(e));
                }
                // the issue could have been caused by a missing scheme, so we try adding http://
                let try_this = format!("http://{}", self.as_str());
                try_this.into_url().map_err(|_| {
                    // return the original error
                    crate::error::builder(e)
                })?
            }
        };
        ProxyScheme::parse(url)
    }
}

// These bounds are accidentally leaked by the blanket impl of IntoProxyScheme
// for all types that implement IntoUrl. So, this function exists to detect
// if we were to break those bounds for a user.
fn _implied_bounds() {
    fn prox<T: IntoProxyScheme>(_t: T) {}

    fn url<T: IntoUrl>(t: T) {
        prox(t);
    }
}

impl IntoProxyScheme for ProxyScheme {
    fn into_proxy_scheme(self) -> crate::Result<ProxyScheme> {
        Ok(self)
    }
}

impl Proxy {
    /// Proxy all HTTP traffic to the passed URL.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate reqwest;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::Client::builder()
    ///     .proxy(reqwest::Proxy::http("https://my.prox")?)
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// # fn main() {}
    /// ```
    pub fn http<U: IntoProxyScheme>(proxy_scheme: U) -> crate::Result<Proxy> {
        Ok(Proxy::new(Intercept::Http(
            proxy_scheme.into_proxy_scheme()?,
        )))
    }

    /// Proxy all HTTPS traffic to the passed URL.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate reqwest;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::Client::builder()
    ///     .proxy(reqwest::Proxy::https("https://example.prox:4545")?)
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// # fn main() {}
    /// ```
    pub fn https<U: IntoProxyScheme>(proxy_scheme: U) -> crate::Result<Proxy> {
        Ok(Proxy::new(Intercept::Https(
            proxy_scheme.into_proxy_scheme()?,
        )))
    }

    /// Proxy **all** traffic to the passed URL.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate reqwest;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::Client::builder()
    ///     .proxy(reqwest::Proxy::all("http://pro.xy")?)
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// # fn main() {}
    /// ```
    pub fn all<U: IntoProxyScheme>(proxy_scheme: U) -> crate::Result<Proxy> {
        Ok(Proxy::new(Intercept::All(
            proxy_scheme.into_proxy_scheme()?,
        )))
    }

    /// Provide a custom function to determine what traffic to proxy to where.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate reqwest;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let target = reqwest::Url::parse("https://my.prox")?;
    /// let client = reqwest::Client::builder()
    ///     .proxy(reqwest::Proxy::custom(move |url| {
    ///         if url.host_str() == Some("hyper.rs") {
    ///             Some(target.clone())
    ///         } else {
    ///             None
    ///         }
    ///     }))
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// # fn main() {}
    /// ```
    pub fn custom<F, U: IntoProxyScheme>(fun: F) -> Proxy
    where
        F: Fn(&Url) -> Option<U> + Send + Sync + 'static,
    {
        Proxy::new(Intercept::Custom(Custom {
            auth: None,
            func: Arc::new(move |url| fun(url).map(IntoProxyScheme::into_proxy_scheme)),
        }))
    }

    pub(crate) fn system() -> Proxy {
        let mut proxy = if cfg!(feature = "__internal_proxy_sys_no_cache") {
            Proxy::new(Intercept::System(Arc::new(get_sys_proxies(
                get_from_platform(),
            ))))
        } else {
            Proxy::new(Intercept::System(SYS_PROXIES.clone()))
        };
        proxy.no_proxy = NoProxy::from_env();
        proxy
    }

    fn new(intercept: Intercept) -> Proxy {
        Proxy {
            intercept,
            no_proxy: None,
        }
    }

    /// Set the `Proxy-Authorization` header using Basic auth.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate reqwest;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let proxy = reqwest::Proxy::https("http://localhost:1234")?
    ///     .basic_auth("Aladdin", "open sesame");
    /// # Ok(())
    /// # }
    /// # fn main() {}
    /// ```
    pub fn basic_auth(mut self, username: &str, password: &str) -> Proxy {
        self.intercept.set_basic_auth(username, password);
        self
    }

    /// Set the `Proxy-Authorization` header to a specified value.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate reqwest;
    /// # use reqwest::header::*;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let proxy = reqwest::Proxy::https("http://localhost:1234")?
    ///     .custom_http_auth(HeaderValue::from_static("justletmeinalreadyplease"));
    /// # Ok(())
    /// # }
    /// # fn main() {}
    /// ```
    pub fn custom_http_auth(mut self, header_value: HeaderValue) -> Proxy {
        self.intercept.set_custom_http_auth(header_value);
        self
    }

    /// Adds a `No Proxy` exclusion list to this Proxy
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate reqwest;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let proxy = reqwest::Proxy::https("http://localhost:1234")?
    ///     .no_proxy(reqwest::NoProxy::from_string("direct.tld, sub.direct2.tld"));
    /// # Ok(())
    /// # }
    /// # fn main() {}
    /// ```
    pub fn no_proxy(mut self, no_proxy: Option<NoProxy>) -> Proxy {
        self.no_proxy = no_proxy;
        self
    }

    pub(crate) fn maybe_has_http_auth(&self) -> bool {
        match &self.intercept {
            Intercept::All(p) | Intercept::Http(p) => p.maybe_http_auth().is_some(),
            // Custom *may* match 'http', so assume so.
            Intercept::Custom(_) => true,
            Intercept::System(system) => system
                .get("http")
                .and_then(|s| s.maybe_http_auth())
                .is_some(),
            Intercept::Https(_) => false,
        }
    }

    pub(crate) fn http_basic_auth<D: Dst>(&self, uri: &D) -> Option<HeaderValue> {
        match &self.intercept {
            Intercept::All(p) | Intercept::Http(p) => p.maybe_http_auth().cloned(),
            Intercept::System(system) => system
                .get("http")
                .and_then(|s| s.maybe_http_auth().cloned()),
            Intercept::Custom(custom) => {
                custom.call(uri).and_then(|s| s.maybe_http_auth().cloned())
            }
            Intercept::Https(_) => None,
        }
    }

    pub(crate) fn intercept<D: Dst>(&self, uri: &D) -> Option<ProxyScheme> {
        let in_no_proxy = self
            .no_proxy
            .as_ref()
            .map_or(false, |np| np.contains(uri.host()));
        match self.intercept {
            Intercept::All(ref u) => {
                if !in_no_proxy {
                    Some(u.clone())
                } else {
                    None
                }
            }
            Intercept::Http(ref u) => {
                if !in_no_proxy && uri.scheme() == "http" {
                    Some(u.clone())
                } else {
                    None
                }
            }
            Intercept::Https(ref u) => {
                if !in_no_proxy && uri.scheme() == "https" {
                    Some(u.clone())
                } else {
                    None
                }
            }
            Intercept::System(ref map) => {
                if in_no_proxy {
                    None
                } else {
                    map.get(uri.scheme()).cloned()
                }
            }
            Intercept::Custom(ref custom) => {
                if !in_no_proxy {
                    custom.call(uri)
                } else {
                    None
                }
            }
        }
    }

    pub(crate) fn is_match<D: Dst>(&self, uri: &D) -> bool {
        match self.intercept {
            Intercept::All(_) => true,
            Intercept::Http(_) => uri.scheme() == "http",
            Intercept::Https(_) => uri.scheme() == "https",
            Intercept::System(ref map) => map.contains_key(uri.scheme()),
            Intercept::Custom(ref custom) => custom.call(uri).is_some(),
        }
    }
}

impl fmt::Debug for Proxy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Proxy")
            .field(&self.intercept)
            .field(&self.no_proxy)
            .finish()
    }
}

impl NoProxy {
    /// Returns a new no-proxy configuration based on environment variables (or `None` if no variables are set)
    /// see [self::NoProxy::from_string()] for the string format
    pub fn from_env() -> Option<NoProxy> {
        let raw = env::var("NO_PROXY")
            .or_else(|_| env::var("no_proxy"))
            .unwrap_or_default();

        Self::from_string(&raw)
    }

    /// Returns a new no-proxy configuration based on a `no_proxy` string (or `None` if no variables
    /// are set)
    /// The rules are as follows:
    /// * The environment variable `NO_PROXY` is checked, if it is not set, `no_proxy` is checked
    /// * If neither environment variable is set, `None` is returned
    /// * Entries are expected to be comma-separated (whitespace between entries is ignored)
    /// * IP addresses (both IPv4 and IPv6) are allowed, as are optional subnet masks (by adding /size,
    /// for example "`192.168.1.0/24`").
    /// * An entry "`*`" matches all hostnames (this is the only wildcard allowed)
    /// * Any other entry is considered a domain name (and may contain a leading dot, for example `google.com`
    /// and `.google.com` are equivalent) and would match both that domain AND all subdomains.
    ///
    /// For example, if `"NO_PROXY=google.com, 192.168.1.0/24"` was set, all of the following would match
    /// (and therefore would bypass the proxy):
    /// * `http://google.com/`
    /// * `http://www.google.com/`
    /// * `http://192.168.1.42/`
    ///
    /// The URL `http://notgoogle.com/` would not match.
    pub fn from_string(no_proxy_list: &str) -> Option<Self> {
        if no_proxy_list.is_empty() {
            return None;
        }
        let mut ips = Vec::new();
        let mut domains = Vec::new();
        let parts = no_proxy_list.split(',').map(str::trim);
        for part in parts {
            match part.parse::<IpNet>() {
                // If we can parse an IP net or address, then use it, otherwise, assume it is a domain
                Ok(ip) => ips.push(Ip::Network(ip)),
                Err(_) => match part.parse::<IpAddr>() {
                    Ok(addr) => ips.push(Ip::Address(addr)),
                    Err(_) => domains.push(part.to_owned()),
                },
            }
        }
        Some(NoProxy {
            ips: IpMatcher(ips),
            domains: DomainMatcher(domains),
        })
    }

    fn contains(&self, host: &str) -> bool {
        // According to RFC3986, raw IPv6 hosts will be wrapped in []. So we need to strip those off
        // the end in order to parse correctly
        let host = if host.starts_with('[') {
            let x: &[_] = &['[', ']'];
            host.trim_matches(x)
        } else {
            host
        };
        match host.parse::<IpAddr>() {
            // If we can parse an IP addr, then use it, otherwise, assume it is a domain
            Ok(ip) => self.ips.contains(ip),
            Err(_) => self.domains.contains(host),
        }
    }
}

impl IpMatcher {
    fn contains(&self, addr: IpAddr) -> bool {
        for ip in &self.0 {
            match ip {
                Ip::Address(address) => {
                    if &addr == address {
                        return true;
                    }
                }
                Ip::Network(net) => {
                    if net.contains(&addr) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl DomainMatcher {
    // The following links may be useful to understand the origin of these rules:
    // * https://curl.se/libcurl/c/CURLOPT_NOPROXY.html
    // * https://github.com/curl/curl/issues/1208
    fn contains(&self, domain: &str) -> bool {
        let domain_len = domain.len();
        for d in &self.0 {
            if d == domain || d.strip_prefix('.') == Some(domain) {
                return true;
            } else if domain.ends_with(d) {
                if d.starts_with('.') {
                    // If the first character of d is a dot, that means the first character of domain
                    // must also be a dot, so we are looking at a subdomain of d and that matches
                    return true;
                } else if domain.as_bytes().get(domain_len - d.len() - 1) == Some(&b'.') {
                    // Given that d is a prefix of domain, if the prior character in domain is a dot
                    // then that means we must be matching a subdomain of d, and that matches
                    return true;
                }
            } else if d == "*" {
                return true;
            }
        }
        false
    }
}

impl ProxyScheme {
    // To start conservative, keep builders private for now.

    /// Proxy traffic via the specified URL over HTTP
    fn http(host: &str) -> crate::Result<Self> {
        Ok(ProxyScheme::Http {
            auth: None,
            host: host.parse().map_err(crate::error::builder)?,
        })
    }

    /// Proxy traffic via the specified URL over HTTPS
    fn https(host: &str) -> crate::Result<Self> {
        Ok(ProxyScheme::Https {
            auth: None,
            host: host.parse().map_err(crate::error::builder)?,
        })
    }

    /// Proxy traffic via the specified socket address over SOCKS5
    ///
    /// # Note
    ///
    /// Current SOCKS5 support is provided via blocking IO.
    #[cfg(feature = "socks")]
    fn socks5(addr: SocketAddr) -> crate::Result<Self> {
        Ok(ProxyScheme::Socks5 {
            addr,
            auth: None,
            remote_dns: false,
        })
    }

    /// Proxy traffic via the specified socket address over SOCKS5H
    ///
    /// This differs from SOCKS5 in that DNS resolution is also performed via the proxy.
    ///
    /// # Note
    ///
    /// Current SOCKS5 support is provided via blocking IO.
    #[cfg(feature = "socks")]
    fn socks5h(addr: SocketAddr) -> crate::Result<Self> {
        Ok(ProxyScheme::Socks5 {
            addr,
            auth: None,
            remote_dns: true,
        })
    }

    /// Use a username and password when connecting to the proxy server
    fn with_basic_auth<T: Into<String>, U: Into<String>>(
        mut self,
        username: T,
        password: U,
    ) -> Self {
        self.set_basic_auth(username, password);
        self
    }

    fn set_basic_auth<T: Into<String>, U: Into<String>>(&mut self, username: T, password: U) {
        match *self {
            ProxyScheme::Http { ref mut auth, .. } => {
                let header = encode_basic_auth(&username.into(), &password.into());
                *auth = Some(header);
            }
            ProxyScheme::Https { ref mut auth, .. } => {
                let header = encode_basic_auth(&username.into(), &password.into());
                *auth = Some(header);
            }
            #[cfg(feature = "socks")]
            ProxyScheme::Socks5 { ref mut auth, .. } => {
                *auth = Some((username.into(), password.into()));
            }
        }
    }

    fn set_custom_http_auth(&mut self, header_value: HeaderValue) {
        match *self {
            ProxyScheme::Http { ref mut auth, .. } => {
                *auth = Some(header_value);
            }
            ProxyScheme::Https { ref mut auth, .. } => {
                *auth = Some(header_value);
            }
            #[cfg(feature = "socks")]
            ProxyScheme::Socks5 { .. } => {
                panic!("Socks is not supported for this method")
            }
        }
    }

    fn if_no_auth(mut self, update: &Option<HeaderValue>) -> Self {
        match self {
            ProxyScheme::Http { ref mut auth, .. } => {
                if auth.is_none() {
                    *auth = update.clone();
                }
            }
            ProxyScheme::Https { ref mut auth, .. } => {
                if auth.is_none() {
                    *auth = update.clone();
                }
            }
            #[cfg(feature = "socks")]
            ProxyScheme::Socks5 { .. } => {}
        }

        self
    }

    /// Convert a URL into a proxy scheme
    ///
    /// Supported schemes: HTTP, HTTPS, (SOCKS5, SOCKS5H if `socks` feature is enabled).
    // Private for now...
    fn parse(url: Url) -> crate::Result<Self> {
        use url::Position;

        // Resolve URL to a host and port
        #[cfg(feature = "socks")]
        let to_addr = || {
            let addrs = url
                .socket_addrs(|| match url.scheme() {
                    "socks5" | "socks5h" => Some(1080),
                    _ => None,
                })
                .map_err(crate::error::builder)?;
            addrs
                .into_iter()
                .next()
                .ok_or_else(|| crate::error::builder("unknown proxy scheme"))
        };

        let mut scheme = match url.scheme() {
            "http" => Self::http(&url[Position::BeforeHost..Position::AfterPort])?,
            "https" => Self::https(&url[Position::BeforeHost..Position::AfterPort])?,
            #[cfg(feature = "socks")]
            "socks5" => Self::socks5(to_addr()?)?,
            #[cfg(feature = "socks")]
            "socks5h" => Self::socks5h(to_addr()?)?,
            _ => return Err(crate::error::builder("unknown proxy scheme")),
        };

        if let Some(pwd) = url.password() {
            let decoded_username = percent_decode(url.username().as_bytes()).decode_utf8_lossy();
            let decoded_password = percent_decode(pwd.as_bytes()).decode_utf8_lossy();
            scheme = scheme.with_basic_auth(decoded_username, decoded_password);
        }

        Ok(scheme)
    }

    #[cfg(test)]
    fn scheme(&self) -> &str {
        match self {
            ProxyScheme::Http { .. } => "http",
            ProxyScheme::Https { .. } => "https",
            #[cfg(feature = "socks")]
            ProxyScheme::Socks5 { .. } => "socks5",
        }
    }

    #[cfg(test)]
    fn host(&self) -> &str {
        match self {
            ProxyScheme::Http { host, .. } => host.as_str(),
            ProxyScheme::Https { host, .. } => host.as_str(),
            #[cfg(feature = "socks")]
            ProxyScheme::Socks5 { .. } => panic!("socks5"),
        }
    }
}

impl fmt::Debug for ProxyScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProxyScheme::Http { auth: _auth, host } => write!(f, "http://{host}"),
            ProxyScheme::Https { auth: _auth, host } => write!(f, "https://{host}"),
            #[cfg(feature = "socks")]
            ProxyScheme::Socks5 {
                addr,
                auth: _auth,
                remote_dns,
            } => {
                let h = if *remote_dns { "h" } else { "" };
                write!(f, "socks5{h}://{addr}")
            }
        }
    }
}

type SystemProxyMap = HashMap<String, ProxyScheme>;

#[derive(Clone, Debug)]
enum Intercept {
    All(ProxyScheme),
    Http(ProxyScheme),
    Https(ProxyScheme),
    System(Arc<SystemProxyMap>),
    Custom(Custom),
}

impl Intercept {
    fn set_basic_auth(&mut self, username: &str, password: &str) {
        match self {
            Intercept::All(ref mut s)
            | Intercept::Http(ref mut s)
            | Intercept::Https(ref mut s) => s.set_basic_auth(username, password),
            Intercept::System(_) => unimplemented!(),
            Intercept::Custom(ref mut custom) => {
                let header = encode_basic_auth(username, password);
                custom.auth = Some(header);
            }
        }
    }

    fn set_custom_http_auth(&mut self, header_value: HeaderValue) {
        match self {
            Intercept::All(ref mut s)
            | Intercept::Http(ref mut s)
            | Intercept::Https(ref mut s) => s.set_custom_http_auth(header_value),
            Intercept::System(_) => unimplemented!(),
            Intercept::Custom(ref mut custom) => {
                custom.auth = Some(header_value);
            }
        }
    }
}

#[derive(Clone)]
struct Custom {
    // This auth only applies if the returned ProxyScheme doesn't have an auth...
    auth: Option<HeaderValue>,
    func: Arc<dyn Fn(&Url) -> Option<crate::Result<ProxyScheme>> + Send + Sync + 'static>,
}

impl Custom {
    fn call<D: Dst>(&self, uri: &D) -> Option<ProxyScheme> {
        let url = format!(
            "{}://{}{}{}",
            uri.scheme(),
            uri.host(),
            uri.port().map_or("", |_| ":"),
            uri.port().map_or(String::new(), |p| p.to_string())
        )
        .parse()
        .expect("should be valid Url");

        (self.func)(&url)
            .and_then(|result| result.ok())
            .map(|scheme| scheme.if_no_auth(&self.auth))
    }
}

impl fmt::Debug for Custom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("_")
    }
}

pub(crate) fn encode_basic_auth(username: &str, password: &str) -> HeaderValue {
    crate::util::basic_auth(username, Some(password))
}

/// A helper trait to allow testing `Proxy::intercept` without having to
/// construct `hyper::client::connect::Destination`s.
pub(crate) trait Dst {
    fn scheme(&self) -> &str;
    fn host(&self) -> &str;
    fn port(&self) -> Option<u16>;
}

#[doc(hidden)]
impl Dst for Uri {
    fn scheme(&self) -> &str {
        self.scheme().expect("Uri should have a scheme").as_str()
    }

    fn host(&self) -> &str {
        Uri::host(self).expect("<Uri as Dst>::host should have a str")
    }

    fn port(&self) -> Option<u16> {
        self.port().map(|p| p.as_u16())
    }
}

static SYS_PROXIES: Lazy<Arc<SystemProxyMap>> =
    Lazy::new(|| Arc::new(get_sys_proxies(get_from_platform())));

/// Get system proxies information.
///
/// All platforms will check for proxy settings via environment variables.
/// If those aren't set, platform-wide proxy settings will be looked up on
/// Windows and MacOS platforms instead. Errors encountered while discovering
/// these settings are ignored.
///
/// Returns:
///     System proxies information as a hashmap like
///     {"http": Url::parse("http://127.0.0.1:80"), "https": Url::parse("https://127.0.0.1:80")}
fn get_sys_proxies(
    #[cfg_attr(
        not(any(target_os = "windows", target_os = "macos")),
        allow(unused_variables)
    )]
    platform_proxies: Option<String>,
) -> SystemProxyMap {
    let proxies = get_from_environment();

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    if proxies.is_empty() {
        // if there are errors in acquiring the platform proxies,
        // we'll just return an empty HashMap
        if let Some(platform_proxies) = platform_proxies {
            return parse_platform_values(platform_proxies);
        }
    }

    proxies
}

fn insert_proxy(proxies: &mut SystemProxyMap, scheme: impl Into<String>, addr: String) -> bool {
    if addr.trim().is_empty() {
        // do not accept empty or whitespace proxy address
        false
    } else if let Ok(valid_addr) = addr.into_proxy_scheme() {
        proxies.insert(scheme.into(), valid_addr);
        true
    } else {
        false
    }
}

fn get_from_environment() -> SystemProxyMap {
    let mut proxies = HashMap::new();

    if is_cgi() {
        if log::log_enabled!(log::Level::Warn) && env::var_os("HTTP_PROXY").is_some() {
            log::warn!("HTTP_PROXY environment variable ignored in CGI");
        }
    } else if !insert_from_env(&mut proxies, "http", "HTTP_PROXY") {
        insert_from_env(&mut proxies, "http", "http_proxy");
    }

    if !insert_from_env(&mut proxies, "https", "HTTPS_PROXY") {
        insert_from_env(&mut proxies, "https", "https_proxy");
    }

    if !(insert_from_env(&mut proxies, "http", "ALL_PROXY")
        && insert_from_env(&mut proxies, "https", "ALL_PROXY"))
    {
        insert_from_env(&mut proxies, "http", "all_proxy");
        insert_from_env(&mut proxies, "https", "all_proxy");
    }

    proxies
}

fn insert_from_env(proxies: &mut SystemProxyMap, scheme: &str, var: &str) -> bool {
    if let Ok(val) = env::var(var) {
        insert_proxy(proxies, scheme, val)
    } else {
        false
    }
}

/// Check if we are being executed in a CGI context.
///
/// If so, a malicious client can send the `Proxy:` header, and it will
/// be in the `HTTP_PROXY` env var. So we don't use it :)
fn is_cgi() -> bool {
    env::var_os("REQUEST_METHOD").is_some()
}

#[cfg(target_os = "windows")]
fn get_from_platform_impl() -> Result<Option<String>, Box<dyn Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let internet_setting: RegKey =
        hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings")?;
    // ensure the proxy is enable, if the value doesn't exist, an error will returned.
    let proxy_enable: u32 = internet_setting.get_value("ProxyEnable")?;
    let proxy_server: String = internet_setting.get_value("ProxyServer")?;

    Ok((proxy_enable == 1).then_some(proxy_server))
}

#[cfg(target_os = "macos")]
fn parse_setting_from_dynamic_store(
    proxies_map: &CFDictionary<CFString, CFType>,
    enabled_key: CFStringRef,
    host_key: CFStringRef,
    port_key: CFStringRef,
    scheme: &str,
) -> Option<String> {
    let proxy_enabled = proxies_map
        .find(enabled_key)
        .and_then(|flag| flag.downcast::<CFNumber>())
        .and_then(|flag| flag.to_i32())
        .unwrap_or(0)
        == 1;

    if proxy_enabled {
        let proxy_host = proxies_map
            .find(host_key)
            .and_then(|host| host.downcast::<CFString>())
            .map(|host| host.to_string());
        let proxy_port = proxies_map
            .find(port_key)
            .and_then(|port| port.downcast::<CFNumber>())
            .and_then(|port| port.to_i32());

        return match (proxy_host, proxy_port) {
            (Some(proxy_host), Some(proxy_port)) => {
                Some(format!("{scheme}={proxy_host}:{proxy_port}"))
            }
            (Some(proxy_host), None) => Some(format!("{scheme}={proxy_host}")),
            (None, Some(_)) => None,
            (None, None) => None,
        };
    }

    None
}

#[cfg(target_os = "macos")]
fn get_from_platform_impl() -> Result<Option<String>, Box<dyn Error>> {
    let store = SCDynamicStoreBuilder::new("reqwest").build();

    let proxies_map = if let Some(proxies_map) = store.get_proxies() {
        proxies_map
    } else {
        return Ok(None);
    };

    let http_proxy_config = parse_setting_from_dynamic_store(
        &proxies_map,
        unsafe { kSCPropNetProxiesHTTPEnable },
        unsafe { kSCPropNetProxiesHTTPProxy },
        unsafe { kSCPropNetProxiesHTTPPort },
        "http",
    );
    let https_proxy_config = parse_setting_from_dynamic_store(
        &proxies_map,
        unsafe { kSCPropNetProxiesHTTPSEnable },
        unsafe { kSCPropNetProxiesHTTPSProxy },
        unsafe { kSCPropNetProxiesHTTPSPort },
        "https",
    );

    match http_proxy_config.as_ref().zip(https_proxy_config.as_ref()) {
        Some((http_config, https_config)) => Ok(Some(format!("{http_config};{https_config}"))),
        None => Ok(http_proxy_config.or(https_proxy_config)),
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn get_from_platform() -> Option<String> {
    get_from_platform_impl().ok().flatten()
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn get_from_platform() -> Option<String> {
    None
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn parse_platform_values_impl(platform_values: String) -> SystemProxyMap {
    let mut proxies = HashMap::new();
    if platform_values.contains("=") {
        // per-protocol settings.
        for p in platform_values.split(";") {
            let protocol_parts: Vec<&str> = p.split("=").collect();
            match protocol_parts.as_slice() {
                [protocol, address] => {
                    // If address doesn't specify an explicit protocol as protocol://address
                    // then default to HTTP
                    let address = if extract_type_prefix(*address).is_some() {
                        String::from(*address)
                    } else {
                        format!("http://{address}")
                    };

                    insert_proxy(&mut proxies, *protocol, address);
                }
                _ => {
                    // Contains invalid protocol setting, just break the loop
                    // And make proxies to be empty.
                    proxies.clear();
                    break;
                }
            }
        }
    } else {
        if let Some(scheme) = extract_type_prefix(&platform_values) {
            // Explicit protocol has been specified
            insert_proxy(&mut proxies, scheme, platform_values.to_owned());
        } else {
            // No explicit protocol has been specified, default to HTTP
            insert_proxy(&mut proxies, "http", format!("http://{platform_values}"));
            insert_proxy(&mut proxies, "https", format!("http://{platform_values}"));
        }
    }
    proxies
}

/// Extract the protocol from the given address, if present
/// For example, "https://example.com" will return Some("https")
#[cfg(any(target_os = "windows", target_os = "macos"))]
fn extract_type_prefix(address: &str) -> Option<&str> {
    if let Some(indice) = address.find("://") {
        if indice == 0 {
            None
        } else {
            let prefix = &address[..indice];
            let contains_banned = prefix.contains(|c| c == ':' || c == '/');

            if !contains_banned {
                Some(prefix)
            } else {
                None
            }
        }
    } else {
        None
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn parse_platform_values(platform_values: String) -> SystemProxyMap {
    parse_platform_values_impl(platform_values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    impl Dst for Url {
        fn scheme(&self) -> &str {
            Url::scheme(self)
        }

        fn host(&self) -> &str {
            Url::host_str(self).expect("<Url as Dst>::host should have a str")
        }

        fn port(&self) -> Option<u16> {
            Url::port(self)
        }
    }

    fn url(s: &str) -> Url {
        s.parse().unwrap()
    }

    fn intercepted_uri(p: &Proxy, s: &str) -> Uri {
        let (scheme, host) = match p.intercept(&url(s)).unwrap() {
            ProxyScheme::Http { host, .. } => ("http", host),
            ProxyScheme::Https { host, .. } => ("https", host),
            #[cfg(feature = "socks")]
            _ => panic!("intercepted as socks"),
        };
        http::Uri::builder()
            .scheme(scheme)
            .authority(host)
            .path_and_query("/")
            .build()
            .expect("intercepted_uri")
    }

    #[test]
    fn test_http() {
        let target = "http://example.domain/";
        let p = Proxy::http(target).unwrap();

        let http = "http://hyper.rs";
        let other = "https://hyper.rs";

        assert_eq!(intercepted_uri(&p, http), target);
        assert!(p.intercept(&url(other)).is_none());
    }

    #[test]
    fn test_https() {
        let target = "http://example.domain/";
        let p = Proxy::https(target).unwrap();

        let http = "http://hyper.rs";
        let other = "https://hyper.rs";

        assert!(p.intercept(&url(http)).is_none());
        assert_eq!(intercepted_uri(&p, other), target);
    }

    #[test]
    fn test_all() {
        let target = "http://example.domain/";
        let p = Proxy::all(target).unwrap();

        let http = "http://hyper.rs";
        let https = "https://hyper.rs";
        let other = "x-youve-never-heard-of-me-mr-proxy://hyper.rs";

        assert_eq!(intercepted_uri(&p, http), target);
        assert_eq!(intercepted_uri(&p, https), target);
        assert_eq!(intercepted_uri(&p, other), target);
    }

    #[test]
    fn test_custom() {
        let target1 = "http://example.domain/";
        let target2 = "https://example.domain/";
        let p = Proxy::custom(move |url| {
            if url.host_str() == Some("hyper.rs") {
                target1.parse().ok()
            } else if url.scheme() == "http" {
                target2.parse().ok()
            } else {
                None::<Url>
            }
        });

        let http = "http://seanmonstar.com";
        let https = "https://hyper.rs";
        let other = "x-youve-never-heard-of-me-mr-proxy://seanmonstar.com";

        assert_eq!(intercepted_uri(&p, http), target2);
        assert_eq!(intercepted_uri(&p, https), target1);
        assert!(p.intercept(&url(other)).is_none());
    }

    #[test]
    fn test_proxy_scheme_parse() {
        let ps = "http://foo:bar@localhost:1239".into_proxy_scheme().unwrap();

        match ps {
            ProxyScheme::Http { auth, host } => {
                assert_eq!(auth.unwrap(), encode_basic_auth("foo", "bar"));
                assert_eq!(host, "localhost:1239");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn test_proxy_scheme_ip_address_default_http() {
        let ps = "192.168.1.1:8888".into_proxy_scheme().unwrap();

        match ps {
            ProxyScheme::Http { auth, host } => {
                assert!(auth.is_none());
                assert_eq!(host, "192.168.1.1:8888");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn test_proxy_scheme_parse_default_http_with_auth() {
        // this should fail because `foo` is interpreted as the scheme and no host can be found
        let ps = "foo:bar@localhost:1239".into_proxy_scheme().unwrap();

        match ps {
            ProxyScheme::Http { auth, host } => {
                assert_eq!(auth.unwrap(), encode_basic_auth("foo", "bar"));
                assert_eq!(host, "localhost:1239");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn test_domain_matcher() {
        let domains = vec![".foo.bar".into(), "bar.foo".into()];
        let matcher = DomainMatcher(domains);

        // domains match with leading `.`
        assert!(matcher.contains("foo.bar"));
        // subdomains match with leading `.`
        assert!(matcher.contains("www.foo.bar"));

        // domains match with no leading `.`
        assert!(matcher.contains("bar.foo"));
        // subdomains match with no leading `.`
        assert!(matcher.contains("www.bar.foo"));

        // non-subdomain string prefixes don't match
        assert!(!matcher.contains("notfoo.bar"));
        assert!(!matcher.contains("notbar.foo"));
    }

    // Smallest possible content for a mutex
    struct MutexInner;

    static ENVLOCK: Lazy<Mutex<MutexInner>> = Lazy::new(|| Mutex::new(MutexInner));

    #[test]
    fn test_get_sys_proxies_parsing() {
        // Stop other threads from modifying process-global ENV while we are.
        let _lock = ENVLOCK.lock();
        // save system setting first.
        let _g1 = env_guard("HTTP_PROXY");
        let _g2 = env_guard("http_proxy");
        let _g3 = env_guard("ALL_PROXY");

        // Mock ENV, get the results, before doing assertions
        // to avoid assert! -> panic! -> Mutex Poisoned.
        let baseline_proxies = get_sys_proxies(None);
        // the system proxy setting url is invalid.
        env::set_var("http_proxy", "file://123465");
        let invalid_proxies = get_sys_proxies(None);
        // set valid proxy
        env::set_var("http_proxy", "127.0.0.1/");
        let valid_proxies = get_sys_proxies(None);
        // set valid ALL_PROXY
        env::set_var("ALL_PROXY", "127.0.0.2/");
        let all_proxies = get_sys_proxies(None);

        // reset user setting when guards drop
        drop(_g1);
        drop(_g2);
        // Let other threads run now
        drop(_lock);

        assert!(!baseline_proxies.contains_key("http"));
        assert!(!invalid_proxies.contains_key("http"));

        let p = &valid_proxies["http"];
        assert_eq!(p.scheme(), "http");
        assert_eq!(p.host(), "127.0.0.1");

        assert_eq!(all_proxies.len(), 2);
        assert!(all_proxies.values().all(|p| p.host() == "127.0.0.2"));
    }

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    #[test]
    fn test_get_sys_proxies_registry_parsing() {
        // Stop other threads from modifying process-global ENV while we are.
        let _lock = ENVLOCK.lock();
        // save system setting first.
        let _g1 = env_guard("HTTP_PROXY");
        let _g2 = env_guard("http_proxy");

        // Mock ENV, get the results, before doing assertions
        // to avoid assert! -> panic! -> Mutex Poisoned.
        let baseline_proxies = get_sys_proxies(None);
        // set valid proxy
        let valid_proxies = get_sys_proxies(Some(String::from("http://127.0.0.1/")));
        let valid_proxies_no_scheme = get_sys_proxies(Some(String::from("127.0.0.1")));
        let valid_proxies_explicit_https =
            get_sys_proxies(Some(String::from("https://127.0.0.1/")));
        let multiple_proxies = get_sys_proxies(Some(String::from(
            "http=127.0.0.1:8888;https=127.0.0.2:8888",
        )));
        let multiple_proxies_explicit_scheme = get_sys_proxies(Some(String::from(
            "http=http://127.0.0.1:8888;https=https://127.0.0.2:8888",
        )));

        // reset user setting when guards drop
        drop(_g1);
        drop(_g2);
        // Let other threads run now
        drop(_lock);

        assert_eq!(baseline_proxies.contains_key("http"), false);

        let p = &valid_proxies["http"];
        assert_eq!(p.scheme(), "http");
        assert_eq!(p.host(), "127.0.0.1");

        let p = &valid_proxies_no_scheme["http"];
        assert_eq!(p.scheme(), "http");
        assert_eq!(p.host(), "127.0.0.1");

        let p = &valid_proxies_no_scheme["https"];
        assert_eq!(p.scheme(), "http");
        assert_eq!(p.host(), "127.0.0.1");

        let p = &valid_proxies_explicit_https["https"];
        assert_eq!(p.scheme(), "https");
        assert_eq!(p.host(), "127.0.0.1");

        let p = &multiple_proxies["http"];
        assert_eq!(p.scheme(), "http");
        assert_eq!(p.host(), "127.0.0.1:8888");

        let p = &multiple_proxies["https"];
        assert_eq!(p.scheme(), "http");
        assert_eq!(p.host(), "127.0.0.2:8888");

        let p = &multiple_proxies_explicit_scheme["http"];
        assert_eq!(p.scheme(), "http");
        assert_eq!(p.host(), "127.0.0.1:8888");

        let p = &multiple_proxies_explicit_scheme["https"];
        assert_eq!(p.scheme(), "https");
        assert_eq!(p.host(), "127.0.0.2:8888");
    }

    #[test]
    fn test_get_sys_proxies_in_cgi() {
        // Stop other threads from modifying process-global ENV while we are.
        let _lock = ENVLOCK.lock();
        // save system setting first.
        let _g1 = env_guard("REQUEST_METHOD");
        let _g2 = env_guard("HTTP_PROXY");

        // Mock ENV, get the results, before doing assertions
        // to avoid assert! -> panic! -> Mutex Poisoned.
        env::set_var("HTTP_PROXY", "http://evil/");

        let baseline_proxies = get_sys_proxies(None);
        // set like we're in CGI
        env::set_var("REQUEST_METHOD", "GET");

        let cgi_proxies = get_sys_proxies(None);

        // reset user setting when guards drop
        drop(_g1);
        drop(_g2);
        // Let other threads run now
        drop(_lock);

        // not in CGI yet
        assert_eq!(baseline_proxies["http"].host(), "evil");
        // In CGI
        assert!(!cgi_proxies.contains_key("http"));
    }

    #[test]
    fn test_sys_no_proxy() {
        // Stop other threads from modifying process-global ENV while we are.
        let _lock = ENVLOCK.lock();
        // save system setting first.
        let _g1 = env_guard("HTTP_PROXY");
        let _g2 = env_guard("NO_PROXY");

        let target = "http://example.domain/";
        env::set_var("HTTP_PROXY", target);

        env::set_var(
            "NO_PROXY",
            ".foo.bar, bar.baz,10.42.1.1/24,::1,10.124.7.8,2001::/17",
        );

        // Manually construct this so we aren't use the cache
        let mut p = Proxy::new(Intercept::System(Arc::new(get_sys_proxies(None))));
        p.no_proxy = NoProxy::from_env();

        // random url, not in no_proxy
        assert_eq!(intercepted_uri(&p, "http://hyper.rs"), target);
        // make sure that random non-subdomain string prefixes don't match
        assert_eq!(intercepted_uri(&p, "http://notfoo.bar"), target);
        // make sure that random non-subdomain string prefixes don't match
        assert_eq!(intercepted_uri(&p, "http://notbar.baz"), target);
        // ipv4 address out of range
        assert_eq!(intercepted_uri(&p, "http://10.43.1.1"), target);
        // ipv4 address out of range
        assert_eq!(intercepted_uri(&p, "http://10.124.7.7"), target);
        // ipv6 address out of range
        assert_eq!(intercepted_uri(&p, "http://[ffff:db8:a0b:12f0::1]"), target);
        // ipv6 address out of range
        assert_eq!(intercepted_uri(&p, "http://[2005:db8:a0b:12f0::1]"), target);

        // make sure subdomains (with leading .) match
        assert!(p.intercept(&url("http://hello.foo.bar")).is_none());
        // make sure exact matches (without leading .) match (also makes sure spaces between entries work)
        assert!(p.intercept(&url("http://bar.baz")).is_none());
        // check case sensitivity
        assert!(p.intercept(&url("http://BAR.baz")).is_none());
        // make sure subdomains (without leading . in no_proxy) match
        assert!(p.intercept(&url("http://foo.bar.baz")).is_none());
        // make sure subdomains (without leading . in no_proxy) match - this differs from cURL
        assert!(p.intercept(&url("http://foo.bar")).is_none());
        // ipv4 address match within range
        assert!(p.intercept(&url("http://10.42.1.100")).is_none());
        // ipv6 address exact match
        assert!(p.intercept(&url("http://[::1]")).is_none());
        // ipv6 address match within range
        assert!(p.intercept(&url("http://[2001:db8:a0b:12f0::1]")).is_none());
        // ipv4 address exact match
        assert!(p.intercept(&url("http://10.124.7.8")).is_none());

        // reset user setting when guards drop
        drop(_g1);
        drop(_g2);
        // Let other threads run now
        drop(_lock);
    }

    #[test]
    fn test_proxy_no_proxy_interception_for_proxy_types() {
        let proxy_url = "http://example.domain/";
        let no_proxy = ".no.proxy.tld";

        // test all proxy interception
        let p = Proxy::all(proxy_url)
            .unwrap()
            .no_proxy(NoProxy::from_string(no_proxy));

        // random url, not in no_proxy
        assert_eq!(intercepted_uri(&p, "http://hyper.rs"), proxy_url);

        // positive match for no proxy
        assert!(p.intercept(&url("https://hello.no.proxy.tld")).is_none());

        // test http proxy interception
        let p = Proxy::http(proxy_url)
            .unwrap()
            .no_proxy(NoProxy::from_string(no_proxy));

        // random url, not in no_proxy
        assert_eq!(intercepted_uri(&p, "http://hyper.rs"), proxy_url);

        // positive match for no proxy
        assert!(p.intercept(&url("http://hello.no.proxy.tld")).is_none());

        // should not be intercepted due to scheme
        assert!(p.intercept(&url("https://hyper.rs")).is_none());

        // test https proxy interception
        let p = Proxy::https(proxy_url)
            .unwrap()
            .no_proxy(NoProxy::from_string(no_proxy));

        // random url, not in no_proxy
        assert_eq!(intercepted_uri(&p, "https://hyper.rs"), proxy_url);

        // positive match for no proxy
        assert!(p.intercept(&url("https://hello.no.proxy.tld")).is_none());

        // should not be intercepted due to scheme
        assert!(p.intercept(&url("http://hyper.rs")).is_none());

        // test custom proxy interception
        let p = Proxy::custom(move |_url| Some(proxy_url)).no_proxy(NoProxy::from_string(no_proxy));

        // random url, not in no_proxy
        assert_eq!(intercepted_uri(&p, "https://hyper.rs"), proxy_url);

        // positive match for no proxy
        assert!(p.intercept(&url("https://hello.no.proxy.tld")).is_none());
        assert!(p.intercept(&url("http://hello.no.proxy.tld")).is_none());
    }

    #[test]
    fn test_wildcard_sys_no_proxy() {
        // Stop other threads from modifying process-global ENV while we are.
        let _lock = ENVLOCK.lock();
        // save system setting first.
        let _g1 = env_guard("HTTP_PROXY");
        let _g2 = env_guard("NO_PROXY");

        let target = "http://example.domain/";
        env::set_var("HTTP_PROXY", target);

        env::set_var("NO_PROXY", "*");

        // Manually construct this so we aren't use the cache
        let mut p = Proxy::new(Intercept::System(Arc::new(get_sys_proxies(None))));
        p.no_proxy = NoProxy::from_env();

        assert!(p.intercept(&url("http://foo.bar")).is_none());

        // reset user setting when guards drop
        drop(_g1);
        drop(_g2);
        // Let other threads run now
        drop(_lock);
    }

    #[test]
    fn test_empty_sys_no_proxy() {
        // Stop other threads from modifying process-global ENV while we are.
        let _lock = ENVLOCK.lock();
        // save system setting first.
        let _g1 = env_guard("HTTP_PROXY");
        let _g2 = env_guard("NO_PROXY");

        let target = "http://example.domain/";
        env::set_var("HTTP_PROXY", target);

        env::set_var("NO_PROXY", ",");

        // Manually construct this so we aren't use the cache
        let mut p = Proxy::new(Intercept::System(Arc::new(get_sys_proxies(None))));
        p.no_proxy = NoProxy::from_env();

        // everything should go through proxy, "effectively" nothing is in no_proxy
        assert_eq!(intercepted_uri(&p, "http://hyper.rs"), target);

        // reset user setting when guards drop
        drop(_g1);
        drop(_g2);
        // Let other threads run now
        drop(_lock);
    }

    #[test]
    fn test_no_proxy_load() {
        // Stop other threads from modifying process-global ENV while we are.
        let _lock = ENVLOCK.lock();

        let _g1 = env_guard("no_proxy");
        let domain = "lower.case";
        env::set_var("no_proxy", domain);
        // Manually construct this so we aren't use the cache
        let mut p = Proxy::new(Intercept::System(Arc::new(get_sys_proxies(None))));
        p.no_proxy = NoProxy::from_env();
        assert_eq!(
            p.no_proxy.expect("should have a no proxy set").domains.0[0],
            domain
        );

        env::remove_var("no_proxy");
        let _g2 = env_guard("NO_PROXY");
        let domain = "upper.case";
        env::set_var("NO_PROXY", domain);
        // Manually construct this so we aren't use the cache
        let mut p = Proxy::new(Intercept::System(Arc::new(get_sys_proxies(None))));
        p.no_proxy = NoProxy::from_env();
        assert_eq!(
            p.no_proxy.expect("should have a no proxy set").domains.0[0],
            domain
        );

        let _g3 = env_guard("HTTP_PROXY");
        env::remove_var("NO_PROXY");
        env::remove_var("no_proxy");
        let target = "http://example.domain/";
        env::set_var("HTTP_PROXY", target);

        // Manually construct this so we aren't use the cache
        let mut p = Proxy::new(Intercept::System(Arc::new(get_sys_proxies(None))));
        p.no_proxy = NoProxy::from_env();
        assert!(p.no_proxy.is_none(), "NoProxy shouldn't have been created");

        assert_eq!(intercepted_uri(&p, "http://hyper.rs"), target);

        // reset user setting when guards drop
        drop(_g1);
        drop(_g2);
        drop(_g3);
        // Let other threads run now
        drop(_lock);
    }

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    #[test]
    fn test_type_prefix_extraction() {
        assert!(extract_type_prefix("test").is_none());
        assert!(extract_type_prefix("://test").is_none());
        assert!(extract_type_prefix("some:prefix://test").is_none());
        assert!(extract_type_prefix("some/prefix://test").is_none());

        assert_eq!(extract_type_prefix("http://test").unwrap(), "http");
        assert_eq!(extract_type_prefix("a://test").unwrap(), "a");
    }

    /// Guard an environment variable, resetting it to the original value
    /// when dropped.
    fn env_guard(name: impl Into<String>) -> EnvGuard {
        let name = name.into();
        let orig_val = env::var(&name).ok();
        env::remove_var(&name);
        EnvGuard { name, orig_val }
    }

    struct EnvGuard {
        name: String,
        orig_val: Option<String>,
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            if let Some(val) = self.orig_val.take() {
                env::set_var(&self.name, val);
            } else {
                env::remove_var(&self.name);
            }
        }
    }

    #[test]
    fn test_has_http_auth() {
        let http_proxy_with_auth = Proxy {
            intercept: Intercept::Http(ProxyScheme::Http {
                auth: Some(HeaderValue::from_static("auth1")),
                host: http::uri::Authority::from_static("authority"),
            }),
            no_proxy: None,
        };
        assert!(http_proxy_with_auth.maybe_has_http_auth());
        assert_eq!(
            http_proxy_with_auth.http_basic_auth(&Uri::from_static("http://example.com")),
            Some(HeaderValue::from_static("auth1"))
        );

        let http_proxy_without_auth = Proxy {
            intercept: Intercept::Http(ProxyScheme::Http {
                auth: None,
                host: http::uri::Authority::from_static("authority"),
            }),
            no_proxy: None,
        };
        assert!(!http_proxy_without_auth.maybe_has_http_auth());
        assert_eq!(
            http_proxy_without_auth.http_basic_auth(&Uri::from_static("http://example.com")),
            None
        );

        let https_proxy_with_auth = Proxy {
            intercept: Intercept::Http(ProxyScheme::Https {
                auth: Some(HeaderValue::from_static("auth2")),
                host: http::uri::Authority::from_static("authority"),
            }),
            no_proxy: None,
        };
        assert!(https_proxy_with_auth.maybe_has_http_auth());
        assert_eq!(
            https_proxy_with_auth.http_basic_auth(&Uri::from_static("http://example.com")),
            Some(HeaderValue::from_static("auth2"))
        );

        let all_http_proxy_with_auth = Proxy {
            intercept: Intercept::All(ProxyScheme::Http {
                auth: Some(HeaderValue::from_static("auth3")),
                host: http::uri::Authority::from_static("authority"),
            }),
            no_proxy: None,
        };
        assert!(all_http_proxy_with_auth.maybe_has_http_auth());
        assert_eq!(
            all_http_proxy_with_auth.http_basic_auth(&Uri::from_static("http://example.com")),
            Some(HeaderValue::from_static("auth3"))
        );

        let all_https_proxy_with_auth = Proxy {
            intercept: Intercept::All(ProxyScheme::Https {
                auth: Some(HeaderValue::from_static("auth4")),
                host: http::uri::Authority::from_static("authority"),
            }),
            no_proxy: None,
        };
        assert!(all_https_proxy_with_auth.maybe_has_http_auth());
        assert_eq!(
            all_https_proxy_with_auth.http_basic_auth(&Uri::from_static("http://example.com")),
            Some(HeaderValue::from_static("auth4"))
        );

        let all_https_proxy_without_auth = Proxy {
            intercept: Intercept::All(ProxyScheme::Https {
                auth: None,
                host: http::uri::Authority::from_static("authority"),
            }),
            no_proxy: None,
        };
        assert!(!all_https_proxy_without_auth.maybe_has_http_auth());
        assert_eq!(
            all_https_proxy_without_auth.http_basic_auth(&Uri::from_static("http://example.com")),
            None
        );

        let system_http_proxy_with_auth = Proxy {
            intercept: Intercept::System(Arc::new({
                let mut m = HashMap::new();
                m.insert(
                    "http".into(),
                    ProxyScheme::Http {
                        auth: Some(HeaderValue::from_static("auth5")),
                        host: http::uri::Authority::from_static("authority"),
                    },
                );
                m
            })),
            no_proxy: None,
        };
        assert!(system_http_proxy_with_auth.maybe_has_http_auth());
        assert_eq!(
            system_http_proxy_with_auth.http_basic_auth(&Uri::from_static("http://example.com")),
            Some(HeaderValue::from_static("auth5"))
        );

        let system_https_proxy_with_auth = Proxy {
            intercept: Intercept::System(Arc::new({
                let mut m = HashMap::new();
                m.insert(
                    "https".into(),
                    ProxyScheme::Https {
                        auth: Some(HeaderValue::from_static("auth6")),
                        host: http::uri::Authority::from_static("authority"),
                    },
                );
                m
            })),
            no_proxy: None,
        };
        assert!(!system_https_proxy_with_auth.maybe_has_http_auth());
        assert_eq!(
            system_https_proxy_with_auth.http_basic_auth(&Uri::from_static("http://example.com")),
            None
        );
    }
}

#[cfg(test)]
mod test {
    mod into_proxy_scheme {
        use crate::Proxy;
        use std::error::Error;
        use std::mem::discriminant;

        fn includes(haystack: &crate::error::Error, needle: url::ParseError) -> bool {
            let mut source = haystack.source();
            while let Some(error) = source {
                if let Some(parse_error) = error.downcast_ref::<url::ParseError>() {
                    if discriminant(parse_error) == discriminant(&needle) {
                        return true;
                    }
                }
                source = error.source();
            }
            false
        }

        fn check_parse_error(url: &str, needle: url::ParseError) {
            let error = Proxy::http(url).unwrap_err();
            if !includes(&error, needle) {
                panic!("{needle:?} expected; {error:?}, {error} found");
            }
        }

        mod when_scheme_missing {
            mod and_url_is_valid {
                use crate::Proxy;

                #[test]
                fn lookback_works() {
                    let _ = Proxy::http("127.0.0.1").unwrap();
                }

                #[test]
                fn loopback_port_works() {
                    let _ = Proxy::http("127.0.0.1:8080").unwrap();
                }

                #[test]
                fn loopback_username_works() {
                    let _ = Proxy::http("username@127.0.0.1").unwrap();
                }

                #[test]
                fn loopback_username_password_works() {
                    let _ = Proxy::http("username:password@127.0.0.1").unwrap();
                }

                #[test]
                fn loopback_username_password_port_works() {
                    let _ = Proxy::http("ldap%5Cgremlin:pass%3Bword@127.0.0.1:8080").unwrap();
                }

                #[test]
                fn domain_works() {
                    let _ = Proxy::http("proxy.example.com").unwrap();
                }

                #[test]
                fn domain_port_works() {
                    let _ = Proxy::http("proxy.example.com:8080").unwrap();
                }

                #[test]
                fn domain_username_works() {
                    let _ = Proxy::http("username@proxy.example.com").unwrap();
                }

                #[test]
                fn domain_username_password_works() {
                    let _ = Proxy::http("username:password@proxy.example.com").unwrap();
                }

                #[test]
                fn domain_username_password_port_works() {
                    let _ =
                        Proxy::http("ldap%5Cgremlin:pass%3Bword@proxy.example.com:8080").unwrap();
                }
            }
            mod and_url_has_bad {
                use super::super::check_parse_error;

                #[test]
                fn host() {
                    check_parse_error("username@", url::ParseError::RelativeUrlWithoutBase);
                }

                #[test]
                fn idna_encoding() {
                    check_parse_error("xn---", url::ParseError::RelativeUrlWithoutBase);
                }

                #[test]
                fn port() {
                    check_parse_error("127.0.0.1:808080", url::ParseError::RelativeUrlWithoutBase);
                }

                #[test]
                fn ip_v4_address() {
                    check_parse_error("421.627.718.469", url::ParseError::RelativeUrlWithoutBase);
                }

                #[test]
                fn ip_v6_address() {
                    check_parse_error(
                        "[56FE::2159:5BBC::6594]",
                        url::ParseError::RelativeUrlWithoutBase,
                    );
                }

                #[test]
                fn invalid_domain_character() {
                    check_parse_error("abc 123", url::ParseError::RelativeUrlWithoutBase);
                }
            }
        }

        mod when_scheme_present {
            mod and_url_is_valid {
                use crate::Proxy;

                #[test]
                fn loopback_works() {
                    let _ = Proxy::http("http://127.0.0.1").unwrap();
                }

                #[test]
                fn loopback_port_works() {
                    let _ = Proxy::http("https://127.0.0.1:8080").unwrap();
                }

                #[test]
                fn loopback_username_works() {
                    let _ = Proxy::http("http://username@127.0.0.1").unwrap();
                }

                #[test]
                fn loopback_username_password_works() {
                    let _ = Proxy::http("https://username:password@127.0.0.1").unwrap();
                }

                #[test]
                fn loopback_username_password_port_works() {
                    let _ =
                        Proxy::http("http://ldap%5Cgremlin:pass%3Bword@127.0.0.1:8080").unwrap();
                }

                #[test]
                fn domain_works() {
                    let _ = Proxy::http("https://proxy.example.com").unwrap();
                }

                #[test]
                fn domain_port_works() {
                    let _ = Proxy::http("http://proxy.example.com:8080").unwrap();
                }

                #[test]
                fn domain_username_works() {
                    let _ = Proxy::http("https://username@proxy.example.com").unwrap();
                }

                #[test]
                fn domain_username_password_works() {
                    let _ = Proxy::http("http://username:password@proxy.example.com").unwrap();
                }

                #[test]
                fn domain_username_password_port_works() {
                    let _ =
                        Proxy::http("https://ldap%5Cgremlin:pass%3Bword@proxy.example.com:8080")
                            .unwrap();
                }
            }
            mod and_url_has_bad {
                use super::super::check_parse_error;

                #[test]
                fn host() {
                    check_parse_error("http://username@", url::ParseError::EmptyHost);
                }

                #[test]
                fn idna_encoding() {
                    check_parse_error("http://xn---", url::ParseError::IdnaError);
                }

                #[test]
                fn port() {
                    check_parse_error("http://127.0.0.1:808080", url::ParseError::InvalidPort);
                }

                #[test]
                fn ip_v4_address() {
                    check_parse_error(
                        "http://421.627.718.469",
                        url::ParseError::InvalidIpv4Address,
                    );
                }

                #[test]
                fn ip_v6_address() {
                    check_parse_error(
                        "http://[56FE::2159:5BBC::6594]",
                        url::ParseError::InvalidIpv6Address,
                    );
                }

                #[test]
                fn invalid_domain_character() {
                    check_parse_error("http://abc 123/", url::ParseError::InvalidDomainCharacter);
                }
            }
        }
    }
}
