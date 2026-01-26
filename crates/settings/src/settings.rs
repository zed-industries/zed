mod base_keymap_setting;
mod content_into_gpui;
mod editable_setting_control;
mod editorconfig_store;
mod keymap_file;
mod settings_file;
mod settings_store;
mod vscode_import;

pub use settings_macros::RegisterSetting;

pub mod settings_content {
    pub use ::settings_content::*;
}

pub mod fallible_options {
    pub use ::settings_content::{FallibleOption, parse_json};
}

#[doc(hidden)]
pub mod private {
    pub use crate::settings_store::{RegisteredSetting, SettingValue};
    pub use inventory;
}

use gpui::{App, Global};
use release_channel::ReleaseChannel;
use rust_embed::RustEmbed;
use std::env;
use std::{borrow::Cow, fmt, str};
use util::asset_str;

pub use ::settings_content::*;
pub use base_keymap_setting::*;
pub use content_into_gpui::IntoGpui;
pub use editable_setting_control::*;
pub use editorconfig_store::{
    Editorconfig, EditorconfigEvent, EditorconfigProperties, EditorconfigStore,
};
pub use keymap_file::{
    KeyBindingValidator, KeyBindingValidatorRegistration, KeybindSource, KeybindUpdateOperation,
    KeybindUpdateTarget, KeymapFile, KeymapFileLoadResult,
};
pub use settings_file::*;
pub use settings_json::*;
pub use settings_store::{
    InvalidSettingsError, LSP_SETTINGS_SCHEMA_URL_PREFIX, LocalSettingsKind, LocalSettingsPath,
    MigrationStatus, Settings, SettingsFile, SettingsJsonSchemaParams, SettingsKey,
    SettingsLocation, SettingsParseResult, SettingsStore,
};

pub use vscode_import::{VsCodeSettings, VsCodeSettingsSource};

pub use keymap_file::ActionSequence;

#[derive(Clone, Debug, PartialEq)]
pub struct ActiveSettingsProfileName(pub String);

impl Global for ActiveSettingsProfileName {}

pub trait UserSettingsContentExt {
    fn for_profile(&self, cx: &App) -> Option<&SettingsContent>;
    fn for_release_channel(&self) -> Option<&SettingsContent>;
    fn for_os(&self) -> Option<&SettingsContent>;
}

impl UserSettingsContentExt for UserSettingsContent {
    fn for_profile(&self, cx: &App) -> Option<&SettingsContent> {
        let Some(active_profile) = cx.try_global::<ActiveSettingsProfileName>() else {
            return None;
        };
        self.profiles.get(&active_profile.0)
    }

    fn for_release_channel(&self) -> Option<&SettingsContent> {
        match *release_channel::RELEASE_CHANNEL {
            ReleaseChannel::Dev => self.dev.as_deref(),
            ReleaseChannel::Nightly => self.nightly.as_deref(),
            ReleaseChannel::Preview => self.preview.as_deref(),
            ReleaseChannel::Stable => self.stable.as_deref(),
        }
    }

    fn for_os(&self) -> Option<&SettingsContent> {
        match env::consts::OS {
            "macos" => self.macos.as_deref(),
            "linux" => self.linux.as_deref(),
            "windows" => self.windows.as_deref(),
            _ => None,
        }
    }
}

/// A unique identifier for a worktree within Zed.
///
/// A worktree represents a root directory in a project. The `WorktreeId` combines
/// a local entity ID with a project ID to ensure global uniqueness across remote
/// project connections.
///
/// # Project ID
///
/// The `project_id` field distinguishes worktrees from different remote projects:
///
/// - **Local worktrees**: Use `project_id = 0` (via [`WorktreeId::local`]).
/// - **Remote server worktrees**: Use `project_id = 0` (the constant
///   `rpc::proto::REMOTE_SERVER_PROJECT_ID`).
/// - **Collab project worktrees**: Use the actual project ID assigned by the
///   collaboration server.
///
/// This distinction is necessary because the local `id` (derived from the entity ID)
/// is only unique within a single project context. When connecting to multiple
/// remote projects simultaneously, two worktrees from different projects could
/// have the same local ID, so the `project_id` is needed to disambiguate them.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord, serde::Serialize)]
pub struct WorktreeId {
    /// The local worktree identifier, typically derived from the entity ID.
    id: usize,
    /// The project ID this worktree belongs to. Zero for local and remote server projects.
    project_id: u64,
}

impl From<WorktreeId> for usize {
    fn from(value: WorktreeId) -> Self {
        value.id
    }
}

impl WorktreeId {
    /// Creates a `WorktreeId` for a local worktree.
    ///
    /// This is a convenience constructor that sets `project_id = 0`, which is
    /// appropriate for worktrees that are not part of a remote collaboration session.
    pub fn local(id: usize) -> Self {
        Self { id, project_id: 0 }
    }

    /// Creates a `WorktreeId` from a handle ID and project ID.
    ///
    /// Use this when you have both the local worktree ID and know which project
    /// it belongs to. For local-only worktrees, prefer [`WorktreeId::local`].
    pub fn from_usize(handle_id: usize, project_id: u64) -> Self {
        Self {
            id: handle_id,
            project_id,
        }
    }

    /// Creates a `WorktreeId` from protobuf message fields.
    ///
    /// This is used when deserializing worktree IDs from RPC messages.
    pub fn from_proto(id: u64, project_id: u64) -> Self {
        Self {
            id: id as usize,
            project_id,
        }
    }

    /// Converts the local ID to the protobuf representation.
    ///
    /// Note: This only returns the local `id` field, not the `project_id`.
    /// The project ID is typically sent separately in protobuf messages.
    pub fn to_proto(self) -> u64 {
        self.id as u64
    }

    /// Returns the local worktree ID as a `usize`.
    pub fn to_usize(self) -> usize {
        self.id
    }

    /// Returns the project ID this worktree belongs to.
    ///
    /// Returns `0` for local worktrees and remote server connections.
    pub fn project_id(self) -> u64 {
        self.project_id
    }
}

impl fmt::Display for WorktreeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.project_id, self.id)
    }
}

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "settings/*"]
#[include = "keymaps/*"]
#[exclude = "*.DS_Store"]
pub struct SettingsAssets;

pub fn init(cx: &mut App) {
    let settings = SettingsStore::new(cx, &default_settings());
    cx.set_global(settings);
    SettingsStore::observe_active_settings_profile_name(cx).detach();
}

pub fn default_settings() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/default.json")
}

#[cfg(target_os = "macos")]
pub const DEFAULT_KEYMAP_PATH: &str = "keymaps/default-macos.json";

#[cfg(target_os = "windows")]
pub const DEFAULT_KEYMAP_PATH: &str = "keymaps/default-windows.json";

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub const DEFAULT_KEYMAP_PATH: &str = "keymaps/default-linux.json";

pub fn default_keymap() -> Cow<'static, str> {
    asset_str::<SettingsAssets>(DEFAULT_KEYMAP_PATH)
}

pub const VIM_KEYMAP_PATH: &str = "keymaps/vim.json";

pub fn vim_keymap() -> Cow<'static, str> {
    asset_str::<SettingsAssets>(VIM_KEYMAP_PATH)
}

pub fn initial_user_settings_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/initial_user_settings.json")
}

pub fn initial_server_settings_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/initial_server_settings.json")
}

pub fn initial_project_settings_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/initial_local_settings.json")
}

pub fn initial_keymap_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("keymaps/initial.json")
}

pub fn initial_tasks_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/initial_tasks.json")
}

pub fn initial_debug_tasks_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/initial_debug_tasks.json")
}

pub fn initial_local_debug_tasks_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/initial_local_debug_tasks.json")
}
