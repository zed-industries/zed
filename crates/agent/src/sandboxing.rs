//! Agent-side glue for the [`sandbox`] crate.
//!
//! Centralizes the "should agent-run terminal commands be sandboxed for this
//! process?" check so the system prompt, the terminal tool, and any other
//! caller see the same answer (and so the `target_os` gate lives in one
//! place instead of scattered across the agent crate).
//!
//! The current policy is: enabled iff we're on macOS *and* the user has the
//! `sandboxing` feature flag turned on. There's deliberately no settings or
//! env-var override yet — the flag is the only switch.
//!
//! On non-macOS hosts we don't have a sandbox integration today, so this
//! returns `false` regardless of the flag.
//!
//! Naming note: this module is about agent terminal sandboxing specifically.
//! Other agent operations (e.g. file edits) are gated separately.

use feature_flags::{FeatureFlagAppExt as _, SandboxingFeatureFlag};
use gpui::App;

/// Whether agent-run terminal commands should be wrapped in an OS-level
/// sandbox for this process. See module docs for the policy.
pub(crate) fn sandboxing_enabled(cx: &App) -> bool {
    cfg!(target_os = "macos") && cx.has_flag::<SandboxingFeatureFlag>()
}
