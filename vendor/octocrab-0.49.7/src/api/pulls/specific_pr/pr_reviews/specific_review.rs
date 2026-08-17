use crate::models::pulls::{Review, ReviewAction};
use crate::pulls::specific_pr::pr_reviews::specific_review::list_comments::ListReviewCommentsBuilder;
use crate::pulls::PullRequestHandler;

mod list_comments;

#[derive(serde::Serialize)]
pub struct SpecificReviewBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    pr_number: u64,
    review_id: u64,
}

impl<'octo, 'b> SpecificReviewBuilder<'octo, 'b> {
    pub(crate) fn new(
        handler: &'b PullRequestHandler<'octo>,
        pr_number: u64,
        review_id: u64,
    ) -> Self {
        Self {
            handler,
            pr_number,
            review_id,
        }
    }

    ///Retrieves a pull request review by its ID.
    ///see https://docs.github.com/en/rest/pulls/reviews?apiVersion=2022-11-28#get-a-review-for-a-pull-request
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    ///
    /// let _ = octocrab.pulls("owner", "repo")
    /// .pull_number(42)
    /// .reviews()
    /// .review(42)
    /// .get()
    /// .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self) -> crate::Result<crate::models::pulls::Review> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pull_number = self.pr_number,
            review_id = self.review_id
        );
        self.handler.crab.get(route, Some(&self)).await
    }

    ///Updates the contents of a specified review summary comment.
    ///see https://docs.github.com/en/rest/pulls/reviews?apiVersion=2022-11-28#update-a-review-for-a-pull-request
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    ///
    /// let _ = octocrab.pulls("owner", "repo")
    /// .pull_number(42)
    /// .reviews()
    /// .review(42)
    /// .update("this is a new body")
    /// .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        body: impl Into<String>,
    ) -> crate::Result<crate::models::pulls::Review> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pull_number = self.pr_number,
            review_id = self.review_id
        );
        self.handler.crab.patch(route, Some(&body.into())).await
    }

    ///Deletes a pull request review that has not been submitted. Submitted reviews cannot be deleted.
    ///see https://docs.github.com/en/rest/pulls/reviews?apiVersion=2022-11-28#delete-a-pending-review-for-a-pull-request
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    ///
    /// let _ = octocrab.pulls("owner", "repo")
    /// .pull_number(42)
    /// .reviews()
    /// .review(42)
    /// .delete_pending()
    /// .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_pending(&self) -> crate::Result<Review> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pull_number = self.pr_number,
            review_id = self.review_id
        );
        self.handler.crab.delete(route, None::<&()>).await
    }

    ///Submits a pending review for a pull request.
    ///see https://docs.github.com/en/rest/pulls/reviews?apiVersion=2022-11-28#submit-a-review-for-a-pull-request
    ///```no_run
    /// # use octocrab::models::pulls::ReviewAction;
    ///  async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    ///
    /// let _ = octocrab.pulls("owner", "repo")
    /// .pull_number(42)
    /// .reviews()
    /// .review(42)
    /// .submit(ReviewAction::RequestChanges, "comment body")
    /// .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn submit(
        &self,
        action: ReviewAction,
        body: impl Into<String>,
    ) -> crate::Result<Review> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/events",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pull_number = self.pr_number,
            review_id = self.review_id
        );
        self.handler
            .crab
            .post(
                route,
                Some(&serde_json::json!({ "body": body.into(), "event": action })),
            )
            .await
    }

    ///Dismisses a specified review on a pull request.
    ///see https://docs.github.com/en/rest/pulls/reviews?apiVersion=2022-11-28#dismiss-a-review-for-a-pull-request
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    ///
    /// let _ = octocrab.pulls("owner", "repo")
    /// .pull_number(42)
    /// .reviews()
    /// .review(42)
    /// .dismiss("message")
    /// .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn dismiss(&self, message: impl Into<String>) -> crate::Result<Review> {
        let route = format!(
            "/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/dismissals",
            owner = self.handler.owner,
            repo = self.handler.repo,
            pull_number = self.pr_number,
            review_id = self.review_id
        );
        self.handler
            .crab
            .put(
                route,
                Some(&serde_json::json!({ "message": message.into(), "event": "DISMISS" })),
            )
            .await
    }

    ///Lists comments for a specific pull request review.
    ///see https://docs.github.com/en/rest/pulls/reviews?apiVersion=2022-11-28#list-comments-for-a-pull-request-review
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    ///
    /// let _ = octocrab.pulls("owner", "repo")
    /// .pull_number(42)
    /// .reviews()
    /// .review(42)
    /// .list_comments()
    /// .per_page(10)
    /// .page(3u32)
    /// .send()
    /// .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_comments(&self) -> ListReviewCommentsBuilder<'octo, '_> {
        ListReviewCommentsBuilder::new(self.handler, self.pr_number, self.review_id)
    }
}
