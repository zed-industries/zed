use std::{fmt, ops::Not as _, rc::Rc};

use futures::StreamExt;
use itertools::Itertools as _;

use crate::{
    git::{AutomatedChangeKind, CommitDetails, CommitList, ZED_ZIPPY_LOGIN},
    github::{
        Approvable, CommitAuthor, CommitFileChange, CommitMetadata, GithubApiClient, GithubLogin,
        PullRequestComment, PullRequestData, PullRequestReview, Repository, ReviewState,
    },
    report::{Report, ReportEntry},
};

const ZED_ZIPPY_COMMENT_APPROVAL_PATTERN: &str = "@zed-zippy approve";
const ZED_ZIPPY_GROUP_APPROVAL: &str = "@zed-industries/approved";

#[derive(Debug)]
pub enum ReviewSuccess {
    ApprovingComment(Vec<PullRequestComment>),
    CoAuthored(Vec<CommitAuthor>),
    PullRequestReviewed(Vec<PullRequestReview>),
    ZedZippyCommit(AutomatedChangeKind, GithubLogin),
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
            Self::ZedZippyCommit(_, login) => vec![login.to_string()],
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
            Self::ZedZippyCommit(kind, _) => {
                write!(formatter, "Fully untampered automated {kind}")
            }
        }
    }
}

#[derive(Debug)]
pub enum ReviewFailure {
    // todo: We could still query the GitHub API here to search for one
    NoPullRequestFound,
    Unreviewed,
    UnexpectedZippyAction(AutomatedChangeFailure),
    Other(anyhow::Error),
}

impl fmt::Display for ReviewFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoPullRequestFound => formatter.write_str("No pull request found"),
            Self::Unreviewed => formatter
                .write_str("No qualifying organization approval found for the pull request"),
            Self::UnexpectedZippyAction(failure) => {
                write!(formatter, "Validating Zed Zippy change failed: {failure}")
            }
            Self::Other(error) => write!(formatter, "Failed to inspect review state: {error}"),
        }
    }
}

#[derive(Debug)]
pub enum AutomatedChangeFailure {
    NoMentionInTitle,
    MissingCommitData,
    AuthorMismatch,
    UnexpectedCoAuthors,
    NotSigned,
    InvalidSignature,
    UnexpectedLineChanges {
        kind: AutomatedChangeKind,
        additions: u64,
        deletions: u64,
    },
    UnexpectedFiles {
        kind: AutomatedChangeKind,
        found: Vec<String>,
    },
}

impl fmt::Display for AutomatedChangeFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoMentionInTitle => formatter.write_str("No @-mention found in commit title"),
            Self::MissingCommitData => formatter.write_str("No commit data found on GitHub"),
            Self::AuthorMismatch => {
                formatter.write_str("GitHub author does not match bot identity")
            }
            Self::UnexpectedCoAuthors => formatter.write_str("Commit has unexpected co-authors"),
            Self::NotSigned => formatter.write_str("Commit is not signed"),
            Self::InvalidSignature => formatter.write_str("Commit signature is invalid"),
            Self::UnexpectedLineChanges {
                kind,
                additions,
                deletions,
            } => {
                write!(
                    formatter,
                    "Unexpected line changes for {kind} \
                     ({additions} additions, {deletions} deletions, \
                     expected {} each)",
                    kind.expected_loc()
                )
            }
            Self::UnexpectedFiles { kind, found } => {
                let expected = kind.expected_files().join(", ");
                let actual = found.join(", ");
                write!(
                    formatter,
                    "Unexpected files changed for {kind} \
                     (expected [{expected}], found [{actual}])"
                )
            }
        }
    }
}

impl AutomatedChangeKind {
    fn validate_changes(
        self,
        metadata: &CommitMetadata,
        files: &[CommitFileChange],
    ) -> Result<(), AutomatedChangeFailure> {
        let expected_loc = self.expected_loc();
        if metadata.additions() != expected_loc || metadata.deletions() != expected_loc {
            return Err(AutomatedChangeFailure::UnexpectedLineChanges {
                kind: self,
                additions: metadata.additions(),
                deletions: metadata.deletions(),
            });
        }

        let files_differ = files.len() != self.expected_files().len()
            || files
                .iter()
                .any(|f| self.expected_files().contains(&f.filename.as_str()).not());

        if files_differ {
            return Err(AutomatedChangeFailure::UnexpectedFiles {
                kind: self,
                found: files.into_iter().map(|f| f.filename.clone()).collect(),
            });
        }

        Ok(())
    }
}

pub(crate) type ReviewResult = Result<ReviewSuccess, ReviewFailure>;

impl<E: Into<anyhow::Error>> From<E> for ReviewFailure {
    fn from(err: E) -> Self {
        Self::Other(anyhow::anyhow!(err))
    }
}

pub struct Reporter {
    commits: CommitList,
    github_client: Rc<dyn GithubApiClient>,
}

impl Reporter {
    pub fn new(commits: CommitList, github_client: Rc<dyn GithubApiClient>) -> Self {
        Self {
            commits,
            github_client,
        }
    }

    pub async fn result_for_commit(
        commit: CommitDetails,
        github_client: Rc<dyn GithubApiClient>,
    ) -> ReviewResult {
        Self::new(Default::default(), github_client)
            .check_commit(&commit)
            .await
    }

    /// Method that checks every commit for compliance
    pub async fn check_commit(
        &self,
        commit: &CommitDetails,
    ) -> Result<ReviewSuccess, ReviewFailure> {
        let Some(pr_number) = commit.pr_number() else {
            if commit.author().is_zed_zippy() {
                return self.check_zippy_automated_change(commit).await;
            } else {
                return Err(ReviewFailure::NoPullRequestFound);
            }
        };

        let pull_request = self
            .github_client
            .get_pull_request(&Repository::ZED, pr_number)
            .await?;

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

        Err(ReviewFailure::Unreviewed)
    }

    async fn check_zippy_automated_change(
        &self,
        commit: &CommitDetails,
    ) -> Result<ReviewSuccess, ReviewFailure> {
        let (change_kind, responsible_actor) =
            commit
                .detect_automated_change()
                .ok_or(ReviewFailure::UnexpectedZippyAction(
                    AutomatedChangeFailure::NoMentionInTitle,
                ))?;

        let commit_data = self
            .github_client
            .get_commit_metadata(&Repository::ZED, &[commit.sha()])
            .await?;

        let metadata =
            commit_data
                .get(commit.sha())
                .ok_or(ReviewFailure::UnexpectedZippyAction(
                    AutomatedChangeFailure::MissingCommitData,
                ))?;

        if !metadata
            .primary_author()
            .user()
            .is_some_and(|login| login.as_str() == ZED_ZIPPY_LOGIN)
        {
            return Err(ReviewFailure::UnexpectedZippyAction(
                AutomatedChangeFailure::AuthorMismatch,
            ));
        }

        if metadata.co_authors().is_some() {
            return Err(ReviewFailure::UnexpectedZippyAction(
                AutomatedChangeFailure::UnexpectedCoAuthors,
            ));
        }

        let signature = metadata
            .signature()
            .ok_or(ReviewFailure::UnexpectedZippyAction(
                AutomatedChangeFailure::NotSigned,
            ))?;

        if !signature.is_valid() {
            return Err(ReviewFailure::UnexpectedZippyAction(
                AutomatedChangeFailure::InvalidSignature,
            ));
        }

        let files = self
            .github_client
            .get_commit_files(&Repository::ZED, commit.sha())
            .await?;

        change_kind
            .validate_changes(metadata, &files)
            .map_err(ReviewFailure::UnexpectedZippyAction)?;

        Ok(ReviewSuccess::ZedZippyCommit(
            change_kind,
            GithubLogin::new(responsible_actor.to_owned()),
        ))
    }

    async fn check_commit_co_authors(
        &self,
        commit: &CommitDetails,
    ) -> Result<Option<ReviewSuccess>, ReviewFailure> {
        if commit.co_authors().is_some()
            && let Some(commit_authors) = self
                .github_client
                .get_commit_metadata(&Repository::ZED, &[commit.sha()])
                .await?
                .get(commit.sha())
                .and_then(|authors| authors.co_authors())
        {
            let mut org_co_authors = Vec::new();
            for co_author in commit_authors {
                if let Some(github_login) = co_author.user()
                    && self
                        .github_client
                        .check_repo_write_permission(&Repository::ZED, github_login)
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

    async fn check_approving_pull_request_review(
        &self,
        pull_request: &PullRequestData,
    ) -> Result<Option<ReviewSuccess>, ReviewFailure> {
        let reviews = self
            .github_client
            .get_pull_request_reviews(&Repository::ZED, pull_request.number)
            .await?;

        let qualifying_reviews = reviews
            .into_iter()
            .filter(|review| Self::is_qualifying_approval(review, pull_request))
            .collect_vec();

        Ok(qualifying_reviews
            .is_empty()
            .not()
            .then_some(ReviewSuccess::PullRequestReviewed(qualifying_reviews)))
    }

    async fn check_approving_pull_request_comment(
        &self,
        pull_request: &PullRequestData,
    ) -> Result<Option<ReviewSuccess>, ReviewFailure> {
        let comments = self
            .github_client
            .get_pull_request_comments(&Repository::ZED, pull_request.number)
            .await?;

        let qualifying_comments = comments
            .into_iter()
            .filter(|comment| Self::is_qualifying_approval(comment, pull_request))
            .collect_vec();

        Ok(qualifying_comments
            .is_empty()
            .not()
            .then_some(ReviewSuccess::ApprovingComment(qualifying_comments)))
    }

    pub fn is_qualifying_approval(item: &impl Approvable, pull_request: &PullRequestData) -> bool {
        let Some(author_login) = item.author_login() else {
            return false;
        };

        let distinct_actor = pull_request
            .user
            .as_ref()
            .is_none_or(|pr_user| pr_user.login != author_login);

        let approving_pattern = item
            .review_state()
            .is_some_and(|state| state == ReviewState::Approved)
            || item.body().is_some_and(Self::contains_approving_pattern);

        let actor_is_authorized = item
            .author_association()
            .is_some_and(|association| association.has_write_access());

        distinct_actor && approving_pattern && actor_is_authorized
    }

    fn contains_approving_pattern(body: &str) -> bool {
        body.contains(ZED_ZIPPY_COMMENT_APPROVAL_PATTERN) || body.contains(ZED_ZIPPY_GROUP_APPROVAL)
    }

    pub async fn generate_report(mut self, max_concurrent_checks: usize) -> Report {
        let commits_to_check = std::mem::take(&mut self.commits);
        let total_commits = commits_to_check.len();

        let reports = futures::stream::iter(commits_to_check.into_iter().enumerate().map(
            async |(i, commit)| {
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

                (commit, review_result)
            },
        ))
        .buffered(max_concurrent_checks)
        .collect::<Vec<_>>()
        .await;

        Report::from_entries(
            reports
                .into_iter()
                .map(|(commit, result)| ReportEntry::new(commit, result)),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::str::FromStr;

    use crate::git::{
        AutomatedChangeKind, CommitDetails, CommitList, CommitSha, ZED_ZIPPY_EMAIL, ZED_ZIPPY_LOGIN,
    };
    use crate::github::{
        AuthorAssociation, CommitFileChange, CommitMetadataBySha, GithubApiClient, GithubLogin,
        GithubUser, PullRequestComment, PullRequestData, PullRequestReview, Repository,
        ReviewState,
    };

    use super::{AutomatedChangeFailure, Reporter, ReviewFailure, ReviewSuccess};

    struct MockGithubApi {
        pull_request: PullRequestData,
        reviews: Vec<PullRequestReview>,
        comments: Vec<PullRequestComment>,
        commit_metadata_json: serde_json::Value,
        commit_files: Vec<CommitFileChange>,
        org_members: Vec<String>,
    }

    #[async_trait::async_trait(?Send)]
    impl GithubApiClient for MockGithubApi {
        async fn get_pull_request(
            &self,
            _repo: &Repository<'_>,
            _pr_number: u64,
        ) -> anyhow::Result<PullRequestData> {
            Ok(self.pull_request.clone())
        }

        async fn get_pull_request_reviews(
            &self,
            _repo: &Repository<'_>,
            _pr_number: u64,
        ) -> anyhow::Result<Vec<PullRequestReview>> {
            Ok(self.reviews.clone())
        }

        async fn get_pull_request_comments(
            &self,
            _repo: &Repository<'_>,
            _pr_number: u64,
        ) -> anyhow::Result<Vec<PullRequestComment>> {
            Ok(self.comments.clone())
        }

        async fn get_commit_metadata(
            &self,
            _repo: &Repository<'_>,
            _commit_shas: &[&CommitSha],
        ) -> anyhow::Result<CommitMetadataBySha> {
            serde_json::from_value(self.commit_metadata_json.clone()).map_err(Into::into)
        }

        async fn get_commit_files(
            &self,
            _repo: &Repository<'_>,
            _sha: &CommitSha,
        ) -> anyhow::Result<Vec<CommitFileChange>> {
            Ok(self.commit_files.clone())
        }

        async fn check_repo_write_permission(
            &self,
            _repo: &Repository<'_>,
            login: &GithubLogin,
        ) -> anyhow::Result<bool> {
            Ok(self
                .org_members
                .iter()
                .any(|member| member == login.as_str()))
        }

        async fn add_label_to_issue(
            &self,
            _repo: &Repository<'_>,
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

    fn review(
        login: &str,
        state: ReviewState,
        author_association: AuthorAssociation,
    ) -> PullRequestReview {
        PullRequestReview {
            user: Some(GithubUser {
                login: login.to_owned(),
            }),
            state: Some(state),
            body: None,
            author_association: Some(author_association),
        }
    }

    fn comment(
        login: &str,
        body: &str,
        author_association: AuthorAssociation,
    ) -> PullRequestComment {
        PullRequestComment {
            user: GithubUser {
                login: login.to_owned(),
            },
            body: Some(body.to_owned()),
            author_association: Some(author_association),
        }
    }

    fn alice_author() -> serde_json::Value {
        serde_json::json!({
            "name": "Alice",
            "email": "alice@test.com",
            "user": { "login": "alice" }
        })
    }

    fn bob_author() -> serde_json::Value {
        serde_json::json!({
            "name": "Bob",
            "email": "bob@test.com",
            "user": { "login": "bob" }
        })
    }

    fn charlie_author() -> serde_json::Value {
        serde_json::json!({
            "name": "Charlie",
            "email": "charlie@test.com",
            "user": { "login": "charlie" }
        })
    }

    fn zippy_author() -> serde_json::Value {
        serde_json::json!({
            "name": "Zed Zippy",
            "email": ZED_ZIPPY_EMAIL,
            "user": { "login": ZED_ZIPPY_LOGIN }
        })
    }

    struct TestScenario {
        pull_request: PullRequestData,
        reviews: Vec<PullRequestReview>,
        comments: Vec<PullRequestComment>,
        commit_metadata_json: serde_json::Value,
        commit_files: Vec<CommitFileChange>,
        org_members: Vec<String>,
        commit: CommitDetails,
    }

    impl TestScenario {
        fn single_commit() -> Self {
            Self {
                pull_request: PullRequestData {
                    number: 1234,
                    user: Some(GithubUser {
                        login: "alice".to_owned(),
                    }),
                    merged_by: None,
                    labels: None,
                },
                reviews: vec![],
                comments: vec![],
                commit_metadata_json: serde_json::json!({}),
                commit_files: vec![],
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

        fn with_commit_metadata_json(mut self, json: serde_json::Value) -> Self {
            self.commit_metadata_json = json;
            self
        }

        fn with_commit(mut self, commit: CommitDetails) -> Self {
            self.commit = commit;
            self
        }

        fn with_commit_files(mut self, filenames: Vec<&str>) -> Self {
            self.commit_files = filenames
                .into_iter()
                .map(|f| CommitFileChange {
                    filename: f.to_owned(),
                })
                .collect();
            self
        }

        fn zippy_version_bump() -> Self {
            Self {
                pull_request: PullRequestData {
                    number: 0,
                    user: None,
                    merged_by: None,
                    labels: None,
                },
                reviews: vec![],
                comments: vec![],
                commit_metadata_json: serde_json::json!({
                    "abc12345abc12345": {
                        "author": zippy_author(),
                        "authors": { "nodes": [] },
                        "signature": {
                            "isValid": true,
                            "signer": { "login": ZED_ZIPPY_LOGIN }
                        },
                        "additions": 2,
                        "deletions": 2
                    }
                }),
                commit_files: vec![
                    CommitFileChange {
                        filename: "Cargo.lock".to_owned(),
                    },
                    CommitFileChange {
                        filename: "crates/zed/Cargo.toml".to_owned(),
                    },
                ],
                org_members: vec![],
                commit: make_commit(
                    "abc12345abc12345",
                    "Zed Zippy",
                    ZED_ZIPPY_EMAIL,
                    "Bump to 0.230.2 for @cole-miller",
                    "",
                ),
            }
        }

        fn zippy_release_channel_update() -> Self {
            Self {
                pull_request: PullRequestData {
                    number: 0,
                    user: None,
                    merged_by: None,
                    labels: None,
                },
                reviews: vec![],
                comments: vec![],
                commit_metadata_json: serde_json::json!({
                    "abc12345abc12345": {
                        "author": zippy_author(),
                        "authors": { "nodes": [] },
                        "signature": {
                            "isValid": true,
                            "signer": { "login": ZED_ZIPPY_LOGIN }
                        },
                        "additions": 1,
                        "deletions": 1
                    }
                }),
                commit_files: vec![CommitFileChange {
                    filename: "crates/zed/RELEASE_CHANNEL".to_owned(),
                }],
                org_members: vec![],
                commit: make_commit(
                    "abc12345abc12345",
                    "Zed Zippy",
                    ZED_ZIPPY_EMAIL,
                    "v0.233.x stable for @cole-miller",
                    "",
                ),
            }
        }

        async fn run_scenario(self) -> Result<ReviewSuccess, ReviewFailure> {
            let mock = MockGithubApi {
                pull_request: self.pull_request,
                reviews: self.reviews,
                comments: self.comments,
                commit_metadata_json: self.commit_metadata_json,
                commit_files: self.commit_files,
                org_members: self.org_members,
            };
            let client = Rc::new(mock);
            let reporter = Reporter::new(CommitList::default(), client);
            reporter.check_commit(&self.commit).await
        }
    }

    #[tokio::test]
    async fn approved_review_by_org_member_succeeds() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![review(
                "bob",
                ReviewState::Approved,
                AuthorAssociation::Member,
            )])
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::PullRequestReviewed(_))));
    }

    #[tokio::test]
    async fn non_approved_review_state_is_not_accepted() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![review(
                "bob",
                ReviewState::Other,
                AuthorAssociation::Member,
            )])
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }

    #[tokio::test]
    async fn review_by_non_org_member_is_not_accepted() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![review(
                "bob",
                ReviewState::Approved,
                AuthorAssociation::None,
            )])
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }

    #[tokio::test]
    async fn pr_author_own_approval_review_is_rejected() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![review(
                "alice",
                ReviewState::Approved,
                AuthorAssociation::Member,
            )])
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }

    #[tokio::test]
    async fn pr_author_own_approval_comment_is_rejected() {
        let result = TestScenario::single_commit()
            .with_comments(vec![comment(
                "alice",
                "@zed-zippy approve",
                AuthorAssociation::Member,
            )])
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }

    #[tokio::test]
    async fn approval_comment_by_org_member_succeeds() {
        let result = TestScenario::single_commit()
            .with_comments(vec![comment(
                "bob",
                "@zed-zippy approve",
                AuthorAssociation::Member,
            )])
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::ApprovingComment(_))));
    }

    #[tokio::test]
    async fn group_approval_comment_by_org_member_succeeds() {
        let result = TestScenario::single_commit()
            .with_comments(vec![comment(
                "bob",
                "@zed-industries/approved",
                AuthorAssociation::Member,
            )])
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::ApprovingComment(_))));
    }

    #[tokio::test]
    async fn comment_without_approval_pattern_is_not_accepted() {
        let result = TestScenario::single_commit()
            .with_comments(vec![comment(
                "bob",
                "looks good",
                AuthorAssociation::Member,
            )])
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
            .with_reviews(vec![review(
                "bob",
                ReviewState::Approved,
                AuthorAssociation::Member,
            )])
            .with_comments(vec![comment(
                "charlie",
                "@zed-zippy approve",
                AuthorAssociation::Member,
            )])
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::PullRequestReviewed(_))));
    }

    #[tokio::test]
    async fn comment_takes_precedence_over_co_author() {
        let result = TestScenario::single_commit()
            .with_comments(vec![comment(
                "bob",
                "@zed-zippy approve",
                AuthorAssociation::Member,
            )])
            .with_commit_metadata_json(serde_json::json!({
                "abc12345abc12345": {
                    "author": alice_author(),
                    "authors": { "nodes": [charlie_author()] }
                }
            }))
            .with_commit(make_commit(
                "abc12345abc12345",
                "Alice",
                "alice@test.com",
                "Fix thing (#1234)",
                "Co-authored-by: Charlie <charlie@test.com>",
            ))
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::ApprovingComment(_))));
    }

    #[tokio::test]
    async fn co_author_org_member_succeeds() {
        let result = TestScenario::single_commit()
            .with_commit_metadata_json(serde_json::json!({
                "abc12345abc12345": {
                    "author": alice_author(),
                    "authors": { "nodes": [bob_author()] }
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
                review("bob", ReviewState::Other, AuthorAssociation::Member)
                    .with_body("@zed-zippy approve"),
            ])
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::PullRequestReviewed(_))));
    }

    #[tokio::test]
    async fn review_with_group_approval_body_is_accepted() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![
                review("bob", ReviewState::Other, AuthorAssociation::Member)
                    .with_body("@zed-industries/approved"),
            ])
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::PullRequestReviewed(_))));
    }

    #[tokio::test]
    async fn review_with_non_approving_body_is_not_accepted() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![
                review("bob", ReviewState::Other, AuthorAssociation::Member)
                    .with_body("looks good to me"),
            ])
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }

    #[tokio::test]
    async fn review_with_approving_body_from_external_user_is_not_accepted() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![
                review("bob", ReviewState::Other, AuthorAssociation::None)
                    .with_body("@zed-zippy approve"),
            ])
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }

    #[tokio::test]
    async fn review_with_approving_body_from_pr_author_is_rejected() {
        let result = TestScenario::single_commit()
            .with_reviews(vec![
                review("alice", ReviewState::Other, AuthorAssociation::Member)
                    .with_body("@zed-zippy approve"),
            ])
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::Unreviewed)));
    }

    #[tokio::test]
    async fn zippy_version_bump_with_valid_signature_succeeds() {
        let result = TestScenario::zippy_version_bump().run_scenario().await;
        assert!(matches!(
            result,
            Ok(ReviewSuccess::ZedZippyCommit(
                AutomatedChangeKind::VersionBump,
                _
            ))
        ));
        if let Ok(ReviewSuccess::ZedZippyCommit(_, login)) = &result {
            assert_eq!(login.as_str(), "cole-miller");
        }
    }

    #[tokio::test]
    async fn zippy_version_bump_without_mention_fails() {
        let result = TestScenario::zippy_version_bump()
            .with_commit(make_commit(
                "abc12345abc12345",
                "Zed Zippy",
                ZED_ZIPPY_EMAIL,
                "Bump to 0.230.2",
                "",
            ))
            .run_scenario()
            .await;
        assert!(matches!(
            result,
            Err(ReviewFailure::UnexpectedZippyAction(
                AutomatedChangeFailure::NoMentionInTitle
            ))
        ));
    }

    #[tokio::test]
    async fn zippy_version_bump_without_signature_fails() {
        let result = TestScenario::zippy_version_bump()
            .with_commit_metadata_json(serde_json::json!({
                "abc12345abc12345": {
                    "author": zippy_author(),
                    "authors": { "nodes": [] },
                    "additions": 2,
                    "deletions": 2
                }
            }))
            .run_scenario()
            .await;
        assert!(matches!(
            result,
            Err(ReviewFailure::UnexpectedZippyAction(
                AutomatedChangeFailure::NotSigned
            ))
        ));
    }

    #[tokio::test]
    async fn zippy_version_bump_with_invalid_signature_fails() {
        let result = TestScenario::zippy_version_bump()
            .with_commit_metadata_json(serde_json::json!({
                "abc12345abc12345": {
                    "author": zippy_author(),
                    "authors": { "nodes": [] },
                    "signature": {
                        "isValid": false,
                        "signer": { "login": ZED_ZIPPY_LOGIN }
                    },
                    "additions": 2,
                    "deletions": 2
                }
            }))
            .run_scenario()
            .await;
        assert!(matches!(
            result,
            Err(ReviewFailure::UnexpectedZippyAction(
                AutomatedChangeFailure::InvalidSignature
            ))
        ));
    }

    #[tokio::test]
    async fn zippy_version_bump_with_unequal_line_changes_fails() {
        let result = TestScenario::zippy_version_bump()
            .with_commit_metadata_json(serde_json::json!({
                "abc12345abc12345": {
                    "author": zippy_author(),
                    "authors": { "nodes": [] },
                    "signature": {
                        "isValid": true,
                        "signer": { "login": ZED_ZIPPY_LOGIN }
                    },
                    "additions": 5,
                    "deletions": 2
                }
            }))
            .run_scenario()
            .await;
        assert!(matches!(
            result,
            Err(ReviewFailure::UnexpectedZippyAction(
                AutomatedChangeFailure::UnexpectedLineChanges { .. }
            ))
        ));
    }

    #[tokio::test]
    async fn zippy_version_bump_with_wrong_github_author_fails() {
        let result = TestScenario::zippy_version_bump()
            .with_commit_metadata_json(serde_json::json!({
                "abc12345abc12345": {
                    "author": alice_author(),
                    "authors": { "nodes": [] },
                    "signature": {
                        "isValid": true,
                        "signer": { "login": "alice" }
                    },
                    "additions": 2,
                    "deletions": 2
                }
            }))
            .run_scenario()
            .await;
        assert!(matches!(
            result,
            Err(ReviewFailure::UnexpectedZippyAction(
                AutomatedChangeFailure::AuthorMismatch
            ))
        ));
    }

    #[tokio::test]
    async fn zippy_version_bump_with_co_authors_fails() {
        let result = TestScenario::zippy_version_bump()
            .with_commit_metadata_json(serde_json::json!({
                "abc12345abc12345": {
                    "author": zippy_author(),
                    "authors": { "nodes": [alice_author()] },
                    "signature": {
                        "isValid": true,
                        "signer": { "login": ZED_ZIPPY_LOGIN }
                    },
                    "additions": 2,
                    "deletions": 2
                }
            }))
            .run_scenario()
            .await;
        assert!(matches!(
            result,
            Err(ReviewFailure::UnexpectedZippyAction(
                AutomatedChangeFailure::UnexpectedCoAuthors
            ))
        ));
    }

    #[tokio::test]
    async fn zippy_version_bump_with_wrong_files_fails() {
        let result = TestScenario::zippy_version_bump()
            .with_commit_files(vec!["crates/zed/RELEASE_CHANNEL"])
            .run_scenario()
            .await;
        assert!(matches!(
            result,
            Err(ReviewFailure::UnexpectedZippyAction(
                AutomatedChangeFailure::UnexpectedFiles { .. }
            ))
        ));
    }

    #[tokio::test]
    async fn zippy_release_channel_update_succeeds() {
        let result = TestScenario::zippy_release_channel_update()
            .run_scenario()
            .await;
        assert!(matches!(
            result,
            Ok(ReviewSuccess::ZedZippyCommit(
                AutomatedChangeKind::ReleaseChannelUpdate,
                _
            ))
        ));
        if let Ok(ReviewSuccess::ZedZippyCommit(_, login)) = &result {
            assert_eq!(login.as_str(), "cole-miller");
        }
    }

    #[tokio::test]
    async fn non_zippy_commit_without_pr_is_no_pr_found() {
        let result = TestScenario::single_commit()
            .with_commit(make_commit(
                "abc12345abc12345",
                "Alice",
                "alice@test.com",
                "Some direct push",
                "",
            ))
            .run_scenario()
            .await;
        assert!(matches!(result, Err(ReviewFailure::NoPullRequestFound)));
    }

    #[tokio::test]
    async fn zippy_commit_with_pr_number_goes_through_normal_flow() {
        let result = TestScenario::single_commit()
            .with_commit(make_commit(
                "abc12345abc12345",
                "Zed Zippy",
                ZED_ZIPPY_EMAIL,
                "Some change (#1234)",
                "",
            ))
            .with_reviews(vec![review(
                "bob",
                ReviewState::Approved,
                AuthorAssociation::Member,
            )])
            .run_scenario()
            .await;
        assert!(matches!(result, Ok(ReviewSuccess::PullRequestReviewed(_))));
    }
}
