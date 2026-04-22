use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, named},
    vars::{self, StepOutput, WorkflowInput},
};

pub fn bump_zed_version() -> Workflow {
    let bump_type =
        WorkflowInput::string("bump_type", None).description("Version bump type: major or minor");
    let target = WorkflowInput::string("target", Some("all".to_string()))
        .description("Which channels to bump: all, main, preview, or stable");
    let tag_only = WorkflowInput::bool("tag_only", Some(false))
        .description("Only create tags on existing branches (skip version bumps and commits)");

    let (versions_job, outputs) = resolve_versions(&bump_type);

    let bump_main_job = bump_main(&target, &tag_only, &versions_job, &outputs);
    let preview_job = create_preview_branch(&target, &tag_only, &versions_job, &outputs);
    let stable_job = promote_to_stable(&target, &tag_only, &versions_job, &outputs);

    named::workflow()
        .on(Event::default().workflow_dispatch(
            WorkflowDispatch::default()
                .add_input(bump_type.name, bump_type.input())
                .add_input(target.name, target.input())
                .add_input(tag_only.name, tag_only.input()),
        ))
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

fn resolve_versions(bump_type: &WorkflowInput) -> (steps::NamedJob, ResolvedOutputs) {
    fn extract_versions(bump_type: &WorkflowInput) -> Step<Run> {
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
            case "$BUMP_TYPE" in
                minor)
                    next_version="${major}.$((minor + 1)).0"
                    ;;
                major)
                    next_version="$((major + 1)).0.0"
                    ;;
                *)
                    echo "::error::bump_type must be 'major' or 'minor', got: $BUMP_TYPE"
                    exit 1
                    ;;
            esac

            next_major=$(echo "$next_version" | cut -d. -f1)
            next_minor=$(echo "$next_version" | cut -d. -f2)
            pr_branch="bump-zed-to-${next_major}.${next_minor}"

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

            echo "next_version=$next_version" >> "$GITHUB_OUTPUT"
            echo "pr_branch=$pr_branch" >> "$GITHUB_OUTPUT"
            echo "preview_branch=$preview_branch" >> "$GITHUB_OUTPUT"
            echo "preview_tag=$preview_tag" >> "$GITHUB_OUTPUT"
            echo "stable_branch=$stable_branch" >> "$GITHUB_OUTPUT"

            echo "Resolved: next=$next_version preview=$preview_branch($preview_tag) stable=$stable_branch pr=$pr_branch"
        "#})
        .id("versions")
        .add_env(("BUMP_TYPE", bump_type.to_string()))
    }

    let (authenticate, token) = steps::authenticate_as_zippy().into();
    let versions_step = extract_versions(bump_type);
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
    tag_only: &WorkflowInput,
    versions_job: &steps::NamedJob,
    outputs: &ResolvedOutputs,
) -> steps::NamedJob {
    fn bump_version(outputs: &ResolvedOutputs) -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            which cargo-set-version > /dev/null || cargo install cargo-edit -f --no-default-features --features "set-version"
            cargo set-version -p zed --bump "$BUMP_TYPE"
        "#})
        .add_env(("BUMP_TYPE", "${{ inputs.bump_type }}"))
        .add_env(("NEXT_VERSION", outputs.next_version.to_string()))
    }

    let (authenticate, token) = steps::authenticate_as_zippy().into();
    let main_sha_step =
        named::bash("echo \"main_sha=$(git rev-parse HEAD)\" >> \"$GITHUB_OUTPUT\"").id("main-sha");
    let main_sha = StepOutput::new(&main_sha_step, "main_sha");

    let commit_step: Step<Use> = steps::BotCommitStep::new(
        format!(
            "Bump to v{} for @${{{{ github.actor }}}}",
            outputs.next_version
        ),
        &outputs.pr_branch,
        &token,
    )
    .into();

    named::job(
        Job::default()
            .cond(Expression::new(format!(
                "({} == 'all' || {} == 'main') && {} != true",
                target.expr(),
                target.expr(),
                tag_only.expr(),
            )))
            .needs(vec![versions_job.name.clone()])
            .runs_on(runners::LINUX_XL)
            .add_step(authenticate)
            .add_step(steps::checkout_repo().with_token(&token).with_ref("main"))
            .add_step(main_sha_step)
            .add_step(bump_version(outputs))
            .add_step(steps::CreateBranchStep::new(
                &outputs.pr_branch,
                &main_sha,
                &token,
            ))
            .add_step(commit_step)
            .add_step(steps::CreatePrStep::new(
                format!("Bump Zed to v{}", outputs.next_version),
                &outputs.pr_branch,
                &token,
            )),
    )
}

fn create_preview_branch(
    target: &WorkflowInput,
    tag_only: &WorkflowInput,
    versions_job: &steps::NamedJob,
    outputs: &ResolvedOutputs,
) -> steps::NamedJob {
    let not_tag_only = Expression::new(format!("{} != true", tag_only.expr()));
    let is_tag_only = Expression::new(format!("{} == true", tag_only.expr()));

    let (authenticate, token) = steps::authenticate_as_zippy().into();

    // Normal mode: checkout main, write RELEASE_CHANNEL, create branch, commit, tag commit
    let main_sha_step =
        named::bash("echo \"main_sha=$(git rev-parse HEAD)\" >> \"$GITHUB_OUTPUT\"")
            .id("main-sha")
            .if_condition(not_tag_only.clone());
    let main_sha = StepOutput::new(&main_sha_step, "main_sha");

    let write_channel = named::bash("echo -n preview > crates/zed/RELEASE_CHANNEL")
        .if_condition(not_tag_only.clone());

    let create_branch_step: Step<Use> =
        steps::CreateBranchStep::new(&outputs.preview_branch, &main_sha, &token).into();
    let create_branch_step = create_branch_step.if_condition(not_tag_only.clone());

    let commit_step: Step<Use> = steps::BotCommitStep::new(
        format!("{} preview", outputs.preview_branch),
        &outputs.preview_branch,
        &token,
    )
    .with_files("crates/zed/RELEASE_CHANNEL")
    .into();
    let commit_step = commit_step.if_condition(not_tag_only.clone());
    let commit_sha = StepOutput::new_unchecked(&commit_step, "commit");

    // Tag-only mode: determine tag from the checked-out preview branch
    let tag_only_head_step = named::bash(indoc::indoc! {r#"
            version=$(script/get-crate-version zed)
            channel=$(cat crates/zed/RELEASE_CHANNEL)
            tag_suffix=""
            case $channel in
              preview) tag_suffix="-pre" ;;
              stable) ;;
              *) echo "::error::unexpected channel $channel on preview branch"; exit 1 ;;
            esac
            echo "tag_name=v${version}${tag_suffix}" >> "$GITHUB_OUTPUT"
            echo "head_sha=$(git rev-parse HEAD)" >> "$GITHUB_OUTPUT"
        "#})
    .id("head-sha")
    .if_condition(is_tag_only);
    let tag_only_tag = StepOutput::new(&tag_only_head_step, "tag_name");
    let tag_only_sha = StepOutput::new(&tag_only_head_step, "head_sha");

    // Single tag step: use commit SHA in normal mode, HEAD SHA in tag-only mode
    let tag_step: Step<Use> = steps::CreateTagStep::new(
        format!(
            "${{{{ {} && '{}' || '{}' }}}}",
            tag_only.expr(),
            tag_only_tag,
            outputs.preview_tag,
        ),
        format!(
            "${{{{ {} && '{}' || '{}' }}}}",
            tag_only.expr(),
            tag_only_sha,
            commit_sha,
        ),
        &token,
    )
    .into();

    named::job(
        Job::default()
            .cond(Expression::new(format!(
                "{} == 'all' || {} == 'preview'",
                target.expr(),
                target.expr(),
            )))
            .needs(vec![versions_job.name.clone()])
            .runs_on(runners::LINUX_XL)
            .add_step(authenticate)
            // Checkout: main for normal mode, preview branch for tag_only
            .add_step(steps::checkout_repo().with_token(&token).with_ref(format!(
                "${{{{ {} && '{}' || 'main' }}}}",
                tag_only.expr(),
                outputs.preview_branch
            )))
            // Normal mode steps
            .add_step(main_sha_step)
            .add_step(write_channel)
            .add_step(create_branch_step)
            .add_step(commit_step)
            // Tag-only mode: determine tag from branch
            .add_step(tag_only_head_step)
            // Tag (uses conditional SHA and tag name)
            .add_step(tag_step),
    )
}

fn promote_to_stable(
    target: &WorkflowInput,
    tag_only: &WorkflowInput,
    versions_job: &steps::NamedJob,
    outputs: &ResolvedOutputs,
) -> steps::NamedJob {
    let not_tag_only = Expression::new(format!("{} != true", tag_only.expr()));

    let (authenticate, token) = steps::authenticate_as_zippy().into();

    // Shared: determine version and tag from the checked-out branch
    let read_version_step = named::bash(indoc::indoc! {r#"
            stable_version=$(script/get-crate-version zed)
            echo "stable_version=$stable_version" >> "$GITHUB_OUTPUT"
            echo "stable_tag=v${stable_version}" >> "$GITHUB_OUTPUT"
            echo "head_sha=$(git rev-parse HEAD)" >> "$GITHUB_OUTPUT"
        "#})
    .id("stable-info");
    let stable_tag = StepOutput::new(&read_version_step, "stable_tag");
    let head_sha = StepOutput::new(&read_version_step, "head_sha");

    // Normal mode: change RELEASE_CHANNEL, commit, tag the commit
    let write_channel = named::bash("echo -n stable > crates/zed/RELEASE_CHANNEL")
        .if_condition(not_tag_only.clone());

    let commit_step: Step<Use> = steps::BotCommitStep::new(
        format!("{} stable", outputs.stable_branch),
        &outputs.stable_branch,
        &token,
    )
    .with_files("crates/zed/RELEASE_CHANNEL")
    .into();
    let commit_step = commit_step.if_condition(not_tag_only.clone());
    let commit_sha = StepOutput::new_unchecked(&commit_step, "commit");

    // Single tag step: use commit SHA in normal mode, HEAD SHA in tag-only mode
    let tag_step: Step<Use> = steps::CreateTagStep::new(
        &stable_tag,
        format!(
            "${{{{ {} && '{}' || '{}' }}}}",
            tag_only.expr(),
            head_sha,
            commit_sha,
        ),
        &token,
    )
    .into();

    named::job(
        Job::default()
            .cond(Expression::new(format!(
                "{} == 'all' || {} == 'stable'",
                target.expr(),
                target.expr(),
            )))
            .needs(vec![versions_job.name.clone()])
            .runs_on(runners::LINUX_XL)
            .add_step(authenticate)
            .add_step(
                steps::checkout_repo()
                    .with_token(&token)
                    .with_ref(outputs.stable_branch.to_string()),
            )
            .add_step(read_version_step)
            // Normal mode steps
            .add_step(write_channel)
            .add_step(commit_step)
            // Tag (uses conditional SHA)
            .add_step(tag_step),
    )
}
