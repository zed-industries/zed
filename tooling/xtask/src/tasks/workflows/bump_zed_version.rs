use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, named},
    vars::{self, StepOutput, WorkflowInput},
};

pub fn bump_zed_version() -> Workflow {
    let target = WorkflowInput::string("target", Some("all".to_string()))
        .description("Which channels to bump: all, main, preview, or stable");

    let (versions_job, outputs) = resolve_versions();

    let bump_main_job = bump_main(&target, &versions_job, &outputs);
    let preview_job = create_preview_branch(&target, &versions_job, &outputs);
    let stable_job = promote_to_stable(&target, &versions_job, &outputs);

    named::workflow()
        .on(Event::default()
            .workflow_dispatch(WorkflowDispatch::default().add_input(target.name, target.input())))
        .add_job(versions_job.name, versions_job.job)
        .add_job(bump_main_job.name, bump_main_job.job)
        .add_job(preview_job.name, preview_job.job)
        .add_job(stable_job.name, stable_job.job)
}

struct ResolvedOutputs {
    next_version: vars::JobOutput,
    pr_branch: vars::JobOutput,
    preview_branch: vars::JobOutput,
    preview_tag: vars::JobOutput,
    stable_branch: vars::JobOutput,
}

fn resolve_versions() -> (steps::NamedJob, ResolvedOutputs) {
    fn extract_versions() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            version=$(script/get-crate-version zed)
            major=$(echo "$version" | cut -d. -f1)
            minor=$(echo "$version" | cut -d. -f2)

            channel=$(cat crates/zed/RELEASE_CHANNEL)
            if [[ "$channel" != "dev" && "$channel" != "nightly" ]]; then
                echo "::error::release channel on main should be dev or nightly, found: $channel"
                exit 1
            fi

            # Next main version after bump
            next_version="${major}.$((minor + 1)).0"
            next_major=$(echo "$next_version" | cut -d. -f1)
            next_minor=$(echo "$next_version" | cut -d. -f2)
            pr_branch="bump-zed-to-v${next_major}.${next_minor}.0"

            # New preview branch from current main
            preview_branch="v${major}.${minor}.x"
            preview_tag="v${version}-pre"

            # Current preview to promote to stable — derive branch from released preview version
            released_preview=$(script/get-released-version preview)
            if [[ -z "$released_preview" ]]; then
                echo "::error::could not determine released preview version"
                exit 1
            fi
            stable_major=$(echo "$released_preview" | cut -d. -f1)
            stable_minor=$(echo "$released_preview" | cut -d. -f2)
            stable_branch="v${stable_major}.${stable_minor}.x"

            # Final validation
            for var in next_version pr_branch preview_branch preview_tag stable_branch; do
                if [[ -z "${!var}" ]]; then
                    echo "::error::failed to compute $var"
                    exit 1
                fi
            done

            {
                echo "next_version=$next_version"
                echo "pr_branch=$pr_branch"
                echo "preview_branch=$preview_branch"
                echo "preview_tag=$preview_tag"
                echo "stable_branch=$stable_branch"
            } >> "$GITHUB_OUTPUT"

            echo "Resolved: next=$next_version preview=$preview_branch($preview_tag) stable=$stable_branch pr=$pr_branch"
        "#})
        .id("versions")
    }

    let (authenticate, token) = steps::authenticate_as_zippy().into();
    let versions_step = extract_versions();
    let next_version = StepOutput::new(&versions_step, "next_version");
    let pr_branch = StepOutput::new(&versions_step, "pr_branch");
    let preview_branch = StepOutput::new(&versions_step, "preview_branch");
    let preview_tag = StepOutput::new(&versions_step, "preview_tag");
    let stable_branch = StepOutput::new(&versions_step, "stable_branch");

    let job = named::job(
        Job::default()
            .cond(Expression::new(
                "github.repository_owner == 'zed-industries'",
            ))
            .runs_on(runners::LINUX_XL)
            .add_step(authenticate)
            .add_step(steps::checkout_repo().with_token(&token).with_ref("main"))
            .add_step(versions_step)
            .outputs([
                (next_version.name.to_owned(), next_version.to_string()),
                (pr_branch.name.to_owned(), pr_branch.to_string()),
                (preview_branch.name.to_owned(), preview_branch.to_string()),
                (preview_tag.name.to_owned(), preview_tag.to_string()),
                (stable_branch.name.to_owned(), stable_branch.to_string()),
            ]),
    );

    let outputs = ResolvedOutputs {
        next_version: next_version.as_job_output(&job),
        pr_branch: pr_branch.as_job_output(&job),
        preview_branch: preview_branch.as_job_output(&job),
        preview_tag: preview_tag.as_job_output(&job),
        stable_branch: stable_branch.as_job_output(&job),
    };

    (job, outputs)
}

fn bump_main(
    target: &WorkflowInput,
    versions_job: &steps::NamedJob,
    outputs: &ResolvedOutputs,
) -> steps::NamedJob {
    fn bump_version() -> Step<Run> {
        named::bash("cargo set-version -p zed --bump minor")
    }

    let (authenticate, token) = steps::authenticate_as_zippy().into();

    named::job(
        Job::default()
            .cond(Expression::new(format!(
                "{} == 'all' || {} == 'main'",
                target.expr(),
                target.expr(),
            )))
            .needs(vec![versions_job.name.clone()])
            .runs_on(runners::LINUX_DEFAULT)
            .add_step(authenticate)
            .add_step(steps::checkout_repo().with_token(&token).with_ref("main"))
            .add_step(steps::install_cargo_edit())
            .add_step(bump_version())
            .add_step(steps::CreatePrStep::new(
                format!("Bump Zed to v{}", outputs.next_version),
                &outputs.pr_branch,
                &token,
            )),
    )
}

fn create_preview_branch(
    target: &WorkflowInput,
    versions_job: &steps::NamedJob,
    outputs: &ResolvedOutputs,
) -> steps::NamedJob {
    fn promote_to_preview() -> Step<Run> {
        named::bash("echo -n preview > crates/zed/RELEASE_CHANNEL")
    }

    fn get_main_sha() -> Step<Run> {
        named::bash("echo \"main_sha=$(git rev-parse HEAD)\" >> \"$GITHUB_OUTPUT\"").id("main-sha")
    }

    let (authenticate, token) = steps::authenticate_as_zippy().into();

    let main_sha_step = get_main_sha();
    let main_sha = StepOutput::new(&main_sha_step, "main_sha");

    let commit_step: Step<Use> = steps::BotCommitStep::new(
        format!("{} preview", outputs.preview_branch),
        &outputs.preview_branch,
        &token,
    )
    .with_files("crates/zed/RELEASE_CHANNEL")
    .into();
    let commit_sha = StepOutput::new_unchecked(&commit_step, "commit");

    named::job(
        Job::default()
            .cond(Expression::new(format!(
                "{} == 'all' || {} == 'preview'",
                target.expr(),
                target.expr(),
            )))
            .needs(vec![versions_job.name.clone()])
            .runs_on(runners::LINUX_DEFAULT)
            .add_step(authenticate)
            .add_step(steps::checkout_repo().with_token(&token).with_ref("main"))
            .add_step(main_sha_step)
            .add_step(promote_to_preview())
            .add_step(steps::create_ref(
                steps::GitRef::branch(&outputs.preview_branch),
                &main_sha,
                &token,
            ))
            .add_step(commit_step)
            .add_step(steps::create_ref(
                steps::GitRef::tag(&outputs.preview_tag),
                &commit_sha,
                &token,
            )),
    )
}

fn promote_to_stable(
    target: &WorkflowInput,
    versions_job: &steps::NamedJob,
    outputs: &ResolvedOutputs,
) -> steps::NamedJob {
    let (authenticate, token) = steps::authenticate_as_zippy().into();

    let read_version_step = named::bash(indoc::indoc! {r#"
            stable_version=$(script/get-crate-version zed)
            {
                echo "stable_tag=v${stable_version}"
            } >> "$GITHUB_OUTPUT"
        "#})
    .id("stable-info");
    let stable_tag = StepOutput::new(&read_version_step, "stable_tag");

    let write_channel = named::bash("echo -n stable > crates/zed/RELEASE_CHANNEL");

    let commit_step: Step<Use> = steps::BotCommitStep::new(
        format!("{} stable", outputs.stable_branch),
        &outputs.stable_branch,
        &token,
    )
    .with_files("crates/zed/RELEASE_CHANNEL")
    .into();
    let commit_sha = StepOutput::new_unchecked(&commit_step, "commit");

    named::job(
        Job::default()
            .cond(Expression::new(format!(
                "{} == 'all' || {} == 'stable'",
                target.expr(),
                target.expr(),
            )))
            .needs(vec![versions_job.name.clone()])
            .runs_on(runners::LINUX_DEFAULT)
            .add_step(authenticate)
            .add_step(
                steps::checkout_repo()
                    .with_token(&token)
                    .with_ref(outputs.stable_branch.to_string()),
            )
            .add_step(read_version_step)
            .add_step(write_channel)
            .add_step(commit_step)
            .add_step(steps::create_ref(
                steps::GitRef::tag(&stable_tag),
                &commit_sha,
                &token,
            )),
    )
}
