/// A builder pattern struct for constructing an Octocrab request to update a
/// pull request.
#[derive(serde::Serialize)]
pub struct UpdatePullRequestBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b super::PullRequestHandler<'octo>,
    pull_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<crate::params::pulls::State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintainer_can_modify: Option<bool>,
}

impl<'octo, 'b> UpdatePullRequestBuilder<'octo, 'b> {
    pub(crate) fn new(
        handler: &'b super::PullRequestHandler<'octo>,
        pull_number: impl Into<u64>,
    ) -> Self {
        Self {
            handler,
            pull_number: pull_number.into(),
            title: None,
            base: None,
            state: None,
            body: None,
            maintainer_can_modify: None,
        }
    }

    /// The contents of the pull request.
    pub fn body<A: Into<String>>(mut self, body: impl Into<Option<A>>) -> Self {
        self.body = body.into().map(A::into);
        self
    }

    /// The contents of the pull request.
    pub fn title<A: Into<String>>(mut self, title: impl Into<Option<A>>) -> Self {
        self.title = title.into().map(A::into);
        self
    }

    /// The contents of the pull request.
    pub fn base<A: Into<String>>(mut self, base: impl Into<Option<A>>) -> Self {
        self.base = base.into().map(A::into);
        self
    }

    /// The contents of the pull request.
    pub fn state<A: Into<crate::params::pulls::State>>(
        mut self,
        state: impl Into<Option<A>>,
    ) -> Self {
        self.state = state.into().map(A::into);
        self
    }

    /// Indicates whether `maintainers` can modify the pull request.
    pub fn maintainer_can_modify(mut self, maintainer_can_modify: impl Into<Option<bool>>) -> Self {
        self.maintainer_can_modify = maintainer_can_modify.into();
        self
    }

    /// Sends the request to update the pull request.
    pub async fn send(self) -> crate::Result<crate::models::pulls::PullRequest> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pr = self.pull_number,
        );

        self.handler.http_patch(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.pulls("rust-lang", "rust");
        let list = handler
            .update(1)
            .title("title")
            .body(String::from("testing..."))
            .state(crate::params::pulls::State::Open)
            .maintainer_can_modify(true);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "pull_number": 1,
                "title": "title",
                "body": "testing...",
                "state": "open",
                "maintainer_can_modify": true,
            })
        )
    }
}
