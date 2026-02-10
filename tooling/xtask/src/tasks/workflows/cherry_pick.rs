use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, NamedJob, named},
    vars::{StepOutput, WorkflowInput},
};

pub fn cherry_pick() -> Workflow {
    let branch = WorkflowInput::string("branch", None);
    let commit = WorkflowInput::string("commit", None);
    let channel = WorkflowInput::string("channel", None);
    let pr_number = WorkflowInput::string("pr_number", None);
    let cherry_pick = run_cherry_pick(&branch, &commit, &channel);
    named::workflow()
        .run_name(format!("cherry_pick to {channel} #{pr_number}"))
        .on(Event::default().workflow_dispatch(
            WorkflowDispatch::default()
                .add_input(commit.name, commit.input())
                .add_input(branch.name, branch.input())
                .add_input(channel.name, channel.input())
                .add_input(pr_number.name, pr_number.input()),
        ))
        .add_job(cherry_pick.name, cherry_pick.job)
}

fn run_cherry_pick(
    branch: &WorkflowInput,
    commit: &WorkflowInput,
    channel: &WorkflowInput,
) -> NamedJob {
    fn cherry_pick(
        branch: &WorkflowInput,
        commit: &WorkflowInput,
        channel: &WorkflowInput,
        token: &StepOutput,
    ) -> Step<Run> {
        named::bash(&format!("./script/cherry-pick {branch} {commit} {channel}"))
            .add_env(("GIT_COMMITTER_NAME", "Zed Zippy"))
            .add_env(("GIT_COMMITTER_EMAIL", "hi@zed.dev"))
            .add_env(("GITHUB_TOKEN", token))
    }

    let (authenticate, token) = steps::authenticate_as_zippy();

    named::job(
        Job::default()
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo())
            .add_step(authenticate)
            .add_step(cherry_pick(branch, commit, channel, &token)),
    )
}
