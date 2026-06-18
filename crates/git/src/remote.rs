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
}
