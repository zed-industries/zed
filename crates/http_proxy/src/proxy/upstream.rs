//! Upstream HTTP proxy configuration: URL parsing, auth, and `NO_PROXY` rules.
//!
//! When the parent process has `HTTPS_PROXY=http://corp.proxy:3128` (etc.)
//! in its environment, we propagate that to our in-process proxy as an
//! [`UpstreamProxy`]. Outbound connections from our proxy then chain through
//! the corporate one — except for hosts that match `NO_PROXY`, which connect
//! direct.
//!
//! Only HTTP upstream proxies are supported (URL scheme must be `http://`).
//! HTTPS upstream proxies are rare in practice and would require a TLS
//! handshake on our side; out of scope for v1.
//!
//! `NO_PROXY` matching is delegated to the [`proxyvars`] crate, which
//! implements the curl/Go convention (CIDR, hostnames with optional port,
//! leading-dot subdomain rules, `*` wildcard, implicit loopback bypass).

use anyhow::{Context, Result, anyhow, bail};
use proxyvars::NoProxy;
use std::fmt;
use std::sync::Arc;
use url::Url;

/// Upstream HTTP proxy used for chaining outbound connections.
///
/// `Clone` shares the parsed `NoProxy` matcher via `Arc` rather than
/// re-parsing the rule list — cloning is essentially free, and the matcher
/// itself isn't cheap to construct (CIDR / IpNet / wildcard parsing).
#[derive(Clone)]
pub struct UpstreamProxy {
    /// Hostname of the upstream proxy. Could be a hostname or IP literal —
    /// the upstream proxy is in the user's trusted environment, so IP
    /// literals are fine here even though we forbid them in the
    /// agent-facing allowlist.
    pub host: String,
    /// Port of the upstream proxy.
    pub port: u16,
    /// Optional `user:pass` for `Proxy-Authorization: Basic ...`. Forwarded
    /// verbatim when chaining.
    pub auth: Option<UpstreamAuth>,
    /// Hosts that bypass the upstream and connect direct, parsed from
    /// `NO_PROXY` / `no_proxy`. `proxyvars::NoProxy` doesn't impl `Clone`,
    /// so we share via `Arc`.
    no_proxy: Arc<NoProxy>,
}

impl fmt::Debug for UpstreamProxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Include host/port but not creds; the `NoProxy` matcher is opaque
        // and its `Debug` would just enumerate internal rule variants —
        // not useful for our log/error output.
        f.debug_struct("UpstreamProxy")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("auth", &self.auth.as_ref().map(|_| "<redacted>"))
            .finish_non_exhaustive()
    }
}

/// Basic credentials for the upstream proxy.
#[derive(Clone)]
pub struct UpstreamAuth {
    pub user: String,
    pub password: String,
}

impl fmt::Debug for UpstreamAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Don't print credentials, even in Debug output.
        f.debug_struct("UpstreamAuth")
            .field("user", &"<redacted>")
            .field("password", &"<redacted>")
            .finish()
    }
}

impl UpstreamProxy {
    /// Reads the parent process's environment for `HTTPS_PROXY` / `HTTP_PROXY`
    /// / `ALL_PROXY` (and their lowercase forms) plus `NO_PROXY` / `no_proxy`.
    /// Returns `Ok(None)` if no upstream proxy is configured.
    ///
    /// Thin convenience over [`UpstreamProxy::parse`] — kept tiny so the
    /// pure function stays the testable surface.
    pub fn from_env() -> Result<Option<Self>> {
        let url = first_nonempty_env(&[
            "HTTPS_PROXY",
            "https_proxy",
            "ALL_PROXY",
            "all_proxy",
            "HTTP_PROXY",
            "http_proxy",
        ]);
        let no_proxy = first_nonempty_env(&["NO_PROXY", "no_proxy"]);
        Self::parse(url.as_deref(), no_proxy.as_deref())
    }

    /// Pure constructor: parse an `(HTTPS_PROXY, NO_PROXY)` pair into an
    /// `UpstreamProxy`. Returns `Ok(None)` if `https_proxy` is `None` (no
    /// upstream configured); `NO_PROXY` is honored either way for callers
    /// that want to inspect the matcher independently.
    ///
    /// Accepts `http://[user[:pass]@]host[:port][/path]`. Path is ignored;
    /// the port defaults to 80 when omitted. A bare `host:port` (no scheme)
    /// is also accepted as a compatibility concession to common proxy-config
    /// conventions.
    pub fn parse(https_proxy: Option<&str>, no_proxy: Option<&str>) -> Result<Option<Self>> {
        let no_proxy = Arc::new(NoProxy::from(no_proxy.unwrap_or("")));
        let Some(url) = https_proxy else {
            return Ok(None);
        };
        Self::parse_url(url, no_proxy).map(Some)
    }

    fn parse_url(url: &str, no_proxy: Arc<NoProxy>) -> Result<Self> {
        let url = url.trim();
        // `Url::parse` requires a scheme. Add `http://` if missing — the
        // bare-`host:port` form is common in proxy configs and we want to
        // keep accepting it.
        let normalized = if url.contains("://") {
            url.to_string()
        } else {
            format!("http://{url}")
        };

        let parsed = Url::parse(&normalized)
            .with_context(|| format!("parsing upstream proxy url '{url}'"))?;

        if parsed.scheme() != "http" {
            bail!(
                "unsupported upstream proxy scheme '{}://' in '{url}': only http:// is supported",
                parsed.scheme()
            );
        }

        let host = parsed
            .host_str()
            .ok_or_else(|| anyhow!("upstream proxy url '{url}' has no host"))?;
        // Brackets around IPv6 literals are URL syntax, not part of the
        // address; strip them so the host can be handed to
        // `ToSocketAddrs` (which rejects bracketed forms).
        let host = host
            .strip_prefix('[')
            .and_then(|stripped| stripped.strip_suffix(']'))
            .unwrap_or(host)
            .to_string();
        // `Url::port` elides scheme-default ports, so a plain `.port()`
        // can't distinguish `http://proxy:80` from `http://proxy`; treat
        // both as port 80.
        let port = parsed
            .port_or_known_default()
            .ok_or_else(|| anyhow!("upstream proxy url '{url}' must include a port"))?;

        let auth = if parsed.username().is_empty() {
            None
        } else {
            Some(UpstreamAuth {
                // `Url` returns percent-encoded credentials; decode for the
                // wire `Basic` token, which expects the raw `user:pass`.
                user: percent_decode(parsed.username()),
                password: percent_decode(parsed.password().unwrap_or("")),
            })
        };

        Ok(Self {
            host,
            port,
            auth,
            no_proxy,
        })
    }

    /// Whether the given destination should bypass this upstream proxy
    /// (i.e., connect direct because it matches `NO_PROXY`).
    ///
    /// Builds a URL from `host:port` and delegates to [`NoProxy::matches`].
    /// Scheme is fixed to `http://` — `proxyvars` only uses the scheme to
    /// supply a default port, and we always pass an explicit port, so it
    /// doesn't actually affect the decision.
    pub fn bypasses(&self, host: &str, port: u16) -> bool {
        // Callers pass normalized (bracket-free) hosts; IPv6 literals need
        // their brackets back to form a valid URL.
        let url = if host.contains(':') && !host.starts_with('[') {
            format!("http://[{host}]:{port}")
        } else {
            format!("http://{host}:{port}")
        };
        self.no_proxy.matches(&url)
    }
}

fn first_nonempty_env(names: &[&str]) -> Option<String> {
    for name in names {
        if let Ok(value) = std::env::var(name)
            && !value.trim().is_empty()
        {
            return Some(value);
        }
    }
    None
}

fn percent_decode(input: &str) -> String {
    percent_encoding::percent_decode_str(input)
        .decode_utf8_lossy()
        .into_owned()
}

impl fmt::Display for UpstreamProxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Don't print credentials. IPv6 literal hosts (stored bracket-free)
        // get their URL brackets back.
        if self.host.contains(':') {
            write!(f, "http://[{}]:{}", self.host, self.port)
        } else {
            write!(f, "http://{}:{}", self.host, self.port)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(url: &str) -> UpstreamProxy {
        UpstreamProxy::parse(Some(url), None).unwrap().unwrap()
    }

    #[test]
    fn parses_plain_host_port() {
        let p = parse("http://corp.proxy:3128");
        assert_eq!(p.host, "corp.proxy");
        assert_eq!(p.port, 3128);
        assert!(p.auth.is_none());
    }

    #[test]
    fn parses_bare_host_port() {
        let p = parse("corp.proxy:3128");
        assert_eq!(p.host, "corp.proxy");
        assert_eq!(p.port, 3128);
    }

    #[test]
    fn parses_with_basic_auth() {
        let p = parse("http://alice:s3cret@corp.proxy:3128");
        let auth = p.auth.unwrap();
        assert_eq!(auth.user, "alice");
        assert_eq!(auth.password, "s3cret");
    }

    #[test]
    fn parses_percent_encoded_auth() {
        let p = parse("http://alice%40example:p%40ss@corp.proxy:3128");
        let auth = p.auth.unwrap();
        assert_eq!(auth.user, "alice@example");
        assert_eq!(auth.password, "p@ss");
    }

    #[test]
    fn parses_bracketed_ipv6() {
        // The brackets are URL syntax; the stored host must be the bare
        // address so `ToSocketAddrs` can use it.
        let p = parse("http://[::1]:3128");
        assert_eq!(p.host, "::1");
        assert_eq!(p.port, 3128);
        assert_eq!(format!("{p}"), "http://[::1]:3128");
    }

    #[test]
    fn ignores_trailing_path() {
        let p = parse("http://corp.proxy:3128/some/path");
        assert_eq!(p.host, "corp.proxy");
        assert_eq!(p.port, 3128);
    }

    #[test]
    fn rejects_non_http_scheme() {
        let err = UpstreamProxy::parse(Some("https://corp.proxy:3128"), None).unwrap_err();
        assert!(format!("{err}").contains("unsupported upstream proxy scheme"));
    }

    #[test]
    fn defaults_to_port_80() {
        // `Url::port` elides scheme-default ports, so both of these must
        // come out as 80 rather than "missing port" errors.
        assert_eq!(parse("http://corp.proxy").port, 80);
        assert_eq!(parse("http://corp.proxy:80").port, 80);
    }

    #[test]
    fn returns_none_when_https_proxy_is_none() {
        assert!(UpstreamProxy::parse(None, None).unwrap().is_none());
        assert!(
            UpstreamProxy::parse(None, Some("example.com"))
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn display_hides_credentials() {
        let p = parse("http://alice:s3cret@corp.proxy:3128");
        let s = format!("{p}");
        assert!(!s.contains("alice"));
        assert!(!s.contains("s3cret"));
        assert!(s.contains("corp.proxy"));
    }

    #[test]
    fn no_proxy_bypasses_listed_host() {
        let p = UpstreamProxy::parse(Some("http://corp.proxy:3128"), Some("internal.example"))
            .unwrap()
            .unwrap();
        assert!(p.bypasses("internal.example", 443));
        assert!(p.bypasses("api.internal.example", 443));
        assert!(!p.bypasses("github.com", 443));
    }

    #[test]
    fn no_proxy_bypasses_loopback_implicitly() {
        let p = UpstreamProxy::parse(Some("http://corp.proxy:3128"), None)
            .unwrap()
            .unwrap();
        assert!(p.bypasses("127.0.0.1", 80));
        assert!(p.bypasses("[::1]", 80));
    }

    #[test]
    fn no_proxy_with_cidr() {
        let p = UpstreamProxy::parse(Some("http://corp.proxy:3128"), Some("10.0.0.0/8"))
            .unwrap()
            .unwrap();
        assert!(p.bypasses("10.5.4.3", 443));
        assert!(!p.bypasses("11.0.0.1", 443));
    }
}
