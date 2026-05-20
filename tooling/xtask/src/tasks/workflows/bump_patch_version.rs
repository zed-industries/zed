use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, CheckoutStep, CommonJobConditions, named},
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
    fn checkout_branch(branch: &WorkflowInput, token: &StepOutput) -> CheckoutStep {
        steps::checkout_repo()
            .with_token(token)
            .with_ref(branch.to_string())
    }

    fn read_channel() -> Step<Run> {
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
                echo "::error::must be run on a stable or preview release branch"
                exit 1
                ;;
            esac

            version=$(script/get-crate-version zed)

            {
                echo "channel=$channel"
                echo "version=$version"
                echo "tag_suffix=$tag_suffix"
            } >> "$GITHUB_OUTPUT"
        "#})
        .id("channel")
    }

    fn bump_version() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            version="$(cargo set-version -p zed --bump patch 2>&1 | sed 's/.* //')"
            echo "version=$version" >> "$GITHUB_OUTPUT"
        "#})
        .id("bump-version")
    }

    let (authenticate, token) = steps::authenticate_as_zippy().into();
    let channel_step = read_channel();
    let tag_suffix = StepOutput::new(&channel_step, "tag_suffix");
    let bump_version_step = bump_version();
    let version = StepOutput::new(&bump_version_step, "version");
    let commit_step: Step<Use> = steps::BotCommitStep::new(
        format!("Bump to {version} for @${{{{ github.actor }}}}"),
        branch,
        &token,
    )
    .into();
    let commit_sha = StepOutput::new_unchecked(&commit_step, "commit");

    named::job(
        Job::default()
            .with_repository_owner_guard()
            .runs_on(runners::LINUX_DEFAULT)
            .add_step(authenticate)
            .add_step(checkout_branch(branch, &token))
            .add_step(channel_step)
            .add_step(steps::install_cargo_edit())
            .add_step(bump_version_step)
            .add_step(commit_step)
            .add_step(steps::create_ref(
                steps::GitRef::tag(format!("v{version}{tag_suffix}")),
                &commit_sha,
                &token,
            )),
    )
}
