use gh_workflow::{
    Concurrency, Event, Expression, Job, Run, Schedule, Step, Use, Workflow, WorkflowDispatch,
};

use crate::tasks::workflows::{
    runners::{self, Platform},
    steps::{self, FluentBuilder as _, NamedJob, named},
};

pub(crate) fn run_unit_evals() -> Workflow {
    let unit_evals = unit_evals();

    named::workflow().on(Event::default()
        .schedule([
            // GitHub might drop jobs at busy times, so we choose a random time in the middle of the night.
            Schedule::default().cron("47 1 * * 2"),
        ])
        .workflow_dispatch(WorkflowDispatch::default()))
    .concurrency(Concurrency::default().group(
        "${{ github.workflow }}-${{ github.ref_name }}-${{ github.ref_name == 'main' && github.sha || 'anysha' }}"
    ).cancel_in_progress(true))
    .add_env(("CARGO_TERM_COLOR", "always"))
    .add_env(("CARGO_INCREMENTAL", 0))
    .add_env(("RUST_BACKTRACE", 1))
    .add_env(("ZED_CLIENT_CHECKSUM_SEED", "${{ secrets.ZED_CLIENT_CHECKSUM_SEED }}"))
    .add_job(unit_evals.name, unit_evals.job)
}

fn unit_evals() -> NamedJob {
    fn run_evals() -> Step<Run> {
        named::bash("./script/run-unit-evals")
    }

    fn send_failure_to_slack() -> Step<Use> {
        named::uses(
            "slackapi",
            "slack-github-action",
            "b0fa283ad8fea605de13dc3f449259339835fc52",
        )
        .if_condition(Expression::new("${{ failure() }}"))
        .add_with(("method", "chat.postMessage"))
        .add_with(("token", "${{ secrets.SLACK_APP_ZED_UNIT_EVALS_BOT_TOKEN }}"))
        .add_with(("payload", indoc::indoc!{r#"
            channel: C04UDRNNJFQ
            text: "Unit Evals Failed: https://github.com/zed-industries/zed/actions/runs/${{ github.run_id }}"
        "#}))
    }

    named::job(
        Job::default()
            .runs_on(runners::LINUX_DEFAULT)
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_cargo_config(Platform::Linux))
            .add_step(steps::cache_rust_dependencies())
            .map(steps::install_linux_dependencies)
            .add_step(steps::cargo_install_nextest(Platform::Linux))
            .add_step(steps::clear_target_dir_if_large(Platform::Linux))
            .add_step(run_evals())
            .add_step(send_failure_to_slack())
            .add_step(steps::cleanup_cargo_config(Platform::Linux)),
    )
}
