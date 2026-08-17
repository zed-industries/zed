use crate::pulls::PullRequestHandler;
use serde::Serialize;

use crate::models::repos::RepoCommit;

#[derive(Serialize)]
pub struct SpecificPullRequestCommitBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r PullRequestHandler<'octo>,
    #[serde(skip)]
    pr_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> SpecificPullRequestCommitBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r PullRequestHandler<'octo>, pr_number: u64) -> Self {
        Self {
            handler,
            pr_number,
            per_page: None,
            page: None,
        }
    }

    /// Results per page.
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Send the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<RepoCommit>> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr}/commits",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pr = self.pr_number,
        );

        self.handler.crab.get(route, Some(&self)).await
    }
}
