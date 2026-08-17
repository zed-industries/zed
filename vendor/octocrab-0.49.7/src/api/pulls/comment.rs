use serde_json::json;

use crate::models::pulls::Comment;

use super::*;

/// A builder pattern struct for listing comments.
///
/// created by [`PullRequestHandler::list_comments`]
///
/// [`PullRequestHandler::list_comments`]: ./struct.PullRequestHandler.html#method.list_comments
#[derive(serde::Serialize)]
pub struct ListCommentsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    #[serde(skip)]
    pr: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<crate::params::pulls::comments::Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<crate::params::Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<chrono::DateTime<chrono::Utc>>,
}

impl<'octo, 'b> ListCommentsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b PullRequestHandler<'octo>, pr: Option<u64>) -> Self {
        Self {
            handler,
            pr,
            sort: None,
            direction: None,
            per_page: None,
            page: None,
            since: None,
        }
    }

    /// What to sort results by. Can be either `created` or `updated`,
    pub fn sort(mut self, sort: impl Into<crate::params::pulls::comments::Sort>) -> Self {
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

    /// Only show notifications updated after the given time.
    pub fn since(mut self, since: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.since = Some(since.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<crate::models::pulls::Comment>> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pr}comments",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pr = if let Some(pr) = self.pr {
                pr.to_string() + "/"
            } else {
                "".into()
            },
        );
        self.handler.http_get(route, Some(&self)).await
    }
}

/// A builder pattern struct for working with specific comment.
///
/// created by [`PullRequestHandler::comment`]
///
/// [`PullRequestHandler::comment`]: ./struct.PullRequestHandler.html#method.comment
#[derive(serde::Serialize)]
pub struct CommentBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    comment_id: CommentId,
}

impl<'octo, 'b> CommentBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b PullRequestHandler<'octo>, comment_id: CommentId) -> Self {
        Self {
            handler,
            comment_id,
        }
    }

    ///https://docs.github.com/en/rest/pulls/comments?apiVersion=2022-11-28#get-a-review-comment-for-a-pull-request
    pub async fn get(self) -> crate::Result<Comment> {
        self.handler
            .crab
            .get(
                format!(
                    "/repos/{owner}/{repo}/pulls/comments/{comment_id}",
                    owner = self.handler.owner,
                    repo = self.handler.repo,
                    comment_id = self.comment_id
                ),
                None::<&Comment>,
            )
            .await
    }

    ///https://docs.github.com/en/rest/pulls/comments?apiVersion=2022-11-28#update-a-review-comment-for-a-pull-request
    pub async fn update(self, comment: &str) -> crate::Result<Comment> {
        self.handler
            .crab
            .patch(
                format!(
                    "/repos/{owner}/{repo}/pulls/comments/{comment_id}",
                    owner = self.handler.owner,
                    repo = self.handler.repo,
                    comment_id = self.comment_id
                ),
                Some(&json!({ "body": comment })),
            )
            .await
    }

    ///https://docs.github.com/en/rest/pulls/comments?apiVersion=2022-11-28#delete-a-review-comment-for-a-pull-request
    pub async fn delete(self) -> crate::Result<()> {
        self.handler
            .crab
            ._delete(
                format!(
                    "/repos/{owner}/{repo}/pulls/comments/{comment_id}",
                    owner = self.handler.owner,
                    repo = self.handler.repo,
                    comment_id = self.comment_id
                ),
                None::<&()>,
            )
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.pulls("rust-lang", "rust");
        let yesterday = chrono::Utc::now() - chrono::Duration::days(1);
        let list = handler
            .list_comments(Some(1))
            .sort(crate::params::pulls::comments::Sort::Updated)
            .direction(crate::params::Direction::Ascending)
            .since(yesterday)
            .per_page(100)
            .page(1u8);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "sort": "updated",
                "direction": "asc",
                "per_page": 100,
                "page": 1,
                "since": yesterday
            })
        )
    }
}
