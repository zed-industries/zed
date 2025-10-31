use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, NamedJob, named, upload_artifact},
    vars::Input,
};

/// Generates the danger.yml workflow
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
    fn echo_inputs(base: &Input, head: &Input) -> Step<Run> {
        named::bash(&format!("echo {} {}", base.var(), head.var()))
    }

    fn create_results() -> Step<Run> {
        named::bash("echo 'Perf is *much* better now' > target/results.md")
    }

    named::job(
        Job::default()
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo())
            .add_step(echo_inputs(base, head))
            .add_step(create_results())
            .add_step(upload_artifact("results.md", "target/results.md")),
    )
}
