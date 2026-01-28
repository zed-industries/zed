use gh_workflow::*;

use crate::tasks::workflows::{
    runners,
    steps::{self, CommonJobConditions, NamedJob, named},
    vars,
};

pub fn publish_extension_cli() -> Workflow {
    let publish = publish_job();

    named::workflow()
        .on(Event::default().push(Push::default().tags(vec!["extension-cli".to_string()])))
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("CARGO_INCREMENTAL", 0))
        .add_job(publish.name, publish.job)
}

fn publish_job() -> NamedJob {
    fn build_extension_cli() -> Step<Run> {
        named::bash("cargo build --release --package extension_cli")
    }

    fn upload_binary() -> Step<Run> {
        named::bash("script/upload-extension-cli ${{ github.sha }}")
            .add_env((
                "DIGITALOCEAN_SPACES_ACCESS_KEY",
                vars::DIGITALOCEAN_SPACES_ACCESS_KEY,
            ))
            .add_env((
                "DIGITALOCEAN_SPACES_SECRET_KEY",
                vars::DIGITALOCEAN_SPACES_SECRET_KEY,
            ))
    }

    named::job(
        Job::default()
            .with_repository_owner_guard()
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo())
            .add_step(steps::cache_rust_dependencies_namespace())
            .add_step(steps::setup_linux())
            .add_step(build_extension_cli())
            .add_step(upload_binary()),
    )
}
