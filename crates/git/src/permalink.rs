use std::ops::Range;

use anyhow::{anyhow, Result};
use url::Url;

pub(crate) enum GitHostingProvider {
    Github,
    Gitlab,
    Gitee,
    Bitbucket,
    Sourcehut,
    Codeberg,
}

impl GitHostingProvider {
    fn base_url(&self) -> Url {
        let base_url = match self {
            Self::Github => "https://github.com",
            Self::Gitlab => "https://gitlab.com",
            Self::Gitee => "https://gitee.com",
            Self::Bitbucket => "https://bitbucket.org",
            Self::Sourcehut => "https://git.sr.ht",
            Self::Codeberg => "https://codeberg.org",
        };

        Url::parse(&base_url).unwrap()
    }

    /// Returns the fragment portion of the URL for the selected lines in
    /// the representation the [`GitHostingProvider`] expects.
    fn line_fragment(&self, selection: &Range<u32>) -> String {
        if selection.start == selection.end {
            let line = selection.start + 1;

            match self {
                Self::Github | Self::Gitlab | Self::Gitee | Self::Sourcehut | Self::Codeberg => {
                    format!("L{}", line)
                }
                Self::Bitbucket => format!("lines-{}", line),
            }
        } else {
            let start_line = selection.start + 1;
            let end_line = selection.end + 1;

            match self {
                Self::Github | Self::Codeberg => format!("L{}-L{}", start_line, end_line),
                Self::Gitlab | Self::Gitee | Self::Sourcehut => {
                    format!("L{}-{}", start_line, end_line)
                }
                Self::Bitbucket => format!("lines-{}:{}", start_line, end_line),
            }
        }
    }
}

pub struct BuildPermalinkParams<'a> {
    pub remote_url: &'a str,
    pub sha: &'a str,
    pub path: &'a str,
    pub selection: Option<Range<u32>>,
}

pub fn build_permalink(params: BuildPermalinkParams) -> Result<Url> {
    let BuildPermalinkParams {
        remote_url,
        sha,
        path,
        selection,
    } = params;

    let ParsedGitRemote {
        provider,
        owner,
        repo,
    } = parse_git_remote_url(remote_url)
        .ok_or_else(|| anyhow!("failed to parse Git remote URL"))?;

    let path = match provider {
        GitHostingProvider::Github => format!("{owner}/{repo}/blob/{sha}/{path}"),
        GitHostingProvider::Gitlab => format!("{owner}/{repo}/-/blob/{sha}/{path}"),
        GitHostingProvider::Gitee => format!("{owner}/{repo}/blob/{sha}/{path}"),
        GitHostingProvider::Bitbucket => format!("{owner}/{repo}/src/{sha}/{path}"),
        GitHostingProvider::Sourcehut => format!("~{owner}/{repo}/tree/{sha}/item/{path}"),
        GitHostingProvider::Codeberg => format!("{owner}/{repo}/src/commit/{sha}/{path}"),
    };
    let line_fragment = selection.map(|selection| provider.line_fragment(&selection));

    let mut permalink = provider.base_url().join(&path).unwrap();
    permalink.set_fragment(line_fragment.as_deref());
    Ok(permalink)
}

pub(crate) struct ParsedGitRemote<'a> {
    pub provider: GitHostingProvider,
    pub owner: &'a str,
    pub repo: &'a str,
}

pub(crate) struct BuildCommitPermalinkParams<'a> {
    pub remote: &'a ParsedGitRemote<'a>,
    pub sha: &'a str,
}

pub(crate) fn build_commit_permalink(params: BuildCommitPermalinkParams) -> Url {
    let BuildCommitPermalinkParams { sha, remote } = params;

    let ParsedGitRemote {
        provider,
        owner,
        repo,
    } = remote;

    let path = match provider {
        GitHostingProvider::Github => format!("{owner}/{repo}/commits/{sha}"),
        GitHostingProvider::Gitlab => format!("{owner}/{repo}/-/commit/{sha}"),
        GitHostingProvider::Gitee => format!("{owner}/{repo}/commit/{sha}"),
        GitHostingProvider::Bitbucket => format!("{owner}/{repo}/commits/{sha}"),
        GitHostingProvider::Sourcehut => format!("~{owner}/{repo}/commit/{sha}"),
        GitHostingProvider::Codeberg => format!("{owner}/{repo}/commit/{sha}"),
    };

    provider.base_url().join(&path).unwrap()
}

pub(crate) fn parse_git_remote_url(url: &str) -> Option<ParsedGitRemote> {
    if url.starts_with("git@github.com:") || url.starts_with("https://github.com/") {
        let repo_with_owner = url
            .trim_start_matches("git@github.com:")
            .trim_start_matches("https://github.com/")
            .trim_end_matches(".git");

        let (owner, repo) = repo_with_owner.split_once('/')?;

        return Some(ParsedGitRemote {
            provider: GitHostingProvider::Github,
            owner,
            repo,
        });
    }

    if url.starts_with("git@gitlab.com:") || url.starts_with("https://gitlab.com/") {
        let repo_with_owner = url
            .trim_start_matches("git@gitlab.com:")
            .trim_start_matches("https://gitlab.com/")
            .trim_end_matches(".git");

        let (owner, repo) = repo_with_owner.split_once('/')?;

        return Some(ParsedGitRemote {
            provider: GitHostingProvider::Gitlab,
            owner,
            repo,
        });
    }

    if url.starts_with("git@gitee.com:") || url.starts_with("https://gitee.com/") {
        let repo_with_owner = url
            .trim_start_matches("git@gitee.com:")
            .trim_start_matches("https://gitee.com/")
            .trim_end_matches(".git");

        let (owner, repo) = repo_with_owner.split_once('/')?;

        return Some(ParsedGitRemote {
            provider: GitHostingProvider::Gitee,
            owner,
            repo,
        });
    }

    if url.contains("bitbucket.org") {
        let (_, repo_with_owner) = url.trim_end_matches(".git").split_once("bitbucket.org")?;
        let (owner, repo) = repo_with_owner
            .trim_start_matches('/')
            .trim_start_matches(':')
            .split_once('/')?;

        return Some(ParsedGitRemote {
            provider: GitHostingProvider::Bitbucket,
            owner,
            repo,
        });
    }

    if url.starts_with("git@git.sr.ht:") || url.starts_with("https://git.sr.ht/") {
        // sourcehut indicates a repo with '.git' suffix as a separate repo.
        // For example, "git@git.sr.ht:~username/repo" and "git@git.sr.ht:~username/repo.git"
        // are two distinct repositories.
        let repo_with_owner = url
            .trim_start_matches("git@git.sr.ht:~")
            .trim_start_matches("https://git.sr.ht/~");

        let (owner, repo) = repo_with_owner.split_once('/')?;

        return Some(ParsedGitRemote {
            provider: GitHostingProvider::Sourcehut,
            owner,
            repo,
        });
    }

    if url.starts_with("git@codeberg.org:") || url.starts_with("https://codeberg.org/") {
        let repo_with_owner = url
            .trim_start_matches("git@codeberg.org:")
            .trim_start_matches("https://codeberg.org/")
            .trim_end_matches(".git");

        let (owner, repo) = repo_with_owner.split_once('/')?;

        return Some(ParsedGitRemote {
            provider: GitHostingProvider::Codeberg,
            owner,
            repo,
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_github_permalink_from_ssh_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@github.com:zed-industries/zed.git",
            sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
            path: "crates/editor/src/git/permalink.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://github.com/zed-industries/zed/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_github_permalink_from_ssh_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@github.com:zed-industries/zed.git",
            sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(6..6),
        })
        .unwrap();

        let expected_url = "https://github.com/zed-industries/zed/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_github_permalink_from_ssh_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@github.com:zed-industries/zed.git",
            sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(23..47),
        })
        .unwrap();

        let expected_url = "https://github.com/zed-industries/zed/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L24-L48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_github_permalink_from_https_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://github.com/zed-industries/zed.git",
            sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
            path: "crates/zed/src/main.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://github.com/zed-industries/zed/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_github_permalink_from_https_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://github.com/zed-industries/zed.git",
            sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
            path: "crates/zed/src/main.rs",
            selection: Some(6..6),
        })
        .unwrap();

        let expected_url = "https://github.com/zed-industries/zed/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_github_permalink_from_https_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://github.com/zed-industries/zed.git",
            sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
            path: "crates/zed/src/main.rs",
            selection: Some(23..47),
        })
        .unwrap();

        let expected_url = "https://github.com/zed-industries/zed/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs#L24-L48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_ssh_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@gitlab.com:zed-industries/zed.git",
            sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
            path: "crates/editor/src/git/permalink.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_ssh_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@gitlab.com:zed-industries/zed.git",
            sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(6..6),
        })
        .unwrap();

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_ssh_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@gitlab.com:zed-industries/zed.git",
            sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(23..47),
        })
        .unwrap();

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_https_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://gitlab.com/zed-industries/zed.git",
            sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
            path: "crates/zed/src/main.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_https_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://gitlab.com/zed-industries/zed.git",
            sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
            path: "crates/zed/src/main.rs",
            selection: Some(6..6),
        })
        .unwrap();

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_https_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://gitlab.com/zed-industries/zed.git",
            sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
            path: "crates/zed/src/main.rs",
            selection: Some(23..47),
        })
        .unwrap();

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_ssh_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@gitee.com:libkitten/zed.git",
            sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
            path: "crates/editor/src/git/permalink.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_ssh_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@gitee.com:libkitten/zed.git",
            sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(6..6),
        })
        .unwrap();

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_ssh_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@gitee.com:libkitten/zed.git",
            sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(23..47),
        })
        .unwrap();

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/editor/src/git/permalink.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_https_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://gitee.com/libkitten/zed.git",
            sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
            path: "crates/zed/src/main.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_https_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://gitee.com/libkitten/zed.git",
            sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
            path: "crates/zed/src/main.rs",
            selection: Some(6..6),
        })
        .unwrap();

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/zed/src/main.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_https_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://gitee.com/libkitten/zed.git",
            sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
            path: "crates/zed/src/main.rs",
            selection: Some(23..47),
        })
        .unwrap();
        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/zed/src/main.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_parse_git_remote_url_bitbucket_https_with_username() {
        let url = "https://thorstenballzed@bitbucket.org/thorstenzed/testingrepo.git";
        let parsed = parse_git_remote_url(url).unwrap();
        assert!(matches!(parsed.provider, GitHostingProvider::Bitbucket));
        assert_eq!(parsed.owner, "thorstenzed");
        assert_eq!(parsed.repo, "testingrepo");
    }

    #[test]
    fn test_parse_git_remote_url_bitbucket_https_without_username() {
        let url = "https://bitbucket.org/thorstenzed/testingrepo.git";
        let parsed = parse_git_remote_url(url).unwrap();
        assert!(matches!(parsed.provider, GitHostingProvider::Bitbucket));
        assert_eq!(parsed.owner, "thorstenzed");
        assert_eq!(parsed.repo, "testingrepo");
    }

    #[test]
    fn test_parse_git_remote_url_bitbucket_git() {
        let url = "git@bitbucket.org:thorstenzed/testingrepo.git";
        let parsed = parse_git_remote_url(url).unwrap();
        assert!(matches!(parsed.provider, GitHostingProvider::Bitbucket));
        assert_eq!(parsed.owner, "thorstenzed");
        assert_eq!(parsed.repo, "testingrepo");
    }

    #[test]
    fn test_build_bitbucket_permalink_from_ssh_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@bitbucket.org:thorstenzed/testingrepo.git",
            sha: "f00b4r",
            path: "main.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://bitbucket.org/thorstenzed/testingrepo/src/f00b4r/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_bitbucket_permalink_from_ssh_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@bitbucket.org:thorstenzed/testingrepo.git",
            sha: "f00b4r",
            path: "main.rs",
            selection: Some(6..6),
        })
        .unwrap();

        let expected_url =
            "https://bitbucket.org/thorstenzed/testingrepo/src/f00b4r/main.rs#lines-7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_bitbucket_permalink_from_ssh_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@bitbucket.org:thorstenzed/testingrepo.git",
            sha: "f00b4r",
            path: "main.rs",
            selection: Some(23..47),
        })
        .unwrap();

        let expected_url =
            "https://bitbucket.org/thorstenzed/testingrepo/src/f00b4r/main.rs#lines-24:48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_from_ssh_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@git.sr.ht:~rajveermalviya/zed",
            sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
            path: "crates/editor/src/git/permalink.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://git.sr.ht/~rajveermalviya/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_from_ssh_url_with_git_prefix() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@git.sr.ht:~rajveermalviya/zed.git",
            sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
            path: "crates/editor/src/git/permalink.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://git.sr.ht/~rajveermalviya/zed.git/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_from_ssh_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@git.sr.ht:~rajveermalviya/zed",
            sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(6..6),
        })
        .unwrap();

        let expected_url = "https://git.sr.ht/~rajveermalviya/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_from_ssh_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@git.sr.ht:~rajveermalviya/zed",
            sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(23..47),
        })
        .unwrap();

        let expected_url = "https://git.sr.ht/~rajveermalviya/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_from_https_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://git.sr.ht/~rajveermalviya/zed",
            sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
            path: "crates/zed/src/main.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://git.sr.ht/~rajveermalviya/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_from_https_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://git.sr.ht/~rajveermalviya/zed",
            sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
            path: "crates/zed/src/main.rs",
            selection: Some(6..6),
        })
        .unwrap();

        let expected_url = "https://git.sr.ht/~rajveermalviya/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/zed/src/main.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_from_https_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://git.sr.ht/~rajveermalviya/zed",
            sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
            path: "crates/zed/src/main.rs",
            selection: Some(23..47),
        })
        .unwrap();

        let expected_url = "https://git.sr.ht/~rajveermalviya/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/zed/src/main.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_codeberg_permalink_from_ssh_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@codeberg.org:rajveermalviya/zed.git",
            sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
            path: "crates/editor/src/git/permalink.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://codeberg.org/rajveermalviya/zed/src/commit/faa6f979be417239b2e070dbbf6392b909224e0b/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_codeberg_permalink_from_ssh_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@codeberg.org:rajveermalviya/zed.git",
            sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(6..6),
        })
        .unwrap();

        let expected_url = "https://codeberg.org/rajveermalviya/zed/src/commit/faa6f979be417239b2e070dbbf6392b909224e0b/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_codeberg_permalink_from_ssh_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@codeberg.org:rajveermalviya/zed.git",
            sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(23..47),
        })
        .unwrap();

        let expected_url = "https://codeberg.org/rajveermalviya/zed/src/commit/faa6f979be417239b2e070dbbf6392b909224e0b/crates/editor/src/git/permalink.rs#L24-L48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_codeberg_permalink_from_https_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://codeberg.org/rajveermalviya/zed.git",
            sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
            path: "crates/zed/src/main.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://codeberg.org/rajveermalviya/zed/src/commit/faa6f979be417239b2e070dbbf6392b909224e0b/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_codeberg_permalink_from_https_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://codeberg.org/rajveermalviya/zed.git",
            sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
            path: "crates/zed/src/main.rs",
            selection: Some(6..6),
        })
        .unwrap();

        let expected_url = "https://codeberg.org/rajveermalviya/zed/src/commit/faa6f979be417239b2e070dbbf6392b909224e0b/crates/zed/src/main.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_codeberg_permalink_from_https_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://codeberg.org/rajveermalviya/zed.git",
            sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
            path: "crates/zed/src/main.rs",
            selection: Some(23..47),
        })
        .unwrap();

        let expected_url = "https://codeberg.org/rajveermalviya/zed/src/commit/faa6f979be417239b2e070dbbf6392b909224e0b/crates/zed/src/main.rs#L24-L48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
