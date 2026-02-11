use gh_workflow::{
    Concurrency, Container, Event, Expression, Job, Port, PullRequest, Push, Run, Step, Use,
    Workflow,
};
use indexmap::IndexMap;

use crate::tasks::workflows::{
    steps::{CommonJobConditions, repository_owner_guard_expression},
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

    let orchestrate = orchestrate(&[
        &should_check_scripts,
        &should_check_docs,
        &should_check_licences,
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
    orchestrate_impl(rules, true)
}

pub fn orchestrate_without_package_filter(rules: &[&PathCondition]) -> NamedJob {
    orchestrate_impl(rules, false)
}

fn orchestrate_impl(rules: &[&PathCondition], include_package_filter: bool) -> NamedJob {
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

    if include_package_filter {
        script.push_str(indoc::indoc! {r#"
        # Check for changes that require full rebuild (no filter)
        # Direct pushes to main/stable/preview always run full suite
        if [ -z "$GITHUB_BASE_REF" ]; then
          echo "Not a PR, running full test suite"
          echo "changed_packages=" >> "$GITHUB_OUTPUT"
        elif echo "$CHANGED_FILES" | grep -qP '^(rust-toolchain\.toml|\.cargo/|\.github/|Cargo\.(toml|lock)$)'; then
          echo "Toolchain, cargo config, or root Cargo files changed, will run all tests"
          echo "changed_packages=" >> "$GITHUB_OUTPUT"
        else
          # Extract changed directories from file paths
          CHANGED_DIRS=$(echo "$CHANGED_FILES" | \
            grep -oP '^(crates|tooling)/\K[^/]+' | \
            sort -u || true)

          # Build directory-to-package mapping using cargo metadata
          DIR_TO_PKG=$(cargo metadata --format-version=1 --no-deps 2>/dev/null | \
            jq -r '.packages[] | select(.manifest_path | test("crates/|tooling/")) | "\(.manifest_path | capture("(crates|tooling)/(?<dir>[^/]+)") | .dir)=\(.name)"')

          # Map directory names to package names
          FILE_CHANGED_PKGS=""
          for dir in $CHANGED_DIRS; do
            pkg=$(echo "$DIR_TO_PKG" | grep "^${dir}=" | cut -d= -f2 | head -1)
            if [ -n "$pkg" ]; then
              FILE_CHANGED_PKGS=$(printf '%s\n%s' "$FILE_CHANGED_PKGS" "$pkg")
            else
              # Fall back to directory name if no mapping found
              FILE_CHANGED_PKGS=$(printf '%s\n%s' "$FILE_CHANGED_PKGS" "$dir")
            fi
          done
          FILE_CHANGED_PKGS=$(echo "$FILE_CHANGED_PKGS" | grep -v '^$' | sort -u || true)

          # If assets/ changed, add crates that depend on those assets
          if echo "$CHANGED_FILES" | grep -qP '^assets/'; then
            FILE_CHANGED_PKGS=$(printf '%s\n%s\n%s\n%s' "$FILE_CHANGED_PKGS" "settings" "storybook" "assets" | sort -u)
          fi

          # Combine all changed packages
          ALL_CHANGED_PKGS=$(echo "$FILE_CHANGED_PKGS" | grep -v '^$' || true)

          if [ -z "$ALL_CHANGED_PKGS" ]; then
            echo "No package changes detected, will run all tests"
            echo "changed_packages=" >> "$GITHUB_OUTPUT"
          else
            # Build nextest filterset with rdeps for each package
            FILTERSET=$(echo "$ALL_CHANGED_PKGS" | \
              sed 's/.*/rdeps(&)/' | \
              tr '\n' '|' | \
              sed 's/|$//')
            echo "Changed packages filterset: $FILTERSET"
            echo "changed_packages=$FILTERSET" >> "$GITHUB_OUTPUT"
          fi
        fi

    "#});

        outputs.insert(
            "changed_packages".to_owned(),
            format!("${{{{ steps.{}.outputs.changed_packages }}}}", step_name),
        );
    }

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
        .add_step(Step::new(step_name.clone()).run(script).id(step_name));

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
            .add_step(steps::setup_sccache(Platform::Linux))
            .add_step(steps::script("cargo build -p collab"))
            .add_step(steps::script("cargo build --workspace --bins --examples"))
            .add_step(steps::show_sccache_stats(Platform::Linux))
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
            .when(
                platform == Platform::Linux || platform == Platform::Mac,
                |this| this.add_step(steps::cache_rust_dependencies_namespace()),
            )
            .when(
                platform == Platform::Linux,
                steps::install_linux_dependencies,
            )
            .add_step(steps::setup_sccache(platform))
            .add_step(steps::install_cargo_mtime(platform))
            .add_step(steps::run_cargo_mtime(platform))
            .add_step(steps::clippy(platform))
            .add_step(steps::show_sccache_stats(platform)),
    }
}

pub(crate) fn run_platform_tests(platform: Platform) -> NamedJob {
    run_platform_tests_impl(platform, true)
}

pub(crate) fn run_platform_tests_no_filter(platform: Platform) -> NamedJob {
    run_platform_tests_impl(platform, false)
}

fn run_platform_tests_impl(platform: Platform, filter_packages: bool) -> NamedJob {
    let runner = match platform {
        Platform::Windows => runners::WINDOWS_DEFAULT,
        Platform::Linux => runners::LINUX_DEFAULT,
        Platform::Mac => runners::MAC_DEFAULT,
    };
    NamedJob {
        name: format!("run_tests_{platform}"),
        job: release_job(&[])
            .runs_on(runner)
            .when(platform == Platform::Linux, |job| {
                job.add_service(
                    "postgres",
                    Container::new("postgres:15")
                        .add_env(("POSTGRES_HOST_AUTH_METHOD", "trust"))
                        .ports(vec![Port::Name("5432:5432".into())])
                        .options(
                            "--health-cmd pg_isready \
                             --health-interval 500ms \
                             --health-timeout 5s \
                             --health-retries 10",
                        ),
                )
            })
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_cargo_config(platform))
            .when(
                platform == Platform::Linux || platform == Platform::Mac,
                |this| this.add_step(steps::cache_rust_dependencies_namespace()),
            )
            .when(
                platform == Platform::Linux,
                steps::install_linux_dependencies,
            )
            .add_step(steps::setup_sccache(platform))
            .add_step(steps::setup_node())
            .when(
                platform == Platform::Linux || platform == Platform::Mac,
                |job| job.add_step(steps::cargo_install_nextest()),
            )
            .add_step(steps::clear_target_dir_if_large(platform))
            .add_step(steps::install_cargo_mtime(platform))
            .add_step(steps::run_cargo_mtime(platform))
            .when(filter_packages, |job| {
                job.add_step(
                    steps::cargo_nextest(platform).with_changed_packages_filter("orchestrate"),
                )
            })
            .when(!filter_packages, |job| {
                job.add_step(steps::cargo_nextest(platform))
            })
            .add_step(steps::show_sccache_stats(platform))
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
            .add_step(steps::setup_sccache(Platform::Linux))
            .add_step(run_doctests())
            .add_step(steps::show_sccache_stats(Platform::Linux))
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
