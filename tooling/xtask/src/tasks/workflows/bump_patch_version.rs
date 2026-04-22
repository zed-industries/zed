use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, CheckoutStep, CommonJobConditions, named},
    vars::{StepOutput, WorkflowInput},
};

pub fn bump_patch_version() -> Workflow {
    let branch = WorkflowInput::string("branch", None).description("Branch name to run on");
    let retag = WorkflowInput::bool("retag", Some(false))
        .description("Re-tag the current version instead of bumping (force-updates the tag)");
    let bump_patch_version_job = run_bump_patch_version(&branch, &retag);
    named::workflow()
        .on(Event::default().workflow_dispatch(
            WorkflowDispatch::default()
                .add_input(branch.name, branch.input())
                .add_input(retag.name, retag.input()),
        ))
        .concurrency(
            Concurrency::new(Expression::new(format!(
                "${{{{ github.workflow }}}}-{branch}"
            )))
            .cancel_in_progress(true),
        )
        .add_job(bump_patch_version_job.name, bump_patch_version_job.job)
}

fn run_bump_patch_version(branch: &WorkflowInput, retag: &WorkflowInput) -> steps::NamedJob {
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

            echo "channel=$channel" >> "$GITHUB_OUTPUT"
            echo "tag_suffix=$tag_suffix" >> "$GITHUB_OUTPUT"
        "#})
        .id("channel")
    }

    fn bump_version(retag: &WorkflowInput) -> Step<Run> {
        named::bash(r#"cargo set-version -p zed --bump patch"#)
            .if_condition(Expression::new(format!("{} != true", retag.expr())))
    }

    fn resolve_version() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            version=$(script/get-crate-version zed)
            {
                echo "version=$version"
                echo "head_sha=$(git rev-parse HEAD)"
            } >> "$GITHUB_OUTPUT"
        "#})
        .id("version")
    }

    fn verify_no_existing_release() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            released=$(script/get-released-version "$CHANNEL" "$VERSION")
            if [[ "$released" == "$VERSION" ]]; then
                echo "::error::version $VERSION is already released on $CHANNEL"
                exit 1
            fi
        "#})
        .add_env(("CHANNEL", "${{ steps.channel.outputs.channel }}"))
        .add_env(("VERSION", "${{ steps.version.outputs.version }}"))
    }

    let not_retag = Expression::new(format!("{} != true", retag.expr()));

    let (authenticate, token) = steps::authenticate_as_zippy().into();
    let channel_step = read_channel();
    let tag_suffix = StepOutput::new(&channel_step, "tag_suffix");
    let version_step = resolve_version();
    let version = StepOutput::new(&version_step, "version");
    let head_sha = StepOutput::new(&version_step, "head_sha");

    let commit_step: Step<Use> = steps::BotCommitStep::new(
        format!("Bump to {version} for @${{{{ github.actor }}}}"),
        branch,
        &token,
    )
    .into();
    let commit_step = commit_step.if_condition(not_retag.clone());
    let commit_sha = StepOutput::new_unchecked(&commit_step, "commit");

    let tag_name = format!("v{version}{tag_suffix}");

    let create_tag: Step<Use> = steps::create_tag(&tag_name, &commit_sha, &token).into();
    let create_tag = create_tag.if_condition(not_retag);

    let is_retag = Expression::new(format!("{} == true", retag.expr()));
    let update_tag: Step<Use> = steps::update_tag(&tag_name, &head_sha, &token, true).into();
    let update_tag = update_tag.if_condition(is_retag);

    named::job(
        Job::default()
            .with_repository_owner_guard()
            .runs_on(runners::LINUX_DEFAULT)
            .add_step(authenticate)
            .add_step(checkout_branch(branch, &token))
            .add_step(channel_step)
            .add_step(
                steps::cargo_install("cargo-edit")
                    .if_condition(Expression::new(format!("{} != true", retag.expr()))),
            )
            .add_step(bump_version(retag))
            .add_step(version_step)
            .add_step(verify_no_existing_release())
            .add_step(commit_step)
            .add_step(create_tag)
            .add_step(update_tag),
    )
}
