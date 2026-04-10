use gh_workflow::ctx::Context;
use gh_workflow::*;
use indoc::indoc;

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
    let cherry_pick = run_cherry_pick(&branch, &commit, &channel, &pr_number);
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
    pr_number: &WorkflowInput,
) -> NamedJob {
    fn cherry_pick(
        commit: &WorkflowInput,
        channel: &WorkflowInput,
    ) -> (Step<Run>, StepOutput, StepOutput, StepOutput) {
        let step = named::bash(indoc! {r#"
            git cherry-pick -n "$COMMIT"

            COMMIT_TITLE=$(git log -1 --pretty=format:"%s" "$COMMIT")
            COMMIT_BODY=$(git log -1 --pretty=format:"%b" "$COMMIT")
            COMMIT_AUTHOR=$(git log -1 --pretty=format:"%aN <%aE>" "$COMMIT")

            COAUTHORS=$(echo "$COMMIT_BODY" | grep -i "^Co-authored-by:" || true)

            COMMIT_MSG="${COMMIT_TITLE} (cherry-pick to ${CHANNEL})"
            COMMIT_MSG="${COMMIT_MSG}"$'\n'$'\n'"Co-authored-by: ${COMMIT_AUTHOR}"
            if [ -n "$COAUTHORS" ]; then
                COMMIT_MSG="${COMMIT_MSG}"$'\n'"${COAUTHORS}"
            fi

            if [[ "$COMMIT_TITLE" =~ \(#([0-9]+)\)$ ]]; then
                ORIGINAL_PR="${BASH_REMATCH[1]}"
                PR_BODY="Cherry-pick of #${ORIGINAL_PR} to ${CHANNEL}"
            else
                PR_BODY="Cherry-pick of ${COMMIT} to ${CHANNEL}"
            fi

            PR_BODY="${PR_BODY}"$'\n'$'\n'"----"$'\n'"${COMMIT_BODY}"

            {
                echo "title=${COMMIT_TITLE} (cherry-pick to ${CHANNEL})"
                echo "commit_message<<EOF"
                echo "$COMMIT_MSG"
                echo "EOF"
                echo "body<<EOF"
                echo "$PR_BODY"
                echo "EOF"
            } >> "$GITHUB_OUTPUT"
        "#})
        .id("cherry-pick")
        .add_env(("COMMIT", commit.to_string()))
        .add_env(("CHANNEL", channel.to_string()));

        let title = StepOutput::new(&step, "title");
        let commit_message = StepOutput::new(&step, "commit_message");
        let body = StepOutput::new_unchecked(&step, "body");
        (step, title, commit_message, body)
    }

    let (authenticate, token) = steps::authenticate_as_zippy()
        .for_repository(RepositoryTarget::current())
        .with_permissions([
            (TokenPermissions::Contents, Level::Write),
            (TokenPermissions::Workflows, Level::Write),
            (TokenPermissions::PullRequests, Level::Write),
        ])
        .into();

    let (cherry_pick_step, title, commit_message, body) = cherry_pick(commit, channel);

    let pr_branch = format!("zippy-cherry-pick-{pr_number}-{channel}");

    named::job(
        Job::default()
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo().with_full_history().with_ref(branch))
            .add_step(authenticate)
            .add_step(cherry_pick_step)
            .add_step(
                steps::create_pull_request(&token)
                    .with_title(title)
                    .with_body(body)
                    .with_commit_message(commit_message)
                    .with_branch(pr_branch)
                    .with_base(branch)
                    .with_assignees(Context::github().actor()),
            ),
    )
}
