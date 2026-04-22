use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, CheckoutStep, named},
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

    fn bump_version() -> Step<Run> {
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
            version="$(cargo set-version -p zed --bump patch 2>&1 | sed 's/.* //')"
            echo "version=$version" >> "$GITHUB_OUTPUT"
            echo "tag_suffix=$tag_suffix" >> "$GITHUB_OUTPUT"
        "#})
        .id("bump-version")
    }

    fn commit_changes(
        version: &StepOutput,
        token: &StepOutput,
        branch: &WorkflowInput,
    ) -> Step<Use> {
        named::uses(
            "IAreKyleW00t",
            "verified-bot-commit",
            "126a6a11889ab05bcff72ec2403c326cd249b84c", // v2.3.0
        )
        .id("commit")
        .add_with((
            "message",
            format!("Bump to {version} for @${{{{ github.actor }}}}"),
        ))
        .add_with(("ref", format!("refs/heads/{branch}")))
        .add_with(("files", "**"))
        .add_with(("token", token.to_string()))
    }

    fn create_version_tag(
        version: &StepOutput,
        tag_suffix: &StepOutput,
        commit_sha: &StepOutput,
        token: &StepOutput,
    ) -> Step<Use> {
        named::uses(
            "actions",
            "github-script",
            "f28e40c7f34bde8b3046d885e986cb6290c5673b", // v7
        )
        .with(
            Input::default()
                .add(
                    "script",
                    indoc::formatdoc! {r#"
                        github.rest.git.createRef({{
                            owner: context.repo.owner,
                            repo: context.repo.repo,
                            ref: 'refs/tags/v{version}{tag_suffix}',
                            sha: '{commit_sha}'
                        }})
                    "#},
                )
                .add("github-token", token.to_string()),
        )
    }

    let (authenticate, token) = steps::authenticate_as_zippy().into();
    let bump_version_step = bump_version();
    let version = StepOutput::new(&bump_version_step, "version");
    let tag_suffix = StepOutput::new(&bump_version_step, "tag_suffix");
    let commit_step = commit_changes(&version, &token, branch);
    let commit_sha = StepOutput::new_unchecked(&commit_step, "commit");

    named::job(
        Job::default()
            .cond(Expression::new(
                "github.repository_owner == 'zed-industries'",
            ))
            .runs_on(runners::LINUX_XL)
            .add_step(authenticate)
            .add_step(checkout_branch(branch, &token))
            .add_step(bump_version_step)
            .add_step(commit_step)
            .add_step(create_version_tag(
                &version,
                &tag_suffix,
                &commit_sha,
                &token,
            )),
    )
}
