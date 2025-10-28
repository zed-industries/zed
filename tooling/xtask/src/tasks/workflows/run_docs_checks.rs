use crate::tasks::workflows::{
    run_tests::run_tests_in,
    runners::{self, Platform},
    steps::{self, FluentBuilder, NamedJob, named, release_job},
};
use gh_workflow::*;

pub(crate) fn run_docs_checks() -> Workflow {
    let docs = check_docs();
    named::workflow()
        .map(|workflow| run_tests_in(&["docs/**", "crates/docs_preprocessor/**"], workflow))
        .add_job(docs.name, docs.job)
}

fn check_docs() -> NamedJob {
    named::job(
        release_job(&[])
            .runs_on(runners::LINUX_LARGE)
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_cargo_config(Platform::Linux))
            // todo(ci): un-inline build_docs/action.yml here
            .add_step(steps::cache_rust_dependencies())
            .add_step(lychee_link_check("./docs/src/**/*")) // check markdown links
            .map(steps::install_linux_dependencies)
            .add_step(install_mdbook())
            .add_step(build_docs())
            .add_step(lychee_link_check("target/deploy/docs")), // check links in generated html
    )
}

fn lychee_link_check(dir: &str) -> Step<Use> {
    named::uses(
        "lycheeverse",
        "lychee-action",
        "82202e5e9c2f4ef1a55a3d02563e1cb6041e5332",
    ) // v2.4.1
    .add_with(("args", format!("--no-progress --exclude '^http' '{dir}'")))
    .add_with(("fail", true))
}

fn install_mdbook() -> Step<Use> {
    named::uses(
        "peaceiris",
        "actions-mdbook",
        "ee69d230fe19748b7abf22df32acaa93833fad08", // v2
    )
    .with(("mdbook-version", "0.4.37"))
}

fn build_docs() -> Step<Run> {
    named::bash(indoc::indoc! {r#"
        mkdir -p target/deploy
        mdbook build ./docs --dest-dir=../target/deploy/docs/
    "#})
}
