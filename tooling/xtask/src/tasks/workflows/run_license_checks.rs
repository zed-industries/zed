use gh_workflow::{Job, Workflow};

use crate::tasks::workflows::{
    run_tests::run_tests_in,
    runners::{self, LINUX_MEDIUM},
    steps::{self, FluentBuilder, NamedJob, named},
};

pub fn run_license_checks() -> Workflow {
    let check_licenses = check_licenses();
    let build_licenses = build_licenses();

    named::workflow()
        .map(|workflow| {
            run_tests_in(
                &[
                    // no Cargo.toml - the case where Cargo.lock isn't updated
                    // is checked by the `check_dependencies` job as part of the
                    // `run_tests` workflow
                    "Cargo.lock",
                    "**/Cargo.lock",
                    "script/*licenses",
                    "**/LICENSE*",
                    ".github/workflows/run_license_checks.yml",
                ],
                workflow,
            )
        })
        .add_job(check_licenses.name, check_licenses.job)
        .add_job(build_licenses.name, build_licenses.job)
}

fn check_licenses() -> NamedJob {
    named::job(
        Job::default()
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo())
            .add_step(steps::script("./script/check-licenses")),
    )
}

fn build_licenses() -> NamedJob {
    named::job(
        Job::default()
            .runs_on(LINUX_MEDIUM)
            .add_step(steps::checkout_repo())
            .add_step(steps::script("./script/generate-licenses")),
    )
}
