use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, CheckoutStep, CommonJobConditions, named},
    vars::{StepOutput, WorkflowInput},
};

pub fn retag_release() -> Workflow {
    let branch = WorkflowInput::string("branch", None)
        .description("Release branch to re-tag (e.g. v0.180.x)");
    let retag_job = run_retag_release(&branch);
    named::workflow()
        .on(Event::default()
            .workflow_dispatch(WorkflowDispatch::default().add_input(branch.name, branch.input())))
        .concurrency(
            Concurrency::new(Expression::new(format!(
                "${{{{ github.workflow }}}}-{branch}"
            )))
            .cancel_in_progress(true),
        )
        .add_job(retag_job.name, retag_job.job)
}

fn run_retag_release(branch: &WorkflowInput) -> steps::NamedJob {
    fn checkout_branch(branch: &WorkflowInput, token: &StepOutput) -> CheckoutStep {
        steps::checkout_repo()
            .with_token(token)
            .with_ref(branch.to_string())
    }

    fn resolve_tag(branch: &WorkflowInput) -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            if [[ ! "$BRANCH" =~ ^v[0-9]+\.[0-9]{1,3}\.x$ ]]; then
                echo "::error::branch '$BRANCH' does not match the release branch pattern v[N].[N].x"
                exit 1
            fi

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
                echo "head_sha=$(git rev-parse HEAD)"
            } >> "$GITHUB_OUTPUT"
        "#})
        .id("info")
        .add_env(("BRANCH", branch.to_string()))
    }

    fn verify_no_existing_release() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            status=$(curl -s -o /dev/null -w '%{http_code}' "https://cloud.zed.dev/releases/$CHANNEL/$VERSION/asset?asset=zed&os=macos&arch=aarch64")
            if [[ "$status" == "200" ]]; then
                echo "::error::version $VERSION is already released on $CHANNEL — cannot re-tag a released version"
                exit 1
            fi
        "#})
        .add_env(("CHANNEL", "${{ steps.info.outputs.channel }}"))
        .add_env(("VERSION", "${{ steps.info.outputs.version }}"))
    }

    let (authenticate, token) = steps::authenticate_as_zippy().into();
    let resolve_step = resolve_tag(branch);
    let version = StepOutput::new(&resolve_step, "version");
    let tag_suffix = StepOutput::new(&resolve_step, "tag_suffix");
    let head_sha = StepOutput::new(&resolve_step, "head_sha");

    named::job(
        Job::default()
            .with_repository_owner_guard()
            .runs_on(runners::LINUX_XL)
            .add_step(authenticate)
            .add_step(checkout_branch(branch, &token))
            .add_step(resolve_step)
            .add_step(verify_no_existing_release())
            .add_step(steps::update_ref(
                steps::GitRef::tag(format!("v{version}{tag_suffix}")),
                &head_sha,
                &token,
                true,
            )),
    )
}
