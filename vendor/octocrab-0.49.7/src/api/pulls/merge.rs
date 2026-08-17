use super::*;

/// A builder pattern struct for merging pull requests.
///
/// created by [`PullRequestHandler::merge`]
///
/// [`PullRequestHandler::merge`]: ./struct.PullRequestHandler.html#method.merge
#[derive(serde::Serialize)]
pub struct MergePullRequestsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    #[serde(skip)]
    pr_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_method: Option<crate::params::pulls::MergeMethod>,
}

impl<'octo, 'b> MergePullRequestsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b PullRequestHandler<'octo>, pr_number: u64) -> Self {
        Self {
            handler,
            pr_number,
            commit_title: None,
            commit_message: None,
            sha: None,
            merge_method: None,
        }
    }

    /// Title for the automatic commit message.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.commit_title = Some(title.into());
        self
    }

    /// Extra detail to append to automatic commit message.
    pub fn message(mut self, msg: impl Into<String>) -> Self {
        self.commit_message = Some(msg.into());
        self
    }

    /// SHA that pull request head must match to allow merge.
    pub fn sha(mut self, sha: impl Into<String>) -> Self {
        self.sha = Some(sha.into());
        self
    }

    /// Merge method to use. Default is `Merge`.
    pub fn method(mut self, method: impl Into<crate::params::pulls::MergeMethod>) -> Self {
        self.merge_method = Some(method.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::models::pulls::Merge> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pull_number}/merge",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pull_number = self.pr_number,
        );

        self.handler.http_put(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.pulls("rust-lang", "rust");
        let merge = handler
            .merge(80818)
            .title("just testing!")
            .message("promise!")
            .sha("luckily this won't deserialize ;)")
            .method(crate::params::pulls::MergeMethod::Squash);

        assert_eq!(
            serde_json::to_value(merge).unwrap(),
            serde_json::json!({
                "commit_title": "just testing!",
                "commit_message": "promise!",
                "sha": "luckily this won't deserialize ;)",
                "merge_method": "squash",
            })
        )
    }
}
