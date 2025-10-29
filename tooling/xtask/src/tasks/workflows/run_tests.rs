use gh_workflow::{
    Concurrency, Event, Expression, Job, PullRequest, Push, Run, Step, Use, Workflow,
};

use super::{
    runners::{self, Platform},
    steps::{self, FluentBuilder, NamedJob, named, release_job},
};

/// Represents a pattern to check for changed files and corresponding output variable
// struct ChangeDetectionRule {
//     /// Name of the output variable (e.g., "run_tests", "run_docs")
//     output_name: &'static str,
//     /// Perl-compatible regex pattern to match against changed files
//     pattern: &'static str,
//     /// If true, set output to "true" when pattern does NOT match (inverted logic)
//     invert_match: bool,
// }

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
    // let choose = choose_which_jobs_to_run();

    let windows_tests = run_platform_tests(Platform::Windows, &[]); //.map(|job| job.cond());
    let linux_tests = run_platform_tests(Platform::Linux, &[]);
    let mac_tests = run_platform_tests(Platform::Mac, &[]);
    let migrations = check_postgres_and_protobuf_migrations();
    let doctests = doctests();
    let check_dependencies = check_dependencies();
    let check_other_binaries = check_workspace_binaries();
    let check_style = check_style();

    let jobs = [
        windows_tests,
        linux_tests,
        mac_tests,
        migrations,
        doctests,
        check_dependencies,
        check_other_binaries,
        check_style,
    ];
    let tests_pass = tests_pass(&jobs);

    let mut workflow = named::workflow()
        .add_event(Event::default()
            .push(
                Push::default()
                    .add_branch("main")
                    .add_branch("v[0-9]+.[0-9]+.x")
            )
            .pull_request(PullRequest::default().add_branch("**"))
        )
        .concurrency(Concurrency::default()
            .group("${{ github.workflow }}-${{ github.ref_name }}-${{ github.ref_name == 'main' && github.sha || 'anysha' }}")
            .cancel_in_progress(true)
        )
        .add_env(( "CARGO_TERM_COLOR", "always" ))
        .add_env(( "RUST_BACKTRACE", 1 ))
        .add_env(( "CARGO_INCREMENTAL", 0 ));
    for job in jobs {
        workflow = workflow.add_job(job.name, job.job)
    }
    workflow.add_job(tests_pass.name, tests_pass.job)
}

/// Generates a bash script that checks changed files against regex patterns
/// and sets GitHub output variables accordingly
// fn decide_which_actions_to_run() -> String {
//     let rules = [
//         ChangeDetectionRule {
//             output_name: "run_tests",
//             pattern: r"^(docs/|script/update_top_ranking_issues/|\.github/(ISSUE_TEMPLATE|workflows/(?!ci)))",
//             invert_match: true,
//         },
//         ChangeDetectionRule {
//             output_name: "run_docs",
//             pattern: r"^docs/",
//             invert_match: false,
//         },
//         ChangeDetectionRule {
//             output_name: "run_actionlint",
//             pattern: r"^\.github/(workflows/|actions/|actionlint.yml)",
//             invert_match: false,
//         },
//         ChangeDetectionRule {
//             output_name: "run_license",
//             pattern: r"^(Cargo.lock|script/.*licenses)",
//             invert_match: false,
//         },
//         ChangeDetectionRule {
//             output_name: "run_nix",
//             pattern: r"^(nix/|flake\.|Cargo\.|rust-toolchain.toml|\.cargo/config.toml)",
//             invert_match: false,
//         },
//     ];

//     let mut script = String::new();

//     // Add the header that determines what to compare against
//     script.push_str(indoc::indoc! {r#"
//         if [ -z "$GITHUB_BASE_REF" ]; then
//           echo "Not in a PR context (i.e., push to main/stable/preview)"
//           COMPARE_REV="$(git rev-parse HEAD~1)"
//         else
//           echo "In a PR context comparing to pull_request.base.ref"
//           git fetch origin "$GITHUB_BASE_REF" --depth=350
//           COMPARE_REV="$(git merge-base "origin/${GITHUB_BASE_REF}" HEAD)"
//         fi
//         CHANGED_FILES="$(git diff --name-only "$COMPARE_REV" ${{ github.sha }})"

//         check_pattern() {
//           local output_name="$1"
//           local pattern="$2"
//           local grep_arg="$3"

//           echo "$CHANGED_FILES" | grep "$grep_arg" "$pattern" && \
//             echo "${output_name}=true" >> "$GITHUB_OUTPUT" || \
//             echo "${output_name}=false" >> "$GITHUB_OUTPUT"
//         }

//     "#});

//     // Generate a function call for each rule
//     for rule in &rules {
//         let grep_arg = if rule.invert_match { "-qvP" } else { "-qP" };
//         script.push_str(&format!(
//             "check_pattern \"{}\" '{}' {}\n",
//             rule.output_name, rule.pattern, grep_arg
//         ));
//     }

//     script
// }

pub(crate) fn tests_pass(jobs: &[NamedJob]) -> NamedJob {
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
        .cond(Expression::new(
            "github.repository_owner == 'zed-industries' && always()",
        ))
        .add_step(named::bash(&script));

    named::job(job)
}

pub(crate) fn check_style() -> NamedJob {
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
            .add_step(steps::script("./script/prettier"))
            .add_step(steps::script("./script/check-todos"))
            .add_step(steps::script("./script/check-keymaps"))
            .add_step(check_for_typos())
            .add_step(steps::cargo_fmt()),
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

pub(crate) fn run_platform_tests(platform: Platform, deps: &[&NamedJob]) -> NamedJob {
    let runner = match platform {
        Platform::Windows => runners::WINDOWS_DEFAULT,
        Platform::Linux => runners::LINUX_DEFAULT,
        Platform::Mac => runners::MAC_DEFAULT,
    };
    NamedJob {
        name: format!("run_tests_{platform}"),
        job: release_job(deps)
            .cond(Expression::new("false"))
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
