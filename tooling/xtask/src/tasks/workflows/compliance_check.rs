use gh_workflow::{Event, Job, Schedule, Workflow, WorkflowDispatch};

use crate::tasks::workflows::{
    release::{ComplianceContext, add_compliance_steps},
    runners,
    steps::{self, CommonJobConditions, named},
    vars::StepOutput,
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

    let job = Job::default()
        .with_repository_owner_guard()
        .runs_on(runners::LINUX_SMALL)
        .add_step(steps::checkout_repo().with_full_history())
        .add_step(steps::cache_rust_dependencies_namespace())
        .add_step(determine_version_step);

    named::job(
        add_compliance_steps(
            job,
            ComplianceContext::Scheduled {
                tag_source: tag_output,
            },
        )
        .0,
    )
}
