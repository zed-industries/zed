use derive_more::Deref;
use url::Url;

/// The URL to a Git remote.
#[derive(Debug, PartialEq, Eq, Clone, Deref)]
pub struct RemoteUrl(Url);

impl std::str::FromStr for RemoteUrl {
    type Err = url::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.starts_with("git@") {
            // Rewrite remote URLs like `git@github.com:user/repo.git` to `ssh://git@github.com/user/repo.git`
            let ssh_url = input.replacen(':', "/", 1).replace("git@", "ssh://git@");
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
                "git@github.com:octocat/zed.git",
                "ssh",
                "github.com",
                "/octocat/zed.git",
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
