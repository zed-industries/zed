use std::{num::NonZeroUsize, path::PathBuf, sync::Arc, time::Duration};

use crate::DockPosition;
use anyhow::Context as _;
use collections::HashMap;
use fs::Fs;
use gpui::{App, AppContext as _, Task};
use serde::Deserialize;
use util::{ResultExt as _, paths::SanitizedPath};
pub use settings::{
    ActionName, AutosaveSetting, BottomDockLayout, EncodingDisplayOptions, InactiveOpacity,
    PaneSplitDirectionHorizontal, PaneSplitDirectionVertical, RegisterSetting,
    RestoreOnStartupBehavior, Settings,
};

#[derive(RegisterSetting)]
pub struct WorkspaceSettings {
    pub active_pane_modifiers: ActivePanelModifiers,
    pub bottom_dock_layout: settings::BottomDockLayout,
    pub pane_split_direction_horizontal: settings::PaneSplitDirectionHorizontal,
    pub pane_split_direction_vertical: settings::PaneSplitDirectionVertical,
    pub centered_layout: settings::CenteredLayoutSettings,
    pub confirm_quit: bool,
    pub show_call_status_icon: bool,
    pub autosave: AutosaveSetting,
    pub restore_on_startup: settings::RestoreOnStartupBehavior,
    pub cli_default_open_behavior: settings::CliDefaultOpenBehavior,
    pub restore_on_file_reopen: bool,
    pub drop_target_size: f32,
    pub use_system_path_prompts: bool,
    pub default_project_folder: Option<String>,
    pub use_system_prompts: bool,
    pub command_aliases: HashMap<String, ActionName>,
    pub max_tabs: Option<NonZeroUsize>,
    pub when_closing_with_no_tabs: settings::CloseWindowWhenNoItems,
    pub on_last_window_closed: settings::OnLastWindowClosed,
    pub text_rendering_mode: settings::TextRenderingMode,
    pub resize_all_panels_in_dock: Vec<DockPosition>,
    pub close_on_file_delete: bool,
    pub close_panel_on_toggle: bool,
    pub use_system_window_tabs: bool,
    pub zoomed_padding: bool,
    pub window_decorations: settings::WindowDecorations,
    pub focus_follows_mouse: FocusFollowsMouse,
}

#[derive(Copy, Clone, Deserialize)]
pub struct FocusFollowsMouse {
    pub enabled: bool,
    pub debounce: Duration,
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct ActivePanelModifiers {
    /// Size of the border surrounding the active pane.
    /// When set to 0, the active pane doesn't have any border.
    /// The border is drawn inset.
    ///
    /// Default: `0.0`
    // TODO: make this not an option, it is never None
    pub border_size: Option<f32>,
    /// Opacity of inactive panels.
    /// When set to 1.0, the inactive panes have the same opacity as the active one.
    /// If set to 0, the inactive panes content will not be visible at all.
    /// Values are clamped to the [0.0, 1.0] range.
    ///
    /// Default: `1.0`
    // TODO: make this not an option, it is never None
    pub inactive_opacity: Option<InactiveOpacity>,
}

#[derive(Deserialize, RegisterSetting)]
pub struct TabBarSettings {
    pub show: bool,
    pub show_nav_history_buttons: bool,
    pub show_tab_bar_buttons: bool,
    pub show_pinned_tabs_in_separate_row: bool,
}

impl Settings for WorkspaceSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let workspace = &content.workspace;
        Self {
            active_pane_modifiers: ActivePanelModifiers {
                border_size: Some(
                    workspace
                        .active_pane_modifiers
                        .unwrap()
                        .border_size
                        .unwrap(),
                ),
                inactive_opacity: Some(
                    workspace
                        .active_pane_modifiers
                        .unwrap()
                        .inactive_opacity
                        .unwrap(),
                ),
            },
            bottom_dock_layout: workspace.bottom_dock_layout.unwrap(),
            pane_split_direction_horizontal: workspace.pane_split_direction_horizontal.unwrap(),
            pane_split_direction_vertical: workspace.pane_split_direction_vertical.unwrap(),
            centered_layout: workspace.centered_layout.unwrap(),
            confirm_quit: workspace.confirm_quit.unwrap(),
            show_call_status_icon: workspace.show_call_status_icon.unwrap(),
            autosave: workspace.autosave.unwrap(),
            restore_on_startup: workspace.restore_on_startup.unwrap(),
            cli_default_open_behavior: workspace.cli_default_open_behavior.unwrap(),
            restore_on_file_reopen: workspace.restore_on_file_reopen.unwrap(),
            drop_target_size: workspace.drop_target_size.unwrap(),
            use_system_path_prompts: workspace.use_system_path_prompts.unwrap(),
            default_project_folder: workspace.default_project_folder.clone(),
            use_system_prompts: workspace.use_system_prompts.unwrap(),
            command_aliases: workspace.command_aliases.clone(),
            max_tabs: workspace.max_tabs,
            when_closing_with_no_tabs: workspace.when_closing_with_no_tabs.unwrap(),
            on_last_window_closed: workspace.on_last_window_closed.unwrap(),
            text_rendering_mode: workspace.text_rendering_mode.unwrap(),
            resize_all_panels_in_dock: workspace
                .resize_all_panels_in_dock
                .clone()
                .unwrap()
                .into_iter()
                .map(Into::into)
                .collect(),
            close_on_file_delete: workspace.close_on_file_delete.unwrap(),
            close_panel_on_toggle: workspace.close_panel_on_toggle.unwrap(),
            use_system_window_tabs: workspace.use_system_window_tabs.unwrap(),
            zoomed_padding: workspace.zoomed_padding.unwrap(),
            window_decorations: workspace.window_decorations.unwrap(),
            focus_follows_mouse: FocusFollowsMouse {
                enabled: workspace
                    .focus_follows_mouse
                    .unwrap()
                    .enabled
                    .unwrap_or(false),
                debounce: Duration::from_millis(
                    workspace
                        .focus_follows_mouse
                        .unwrap()
                        .debounce_ms
                        .unwrap_or(250),
                ),
            },
        }
    }
}

/// Resolves the user-configured default folder for "Open a new project" into a
/// canonicalized, sanitized absolute path. Both the system file dialog and the
/// in-app keyboard picker consult this single accessor so the override is
/// determined in exactly one place.
///
/// Returns `Task<None>` when the setting is unset, the path does not exist, or
/// the path does not point at a directory. All filesystem work happens
/// asynchronously through the supplied [`Fs`] handle.
pub fn default_open_path(fs: Arc<dyn Fs>, cx: &App) -> Task<Option<PathBuf>> {
    let raw = WorkspaceSettings::get_global(cx)
        .default_project_folder
        .clone();
    cx.background_spawn(async move {
        let raw = raw?;
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return None;
        }
        let expanded = PathBuf::from(shellexpand::tilde(trimmed).into_owned());
        let canonical = fs
            .canonicalize(&expanded)
            .await
            .with_context(|| format!("canonicalizing default_project_folder {expanded:?}"))
            .log_err()?;
        if !fs.is_dir(&canonical).await {
            log::warn!("default_project_folder {expanded:?} is not a directory; ignoring");
            return None;
        }
        Some(SanitizedPath::new(&canonical).as_path().to_path_buf())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::{BorrowAppContext as _, TestAppContext};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    fn init_settings(cx: &mut TestAppContext, default_project_folder: Option<&str>) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            cx.update_global(|store: &mut SettingsStore, cx| {
                store.update_user_settings(cx, |content| {
                    content.workspace.default_project_folder =
                        default_project_folder.map(|s| s.to_string());
                });
            });
        });
    }

    #[gpui::test]
    async fn default_open_path_returns_canonical_directory(cx: &mut TestAppContext) {
        init_settings(cx, Some(path!("/projects")));
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/projects"), json!({ "alpha": {} }))
            .await;

        let task = cx.update(|cx| default_open_path(fs.clone(), cx));
        let result = task.await;
        assert_eq!(result.as_deref(), Some(std::path::Path::new(path!("/projects"))));
    }

    #[gpui::test]
    async fn default_open_path_ignores_missing_path(cx: &mut TestAppContext) {
        init_settings(cx, Some(path!("/does-not-exist")));
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/projects"), json!({})).await;

        let task = cx.update(|cx| default_open_path(fs, cx));
        assert!(task.await.is_none());
    }

    #[gpui::test]
    async fn default_open_path_ignores_file_path(cx: &mut TestAppContext) {
        init_settings(cx, Some(path!("/projects/readme.txt")));
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/projects"),
            json!({ "readme.txt": "hi" }),
        )
        .await;

        let task = cx.update(|cx| default_open_path(fs, cx));
        assert!(task.await.is_none());
    }

    #[gpui::test]
    async fn default_open_path_returns_none_when_unset(cx: &mut TestAppContext) {
        init_settings(cx, None);
        let fs = FakeFs::new(cx.executor());
        let task = cx.update(|cx| default_open_path(fs, cx));
        assert!(task.await.is_none());
    }
}

impl Settings for TabBarSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let tab_bar = content.tab_bar.clone().unwrap();
        TabBarSettings {
            show: tab_bar.show.unwrap(),
            show_nav_history_buttons: tab_bar.show_nav_history_buttons.unwrap(),
            show_tab_bar_buttons: tab_bar.show_tab_bar_buttons.unwrap(),
            show_pinned_tabs_in_separate_row: tab_bar.show_pinned_tabs_in_separate_row.unwrap(),
        }
    }
}

#[derive(Deserialize, RegisterSetting)]
pub struct StatusBarSettings {
    pub show: bool,
    pub show_active_file: bool,
    pub active_language_button: bool,
    pub cursor_position_button: bool,
    pub line_endings_button: bool,
    pub active_encoding_button: EncodingDisplayOptions,
}

impl Settings for StatusBarSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let status_bar = content.status_bar.clone().unwrap();
        StatusBarSettings {
            show: status_bar.show.unwrap(),
            show_active_file: status_bar.show_active_file.unwrap(),
            active_language_button: status_bar.active_language_button.unwrap(),
            cursor_position_button: status_bar.cursor_position_button.unwrap(),
            line_endings_button: status_bar.line_endings_button.unwrap(),
            active_encoding_button: status_bar.active_encoding_button.unwrap(),
        }
    }
}
