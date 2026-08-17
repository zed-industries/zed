/// A builder pattern struct for constructing an Octocrab request to create a
/// pull request.
#[derive(serde::Serialize)]
pub struct CreatePullRequestBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b super::PullRequestHandler<'octo>,
    title: String,
    head: String,
    base: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintainer_can_modify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head_repo: Option<String>,
}

impl<'octo, 'b> CreatePullRequestBuilder<'octo, 'b> {
    pub(crate) fn new(
        handler: &'b super::PullRequestHandler<'octo>,
        title: impl Into<String>,
        head: impl Into<String>,
        base: impl Into<String>,
    ) -> Self {
        Self {
            handler,
            title: title.into(),
            head: head.into(),
            base: base.into(),
            body: None,
            draft: None,
            maintainer_can_modify: None,
            head_repo: None,
        }
    }

    /// The contents of the pull request.
    pub fn body<A: Into<String>>(mut self, body: impl Into<Option<A>>) -> Self {
        self.body = body.into().map(A::into);
        self
    }

    /// Indicates whether the pull request is a draft.
    pub fn draft(mut self, draft: impl Into<Option<bool>>) -> Self {
        self.draft = draft.into();
        self
    }

    /// Indicates whether `maintainers` can modify the pull request.
    pub fn maintainer_can_modify(mut self, maintainer_can_modify: impl Into<Option<bool>>) -> Self {
        self.maintainer_can_modify = maintainer_can_modify.into();
        self
    }

    pub fn head_repo(mut self, head_repo: impl Into<Option<String>>) -> Self {
        self.head_repo = head_repo.into();
        self
    }

    /// Sends the request to create the pull request.
    pub async fn send(self) -> crate::Result<crate::models::pulls::PullRequest> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls",
            owner = self.handler.owner,
            repo = self.handler.repo
        );

        self.handler.http_post(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.pulls("rust-lang", "rust");
        let list = handler
            .create("test-pr", "master", "branch")
            .body(String::from("testing..."))
            .draft(true)
            .head_repo(String::from("foobar"))
            .maintainer_can_modify(true);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "title": "test-pr",
                "head": "master",
                "base": "branch",
                "body": "testing...",
                "draft": true,
                "maintainer_can_modify": true,
                "head_repo": "foobar",
            })
        )
    }
}
