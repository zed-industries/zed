use gh_workflow::*;
use indoc::indoc;

use crate::tasks::workflows::{
    extension_tests::{self},
    runners,
    steps::{self, CommonJobConditions, NamedJob, named},
    vars::{self, StepOutput, one_workflow_per_non_main_branch},
};

// This is used by various extensions repos in the zed-extensions org to bump extension versions.
pub(crate) fn extension_release() -> Workflow {
    let test_extension = extension_tests::check_extension();
    let bump_version = bump_extension_version(&[&test_extension]);

    named::workflow()
        .add_event(
            Event::default()
                .workflow_call(WorkflowCall::default())
                .workflow_dispatch(WorkflowDispatch::default()),
        )
        .concurrency(one_workflow_per_non_main_branch())
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("RUST_BACKTRACE", 1))
        .add_env(("CARGO_INCREMENTAL", 0))
        .add_env((
            "ZED_EXTENSION_CLI_SHA",
            extension_tests::ZED_EXTENSION_CLI_SHA,
        ))
        .add_job(test_extension.name, test_extension.job)
        .add_job(bump_version.name, bump_version.job)
}

fn bump_extension_version(dependencies: &[&NamedJob]) -> NamedJob {
    let (generate_token, generated_token) = generate_token();
    let (bump_version, new_version) = bump_version();

    let job = steps::dependant_job(dependencies)
        .with_repository_owner_guard()
        .runs_on(runners::LINUX_LARGE)
        .timeout_minutes(1u32)
        .add_step(generate_token)
        .add_step(steps::checkout_repo())
        .add_step(steps::cache_rust_dependencies_namespace())
        .add_step(install_bump_2_version())
        .add_step(bump_version)
        .add_step(create_pull_request(new_version, generated_token));

    named::job(job)
}

fn generate_token() -> (Step<Use>, StepOutput) {
    let step = named::uses("actions", "create-github-app-token", "v2")
        .id("generate-token")
        .add_with(
            Input::default()
                .add("app-id", vars::ZED_ZIPPY_APP_ID)
                .add("private-key", vars::ZED_ZIPPY_APP_PRIVATE_KEY),
        );

    let generated_token = StepOutput::new(&step, "token");

    (step, generated_token)
}

fn install_bump_2_version() -> Step<Run> {
    named::run(runners::Platform::Linux, "pip install bump2version")
}

fn bump_version() -> (Step<Run>, StepOutput) {
    let step = named::bash(indoc! {
        r#"
        OLD_VERSION="$(cat extension.toml| sed -n 's/version = \"\(.*\)\"/\1/p')"

        cat <<EOF > .bumpversion.cfg
        [bumpversion]
        current_version = $OLD_VERSION

        [bumpversion:file:Cargo.toml]

        [bumpversion:file:extension.toml]
        EOF

        bump2version --verbose minor
        NEW_VERSION="$(cat extension.toml| sed -n 's/version = \"\(.*\)\"/\1/p')"
        cargo b

        rm .bumpversion.cfg

        echo "new_version=${NEW_VERSION}" >> $GITHUB_OUTPUT
        "#
    })
    .id("bump-version");

    let step_output = StepOutput::new(&step, "new_version");
    (step, step_output)
}

fn create_pull_request(new_version: StepOutput, generated_token: StepOutput) -> Step<Use> {
    let formatted_version = format!("v{}", new_version);

    named::uses("peter-evans", "create-pull-request", "v6").with(
        Input::default()
            .add("title", format!("Bump to {}", formatted_version))
            .add(
                "body",
                format!(
                    "This PR bumps the version of this extension to {}",
                    formatted_version
                ),
            )
            .add(
                "commit-message",
                format!("Bump version to {}", formatted_version),
            )
            .add("branch", formatted_version)
            .add("base", "main")
            .add("delete-branch", true)
            .add("token", generated_token.to_string())
            .add("signoff", true),
    )
}
