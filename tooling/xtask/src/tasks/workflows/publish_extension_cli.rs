use gh_workflow::*;
use indoc::{formatdoc, indoc};

use crate::tasks::workflows::{
    runners,
    steps::{
        self, DEFAULT_REPOSITORY_OWNER_GUARD, GitRef, NamedJob, RefSha, RepositoryTarget,
        TokenPermissions, generate_token, named,
    },
    vars::{self, StepOutput, WorkflowInput},
};

const EXTENSION_CLI_TAG: &str = "extension-cli";

pub fn publish_extension_cli() -> Workflow {
    let message = WorkflowInput::string("message", None).description(
        "Describe why the extension CLI is being bumped and/or what changes are included.",
    );

    let publish = publish_job();
    let update_sha_in_zed = update_sha_in_zed(&publish, &message);
    let update_sha_in_extensions = update_sha_in_extensions(&publish, &message);

    named::workflow()
        .on(Event::default().workflow_dispatch(
            WorkflowDispatch::default().add_input(message.name, message.input()),
        ))
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("CARGO_INCREMENTAL", 0))
        .add_job(publish.name, publish.job)
        .add_job(update_sha_in_zed.name, update_sha_in_zed.job)
        .add_job(update_sha_in_extensions.name, update_sha_in_extensions.job)
}

// `workflow_dispatch` can be triggered from any branch where this workflow file
// exists, so we additionally guard the jobs to only run when dispatched from
// `main`. Jobs that depend on `publish_job` inherit this guard transitively
// because they are skipped when `publish_job` is skipped.
fn dispatched_from_main_guard() -> Expression {
    Expression::new(format!(
        "{DEFAULT_REPOSITORY_OWNER_GUARD} && github.ref == 'refs/heads/main'"
    ))
}

fn publish_job() -> NamedJob {
    fn build_extension_cli() -> Step<Run> {
        named::bash("cargo build --release --package extension_cli")
    }

    fn upload_binary() -> Step<Run> {
        named::bash(r#"script/upload-extension-cli "$GITHUB_SHA""#)
            .add_env((
                "DIGITALOCEAN_SPACES_ACCESS_KEY",
                vars::DIGITALOCEAN_SPACES_ACCESS_KEY,
            ))
            .add_env((
                "DIGITALOCEAN_SPACES_SECRET_KEY",
                vars::DIGITALOCEAN_SPACES_SECRET_KEY,
            ))
    }

    let (authenticate, token) = steps::authenticate_as_zippy()
        .for_repository(RepositoryTarget::current())
        .with_permissions([(TokenPermissions::Contents, Level::Write)])
        .into();

    named::job(
        Job::default()
            .cond(dispatched_from_main_guard())
            .runs_on(runners::LINUX_DEFAULT)
            .add_step(steps::checkout_repo())
            .add_step(steps::cache_rust_dependencies_namespace())
            .add_step(steps::setup_linux())
            .add_step(build_extension_cli())
            .add_step(upload_binary())
            .add_step(authenticate)
            .add_step(steps::update_ref(
                GitRef::tag(EXTENSION_CLI_TAG),
                RefSha::Context,
                &token,
                true,
            )),
    )
}

fn update_sha_in_zed(publish_job: &NamedJob, message: &WorkflowInput) -> NamedJob {
    let (generate_token, generated_token) =
        generate_token(vars::ZED_ZIPPY_APP_ID, vars::ZED_ZIPPY_APP_PRIVATE_KEY).into();

    fn replace_sha() -> Step<Run> {
        named::bash(indoc! {r#"
            sed -i "s/ZED_EXTENSION_CLI_SHA: &str = \"[a-f0-9]*\"/ZED_EXTENSION_CLI_SHA: \&str = \"$GITHUB_SHA\"/" \
                tooling/xtask/src/tasks/workflows/extension_tests.rs
        "#})
    }

    fn regenerate_workflows() -> Step<Run> {
        named::bash("cargo xtask workflows")
    }

    let (get_short_sha_step, short_sha) = get_short_sha();

    named::job(
        Job::default()
            .cond(dispatched_from_main_guard())
            .needs(vec![publish_job.name.clone()])
            .runs_on(runners::LINUX_LARGE)
            .add_step(generate_token)
            .add_step(steps::checkout_repo())
            .add_step(steps::cache_rust_dependencies_namespace())
            .add_step(get_short_sha_step)
            .add_step(replace_sha())
            .add_step(regenerate_workflows())
            .add_step(create_pull_request_zed(
                &generated_token,
                &short_sha,
                message,
            )),
    )
}

fn create_pull_request_zed(
    generated_token: &StepOutput,
    short_sha: &StepOutput,
    message: &WorkflowInput,
) -> Step<Use> {
    let title = format!(
        "extension_ci: Bump extension CLI version to `{}`",
        short_sha
    );

    let body = formatdoc! {r#"
        This PR bumps the extension CLI version used in the extension workflows to `${{{{ github.sha }}}}`.

        {message}

        Release Notes:

        - N/A
    "#};

    steps::CreatePrStep::new(title, "update-extension-cli-sha", generated_token)
        .with_body(body)
        .into()
}

fn update_sha_in_extensions(publish_job: &NamedJob, message: &WorkflowInput) -> NamedJob {
    let extensions_repo = RepositoryTarget::new("zed-industries", &["extensions"]);
    let (generate_token, generated_token) =
        generate_token(vars::ZED_ZIPPY_APP_ID, vars::ZED_ZIPPY_APP_PRIVATE_KEY)
            .for_repository(extensions_repo)
            .into();

    fn checkout_extensions_repo(token: &StepOutput) -> Step<Use> {
        named::uses(
            "actions",
            "checkout",
            "11bd71901bbe5b1630ceea73d27597364c9af683", // v4
        )
        .add_with(("repository", "zed-industries/extensions"))
        .add_with(("token", token.to_string()))
    }

    fn replace_sha() -> Step<Run> {
        named::bash(indoc! {r#"
            sed -i "s/ZED_EXTENSION_CLI_SHA: [a-f0-9]*/ZED_EXTENSION_CLI_SHA: $GITHUB_SHA/" \
                .github/workflows/ci.yml
        "#})
    }

    let (get_short_sha_step, short_sha) = get_short_sha();

    named::job(
        Job::default()
            .cond(dispatched_from_main_guard())
            .needs(vec![publish_job.name.clone()])
            .runs_on(runners::LINUX_SMALL)
            .add_step(generate_token)
            .add_step(get_short_sha_step)
            .add_step(checkout_extensions_repo(&generated_token))
            .add_step(replace_sha())
            .add_step(create_pull_request_extensions(
                &generated_token,
                &short_sha,
                message,
            )),
    )
}

fn create_pull_request_extensions(
    generated_token: &StepOutput,
    short_sha: &StepOutput,
    message: &WorkflowInput,
) -> Step<Use> {
    let title = format!("Bump extension CLI version to `{}`", short_sha);

    let body = formatdoc! {r#"
        This PR bumps the extension CLI version to https://github.com/zed-industries/zed/commit/${{{{ github.sha }}}}.

        {message}
    "#};

    steps::CreatePrStep::new(title, "update-extension-cli-sha", generated_token)
        .with_body(body)
        .with_labels("allow-no-extension")
        .into()
}

fn get_short_sha() -> (Step<Run>, StepOutput) {
    let step = named::bash(indoc::indoc! {r#"
        echo "sha_short=$(echo "$GITHUB_SHA" | cut -c1-7)" >> "$GITHUB_OUTPUT"
    "#})
    .id("short-sha");

    let step_output = vars::StepOutput::new(&step, "sha_short");

    (step, step_output)
}
