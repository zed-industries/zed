use gh_workflow::*;

use crate::tasks::workflows::{
    run_tests::run_tests_in,
    runners,
    steps::{self, FluentBuilder, NamedJob, named, release_job},
};

pub(crate) fn run_action_checks() -> Workflow {
    let action_checks = actionlint();
    let shell_checks = shellcheck();

    named::workflow()
        .map(|workflow| {
            run_tests_in(
                &[
                    ".github/workflows/**",
                    ".github/actions/**",
                    ".github/actionlint.yml",
                    "script/**",
                ],
                workflow,
            )
        })
        .add_job(action_checks.name, action_checks.job)
        .add_job(shell_checks.name, shell_checks.job)
}
const ACTION_LINT_STEP_ID: &'static str = "get_actionlint";

fn actionlint() -> NamedJob {
    named::job(
        release_job(&[])
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo())
            .add_step(download_actionlint())
            .add_step(run_actionlint()),
    )
}

fn shellcheck() -> NamedJob {
    named::job(
        release_job(&[])
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo())
            .add_step(run_shellcheck()),
    )
}

fn download_actionlint() -> Step<Run> {
    named::bash("bash <(curl https://raw.githubusercontent.com/rhysd/actionlint/main/scripts/download-actionlint.bash)").id(ACTION_LINT_STEP_ID)
}

fn run_actionlint() -> Step<Run> {
    named::bash(indoc::indoc! {r#"
            ${{ steps.get_actionlint.outputs.executable }} -color
        "#})
}

fn run_shellcheck() -> Step<Run> {
    named::bash("./script/shellcheck-scripts error")
}
