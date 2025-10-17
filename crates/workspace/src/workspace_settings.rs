use std::num::NonZeroUsize;

use crate::DockPosition;
use collections::HashMap;
use serde::Deserialize;
pub use settings::AutosaveSetting;
use settings::Settings;
pub use settings::{
    BottomDockLayout, PaneSplitDirectionHorizontal, PaneSplitDirectionVertical,
    RestoreOnStartupBehavior,
};

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
    pub restore_on_file_reopen: bool,
    pub drop_target_size: f32,
    pub use_system_path_prompts: bool,
    pub use_system_prompts: bool,
    pub command_aliases: HashMap<String, String>,
    pub max_tabs: Option<NonZeroUsize>,
    pub when_closing_with_no_tabs: settings::CloseWindowWhenNoItems,
    pub on_last_window_closed: settings::OnLastWindowClosed,
    pub resize_all_panels_in_dock: Vec<DockPosition>,
    pub close_on_file_delete: bool,
    pub use_system_window_tabs: bool,
    pub zoomed_padding: bool,
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
    pub inactive_opacity: Option<f32>,
}

#[derive(Deserialize)]
pub struct TabBarSettings {
    pub show: bool,
    pub show_nav_history_buttons: bool,
    pub show_tab_bar_buttons: bool,
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
            restore_on_file_reopen: workspace.restore_on_file_reopen.unwrap(),
            drop_target_size: workspace.drop_target_size.unwrap(),
            use_system_path_prompts: workspace.use_system_path_prompts.unwrap(),
            use_system_prompts: workspace.use_system_prompts.unwrap(),
            command_aliases: workspace.command_aliases.clone(),
            max_tabs: workspace.max_tabs,
            when_closing_with_no_tabs: workspace.when_closing_with_no_tabs.unwrap(),
            on_last_window_closed: workspace.on_last_window_closed.unwrap(),
            resize_all_panels_in_dock: workspace
                .resize_all_panels_in_dock
                .clone()
                .unwrap()
                .into_iter()
                .map(Into::into)
                .collect(),
            close_on_file_delete: workspace.close_on_file_delete.unwrap(),
            use_system_window_tabs: workspace.use_system_window_tabs.unwrap(),
            zoomed_padding: workspace.zoomed_padding.unwrap(),
        }
    }

    fn import_from_vscode(
        vscode: &settings::VsCodeSettings,
        current: &mut settings::SettingsContent,
    ) {
        if vscode
            .read_bool("accessibility.dimUnfocused.enabled")
            .unwrap_or_default()
            && let Some(opacity) = vscode
                .read_value("accessibility.dimUnfocused.opacity")
                .and_then(|v| v.as_f64())
        {
            current
                .workspace
                .active_pane_modifiers
                .get_or_insert_default()
                .inactive_opacity = Some(opacity as f32);
        }

        vscode.enum_setting(
            "window.confirmBeforeClose",
            &mut current.workspace.confirm_quit,
            |s| match s {
                "always" | "keyboardOnly" => Some(true),
                "never" => Some(false),
                _ => None,
            },
        );

        vscode.bool_setting(
            "workbench.editor.restoreViewState",
            &mut current.workspace.restore_on_file_reopen,
        );

        if let Some(b) = vscode.read_bool("window.closeWhenEmpty") {
            current.workspace.when_closing_with_no_tabs = Some(if b {
                settings::CloseWindowWhenNoItems::CloseWindow
            } else {
                settings::CloseWindowWhenNoItems::KeepWindowOpen
            });
        }

        if let Some(b) = vscode.read_bool("files.simpleDialog.enable") {
            current.workspace.use_system_path_prompts = Some(!b);
        }

        if let Some(v) = vscode.read_enum("files.autoSave", |s| match s {
            "off" => Some(AutosaveSetting::Off),
            "afterDelay" => Some(AutosaveSetting::AfterDelay {
                milliseconds: vscode
                    .read_value("files.autoSaveDelay")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1000),
            }),
            "onFocusChange" => Some(AutosaveSetting::OnFocusChange),
            "onWindowChange" => Some(AutosaveSetting::OnWindowChange),
            _ => None,
        }) {
            current.workspace.autosave = Some(v);
        }

        // workbench.editor.limit contains "enabled", "value", and "perEditorGroup"
        // our semantics match if those are set to true, some N, and true respectively.
        // we'll ignore "perEditorGroup" for now since we only support a global max
        if let Some(n) = vscode
            .read_value("workbench.editor.limit.value")
            .and_then(|v| v.as_u64())
            .and_then(|n| NonZeroUsize::new(n as usize))
            && vscode
                .read_bool("workbench.editor.limit.enabled")
                .unwrap_or_default()
        {
            current.workspace.max_tabs = Some(n)
        }

        if let Some(b) = vscode.read_bool("window.nativeTabs") {
            current.workspace.use_system_window_tabs = Some(b);
        }

        // some combination of "window.restoreWindows" and "workbench.startupEditor" might
        // map to our "restore_on_startup"

        // there doesn't seem to be a way to read whether the bottom dock's "justified"
        // setting is enabled in vscode. that'd be our equivalent to "bottom_dock_layout"
    }
}

impl Settings for TabBarSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let tab_bar = content.tab_bar.clone().unwrap();
        TabBarSettings {
            show: tab_bar.show.unwrap(),
            show_nav_history_buttons: tab_bar.show_nav_history_buttons.unwrap(),
            show_tab_bar_buttons: tab_bar.show_tab_bar_buttons.unwrap(),
        }
    }

    fn import_from_vscode(
        vscode: &settings::VsCodeSettings,
        current: &mut settings::SettingsContent,
    ) {
        if let Some(b) = vscode.read_enum("workbench.editor.showTabs", |s| match s {
            "multiple" => Some(true),
            "single" | "none" => Some(false),
            _ => None,
        }) {
            current.tab_bar.get_or_insert_default().show = Some(b);
        }
        if Some("hidden") == vscode.read_string("workbench.editor.editorActionsLocation") {
            current.tab_bar.get_or_insert_default().show_tab_bar_buttons = Some(false)
        }
    }
}

#[derive(Deserialize)]
pub struct StatusBarSettings {
    pub show: bool,
    pub active_language_button: bool,
    pub cursor_position_button: bool,
}

impl Settings for StatusBarSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let status_bar = content.status_bar.clone().unwrap();
        StatusBarSettings {
            show: status_bar.show.unwrap(),
            active_language_button: status_bar.active_language_button.unwrap(),
            cursor_position_button: status_bar.cursor_position_button.unwrap(),
        }
    }

    fn import_from_vscode(
        vscode: &settings::VsCodeSettings,
        current: &mut settings::SettingsContent,
    ) {
        if let Some(show) = vscode.read_bool("workbench.statusBar.visible") {
            current.status_bar.get_or_insert_default().show = Some(show);
        }
    }
}
