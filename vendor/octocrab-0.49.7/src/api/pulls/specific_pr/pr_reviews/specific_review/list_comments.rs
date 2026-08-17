use super::*;

#[derive(serde::Serialize)]
pub struct ListReviewCommentsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    pr_number: u64,
    review_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> ListReviewCommentsBuilder<'octo, 'b> {
    pub(crate) fn new(
        handler: &'b PullRequestHandler<'octo>,
        pr_number: u64,
        review_id: u64,
    ) -> Self {
        Self {
            handler,
            pr_number,
            review_id,
            per_page: None,
            page: None,
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
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::pulls::ReviewComment>> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/comments",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pull_number = self.pr_number,
            review_id = self.review_id
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
