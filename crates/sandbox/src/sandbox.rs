//! Per-OS sandbox integrations for terminal commands run on behalf of the
//! agent.
//!
//! Each supported operating system has its own module here, gated behind
//! its `target_os` cfg so callers reach for the right one explicitly and
//! non-host targets don't carry dead code.
//!
//! macOS has [`macos_seatbelt`], wrapping Apple's Seatbelt / `sandbox-exec`
//! framework; Windows has [`windows_appcontainer`], wrapping AppContainer
//! profiles and DACL grants.

#[cfg(target_os = "macos")]
pub mod macos_seatbelt;

#[cfg(target_os = "windows")]
pub mod windows_appcontainer;
