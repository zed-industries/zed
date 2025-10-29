use gh_workflow::{Concurrency, Event, Expression, PullRequest, Push, Run, Step, Use, Workflow};

use super::{
    runners::{self, Platform},
    steps::{self, FluentBuilder, NamedJob, named, release_job},
};

fn str_vec(values: &'static [&'static str]) -> Vec<String> {
    values.into_iter().map(ToString::to_string).collect()
}

pub(crate) fn run_tests_in(paths: &'static [&'static str], workflow: Workflow) -> Workflow {
    let paths = str_vec(paths);
    workflow
        .add_event(Event::default()
            .push(
                Push::default()
                    .branches(
                        [
                            "main",
                            "v[0-9]+.[0-9]+.x", // any release branch
                        ]
                        .map(String::from),
                    )
                    .paths(paths.clone())
                ,
            )
            .pull_request(
                PullRequest::default().branches(
                    [
                        "**", // all branches
                    ]
                    .map(String::from),
                )
                .paths(paths),
            ))
        .concurrency(Concurrency::default()
            .group("${{ github.workflow }}-${{ github.ref_name }}-${{ github.ref_name == 'main' && github.sha || 'anysha' }}")
            .cancel_in_progress(true)
        )
        .add_env(( "CARGO_TERM_COLOR", "always" ))
        .add_env(( "RUST_BACKTRACE", 1 ))
        .add_env(( "CARGO_INCREMENTAL", 0 ))
}

pub(crate) fn run_tests() -> Workflow {
    let windows_tests = run_platform_tests(Platform::Windows);
    let linux_tests = run_platform_tests(Platform::Linux);
    let mac_tests = run_platform_tests(Platform::Mac);
    let migrations = check_postgres_and_protobuf_migrations();
    let doctests = doctests();
    let check_dependencies = check_dependencies();
    let check_other_binaries = check_workspace_binaries();

    named::workflow()
        .map(|workflow| {
            run_tests_in(
                &[
                    "!docs/**",
                    "!script/update_top_ranking_issues/**",
                    "!.github/ISSUE_TEMPLATE/**",
                    "!.github/workflows/**",
                    ".github/workflows/run_tests.yml", // re-include this workflow so it re-runs when changed
                ],
                workflow,
            )
        })
        .add_job(windows_tests.name, windows_tests.job)
        .add_job(linux_tests.name, linux_tests.job)
        .add_job(mac_tests.name, mac_tests.job)
        .add_job(migrations.name, migrations.job)
        .add_job(doctests.name, doctests.job)
        .add_job(check_dependencies.name, check_dependencies.job)
        .add_job(check_other_binaries.name, check_other_binaries.job)
}

fn check_dependencies() -> NamedJob {
    fn install_cargo_machete() -> Step<Use> {
        named::uses(
            "clechasseur",
            "rs-cargo",
            "8435b10f6e71c2e3d4d3b7573003a8ce4bfc6386", // v2
        )
        .add_with(("command", "install"))
        .add_with(("args", "cargo-machete@0.7.0"))
    }

    fn run_cargo_machete() -> Step<Use> {
        named::uses(
            "clechasseur",
            "rs-cargo",
            "8435b10f6e71c2e3d4d3b7573003a8ce4bfc6386", // v2
        )
        .add_with(("command", "machete"))
    }

    fn check_cargo_lock() -> Step<Run> {
        named::bash("cargo update --locked --workspace")
    }

    fn check_vulnerable_dependencies() -> Step<Use> {
        named::uses(
            "actions",
            "dependency-review-action",
            "67d4f4bd7a9b17a0db54d2a7519187c65e339de8", // v4
        )
        .if_condition(Expression::new("github.event_name == 'pull_request'"))
        .with(("license-check", false))
    }

    named::job(
        release_job(&[])
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo())
            .add_step(install_cargo_machete())
            .add_step(run_cargo_machete())
            .add_step(check_cargo_lock())
            .add_step(check_vulnerable_dependencies()),
    )
}

fn check_workspace_binaries() -> NamedJob {
    named::job(
        release_job(&[])
            .runs_on(runners::LINUX_LARGE)
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_cargo_config(Platform::Linux))
            .map(steps::install_linux_dependencies)
            .add_step(steps::script("cargo build -p collab"))
            .add_step(steps::script("cargo build --workspace --bins --examples"))
            .add_step(steps::cleanup_cargo_config(Platform::Linux)),
    )
}

pub(crate) fn run_platform_tests(platform: Platform) -> NamedJob {
    let runner = match platform {
        Platform::Windows => runners::WINDOWS_DEFAULT,
        Platform::Linux => runners::LINUX_DEFAULT,
        Platform::Mac => runners::MAC_DEFAULT,
    };
    NamedJob {
        name: format!("run_tests_{platform}"),
        job: release_job(&[])
            .runs_on(runner)
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_cargo_config(platform))
            .when(
                platform == Platform::Linux,
                steps::install_linux_dependencies,
            )
            .add_step(steps::setup_node())
            .add_step(steps::clippy(platform))
            .add_step(steps::cargo_install_nextest(platform))
            .add_step(steps::clear_target_dir_if_large(platform))
            .add_step(steps::cargo_nextest(platform))
            .add_step(steps::cleanup_cargo_config(platform)),
    }
}

pub(crate) fn check_postgres_and_protobuf_migrations() -> NamedJob {
    fn remove_untracked_files() -> Step<Run> {
        named::bash("git clean -df")
    }

    fn ensure_fresh_merge() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            if [ -z "$GITHUB_BASE_REF" ];
            then
              echo "BUF_BASE_BRANCH=$(git merge-base origin/main HEAD)" >> "$GITHUB_ENV"
            else
              git checkout -B temp
              git merge -q "origin/$GITHUB_BASE_REF" -m "merge main into temp"
              echo "BUF_BASE_BRANCH=$GITHUB_BASE_REF" >> "$GITHUB_ENV"
            fi
        "#})
    }

    fn bufbuild_setup_action() -> Step<Use> {
        named::uses("bufbuild", "buf-setup-action", "v1").add_with(("version", "v1.29.0"))
    }

    fn bufbuild_breaking_action() -> Step<Use> {
        named::uses("bufbuild", "buf-breaking-action", "v1").add_with(("input", "crates/proto/proto/"))
            .add_with(("against", "https://github.com/${GITHUB_REPOSITORY}.git#branch=${BUF_BASE_BRANCH},subdir=crates/proto/proto/"))
    }

    named::job(
        release_job(&[])
            .runs_on(runners::MAC_DEFAULT)
            .add_step(steps::checkout_repo().with(("fetch-depth", 0))) // fetch full history
            .add_step(remove_untracked_files())
            .add_step(ensure_fresh_merge())
            .add_step(bufbuild_setup_action())
            .add_step(bufbuild_breaking_action()),
    )
}

fn doctests() -> NamedJob {
    fn run_doctests() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            cargo test --workspace --doc --no-fail-fast
        "#})
        .id("run_doctests")
    }

    named::job(
        release_job(&[])
            .runs_on(runners::LINUX_DEFAULT)
            .add_step(steps::checkout_repo())
            .add_step(steps::cache_rust_dependencies())
            .map(steps::install_linux_dependencies)
            .add_step(steps::setup_cargo_config(Platform::Linux))
            .add_step(run_doctests())
            .add_step(steps::cleanup_cargo_config(Platform::Linux)),
    )
}
