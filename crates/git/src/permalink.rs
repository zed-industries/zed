use std::ops::Range;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use url::Url;

use crate::hosting_provider::{GitHostingProvider, HostingProvider};
use crate::hosting_providers::{Bitbucket, Codeberg, Gitee, Github, Gitlab, Sourcehut};

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
        HostingProvider::Github => format!("{owner}/{repo}/blob/{sha}/{path}"),
        HostingProvider::Gitlab => format!("{owner}/{repo}/-/blob/{sha}/{path}"),
        HostingProvider::Gitee => format!("{owner}/{repo}/blob/{sha}/{path}"),
        HostingProvider::Bitbucket => format!("{owner}/{repo}/src/{sha}/{path}"),
        HostingProvider::Sourcehut => format!("~{owner}/{repo}/tree/{sha}/item/{path}"),
        HostingProvider::Codeberg => format!("{owner}/{repo}/src/commit/{sha}/{path}"),
    };
    let line_fragment = selection.map(|selection| provider.line_fragment(&selection));

    let mut permalink = provider.base_url().join(&path).unwrap();
    permalink.set_fragment(line_fragment.as_deref());
    Ok(permalink)
}

#[derive(Debug)]
pub struct ParsedGitRemote<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

pub struct BuildCommitPermalinkParams<'a> {
    pub remote: &'a ParsedGitRemote<'a>,
    pub sha: &'a str,
}

pub fn build_commit_permalink(params: BuildCommitPermalinkParams) -> Url {
    let BuildCommitPermalinkParams { sha, remote } = params;

    let ParsedGitRemote { owner, repo } = remote;

    let path = match provider {
        HostingProvider::Github => format!("{owner}/{repo}/commit/{sha}"),
        HostingProvider::Gitlab => format!("{owner}/{repo}/-/commit/{sha}"),
        HostingProvider::Gitee => format!("{owner}/{repo}/commit/{sha}"),
        HostingProvider::Bitbucket => format!("{owner}/{repo}/commits/{sha}"),
        HostingProvider::Sourcehut => format!("~{owner}/{repo}/commit/{sha}"),
        HostingProvider::Codeberg => format!("{owner}/{repo}/commit/{sha}"),
    };

    provider.base_url().join(&path).unwrap()
}

pub fn parse_git_remote_url(url: &str) -> Option<(Arc<dyn GitHostingProvider>, ParsedGitRemote)> {
    let providers: Vec<Arc<dyn GitHostingProvider>> = vec![
        Arc::new(Github),
        Arc::new(Gitlab),
        Arc::new(Bitbucket),
        Arc::new(Codeberg),
        Arc::new(Gitee),
        Arc::new(Sourcehut),
    ];

    providers.into_iter().find_map(|provider| {
        provider
            .parse_remote_url(&url)
            .map(|parsed_remote| (provider, parsed_remote))
    })
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
        assert!(matches!(parsed.provider, HostingProvider::Bitbucket));
        assert_eq!(parsed.owner, "thorstenzed");
        assert_eq!(parsed.repo, "testingrepo");
    }

    #[test]
    fn test_parse_git_remote_url_bitbucket_https_without_username() {
        let url = "https://bitbucket.org/thorstenzed/testingrepo.git";
        let parsed = parse_git_remote_url(url).unwrap();
        assert!(matches!(parsed.provider, HostingProvider::Bitbucket));
        assert_eq!(parsed.owner, "thorstenzed");
        assert_eq!(parsed.repo, "testingrepo");
    }

    #[test]
    fn test_parse_git_remote_url_bitbucket_git() {
        let url = "git@bitbucket.org:thorstenzed/testingrepo.git";
        let parsed = parse_git_remote_url(url).unwrap();
        assert!(matches!(parsed.provider, HostingProvider::Bitbucket));
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
