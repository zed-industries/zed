use super::*;
use crate::from_response::FromResponse;

#[derive(serde::Serialize)]
pub struct MergeBranchBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    head: String,
    base: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_message: Option<String>,
}

impl<'octo, 'r> MergeBranchBuilder<'octo, 'r> {
    pub fn new(
        handler: &'r RepoHandler<'octo>,
        head: impl Into<String>,
        base: impl Into<String>,
    ) -> Self {
        Self {
            handler,
            head: head.into(),
            base: base.into(),
            commit_message: None,
        }
    }

    /// The message to use for the merge commit.
    pub fn commit_message(mut self, commit_message: impl Into<String>) -> Self {
        self.commit_message = Some(commit_message.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Option<models::repos::MergeCommit>> {
        let route = format!("/{}/merges", self.handler.repo);
        let post_response = self.handler.crab._post(route, Some(&self)).await?;

        if post_response.status() == http::StatusCode::NO_CONTENT {
            return Ok(None);
        }

        match FromResponse::from_response(crate::map_github_error(post_response).await?).await {
            Ok(res) => Ok(Some(res)),
            Err(e) => Err(e),
        }
    }
}
