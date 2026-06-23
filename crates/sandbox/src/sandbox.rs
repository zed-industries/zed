//! Per-OS sandbox integrations for terminal commands run on behalf of the
//! agent.
//!
//! Each supported operating system has its own module here, gated behind
//! its `target_os` cfg so callers reach for the right one explicitly and
//! non-host targets don't carry dead code.
//!
//! macOS has an integration ([`macos_seatbelt`]) wrapping Apple's Seatbelt
//! / `sandbox-exec` framework, and Linux has one ([`linux_bubblewrap`]) built
//! on Bubblewrap (`bwrap`) for both the filesystem and the network. Windows
//! routes commands through WSL and runs them under Bubblewrap there (see
//! [`windows_wsl`]).

#[cfg(target_os = "linux")]
pub mod linux_bubblewrap;

#[cfg(target_os = "macos")]
pub mod macos_seatbelt;

#[cfg(target_os = "windows")]
pub mod windows_wsl;

/// Marker prefix for [`windows_wsl`] errors that mean the sandboxing
/// *environment* is unavailable (WSL missing or failing to start, no usable
/// `bwrap`, the probe/path-resolution protocol breaking down) — as opposed
/// to per-request errors such as a writable path that doesn't exist, which
/// never carry this prefix.
///
/// The agent matches on this prefix to decide whether a failed sandboxed
/// command should offer the user the option of turning sandboxing off
/// (an environment that can't sandbox at all) or simply report the error
/// back to the model (a fixable bad request). Defined here rather than in
/// [`windows_wsl`] so non-Windows builds of the agent can still reference
/// it.
pub const WSL_SANDBOX_UNAVAILABLE_PREFIX: &str = "Windows sandboxing via WSL is unavailable";

/// Per-command relaxations of the Bubblewrap-inside-WSL (Windows) sandbox.
///
/// All-false is the default, fully-sandboxed run. Setting any field requires
/// user approval before the command is launched.
///
/// Network access is a plain on/off toggle here because Bubblewrap inside WSL
/// can only enforce it wholesale (an `--unshare-net` namespace, loopback only).
/// macOS ([`macos_seatbelt::SandboxPermissions`]) and Linux
/// ([`linux_bubblewrap::SandboxPermissions`]) can additionally confine egress to
/// the in-process proxy, so they use their own richer
/// [`macos_seatbelt::NetworkAccess`]-style enum instead of this bool. Some
/// baseline operations remain denied regardless of these flags; the only way to
/// lift those is to skip the sandbox entirely, which these integrations
/// deliberately don't expose.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SandboxPermissions {
    /// Allow network access for the command.
    pub allow_network: bool,
    /// Allow unrestricted filesystem writes.
    pub allow_fs_write: bool,
}

/// Handle a possible re-exec of this binary as an in-sandbox helper.
///
/// On Linux restricted-network runs, Bubblewrap launches this binary in bridge
/// mode inside the sandbox network namespace so it can expose a loopback proxy
/// port before spawning the real command. On every other platform, and for
/// normal launches, this returns immediately.
///
/// Call this at the very top of `main`, before any argument parsing.
pub fn run_sandbox_launcher_if_invoked() {
    #[cfg(target_os = "linux")]
    linux_bubblewrap::run_launcher_if_invoked();
}
