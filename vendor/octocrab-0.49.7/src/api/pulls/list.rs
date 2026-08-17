use super::*;

/// A builder pattern struct for listing pull requests.
///
/// created by [`PullRequestHandler::list`]
///
/// [`PullRequestHandler::list`]: ./struct.PullRequestHandler.html#method.list
#[derive(serde::Serialize)]
pub struct ListPullRequestsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<crate::params::State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<crate::params::pulls::Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<crate::params::Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> ListPullRequestsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b PullRequestHandler<'octo>) -> Self {
        Self {
            handler,
            state: None,
            head: None,
            base: None,
            sort: None,
            direction: None,
            per_page: None,
            page: None,
        }
    }

    /// Filter pull requests by `state`.
    pub fn state(mut self, state: crate::params::State) -> Self {
        self.state = Some(state);
        self
    }

    /// Filter pull requests by head user or head organization and branch name
    /// in the format of `user:ref-name` or `organization:ref-name`. For
    /// example: `github:new-script-format` or `octocrab:test-branch`.
    pub fn head(mut self, head: impl Into<String>) -> Self {
        self.head = Some(head.into());
        self
    }

    /// Filter pulls by base branch name. Example: `gh-pages`.
    pub fn base(mut self, base: impl Into<String>) -> Self {
        self.base = Some(base.into());
        self
    }

    /// What to sort results by. Can be either `created`, `updated`,
    /// `popularity` (comment count) or `long-running` (age, filtering by pulls
    /// updated in the last month).
    pub fn sort(mut self, sort: impl Into<crate::params::pulls::Sort>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// The direction of the sort. Can be either ascending or descending.
    /// Default: descending when sort is `created` or sort is not specified,
    /// otherwise ascending sort.
    pub fn direction(mut self, direction: impl Into<crate::params::Direction>) -> Self {
        self.direction = Some(direction.into());
        self
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
    pub async fn send(self) -> crate::Result<Page<crate::models::pulls::PullRequest>> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.http_get(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.pulls("rust-lang", "rust");
        let list = handler
            .list()
            .state(crate::params::State::Open)
            .head("master")
            .base("branch")
            .sort(crate::params::pulls::Sort::Popularity)
            .direction(crate::params::Direction::Ascending)
            .per_page(100)
            .page(1u8);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "state": "open",
                "head": "master",
                "base": "branch",
                "sort": "popularity",
                "direction": "asc",
                "per_page": 100,
                "page": 1,
            })
        )
    }
}
