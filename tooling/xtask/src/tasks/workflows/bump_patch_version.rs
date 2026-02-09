use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, named},
    vars::{StepOutput, WorkflowInput},
};

pub fn bump_patch_version() -> Workflow {
    let branch = WorkflowInput::string("branch", None).description("Branch name to run on");
    let bump_patch_version_job = run_bump_patch_version(&branch);
    named::workflow()
        .on(Event::default()
            .workflow_dispatch(WorkflowDispatch::default().add_input(branch.name, branch.input())))
        .concurrency(
            Concurrency::new(Expression::new(format!(
                "${{{{ github.workflow }}}}-{branch}"
            )))
            .cancel_in_progress(true),
        )
        .add_job(bump_patch_version_job.name, bump_patch_version_job.job)
}

fn run_bump_patch_version(branch: &WorkflowInput) -> steps::NamedJob {
    fn checkout_branch(branch: &WorkflowInput, token: &StepOutput) -> Step<Use> {
        steps::checkout_repo_with_token(token).add_with(("ref", branch.to_string()))
    }

    fn bump_patch_version(token: &StepOutput) -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            channel="$(cat crates/zed/RELEASE_CHANNEL)"

            tag_suffix=""
            case $channel in
              stable)
                ;;
              preview)
                tag_suffix="-pre"
                ;;
              *)
                echo "this must be run on either of stable|preview release branches" >&2
                exit 1
                ;;
            esac
            which cargo-set-version > /dev/null || cargo install cargo-edit -f --no-default-features --features "set-version"
            output="$(cargo set-version -p zed --bump patch 2>&1 | sed 's/.* //')"
            git commit -am "Bump to $output for @$GITHUB_ACTOR"
            git tag "v${output}${tag_suffix}"
            git push origin HEAD "v${output}${tag_suffix}"
        "#})
        .add_env(("GIT_COMMITTER_NAME", "Zed Zippy"))
        .add_env((
            "GIT_COMMITTER_EMAIL",
            "234243425+zed-zippy[bot]@users.noreply.github.com",
        ))
        .add_env(("GIT_AUTHOR_NAME", "Zed Zippy"))
        .add_env((
            "GIT_AUTHOR_EMAIL",
            "234243425+zed-zippy[bot]@users.noreply.github.com",
        ))
        .add_env(("GITHUB_TOKEN", token))
    }

    let (authenticate, token) = steps::authenticate_as_zippy();

    named::job(
        Job::default()
            .cond(Expression::new(
                "github.repository_owner == 'zed-industries'",
            ))
            .runs_on(runners::LINUX_XL)
            .add_step(authenticate)
            .add_step(checkout_branch(branch, &token))
            .add_step(bump_patch_version(&token)),
    )
}
