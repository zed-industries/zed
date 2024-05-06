use url::Url;

use crate::hosting_provider::GitHostingProvider;
use crate::permalink::{BuildCommitPermalinkParams, BuildPermalinkParams, ParsedGitRemote};

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

    fn build_commit_permalink(&self, params: BuildCommitPermalinkParams) -> Url {
        let BuildCommitPermalinkParams { sha, remote } = params;
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
            ..
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
