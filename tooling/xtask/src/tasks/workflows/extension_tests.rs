use gh_workflow::*;
use indoc::indoc;

use crate::tasks::workflows::{
    extension_bump::compare_versions,
    run_tests::{fetch_ts_query_ls, orchestrate_for_extension, run_ts_query_ls, tests_pass},
    runners,
    steps::{
        self, BASH_SHELL, CommonJobConditions, FluentBuilder, NamedJob,
        cache_rust_dependencies_namespace, named,
    },
    vars::{PathCondition, StepOutput, WorkflowInput, one_workflow_per_non_main_branch_and_token},
};

pub(crate) const ZED_EXTENSION_CLI_SHA: &str = "1fa7f1a3ec28ea1eae6db2e937d7a538fb10c0c7";

// This should follow the set target in crates/extension/src/extension_builder.rs
const EXTENSION_RUST_TARGET: &str = "wasm32-wasip2";

// This is used by various extensions repos in the zed-extensions org to run automated tests.
pub(crate) fn extension_tests() -> Workflow {
    let should_check_rust = PathCondition::new("check_rust", r"^(Cargo.lock|Cargo.toml|.*\.rs)$");
    let should_check_extension =
        PathCondition::new("check_extension", r"^(extension\.toml|.*\.scm)$");

    let orchestrate = with_extension_defaults(orchestrate_for_extension(&[
        &should_check_rust,
        &should_check_extension,
    ]));

    let jobs = [
        orchestrate,
        should_check_rust.and_always().then(check_rust()),
        should_check_extension.and_always().then(check_extension()),
    ];

    let tests_pass = tests_pass(&jobs, &[]);

    let working_directory = WorkflowInput::string("working-directory", Some(".".to_owned()));

    named::workflow()
        .add_event(
            Event::default().workflow_call(
                WorkflowCall::default()
                    .add_input(working_directory.name, working_directory.call_input()),
            ),
        )
        .concurrency(one_workflow_per_non_main_branch_and_token(
            "extension-tests",
        ))
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("RUST_BACKTRACE", 1))
        .add_env(("CARGO_INCREMENTAL", 0))
        .add_env(("ZED_EXTENSION_CLI_SHA", ZED_EXTENSION_CLI_SHA))
        .add_env(("RUSTUP_TOOLCHAIN", "stable"))
        .add_env(("CARGO_BUILD_TARGET", EXTENSION_RUST_TARGET))
        .map(|workflow| {
            jobs.into_iter()
                .chain([tests_pass])
                .fold(workflow, |workflow, job| {
                    workflow.add_job(job.name, job.job)
                })
        })
}

fn install_rust_target() -> Step<Run> {
    named::bash(format!("rustup target add {EXTENSION_RUST_TARGET}",))
}

fn get_package_name() -> (Step<Run>, StepOutput) {
    let step = named::bash(indoc! {r#"
        PACKAGE_NAME="$(sed -n 's/^name = "\(.*\)"/\1/p' < Cargo.toml | head -1 | tr -d '[:space:]')"
        echo "package_name=${PACKAGE_NAME}" >> "$GITHUB_OUTPUT"
    "#})
    .id("get-package-name");

    let output = StepOutput::new(&step, "package_name");
    (step, output)
}

fn cargo_fmt_package(package_name: &StepOutput) -> Step<Run> {
    named::bash(r#"cargo fmt -p "$PACKAGE_NAME" -- --check"#)
        .add_env(("PACKAGE_NAME", package_name.to_string()))
}

fn run_clippy(package_name: &StepOutput) -> Step<Run> {
    named::bash(r#"cargo clippy -p "$PACKAGE_NAME" --release --all-features -- --deny warnings"#)
        .add_env(("PACKAGE_NAME", package_name.to_string()))
}

fn run_nextest(package_name: &StepOutput) -> Step<Run> {
    named::bash(
        r#"cargo nextest run -p "$PACKAGE_NAME" --no-fail-fast --no-tests=warn --target "$(rustc -vV | sed -n 's|host: ||p')""#,
    )
    .add_env(("PACKAGE_NAME", package_name.to_string()))
    .add_env(("NEXTEST_NO_TESTS", "warn"))
}

fn extension_job_defaults() -> Defaults {
    Defaults::default().run(
        RunDefaults::default()
            .shell(BASH_SHELL)
            .working_directory("${{ inputs.working-directory }}"),
    )
}

fn with_extension_defaults(named_job: NamedJob) -> NamedJob {
    NamedJob {
        name: named_job.name,
        job: named_job.job.defaults(extension_job_defaults()),
    }
}

fn check_rust() -> NamedJob {
    let (get_package, package_name) = get_package_name();

    let job = Job::default()
        .defaults(extension_job_defaults())
        .with_repository_owner_guard()
        .runs_on(runners::LINUX_LARGE_RAM)
        .timeout_minutes(6u32)
        .add_step(steps::checkout_repo())
        .add_step(steps::cache_rust_dependencies_namespace())
        .add_step(install_rust_target())
        .add_step(get_package)
        .add_step(cargo_fmt_package(&package_name))
        .add_step(run_clippy(&package_name))
        .add_step(steps::cargo_install_nextest())
        .add_step(run_nextest(&package_name));

    named::job(job)
}

pub(crate) fn check_extension() -> NamedJob {
    let (cache_download, cache_hit) = cache_zed_extension_cli();
    let (check_version_job, version_changed, _) = compare_versions();

    let job = Job::default()
        .defaults(extension_job_defaults())
        .with_repository_owner_guard()
        .runs_on(runners::LINUX_LARGE_RAM)
        .timeout_minutes(6u32)
        .add_step(steps::checkout_repo().with_full_history())
        .add_step(cache_download)
        .add_step(download_zed_extension_cli(cache_hit))
        .add_step(cache_rust_dependencies_namespace()) // Extensions can compile Rust, so provide the cache if needed.
        .add_step(check())
        .add_step(fetch_ts_query_ls())
        .add_step(run_ts_query_ls())
        .add_step(check_version_job)
        .add_step(verify_version_did_not_change(version_changed));

    named::job(job)
}

pub fn cache_zed_extension_cli() -> (Step<Use>, StepOutput) {
    let step = named::uses(
        "actions",
        "cache",
        "0057852bfaa89a56745cba8c7296529d2fc39830",
    )
    .id("cache-zed-extension-cli")
    .with(
        Input::default()
            .add("path", "zed-extension")
            .add("key", "zed-extension-${{ env.ZED_EXTENSION_CLI_SHA }}"),
    );
    let output = StepOutput::new(&step, "cache-hit");
    (step, output)
}

pub fn download_zed_extension_cli(cache_hit: StepOutput) -> Step<Run> {
    named::bash(
    indoc! {
        r#"
        wget --quiet "https://zed-extension-cli.nyc3.digitaloceanspaces.com/$ZED_EXTENSION_CLI_SHA/x86_64-unknown-linux-gnu/zed-extension" -O "$GITHUB_WORKSPACE/zed-extension"
        chmod +x "$GITHUB_WORKSPACE/zed-extension"
        "#,
    }
    ).if_condition(Expression::new(format!("{} != 'true'", cache_hit.expr())))
}

pub fn check() -> Step<Run> {
    named::bash(indoc! {
        r#"
        mkdir -p /tmp/ext-scratch
        mkdir -p /tmp/ext-output
        "$GITHUB_WORKSPACE/zed-extension" --source-dir . --scratch-dir /tmp/ext-scratch --output-dir /tmp/ext-output
        "#
    })
}

fn verify_version_did_not_change(version_changed: StepOutput) -> Step<Run> {
    named::bash(indoc! {r#"
        if [[ "$VERSION_CHANGED" == "true" && "$GITHUB_EVENT_NAME" == "pull_request" && "$PR_USER_LOGIN" != "zed-zippy[bot]" ]] ; then
            echo "Version change detected in your change!"
            echo "Version changes happen in separate PRs and will be performed by the zed-zippy bot"
            exit 42
        fi
        "#
    })
    .add_env(("VERSION_CHANGED", version_changed.to_string()))
    .add_env(("PR_USER_LOGIN", "${{ github.event.pull_request.user.login }}"))
}
