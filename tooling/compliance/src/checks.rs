use std::{fmt, ops::Not as _};

use itertools::Itertools as _;

use crate::{
    git::{CommitDetails, CommitList},
    github::{
        CommitAuthor, GitHubClient, GitHubUser, GithubLogin, PullRequestComment, PullRequestData,
        PullRequestReview, ReviewState,
    },
    report::Report,
};

const ZED_ZIPPY_COMMENT_APPROVAL_PATTERN: &str = "@zed-zippy approve";
const ZED_ZIPPY_GROUP_APPROVAL: &str = "@zed-industries/approved";

#[derive(Debug)]
pub enum ReviewSuccess {
    ApprovingComment(Vec<PullRequestComment>),
    CoAuthored(Vec<CommitAuthor>),
    ExternalMergedContribution { merged_by: GitHubUser },
    PullRequestReviewed(Vec<PullRequestReview>),
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
            Self::ExternalMergedContribution { merged_by } => {
                vec![format!("@{}", merged_by.login)]
            }
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
    async fn check_commit(&self, commit: &CommitDetails) -> Result<ReviewSuccess, ReviewFailure> {
        let Some(pr_number) = commit.pr_number() else {
            return Err(ReviewFailure::NoPullRequestFound);
        };

        let pull_request = self.github_client.get_pull_request(pr_number).await?;

        if let Some(approval) = self
            .check_approving_pull_request_review(&pull_request)
            .await?
        {
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
        &self,
        commit: &CommitDetails,
    ) -> Result<Option<ReviewSuccess>, ReviewFailure> {
        if commit.co_authors().is_some()
            && let Some(commit_authors) = self
                .github_client
                .get_commit_authors(&[commit.sha()])
                .await?
                .get(commit.sha())
                .and_then(|authors| authors.co_authors())
        {
            let mut org_co_authors = Vec::new();
            for co_author in commit_authors {
                if let Some(github_login) = co_author.user()
                    && self
                        .github_client
                        .actor_has_repository_write_permission(github_login)
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
        &self,
        pull_request: PullRequestData,
    ) -> Result<Option<ReviewSuccess>, ReviewFailure> {
        if let Some(user) = pull_request.user
            && self
                .github_client
                .actor_has_repository_write_permission(&GithubLogin::new(user.login))
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

    async fn check_approving_pull_request_review(
        &self,
        pull_request: &PullRequestData,
    ) -> Result<Option<ReviewSuccess>, ReviewFailure> {
        let pr_reviews = self
            .github_client
            .get_pull_request_reviews(pull_request.number)
            .await?;

        if !pr_reviews.is_empty() {
            let mut org_approving_reviews = Vec::new();
            for review in pr_reviews {
                if let Some(github_login) = review.user.as_ref()
                    && pull_request
                        .user
                        .as_ref()
                        .is_none_or(|pr_user| pr_user.login != github_login.login)
                    && (review
                        .state
                        .is_some_and(|state| state == ReviewState::Approved)
                        || review
                            .body
                            .as_deref()
                            .is_some_and(Self::contains_approving_pattern))
                    && self
                        .github_client
                        .actor_has_repository_write_permission(&GithubLogin::new(
                            github_login.login.clone(),
                        ))
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
        &self,
        pull_request: &PullRequestData,
    ) -> Result<Option<ReviewSuccess>, ReviewFailure> {
        let other_comments = self
            .github_client
            .get_pull_request_comments(pull_request.number)
            .await?;

        if !other_comments.is_empty() {
            let mut org_approving_comments = Vec::new();

            for comment in other_comments {
                if pull_request
                    .user
                    .as_ref()
                    .is_some_and(|pr_author| pr_author.login != comment.user.login)
                    && comment
                        .body
                        .as_deref()
                        .is_some_and(Self::contains_approving_pattern)
                    && self
                        .github_client
                        .actor_has_repository_write_permission(&GithubLogin::new(
                            comment.user.login.clone(),
                        ))
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

    fn contains_approving_pattern(body: &str) -> bool {
        body.contains(ZED_ZIPPY_COMMENT_APPROVAL_PATTERN) || body.contains(ZED_ZIPPY_GROUP_APPROVAL)
    }

    pub async fn generate_report(mut self) -> anyhow::Result<Report> {
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

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::str::FromStr;

    use crate::git::{CommitDetails, CommitList, CommitSha};
    use crate::github::{
        AuthorsForCommits, GitHubApiClient, GitHubClient, GitHubUser, GithubLogin,
        PullRequestComment, PullRequestData, PullRequestReview, ReviewState,
    };

    use super::{Reporter, ReviewFailure, ReviewSuccess};

    struct MockGitHubApi {
        pull_request: PullRequestData,
        reviews: Vec<PullRequestReview>,
        comments: Vec<PullRequestComment>,
        commit_authors_json: serde_json::Value,
        org_members: Vec<String>,
    }

    #[async_trait::async_trait(?Send)]
    impl GitHubApiClient for MockGitHubApi {
        async fn get_pull_request(&self, _pr_number: u64) -> anyhow::Result<PullRequestData> {
            Ok(self.pull_request.clone())
        }

        async fn get_pull_request_reviews(
            &self,
            _pr_number: u64,
        ) -> anyhow::Result<Vec<PullRequestReview>> {
            Ok(self.reviews.clone())
        }

        async fn get_pull_request_comments(
            &self,
            _pr_number: u64,
        ) -> anyhow::Result<Vec<PullRequestComment>> {
            Ok(self.comments.clone())
        }

        async fn get_commit_authors(
            &self,
            _commit_shas: &[&CommitSha],
        ) -> anyhow::Result<AuthorsForCommits> {
            serde_json::from_value(self.commit_authors_json.clone()).map_err(Into::into)
        }

        async fn check_org_membership(&self, login: &GithubLogin) -> anyhow::Result<bool> {
            Ok(self
                .org_members
                .iter()
                .any(|member| member == login.as_str()))
        }

        async fn check_repo_write_permission(&self, _login: &GithubLogin) -> anyhow::Result<bool> {
            Ok(false)
        }

        async fn ensure_pull_request_has_label(
            &self,
            _label: &str,
            _pr_number: u64,
        ) -> anyhow::Result<()> {
            Ok(())
        }
    }

    fn make_commit(
        sha: &str,
        author_name: &str,
        author_email: &str,
        title: &str,
        body: &str,
    ) -> CommitDetails {
        let formatted = format!(
            "{sha}|field-delimiter|{author_name}|field-delimiter|{author_email}|field-delimiter|\
             {title}|body-delimiter|{body}|commit-delimiter|"
        );
        CommitList::from_str(&formatted)
            .expect("test commit should parse")
            .into_iter()
            .next()
            .expect("should have one commit")
    }

    fn review(login: &str, state: ReviewState) -> PullRequestReview {
        PullRequestReview {
            user: Some(GitHubUser {
                login: login.to_owned(),
            }),
            state: Some(state),
            body: None,
        }
    }

    fn comment(login: &str, body: &str) -> PullRequestComment {
        PullRequestComment {
            user: GitHubUser {
                login: login.to_owned(),
            },
            body: Some(body.to_owned()),
        }
    }

    struct TestScenario {
        pull_request: PullRequestData,
        reviews: Vec<PullRequestReview>,
        comments: Vec<PullRequestComment>,
        commit_authors_json: serde_json::Value,
        org_members: Vec<String>,
        commit: CommitDetails,
    }

    impl TestScenario {
        fn single_commit() -> Self {
            Self {
                pull_request: PullRequestData {
                    number: 1234,
                    user: Some(GitHubUser {
                        login: "alice".to_owned(),
                    }),
                    merged_by: None,
                },
                reviews: vec![],
                comments: vec![],
                commit_authors_json: serde_json::json!({}),
                org_members: vec![],
                commit: make_commit(
                    "abc12345abc12345",
                    "Alice",
                    "alice@test.com",
                    "Fix thing (#1234)",
                    "",
                ),
            }
        }

        fn with_reviews(mut self, reviews: Vec<PullRequestReview>) -> Self {
            self.reviews = reviews;
            self
        }

        fn with_comments(mut self, comments: Vec<PullRequestComment>) -> Self {
            self.comments = comments;
            self
        }

        fn with_org_members(mut self, members: Vec<&str>) -> Self {
            self.org_members = members.into_iter().map(str::to_owned).collect();
            self
        }

        fn with_commit_authors_json(mut self, json: serde_json::Value) -> Self {
            self.commit_authors_json = json;
            self
        }

        fn with_commit(mut self, commit: CommitDetails) -> Self {
            self.commit = commit;
            self
        }

        async fn run_scenario(self) -> Result<ReviewSuccess, ReviewFailure> {
            let mock = MockGitHubApi {
                pull_request: self.pull_request,
                reviews: self.reviews,
                comments: self.comments,
                commit_authors_json: self.commit_authors_json,
                org_members: self.org_members,
            };
            let client = GitHubClient::new(Rc::new(mock));
            let reporter = Reporter::new(CommitList::default(), &client);
            reporter.check_commit(&self.commit).await
        }
    }

    #[tokio::test]
    async fn approved_review_by_org_member_succeeds() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![review("bob", ReviewState::Approved)])
            .with_org_members(vec!["bob"])
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::PullRequestReviewed(_))));
    }

    #[tokio::test]
    async fn non_approved_review_state_is_not_accepted() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![review("bob", ReviewState::Other)])
            .with_org_members(vec!["bob"])
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }

    #[tokio::test]
    async fn review_by_non_org_member_is_not_accepted() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![review("bob", ReviewState::Approved)])
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }

    #[tokio::test]
    async fn pr_author_own_approval_review_is_rejected() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![review("alice", ReviewState::Approved)])
            .with_org_members(vec!["alice"])
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }

    #[tokio::test]
    async fn pr_author_own_approval_comment_is_rejected() {
        let result = TestScenario::single_commit()
            .with_comments(vec![comment("alice", "@zed-zippy approve")])
            .with_org_members(vec!["alice"])
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }

    #[tokio::test]
    async fn approval_comment_by_org_member_succeeds() {
        let result = TestScenario::single_commit()
            .with_comments(vec![comment("bob", "@zed-zippy approve")])
            .with_org_members(vec!["bob"])
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::ApprovingComment(_))));
    }

    #[tokio::test]
    async fn group_approval_comment_by_org_member_succeeds() {
        let result = TestScenario::single_commit()
            .with_comments(vec![comment("bob", "@zed-industries/approved")])
            .with_org_members(vec!["bob"])
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::ApprovingComment(_))));
    }

    #[tokio::test]
    async fn comment_without_approval_pattern_is_not_accepted() {
        let result = TestScenario::single_commit()
            .with_comments(vec![comment("bob", "looks good")])
            .with_org_members(vec!["bob"])
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }

    #[tokio::test]
    async fn commit_without_pr_number_is_no_pr_found() {
        let result = TestScenario::single_commit()
            .with_commit(make_commit(
                "abc12345abc12345",
                "Alice",
                "alice@test.com",
                "Fix thing without PR number",
                "",
            ))
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::NoPullRequestFound)));
    }

    #[tokio::test]
    async fn pr_review_takes_precedence_over_comment() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![review("bob", ReviewState::Approved)])
            .with_comments(vec![comment("charlie", "@zed-zippy approve")])
            .with_org_members(vec!["bob", "charlie"])
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::PullRequestReviewed(_))));
    }

    #[tokio::test]
    async fn comment_takes_precedence_over_co_author() {
        let result = TestScenario::single_commit()
            .with_comments(vec![comment("bob", "@zed-zippy approve")])
            .with_commit_authors_json(serde_json::json!({
                "abc12345abc12345": {
                    "author": {
                        "name": "Alice",
                        "email": "alice@test.com",
                        "user": { "login": "alice" }
                    },
                    "authors": [{
                        "name": "Charlie",
                        "email": "charlie@test.com",
                        "user": { "login": "charlie" }
                    }]
                }
            }))
            .with_commit(make_commit(
                "abc12345abc12345",
                "Alice",
                "alice@test.com",
                "Fix thing (#1234)",
                "Co-authored-by: Charlie <charlie@test.com>",
            ))
            .with_org_members(vec!["bob", "charlie"])
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::ApprovingComment(_))));
    }

    #[tokio::test]
    async fn co_author_org_member_succeeds() {
        let result = TestScenario::single_commit()
            .with_commit_authors_json(serde_json::json!({
                "abc12345abc12345": {
                    "author": {
                        "name": "Alice",
                        "email": "alice@test.com",
                        "user": { "login": "alice" }
                    },
                    "authors": [{
                        "name": "Bob",
                        "email": "bob@test.com",
                        "user": { "login": "bob" }
                    }]
                }
            }))
            .with_commit(make_commit(
                "abc12345abc12345",
                "Alice",
                "alice@test.com",
                "Fix thing (#1234)",
                "Co-authored-by: Bob <bob@test.com>",
            ))
            .with_org_members(vec!["bob"])
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::CoAuthored(_))));
    }

    #[tokio::test]
    async fn no_reviews_no_comments_no_coauthors_is_unreviewed() {
        let result = TestScenario::single_commit().run_scenario().await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }

    #[tokio::test]
    async fn review_with_zippy_approval_body_is_accepted() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![
                review("bob", ReviewState::Other).with_body("@zed-zippy approve"),
            ])
            .with_org_members(vec!["bob"])
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::PullRequestReviewed(_))));
    }

    #[tokio::test]
    async fn review_with_group_approval_body_is_accepted() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![
                review("bob", ReviewState::Other).with_body("@zed-industries/approved"),
            ])
            .with_org_members(vec!["bob"])
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::PullRequestReviewed(_))));
    }

    #[tokio::test]
    async fn review_with_non_approving_body_is_not_accepted() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![
                review("bob", ReviewState::Other).with_body("looks good to me"),
            ])
            .with_org_members(vec!["bob"])
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }

    #[tokio::test]
    async fn review_with_approving_body_from_external_user_is_not_accepted() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![
                review("bob", ReviewState::Other).with_body("@zed-zippy approve"),
            ])
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }

    #[tokio::test]
    async fn review_with_approving_body_from_pr_author_is_rejected() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![
                review("alice", ReviewState::Other).with_body("@zed-zippy approve"),
            ])
            .with_org_members(vec!["alice"])
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }
}
