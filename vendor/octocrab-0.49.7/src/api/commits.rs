//! The commit API.
mod associated_check_runs;
mod associated_pull_requests;
mod compare_commit;
mod create_comment;

pub use associated_pull_requests::PullRequestTarget;

pub use self::create_comment::CreateCommentBuilder;
use crate::params::repos::Reference;
use crate::{models, Octocrab, Result};

pub struct CommitHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

impl<'octo> CommitHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    // pub fn create(&self, title: impl Into<String>) -> create::CreateIssueBuilder<'_, '_> {
    //     create::CreateIssueBuilder::new(self, title.into())
    // }

    pub fn compare(
        &self,
        base: impl Into<String>,
        head: impl Into<String>,
    ) -> compare_commit::CompareCommitsBuilder<'_, '_> {
        compare_commit::CompareCommitsBuilder::new(self, base.into(), head.into())
    }

    pub fn associated_check_runs(
        &self,
        reference: impl Into<Reference>,
    ) -> associated_check_runs::AssociatedCheckRunsBuilder<'_, '_> {
        associated_check_runs::AssociatedCheckRunsBuilder::new(self, reference)
    }

    pub fn associated_pull_requests(
        &self,
        target: PullRequestTarget,
    ) -> associated_pull_requests::AssociatedPullRequestsBuilder<'_, '_> {
        associated_pull_requests::AssociatedPullRequestsBuilder::new(self, target)
    }

    pub fn create_comment(
        &self,
        sha: impl Into<String>,
        body: impl Into<String>,
    ) -> create_comment::CreateCommentBuilder<'_, '_> {
        create_comment::CreateCommentBuilder::new(self, sha.into(), body.into())
    }

    pub async fn get(&self, reference: impl Into<String>) -> Result<models::repos::RepoCommit> {
        let route = format!(
            "/repos/{owner}/{repo}/commits/{reference}",
            owner = self.owner,
            repo = self.repo,
            reference = reference.into(),
        );
        self.crab.get(route, None::<&()>).await
    }
}
