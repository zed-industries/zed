//! `cedian://` virtual files (plan §§11, 40: deep context via host URIs).
//!
//! Kinds (Phase 6): `buffer/<path>`, `selection`, `active-file`,
//! `diagnostics[/<path>]`, `open-editors`. Later phases add `browser/current`,
//! `ios/current`, `review/current` — unknown kinds fail with `isError`, never
//! silent empty. Reserved OMP schemes are rejected at parse (never shadowed).
//! OMP `edit` does NOT target host URIs (upstream) — writes route via host tools.

/// Reserved OMP built-in schemes — never registered, never shadowed.
pub const RESERVED_SCHEMES: &[&str] = &["local", "skill", "artifact", "security", "mcp"];

/// Known `cedian://` kinds. Unknown kind strings parse fine (forward-compat)
/// but fail at serve time with a visible error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UriKind {
    Buffer,
    Selection,
    ActiveFile,
    Diagnostics,
    OpenEditors,
    /// LSP symbols: `cedian://symbols/<query>` (workspace) or
    /// `cedian://symbols/file/<path>` (document). Served by the LSP bridge.
    Symbols,
    /// `cedian://definitions/<path>:<line>:<column>` (0-based, as LSP):
    /// where the symbol there is defined. Served by the app (ADR-0048).
    Definitions,
    /// `cedian://references/<path>:<line>:<column>` (0-based, as LSP):
    /// every use of the symbol there. Served by the app (ADR-0048).
    References,
    /// `cedian://corrections`: the correction ledger (ADR-0032). No path.
    Corrections,
    Unknown(String),
}

impl UriKind {
    /// Parse a kind segment.
    pub fn parse(kind: &str) -> Self {
        match kind {
            "buffer" => Self::Buffer,
            "selection" => Self::Selection,
            "active-file" => Self::ActiveFile,
            "diagnostics" => Self::Diagnostics,
            "open-editors" => Self::OpenEditors,
            "symbols" => Self::Symbols,
            "definitions" => Self::Definitions,
            "references" => Self::References,
            "corrections" => Self::Corrections,
            other => Self::Unknown(other.to_string()),
        }
    }
}

/// A parsed `cedian://` URL: kind + path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CedianUri {
    /// Typed kind (unknown kinds preserved, fail at serve).
    pub kind: UriKind,
    /// Path within the kind namespace (`""` for bare kinds like `selection`).
    pub path: String,
}

/// URI failures (host answers `isError` on these).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UriError {
    /// Not a `cedian://` URL.
    WrongScheme { got: String },
    /// Missing kind segment.
    BadShape { url: String },
    /// Reserved OMP scheme — refused, never shadowed.
    Reserved { scheme: String },
}

impl std::fmt::Display for UriError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongScheme { got } => write!(f, "not a cedian:// URL: {got}"),
            Self::BadShape { url } => write!(f, "bad cedian:// shape: {url}"),
            Self::Reserved { scheme } => write!(f, "scheme reserved by OMP: {scheme}://"),
        }
    }
}

impl std::error::Error for UriError {}

/// Parse a URL into a [`CedianUri`]. Accepts `cedian://kind[/path]`; bare kinds
/// (`selection`, `active-file`, `open-editors`) carry an empty path.
pub fn parse_cedian_uri(url: &str) -> Result<CedianUri, UriError> {
    let (scheme, rest) = url.split_once("://").ok_or_else(|| UriError::BadShape {
        url: url.to_string(),
    })?;
    let scheme = scheme.to_lowercase();
    if RESERVED_SCHEMES.contains(&scheme.as_str()) {
        return Err(UriError::Reserved { scheme });
    }
    if scheme != "cedian" {
        return Err(UriError::WrongScheme { got: scheme });
    }
    if rest.is_empty() {
        return Err(UriError::BadShape {
            url: url.to_string(),
        });
    }
    let (kind, path) = match rest.split_once('/') {
        Some((kind, path)) => (kind, path.to_string()),
        None => (rest, String::new()),
    };
    if kind.is_empty() {
        return Err(UriError::BadShape {
            url: url.to_string(),
        });
    }
    Ok(CedianUri {
        kind: UriKind::parse(kind),
        path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_url_parses() {
        let u = parse_cedian_uri("cedian://buffer/src/main.rs").unwrap();
        assert_eq!(u.kind, UriKind::Buffer);
        assert_eq!(u.path, "src/main.rs");
    }

    #[test]
    fn bare_kinds_parse_empty_path() {
        for (url, kind) in [
            ("cedian://selection", UriKind::Selection),
            ("cedian://active-file", UriKind::ActiveFile),
            ("cedian://open-editors", UriKind::OpenEditors),
            ("cedian://diagnostics", UriKind::Diagnostics),
            ("cedian://corrections", UriKind::Corrections),
        ] {
            let u = parse_cedian_uri(url).unwrap();
            assert_eq!(u.kind, kind, "{url}");
            assert!(u.path.is_empty());
        }
        let u = parse_cedian_uri("cedian://diagnostics/src/a.rs").unwrap();
        assert_eq!(u.kind, UriKind::Diagnostics);
    }

    #[test]
    fn unknown_kind_preserved() {
        let u = parse_cedian_uri("cedian://browser/current").unwrap();
        assert_eq!(u.kind, UriKind::Unknown("browser".to_string()));
    }

    #[test]
    fn reserved_never_shadowed() {
        for scheme in ["local", "skill", "artifact", "security", "mcp"] {
            let err = parse_cedian_uri(&format!("{scheme}://x/y")).unwrap_err();
            assert_eq!(
                err,
                UriError::Reserved {
                    scheme: scheme.to_string()
                }
            );
        }
    }

    #[test]
    fn wrong_scheme_and_shape_rejected() {
        assert!(matches!(
            parse_cedian_uri("db://x/y"),
            Err(UriError::WrongScheme { .. })
        ));
        assert!(matches!(
            parse_cedian_uri("cedian://"),
            Err(UriError::BadShape { .. })
        ));
    }
}
