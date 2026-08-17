use crate::models::pulls::{ReviewAction, ReviewComment};
use crate::pulls::specific_pr::pr_reviews::specific_review::SpecificReviewBuilder;
use crate::pulls::PullRequestHandler;

pub mod specific_review;

#[derive(serde::Serialize)]
pub struct ReviewsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    pr_number: u64,
}

impl<'octo, 'b> ReviewsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b PullRequestHandler<'octo>, pr_number: u64) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            pr_number,
        }
    }

    /// Creates a new `SpecificReviewBuilder`
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    ///
    /// let _ = octocrab.pulls("owner", "repo")
    /// .pull_number(42)
    /// .reviews()
    /// .review(42)
    /// .get() // + update, delete_pending, submit, dismiss, list_comments
    /// .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn review(&self, review_id: u64) -> SpecificReviewBuilder<'octo, '_> {
        SpecificReviewBuilder::new(self.handler, self.pr_number, review_id)
    }

    /// Creates a review for a pull request
    ///
    /// The fine-grained token must have the following permission set:
    /// * "Pull requests" repository permissions (write)
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    ///  use octocrab::models::pulls::{ReviewAction, ReviewComment};
    ///
    ///  let _ = octocrab.pulls("owner", "repo")
    ///  .pull_number(42)
    ///  .reviews()
    ///  .create_review(
    ///     "ecdd80bb57125d7ba9641ffaa4d7d2c19d3f3091",
    ///     "This is close to perfect! Please address the suggested inline change.",
    ///     ReviewAction::RequestChanges,
    ///     [ /* ReviewComment{ path: "file.md".to_string(), position: Some(6), body: "Please add more information".to_string() } */].to_vec()
    ///  )
    ///  .await?;
    ///  # Ok(())
    /// # }
    /// ```
    pub async fn create_review(
        &self,
        commit_id: impl Into<String>,
        body: impl Into<String>,
        event: ReviewAction,
        comments: Vec<ReviewComment>,
    ) -> crate::Result<crate::models::pulls::Review> {
        let url = format!(
            "/repos/{owner}/{repo}/pulls/{pull_number}/reviews",
            owner = &self.handler.owner,
            repo = &self.handler.repo,
            pull_number = &self.pr_number
        );
        let body = &serde_json::json!({ "body": body.into(), "event": event, "commit_id": commit_id.into(), "comments": comments });
        self.handler.crab.post(url, Some(&body)).await
    }
}
