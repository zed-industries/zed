use gh_workflow::{
    Event, Expression, Job, PullRequest, PullRequestType, Run, Schedule, Step, Use, Workflow,
    WorkflowDispatch,
};

use crate::tasks::workflows::{
    runners::{self, Platform},
    steps::{self, FluentBuilder as _, NamedJob, named, setup_cargo_config},
    vars,
};

pub(crate) fn run_agent_evals() -> Workflow {
    let agent_evals = agent_evals();

    named::workflow()
        .on(Event::default()
            .schedule([Schedule::default().cron("0 0 * * *")])
            .pull_request(PullRequest::default().add_branch("**").types([
                PullRequestType::Synchronize,
                PullRequestType::Reopened,
                PullRequestType::Labeled,
            ]))
            .workflow_dispatch(WorkflowDispatch::default()))
        .concurrency(vars::one_workflow_per_non_main_branch())
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("CARGO_INCREMENTAL", 0))
        .add_env(("RUST_BACKTRACE", 1))
        .add_env(("ANTHROPIC_API_KEY", "${{ secrets.ANTHROPIC_API_KEY }}"))
        .add_env((
            "ZED_CLIENT_CHECKSUM_SEED",
            "${{ secrets.ZED_CLIENT_CHECKSUM_SEED }}",
        ))
        .add_env(("ZED_EVAL_TELEMETRY", 1))
        .add_job(agent_evals.name, agent_evals.job)
}

fn agent_evals() -> NamedJob {
    fn run_eval() -> Step<Run> {
        named::bash("cargo run --package=eval -- --repetitions=8 --concurrency=1")
    }

    named::job(
        Job::default()
            .cond(Expression::new(indoc::indoc!{r#"
                github.repository_owner == 'zed-industries' &&
                (github.event_name != 'pull_request' || contains(github.event.pull_request.labels.*.name, 'run-eval'))
            "#}))
            .runs_on(runners::LINUX_DEFAULT)
            .timeout_minutes(60_u32)
            .add_step(steps::checkout_repo())
            .add_step(steps::cache_rust_dependencies())
            .map(steps::install_linux_dependencies)
            .add_step(setup_cargo_config(Platform::Linux))
            .add_step(steps::script("cargo build --package=eval"))
            .add_step(run_eval())
            .add_step(steps::cleanup_cargo_config(Platform::Linux))
    )
}

pub(crate) fn run_unit_evals() -> Workflow {
    let unit_evals = unit_evals();

    named::workflow()
        .on(Event::default()
            .schedule([
                // GitHub might drop jobs at busy times, so we choose a random time in the middle of the night.
                Schedule::default().cron("47 1 * * 2"),
            ])
            .workflow_dispatch(WorkflowDispatch::default()))
        .concurrency(vars::one_workflow_per_non_main_branch())
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("CARGO_INCREMENTAL", 0))
        .add_env(("RUST_BACKTRACE", 1))
        .add_env((
            "ZED_CLIENT_CHECKSUM_SEED",
            "${{ secrets.ZED_CLIENT_CHECKSUM_SEED }}",
        ))
        .add_job(unit_evals.name, unit_evals.job)
}

fn unit_evals() -> NamedJob {
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
            .add_step(
                steps::script("./script/run-unit-evals")
                    .add_env(("ANTHROPIC_API_KEY", "${{ secrets.ANTHROPIC_API_KEY }}")),
            )
            .add_step(send_failure_to_slack())
            .add_step(steps::cleanup_cargo_config(Platform::Linux)),
    )
}
