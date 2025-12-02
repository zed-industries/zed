use gh_workflow::{Event, Job, Run, Step, Use, Workflow, WorkflowCall};
use indoc::indoc;

use crate::tasks::workflows::{
    extension_bump::{RepositoryTarget, generate_token},
    runners,
    steps::{CommonJobConditions, NamedJob, checkout_repo, named},
    vars::{StepOutput, WorkflowSecret},
};

pub(crate) fn extension_release() -> Workflow {
    let (app_id, app_secret) = extension_workflow_secrets();

    let create_release = create_release(&app_id, &app_secret);
    named::workflow()
        .on(
            Event::default().workflow_call(WorkflowCall::default().secrets([
                (app_id.name.to_owned(), app_id.secret_configuration()),
                (
                    app_secret.name.to_owned(),
                    app_secret.secret_configuration(),
                ),
            ])),
        )
        .add_job(create_release.name, create_release.job)
}

fn create_release(app_id: &WorkflowSecret, app_secret: &WorkflowSecret) -> NamedJob {
    let extension_registry = RepositoryTarget::new("zed-industries", &["extensions"]);
    let (generate_token, generated_token) =
        generate_token(&app_id, &app_secret, Some(extension_registry));
    let (get_extension_id, extension_id) = get_extension_id();

    let job = Job::default()
        .with_repository_owner_guard()
        .runs_on(runners::LINUX_LARGE)
        .add_step(generate_token)
        .add_step(checkout_repo())
        .add_step(get_extension_id)
        .add_step(release_action(extension_id, generated_token));

    named::job(job)
}

fn get_extension_id() -> (Step<Run>, StepOutput) {
    let step = named::bash(indoc! {
    r#"
        EXTENSION_ID="$(sed -n 's/id = \"\(.*\)\"/\1/p' < extension.toml)"

        echo "extension_id=${EXTENSION_ID}" >> "$GITHUB_OUTPUT"
    "#})
    .id("get-extension-id");

    let extension_id = StepOutput::new(&step, "extension_id");

    (step, extension_id)
}

fn release_action(extension_id: StepOutput, generated_token: StepOutput) -> Step<Use> {
    named::uses("huacnlee", "zed-extension-action", "v2")
        .add_with(("extension-name", extension_id.to_string()))
        .add_with(("push-to", "zed-industries/extensions"))
        .add_env(("COMMITTER_TOKEN", generated_token.to_string()))
}

pub(crate) fn extension_workflow_secrets() -> (WorkflowSecret, WorkflowSecret) {
    let app_id = WorkflowSecret::new("app-id", "The app ID used to create the PR");
    let app_secret =
        WorkflowSecret::new("app-secret", "The app secret for the corresponding app ID");

    (app_id, app_secret)
}
