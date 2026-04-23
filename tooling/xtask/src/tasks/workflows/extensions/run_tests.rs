use gh_workflow::{Event, Job, Level, Permissions, PullRequest, Push, UsesJob, Workflow};

use crate::tasks::workflows::{
    GenerateWorkflowArgs, GitSha,
    steps::{NamedJob, named},
    vars::one_workflow_per_non_main_branch_and_token,
};

pub(crate) fn run_tests(args: &GenerateWorkflowArgs) -> Workflow {
    let call_extension_tests = call_extension_tests(args.sha.as_ref());
    named::workflow()
        .on(Event::default()
            .pull_request(PullRequest::default().add_branch("**"))
            .push(Push::default().add_branch("main")))
        .concurrency(one_workflow_per_non_main_branch_and_token("pr"))
        .add_job(call_extension_tests.name, call_extension_tests.job)
}

pub(crate) fn call_extension_tests(target_ref: Option<&GitSha>) -> NamedJob<UsesJob> {
    let job = Job::default()
        .permissions(Permissions::default().contents(Level::Read))
        .uses(
            "zed-industries",
            "zed",
            ".github/workflows/extension_tests.yml",
            target_ref.map_or("main", AsRef::as_ref),
        );

    named::job(job)
}
