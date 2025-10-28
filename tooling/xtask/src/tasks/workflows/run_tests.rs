use gh_workflow::{Event, Run, Step, Use, Workflow, WorkflowCall};

use super::{
    runners::{self, Platform},
    steps::{self, FluentBuilder, NamedJob, named, release_job},
};

pub(crate) fn run_tests() -> Workflow {
    let windows_tests = run_platform_tests(Platform::Windows);
    let linux_tests = run_platform_tests(Platform::Linux);
    let mac_tests = run_platform_tests(Platform::Mac);
    let migrations = check_postgres_and_protobuf_migrations();
    let style = style();
    let docs = check_docs();
    let action_lint = actionlint();
    let doctests = doctests();

    named::workflow()
        // todo! inputs?
        .on(Event::default().workflow_call(WorkflowCall::default()))
        .add_job(style.name, style.job)
        .add_job(windows_tests.name, windows_tests.job)
        .add_job(linux_tests.name, linux_tests.job)
        .add_job(mac_tests.name, mac_tests.job)
        .add_job(migrations.name, migrations.job)
        .add_job(docs.name, docs.job)
        .add_job(action_lint.name, action_lint.job)
        .add_job(doctests.name, doctests.job)
}

pub(crate) fn run_platform_tests(platform: Platform) -> NamedJob {
    let runner = match platform {
        Platform::Windows => runners::WINDOWS_DEFAULT,
        Platform::Linux => runners::LINUX_DEFAULT,
        Platform::Mac => runners::MAC_DEFAULT,
    };
    // todo! missing script/linux step
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

    fn check_for_typos() -> Step<Use> {
        named::uses(
            "crate-ci",
            "typos",
            "80c8a4945eec0f6d464eaf9e65ed98ef085283d1",
        ) // v1.38.1
        .with(("config", "./typos.toml"))
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
            .add_step(check_for_typos())
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

fn check_docs() -> NamedJob {
    // todo! would have preferred to just reference the action here while building, but the gh-workflow crate
    // only supports using repo actions (owner, name, version), not local actions (path)
    fn lychee_link_check(dir: &str) -> Step<Use> {
        named::uses(
            "lycheeverse",
            "lychee-action",
            "82202e5e9c2f4ef1a55a3d02563e1cb6041e5332",
        ) // v2.4.1
        .add_with(("args", format!("--no-progress --exclude '^http' '{dir}'")))
        .add_with(("fail", true))
    }

    fn install_mdbook() -> Step<Use> {
        named::uses(
            "peaceiris",
            "actions-mdbook",
            "ee69d230fe19748b7abf22df32acaa93833fad08", // v2
        )
        .with(("mdbook-version", "0.4.37"))
    }

    fn build_docs() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            mkdir -p target/deploy
            mdbook build ./docs --dest-dir=../target/deploy/docs/
        "#})
    }

    named::job(
        release_job(&[])
            .runs_on(runners::LINUX_LARGE)
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_cargo_config(Platform::Linux))
            // todo(ci): un-inline build_docs/action.yml here
            .add_step(steps::cache_rust_dependencies())
            .add_step(lychee_link_check("./docs/src/**/*")) // check markdown links
            .map(steps::install_linux_dependencies)
            .add_step(install_mdbook())
            .add_step(build_docs())
            .add_step(lychee_link_check("target/deploy/docs")), // check links in generated html
    )
}

fn actionlint() -> NamedJob {
    const ACTION_LINT_STEP_ID: &'static str = "get_actionlint";

    fn download_actionlint() -> Step<Run> {
        named::bash("bash <(curl https://raw.githubusercontent.com/rhysd/actionlint/main/scripts/download-actionlint.bash)").id(ACTION_LINT_STEP_ID)
    }

    fn run_actionlint() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            ${{ steps.get_actionlint.outputs.executable }} -color
        "#})
    }

    named::job(
        release_job(&[])
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo())
            .add_step(download_actionlint())
            .add_step(run_actionlint()),
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
