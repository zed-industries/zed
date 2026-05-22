use std::{path::PathBuf, rc::Rc};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use compliance::{
    checks::Reporter,
    git::{CommitsFromVersionToVersion, GetVersionTags, GitCommand, InfoForCommit, VersionTag},
    github::{GithubApiClient as _, OctocrabClient, Repository},
    report::ReportReviewSummary,
};

const MAX_CONCURRENT_REQUESTS: usize = 5;

#[derive(Parser)]
pub(crate) struct ComplianceArgs {
    #[clap(subcommand)]
    mode: ComplianceMode,
}

const IGNORE_LIST: &[&str] = &[
    "75fa566511e3ae7d03cfd76008512080291bd81d", // GitHub nuked this PR out of orbit
];

#[derive(Subcommand)]
pub(crate) enum ComplianceMode {
    // Check compliance for all commits between two version tags
    Version(VersionArgs),
    // Check compliance for a single commit
    Single {
        // The full commit SHA to check
        commit_sha: String,
    },
}

#[derive(Parser)]
pub(crate) struct VersionArgs {
    #[arg(value_parser = VersionTag::parse)]
    // The version to be on the lookout for
    version_tag: VersionTag,
    #[arg(long)]
    // The markdown file to write the compliance report to
    report_path: PathBuf,
    #[arg(long)]
    // An optional branch to use instead of the determined version branch
    branch: Option<String>,
}

impl VersionArgs {
    pub(crate) fn version_tag(&self) -> &VersionTag {
        &self.version_tag
    }

    fn version_head(&self) -> String {
        self.branch
            .clone()
            .unwrap_or_else(|| self.version_tag().to_string())
    }
}

async fn check_compliance_impl(args: ComplianceArgs) -> Result<()> {
    let app_id = std::env::var("GITHUB_APP_ID").context("Missing GITHUB_APP_ID")?;
    let key = std::env::var("GITHUB_APP_KEY").context("Missing GITHUB_APP_KEY")?;

    let client = Rc::new(
        OctocrabClient::new(
            app_id.parse().context("Failed to parse app ID as int")?,
            key.as_ref(),
            Repository::ZED.owner(),
        )
        .await?,
    );

    println!("Initialized GitHub client for app ID {app_id}");

    let args = match args.mode {
        ComplianceMode::Version(version) => version,
        ComplianceMode::Single { commit_sha } => {
            let commit = GitCommand::run(InfoForCommit::new(&commit_sha))?;

            return match Reporter::result_for_commit(commit, client).await {
                Ok(review_success) => {
                    println!("Check for commit {commit_sha} succeeded. Result: {review_success}",);
                    Ok(())
                }

                Err(review_failure) => Err(anyhow::anyhow!(
                    "Check for commit {commit_sha} failed. Result: {review_failure}"
                )),
            };
        }
    };

    let tag = args.version_tag();

    let previous_version = GitCommand::run(GetVersionTags)?
        .sorted()
        .find_previous_minor_version(&tag)
        .cloned()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Could not find previous version for tag {tag}",
                tag = tag.to_string()
            )
        })?;

    println!(
        "Checking compliance for version {} with version {} as base",
        tag.version(),
        previous_version.version()
    );

    let commits = GitCommand::run(CommitsFromVersionToVersion::new(
        previous_version,
        args.version_head(),
    ))?;

    let Some(range) = commits.range() else {
        anyhow::bail!("No commits found to check");
    };

    println!("Checking commit range {range}, {} total", commits.len());

    let report = Reporter::new(commits, client.clone())
        .generate_report(MAX_CONCURRENT_REQUESTS)
        .await;

    println!(
        "Generated report for version {}",
        args.version_tag().to_string()
    );

    let summary = report.summary();

    println!(
        "Applying compliance labels to {} pull requests",
        summary.prs_with_errors()
    );

    let all_errors_known = report.errors().all(|error| {
        error.is_unknown_error() && IGNORE_LIST.contains(&error.commit.sha().as_str())
    });

    for report in report.errors() {
        if let Some(pr_number) = report.commit.pr_number()
            && let Ok(pull_request) = client.get_pull_request(&Repository::ZED, pr_number).await
            && pull_request.labels.is_none_or(|labels| {
                labels
                    .iter()
                    .all(|label| label != compliance::github::PR_REVIEW_LABEL)
            })
        {
            println!("Adding review label to PR {}...", pr_number);

            client
                .add_label_to_issue(
                    &Repository::ZED,
                    compliance::github::PR_REVIEW_LABEL,
                    pr_number,
                )
                .await?;
        }
    }

    report.write_markdown(&args.report_path)?;

    println!("Wrote compliance report to {}", args.report_path.display());

    match summary.review_summary() {
        ReportReviewSummary::MissingReviews => Err(anyhow::anyhow!(
            "Compliance check failed, found {} commits not reviewed",
            summary.not_reviewed
        )),
        ReportReviewSummary::MissingReviewsWithErrors if all_errors_known => {
            println!(
                "Compliance check failed with {} unreviewed commits, but all errors are known.",
                summary.not_reviewed
            );

            Ok(())
        }
        ReportReviewSummary::MissingReviewsWithErrors => Err(anyhow::anyhow!(
            "Compliance check failed with {} unreviewed commits and {} other issues",
            summary.not_reviewed,
            summary.errors
        )),
        ReportReviewSummary::NoIssuesFound => {
            println!("No issues found, compliance check passed.");
            Ok(())
        }
    }
}

pub fn check_compliance(args: ComplianceArgs) -> Result<()> {
    tokio::runtime::Runtime::new()
        .context("Failed to create tokio runtime")
        .and_then(|handle| handle.block_on(check_compliance_impl(args)))
}
