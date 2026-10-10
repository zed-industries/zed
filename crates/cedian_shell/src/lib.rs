//! `cedian_shell`: headless app-shell models (Phase 2.5 without GPUI).
//!
//! Owns what the plan's App Shell owns minus rendering: settings TOML
//! (permissions + reviewer allow-list + update channel — the SAME files CI
//! gates, no second source of truth), palette registry (every agent action
//! discoverable), session manager (task lifecycle over OMP sessions).
//! Onboarding wizard + auto-update + keybinding UI bind with the Zed fork.

pub mod audit;
pub mod corrections;
pub mod launch;
pub mod palette;
pub mod review_agent;
pub mod review_findings;
pub mod session_manager;
pub mod settings;
pub mod state;
#[cfg(test)]
mod test_dir;
pub mod timing;
pub mod verify_store;
pub mod workflow_host;
pub mod workflow_store;
pub mod workspace_files;

pub use palette::{Palette, PaletteAction};
pub use session_manager::{SESSIONS_SNAPSHOT_VERSION, SessionEntry, SessionManager};
pub use settings::{
    CONFIG_ENV, Permissions, Policy, PolicyChoice, RunKind, SETTINGS_FILE, SETTINGS_SCHEMA,
    Settings, SettingsError, UpdateChannel, Verdict, default_settings_toml, parse_settings,
    resolve_settings,
};
