use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, FluentBuilder, NamedJob, named},
    vars::{self, StepOutput, WorkflowInput},
};

pub fn autofix_pr() -> Workflow {
    let pr_number = WorkflowInput::string("pr_number", None);
    let run_clippy = WorkflowInput::bool("run_clippy", Some(true));
    let run_autofix = run_autofix(&pr_number, &run_clippy);
    let commit_changes = commit_changes(&pr_number, &run_autofix);
    named::workflow()
        .run_name(format!("autofix PR #{pr_number}"))
        .on(Event::default().workflow_dispatch(
            WorkflowDispatch::default()
                .add_input(pr_number.name, pr_number.input())
                .add_input(run_clippy.name, run_clippy.input()),
        ))
        .concurrency(
            Concurrency::new(Expression::new(format!(
                "${{{{ github.workflow }}}}-{pr_number}"
            )))
            .cancel_in_progress(true),
        )
        .add_job(run_autofix.name.clone(), run_autofix.job)
        .add_job(commit_changes.name, commit_changes.job)
}

const PATCH_ARTIFACT_NAME: &str = "autofix-patch";
const PATCH_FILE_PATH: &str = "autofix.patch";

fn upload_patch_artifact() -> Step<Use> {
    Step::new(format!("upload artifact {}", PATCH_ARTIFACT_NAME))
        .uses(
            "actions",
            "upload-artifact",
            "330a01c490aca151604b8cf639adc76d48f6c5d4", // v5
        )
        .add_with(("name", PATCH_ARTIFACT_NAME))
        .add_with(("path", PATCH_FILE_PATH))
        .add_with(("if-no-files-found", "ignore"))
        .add_with(("retention-days", "1"))
}

fn download_patch_artifact() -> Step<Use> {
    named::uses(
        "actions",
        "download-artifact",
        "018cc2cf5baa6db3ef3c5f8a56943fffe632ef53", // v6.0.0
    )
    .add_with(("name", PATCH_ARTIFACT_NAME))
}

fn run_autofix(pr_number: &WorkflowInput, run_clippy: &WorkflowInput) -> NamedJob {
    fn checkout_pr(pr_number: &WorkflowInput) -> Step<Run> {
        named::bash(&format!("gh pr checkout {pr_number}"))
            .add_env(("GITHUB_TOKEN", vars::GITHUB_TOKEN))
    }

    fn install_cargo_machete() -> Step<Use> {
        named::uses(
            "clechasseur",
            "rs-cargo",
            "8435b10f6e71c2e3d4d3b7573003a8ce4bfc6386", // v2
        )
        .add_with(("command", "install"))
        .add_with(("args", "cargo-machete@0.7.0"))
    }

    fn run_cargo_fmt() -> Step<Run> {
        named::bash("cargo fmt --all")
    }

    fn run_cargo_fix() -> Step<Run> {
        named::bash(
            "cargo fix --workspace --release --all-targets --all-features --allow-dirty --allow-staged",
        )
    }

    fn run_cargo_machete_fix() -> Step<Run> {
        named::bash("cargo machete --fix")
    }

    fn run_clippy_fix() -> Step<Run> {
        named::bash(
            "cargo clippy --workspace --release --all-targets --all-features --fix --allow-dirty --allow-staged",
        )
    }

    fn run_prettier_fix() -> Step<Run> {
        named::bash("./script/prettier --write")
    }

    fn create_patch() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            if git diff --quiet; then
                echo "No changes to commit"
                echo "has_changes=false" >> "$GITHUB_OUTPUT"
            else
                git diff > autofix.patch
                echo "has_changes=true" >> "$GITHUB_OUTPUT"
            fi
        "#})
        .id("create-patch")
    }

    named::job(
        Job::default()
            .runs_on(runners::LINUX_DEFAULT)
            .outputs([(
                "has_changes".to_owned(),
                "${{ steps.create-patch.outputs.has_changes }}".to_owned(),
            )])
            .add_step(steps::checkout_repo())
            .add_step(checkout_pr(pr_number))
            .add_step(steps::setup_cargo_config(runners::Platform::Linux))
            .add_step(steps::cache_rust_dependencies_namespace())
            .map(steps::install_linux_dependencies)
            .add_step(steps::setup_pnpm())
            .add_step(install_cargo_machete().if_condition(Expression::new(run_clippy.to_string())))
            .add_step(run_cargo_fix().if_condition(Expression::new(run_clippy.to_string())))
            .add_step(run_cargo_machete_fix().if_condition(Expression::new(run_clippy.to_string())))
            .add_step(run_clippy_fix().if_condition(Expression::new(run_clippy.to_string())))
            .add_step(run_prettier_fix())
            .add_step(run_cargo_fmt())
            .add_step(create_patch())
            .add_step(upload_patch_artifact())
            .add_step(steps::cleanup_cargo_config(runners::Platform::Linux)),
    )
}

fn commit_changes(pr_number: &WorkflowInput, autofix_job: &NamedJob) -> NamedJob {
    fn checkout_pr(pr_number: &WorkflowInput, token: &StepOutput) -> Step<Run> {
        named::bash(&format!("gh pr checkout {pr_number}")).add_env(("GITHUB_TOKEN", token))
    }

    fn apply_patch() -> Step<Run> {
        named::bash("git apply autofix.patch")
    }

    fn commit_and_push(token: &StepOutput) -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            git commit -am "Autofix"
            git push
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
            .runs_on(runners::LINUX_SMALL)
            .needs(vec![autofix_job.name.clone()])
            .cond(Expression::new(format!(
                "needs.{}.outputs.has_changes == 'true'",
                autofix_job.name
            )))
            .add_step(authenticate)
            .add_step(steps::checkout_repo_with_token(&token))
            .add_step(checkout_pr(pr_number, &token))
            .add_step(download_patch_artifact())
            .add_step(apply_patch())
            .add_step(commit_and_push(&token)),
    )
}
