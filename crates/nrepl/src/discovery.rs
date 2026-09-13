//! `.nrepl-port` discovery.
//!
//! When you run `lein repl`, `clj -M:nrepl`, `bb nrepl-server`, or
//! shadow-cljs, the server writes its TCP port (as ASCII decimal) to
//! `.nrepl-port` in the project root. This is the *de facto* way every
//! nREPL editor (CIDER, Calva, Conjure, Cursive) auto-discovers a running
//! server, and it's the only discovery mechanism v1 supports.
//!
//! The file format is just digits, optionally with surrounding whitespace
//! / a trailing newline. We're tolerant of leading/trailing whitespace and
//! reject anything else with a clear error message — silent fallbacks here
//! would manifest as confusing "connection refused" errors later.

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Component, Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context as _, Result, anyhow};
use project::Fs;

/// A successful port-file discovery.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscoveredPort {
    /// Absolute path to the `.nrepl-port` file we read.
    pub port_file: PathBuf,
    /// Parsed TCP port.
    pub port: u16,
}

impl DiscoveredPort {
    /// Builds a `SocketAddr` for an IP address or `localhost`.
    pub fn socket_addr(&self, host: &str) -> Result<SocketAddr> {
        let ip = if let Ok(ip) = host.parse::<IpAddr>() {
            ip
        } else if host.eq_ignore_ascii_case("localhost") {
            IpAddr::V4(Ipv4Addr::LOCALHOST)
        } else {
            return Err(anyhow!(
                "unsupported nREPL host {host:?}; use `localhost` or an IP address"
            ));
        };
        Ok(SocketAddr::new(ip, self.port))
    }
}

/// Reads `<worktree_root>/<port_file>` and parses a port number out of it.
///
/// Returns `Ok(None)` when the file simply doesn't exist (the common case
/// for non-Clojure projects), and `Err` when the file *is* there but isn't
/// a valid port — that's a real configuration error worth surfacing.
pub async fn discover_port_in(
    fs: &Arc<dyn Fs>,
    worktree_root: &Path,
    port_file: &str,
) -> Result<Option<DiscoveredPort>> {
    validate_port_file(port_file)?;
    let path = worktree_root.join(port_file);

    // Distinguish "missing" from "unreadable" before calling `load`, so
    // callers can treat absence as a non-error without swallowing genuine
    // I/O failures.
    if !fs.is_file(&path).await {
        return Ok(None);
    }

    let contents = fs
        .load(&path)
        .await
        .with_context(|| format!("reading {}", path.display()))?;

    let port =
        parse_port(&contents).with_context(|| format!("parsing port from {}", path.display()))?;

    Ok(Some(DiscoveredPort {
        port_file: path,
        port,
    }))
}

fn validate_port_file(port_file: &str) -> Result<()> {
    let path = Path::new(port_file);
    if port_file.is_empty() {
        return Err(anyhow!("nREPL port file path cannot be empty"));
    }
    if path
        .components()
        .any(|component| !matches!(component, Component::Normal(_) | Component::CurDir))
    {
        return Err(anyhow!(
            "nREPL port file must be a relative path within the worktree"
        ));
    }
    Ok(())
}

/// Walks `roots` in order, returning the first `.nrepl-port` we find.
///
/// Order matters: callers typically pass workspace worktrees in
/// user-visible order, and "first match wins" matches what CIDER does for
/// multi-project workspaces.
pub async fn discover_port(
    fs: &Arc<dyn Fs>,
    roots: impl IntoIterator<Item = PathBuf>,
    port_file: &str,
) -> Result<Option<DiscoveredPort>> {
    for root in roots {
        if let Some(found) = discover_port_in(fs, &root, port_file).await? {
            return Ok(Some(found));
        }
    }
    Ok(None)
}

fn parse_port(contents: &str) -> Result<u16> {
    let trimmed = contents.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("port file is empty"));
    }
    // `.nrepl-port` is plain ASCII digits. Reject anything else explicitly
    // rather than letting `parse::<u16>` produce a generic "invalid digit"
    // for inputs like "PORT=12345" that a user might reasonably try.
    if !trimmed.bytes().all(|b| b.is_ascii_digit()) {
        return Err(anyhow!(
            "expected a TCP port (decimal digits only), got {trimmed:?}"
        ));
    }
    let port: u16 = trimmed
        .parse()
        .with_context(|| format!("port {trimmed:?} out of range for u16"))?;
    if port == 0 {
        return Err(anyhow!("port 0 is not a valid nREPL port"));
    }
    Ok(port)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use project::FakeFs;
    use serde_json::json;
    use std::path::Path;

    #[test]
    fn parse_port_accepts_trailing_newline() {
        assert_eq!(parse_port("12345\n").unwrap(), 12345);
        assert_eq!(parse_port("  56789  \n").unwrap(), 56789);
        assert_eq!(parse_port("1").unwrap(), 1);
    }

    #[test]
    fn parse_port_rejects_garbage() {
        assert!(parse_port("").is_err());
        assert!(parse_port("   ").is_err());
        assert!(parse_port("PORT=12345").is_err());
        assert!(parse_port("12345 some-comment").is_err());
        assert!(parse_port("12.34").is_err());
        assert!(parse_port("-1").is_err());
        assert!(parse_port("0").is_err());
        assert!(parse_port("99999").is_err()); // > u16::MAX
    }

    #[gpui::test]
    async fn discover_port_in_returns_none_when_missing(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                "src": { "core.clj": "(ns foo.core)" },
            }),
        )
        .await;

        let fs: Arc<dyn Fs> = fs;
        let result = discover_port_in(&fs, Path::new("/project"), ".nrepl-port")
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    #[gpui::test]
    async fn discover_port_in_rejects_paths_outside_worktree(cx: &mut TestAppContext) {
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());

        assert!(
            discover_port_in(&fs, Path::new("/project"), "../.nrepl-port")
                .await
                .is_err()
        );
        assert!(
            discover_port_in(&fs, Path::new("/project"), "/tmp/.nrepl-port")
                .await
                .is_err()
        );
        assert!(
            discover_port_in(&fs, Path::new("/project"), "")
                .await
                .is_err()
        );
    }

    #[gpui::test]
    async fn discover_port_in_reads_port(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".nrepl-port": "54321\n",
            }),
        )
        .await;

        let fs: Arc<dyn Fs> = fs;
        let found = discover_port_in(&fs, Path::new("/project"), ".nrepl-port")
            .await
            .unwrap()
            .expect("port file should be found");
        assert_eq!(found.port, 54321);
        assert_eq!(found.port_file, Path::new("/project/.nrepl-port"));
    }

    #[gpui::test]
    async fn discover_port_in_propagates_parse_errors(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".nrepl-port": "not-a-port\n",
            }),
        )
        .await;

        let fs: Arc<dyn Fs> = fs;
        let err = discover_port_in(&fs, Path::new("/project"), ".nrepl-port")
            .await
            .unwrap_err();
        // The chain should mention both the path and the offending value
        // so the user can fix the right file.
        let msg = format!("{err:#}");
        assert!(msg.contains(".nrepl-port"), "error missing path: {msg}");
        assert!(msg.contains("not-a-port"), "error missing value: {msg}");
    }

    #[gpui::test]
    async fn discover_port_returns_first_match(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/a",
            json!({
                "src": {},
            }),
        )
        .await;
        fs.insert_tree(
            "/b",
            json!({
                ".nrepl-port": "7777",
            }),
        )
        .await;
        fs.insert_tree(
            "/c",
            json!({
                ".nrepl-port": "8888",
            }),
        )
        .await;

        let fs: Arc<dyn Fs> = fs;
        let found = discover_port(
            &fs,
            [
                PathBuf::from("/a"),
                PathBuf::from("/b"),
                PathBuf::from("/c"),
            ],
            ".nrepl-port",
        )
        .await
        .unwrap()
        .expect("a match exists");
        assert_eq!(found.port, 7777);
        assert_eq!(found.port_file, Path::new("/b/.nrepl-port"));
    }

    #[gpui::test]
    async fn discover_port_returns_none_when_no_root_matches(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/a", json!({})).await;
        fs.insert_tree("/b", json!({})).await;

        let fs: Arc<dyn Fs> = fs;
        let result = discover_port(
            &fs,
            [PathBuf::from("/a"), PathBuf::from("/b")],
            ".nrepl-port",
        )
        .await
        .unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn discovered_port_socket_addr_handles_localhost_and_ips() {
        let p = DiscoveredPort {
            port_file: PathBuf::from("/x/.nrepl-port"),
            port: 1234,
        };
        assert_eq!(
            p.socket_addr("127.0.0.1").expect("valid IPv4 address"),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234),
        );
        assert_eq!(
            p.socket_addr("localhost").expect("localhost is supported"),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1234),
        );
        assert_eq!(
            p.socket_addr("LOCALHOST")
                .expect("localhost is case-insensitive"),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1234),
        );
        assert!(p.socket_addr("example.com").is_err());
    }
}
