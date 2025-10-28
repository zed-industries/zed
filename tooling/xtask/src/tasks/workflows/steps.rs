use gh_workflow::*;

use crate::tasks::workflows::vars;

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

pub fn clear_target_dir_if_large() -> Step<Run> {
    named::bash("script/clear-target-dir-if-larger-than ${{ env.MAX_SIZE }}")
        .add_env(("MAX_SIZE", "${{ runner.os == 'macOS' && 300 || 100 }}"))
}

pub fn script(name: &str) -> Step<Run> {
    if name.ends_with(".ps1") {
        Step::new(name).run(name).shell(PWSH_SHELL)
    } else {
        Step::new(name).run(name).shell(BASH_SHELL)
    }
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
