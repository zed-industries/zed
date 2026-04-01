use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, NamedJob, RepositoryTarget, TokenPermissions, named},
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
        named::bash(r#"./script/cherry-pick "$BRANCH" "$COMMIT" "$CHANNEL""#)
            .add_env(("BRANCH", branch.to_string()))
            .add_env(("COMMIT", commit.to_string()))
            .add_env(("CHANNEL", channel.to_string()))
            .add_env(("GIT_COMMITTER_NAME", "Zed Zippy"))
            .add_env(("GIT_COMMITTER_EMAIL", "hi@zed.dev"))
            .add_env(("GITHUB_TOKEN", token))
    }

    let (authenticate, token) = steps::authenticate_as_zippy()
        .for_repository(RepositoryTarget::current())
        .with_permissions([
            (TokenPermissions::Contents, Level::Write),
            (TokenPermissions::Workflows, Level::Write),
        ])
        .into();

    named::job(
        Job::default()
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo())
            .add_step(authenticate)
            .add_step(cherry_pick(branch, commit, channel, &token)),
    )
}
