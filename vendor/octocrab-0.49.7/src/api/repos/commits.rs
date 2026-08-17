use super::*;
use chrono::{DateTime, Utc};

#[derive(serde::Serialize)]
pub struct ListCommitsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sha: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    until: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListCommitsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            sha: None,
            path: None,
            author: None,
            since: None,
            until: None,
            per_page: None,
            page: None,
        }
    }

    /// SHA or branch to start listing commits from. Default: the repository’s default
    /// branch (usually master).
    pub fn sha(mut self, sha: impl Into<String>) -> Self {
        self.sha = Some(sha.into());
        self
    }

    /// Alias for [`ListCommitsBuilder::sha`], setting a branch will replace the SHA or vice versa.
    pub fn branch(mut self, branch: impl Into<String>) -> Self {
        self.sha = Some(branch.into());
        self
    }

    /// Only commits containing this file path will be returned.
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// GitHub login or email address by which to filter by commit author.
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Only show notifications updated after the given time.
    pub fn since(mut self, since: DateTime<Utc>) -> Self {
        self.since = Some(since);
        self
    }

    /// Only commits before this date will be returned.
    pub fn until(mut self, until: DateTime<Utc>) -> Self {
        self.until = Some(until);
        self
    }

    /// Results per page (max: 100, default: 30).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch. (default: 1)
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::repos::RepoCommit>> {
        let route = format!("/{}/commits", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}
