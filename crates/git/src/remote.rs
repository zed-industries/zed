use std::str::FromStr;
use std::sync::LazyLock;

use derive_more::Deref;
use regex::Regex;
use url::Url;

/// The URL to a Git remote.
#[derive(Debug, PartialEq, Eq, Clone, Deref)]
pub struct RemoteUrl(Url);

// Detect the `user@` prefix of an SCP-like remote (e.g. `git@host:path`). The
// username may contain anything but the `@`/`:`/`/` that delimit the user,
// host, and path, so match by exclusion rather than an allowlist that misses
// names like `first.last`.
static USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[^/@:]+@").expect("Failed to create USERNAME_REGEX"));

impl FromStr for RemoteUrl {
    type Err = url::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if USERNAME_REGEX.is_match(input) {
            // Rewrite remote URLs like `git@github.com:user/repo.git` to `ssh://git@github.com/user/repo.git`
            let ssh_url = format!("ssh://{}", input.replacen(':', "/", 1));
            Ok(RemoteUrl(Url::parse(&ssh_url)?))
        } else {
            Ok(RemoteUrl(Url::parse(input)?))
        }
    }
}

/// Normalize a user-entered Git remote URL. Recognizable bare `host/path` URLs
/// are prefixed with `https://`, while URLs with a scheme, SCP-like remotes, and
/// local paths are returned unchanged.
pub fn normalize_remote_url(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.parse::<RemoteUrl>().is_ok() {
        return trimmed.to_string();
    }

    if let Some((host, path)) = trimmed.split_once(':')
        && !host.is_empty()
        && !path.is_empty()
        && !host.contains('/')
        && !host.contains('\\')
    {
        return trimmed.to_string();
    }

    let Some((host, path)) = trimmed.split_once('/') else {
        return trimmed.to_string();
    };
    let is_host = host.eq_ignore_ascii_case("localhost")
        || host.contains('.') && host.split('.').all(|label| !label.is_empty());
    if path.is_empty() || !is_host {
        return trimmed.to_string();
    }

    let normalized = format!("https://{trimmed}");
    if normalized.parse::<RemoteUrl>().is_ok() {
        normalized
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parsing_valid_remote_urls() {
        let valid_urls = vec![
            (
                "https://github.com/octocat/zed.git",
                "https",
                "github.com",
                "/octocat/zed.git",
            ),
            (
                "https://jlannister@github.com/octocat/zed.git",
                "https",
                "github.com",
                "/octocat/zed.git",
            ),
            (
                "git@github.com:octocat/zed.git",
                "ssh",
                "github.com",
                "/octocat/zed.git",
            ),
            (
                "org-000000@github.com:octocat/zed.git",
                "ssh",
                "github.com",
                "/octocat/zed.git",
            ),
            (
                "first.last@gitlab.example.com:group/repo.git",
                "ssh",
                "gitlab.example.com",
                "/group/repo.git",
            ),
            (
                "ssh://git@github.com/octocat/zed.git",
                "ssh",
                "github.com",
                "/octocat/zed.git",
            ),
            (
                "file:///path/to/local/zed",
                "file",
                "",
                "/path/to/local/zed",
            ),
        ];

        for (input, expected_scheme, expected_host, expected_path) in valid_urls {
            let parsed = input.parse::<RemoteUrl>().expect("failed to parse URL");
            let url = parsed.0;
            assert_eq!(
                url.scheme(),
                expected_scheme,
                "unexpected scheme for {input:?}",
            );
            assert_eq!(
                url.host_str().unwrap_or(""),
                expected_host,
                "unexpected host for {input:?}",
            );
            assert_eq!(url.path(), expected_path, "unexpected path for {input:?}");
        }
    }

    #[test]
    fn test_parsing_invalid_remote_urls() {
        let invalid_urls = vec!["not_a_url", "http://"];

        for url in invalid_urls {
            assert!(
                url.parse::<RemoteUrl>().is_err(),
                "expected \"{url}\" to not parse as a Git remote URL",
            );
        }
    }

    #[test]
    fn test_normalize_remote_url() {
        // Bare `host/path` URLs get `https://` prepended.
        assert_eq!(
            normalize_remote_url("github.com/octocat/zed"),
            "https://github.com/octocat/zed",
        );
        assert_eq!(
            normalize_remote_url("github.com/octocat/zed.git"),
            "https://github.com/octocat/zed.git",
        );

        // Surrounding whitespace is trimmed before prefixing.
        assert_eq!(
            normalize_remote_url("  github.com/octocat/zed  "),
            "https://github.com/octocat/zed",
        );

        // URLs that already have a scheme are left unchanged (modulo trim).
        for url in [
            "https://github.com/octocat/zed.git",
            "http://github.com/octocat/zed.git",
            "ssh://git@github.com/octocat/zed.git",
            "file:///path/to/local/zed",
        ] {
            assert_eq!(normalize_remote_url(url), url);
        }

        // SCP-like remotes are left unchanged, with or without a username.
        for url in [
            "git@github.com:octocat/zed.git",
            "github.com:octocat/zed.git",
        ] {
            assert_eq!(normalize_remote_url(url), url);
        }

        // Local clone sources are left unchanged.
        for path in [
            "/path/to/local/zed",
            "./path/to/local/zed",
            "../path/to/local/zed",
            "path/to/local/zed",
            r"C:\path\to\local\zed",
            r"\\server\share\zed",
        ] {
            assert_eq!(normalize_remote_url(path), path);
        }
    }
}
