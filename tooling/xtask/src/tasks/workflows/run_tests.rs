use gh_workflow::{
    Concurrency, Event, Expression, Job, PullRequest, Push, Run, Step, Use, Workflow,
};
use indexmap::IndexMap;

use crate::tasks::workflows::{
    nix_build::build_nix,
    runners::Arch,
    steps::{BASH_SHELL, CommonJobConditions, repository_owner_guard_expression},
    vars::{self, PathCondition},
};

use super::{
    runners::{self, Platform},
    steps::{self, FluentBuilder, NamedJob, named, release_job},
};

pub(crate) fn run_tests() -> Workflow {
    // Specify anything which should potentially skip full test suite in this regex:
    // - docs/
    // - script/update_top_ranking_issues/
    // - .github/ISSUE_TEMPLATE/
    // - .github/workflows/  (except .github/workflows/ci.yml)
    let should_run_tests = PathCondition::inverted(
        "run_tests",
        r"^(docs/|script/update_top_ranking_issues/|\.github/(ISSUE_TEMPLATE|workflows/(?!run_tests)))",
    );
    let should_check_docs = PathCondition::new("run_docs", r"^(docs/|crates/.*\.rs)");
    let should_check_scripts = PathCondition::new(
        "run_action_checks",
        r"^\.github/(workflows/|actions/|actionlint.yml)|tooling/xtask|script/",
    );
    let should_check_licences =
        PathCondition::new("run_licenses", r"^(Cargo.lock|script/.*licenses)");
    let should_build_nix = PathCondition::new(
        "run_nix",
        r"^(nix/|flake\.|Cargo\.|rust-toolchain.toml|\.cargo/config.toml)",
    );

    let orchestrate = orchestrate(&[
        &should_check_scripts,
        &should_check_docs,
        &should_check_licences,
        &should_build_nix,
        &should_run_tests,
    ]);

    let mut jobs = vec![
        orchestrate,
        check_style(),
        should_run_tests.guard(clippy(Platform::Windows)),
        should_run_tests.guard(clippy(Platform::Linux)),
        should_run_tests.guard(clippy(Platform::Mac)),
        should_run_tests.guard(run_platform_tests(Platform::Windows)),
        should_run_tests.guard(run_platform_tests(Platform::Linux)),
        should_run_tests.guard(run_platform_tests(Platform::Mac)),
        should_run_tests.guard(doctests()),
        should_run_tests.guard(check_workspace_binaries()),
        should_run_tests.guard(check_dependencies()), // could be more specific here?
        should_check_docs.guard(check_docs()),
        should_check_licences.guard(check_licenses()),
        should_check_scripts.guard(check_scripts()),
        should_build_nix.guard(build_nix(
            Platform::Linux,
            Arch::X86_64,
            "debug",
            // *don't* cache the built output
            Some("-zed-editor-[0-9.]*-nightly"),
            &[],
        )),
        should_build_nix.guard(build_nix(
            Platform::Mac,
            Arch::AARCH64,
            "debug",
            // *don't* cache the built output
            Some("-zed-editor-[0-9.]*-nightly"),
            &[],
        )),
    ];
    let tests_pass = tests_pass(&jobs);

    jobs.push(should_run_tests.guard(check_postgres_and_protobuf_migrations())); // could be more specific here?

    named::workflow()
        .add_event(
            Event::default()
                .push(
                    Push::default()
                        .add_branch("main")
                        .add_branch("v[0-9]+.[0-9]+.x"),
                )
                .pull_request(PullRequest::default().add_branch("**")),
        )
        .concurrency(
            Concurrency::default()
                .group(concat!(
                    "${{ github.workflow }}-${{ github.ref_name }}-",
                    "${{ github.ref_name == 'main' && github.sha || 'anysha' }}"
                ))
                .cancel_in_progress(true),
        )
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("RUST_BACKTRACE", 1))
        .add_env(("CARGO_INCREMENTAL", 0))
        .map(|mut workflow| {
            for job in jobs {
                workflow = workflow.add_job(job.name, job.job)
            }
            workflow
        })
        .add_job(tests_pass.name, tests_pass.job)
}

// Generates a bash script that checks changed files against regex patterns
// and sets GitHub output variables accordingly
pub fn orchestrate(rules: &[&PathCondition]) -> NamedJob {
    let name = "orchestrate".to_owned();
    let step_name = "filter".to_owned();
    let mut script = String::new();

    script.push_str(indoc::indoc! {r#"
        if [ -z "$GITHUB_BASE_REF" ]; then
          echo "Not in a PR context (i.e., push to main/stable/preview)"
          COMPARE_REV="$(git rev-parse HEAD~1)"
        else
          echo "In a PR context comparing to pull_request.base.ref"
          git fetch origin "$GITHUB_BASE_REF" --depth=350
          COMPARE_REV="$(git merge-base "origin/${GITHUB_BASE_REF}" HEAD)"
        fi
        CHANGED_FILES="$(git diff --name-only "$COMPARE_REV" ${{ github.sha }})"

        check_pattern() {
          local output_name="$1"
          local pattern="$2"
          local grep_arg="$3"

          echo "$CHANGED_FILES" | grep "$grep_arg" "$pattern" && \
            echo "${output_name}=true" >> "$GITHUB_OUTPUT" || \
            echo "${output_name}=false" >> "$GITHUB_OUTPUT"
        }

    "#});

    let mut outputs = IndexMap::new();

    for rule in rules {
        assert!(
            rule.set_by_step
                .borrow_mut()
                .replace(name.clone())
                .is_none()
        );
        assert!(
            outputs
                .insert(
                    rule.name.to_owned(),
                    format!("${{{{ steps.{}.outputs.{} }}}}", step_name, rule.name)
                )
                .is_none()
        );

        let grep_arg = if rule.invert { "-qvP" } else { "-qP" };
        script.push_str(&format!(
            "check_pattern \"{}\" '{}' {}\n",
            rule.name, rule.pattern, grep_arg
        ));
    }

    let job = Job::default()
        .runs_on(runners::LINUX_SMALL)
        .with_repository_owner_guard()
        .outputs(outputs)
        .add_step(steps::checkout_repo().add_with((
            "fetch-depth",
            "${{ github.ref == 'refs/heads/main' && 2 || 350 }}",
        )))
        .add_step(
            Step::new(step_name.clone())
                .run(script)
                .id(step_name)
                .shell(BASH_SHELL),
        );

    NamedJob { name, job }
}

pub fn tests_pass(jobs: &[NamedJob]) -> NamedJob {
    let mut script = String::from(indoc::indoc! {r#"
        set +x
        EXIT_CODE=0

        check_result() {
          echo "* $1: $2"
          if [[ "$2" != "skipped" && "$2" != "success" ]]; then EXIT_CODE=1; fi
        }

    "#});

    script.push_str(
        &jobs
            .iter()
            .map(|job| {
                format!(
                    "check_result \"{}\" \"${{{{ needs.{}.result }}}}\"",
                    job.name, job.name
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
    );

    script.push_str("\n\nexit $EXIT_CODE\n");

    let job = Job::default()
        .runs_on(runners::LINUX_SMALL)
        .needs(
            jobs.iter()
                .map(|j| j.name.to_string())
                .collect::<Vec<String>>(),
        )
        .cond(repository_owner_guard_expression(true))
        .add_step(named::bash(&script));

    named::job(job)
}

fn check_style() -> NamedJob {
    fn check_for_typos() -> Step<Use> {
        named::uses(
            "crate-ci",
            "typos",
            "2d0ce569feab1f8752f1dde43cc2f2aa53236e06",
        ) // v1.40.0
        .with(("config", "./typos.toml"))
    }
    named::job(
        release_job(&[])
            .runs_on(runners::LINUX_MEDIUM)
            .add_step(steps::checkout_repo())
            .add_step(steps::cache_rust_dependencies_namespace())
            .add_step(steps::setup_pnpm())
            .add_step(steps::prettier())
            .add_step(steps::cargo_fmt())
            .add_step(steps::script("./script/check-todos"))
            .add_step(steps::script("./script/check-keymaps"))
            .add_step(check_for_typos()),
    )
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
            .add_step(steps::cache_rust_dependencies_namespace())
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
            .add_step(steps::cache_rust_dependencies_namespace())
            .map(steps::install_linux_dependencies)
            .add_step(steps::script("cargo build -p collab"))
            .add_step(steps::script("cargo build --workspace --bins --examples"))
            .add_step(steps::cleanup_cargo_config(Platform::Linux)),
    )
}

pub(crate) fn clippy(platform: Platform) -> NamedJob {
    let runner = match platform {
        Platform::Windows => runners::WINDOWS_DEFAULT,
        Platform::Linux => runners::LINUX_DEFAULT,
        Platform::Mac => runners::MAC_DEFAULT,
    };
    NamedJob {
        name: format!("clippy_{platform}"),
        job: release_job(&[])
            .runs_on(runner)
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_cargo_config(platform))
            .when(platform == Platform::Linux, |this| {
                this.add_step(steps::cache_rust_dependencies_namespace())
            })
            .when(
                platform == Platform::Linux,
                steps::install_linux_dependencies,
            )
            .add_step(steps::clippy(platform)),
    }
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
            .when(platform == Platform::Linux, |this| {
                this.add_step(steps::cache_rust_dependencies_namespace())
            })
            .when(
                platform == Platform::Linux,
                steps::install_linux_dependencies,
            )
            .add_step(steps::setup_node())
            .when(platform == Platform::Linux, |job| {
                job.add_step(steps::cargo_install_nextest())
            })
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
        named::uses("bufbuild", "buf-setup-action", "v1")
            .add_with(("version", "v1.29.0"))
            .add_with(("github_token", vars::GITHUB_TOKEN))
    }

    fn bufbuild_breaking_action() -> Step<Use> {
        named::uses("bufbuild", "buf-breaking-action", "v1").add_with(("input", "crates/proto/proto/"))
            .add_with(("against", "https://github.com/${GITHUB_REPOSITORY}.git#branch=${BUF_BASE_BRANCH},subdir=crates/proto/proto/"))
    }

    named::job(
        release_job(&[])
            .runs_on(runners::LINUX_DEFAULT)
            .add_env(("GIT_AUTHOR_NAME", "Protobuf Action"))
            .add_env(("GIT_AUTHOR_EMAIL", "ci@zed.dev"))
            .add_env(("GIT_COMMITTER_NAME", "Protobuf Action"))
            .add_env(("GIT_COMMITTER_EMAIL", "ci@zed.dev"))
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
            .add_step(steps::cache_rust_dependencies_namespace())
            .map(steps::install_linux_dependencies)
            .add_step(steps::setup_cargo_config(Platform::Linux))
            .add_step(run_doctests())
            .add_step(steps::cleanup_cargo_config(Platform::Linux)),
    )
}

fn check_licenses() -> NamedJob {
    named::job(
        Job::default()
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo())
            .add_step(steps::cache_rust_dependencies_namespace())
            .add_step(steps::script("./script/check-licenses"))
            .add_step(steps::script("./script/generate-licenses")),
    )
}

fn check_docs() -> NamedJob {
    fn lychee_link_check(dir: &str) -> Step<Use> {
        named::uses(
            "lycheeverse",
            "lychee-action",
            "82202e5e9c2f4ef1a55a3d02563e1cb6041e5332",
        ) // v2.4.1
        .add_with(("args", format!("--no-progress --exclude '^http' '{dir}'")))
        .add_with(("fail", true))
        .add_with(("jobSummary", false))
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
            .add_step(steps::cache_rust_dependencies_namespace())
            .add_step(
                lychee_link_check("./docs/src/**/*"), // check markdown links
            )
            .map(steps::install_linux_dependencies)
            .add_step(steps::script("./script/generate-action-metadata"))
            .add_step(install_mdbook())
            .add_step(build_docs())
            .add_step(
                lychee_link_check("target/deploy/docs"), // check links in generated html
            ),
    )
}

pub(crate) fn check_scripts() -> NamedJob {
    fn download_actionlint() -> Step<Run> {
        named::bash(
            "bash <(curl https://raw.githubusercontent.com/rhysd/actionlint/main/scripts/download-actionlint.bash)",
        )
    }

    fn run_actionlint() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            ${{ steps.get_actionlint.outputs.executable }} -color
        "#})
    }

    fn run_shellcheck() -> Step<Run> {
        named::bash("./script/shellcheck-scripts error")
    }

    fn check_xtask_workflows() -> Step<Run> {
        named::bash(indoc::indoc! {r#"
            cargo xtask workflows
            if ! git diff --exit-code .github; then
              echo "Error: .github directory has uncommitted changes after running 'cargo xtask workflows'"
              echo "Please run 'cargo xtask workflows' locally and commit the changes"
              exit 1
            fi
        "#})
    }

    named::job(
        release_job(&[])
            .runs_on(runners::LINUX_SMALL)
            .add_step(steps::checkout_repo())
            .add_step(run_shellcheck())
            .add_step(download_actionlint().id("get_actionlint"))
            .add_step(run_actionlint())
            .add_step(check_xtask_workflows()),
    )
}
