use gh_workflow::{Event, Expression, Job, Run, Schedule, Step, Workflow};

use crate::tasks::workflows::{
    runners,
    steps::{self, CommonJobConditions, named},
    vars::{self, StepOutput},
};

pub fn compliance_check() -> Workflow {
    let check = scheduled_compliance_check();

    named::workflow()
        .on(Event::default().schedule([Schedule::new("30 17 * * 2")]))
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_job(check.name, check.job)
}

fn scheduled_compliance_check() -> steps::NamedJob {
    let determine_version_step = named::bash(indoc::indoc! {r#"
        VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' crates/zed/Cargo.toml | tr -d '[:space:]')
        if [ -z "$VERSION" ]; then
            echo "Could not determine version from crates/zed/Cargo.toml"
            exit 1
        fi
        TAG="v${VERSION}-pre"
        echo "Checking compliance for $TAG"
        echo "tag=$TAG" >> "$GITHUB_OUTPUT"
    "#})
    .id("determine-version");

    let tag_output = StepOutput::new(&determine_version_step, "tag");

    fn run_compliance_check(tag: &StepOutput) -> Step<Run> {
        named::bash(
            r#"cargo xtask compliance "$LATEST_TAG" --branch main --report-path target/compliance-report"#,
        )
        .id("run-compliance-check")
        .add_env(("LATEST_TAG", tag.to_string()))
        .add_env(("GITHUB_APP_ID", vars::ZED_ZIPPY_APP_ID))
        .add_env(("GITHUB_APP_KEY", vars::ZED_ZIPPY_APP_PRIVATE_KEY))
    }

    fn send_failure_slack_notification(tag: &StepOutput) -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            MESSAGE="⚠️ Scheduled compliance check failed for upcoming preview release $LATEST_TAG: There are PRs with missing reviews."

            curl -X POST -H 'Content-type: application/json' \
                --data "$(jq -n --arg text "$MESSAGE" '{"text": $text}')" \
                "$SLACK_WEBHOOK"
        "#})
        .if_condition(Expression::new("failure()"))
        .add_env(("SLACK_WEBHOOK", vars::SLACK_WEBHOOK_WORKFLOW_FAILURES))
        .add_env(("LATEST_TAG", tag.to_string()))
    }

    named::job(
        Job::default()
            .with_repository_owner_guard()
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo().with_full_history())
            .add_step(steps::cache_rust_dependencies_namespace())
            .add_step(determine_version_step)
            .add_step(run_compliance_check(&tag_output))
            .add_step(send_failure_slack_notification(&tag_output)),
    )
}
