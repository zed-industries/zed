use gh_workflow::{ctx::Context, *};
use indoc::indoc;

use crate::tasks::workflows::{
    extension_bump::{RepositoryTarget, generate_token},
    runners,
    steps::{self, CommonJobConditions, NamedJob, named},
    vars::{self, StepOutput},
};

pub fn publish_extension_cli() -> Workflow {
    let publish = publish_job();
    let update_sha_in_zed = update_sha_in_zed(&publish);
    let update_sha_in_extensions = update_sha_in_extensions(&publish);

    named::workflow()
        .on(Event::default().push(Push::default().tags(vec!["extension-cli".to_string()])))
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("CARGO_INCREMENTAL", 0))
        .add_job(publish.name, publish.job)
        .add_job(update_sha_in_zed.name, update_sha_in_zed.job)
        .add_job(update_sha_in_extensions.name, update_sha_in_extensions.job)
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

fn update_sha_in_zed(publish_job: &NamedJob) -> NamedJob {
    let (generate_token, generated_token) = generate_token(
        vars::ZED_ZIPPY_APP_ID,
        vars::ZED_ZIPPY_APP_PRIVATE_KEY,
        Some(RepositoryTarget::current()),
    );

    fn replace_sha() -> Step<Run> {
        named::bash(indoc! {r#"
            sed -i "s/ZED_EXTENSION_CLI_SHA: &str = \"[a-f0-9]*\"/ZED_EXTENSION_CLI_SHA: \&str = \"${{ github.sha }}\"/" \
                tooling/xtask/src/tasks/workflows/extension_tests.rs
        "#})
    }

    fn regenerate_workflows() -> Step<Run> {
        named::bash("cargo xtask workflows")
    }

    let (get_short_sha_step, short_sha) = get_short_sha();

    named::job(
        Job::default()
            .with_repository_owner_guard()
            .needs(vec![publish_job.name.clone()])
            .runs_on(runners::LINUX_LARGE)
            .add_step(generate_token)
            .add_step(steps::checkout_repo())
            .add_step(steps::cache_rust_dependencies_namespace())
            .add_step(get_short_sha_step)
            .add_step(replace_sha())
            .add_step(regenerate_workflows())
            .add_step(create_pull_request_zed(&generated_token, &short_sha)),
    )
}

fn create_pull_request_zed(generated_token: &StepOutput, short_sha: &StepOutput) -> Step<Use> {
    let title = format!(
        "extension_ci: Bump extension CLI version to `{}`",
        short_sha
    );

    named::uses("peter-evans", "create-pull-request", "v7").with(
        Input::default()
            .add("title", title.clone())
            .add(
                "body",
                indoc! {r#"
                    This PR bumps the extension CLI version used in the extension workflows to `${{ github.sha }}`.

                    Release Notes:

                    - N/A
                "#},
            )
            .add("commit-message", title)
            .add("branch", "update-extension-cli-sha")
            .add(
                "committer",
                "zed-zippy[bot] <234243425+zed-zippy[bot]@users.noreply.github.com>",
            )
            .add("base", "main")
            .add("delete-branch", true)
            .add("token", generated_token.to_string())
            .add("sign-commits", true)
            .add("assignees", Context::github().actor().to_string()),
    )
}

fn update_sha_in_extensions(publish_job: &NamedJob) -> NamedJob {
    let extensions_repo = RepositoryTarget::new("zed-industries", &["extensions"]);
    let (generate_token, generated_token) = generate_token(
        vars::ZED_ZIPPY_APP_ID,
        vars::ZED_ZIPPY_APP_PRIVATE_KEY,
        Some(extensions_repo),
    );

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
            sed -i "s/ZED_EXTENSION_CLI_SHA: [a-f0-9]*/ZED_EXTENSION_CLI_SHA: ${{ github.sha }}/" \
                .github/workflows/ci.yml
        "#})
    }

    let (get_short_sha_step, short_sha) = get_short_sha();

    named::job(
        Job::default()
            .with_repository_owner_guard()
            .needs(vec![publish_job.name.clone()])
            .runs_on(runners::LINUX_SMALL)
            .add_step(generate_token)
            .add_step(get_short_sha_step)
            .add_step(checkout_extensions_repo(&generated_token))
            .add_step(replace_sha())
            .add_step(create_pull_request_extensions(&generated_token, &short_sha)),
    )
}

fn create_pull_request_extensions(
    generated_token: &StepOutput,
    short_sha: &StepOutput,
) -> Step<Use> {
    let title = format!("Bump extension CLI version to `{}`", short_sha);

    named::uses("peter-evans", "create-pull-request", "v7").with(
        Input::default()
            .add("title", title.clone())
            .add(
                "body",
                indoc! {r#"
                    This PR bumps the extension CLI version to https://github.com/zed-industries/zed/commit/${{ github.sha }}.
                "#},
            )
            .add("commit-message", title)
            .add("branch", "update-extension-cli-sha")
            .add(
                "committer",
                "zed-zippy[bot] <234243425+zed-zippy[bot]@users.noreply.github.com>",
            )
            .add("base", "main")
            .add("delete-branch", true)
            .add("token", generated_token.to_string())
            .add("sign-commits", true)
            .add("labels", "allow-no-extension")
            .add("assignees", Context::github().actor().to_string()),
    )
}

fn get_short_sha() -> (Step<Run>, StepOutput) {
    let step = named::bash(indoc::indoc! {r#"
        echo "sha_short=$(echo "${{ github.sha }}" | cut -c1-7)" >> "$GITHUB_OUTPUT"
    "#})
    .id("short-sha");

    let step_output = vars::StepOutput::new(&step, "sha_short");

    (step, step_output)
}
