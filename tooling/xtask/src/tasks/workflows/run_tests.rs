use gh_workflow::{Run, Step, Use, Workflow};

use super::{
    runners::{self, Platform},
    steps::{self, NamedJob, named, release_job},
};

pub(crate) fn run_tests() -> Workflow {
    let windows_tests = run_platform_tests(Platform::Windows);
    let linux_tests = run_platform_tests(Platform::Linux);
    let mac_tests = run_platform_tests(Platform::Mac);
    let migrations = check_postgres_and_protobuf_migrations();
    let style = style();

    named::workflow()
        .add_job(style.name, style.job)
        .add_job(windows_tests.name, windows_tests.job)
        .add_job(linux_tests.name, linux_tests.job)
        .add_job(mac_tests.name, mac_tests.job)
        .add_job(migrations.name, migrations.job)
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
            .add_step(steps::setup_node())
            .add_step(steps::cargo_install_nextest(platform))
            .add_step(steps::clear_target_dir_if_large(platform))
            .add_step(steps::cargo_nextest(platform))
            .add_step(steps::cleanup_cargo_config(platform)),
    }
}

pub(crate) fn style() -> NamedJob {
    fn prettier_check_docs() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            pnpm dlx "prettier@${PRETTIER_VERSION}" . --check || {
              echo "To fix, run from the root of the Zed repo:"
              echo "  cd docs && pnpm dlx prettier@${PRETTIER_VERSION} . --write && cd .."
              false
            }
        "#})
        .working_directory("./docs")
        .add_env(("PRETTIER_VERSION", "3.5.0"))
    }

    fn prettier_check_default_json() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            pnpm dlx "prettier@${PRETTIER_VERSION}" assets/settings/default.json --check || {
              echo "To fix, run from the root of the Zed repo:"
              echo "  pnpm dlx prettier@${PRETTIER_VERSION} assets/settings/default.json --write"
              false
            }
        "#})
        .add_env(("PRETTIER_VERSION", "3.5.0"))
    }

    named::job(
        release_job(&[])
            .runs_on(runners::LINUX_MEDIUM)
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_pnpm())
            .add_step(prettier_check_docs())
            .add_step(prettier_check_default_json())
            .add_step(steps::script("./script/check-todos"))
            .add_step(steps::script("./script/check-keymaps"))
            // check style steps inlined
            .add_step(steps::cargo_fmt())
            .add_step(steps::script("./script/clippy")),
    )
}

pub(crate) fn check_style() -> NamedJob {
    let job = release_job(&[])
        .runs_on(runners::MAC_DEFAULT)
        .add_step(
            steps::checkout_repo()
                .add_with(("clean", false))
                // todo! why is this fetching full history?
                .add_with(("fetch-depth", 0)),
        )
        .add_step(steps::cargo_fmt())
        .add_step(steps::script("./script/clippy"));

    named::job(job)
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
