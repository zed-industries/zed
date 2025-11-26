use gh_workflow::*;
use indoc::indoc;

use crate::tasks::workflows::{
    extension_tests::{self},
    runners,
    steps::{self, CommonJobConditions, DEFAULT_REPOSITORY_OWNER_GUARD, NamedJob, named},
    vars::{
        JobOutput, StepOutput, WorkflowInput, WorkflowSecret, one_workflow_per_non_main_branch,
    },
};

const BUMPVERSION_CONFIG: &str = indoc! {r#"
    [bumpversion]
    current_version = "$OLD_VERSION"

    [bumpversion:file:Cargo.toml]

    [bumpversion:file:extension.toml]
    "#
};

const VERSION_CHECK: &str = r#"sed -n 's/version = \"\(.*\)\"/\1/p' < extension.toml"#;

// This is used by various extensions repos in the zed-extensions org to bump extension versions.
pub(crate) fn extension_bump() -> Workflow {
    let bump_type = WorkflowInput::string("bump-type", Some("patch".to_owned()));

    let app_id = WorkflowSecret::new("app-id", "The app ID used to create the PR");
    let app_secret =
        WorkflowSecret::new("app-secret", "The app secret for the corresponding app ID");

    let test_extension = extension_tests::check_extension();
    let (check_bump_needed, needs_bump) = check_bump_needed();
    let bump_version = bump_extension_version(
        &[&test_extension, &check_bump_needed],
        &bump_type,
        needs_bump.as_job_output(&check_bump_needed),
        &app_id,
        &app_secret,
    );

    named::workflow()
        .add_event(
            Event::default().workflow_call(
                WorkflowCall::default()
                    .add_input(bump_type.name, bump_type.call_input())
                    .secrets([
                        (app_id.name.to_owned(), app_id.secret_configuration()),
                        (
                            app_secret.name.to_owned(),
                            app_secret.secret_configuration(),
                        ),
                    ]),
            ),
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
        .add_job(check_bump_needed.name, check_bump_needed.job)
        .add_job(bump_version.name, bump_version.job)
}

fn check_bump_needed() -> (NamedJob, StepOutput) {
    let (compare_versions, version_changed) = compare_versions();

    let job = Job::default()
        .with_repository_owner_guard()
        .outputs([(version_changed.name.to_owned(), version_changed.to_string())])
        .runs_on(runners::LINUX_SMALL)
        .timeout_minutes(1u32)
        .add_step(steps::checkout_repo().add_with(("fetch-depth", 10)))
        .add_step(compare_versions);

    (named::job(job), version_changed)
}

/// Compares the current and previous commit and checks whether versions changed inbetween.
fn compare_versions() -> (Step<Run>, StepOutput) {
    let check_needs_bump = named::bash(format!(
        indoc! {
            r#"
        CURRENT_VERSION="$({})"

        git checkout "$(git log -1 --format=%H)"~1

        PREV_COMMIT_VERSION="$({})"

        [[ "$CURRENT_VERSION" == "$PREV_COMMIT_VERSION" ]] && \
          echo "needs_bump=true" >> "$GITHUB_OUTPUT" || \
          echo "needs_bump=false" >> "$GITHUB_OUTPUT"

        "#
        },
        VERSION_CHECK, VERSION_CHECK
    ))
    .id("compare-versions-check");

    let needs_bump = StepOutput::new(&check_needs_bump, "needs_bump");

    (check_needs_bump, needs_bump)
}

fn bump_extension_version(
    dependencies: &[&NamedJob],
    bump_type: &WorkflowInput,
    needs_bump: JobOutput,
    app_id: &WorkflowSecret,
    app_secret: &WorkflowSecret,
) -> NamedJob {
    let (generate_token, generated_token) = generate_token(app_id, app_secret);
    let (bump_version, old_version, new_version) = bump_version(bump_type);

    let job = steps::dependant_job(dependencies)
        .cond(Expression::new(format!(
            "{DEFAULT_REPOSITORY_OWNER_GUARD} && {} == 'true'",
            needs_bump.expr(),
        )))
        .runs_on(runners::LINUX_LARGE)
        .timeout_minutes(1u32)
        .add_step(generate_token)
        .add_step(steps::checkout_repo())
        .add_step(install_bump_2_version())
        .add_step(bump_version)
        .add_step(create_pull_request(
            old_version,
            new_version,
            generated_token,
        ));

    named::job(job)
}

fn generate_token(app_id: &WorkflowSecret, app_secret: &WorkflowSecret) -> (Step<Use>, StepOutput) {
    let step = named::uses("actions", "create-github-app-token", "v2")
        .id("generate-token")
        .add_with(
            Input::default()
                .add("app-id", app_id.to_string())
                .add("private-key", app_secret.to_string()),
        );

    let generated_token = StepOutput::new(&step, "token");

    (step, generated_token)
}

fn install_bump_2_version() -> Step<Run> {
    named::run(runners::Platform::Linux, "pip install bump2version")
}

fn bump_version(bump_type: &WorkflowInput) -> (Step<Run>, StepOutput, StepOutput) {
    let step = named::bash(format!(
        indoc! {r#"
            OLD_VERSION="$({})"

            cat <<EOF > .bumpversion.cfg
            {}
            EOF

            bump2version --verbose {}
            NEW_VERSION="$({})"
            cargo update --workspace

            rm .bumpversion.cfg

            echo "old_version=${{OLD_VERSION}}" >> "$GITHUB_OUTPUT"
            echo "new_version=${{NEW_VERSION}}" >> "$GITHUB_OUTPUT"
            "#
        },
        VERSION_CHECK, BUMPVERSION_CONFIG, bump_type, VERSION_CHECK
    ))
    .id("bump-version");

    let old_version = StepOutput::new(&step, "old_version");
    let new_version = StepOutput::new(&step, "new_version");
    (step, old_version, new_version)
}

fn create_pull_request(
    old_version: StepOutput,
    new_version: StepOutput,
    generated_token: StepOutput,
) -> Step<Use> {
    let formatted_version = format!("v{}", new_version);

    named::uses("peter-evans", "create-pull-request", "v7").with(
        Input::default()
            .add("title", format!("Bump version to {}", new_version))
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
            .add("branch", format!("bump-from-{}", old_version))
            .add(
                "committer",
                "zed-zippy[bot] <234243425+zed-zippy[bot]@users.noreply.github.com>",
            )
            .add("base", "main")
            .add("delete-branch", true)
            .add("token", generated_token.to_string())
            .add("sign-commits", true),
    )
}
