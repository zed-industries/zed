use gh_workflow::*;
use indoc::indoc;

use crate::tasks::workflows::{
    run_tests::{orchestrate, tests_pass},
    runners,
    steps::{self, CommonJobConditions, FluentBuilder, NamedJob, named},
    vars::{PathCondition, StepOutput, one_workflow_per_non_main_branch},
};

pub(crate) const ZED_EXTENSION_CLI_SHA: &str = "7cfce605704d41ca247e3f84804bf323f6c6caaf";

// This is used by various extensions repos in the zed-extensions org to run automated tests.
pub(crate) fn extension_tests() -> Workflow {
    let should_check_rust = PathCondition::new("check_rust", r"^(Cargo.lock|Cargo.toml|.*\.rs)$");
    let should_check_extension = PathCondition::new("check_extension", r"^.*\.scm$");

    let orchestrate = orchestrate(&[&should_check_rust, &should_check_extension]);

    let jobs = [
        orchestrate,
        should_check_rust.guard(check_rust()),
        should_check_extension.guard(check_extension()),
    ];

    let tests_pass = tests_pass(&jobs);

    named::workflow()
        .add_event(Event::default().workflow_call(WorkflowCall::default()))
        .concurrency(one_workflow_per_non_main_branch())
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("RUST_BACKTRACE", 1))
        .add_env(("CARGO_INCREMENTAL", 0))
        .add_env(("ZED_EXTENSION_CLI_SHA", ZED_EXTENSION_CLI_SHA))
        .map(|workflow| {
            jobs.into_iter()
                .chain([tests_pass])
                .fold(workflow, |workflow, job| {
                    workflow.add_job(job.name, job.job)
                })
        })
}

fn run_clippy() -> Step<Run> {
    named::bash("cargo clippy --release --all-targets --all-features -- --deny warnings")
}

fn check_rust() -> NamedJob {
    let job = Job::default()
        .with_repository_owner_guard()
        .runs_on(runners::LINUX_MEDIUM)
        .timeout_minutes(3u32)
        .add_step(steps::checkout_repo())
        .add_step(steps::cache_rust_dependencies_namespace())
        .add_step(steps::cargo_fmt())
        .add_step(run_clippy())
        .add_step(steps::cargo_install_nextest())
        .add_step(
            steps::cargo_nextest(runners::Platform::Linux).add_env(("NEXTEST_NO_TESTS", "warn")),
        );

    named::job(job)
}

pub(crate) fn check_extension() -> NamedJob {
    let (cache_download, cache_hit) = cache_zed_extension_cli();
    let job = Job::default()
        .with_repository_owner_guard()
        .runs_on(runners::LINUX_LARGE_RAM)
        .timeout_minutes(2u32)
        .add_step(steps::checkout_repo())
        .add_step(cache_download)
        .add_step(download_zed_extension_cli(cache_hit))
        .add_step(check());

    named::job(job)
}

pub fn cache_zed_extension_cli() -> (Step<Use>, StepOutput) {
    let step = named::uses(
        "actions",
        "cache",
        "0057852bfaa89a56745cba8c7296529d2fc39830",
    )
    .id("cache-zed-extension-cli")
    .with(
        Input::default()
            .add("path", "zed-extension")
            .add("key", "zed-extension-${{ env.ZED_EXTENSION_CLI_SHA }}"),
    );
    let output = StepOutput::new(&step, "cache-hit");
    (step, output)
}

pub fn download_zed_extension_cli(cache_hit: StepOutput) -> Step<Run> {
    named::bash(
    indoc! {
        r#"
        wget --quiet "https://zed-extension-cli.nyc3.digitaloceanspaces.com/$ZED_EXTENSION_CLI_SHA/x86_64-unknown-linux-gnu/zed-extension"
        chmod +x zed-extension
        "#,
    }
    ).if_condition(Expression::new(format!("{} != 'true'", cache_hit.expr())))
}

pub fn check() -> Step<Run> {
    named::bash(indoc! {
        r#"
        mkdir -p /tmp/ext-scratch
        mkdir -p /tmp/ext-output
        ./zed-extension --source-dir . --scratch-dir /tmp/ext-scratch --output-dir /tmp/ext-output
        "#
    })
}
