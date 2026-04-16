use gh_workflow::*;

use crate::tasks::workflows::run_bundling::upload_artifact;
use crate::tasks::workflows::steps::FluentBuilder;
use crate::tasks::workflows::{
    runners,
    steps::{self, NamedJob, named},
    vars::WorkflowInput,
};

pub fn compare_perf() -> Workflow {
    let head = WorkflowInput::string("head", None);
    let base = WorkflowInput::string("base", None);
    let crate_name = WorkflowInput::string("crate_name", Some("".to_owned()));
    let run_perf = run_perf(&base, &head, &crate_name);
    named::workflow()
        .on(Event::default().workflow_dispatch(
            WorkflowDispatch::default()
                .add_input(head.name, head.input())
                .add_input(base.name, base.input())
                .add_input(crate_name.name, crate_name.input()),
        ))
        .add_job(run_perf.name, run_perf.job)
}

pub fn run_perf(
    base: &WorkflowInput,
    head: &WorkflowInput,
    crate_name: &WorkflowInput,
) -> NamedJob {
    fn cargo_perf_test(ref_name: &WorkflowInput, crate_name: &WorkflowInput) -> Step<Run> {
        named::bash(
            r#"
            if [ -n "$CRATE_NAME" ]; then
                cargo perf-test -p "$CRATE_NAME" -- --json="$REF_NAME";
            else
                cargo perf-test -p vim -- --json="$REF_NAME";
            fi"#,
        )
        .add_env(("REF_NAME", ref_name.to_string()))
        .add_env(("CRATE_NAME", crate_name.to_string()))
    }

    fn install_hyperfine() -> Step<Use> {
        named::uses(
            "taiki-e",
            "install-action",
            "b4f2d5cb8597b15997c8ede873eb6185efc5f0ad", // hyperfine
        )
    }

    fn compare_runs(head: &WorkflowInput, base: &WorkflowInput) -> Step<Run> {
        named::bash(r#"cargo perf-compare --save=results.md "$BASE" "$HEAD""#)
            .add_env(("BASE", base.to_string()))
            .add_env(("HEAD", head.to_string()))
    }

    named::job(
        Job::default()
            .runs_on(runners::LINUX_DEFAULT)
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_cargo_config(runners::Platform::Linux))
            .map(steps::install_linux_dependencies)
            .add_step(install_hyperfine())
            .add_step(steps::git_checkout(base))
            .add_step(cargo_perf_test(base, crate_name))
            .add_step(steps::git_checkout(head))
            .add_step(cargo_perf_test(head, crate_name))
            .add_step(compare_runs(head, base))
            .add_step(upload_artifact("results.md"))
            .add_step(steps::cleanup_cargo_config(runners::Platform::Linux)),
    )
}
