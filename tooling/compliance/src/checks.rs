use std::{fmt, ops::Not as _};

use itertools::Itertools as _;
use octocrab::models::{
    Author,
    issues::Comment,
    pulls::{PullRequest, Review, ReviewState},
};

use crate::{
    git::{CommitDetails, CommitList},
    github::{CommitAuthor, GitHubClient, GithubLogin},
    report::Report,
};

const ZED_ZIPPY_COMMENT_APPROVAL_PATTERN: &str = "@zed-zippy approve";
const ZED_ZIPPY_GROUP_APPROVAL: &str = "@zed-industries/approved";

#[derive(Debug)]
pub enum ReviewSuccess {
    ApprovingComment(Vec<Comment>),
    CoAuthored(Vec<CommitAuthor>),
    ExternalMergedContribution { merged_by: Box<Author> },
    PullRequestReviewed(Vec<Review>),
}

impl ReviewSuccess {
    pub(crate) fn reviewers(&self) -> anyhow::Result<String> {
        let reviewers = match self {
            Self::CoAuthored(authors) => authors.iter().map(ToString::to_string).collect_vec(),
            Self::PullRequestReviewed(reviews) => reviews
                .iter()
                .filter_map(|review| review.user.as_ref())
                .map(|user| format!("@{}", user.login))
                .collect_vec(),
            Self::ApprovingComment(comments) => comments
                .iter()
                .map(|comment| format!("@{}", comment.user.login))
                .collect_vec(),
            Self::ExternalMergedContribution { merged_by } => vec![format!("@{}", merged_by.login)],
        };

        let reviewers = reviewers.into_iter().unique().collect_vec();

        reviewers
            .is_empty()
            .not()
            .then(|| reviewers.join(", "))
            .ok_or_else(|| anyhow::anyhow!("Expected at least one reviewer"))
    }
}

impl fmt::Display for ReviewSuccess {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CoAuthored(_) => formatter.write_str("Co-authored by an organization member"),
            Self::PullRequestReviewed(_) => {
                formatter.write_str("Approved by an organization review")
            }
            Self::ApprovingComment(_) => {
                formatter.write_str("Approved by an organization approval comment")
            }
            Self::ExternalMergedContribution { .. } => {
                formatter.write_str("External merged contribution")
            }
        }
    }
}

#[derive(Debug)]
pub enum ReviewFailure {
    // todo: We could still query the GitHub API here to search for one
    NoPullRequestFound,
    Unreviewed,
    UnableToDetermineReviewer,
    Other(anyhow::Error),
}

impl fmt::Display for ReviewFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoPullRequestFound => formatter.write_str("No pull request found"),
            Self::Unreviewed => formatter
                .write_str("No qualifying organization approval found for the pull request"),
            Self::UnableToDetermineReviewer => formatter.write_str("Could not determine reviewer"),
            Self::Other(error) => write!(formatter, "Failed to inspect review state: {error}"),
        }
    }
}

pub(crate) type ReviewResult = Result<ReviewSuccess, ReviewFailure>;

impl<E: Into<anyhow::Error>> From<E> for ReviewFailure {
    fn from(err: E) -> Self {
        Self::Other(anyhow::anyhow!(err))
    }
}

pub struct Reporter<'a> {
    commits: CommitList,
    github_client: &'a GitHubClient,
}

impl<'a> Reporter<'a> {
    pub fn new(commits: CommitList, github_client: &'a GitHubClient) -> Self {
        Self {
            commits,
            github_client,
        }
    }

    /// Method that checks every commit for compliance
    async fn check_commit(
        &mut self,
        commit: &CommitDetails,
    ) -> Result<ReviewSuccess, ReviewFailure> {
        let Some(pr_number) = commit.pr_number() else {
            return Err(ReviewFailure::NoPullRequestFound);
        };

        let pull_request = self.github_client.get_pull_request(pr_number).await?;

        if let Some(approval) = self.check_pull_request_approved(&pull_request).await? {
            return Ok(approval);
        }

        if let Some(approval) = self
            .check_approving_pull_request_comment(&pull_request)
            .await?
        {
            return Ok(approval);
        }

        if let Some(approval) = self.check_commit_co_authors(commit).await? {
            return Ok(approval);
        }

        // if let Some(approval) = self.check_external_merged_pr(pr_number).await? {
        //     return Ok(approval);
        // }

        Err(ReviewFailure::Unreviewed)
    }

    async fn check_commit_co_authors(
        &mut self,
        commit: &CommitDetails,
    ) -> Result<Option<ReviewSuccess>, ReviewFailure> {
        if commit.co_authors().is_some()
            && let Some(commit_authors) = self
                .github_client
                .get_commit_co_authors([commit.sha()])
                .await?
                .get(commit.sha())
                .and_then(|authors| authors.co_authors())
        {
            let mut org_co_authors = Vec::new();
            for co_author in commit_authors {
                if let Some(github_login) = co_author.user()
                    && self
                        .github_client
                        .check_org_membership(github_login)
                        .await?
                {
                    org_co_authors.push(co_author.clone());
                }
            }

            Ok(org_co_authors
                .is_empty()
                .not()
                .then_some(ReviewSuccess::CoAuthored(org_co_authors)))
        } else {
            Ok(None)
        }
    }

    #[allow(unused)]
    async fn check_external_merged_pr(
        &mut self,
        pull_request: PullRequest,
    ) -> Result<Option<ReviewSuccess>, ReviewFailure> {
        if let Some(user) = pull_request.user
            && self
                .github_client
                .check_org_membership(&GithubLogin::new(user.login))
                .await?
                .not()
        {
            pull_request.merged_by.map_or(
                Err(ReviewFailure::UnableToDetermineReviewer),
                |merged_by| {
                    Ok(Some(ReviewSuccess::ExternalMergedContribution {
                        merged_by,
                    }))
                },
            )
        } else {
            Ok(None)
        }
    }

    async fn check_pull_request_approved(
        &mut self,
        pull_request: &PullRequest,
    ) -> Result<Option<ReviewSuccess>, ReviewFailure> {
        let pr_reviews = self
            .github_client
            .get_pr_reviews(pull_request.number)
            .await?
            .collect_vec();

        if !pr_reviews.is_empty() {
            let mut org_approving_reviews = Vec::new();
            for review in pr_reviews {
                if let Some(github_login) = review.user.as_ref()
                    && pull_request
                        .user
                        .as_ref()
                        .is_none_or(|pr_user| pr_user.login != github_login.login)
                    && review
                        .state
                        .is_some_and(|state| state == ReviewState::Approved)
                    && self
                        .github_client
                        .check_org_membership(&GithubLogin::new(github_login.login.clone()))
                        .await?
                {
                    org_approving_reviews.push(review);
                }
            }

            Ok(org_approving_reviews
                .is_empty()
                .not()
                .then_some(ReviewSuccess::PullRequestReviewed(org_approving_reviews)))
        } else {
            Ok(None)
        }
    }

    async fn check_approving_pull_request_comment(
        &mut self,
        pull_request: &PullRequest,
    ) -> Result<Option<ReviewSuccess>, ReviewFailure> {
        let other_comments = self
            .github_client
            .get_pr_comments(pull_request.number)
            .await?
            .collect_vec();

        if !other_comments.is_empty() {
            let mut org_approving_comments = Vec::new();

            for comment in other_comments {
                if pull_request
                    .user
                    .as_ref()
                    .is_some_and(|pr_author| pr_author.login != comment.user.login)
                    && comment.body.as_ref().is_some_and(|body| {
                        body.contains(ZED_ZIPPY_COMMENT_APPROVAL_PATTERN)
                            || body.contains(ZED_ZIPPY_GROUP_APPROVAL)
                    })
                    && self
                        .github_client
                        .check_org_membership(&GithubLogin::new(comment.user.login.clone()))
                        .await?
                {
                    org_approving_comments.push(comment);
                }
            }

            Ok(org_approving_comments
                .is_empty()
                .not()
                .then_some(ReviewSuccess::ApprovingComment(org_approving_comments)))
        } else {
            Ok(None)
        }
    }

    pub async fn generate_report(&mut self) -> anyhow::Result<Report> {
        let mut report = Report::new();

        let commits_to_check = std::mem::take(&mut self.commits);
        let total_commits = commits_to_check.len();

        for (i, commit) in commits_to_check.into_iter().enumerate() {
            println!(
                "Checking commit {:?} ({current}/{total})",
                commit.sha().short(),
                current = i + 1,
                total = total_commits
            );

            let review_result = self.check_commit(&commit).await;

            if let Err(err) = &review_result {
                println!("Commit {:?} failed review: {:?}", commit.sha().short(), err);
            }

            report.add(commit, review_result);
        }

        Ok(report)
    }
}
