use core::fmt;
use std::{ops::Range, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use url::Url;
use util::{codeberg, github, http::HttpClient};

use crate::hosting_providers::{Bitbucket, Codeberg, Gitee, Github, Gitlab, Sourcehut};
use crate::permalink::{BuildCommitPermalinkParams, ParsedGitRemote};
use crate::Oid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PullRequest {
    pub number: u32,
    pub url: Url,
}

/// A Git hosting provider.
#[async_trait]
pub trait GitHostingProvider {
    /// Returns the name of the provider.
    fn name(&self) -> String;

    /// Returns the base URL of the provider.
    fn base_url(&self) -> Url;

    fn build_commit_permalink(&self, params: BuildCommitPermalinkParams) -> Url;

    /// Returns whether this provider supports avatars.
    fn supports_avatars(&self) -> bool;

    /// Returns a URL fragment to the given line selection.
    fn line_fragment(&self, selection: &Range<u32>) -> String {
        if selection.start == selection.end {
            let line = selection.start + 1;

            self.format_line_number(line)
        } else {
            let start_line = selection.start + 1;
            let end_line = selection.end + 1;

            self.format_line_numbers(start_line, end_line)
        }
    }

    /// Returns a formatted line number to be placed in a permalink URL.
    fn format_line_number(&self, line: u32) -> String;

    /// Returns a formatted range of line numbers to be placed in a permalink URL.
    fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String;

    fn parse_remote_url<'a>(&self, url: &'a str) -> Option<ParsedGitRemote<'a>>;

    fn extract_pull_request(
        &self,
        _remote: &ParsedGitRemote,
        _message: &str,
    ) -> Option<PullRequest> {
        None
    }

    async fn commit_author_avatar_url(
        &self,
        _repo_owner: &str,
        _repo: &str,
        _commit: Oid,
        _http_client: Arc<dyn HttpClient>,
    ) -> Result<Option<Url>> {
        Ok(None)
    }
}

pub enum HostingProvider {
    Github,
    Gitlab,
    Gitee,
    Bitbucket,
    Sourcehut,
    Codeberg,
}

impl HostingProvider {
    fn provider(&self) -> Arc<dyn GitHostingProvider> {
        match self {
            HostingProvider::Github => Arc::new(Github),
            HostingProvider::Gitlab => Arc::new(Gitlab),
            HostingProvider::Gitee => Arc::new(Gitee),
            HostingProvider::Bitbucket => Arc::new(Bitbucket),
            HostingProvider::Sourcehut => Arc::new(Sourcehut),
            HostingProvider::Codeberg => Arc::new(Codeberg),
        }
    }

    pub fn supports_avatars(&self) -> bool {
        self.provider().supports_avatars()
    }

    pub async fn commit_author_avatar_url(
        &self,
        repo_owner: &str,
        repo: &str,
        commit: Oid,
        client: Arc<dyn HttpClient>,
    ) -> Result<Option<Url>> {
        Ok(match self {
            HostingProvider::Github => {
                let commit = commit.to_string();
                github::fetch_github_commit_author(repo_owner, repo, &commit, &client)
                    .await?
                    .map(|author| -> Result<Url, url::ParseError> {
                        let mut url = Url::parse(&author.avatar_url)?;
                        url.set_query(Some("size=128"));
                        Ok(url)
                    })
                    .transpose()
            }
            HostingProvider::Codeberg => {
                let commit = commit.to_string();
                codeberg::fetch_codeberg_commit_author(repo_owner, repo, &commit, &client)
                    .await?
                    .map(|author| Url::parse(&author.avatar_url))
                    .transpose()
            }
            _ => Ok(None),
        }?)
    }
}

impl fmt::Display for HostingProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.provider().name())
    }
}
