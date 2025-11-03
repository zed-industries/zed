use gh_workflow::*;

use crate::tasks::workflows::steps::FluentBuilder;
use crate::tasks::workflows::{
    runners,
    steps::{self, named, upload_artifact, NamedJob},
    vars::Input,
};

pub fn compare_perf() -> Workflow {
    let head = Input::string("head", None);
    let base = Input::string("base", None);
    let run_perf = run_perf(&base, &head);
    named::workflow()
        .on(Event::default().workflow_dispatch(
            WorkflowDispatch::default()
                .add_input(head.name, head.input())
                .add_input(base.name, base.input()),
        ))
        .add_job(run_perf.name, run_perf.job)
}

pub fn run_perf(base: &Input, head: &Input) -> NamedJob {
    fn cargo_perf_test(ref_name: String) -> Step<Run> {
        // TODO: vim not gpui, and ideally allow args
        named::bash(&format!("cargo perf-test -p gpui -- --json={ref_name}"))
    }

    fn git_checkout(ref_name: String) -> Step<Run> {
        named::bash(&format!(
            "git fetch origin {ref_name} && git checkout {ref_name}"
        ))
    }

    fn compare_runs(head: String, base: String) -> Step<Run> {
        // TODO: this should really be swapped...
        named::bash(&format!(
            "cargo perf-compare {base} {head} --save=results.md"
        ))
    }

    named::job(
        Job::default()
            .runs_on(runners::LINUX_DEFAULT)
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_cargo_config(runners::Platform::Linux))
            .map(steps::install_linux_dependencies)
            .add_step(git_checkout(base.var()))
            .add_step(cargo_perf_test(base.var()))
            .add_step(git_checkout(head.var()))
            .add_step(cargo_perf_test(head.var()))
            .add_step(compare_runs(head.var(), base.var()))
            .add_step(upload_artifact("results.md", "results.md"))
            .add_step(steps::cleanup_cargo_config(runners::Platform::Linux)),
    )
}
