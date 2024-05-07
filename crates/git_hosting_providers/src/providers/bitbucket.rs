use url::Url;

use git::{BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote};

pub struct Bitbucket;

impl GitHostingProvider for Bitbucket {
    fn name(&self) -> String {
        "Bitbucket".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://bitbucket.org").unwrap()
    }

    fn supports_avatars(&self) -> bool {
        false
    }

    fn format_line_number(&self, line: u32) -> String {
        format!("lines-{line}")
    }

    fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String {
        format!("lines-{start_line}:{end_line}")
    }

    fn parse_remote_url<'a>(&self, url: &'a str) -> Option<ParsedGitRemote<'a>> {
        if url.contains("bitbucket.org") {
            let (_, repo_with_owner) = url.trim_end_matches(".git").split_once("bitbucket.org")?;
            let (owner, repo) = repo_with_owner
                .trim_start_matches('/')
                .trim_start_matches(':')
                .split_once('/')?;

            return Some(ParsedGitRemote { owner, repo });
        }

        None
    }

    fn build_commit_permalink(
        &self,
        remote: &ParsedGitRemote,
        params: BuildCommitPermalinkParams,
    ) -> Url {
        let BuildCommitPermalinkParams { sha } = params;
        let ParsedGitRemote { owner, repo } = remote;

        self.base_url()
            .join(&format!("{owner}/{repo}/commits/{sha}"))
            .unwrap()
    }

    fn build_permalink(&self, remote: ParsedGitRemote, params: BuildPermalinkParams) -> Url {
        let ParsedGitRemote { owner, repo } = remote;
        let BuildPermalinkParams {
            sha,
            path,
            selection,
        } = params;

        let mut permalink = self
            .base_url()
            .join(&format!("{owner}/{repo}/src/{sha}/{path}"))
            .unwrap();
        permalink.set_fragment(
            selection
                .map(|selection| self.line_fragment(&selection))
                .as_deref(),
        );
        permalink
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use git::{parse_git_remote_url, GitHostingProviderRegistry};

    use super::*;

    #[test]
    fn test_parse_git_remote_url_bitbucket_https_with_username() {
        let provider_registry = Arc::new(GitHostingProviderRegistry::new());
        provider_registry.register_hosting_provider(Arc::new(Bitbucket));
        let url = "https://thorstenballzed@bitbucket.org/thorstenzed/testingrepo.git";
        let (provider, parsed) = parse_git_remote_url(provider_registry, url).unwrap();
        assert_eq!(provider.name(), "Bitbucket");
        assert_eq!(parsed.owner, "thorstenzed");
        assert_eq!(parsed.repo, "testingrepo");
    }

    #[test]
    fn test_parse_git_remote_url_bitbucket_https_without_username() {
        let provider_registry = Arc::new(GitHostingProviderRegistry::new());
        provider_registry.register_hosting_provider(Arc::new(Bitbucket));
        let url = "https://bitbucket.org/thorstenzed/testingrepo.git";
        let (provider, parsed) = parse_git_remote_url(provider_registry, url).unwrap();
        assert_eq!(provider.name(), "Bitbucket");
        assert_eq!(parsed.owner, "thorstenzed");
        assert_eq!(parsed.repo, "testingrepo");
    }

    #[test]
    fn test_parse_git_remote_url_bitbucket_git() {
        let provider_registry = Arc::new(GitHostingProviderRegistry::new());
        provider_registry.register_hosting_provider(Arc::new(Bitbucket));
        let url = "git@bitbucket.org:thorstenzed/testingrepo.git";
        let (provider, parsed) = parse_git_remote_url(provider_registry, url).unwrap();
        assert_eq!(provider.name(), "Bitbucket");
        assert_eq!(parsed.owner, "thorstenzed");
        assert_eq!(parsed.repo, "testingrepo");
    }

    #[test]
    fn test_build_bitbucket_permalink_from_ssh_url() {
        let remote = ParsedGitRemote {
            owner: "thorstenzed",
            repo: "testingrepo",
        };
        let permalink = Bitbucket.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "f00b4r",
                path: "main.rs",
                selection: None,
            },
        );

        let expected_url = "https://bitbucket.org/thorstenzed/testingrepo/src/f00b4r/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_bitbucket_permalink_from_ssh_url_single_line_selection() {
        let remote = ParsedGitRemote {
            owner: "thorstenzed",
            repo: "testingrepo",
        };
        let permalink = Bitbucket.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "f00b4r",
                path: "main.rs",
                selection: Some(6..6),
            },
        );

        let expected_url =
            "https://bitbucket.org/thorstenzed/testingrepo/src/f00b4r/main.rs#lines-7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_bitbucket_permalink_from_ssh_url_multi_line_selection() {
        let remote = ParsedGitRemote {
            owner: "thorstenzed",
            repo: "testingrepo",
        };
        let permalink = Bitbucket.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "f00b4r",
                path: "main.rs",
                selection: Some(23..47),
            },
        );

        let expected_url =
            "https://bitbucket.org/thorstenzed/testingrepo/src/f00b4r/main.rs#lines-24:48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
