use gh_workflow::*;

use crate::tasks::workflows::{runners::Platform, vars};

const BASH_SHELL: &str = "bash -euxo pipefail {0}";
// https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#jobsjob_idstepsshell
const PWSH_SHELL: &str = "pwsh";

pub fn checkout_repo() -> Step<Use> {
    named::uses(
        "actions",
        "checkout",
        "11bd71901bbe5b1630ceea73d27597364c9af683", // v4
    )
    // prevent checkout action from running `git clean -ffdx` which
    // would delete the target directory
    .add_with(("clean", false))
}

pub fn setup_pnpm() -> Step<Use> {
    named::uses(
        "pnpm",
        "action-setup",
        "fe02b34f77f8bc703788d5817da081398fad5dd2", // v4.0.0
    )
    .add_with(("version", "9"))
}

pub fn setup_node() -> Step<Use> {
    named::uses(
        "actions",
        "setup-node",
        "49933ea5288caeca8642d1e84afbd3f7d6820020", // v4
    )
    .add_with(("node-version", "20"))
}

pub fn setup_sentry() -> Step<Use> {
    named::uses(
        "matbour",
        "setup-sentry-cli",
        "3e938c54b3018bdd019973689ef984e033b0454b",
    )
    .add_with(("token", vars::SENTRY_AUTH_TOKEN))
}

pub fn cargo_fmt() -> Step<Run> {
    named::bash("cargo fmt --all -- --check")
}

pub fn cargo_install_nextest(platform: Platform) -> Step<Run> {
    named::run(platform, "cargo install cargo-nextest --locked")
}

pub fn cargo_nextest(platform: Platform) -> Step<Run> {
    named::run(
        platform,
        "cargo nextest run --workspace --no-fail-fast --failure-output immediate-final",
    )
}

pub fn setup_cargo_config(platform: Platform) -> Step<Run> {
    match platform {
        Platform::Windows => named::pwsh(indoc::indoc! {r#"
            New-Item -ItemType Directory -Path "./../.cargo" -Force
            Copy-Item -Path "./.cargo/ci-config.toml" -Destination "./../.cargo/config.toml"
        "#}),

        Platform::Linux | Platform::Mac => named::bash(indoc::indoc! {r#"
            mkdir -p ./../.cargo
            cp ./.cargo/ci-config.toml ./../.cargo/config.toml
        "#}),
    }
}

pub fn cleanup_cargo_config(platform: Platform) -> Step<Run> {
    let step = match platform {
        Platform::Windows => named::pwsh(indoc::indoc! {r#"
            Remove-Item -Recurse -Path "./../.cargo" -Force -ErrorAction SilentlyContinue
        "#}),
        Platform::Linux | Platform::Mac => named::bash(indoc::indoc! {r#"
            rm -rf ./../.cargo
        "#}),
    };

    step.if_condition(Expression::new("always()"))
}

pub fn upload_artifact(name: &str, path: &str) -> Step<Use> {
    Step::new(format!("@actions/upload-artifact {}", name))
        .uses(
            "actions",
            "upload-artifact",
            "330a01c490aca151604b8cf639adc76d48f6c5d4", // v5
        )
        .add_with(("name", name))
        .add_with(("path", path))
}

pub fn clear_target_dir_if_large(platform: Platform) -> Step<Run> {
    match platform {
        Platform::Windows => named::pwsh("./script/clear-target-dir-if-larger-than.ps1 250"),
        Platform::Linux => named::bash("./script/clear-target-dir-if-larger-than 100"),
        Platform::Mac => named::bash("./script/clear-target-dir-if-larger-than 300"),
    }
}

pub fn script(name: &str) -> Step<Run> {
    if name.ends_with(".ps1") {
        Step::new(name).run(name).shell(PWSH_SHELL)
    } else {
        Step::new(name).run(name).shell(BASH_SHELL)
    }
}

pub(crate) struct NamedJob {
    pub name: String,
    pub job: Job,
}

// (janky) helper to generate steps with a name that corresponds
// to the name of the calling function.
pub(crate) mod named {
    use super::*;

    /// Returns a uses step with the same name as the enclosing function.
    /// (You shouldn't inline this function into the workflow definition, you must
    /// wrap it in a new function.)
    pub(crate) fn uses(owner: &str, repo: &str, ref_: &str) -> Step<Use> {
        Step::new(function_name(1)).uses(owner, repo, ref_)
    }

    /// Returns a bash-script step with the same name as the enclosing function.
    /// (You shouldn't inline this function into the workflow definition, you must
    /// wrap it in a new function.)
    pub(crate) fn bash(script: &str) -> Step<Run> {
        Step::new(function_name(1)).run(script).shell(BASH_SHELL)
    }

    /// Returns a pwsh-script step with the same name as the enclosing function.
    /// (You shouldn't inline this function into the workflow definition, you must
    /// wrap it in a new function.)
    pub(crate) fn pwsh(script: &str) -> Step<Run> {
        Step::new(function_name(1)).run(script).shell(PWSH_SHELL)
    }

    /// Runs the command in either powershell or bash, depending on platform.
    /// (You shouldn't inline this function into the workflow definition, you must
    /// wrap it in a new function.)
    pub(crate) fn run(platform: Platform, script: &str) -> Step<Run> {
        match platform {
            Platform::Windows => Step::new(function_name(1)).run(script).shell(PWSH_SHELL),
            Platform::Linux | Platform::Mac => {
                Step::new(function_name(1)).run(script).shell(BASH_SHELL)
            }
        }
    }

    /// Returns a Workflow with the same name as the enclosing module.
    pub(crate) fn workflow() -> Workflow {
        Workflow::default().name(
            named::function_name(1)
                .split("::")
                .next()
                .unwrap()
                .to_owned(),
        )
    }

    /// Returns a Job with the same name as the enclosing function.
    /// (note job names may not contain `::`)
    pub(crate) fn job(job: Job) -> NamedJob {
        NamedJob {
            name: function_name(1).split("::").last().unwrap().to_owned(),
            job,
        }
    }

    /// Returns the function name N callers above in the stack
    /// (typically 1).
    /// This only works because xtask always runs debug builds.
    pub(crate) fn function_name(i: usize) -> String {
        let mut name = "<unknown>".to_string();
        let mut count = 0;
        backtrace::trace(|frame| {
            if count < i + 3 {
                count += 1;
                return true;
            }
            backtrace::resolve_frame(frame, |cb| {
                if let Some(s) = cb.name() {
                    name = s.to_string()
                }
            });
            false
        });
        name.split("::")
            .skip_while(|s| s != &"workflows")
            .skip(1)
            .collect::<Vec<_>>()
            .join("::")
    }
}
