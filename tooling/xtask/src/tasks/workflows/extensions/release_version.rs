use gh_workflow::{Event, Job, Level, Permissions, Push, UsesJob, Workflow};

use crate::tasks::workflows::{
    extensions::WithAppSecrets,
    steps::{NamedJob, named},
};

pub(crate) fn release_version() -> Workflow {
    let create_release = call_release_version();
    named::workflow()
        .on(Event::default().push(Push::default().add_tag("v**")))
        .add_job(create_release.name, create_release.job)
}

pub(crate) fn call_release_version() -> NamedJob<UsesJob> {
    let job = Job::default()
        .permissions(
            Permissions::default()
                .contents(Level::Write)
                .pull_requests(Level::Write),
        )
        .uses(
            "zed-industries",
            "zed",
            ".github/workflows/extension_release.yml",
            "main",
        )
        .with_app_secrets();

    named::job(job)
}
