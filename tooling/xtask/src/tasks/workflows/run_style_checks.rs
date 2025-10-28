use crate::tasks::workflows::{runners, steps::release_job};

use super::{
    run_tests::run_tests_in,
    steps::{self, FluentBuilder as _, NamedJob, named},
};
use gh_workflow::*;

pub(crate) fn run_style_checks() -> Workflow {
    let style = check_style();
    named::workflow()
        .map(|workflow| run_tests_in(&[], workflow))
        .add_job(style.name, style.job)
}

pub(crate) fn check_style() -> NamedJob {
    named::job(
        release_job(&[])
            .runs_on(runners::LINUX_MEDIUM)
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_pnpm())
            .add_step(steps::script("./script/prettier"))
            .add_step(steps::script("./script/check-todos"))
            .add_step(steps::script("./script/check-keymaps"))
            .add_step(check_for_typos())
            .add_step(steps::cargo_fmt()),
    )
}

fn check_for_typos() -> Step<Use> {
    named::uses(
        "crate-ci",
        "typos",
        "80c8a4945eec0f6d464eaf9e65ed98ef085283d1",
    ) // v1.38.1
    .with(("config", "./typos.toml"))
}
