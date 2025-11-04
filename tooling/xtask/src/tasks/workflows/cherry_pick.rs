use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, NamedJob, named},
    vars::Input,
};

pub fn cherry_pick() -> Workflow {
    let branch = Input::string("branch", None);
    let commit = Input::string("commit", None);
    let cherry_pick = run_cherry_pick(&branch, &commit);
    named::workflow()
        .on(Event::default().workflow_dispatch(
            WorkflowDispatch::default()
                .add_input(commit.name, commit.input())
                .add_input(branch.name, branch.input()),
        ))
        .add_job(cherry_pick.name, cherry_pick.job)
}

fn run_cherry_pick(branch: &Input, commit: &Input) -> NamedJob {
    fn cherry_pick(branch: &str, commit: &str) -> Step<Run> {
        named::bash(&format!("./scripts/cherry_pick.sh {branch} {commit}"))
    }

    named::job(
        Job::default()
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo())
            .add_step(cherry_pick(&branch.var(), &commit.var())),
    )
}
