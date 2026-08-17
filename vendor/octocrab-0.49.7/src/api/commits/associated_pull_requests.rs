use std::fmt::Display;

use super::*;

/// helper to let users know they can pass a branch name or a commit sha
#[derive(Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum PullRequestTarget {
    Branch(String),
    Sha(String),
}

impl Display for PullRequestTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Branch(branch) => write!(f, "{branch}"),
            Self::Sha(commit) => write!(f, "{commit}"),
        }
    }
}

#[derive(serde::Serialize)]
pub struct AssociatedPullRequestsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r super::CommitHandler<'octo>,
    target: PullRequestTarget,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> AssociatedPullRequestsBuilder<'octo, 'r> {
    /// Sha will return all closed pull requests for the given commit sha.
    ///
    /// Pass a Branch to return all open pull requests against that branch.
    pub(crate) fn new(handler: &'r super::CommitHandler<'octo>, target: PullRequestTarget) -> Self {
        Self {
            handler,
            target,
            page: None,
            per_page: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<models::pulls::PullRequest>> {
        let route = format!(
            "/repos/{owner}/{repo}/commits/{target}/pulls",
            owner = self.handler.owner,
            repo = self.handler.repo,
            target = self.target,
        );

        self.handler.crab.get(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn associated_pull_requests_serializes_correctly() {
        use super::PullRequestTarget;

        let octocrab = crate::Octocrab::default();
        let handler = octocrab.commits("owner", "repo");
        let associated_prs =
            handler.associated_pull_requests(PullRequestTarget::Sha("commit_sha".to_string()));

        assert_eq!(
            serde_json::to_value(associated_prs).unwrap(),
            serde_json::json!({
                "target": "commit_sha"
            })
        );
    }
}
