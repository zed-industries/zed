use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

use compliance::{
    checks::Reporter,
    git::{CommitsFromVersionToVersion, GetVersionTags, GitCommand, VersionTag},
    github::GitHubClient,
    report::ReportReviewSummary,
};

#[derive(Parser)]
pub struct ComplianceArgs {
    #[arg(value_parser = VersionTag::parse)]
    // The version to be on the lookout for
    pub(crate) version_tag: VersionTag,
    #[arg(long)]
    // The markdown file to write the compliance report to
    report_path: PathBuf,
    #[arg(long)]
    // An optional branch to use instead of the determined version branch
    branch: Option<String>,
}

impl ComplianceArgs {
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

    let client = GitHubClient::for_app(
        app_id.parse().context("Failed to parse app ID as int")?,
        key.as_ref(),
    )
    .await?;

    println!("Initialized GitHub client for app ID {app_id}");

    let report = Reporter::new(commits, &client).generate_report().await?;

    println!(
        "Generated report for version {}",
        args.version_tag().to_string()
    );

    let summary = report.summary();

    println!(
        "Applying compliance labels to {} pull requests",
        summary.prs_with_errors()
    );

    for report in report.errors() {
        if let Some(pr_number) = report.commit.pr_number()
            && let Ok(pull_request) = client.get_pull_request(pr_number).await
            && pull_request.labels.is_none_or(|labels| {
                labels
                    .iter()
                    .all(|label| label != compliance::github::PR_REVIEW_LABEL)
            })
        {
            println!("Adding review label to PR {}...", pr_number);

            client
                .add_label_to_issue(compliance::github::PR_REVIEW_LABEL, pr_number)
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
