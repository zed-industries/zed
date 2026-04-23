use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::Context as _;
use derive_more::Display;
use itertools::{Either, Itertools};

use crate::{
    checks::{ReviewFailure, ReviewResult, ReviewSuccess},
    git::CommitDetails,
};

const PULL_REQUEST_BASE_URL: &str = "https://github.com/zed-industries/zed/pull";

#[derive(Debug)]
pub struct ReportEntry<R> {
    pub commit: CommitDetails,
    reason: R,
}

impl<R: ToString> ReportEntry<R> {
    fn commit_cell(&self) -> String {
        let title = escape_markdown_link_text(self.commit.title());

        match self.commit.pr_number() {
            Some(pr_number) => format!("[{title}]({PULL_REQUEST_BASE_URL}/{pr_number})"),
            None => escape_markdown_table_text(self.commit.title()),
        }
    }

    fn pull_request_cell(&self) -> String {
        self.commit
            .pr_number()
            .map(|pr_number| format!("#{pr_number}"))
            .unwrap_or_else(|| "—".to_owned())
    }

    fn author_cell(&self) -> String {
        escape_markdown_table_text(&self.commit.author().to_string())
    }

    fn reason_cell(&self) -> String {
        escape_markdown_table_text(&self.reason.to_string())
    }
}

impl ReportEntry<ReviewResult> {
    pub fn is_unknown_error(&self) -> bool {
        matches!(self.reason, Err(ReviewFailure::Other(_)))
    }
}

impl ReportEntry<ReviewFailure> {
    fn issue_kind(&self) -> IssueKind {
        match self.reason {
            ReviewFailure::Other(_) => IssueKind::Error,
            _ => IssueKind::NotReviewed,
        }
    }
}

impl ReportEntry<ReviewSuccess> {
    fn reviewers_cell(&self) -> String {
        match &self.reason.reviewers() {
            Ok(reviewers) => escape_markdown_table_text(&reviewers),
            Err(_) => "—".to_owned(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ReportSummary {
    pub pull_requests: usize,
    pub reviewed_prs: usize,
    pub other_checked: usize,
    pub not_reviewed: usize,
    pub errors: usize,
}

pub enum ReportReviewSummary {
    MissingReviews,
    MissingReviewsWithErrors,
    NoIssuesFound,
}

impl ReportSummary {
    fn from_entries(entries: &[ReportEntry<ReviewResult>]) -> Self {
        Self {
            pull_requests: entries
                .iter()
                .filter_map(|entry| entry.commit.pr_number())
                .unique()
                .count(),
            reviewed_prs: entries
                .iter()
                .filter(|entry| entry.reason.is_ok() && entry.commit.pr_number().is_some())
                .count(),
            other_checked: entries
                .iter()
                .filter(|entry| entry.reason.is_ok() && entry.commit.pr_number().is_none())
                .count(),
            not_reviewed: entries
                .iter()
                .filter(|entry| {
                    matches!(
                        entry.reason,
                        Err(ReviewFailure::NoPullRequestFound
                            | ReviewFailure::Unreviewed
                            | ReviewFailure::UnexpectedZippyAction(_))
                    )
                })
                .count(),
            errors: entries
                .iter()
                .filter(|entry| entry.is_unknown_error())
                .count(),
        }
    }

    pub fn review_summary(&self) -> ReportReviewSummary {
        match self.not_reviewed {
            0 if self.errors == 0 => ReportReviewSummary::NoIssuesFound,
            1.. if self.errors == 0 => ReportReviewSummary::MissingReviews,
            _ => ReportReviewSummary::MissingReviewsWithErrors,
        }
    }

    fn has_errors(&self) -> bool {
        self.errors > 0
    }

    pub fn prs_with_errors(&self) -> usize {
        self.pull_requests.saturating_sub(self.reviewed_prs)
    }
}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
enum IssueKind {
    #[display("Error")]
    Error,
    #[display("Not reviewed")]
    NotReviewed,
}

#[derive(Debug, Default)]
pub struct Report {
    entries: Vec<ReportEntry<ReviewResult>>,
}

impl Report {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, commit: CommitDetails, result: ReviewResult) {
        self.entries.push(ReportEntry {
            commit,
            reason: result,
        });
    }

    pub fn errors(&self) -> impl Iterator<Item = &ReportEntry<ReviewResult>> {
        self.entries.iter().filter(|entry| entry.reason.is_err())
    }

    pub fn summary(&self) -> ReportSummary {
        ReportSummary::from_entries(&self.entries)
    }

    pub fn write_markdown(self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();

        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "Failed to create parent directory for markdown report at {}",
                    path.display()
                )
            })?;
        }

        let summary = self.summary();
        let (successes, mut issues): (Vec<_>, Vec<_>) =
            self.entries
                .into_iter()
                .partition_map(|entry| match entry.reason {
                    Ok(success) => Either::Left(ReportEntry {
                        reason: success,
                        commit: entry.commit,
                    }),
                    Err(fail) => Either::Right(ReportEntry {
                        reason: fail,
                        commit: entry.commit,
                    }),
                });

        issues.sort_by_key(|entry| entry.issue_kind());

        let file = File::create(path)
            .with_context(|| format!("Failed to create markdown report at {}", path.display()))?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "# Compliance report")?;
        writeln!(writer)?;
        writeln!(writer, "## Overview")?;
        writeln!(writer)?;
        writeln!(writer, "- PRs: {}", summary.pull_requests)?;
        writeln!(writer, "- Reviewed: {}", summary.reviewed_prs)?;
        writeln!(writer, "- Not reviewed: {}", summary.not_reviewed)?;
        writeln!(
            writer,
            "- Differently validated commits: {}",
            summary.other_checked
        )?;
        if summary.has_errors() {
            writeln!(writer, "- Errors: {}", summary.errors)?;
        }
        writeln!(writer)?;

        write_issue_table(&mut writer, &issues, &summary)?;
        write_success_table(&mut writer, &successes)?;

        writer
            .flush()
            .with_context(|| format!("Failed to flush markdown report to {}", path.display()))
    }
}

fn write_issue_table(
    writer: &mut impl Write,
    issues: &[ReportEntry<ReviewFailure>],
    summary: &ReportSummary,
) -> std::io::Result<()> {
    if summary.has_errors() {
        writeln!(writer, "## Errors and unreviewed commits")?;
    } else {
        writeln!(writer, "## Unreviewed commits")?;
    }
    writeln!(writer)?;

    if issues.is_empty() {
        if summary.has_errors() {
            writeln!(writer, "No errors or unreviewed commits found.")?;
        } else {
            writeln!(writer, "No unreviewed commits found.")?;
        }
        writeln!(writer)?;
        return Ok(());
    }

    writeln!(writer, "| Commit | PR | Author | Outcome | Reason |")?;
    writeln!(writer, "| --- | --- | --- | --- | --- |")?;

    for entry in issues {
        let issue_kind = entry.issue_kind();
        writeln!(
            writer,
            "| {} | {} | {} | {} | {} |",
            entry.commit_cell(),
            entry.pull_request_cell(),
            entry.author_cell(),
            issue_kind,
            entry.reason_cell(),
        )?;
    }

    writeln!(writer)?;
    Ok(())
}

fn write_success_table(
    writer: &mut impl Write,
    successful_entries: &[ReportEntry<ReviewSuccess>],
) -> std::io::Result<()> {
    writeln!(writer, "## Successful commits")?;
    writeln!(writer)?;

    if successful_entries.is_empty() {
        writeln!(writer, "No successful commits found.")?;
        writeln!(writer)?;
        return Ok(());
    }

    writeln!(writer, "| Commit | PR | Author | Reviewers | Reason |")?;
    writeln!(writer, "| --- | --- | --- | --- | --- |")?;

    for entry in successful_entries {
        writeln!(
            writer,
            "| {} | {} | {} | {} | {} |",
            entry.commit_cell(),
            entry.pull_request_cell(),
            entry.author_cell(),
            entry.reviewers_cell(),
            entry.reason_cell(),
        )?;
    }

    writeln!(writer)?;
    Ok(())
}

fn escape_markdown_link_text(input: &str) -> String {
    escape_markdown_table_text(input)
        .replace('[', r"\[")
        .replace(']', r"\]")
}

fn escape_markdown_table_text(input: &str) -> String {
    input
        .replace('\\', r"\\")
        .replace('|', r"\|")
        .replace('\r', "")
        .replace('\n', "<br>")
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        checks::{ReviewFailure, ReviewSuccess},
        git::{CommitDetails, CommitList},
        github::{GithubLogin, GithubUser, PullRequestReview, ReviewState},
    };

    use super::{Report, ReportReviewSummary};

    fn make_commit(
        sha: &str,
        author_name: &str,
        author_email: &str,
        title: &str,
        body: &str,
    ) -> CommitDetails {
        let formatted = format!(
            "{sha}|field-delimiter|{author_name}|field-delimiter|{author_email}|field-delimiter|{title}|body-delimiter|{body}|commit-delimiter|"
        );
        CommitList::from_str(&formatted)
            .expect("test commit should parse")
            .into_iter()
            .next()
            .expect("should have one commit")
    }

    fn reviewed() -> ReviewSuccess {
        ReviewSuccess::PullRequestReviewed(vec![PullRequestReview {
            user: Some(GithubUser {
                login: "reviewer".to_owned(),
            }),
            state: Some(ReviewState::Approved),
            body: None,
        }])
    }

    #[test]
    fn report_summary_counts_are_accurate() {
        let mut report = Report::new();

        report.add(
            make_commit(
                "aaa",
                "Alice",
                "alice@test.com",
                "Reviewed commit (#100)",
                "",
            ),
            Ok(reviewed()),
        );
        report.add(
            make_commit("bbb", "Bob", "bob@test.com", "Unreviewed commit (#200)", ""),
            Err(ReviewFailure::Unreviewed),
        );
        report.add(
            make_commit("ccc", "Carol", "carol@test.com", "No PR commit", ""),
            Err(ReviewFailure::NoPullRequestFound),
        );
        report.add(
            make_commit("ddd", "Dave", "dave@test.com", "Error commit (#300)", ""),
            Err(ReviewFailure::Other(anyhow::anyhow!("some error"))),
        );
        report.add(
            make_commit("ddd", "Dave", "dave@test.com", "Bump Version", ""),
            Ok(ReviewSuccess::ZedZippyCommit(GithubLogin::new(
                "dave".to_string(),
            ))),
        );

        let summary = report.summary();
        assert_eq!(summary.pull_requests, 3);
        assert_eq!(summary.reviewed_prs, 1);
        assert_eq!(summary.other_checked, 1);
        assert_eq!(summary.not_reviewed, 2);
        assert_eq!(summary.errors, 1);
    }

    #[test]
    fn report_summary_all_reviewed_is_no_issues() {
        let mut report = Report::new();

        report.add(
            make_commit("aaa", "Alice", "alice@test.com", "First (#100)", ""),
            Ok(reviewed()),
        );
        report.add(
            make_commit("bbb", "Bob", "bob@test.com", "Second (#200)", ""),
            Ok(reviewed()),
        );

        let summary = report.summary();
        assert!(matches!(
            summary.review_summary(),
            ReportReviewSummary::NoIssuesFound
        ));
    }

    #[test]
    fn report_summary_missing_reviews_only() {
        let mut report = Report::new();

        report.add(
            make_commit("aaa", "Alice", "alice@test.com", "Reviewed (#100)", ""),
            Ok(reviewed()),
        );
        report.add(
            make_commit("bbb", "Bob", "bob@test.com", "Unreviewed (#200)", ""),
            Err(ReviewFailure::Unreviewed),
        );

        let summary = report.summary();
        assert!(matches!(
            summary.review_summary(),
            ReportReviewSummary::MissingReviews
        ));
    }

    #[test]
    fn report_summary_errors_and_missing_reviews() {
        let mut report = Report::new();

        report.add(
            make_commit("aaa", "Alice", "alice@test.com", "Unreviewed (#100)", ""),
            Err(ReviewFailure::Unreviewed),
        );
        report.add(
            make_commit("bbb", "Bob", "bob@test.com", "Errored (#200)", ""),
            Err(ReviewFailure::Other(anyhow::anyhow!("check failed"))),
        );

        let summary = report.summary();
        assert!(matches!(
            summary.review_summary(),
            ReportReviewSummary::MissingReviewsWithErrors
        ));
    }

    #[test]
    fn report_summary_deduplicates_pull_requests() {
        let mut report = Report::new();

        report.add(
            make_commit("aaa", "Alice", "alice@test.com", "First change (#100)", ""),
            Ok(reviewed()),
        );
        report.add(
            make_commit("bbb", "Bob", "bob@test.com", "Second change (#100)", ""),
            Ok(reviewed()),
        );

        let summary = report.summary();
        assert_eq!(summary.pull_requests, 1);
    }
}
