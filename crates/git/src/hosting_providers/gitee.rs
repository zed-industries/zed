use url::Url;

use crate::hosting_provider::GitHostingProvider;
use crate::permalink::{BuildCommitPermalinkParams, ParsedGitRemote};

pub struct Gitee;

impl GitHostingProvider for Gitee {
    fn name(&self) -> String {
        "Gitee".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://gitee.com").unwrap()
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
        if url.starts_with("git@gitee.com:") || url.starts_with("https://gitee.com/") {
            let repo_with_owner = url
                .trim_start_matches("git@gitee.com:")
                .trim_start_matches("https://gitee.com/")
                .trim_end_matches(".git");

            let (owner, repo) = repo_with_owner.split_once('/')?;

            return Some(ParsedGitRemote { owner, repo });
        }

        None
    }

    fn build_commit_permalink(&self, params: BuildCommitPermalinkParams) -> Url {
        let BuildCommitPermalinkParams { sha, remote } = params;
        let ParsedGitRemote { owner, repo } = remote;

        self.base_url()
            .join(&format!("{owner}/{repo}/commit/{sha}"))
            .unwrap()
    }
}
