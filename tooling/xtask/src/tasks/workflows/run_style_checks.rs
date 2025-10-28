use crate::tasks::workflows::{runners, steps::release_job};

use super::{
    run_tests::tests_workflow,
    steps::{self, NamedJob, named},
};
use gh_workflow::*;

pub(crate) fn run_style_checks() -> Workflow {
    let style = check_style();
    tests_workflow(&[]).add_job(style.name, style.job)
}

pub(crate) fn check_style() -> NamedJob {
    named::job(
        release_job(&[])
            .runs_on(runners::LINUX_MEDIUM)
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_pnpm())
            .add_step(prettier_check_docs())
            .add_step(prettier_check_default_json())
            .add_step(steps::script("./script/check-todos"))
            .add_step(steps::script("./script/check-keymaps"))
            .add_step(check_for_typos())
            .add_step(steps::cargo_fmt())
            .add_step(steps::script("./script/clippy")),
    )
}

fn prettier_check_docs() -> Step<Run> {
    named::bash(indoc::indoc! {r#"
            pnpm dlx "prettier@${PRETTIER_VERSION}" . --check || {
              echo "To fix, run from the root of the Zed repo:"
              echo "  cd docs && pnpm dlx prettier@${PRETTIER_VERSION} . --write && cd .."
              false
            }
        "#})
    .working_directory("./docs")
    .add_env(("PRETTIER_VERSION", "3.5.0"))
}

fn prettier_check_default_json() -> Step<Run> {
    named::bash(indoc::indoc! {r#"
            pnpm dlx "prettier@${PRETTIER_VERSION}" assets/settings/default.json --check || {
              echo "To fix, run from the root of the Zed repo:"
              echo "  pnpm dlx prettier@${PRETTIER_VERSION} assets/settings/default.json --write"
              false
            }
        "#})
    .add_env(("PRETTIER_VERSION", "3.5.0"))
}

fn check_for_typos() -> Step<Use> {
    named::uses(
        "crate-ci",
        "typos",
        "80c8a4945eec0f6d464eaf9e65ed98ef085283d1",
    ) // v1.38.1
    .with(("config", "./typos.toml"))
}
