use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, FluentBuilder, NamedJob, named},
    vars::{self, StepOutput, WorkflowInput},
};

pub fn autofix_pr() -> Workflow {
    let pr_number = WorkflowInput::string("pr_number", None);
    let autofix = run_autofix(&pr_number);
    named::workflow()
        .run_name(format!("autofix PR #{pr_number}"))
        .on(Event::default().workflow_dispatch(
            WorkflowDispatch::default().add_input(pr_number.name, pr_number.input()),
        ))
        .add_job(autofix.name, autofix.job)
}

fn run_autofix(pr_number: &WorkflowInput) -> NamedJob {
    fn authenticate_as_zippy() -> (Step<Use>, StepOutput) {
        let step = named::uses(
            "actions",
            "create-github-app-token",
            "bef1eaf1c0ac2b148ee2a0a74c65fbe6db0631f1",
        )
        .add_with(("app-id", vars::ZED_ZIPPY_APP_ID))
        .add_with(("private-key", vars::ZED_ZIPPY_APP_PRIVATE_KEY))
        .id("get-app-token");
        let output = StepOutput::new(&step, "token");
        (step, output)
    }

    fn checkout_pr(pr_number: &WorkflowInput, token: &StepOutput) -> Step<Run> {
        named::bash(&format!("gh pr checkout {pr_number}")).add_env(("GITHUB_TOKEN", token))
    }

    fn run_cargo_fmt() -> Step<Run> {
        named::bash("cargo fmt --all")
    }

    fn run_clippy_fix() -> Step<Run> {
        named::bash(
            "cargo clippy --workspace --release --all-targets --all-features --fix --allow-dirty --allow-staged",
        )
    }

    fn run_prettier_fix() -> Step<Run> {
        named::bash("./script/prettier --write")
    }

    fn commit_and_push(token: &StepOutput) -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            if git diff --quiet; then
                echo "No changes to commit"
            else
                git add -A
                git commit -m "Autofix"
                git push
            fi
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

    let (authenticate, token) = authenticate_as_zippy();

    named::job(
        Job::default()
            .runs_on(runners::LINUX_DEFAULT)
            .add_step(authenticate)
            .add_step(steps::checkout_repo_with_token(&token))
            .add_step(checkout_pr(pr_number, &token))
            .add_step(steps::setup_cargo_config(runners::Platform::Linux))
            .add_step(steps::cache_rust_dependencies_namespace())
            .map(steps::install_linux_dependencies)
            .add_step(steps::setup_pnpm())
            .add_step(run_prettier_fix())
            .add_step(run_cargo_fmt())
            .add_step(run_clippy_fix())
            .add_step(commit_and_push(&token))
            .add_step(steps::cleanup_cargo_config(runners::Platform::Linux)),
    )
}
