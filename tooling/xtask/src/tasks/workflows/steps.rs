use gh_workflow::*;

use crate::tasks::workflows::{
    runners::{Arch, Platform},
    vars,
    vars::StepOutput,
};

const SCCACHE_R2_BUCKET: &str = "sccache-zed";
const CARGO_MTIME_VERSION: &str = "v0.1.2";

const BASH_SHELL: &str = "bash -euxo pipefail {0}";
// https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#jobsjob_idstepsshell
pub const PWSH_SHELL: &str = "pwsh";

pub(crate) struct Nextest(Step<Run>);

pub(crate) fn cargo_nextest(platform: Platform) -> Nextest {
    Nextest(named::run(
        platform,
        "cargo nextest run --workspace --no-fail-fast",
    ))
}

impl Nextest {
    pub(crate) fn with_target(mut self, target: &str) -> Step<Run> {
        if let Some(nextest_command) = self.0.value.run.as_mut() {
            nextest_command.push_str(&format!(r#" --target "{target}""#));
        }
        self.into()
    }

    #[allow(dead_code)]
    pub(crate) fn with_filter_expr(mut self, filter_expr: &str) -> Self {
        if let Some(nextest_command) = self.0.value.run.as_mut() {
            nextest_command.push_str(&format!(r#" -E "{filter_expr}""#));
        }
        self
    }

    pub(crate) fn with_changed_packages_filter(mut self, orchestrate_job: &str) -> Self {
        if let Some(nextest_command) = self.0.value.run.as_mut() {
            nextest_command.push_str(&format!(
                r#"${{{{ needs.{orchestrate_job}.outputs.changed_packages && format(' -E "{{0}}"', needs.{orchestrate_job}.outputs.changed_packages) || '' }}}}"#
            ));
        }
        self
    }
}

impl From<Nextest> for Step<Run> {
    fn from(value: Nextest) -> Self {
        value.0
    }
}

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

pub fn checkout_repo_with_token(token: &StepOutput) -> Step<Use> {
    named::uses(
        "actions",
        "checkout",
        "11bd71901bbe5b1630ceea73d27597364c9af683", // v4
    )
    .add_with(("clean", false))
    .add_with(("token", token.to_string()))
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

pub fn prettier() -> Step<Run> {
    named::bash("./script/prettier")
}

pub fn cargo_fmt() -> Step<Run> {
    named::bash("cargo fmt --all -- --check")
}

pub fn cargo_install_nextest() -> Step<Use> {
    named::uses("taiki-e", "install-action", "nextest")
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
    named::uses("namespacelabs", "nscloud-cache-action", "v1")
        .add_with(("cache", "rust"))
        .add_with(("path", "~/.rustup"))
}

fn cargo_mtime_target(platform: Platform, arch: Arch) -> &'static str {
    match (platform, arch) {
        (Platform::Linux, Arch::X86_64) => "x86_64-unknown-linux-musl",
        (Platform::Linux, Arch::AARCH64) => "aarch64-unknown-linux-musl",
        (Platform::Mac, Arch::X86_64) => "x86_64-apple-darwin",
        (Platform::Mac, Arch::AARCH64) => "aarch64-apple-darwin",
        (Platform::Windows, Arch::X86_64) => "x86_64-pc-windows-msvc",
        (Platform::Windows, Arch::AARCH64) => "aarch64-pc-windows-msvc",
    }
}

fn default_runner_arch(platform: Platform) -> Arch {
    match platform {
        Platform::Linux => Arch::X86_64,
        Platform::Mac => Arch::AARCH64,
        Platform::Windows => Arch::X86_64,
    }
}

pub fn install_cargo_mtime(platform: Platform) -> Step<Run> {
    install_cargo_mtime_for_arch(platform, default_runner_arch(platform))
}

pub fn install_cargo_mtime_for_arch(platform: Platform, arch: Arch) -> Step<Run> {
    let target = cargo_mtime_target(platform, arch);
    let url = format!(
        "https://github.com/zed-industries/cargo-mtime/releases/download/{}/cargo-mtime-{}-{}.tar.gz",
        CARGO_MTIME_VERSION, target, CARGO_MTIME_VERSION
    );
    match platform {
        Platform::Windows => Step::new("install_cargo_mtime")
            .run(format!(
                "New-Item -ItemType Directory -Path \"target\" -Force\n\
             Invoke-WebRequest -Uri \"{url}\" -OutFile \"target/cargo-mtime.tar.gz\"\n\
             tar xzf target/cargo-mtime.tar.gz -C target\n\
             Remove-Item target/cargo-mtime.tar.gz"
            ))
            .shell(PWSH_SHELL),
        Platform::Linux | Platform::Mac => Step::new("install_cargo_mtime").run(format!(
            "mkdir -p target\n\
             curl -sL \"{url}\" | tar xz -C target"
        )),
    }
}

pub fn run_cargo_mtime(platform: Platform) -> Step<Run> {
    match platform {
        Platform::Windows => Step::new("run_cargo_mtime")
            .run("./target/cargo-mtime.exe . target/cargo-mtime.db")
            .shell(PWSH_SHELL),
        Platform::Linux | Platform::Mac => {
            Step::new("run_cargo_mtime").run("./target/cargo-mtime . target/cargo-mtime.db")
        }
    }
}

pub fn setup_sccache(platform: Platform) -> Step<Run> {
    let step = match platform {
        Platform::Windows => named::pwsh("./script/setup-sccache.ps1"),
        Platform::Linux | Platform::Mac => named::bash("./script/setup-sccache"),
    };
    step.add_env(("R2_ACCOUNT_ID", vars::R2_ACCOUNT_ID))
        .add_env(("R2_ACCESS_KEY_ID", vars::R2_ACCESS_KEY_ID))
        .add_env(("R2_SECRET_ACCESS_KEY", vars::R2_SECRET_ACCESS_KEY))
        .add_env(("SCCACHE_BUCKET", SCCACHE_R2_BUCKET))
}

pub fn show_sccache_stats(platform: Platform) -> Step<Run> {
    match platform {
        // Use $env:RUSTC_WRAPPER (absolute path) because GITHUB_PATH changes
        // don't take effect until the next step in PowerShell.
        // Check if RUSTC_WRAPPER is set first (it won't be for fork PRs without secrets).
        Platform::Windows => {
            named::pwsh("if ($env:RUSTC_WRAPPER) { & $env:RUSTC_WRAPPER --show-stats }; exit 0")
        }
        Platform::Linux | Platform::Mac => named::bash("sccache --show-stats || true"),
    }
}

pub fn cache_nix_dependencies_namespace() -> Step<Use> {
    named::uses("namespacelabs", "nscloud-cache-action", "v1").add_with(("cache", "nix"))
}

pub fn cache_nix_store_macos() -> Step<Use> {
    // On macOS, `/nix` is on a read-only root filesystem so nscloud's `cache: nix`
    // cannot mount or symlink there. Instead we cache a user-writable directory and
    // use nix-store --import/--export in separate steps to transfer store paths.
    named::uses("namespacelabs", "nscloud-cache-action", "v1").add_with(("path", "~/nix-cache"))
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
        Step::new(name).run(name)
    }
}

pub struct NamedJob<J: JobType = RunJob> {
    pub name: String,
    pub job: Job<J>,
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
impl FluentBuilder for Input {}

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
        Step::new(function_name(1)).run(script.as_ref())
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
            Platform::Linux | Platform::Mac => Step::new(function_name(1)).run(script),
        }
    }

    /// Returns a Workflow with the same name as the enclosing module with default
    /// set for the running shell.
    pub fn workflow() -> Workflow {
        Workflow::default()
            .name(
                named::function_name(1)
                    .split("::")
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .skip(1)
                    .rev()
                    .collect::<Vec<_>>()
                    .join("::"),
            )
            .defaults(Defaults::default().run(RunDefaults::default().shell(BASH_SHELL)))
    }

    /// Returns a Job with the same name as the enclosing function.
    /// (note job names may not contain `::`)
    pub fn job<J: JobType>(job: Job<J>) -> NamedJob<J> {
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

pub fn authenticate_as_zippy() -> (Step<Use>, StepOutput) {
    let step = named::uses(
        "actions",
        "create-github-app-token",
        "bef1eaf1c0ac2b148ee2a0a74c65fbe6db0631f1",
    )
    .add_with(("app-id", vars::ZED_ZIPPY_APP_ID))
    .add_with(("private-key", vars::ZED_ZIPPY_APP_PRIVATE_KEY))
    .id("get-app-token");
    let output = StepOutput::new(&step, "token");
    (step, output)
}
