use gh_workflow::{Event, Job, Run, Schedule, Step, Workflow, WorkflowDispatch};
use indoc::formatdoc;

use crate::tasks::workflows::{
    release::{COMPLIANCE_REPORT_PATH, ComplianceContext, add_compliance_notification_steps},
    runners,
    steps::{self, CommonJobConditions, named},
    vars::{self, StepOutput},
};

pub fn compliance_check() -> Workflow {
    let check = scheduled_compliance_check();

    named::workflow()
        .on(Event::default()
            .schedule([Schedule::new("30 17 * * 2")])
            .workflow_dispatch(WorkflowDispatch::default()))
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
            formatdoc! {r#"
                echo "tag=$LATEST_TAG" >> "$GITHUB_OUTPUT"
                cargo xtask compliance "$LATEST_TAG" --branch main --report-path {COMPLIANCE_REPORT_PATH}
                "#,
            }
        )
        .id("run-compliance-check")
        .add_env(("LATEST_TAG", tag.to_string()))
        .add_env(("GITHUB_APP_ID", vars::ZED_ZIPPY_APP_ID))
        .add_env(("GITHUB_APP_KEY", vars::ZED_ZIPPY_APP_PRIVATE_KEY))
    }

    let job = Job::default()
        .with_repository_owner_guard()
        .runs_on(runners::LINUX_SMALL)
        .add_step(steps::checkout_repo().with_full_history())
        .add_step(steps::cache_rust_dependencies_namespace())
        .add_step(determine_version_step)
        .add_step(run_compliance_check(&tag_output));

    named::job(add_compliance_notification_steps(
        job,
        ComplianceContext::Scheduled {
            tag_source: tag_output,
        },
        "run-compliance-check",
    ))
}
