//! Per-OS sandbox integrations for terminal commands run on behalf of the
//! agent.
//!
//! Each supported operating system has its own module here, gated behind
//! its `target_os` cfg so callers reach for the right one explicitly and
//! non-host targets don't carry dead code.
//!
//! macOS has an integration ([`macos_seatbelt`]) wrapping Apple's Seatbelt
//! / `sandbox-exec` framework, and Linux has a basic integration
//! ([`linux_landlock`]) built on the Landlock LSM.

#[cfg(target_os = "linux")]
pub mod linux_landlock;

#[cfg(target_os = "macos")]
pub mod macos_seatbelt;

/// Per-command relaxations of the default sandbox.
///
/// All-false is the default, fully-sandboxed run. Setting any field
/// requires user approval before the command is launched.
///
/// This is the platform-independent request. Each OS integration maps it
/// onto its own mechanism and may enforce it with different granularity
/// (for example, network restriction is coarser under Linux's Landlock than
/// under macOS's Seatbelt). Some baseline operations remain denied
/// regardless of these flags; the only way to lift those is to skip the
/// sandbox entirely, which these integrations deliberately don't expose.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SandboxPermissions {
    /// Allow network access for the command.
    pub allow_network: bool,
    /// Allow unrestricted filesystem writes.
    pub allow_fs_write: bool,
}

/// Handle a possible re-exec of this binary as a sandbox launcher.
///
/// On Linux, the terminal integration sandboxes commands by re-executing
/// this binary as a Landlock launcher (see
/// [`linux_landlock::run_launcher_if_invoked`]); when that marker is present
/// this applies the ruleset and `exec`s the wrapped command, never
/// returning. On every other platform, and for normal launches, it returns
/// immediately.
///
/// Call this at the very top of `main`, before any argument parsing.
pub fn run_sandbox_launcher_if_invoked() {
    #[cfg(target_os = "linux")]
    linux_landlock::run_launcher_if_invoked();
}
