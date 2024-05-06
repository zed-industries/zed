use url::Url;

use crate::hosting_provider::GitHostingProvider;
use crate::permalink::{BuildCommitPermalinkParams, BuildPermalinkParams, ParsedGitRemote};

pub struct Sourcehut;

impl GitHostingProvider for Sourcehut {
    fn name(&self) -> String {
        "Gitee".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://git.sr.ht").unwrap()
    }

    fn supports_avatars(&self) -> bool {
        false
    }

    fn format_line_number(&self, line: u32) -> String {
        format!("L{line}")
    }

    fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String {
        format!("L{start_line}-{end_line}")
    }

    fn parse_remote_url<'a>(&self, url: &'a str) -> Option<ParsedGitRemote<'a>> {
        if url.starts_with("git@git.sr.ht:") || url.starts_with("https://git.sr.ht/") {
            // sourcehut indicates a repo with '.git' suffix as a separate repo.
            // For example, "git@git.sr.ht:~username/repo" and "git@git.sr.ht:~username/repo.git"
            // are two distinct repositories.
            let repo_with_owner = url
                .trim_start_matches("git@git.sr.ht:~")
                .trim_start_matches("https://git.sr.ht/~");

            let (owner, repo) = repo_with_owner.split_once('/')?;

            return Some(ParsedGitRemote { owner, repo });
        }

        None
    }

    fn build_commit_permalink(&self, params: BuildCommitPermalinkParams) -> Url {
        let BuildCommitPermalinkParams { sha, remote } = params;
        let ParsedGitRemote { owner, repo } = remote;

        self.base_url()
            .join(&format!("~{owner}/{repo}/commit/{sha}"))
            .unwrap()
    }

    fn build_permalink(&self, remote: ParsedGitRemote, params: BuildPermalinkParams) -> Url {
        let ParsedGitRemote { owner, repo } = remote;
        let BuildPermalinkParams {
            sha,
            path,
            selection,
            ..
        } = params;

        let mut permalink = self
            .base_url()
            .join(&format!("~{owner}/{repo}/tree/{sha}/item/{path}"))
            .unwrap();
        permalink.set_fragment(
            selection
                .map(|selection| self.line_fragment(&selection))
                .as_deref(),
        );
        permalink
    }
}
