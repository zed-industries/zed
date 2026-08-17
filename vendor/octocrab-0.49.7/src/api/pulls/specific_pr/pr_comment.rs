use serde_json::json;

use crate::models::pulls::ReviewComment;
use crate::models::CommentId;
use crate::pulls::PullRequestHandler;

#[derive(serde::Serialize)]
pub struct SpecificPullRequestCommentBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    pr_number: u64,
    comment_id: CommentId,
}

impl<'octo, 'b> SpecificPullRequestCommentBuilder<'octo, 'b> {
    pub(crate) fn new(
        handler: &'b PullRequestHandler<'octo>,
        pr_number: u64,
        comment_id: CommentId,
    ) -> Self {
        Self {
            handler,
            comment_id,
            pr_number,
        }
    }

    pub async fn reply(&self, comment: impl Into<String>) -> crate::Result<ReviewComment> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pull_number}/comments/{comment_id}/replies",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pull_number = self.pr_number,
            comment_id = self.comment_id
        );
        self.handler
            .crab
            .post(route, Some(&json!({ "body": comment.into() })))
            .await
    }
}
