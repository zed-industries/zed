use std::sync::{Arc, OnceLock};

use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use url::Url;
use util::github;
use util::http::HttpClient;

use crate::hosting_provider::{GitHostingProvider, PullRequest};
use crate::permalink::{BuildCommitPermalinkParams, ParsedGitRemote};
use crate::Oid;

fn pull_request_number_regex() -> &'static Regex {
    static PULL_REQUEST_NUMBER_REGEX: OnceLock<Regex> = OnceLock::new();

    PULL_REQUEST_NUMBER_REGEX.get_or_init(|| Regex::new(r"\(#(\d+)\)$").unwrap())
}

pub struct Github;

#[async_trait]
impl GitHostingProvider for Github {
    fn name(&self) -> String {
        "GitHub".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://github.com").unwrap()
    }

    fn supports_avatars(&self) -> bool {
        true
    }

    fn format_line_number(&self, line: u32) -> String {
        format!("L{line}")
    }

    fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String {
        format!("L{start_line}-L{end_line}")
    }

    fn parse_remote_url<'a>(&self, url: &'a str) -> Option<ParsedGitRemote<'a>> {
        if url.starts_with("git@github.com:") || url.starts_with("https://github.com/") {
            let repo_with_owner = url
                .trim_start_matches("git@github.com:")
                .trim_start_matches("https://github.com/")
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

    fn extract_pull_request(&self, remote: &ParsedGitRemote, message: &str) -> Option<PullRequest> {
        let line = message.lines().next()?;
        let capture = pull_request_number_regex().captures(line)?;
        let number = capture.get(1)?.as_str().parse::<u32>().ok()?;

        let mut url = self.base_url();
        let path = format!("/{}/{}/pull/{}", remote.owner, remote.repo, number);
        url.set_path(&path);

        Some(PullRequest { number, url })
    }

    async fn commit_author_avatar_url(
        &self,
        repo_owner: &str,
        repo: &str,
        commit: Oid,
        http_client: Arc<dyn HttpClient>,
    ) -> Result<Option<Url>> {
        let commit = commit.to_string();
        let avatar_url =
            github::fetch_github_commit_author(repo_owner, repo, &commit, &http_client)
                .await?
                .map(|author| -> Result<Url, url::ParseError> {
                    let mut url = Url::parse(&author.avatar_url)?;
                    url.set_query(Some("size=128"));
                    Ok(url)
                })
                .transpose()?;
        Ok(avatar_url)
    }
}

#[cfg(test)]
mod tests {
    // TODO: Replace with `indoc`.
    use unindent::Unindent;

    use super::*;

    #[test]
    fn test_github_pull_requests() {
        let remote = ParsedGitRemote {
            owner: "zed-industries",
            repo: "zed",
        };

        let message = "This does not contain a pull request";
        assert!(Github.extract_pull_request(&remote, message).is_none());

        // Pull request number at end of first line
        let message = r#"
            project panel: do not expand collapsed worktrees on "collapse all entries" (#10687)

            Fixes #10597

            Release Notes:

            - Fixed "project panel: collapse all entries" expanding collapsed worktrees.
            "#
        .unindent();

        assert_eq!(
            Github
                .extract_pull_request(&remote, &message)
                .unwrap()
                .url
                .as_str(),
            "https://github.com/zed-industries/zed/pull/10687"
        );

        // Pull request number in middle of line, which we want to ignore
        let message = r#"
            Follow-up to #10687 to fix problems

            See the original PR, this is a fix.
            "#
        .unindent();
        assert_eq!(Github.extract_pull_request(&remote, &message), None);
    }
}
