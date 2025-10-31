use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, NamedJob, named},
};

/// Generates the danger.yml workflow
pub fn compare_perf() -> Workflow {
    let run_perf = run_perf();
    named::workflow()
        .on(Event::default().workflow_dispatch(WorkflowDispatch::default()))
        .add_job(run_perf.name, run_perf.job)
}

pub fn run_perf() -> NamedJob {
    named::job(
        Job::default()
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo()),
    )
}
