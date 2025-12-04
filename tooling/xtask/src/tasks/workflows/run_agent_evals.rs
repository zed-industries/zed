use gh_workflow::{
    Event, Expression, Job, Run, Schedule, Step, Strategy, Use, Workflow, WorkflowDispatch,
};
use serde_json::json;

use crate::tasks::workflows::{
    runners::{self, Platform},
    steps::{self, FluentBuilder as _, NamedJob, named, setup_cargo_config},
    vars::{self, WorkflowInput},
};

pub(crate) fn run_agent_evals() -> Workflow {
    let agent_evals = agent_evals();
    let model_name = WorkflowInput::string("model_name", None);

    named::workflow()
        .on(Event::default().workflow_dispatch(
            WorkflowDispatch::default().add_input(model_name.name, model_name.input()),
        ))
        .concurrency(vars::one_workflow_per_non_main_branch())
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("CARGO_INCREMENTAL", 0))
        .add_env(("RUST_BACKTRACE", 1))
        .add_env(("ANTHROPIC_API_KEY", vars::ANTHROPIC_API_KEY))
        .add_env(("OPENAI_API_KEY", vars::OPENAI_API_KEY))
        .add_env(("GOOGLE_AI_API_KEY", vars::GOOGLE_AI_API_KEY))
        .add_env(("GOOGLE_CLOUD_PROJECT", vars::GOOGLE_CLOUD_PROJECT))
        .add_env(("ZED_CLIENT_CHECKSUM_SEED", vars::ZED_CLIENT_CHECKSUM_SEED))
        .add_env(("ZED_EVAL_TELEMETRY", 1))
        .add_env(("MODEL_NAME", model_name.to_string()))
        .add_job(agent_evals.name, agent_evals.job)
}

pub(crate) fn run_unit_evals() -> Workflow {
    let model_name = WorkflowInput::string("model_name", None);
    let commit_sha = WorkflowInput::string("commit_sha", None);

    let unit_evals = named::job(unit_evals(Some(&commit_sha)));

    named::workflow()
        .name("run_unit_evals")
        .on(Event::default().workflow_dispatch(
            WorkflowDispatch::default()
                .add_input(model_name.name, model_name.input())
                .add_input(commit_sha.name, commit_sha.input()),
        ))
        .concurrency(vars::allow_concurrent_runs())
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("CARGO_INCREMENTAL", 0))
        .add_env(("RUST_BACKTRACE", 1))
        .add_env(("ZED_CLIENT_CHECKSUM_SEED", vars::ZED_CLIENT_CHECKSUM_SEED))
        .add_env(("ZED_EVAL_TELEMETRY", 1))
        .add_env(("MODEL_NAME", model_name.to_string()))
        .add_job(unit_evals.name, unit_evals.job)
}

fn add_api_keys(step: Step<Run>) -> Step<Run> {
    step.add_env(("ANTHROPIC_API_KEY", vars::ANTHROPIC_API_KEY))
        .add_env(("OPENAI_API_KEY", vars::OPENAI_API_KEY))
        .add_env(("GOOGLE_AI_API_KEY", vars::GOOGLE_AI_API_KEY))
        .add_env(("GOOGLE_CLOUD_PROJECT", vars::GOOGLE_CLOUD_PROJECT))
}

fn agent_evals() -> NamedJob {
    fn run_eval() -> Step<Run> {
        named::bash(
            "cargo run --package=eval -- --repetitions=8 --concurrency=1 --model \"${MODEL_NAME}\"",
        )
    }

    named::job(
        Job::default()
            .runs_on(runners::LINUX_DEFAULT)
            .timeout_minutes(60_u32 * 10)
            .add_step(steps::checkout_repo())
            .add_step(steps::cache_rust_dependencies_namespace())
            .map(steps::install_linux_dependencies)
            .add_step(setup_cargo_config(Platform::Linux))
            .add_step(steps::script("cargo build --package=eval"))
            .add_step(add_api_keys(run_eval()))
            .add_step(steps::cleanup_cargo_config(Platform::Linux)),
    )
}

pub(crate) fn run_cron_unit_evals() -> Workflow {
    let unit_evals = cron_unit_evals();

    named::workflow()
        .name("run_cron_unit_evals")
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
        .add_env(("ZED_CLIENT_CHECKSUM_SEED", vars::ZED_CLIENT_CHECKSUM_SEED))
        .add_job(unit_evals.name, unit_evals.job)
}

fn cron_unit_evals() -> NamedJob {
    fn send_failure_to_slack() -> Step<Use> {
        named::uses(
            "slackapi",
            "slack-github-action",
            "b0fa283ad8fea605de13dc3f449259339835fc52",
        )
        .if_condition(Expression::new("${{ failure() }}"))
        .add_with(("method", "chat.postMessage"))
        .add_with(("token", vars::SLACK_APP_ZED_UNIT_EVALS_BOT_TOKEN))
        .add_with(("payload", indoc::indoc!{r#"
            channel: C04UDRNNJFQ
            text: "Unit Evals Failed: https://github.com/zed-industries/zed/actions/runs/${{ github.run_id }}"
        "#}))
    }

    named::job(cron_unit_evals_job().add_step(send_failure_to_slack()))
}

const UNIT_EVAL_MODELS: &[&str] = &[
    "anthropic/claude-sonnet-4-5-latest",
    "anthropic/claude-opus-4-5-latest",
    "google/gemini-3-pro",
    "openai/gpt-5",
];

fn cron_unit_evals_job() -> Job {
    let script_step = add_api_keys(steps::script("./script/run-unit-evals"))
        .add_env(("ZED_AGENT_MODEL", "${{ matrix.model }}"));

    Job::default()
        .runs_on(runners::LINUX_DEFAULT)
        .strategy(Strategy::default().fail_fast(false).matrix(json!({
            "model": UNIT_EVAL_MODELS
        })))
        .add_step(steps::checkout_repo())
        .add_step(steps::setup_cargo_config(Platform::Linux))
        .add_step(steps::cache_rust_dependencies_namespace())
        .map(steps::install_linux_dependencies)
        .add_step(steps::cargo_install_nextest())
        .add_step(steps::clear_target_dir_if_large(Platform::Linux))
        .add_step(script_step)
        .add_step(steps::cleanup_cargo_config(Platform::Linux))
}

fn unit_evals(commit: Option<&WorkflowInput>) -> Job {
    let script_step = add_api_keys(steps::script("./script/run-unit-evals"));

    Job::default()
        .runs_on(runners::LINUX_DEFAULT)
        .add_step(steps::checkout_repo())
        .add_step(steps::setup_cargo_config(Platform::Linux))
        .add_step(steps::cache_rust_dependencies_namespace())
        .map(steps::install_linux_dependencies)
        .add_step(steps::cargo_install_nextest())
        .add_step(steps::clear_target_dir_if_large(Platform::Linux))
        .add_step(match commit {
            Some(commit) => script_step.add_env(("UNIT_EVAL_COMMIT", commit)),
            None => script_step,
        })
        .add_step(steps::cleanup_cargo_config(Platform::Linux))
}
