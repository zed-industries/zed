use gh_workflow::*;
use indoc::indoc;
use serde_json::json;

use super::{
    runners, steps,
    steps::{
        BotCommitStep, CommonPermissionSets, NamedJob, RepositoryTarget, TokenPermissions, named,
    },
    vars::{StepOutput, WorkflowInput},
};

const CRATES_IO_TOKEN: &str = "${{ secrets.GPUI_CRATES_IO_TOKEN }}";
const REPOSITORY: &str = "${{ matrix.release.repository }}";
const REVISION: &str = "${{ matrix.release.revision }}";
const REPOSITORY_NAME: &str = "${{ matrix.release.repo }}";
const CHECKOUT: &str = "source";
const TAG: &str = "${{ matrix.release.tag }}";

pub fn publish_gpui() -> Workflow {
    let bump = WorkflowInput::string("bump", Some("patch".to_owned()))
        .description("Version component to bump: major, minor, or patch");
    let plan = plan(&bump);
    let prepare = prepare(&plan);
    let publish = publish(&plan, &prepare);
    named::workflow()
        .with_minimal_permissions()
        .on(Event::default()
            .workflow_dispatch(WorkflowDispatch::default().add_input(bump.name, bump.input())))
        .concurrency(Concurrency::new(Expression::new("publish-gpui")))
        .add_job(plan.name, plan.job)
        .add_job(prepare.name, prepare.job)
        .add_job(publish.name, publish.job)
}

fn plan(bump: &WorkflowInput) -> NamedJob {
    let plan = named::bash(indoc! {r#"
        cargo xtask gpui plan --sha "$GITHUB_SHA" --bump "$BUMP" \
            --workflow-output "$GITHUB_OUTPUT"
        sed -n 's/^release_plan=//p' "$GITHUB_OUTPUT" > release.json
        test -s release.json
    "#})
    .id("plan")
    .add_env(("BUMP", bump.to_string()))
    .add_env(("CARGO_REGISTRY_TOKEN", CRATES_IO_TOKEN));
    let repositories = StepOutput::new_unchecked(&plan, "repositories");
    let crates = StepOutput::new_unchecked(&plan, "crates");
    named::job(
        Job::default()
            .cond(Expression::new(
                "github.repository == 'zed-industries/zed' && github.ref == 'refs/heads/main'",
            ))
            .runs_on(runners::LINUX_DEFAULT)
            .outputs([
                ("repositories".to_owned(), repositories.to_string()),
                ("crates".to_owned(), crates.to_string()),
            ])
            .add_step(steps::checkout_repo())
            .add_step(steps::cache_rust_dependencies_namespace())
            .add_step(plan)
            .add_step(steps::upload_artifact("gpui-release", "release.json")),
    )
}

fn release_matrix(plan: &NamedJob, output: &str) -> Strategy {
    Strategy::default().matrix(json!({
        "release": format!("${{{{ fromJSON(needs.{}.outputs.{output}) }}}}", plan.name)
    }))
}

fn prepare(plan: &NamedJob) -> NamedJob {
    let (authenticate, token) = authenticate_repository(Level::Read);
    let script = named::bash(indoc! {r#"
        mkdir prepared
        refs=$(git -C source ls-remote origin refs/heads/GPUI "refs/tags/$TAG")
        if grep -Fq "refs/tags/$TAG" <<< "$refs"; then
            echo "Tag $TAG already exists in $REPOSITORY" >&2
            exit 1
        fi
        awk '$2 == "refs/heads/GPUI" { print $1 }' <<< "$refs" > prepared/previous
        cargo xtask gpui prepare --plan release.json --repository "$REPOSITORY" --checkout source
        git -C source diff --binary HEAD > prepared/release.patch
        test -s prepared/release.patch
    "#})
    .add_env(("TAG", TAG))
    .add_env(("REPOSITORY", REPOSITORY));
    named::job(
        Job::default()
            .needs([plan.name.clone()])
            .runs_on(runners::LINUX_DEFAULT)
            .strategy(release_matrix(plan, "repositories"))
            .add_step(steps::checkout_repo())
            .add_step(authenticate)
            .add_step(
                steps::checkout_repo()
                    .with_repository(REPOSITORY)
                    .with_ref(REVISION)
                    .with_path(CHECKOUT)
                    .with_token(&token),
            )
            .add_step(steps::cache_rust_dependencies_namespace())
            .add_step(steps::download_artifact().artifact_name("gpui-release"))
            .add_step(script)
            .add_step(steps::upload_artifact(prepared_artifact(), "prepared/")),
    )
}

fn prepared_artifact() -> String {
    format!("gpui-prepared-{REPOSITORY_NAME}")
}

fn publish(plan: &NamedJob, prepare: &NamedJob) -> NamedJob {
    let prepare_ref = Expression::new("matrix.release.prepare_ref");
    let (authenticate, token) = authenticate_repository(Level::Write);
    let checkout: Step<Use> = steps::checkout_repo()
        .with_repository(REPOSITORY)
        .with_path(CHECKOUT)
        .with_ref("${{ matrix.release.prepare_ref && matrix.release.revision || format('refs/tags/{0}', matrix.release.tag) }}")
        .into();
    let patch: Step<Use> = steps::download_artifact()
        .artifact_name(prepared_artifact())
        .path("prepared")
        .into();
    let trust = Expression::new("!matrix.release.first_publish");
    let auth = named::uses(
        "rust-lang",
        "crates-io-auth-action",
        "3268eb413133e2960fa4ae0e963ca76380c54cd9", // v1.0.6
    )
    .id("crates-io")
    .if_condition(trust);
    let registry_token = StepOutput::new(&auth, "token");
    let commit: Step<Use> =
        BotCommitStep::new(format!("Prepare {TAG} release"), "GPUI", &token).into();
    let commit = commit
        .add_with(("repository", REPOSITORY))
        .add_with((
            "workspace",
            format!("${{{{ github.workspace }}}}/{CHECKOUT}"),
        ))
        .add_with(("auto-stage", false))
        .add_with(("update-local", false))
        .add_with(("if-no-commit", "error"));
    let commit_sha = StepOutput::new(&commit, "commit");
    named::job(
        Job::default()
            .needs([plan.name.clone(), prepare.name.clone()])
            .runs_on("${{ matrix.release.runner }}")
            .strategy(
                release_matrix(plan, "crates")
                    .max_parallel(1u32)
                    .fail_fast(true),
            )
            .permissions(
                Permissions::default()
                    .contents(Level::Read)
                    .id_token(Level::Write),
            )
            .add_step(steps::checkout_repo().with_path("zed"))
            .add_step(authenticate.if_condition(prepare_ref.clone()))
            .add_step(checkout.add_with((
                "token",
                format!(
                    "${{{{ matrix.release.prepare_ref && {} || github.token }}}}",
                    token.expr()
                ),
            )))
            .add_step(
                steps::setup_linux()
                    .working_directory("zed")
                    .if_condition(Expression::new("runner.os == 'Linux'")),
            )
            .add_step(setup_toolchain())
            .add_step(patch.if_condition(prepare_ref.clone()))
            .add_step(update_refs().if_condition(prepare_ref.clone()))
            .add_step(commit.if_condition(prepare_ref.clone()))
            .add_step(
                steps::create_ref_in_repository(
                    REPOSITORY,
                    steps::GitRef::tag(TAG),
                    &commit_sha,
                    &token,
                )
                .if_condition(prepare_ref.clone()),
            )
            .add_step(checkout_commit(&commit_sha).if_condition(prepare_ref))
            .add_step(auth)
            .add_step(publish_crate(&registry_token)),
    )
}

fn authenticate_repository(level: Level) -> (Step<Use>, StepOutput) {
    let write = level == Level::Write;
    let mut permissions = vec![(TokenPermissions::Contents, level)];
    if write {
        permissions.push((TokenPermissions::Workflows, Level::Write));
    }
    steps::authenticate_as_zippy()
        .for_repository(RepositoryTarget::new("zed-industries", &[REPOSITORY_NAME]))
        .with_permissions(permissions)
        .into()
}

fn setup_toolchain() -> Step<Run> {
    named::bash(indoc! {r#"
        cargo --version
        toolchain=$(rustup show active-toolchain)
        echo "RUSTUP_TOOLCHAIN=${toolchain%% *}" >> "$GITHUB_ENV"
    "#})
    .working_directory("zed")
}

fn update_refs() -> Step<Run> {
    // Cross-repository dependencies must be on crates.io before Cargo can resolve
    // the lockfile, so finalize each repo just before publishing its first crate.
    named::bash(indoc! {r#"
        previous=$(cat ../prepared/previous)
        git switch -c GPUI
        git apply --index ../prepared/release.patch
        cargo update --workspace
        git add -u
        git add -f Cargo.lock
        git push --force-with-lease="refs/heads/GPUI:$previous" origin HEAD:refs/heads/GPUI
    "#})
    .working_directory(CHECKOUT)
}

fn checkout_commit(commit: &StepOutput) -> Step<Run> {
    named::bash(indoc! {r#"
        git fetch --depth 1 origin "$COMMIT"
        git reset --hard "$COMMIT"
    "#})
    .working_directory(CHECKOUT)
    .add_env(("COMMIT", commit.to_string()))
}

fn publish_crate(token: &StepOutput) -> Step<Run> {
    named::bash(r#"cargo publish --manifest-path "$MANIFEST" -p "$CRATE_NAME""#)
        .working_directory(CHECKOUT)
        .add_env(("MANIFEST", "${{ matrix.release.manifest }}"))
        .add_env(("CRATE_NAME", "${{ matrix.release.name }}"))
        .add_env((
            "CARGO_REGISTRY_TOKEN",
            format!(
                "${{{{ matrix.release.first_publish && secrets.GPUI_CRATES_IO_TOKEN || {} }}}}",
                token.expr()
            ),
        ))
}
