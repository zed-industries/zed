use gh_workflow::{Event, Job, PullRequest, Push, UsesJob, Workflow};

use crate::tasks::workflows::{
    steps::{NamedJob, named},
    vars::one_workflow_per_non_main_branch_and_token,
};

pub(crate) fn run_tests() -> Workflow {
    let call_extension_tests = call_extension_tests();
    named::workflow()
        .on(Event::default()
            .pull_request(PullRequest::default().add_branch("**"))
            .push(Push::default().add_branch("main")))
        .concurrency(one_workflow_per_non_main_branch_and_token("pr"))
        .add_job(call_extension_tests.name, call_extension_tests.job)
}

pub(crate) fn call_extension_tests() -> NamedJob<UsesJob> {
    let job = Job::default().uses(
        "zed-industries",
        "zed",
        ".github/workflows/extension_tests.yml",
        "main",
    );

    named::job(job)
}
