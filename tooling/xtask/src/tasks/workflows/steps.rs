use gh_workflow::*;

use crate::tasks::workflows::{runners::Platform, vars};

pub const BASH_SHELL: &str = "bash -euxo pipefail {0}";
// https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#jobsjob_idstepsshell
pub const PWSH_SHELL: &str = "pwsh";

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

pub fn cargo_install_nextest() -> Step<Use> {
    named::uses("taiki-e", "install-action", "nextest")
}

pub fn cargo_nextest(platform: Platform) -> Step<Run> {
    named::run(platform, "cargo nextest run --workspace --no-fail-fast")
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

pub fn clear_target_dir_if_large(platform: Platform) -> Step<Run> {
    match platform {
        Platform::Windows => named::pwsh("./script/clear-target-dir-if-larger-than.ps1 250"),
        Platform::Linux => named::bash("./script/clear-target-dir-if-larger-than 250"),
        Platform::Mac => named::bash("./script/clear-target-dir-if-larger-than 300"),
    }
}

pub fn clippy(platform: Platform) -> Step<Run> {
    match platform {
        Platform::Windows => named::pwsh("./script/clippy.ps1"),
        _ => named::bash("./script/clippy"),
    }
}

pub fn cache_rust_dependencies_namespace() -> Step<Use> {
    named::uses("namespacelabs", "nscloud-cache-action", "v1").add_with(("cache", "rust"))
}

pub fn setup_linux() -> Step<Run> {
    named::bash("./script/linux")
}

fn install_mold() -> Step<Run> {
    named::bash("./script/install-mold")
}

fn download_wasi_sdk() -> Step<Run> {
    named::bash("./script/download-wasi-sdk")
}

pub(crate) fn install_linux_dependencies(job: Job) -> Job {
    job.add_step(setup_linux())
        .add_step(install_mold())
        .add_step(download_wasi_sdk())
}

pub fn script(name: &str) -> Step<Run> {
    if name.ends_with(".ps1") {
        Step::new(name).run(name).shell(PWSH_SHELL)
    } else {
        Step::new(name).run(name).shell(BASH_SHELL)
    }
}

pub struct NamedJob {
    pub name: String,
    pub job: Job,
}

// impl NamedJob {
//     pub fn map(self, f: impl FnOnce(Job) -> Job) -> Self {
//         NamedJob {
//             name: self.name,
//             job: f(self.job),
//         }
//     }
// }

pub(crate) const DEFAULT_REPOSITORY_OWNER_GUARD: &str =
    "(github.repository_owner == 'zed-industries' || github.repository_owner == 'zed-extensions')";

pub fn repository_owner_guard_expression(trigger_always: bool) -> Expression {
    Expression::new(format!(
        "{}{}",
        DEFAULT_REPOSITORY_OWNER_GUARD,
        trigger_always.then_some(" && always()").unwrap_or_default()
    ))
}

pub trait CommonJobConditions: Sized {
    fn with_repository_owner_guard(self) -> Self;
}

impl CommonJobConditions for Job {
    fn with_repository_owner_guard(self) -> Self {
        self.cond(repository_owner_guard_expression(false))
    }
}

pub(crate) fn release_job(deps: &[&NamedJob]) -> Job {
    dependant_job(deps)
        .with_repository_owner_guard()
        .timeout_minutes(60u32)
}

pub(crate) fn dependant_job(deps: &[&NamedJob]) -> Job {
    let job = Job::default();
    if deps.len() > 0 {
        job.needs(deps.iter().map(|j| j.name.clone()).collect::<Vec<_>>())
    } else {
        job
    }
}

impl FluentBuilder for Job {}
impl FluentBuilder for Workflow {}

/// A helper trait for building complex objects with imperative conditionals in a fluent style.
/// Copied from GPUI to avoid adding GPUI as dependency
/// todo(ci) just put this in gh-workflow
#[allow(unused)]
pub trait FluentBuilder {
    /// Imperatively modify self with the given closure.
    fn map<U>(self, f: impl FnOnce(Self) -> U) -> U
    where
        Self: Sized,
    {
        f(self)
    }

    /// Conditionally modify self with the given closure.
    fn when(self, condition: bool, then: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        self.map(|this| if condition { then(this) } else { this })
    }

    /// Conditionally modify self with the given closure.
    fn when_else(
        self,
        condition: bool,
        then: impl FnOnce(Self) -> Self,
        else_fn: impl FnOnce(Self) -> Self,
    ) -> Self
    where
        Self: Sized,
    {
        self.map(|this| if condition { then(this) } else { else_fn(this) })
    }

    /// Conditionally unwrap and modify self with the given closure, if the given option is Some.
    fn when_some<T>(self, option: Option<T>, then: impl FnOnce(Self, T) -> Self) -> Self
    where
        Self: Sized,
    {
        self.map(|this| {
            if let Some(value) = option {
                then(this, value)
            } else {
                this
            }
        })
    }
    /// Conditionally unwrap and modify self with the given closure, if the given option is None.
    fn when_none<T>(self, option: &Option<T>, then: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        self.map(|this| if option.is_some() { this } else { then(this) })
    }
}

// (janky) helper to generate steps with a name that corresponds
// to the name of the calling function.
pub mod named {
    use super::*;

    /// Returns a uses step with the same name as the enclosing function.
    /// (You shouldn't inline this function into the workflow definition, you must
    /// wrap it in a new function.)
    pub fn uses(owner: &str, repo: &str, ref_: &str) -> Step<Use> {
        Step::new(function_name(1)).uses(owner, repo, ref_)
    }

    /// Returns a bash-script step with the same name as the enclosing function.
    /// (You shouldn't inline this function into the workflow definition, you must
    /// wrap it in a new function.)
    pub fn bash(script: impl AsRef<str>) -> Step<Run> {
        Step::new(function_name(1))
            .run(script.as_ref())
            .shell(BASH_SHELL)
    }

    /// Returns a pwsh-script step with the same name as the enclosing function.
    /// (You shouldn't inline this function into the workflow definition, you must
    /// wrap it in a new function.)
    pub fn pwsh(script: &str) -> Step<Run> {
        Step::new(function_name(1)).run(script).shell(PWSH_SHELL)
    }

    /// Runs the command in either powershell or bash, depending on platform.
    /// (You shouldn't inline this function into the workflow definition, you must
    /// wrap it in a new function.)
    pub fn run(platform: Platform, script: &str) -> Step<Run> {
        match platform {
            Platform::Windows => Step::new(function_name(1)).run(script).shell(PWSH_SHELL),
            Platform::Linux | Platform::Mac => {
                Step::new(function_name(1)).run(script).shell(BASH_SHELL)
            }
        }
    }

    /// Returns a Workflow with the same name as the enclosing module.
    pub fn workflow() -> Workflow {
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
    pub fn job(job: Job) -> NamedJob {
        NamedJob {
            name: function_name(1).split("::").last().unwrap().to_owned(),
            job,
        }
    }

    /// Returns the function name N callers above in the stack
    /// (typically 1).
    /// This only works because xtask always runs debug builds.
    pub fn function_name(i: usize) -> String {
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

pub fn git_checkout(ref_name: &dyn std::fmt::Display) -> Step<Run> {
    named::bash(&format!(
        "git fetch origin {ref_name} && git checkout {ref_name}"
    ))
}
