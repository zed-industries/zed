/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Proxy configuration for HTTP clients
//!
//! This module provides types and utilities for configuring HTTP and HTTPS proxies,
//! including support for environment variable detection, authentication, and bypass rules.

use http_1x::Uri;
use hyper_util::client::proxy::matcher::Matcher;
use std::fmt;

/// Proxy configuration for HTTP clients
///
/// Supports HTTP and HTTPS proxy configuration with authentication and bypass rules.
/// Can be configured programmatically or automatically detected from environment variables.
///
/// # Examples
///
/// ```rust
/// use aws_smithy_http_client::proxy::ProxyConfig;
///
/// // HTTP proxy for all traffic
/// let config = ProxyConfig::http("http://proxy.example.com:8080")?;
///
/// // HTTPS traffic through HTTP proxy (common case - no TLS needed for proxy connection)
/// let config = ProxyConfig::https("http://proxy.example.com:8080")?
///     .with_basic_auth("username", "password")
///     .no_proxy("localhost,*.internal");
///
/// // Detect from environment variables
/// let config = ProxyConfig::from_env();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    inner: ProxyConfigInner,
}

/// Internal configuration representation
#[derive(Debug, Clone)]
enum ProxyConfigInner {
    /// Use environment variable detection
    FromEnvironment,
    /// Explicit HTTP proxy
    Http {
        uri: Uri,
        auth: Option<ProxyAuth>,
        no_proxy: Option<String>,
    },
    /// Explicit HTTPS proxy
    Https {
        uri: Uri,
        auth: Option<ProxyAuth>,
        no_proxy: Option<String>,
    },
    /// Proxy for all traffic
    All {
        uri: Uri,
        auth: Option<ProxyAuth>,
        no_proxy: Option<String>,
    },
    /// Explicitly disabled
    Disabled,
}

/// Proxy authentication configuration
///
/// Stored for later conversion to hyper-util format.
#[derive(Debug, Clone)]
struct ProxyAuth {
    /// Username for authentication
    username: String,
    /// Password for authentication
    password: String,
}

/// Errors that can occur during proxy configuration
#[derive(Debug)]
pub struct ProxyError {
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    InvalidUrl(String),
}

impl From<ErrorKind> for ProxyError {
    fn from(value: ErrorKind) -> Self {
        Self { kind: value }
    }
}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::InvalidUrl(url) => write!(f, "invalid proxy URL: {url}"),
        }
    }
}

impl std::error::Error for ProxyError {}

impl ProxyConfig {
    /// Create a new proxy configuration for HTTP traffic only
    ///
    /// # Arguments
    /// * `proxy_url` - The HTTP proxy URL
    ///
    /// # Examples
    /// ```rust
    /// use aws_smithy_http_client::proxy::ProxyConfig;
    ///
    /// let config = ProxyConfig::http("http://proxy.example.com:8080")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn http<U>(proxy_url: U) -> Result<Self, ProxyError>
    where
        U: TryInto<Uri>,
        U::Error: fmt::Display,
    {
        let uri = proxy_url
            .try_into()
            .map_err(|e| ErrorKind::InvalidUrl(e.to_string()))?;

        Self::validate_proxy_uri(&uri)?;

        Ok(ProxyConfig {
            inner: ProxyConfigInner::Http {
                uri,
                auth: None,
                no_proxy: None,
            },
        })
    }

    /// Create a new proxy configuration for HTTPS traffic only
    ///
    /// This proxy will only be used for `https://` requests. HTTP requests
    /// will connect directly unless a separate HTTP proxy is configured.
    ///
    /// The proxy URL itself can use either HTTP or HTTPS scheme:
    /// - `http://proxy.example.com:8080` - Connect to proxy using HTTP (no TLS needed)
    /// - `https://proxy.example.com:8080` - Connect to proxy using HTTPS (TLS required)
    ///
    /// **Note**: If the proxy URL itself uses HTTPS scheme, TLS support must be
    /// available when building the connector, otherwise connections will fail.
    ///
    /// # Arguments
    /// * `proxy_url` - The proxy URL
    ///
    /// # Examples
    /// ```rust
    /// use aws_smithy_http_client::proxy::ProxyConfig;
    ///
    /// // HTTPS traffic through HTTP proxy (no TLS needed for proxy connection)
    /// let config = ProxyConfig::https("http://proxy.example.com:8080")?;
    ///
    /// // HTTPS traffic through HTTPS proxy (TLS needed for proxy connection)
    /// let config = ProxyConfig::https("https://secure-proxy.example.com:8080")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn https<U>(proxy_url: U) -> Result<Self, ProxyError>
    where
        U: TryInto<Uri>,
        U::Error: fmt::Display,
    {
        let uri = proxy_url
            .try_into()
            .map_err(|e| ErrorKind::InvalidUrl(e.to_string()))?;

        Self::validate_proxy_uri(&uri)?;

        Ok(ProxyConfig {
            inner: ProxyConfigInner::Https {
                uri,
                auth: None,
                no_proxy: None,
            },
        })
    }

    /// Create a new proxy configuration for all HTTP and HTTPS traffic
    ///
    /// This proxy will be used for both `http://` and `https://` requests.
    /// This is equivalent to setting both HTTP and HTTPS proxies to the same URL.
    ///
    /// **Note**: If the proxy URL itself uses HTTPS scheme, TLS support must be
    /// available when building the connector, otherwise connections will fail.
    ///
    /// # Arguments
    /// * `proxy_url` - The proxy URL
    ///
    /// # Examples
    /// ```rust
    /// use aws_smithy_http_client::proxy::ProxyConfig;
    ///
    /// let config = ProxyConfig::all("http://proxy.example.com:8080")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn all<U>(proxy_url: U) -> Result<Self, ProxyError>
    where
        U: TryInto<Uri>,
        U::Error: fmt::Display,
    {
        let uri = proxy_url
            .try_into()
            .map_err(|e| ErrorKind::InvalidUrl(e.to_string()))?;

        Self::validate_proxy_uri(&uri)?;

        Ok(ProxyConfig {
            inner: ProxyConfigInner::All {
                uri,
                auth: None,
                no_proxy: None,
            },
        })
    }

    /// Create a proxy configuration that disables all proxy usage
    ///
    /// This is useful for explicitly disabling proxy support even when
    /// environment variables are set.
    ///
    /// # Examples
    /// ```rust
    /// use aws_smithy_http_client::proxy::ProxyConfig;
    ///
    /// let config = ProxyConfig::disabled();
    /// ```
    pub fn disabled() -> Self {
        ProxyConfig {
            inner: ProxyConfigInner::Disabled,
        }
    }

    /// Add basic authentication to this proxy configuration
    ///
    /// # Arguments
    /// * `username` - Username for proxy authentication
    /// * `password` - Password for proxy authentication
    ///
    /// # Examples
    /// ```rust
    /// use aws_smithy_http_client::proxy::ProxyConfig;
    ///
    /// let config = ProxyConfig::http("http://proxy.example.com:8080")?
    ///     .with_basic_auth("username", "password");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn with_basic_auth<U, P>(mut self, username: U, password: P) -> Self
    where
        U: Into<String>,
        P: Into<String>,
    {
        let auth = ProxyAuth {
            username: username.into(),
            password: password.into(),
        };

        match &mut self.inner {
            ProxyConfigInner::Http {
                auth: ref mut a, ..
            } => *a = Some(auth),
            ProxyConfigInner::Https {
                auth: ref mut a, ..
            } => *a = Some(auth),
            ProxyConfigInner::All {
                auth: ref mut a, ..
            } => *a = Some(auth),
            ProxyConfigInner::FromEnvironment | ProxyConfigInner::Disabled => {
                // Cannot add auth to environment or disabled configs
            }
        }

        self
    }

    /// Add NO_PROXY rules to this configuration
    ///
    /// NO_PROXY rules specify hosts that should bypass the proxy and connect directly.
    ///
    /// # Arguments
    /// * `rules` - Comma-separated list of bypass rules
    ///
    /// # Examples
    /// ```rust
    /// use aws_smithy_http_client::proxy::ProxyConfig;
    ///
    /// let config = ProxyConfig::http("http://proxy.example.com:8080")?
    ///     .no_proxy("localhost,127.0.0.1,*.internal,10.0.0.0/8");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn no_proxy<S: AsRef<str>>(mut self, rules: S) -> Self {
        let rules_str = rules.as_ref().to_string();

        match &mut self.inner {
            ProxyConfigInner::Http {
                no_proxy: ref mut n,
                ..
            } => *n = Some(rules_str),
            ProxyConfigInner::Https {
                no_proxy: ref mut n,
                ..
            } => *n = Some(rules_str),
            ProxyConfigInner::All {
                no_proxy: ref mut n,
                ..
            } => *n = Some(rules_str),
            ProxyConfigInner::FromEnvironment | ProxyConfigInner::Disabled => {
                // Cannot add no_proxy to environment or disabled configs
                // Environment configs will use NO_PROXY env var
                // FIXME - is this what we want?
            }
        }

        self
    }

    /// Create proxy configuration from environment variables
    ///
    /// Reads standard proxy environment variables:
    /// - `HTTP_PROXY` / `http_proxy`: HTTP proxy URL
    /// - `HTTPS_PROXY` / `https_proxy`: HTTPS proxy URL
    /// - `ALL_PROXY` / `all_proxy`: Proxy for all protocols (fallback)
    /// - `NO_PROXY` / `no_proxy`: Comma-separated bypass rules
    ///
    /// If no proxy environment variables are set, this returns a configuration
    /// that won't intercept any requests (equivalent to no proxy).
    ///
    /// # Examples
    /// ```rust
    /// use aws_smithy_http_client::proxy::ProxyConfig;
    ///
    /// // Always succeeds, even if no environment variables are set
    /// let config = ProxyConfig::from_env();
    /// ```
    pub fn from_env() -> Self {
        // Delegate to environment variable parsing
        // If no env vars are set, creates a matcher that doesn't intercept anything
        ProxyConfig {
            inner: ProxyConfigInner::FromEnvironment,
        }
    }

    /// Check if proxy is disabled (no proxy configuration)
    pub fn is_disabled(&self) -> bool {
        matches!(self.inner, ProxyConfigInner::Disabled)
    }

    /// Check if this configuration uses environment variables
    pub fn is_from_env(&self) -> bool {
        matches!(self.inner, ProxyConfigInner::FromEnvironment)
    }

    /// Convert this configuration to internal proxy matcher
    ///
    /// This method converts the user-friendly configuration to the internal
    /// proxy matching implementation used by the HTTP client.
    pub(crate) fn into_hyper_util_matcher(self) -> Matcher {
        match self.inner {
            ProxyConfigInner::FromEnvironment => Matcher::from_env(),
            ProxyConfigInner::Http {
                uri,
                auth,
                no_proxy,
            } => {
                let mut builder = Matcher::builder();

                // Set HTTP proxy with authentication embedded in URL if present
                let proxy_url = Self::build_proxy_url(uri, auth);
                builder = builder.http(proxy_url);

                // Add NO_PROXY rules if present
                if let Some(no_proxy_rules) = no_proxy {
                    builder = builder.no(no_proxy_rules);
                }

                builder.build()
            }
            ProxyConfigInner::Https {
                uri,
                auth,
                no_proxy,
            } => {
                let mut builder = Matcher::builder();

                // Set HTTPS proxy with authentication embedded in URL if present
                let proxy_url = Self::build_proxy_url(uri, auth);
                builder = builder.https(proxy_url);

                // Add NO_PROXY rules if present
                if let Some(no_proxy_rules) = no_proxy {
                    builder = builder.no(no_proxy_rules);
                }

                builder.build()
            }
            ProxyConfigInner::All {
                uri,
                auth,
                no_proxy,
            } => {
                let mut builder = Matcher::builder();

                // Set proxy for all traffic with authentication embedded in URL if present
                let proxy_url = Self::build_proxy_url(uri, auth);
                builder = builder.all(proxy_url);

                // Add NO_PROXY rules if present
                if let Some(no_proxy_rules) = no_proxy {
                    builder = builder.no(no_proxy_rules);
                }

                builder.build()
            }
            ProxyConfigInner::Disabled => {
                // Create an empty matcher that won't intercept anything
                Matcher::builder().build()
            }
        }
    }

    /// Check if this proxy configuration requires TLS support
    ///
    /// Returns true if any of the configured proxy URLs use HTTPS scheme,
    /// which requires TLS to establish the connection to the proxy server.
    pub(crate) fn requires_tls(&self) -> bool {
        match &self.inner {
            ProxyConfigInner::Http { uri, .. } => uri.scheme_str() == Some("https"),
            ProxyConfigInner::Https { uri, .. } => uri.scheme_str() == Some("https"),
            ProxyConfigInner::All { uri, .. } => uri.scheme_str() == Some("https"),
            ProxyConfigInner::FromEnvironment => {
                // Check environment variables for HTTPS proxy URLs
                Self::env_vars_require_tls()
            }
            ProxyConfigInner::Disabled => false,
        }
    }

    /// Check if any environment proxy variables contain HTTPS URLs
    fn env_vars_require_tls() -> bool {
        let proxy_vars = [
            "HTTP_PROXY",
            "http_proxy",
            "HTTPS_PROXY",
            "https_proxy",
            "ALL_PROXY",
            "all_proxy",
        ];

        for var in &proxy_vars {
            if let Ok(proxy_url) = std::env::var(var) {
                if !proxy_url.is_empty() {
                    // Simple check for https:// scheme
                    if proxy_url.starts_with("https://") {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn validate_proxy_uri(uri: &Uri) -> Result<(), ProxyError> {
        // Validate scheme
        match uri.scheme_str() {
            Some("http") | Some("https") => {}
            Some(scheme) => {
                return Err(
                    ErrorKind::InvalidUrl(format!("unsupported proxy scheme: {scheme}")).into(),
                );
            }
            None => {
                return Err(ErrorKind::InvalidUrl(
                    "proxy URL must include scheme (http:// or https://)".to_string(),
                )
                .into());
            }
        }

        // Validate host
        if uri.host().is_none() {
            return Err(ErrorKind::InvalidUrl("proxy URL must include host".to_string()).into());
        }

        Ok(())
    }

    fn build_proxy_url(uri: Uri, auth: Option<ProxyAuth>) -> String {
        let uri_str = uri.to_string();

        if let Some(auth) = auth {
            // Embed authentication in the URL: scheme://username:password@host:port/path
            if let Some(scheme_end) = uri_str.find("://") {
                let scheme = &uri_str[..scheme_end + 3];
                let rest = &uri_str[scheme_end + 3..];

                // Check if auth is already present in the URI
                if rest.contains('@') {
                    // Auth already present, return as-is
                    uri_str
                } else {
                    // Add auth to the URI
                    format!("{}{}:{}@{}", scheme, auth.username, auth.password, rest)
                }
            } else {
                // Invalid URI format, return as-is
                uri_str
            }
        } else {
            // No authentication, return URI as-is
            uri_str
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_proxy_config_http() {
        let config = ProxyConfig::http("http://proxy.example.com:8080").unwrap();
        assert!(!config.is_disabled());
        assert!(!config.is_from_env());
    }

    #[test]
    fn test_proxy_config_https() {
        let config = ProxyConfig::https("http://proxy.example.com:8080").unwrap();
        assert!(!config.is_disabled());
        assert!(!config.is_from_env());
    }

    #[test]
    fn test_proxy_config_all() {
        let config = ProxyConfig::all("http://proxy.example.com:8080").unwrap();
        assert!(!config.is_disabled());
        assert!(!config.is_from_env());
    }

    #[test]
    fn test_proxy_config_disabled() {
        let config = ProxyConfig::disabled();
        assert!(config.is_disabled());
        assert!(!config.is_from_env());
    }

    #[test]
    fn test_proxy_config_with_auth() {
        let config = ProxyConfig::http("http://proxy.example.com:8080")
            .unwrap()
            .with_basic_auth("user", "pass");

        // Auth is stored internally
        assert!(!config.is_disabled());
    }

    #[test]
    fn test_proxy_config_with_no_proxy() {
        let config = ProxyConfig::http("http://proxy.example.com:8080")
            .unwrap()
            .no_proxy("localhost,*.internal");

        // NO_PROXY rules are stored internally
        assert!(!config.is_disabled());
    }

    #[test]
    fn test_proxy_config_invalid_url() {
        let result = ProxyConfig::http("not-a-url");
        assert!(result.is_err());
    }

    #[test]
    fn test_proxy_config_invalid_scheme() {
        let result = ProxyConfig::http("ftp://proxy.example.com:8080");
        assert!(result.is_err());
    }

    #[test]
    #[serial_test::serial]
    fn test_proxy_config_from_env_with_vars() {
        // Save original environment
        let original_http = env::var("HTTP_PROXY");

        // Set test environment
        env::set_var("HTTP_PROXY", "http://test-proxy:8080");

        let config = ProxyConfig::from_env();
        assert!(config.is_from_env());

        // Restore original environment
        match original_http {
            Ok(val) => env::set_var("HTTP_PROXY", val),
            Err(_) => env::remove_var("HTTP_PROXY"),
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_proxy_config_from_env_without_vars() {
        // Save original environment
        let original_vars: Vec<_> = [
            "HTTP_PROXY",
            "http_proxy",
            "HTTPS_PROXY",
            "https_proxy",
            "ALL_PROXY",
            "all_proxy",
        ]
        .iter()
        .map(|var| (*var, env::var(var)))
        .collect();

        // Clear all proxy environment variables
        for (var, _) in &original_vars {
            env::remove_var(var);
        }

        let config = ProxyConfig::from_env();
        assert!(config.is_from_env());

        // Restore original environment
        for (var, original_value) in original_vars {
            match original_value {
                Ok(val) => env::set_var(var, val),
                Err(_) => env::remove_var(var),
            }
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_auth_cannot_be_added_to_env_config() {
        // Save original environment
        let original_http = env::var("HTTP_PROXY");
        env::set_var("HTTP_PROXY", "http://test-proxy:8080");

        let config = ProxyConfig::from_env().with_basic_auth("user", "pass"); // This should be ignored

        assert!(config.is_from_env());

        // Restore original environment
        match original_http {
            Ok(val) => env::set_var("HTTP_PROXY", val),
            Err(_) => env::remove_var("HTTP_PROXY"),
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_no_proxy_cannot_be_added_to_env_config() {
        // Save original environment
        let original_http = env::var("HTTP_PROXY");
        env::set_var("HTTP_PROXY", "http://test-proxy:8080");

        let config = ProxyConfig::from_env().no_proxy("localhost"); // This should be ignored

        assert!(config.is_from_env());

        // Restore original environment
        match original_http {
            Ok(val) => env::set_var("HTTP_PROXY", val),
            Err(_) => env::remove_var("HTTP_PROXY"),
        }
    }

    #[test]
    fn test_build_proxy_url_without_auth() {
        let uri = "http://proxy.example.com:8080".parse().unwrap();
        let url = ProxyConfig::build_proxy_url(uri, None);
        assert_eq!(url, "http://proxy.example.com:8080/");
    }

    #[test]
    fn test_build_proxy_url_with_auth() {
        let uri = "http://proxy.example.com:8080".parse().unwrap();
        let auth = ProxyAuth {
            username: "user".to_string(),
            password: "pass".to_string(),
        };
        let url = ProxyConfig::build_proxy_url(uri, Some(auth));
        assert_eq!(url, "http://user:pass@proxy.example.com:8080/");
    }

    #[test]
    fn test_build_proxy_url_with_existing_auth() {
        let uri = "http://existing:creds@proxy.example.com:8080"
            .parse()
            .unwrap();
        let auth = ProxyAuth {
            username: "user".to_string(),
            password: "pass".to_string(),
        };
        let url = ProxyConfig::build_proxy_url(uri, Some(auth));
        // Should not override existing auth
        assert_eq!(url, "http://existing:creds@proxy.example.com:8080/");
    }

    #[test]
    #[serial_test::serial]
    fn test_into_hyper_util_matcher_from_env() {
        // Save original environment
        let original_http = env::var("HTTP_PROXY");
        env::set_var("HTTP_PROXY", "http://test-proxy:8080");

        let config = ProxyConfig::from_env();
        let matcher = config.into_hyper_util_matcher();

        // Test that the matcher intercepts HTTP requests
        let test_uri = "http://example.com".parse().unwrap();
        let intercept = matcher.intercept(&test_uri);
        assert!(intercept.is_some());

        // Restore original environment
        match original_http {
            Ok(val) => env::set_var("HTTP_PROXY", val),
            Err(_) => env::remove_var("HTTP_PROXY"),
        }
    }

    #[test]
    fn test_into_hyper_util_matcher_http() {
        let config = ProxyConfig::http("http://proxy.example.com:8080").unwrap();
        let matcher = config.into_hyper_util_matcher();

        // Test that the matcher intercepts HTTP requests
        let test_uri = "http://example.com".parse().unwrap();
        let intercept = matcher.intercept(&test_uri);
        assert!(intercept.is_some());
        // The intercept URI might be normalized
        assert!(intercept
            .unwrap()
            .uri()
            .to_string()
            .starts_with("http://proxy.example.com:8080"));

        // Test that it doesn't intercept HTTPS requests
        let https_uri = "https://example.com".parse().unwrap();
        let https_intercept = matcher.intercept(&https_uri);
        assert!(https_intercept.is_none());
    }

    #[test]
    fn test_into_hyper_util_matcher_with_auth() {
        let config = ProxyConfig::http("http://proxy.example.com:8080")
            .unwrap()
            .with_basic_auth("user", "pass");
        let matcher = config.into_hyper_util_matcher();

        // Test that the matcher intercepts HTTP requests
        let test_uri = "http://example.com".parse().unwrap();
        let intercept = matcher.intercept(&test_uri);
        assert!(intercept.is_some());

        let intercept = intercept.unwrap();
        // The proxy URI should contain the host (auth is handled separately)
        assert!(intercept
            .uri()
            .to_string()
            .contains("proxy.example.com:8080"));

        // Test that basic auth is available
        assert!(intercept.basic_auth().is_some());
    }

    #[test]
    fn test_into_hyper_util_matcher_disabled() {
        let config = ProxyConfig::disabled();
        let matcher = config.into_hyper_util_matcher();

        // Test that the matcher doesn't intercept any requests
        let test_uri = "http://example.com".parse().unwrap();
        let intercept = matcher.intercept(&test_uri);
        assert!(intercept.is_none());
    }

    #[test]
    #[serial_test::serial]
    fn test_requires_tls_detection() {
        // HTTP proxy should not require TLS
        let http_config = ProxyConfig::http("http://proxy.example.com:8080").unwrap();
        assert!(!http_config.requires_tls());

        // HTTPS proxy URL should require TLS
        let https_config = ProxyConfig::http("https://proxy.example.com:8080").unwrap();
        assert!(https_config.requires_tls());

        // All proxy with HTTP URL should not require TLS
        let all_http_config = ProxyConfig::all("http://proxy.example.com:8080").unwrap();
        assert!(!all_http_config.requires_tls());

        // Environment config with HTTPS proxy should require TLS
        env::set_var("HTTP_PROXY", "https://proxy.example.com:8080");
        let env_config = ProxyConfig::from_env();
        assert!(env_config.requires_tls()); // Now detects HTTPS in env vars
        env::remove_var("HTTP_PROXY");

        // Environment config with HTTP proxy should not require TLS
        env::set_var("HTTP_PROXY", "http://proxy.example.com:8080");
        let env_config = ProxyConfig::from_env();
        assert!(!env_config.requires_tls());
        env::remove_var("HTTP_PROXY");

        // Disabled config should not require TLS
        let disabled_config = ProxyConfig::disabled();
        assert!(!disabled_config.requires_tls());
    }
}
