use std::collections::HashMap;
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

/// Resolves SSH config `Host` aliases to their `HostName` so that
/// SCP-like remote URLs like `git@personal:user/repo` parse with the
/// real host (e.g. `github.com`) instead of the alias.
static SSH_HOST_ALIASES: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let home = std::env::var("HOME").unwrap_or_default();
    let path = format!("{home}/.ssh/config");
    match std::fs::read_to_string(&path) {
        Ok(config) => parse_ssh_host_aliases(&config),
        Err(_) => HashMap::new(),
    }
});

fn parse_ssh_host_aliases(config: &str) -> HashMap<String, String> {
    let mut aliases = HashMap::new();
    let mut current_hosts: Vec<String> = Vec::new();

    for line in config.lines() {
        let line = line.trim();
        let Some((keyword, value)) = line.split_once(char::is_whitespace) else {
            continue;
        };

        if keyword.eq_ignore_ascii_case("host") {
            current_hosts = value
                .split_whitespace()
                .filter(|h| !h.contains('*') && !h.starts_with('!'))
                .map(|h| h.to_owned())
                .collect();
        } else if keyword.eq_ignore_ascii_case("hostname") {
            if let Some(hostname) = value.split_whitespace().next() {
                for host in &current_hosts {
                    aliases.insert(host.clone(), hostname.to_owned());
                }
            }
        }
    }

    aliases
}

impl FromStr for RemoteUrl {
    type Err = url::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if USERNAME_REGEX.is_match(input) {
            let ssh_url = format!("ssh://{}", input.replacen(':', "/", 1));
            let mut url = Url::parse(&ssh_url)?;

            if let Some(host) = url.host_str() {
                if let Some(resolved) = SSH_HOST_ALIASES.get(host) {
                    url.set_host(Some(resolved))
                        .map_err(|_| url::ParseError::InvalidPort)?;
                }
            }

            Ok(RemoteUrl(url))
        } else {
            Ok(RemoteUrl(Url::parse(input)?))
        }
    }
}

impl RemoteUrl {
    /// Like `from_str` but with an explicit alias map, for testing.
    fn parse_with_ssh_aliases(
        input: &str,
        aliases: &HashMap<String, String>,
    ) -> Result<Self, url::ParseError> {
        if USERNAME_REGEX.is_match(input) {
            let ssh_url = format!("ssh://{}", input.replacen(':', "/", 1));
            let mut url = Url::parse(&ssh_url)?;
            if let Some(host) = url.host_str() {
                if let Some(resolved) = aliases.get(host) {
                    url.set_host(Some(resolved))
                        .map_err(|_| url::ParseError::InvalidPort)?;
                }
            }
            Ok(RemoteUrl(url))
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

    #[test]
    fn test_parse_ssh_host_aliases() {
        let config = "\
Host *
  AddKeysToAgent yes

Host personal
  HostName github.com
  User git

Host work
  HostName gitlab.com

Host multi host1 host2
  HostName example.com
";
        let aliases = parse_ssh_host_aliases(config);
        assert_eq!(aliases.get("personal"), Some(&"github.com".to_owned()));
        assert_eq!(aliases.get("work"), Some(&"gitlab.com".to_owned()));
        assert_eq!(aliases.get("multi"), Some(&"example.com".to_owned()));
        assert_eq!(aliases.get("host1"), Some(&"example.com".to_owned()));
        assert_eq!(aliases.get("host2"), Some(&"example.com".to_owned()));
        assert!(!aliases.contains_key("*"));
    }

    #[test]
    fn test_remote_url_resolves_ssh_host_alias() {
        let mut aliases = HashMap::new();
        aliases.insert("personal".to_owned(), "github.com".to_owned());

        let parsed = RemoteUrl::parse_with_ssh_aliases("git@personal:user/repo.git", &aliases)
            .expect("failed to parse");
        assert_eq!(parsed.host_str(), Some("github.com"));
        assert_eq!(parsed.path(), "/user/repo.git");

        let parsed = RemoteUrl::parse_with_ssh_aliases("git@github.com:user/repo.git", &aliases)
            .expect("failed to parse");
        assert_eq!(parsed.host_str(), Some("github.com"));

        let parsed = RemoteUrl::parse_with_ssh_aliases("git@unknown:host/repo.git", &aliases)
            .expect("failed to parse");
        assert_eq!(parsed.host_str(), Some("unknown"));
    }
}
