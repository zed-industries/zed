mod no_proxy;

pub use no_proxy::NoProxy;

/// Fetches the environment variable `HTTP_PROXY` or 'http_proxy'.
/// Empty strings are ignored and return `None`.
pub fn http() -> Option<String> {
    invariant_var("HTTP_PROXY")
}

/// Fetches the environment variable `HTTPS_PROXY` or 'https_proxy'.
/// Empty strings are ignored and return `None`.
pub fn https() -> Option<String> {
    invariant_var("HTTPS_PROXY")
}

/// Fetches and parse the environment variable `NO_PROXY` or 'no_proxy'.
///
/// ```
/// if let Some(no_proxy) = proxyvars::no_proxy() {
///     // This environment has NO_PROXY defined
///     if no_proxy.matches("https://company.com") {
///         // We should not use a proxy for this URL
///     }
/// }
///
/// ```
pub fn no_proxy() -> Option<NoProxy> {
    invariant_var("NO_PROXY").map(NoProxy::from)
}

fn invariant_var(name: &str) -> Option<String> {
    let nonempty = |o: Option<String>| o.filter(|s| !s.is_empty());

    nonempty(std::env::var(name).ok()).or_else(|| nonempty(std::env::var(name.to_lowercase()).ok()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn undefined_var() {
        std::env::remove_var("HTTP_PROXY");
        std::env::remove_var("http_proxy");
        std::env::remove_var("https_proxy");
        std::env::remove_var("HTTPS_PROXY");

        let http_proxy = crate::http();
        assert!(http_proxy.is_none());

        let https_proxy = crate::https();
        assert!(https_proxy.is_none());
    }

    #[test]
    fn defined_uppercase_vars() {
        std::env::set_var("HTTP_PROXY", "http://proxy.example.com:8080");
        std::env::set_var("HTTPS_PROXY", "https://proxy.example.com:4433");

        let http_proxy = crate::http();
        assert_eq!(http_proxy, Some("http://proxy.example.com:8080".into()));

        let https_proxy = crate::https();
        assert_eq!(https_proxy, Some("https://proxy.example.com:4433".into()));
    }

    #[test]
    fn defined_lowercase_vars() {
        std::env::set_var("http_proxy", "http://proxy.example.com:8080");
        std::env::set_var("https_proxy", "https://proxy.example.com:4433");

        let http_proxy = crate::http();
        assert_eq!(http_proxy, Some("http://proxy.example.com:8080".into()));

        let https_proxy = crate::https();
        assert_eq!(https_proxy, Some("https://proxy.example.com:4433".into()));
    }
}
