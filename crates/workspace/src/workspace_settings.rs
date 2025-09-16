use std::num::NonZeroUsize;

use crate::DockPosition;
use anyhow::Result;
use collections::HashMap;
use gpui::App;
use serde::Deserialize;
pub use settings::AutosaveSetting;
use settings::Settings;
pub use settings::{
    BottomDockLayout, PaneSplitDirectionHorizontal, PaneSplitDirectionVertical,
    RestoreOnStartupBehavior,
};
use util::MergeFrom as _;

pub struct WorkspaceSettings {
    pub active_pane_modifiers: ActivePanelModifiers,
    pub bottom_dock_layout: settings::BottomDockLayout,
    pub pane_split_direction_horizontal: settings::PaneSplitDirectionHorizontal,
    pub pane_split_direction_vertical: settings::PaneSplitDirectionVertical,
    pub centered_layout: settings::CenteredLayoutSettings, // <- This one is hard to describe, especially as it has
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
    pub resize_all_panels_in_dock: Vec<DockPosition>, // <- This one is not an overwrite merge, it is an extend merge
    pub close_on_file_delete: bool,
    pub use_system_window_tabs: bool,
    pub zoomed_padding: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CenteredLayoutSettings {
    /// The relative width of the left padding of the central pane from the
    /// workspace when the centered layout is used.
    ///
    /// Default: 0.2
    pub left_padding: f32,
    // The relative width of the right padding of the central pane from the
    // workspace when the centered layout is used.
    ///
    /// Default: 0.2
    pub right_padding: f32,
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
    fn from_defaults(content: &settings::SettingsContent, cx: &mut App) -> Self {
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
            bottom_dock_layout: workspace.bottom_dock_layout.clone().unwrap(),
            pane_split_direction_horizontal: workspace
                .pane_split_direction_horizontal
                .clone()
                .unwrap(),
            pane_split_direction_vertical: workspace.pane_split_direction_vertical.clone().unwrap(),
            centered_layout: workspace.centered_layout.clone().unwrap(),
            confirm_quit: workspace.confirm_quit.clone().unwrap(),
            show_call_status_icon: workspace.show_call_status_icon.clone().unwrap(),
            autosave: workspace.autosave.clone().unwrap(),
            restore_on_startup: workspace.restore_on_startup.clone().unwrap(),
            restore_on_file_reopen: workspace.restore_on_file_reopen.clone().unwrap(),
            drop_target_size: workspace.drop_target_size.clone().unwrap(),
            use_system_path_prompts: workspace.use_system_path_prompts.clone().unwrap(),
            use_system_prompts: workspace.use_system_prompts.clone().unwrap(),
            command_aliases: workspace.command_aliases.clone(),
            max_tabs: workspace.max_tabs.clone(),
            when_closing_with_no_tabs: workspace.when_closing_with_no_tabs.clone().unwrap(),
            on_last_window_closed: workspace.on_last_window_closed.clone().unwrap(),
            resize_all_panels_in_dock: workspace
                .resize_all_panels_in_dock
                .iter()
                .copied()
                .map(Into::into)
                .collect(),
            close_on_file_delete: workspace.close_on_file_delete.clone().unwrap(),
            use_system_window_tabs: workspace.use_system_window_tabs.clone().unwrap(),
            zoomed_padding: workspace.zoomed_padding.clone().unwrap(),
        }
    }

    fn refine(&mut self, content: &settings::SettingsContent, cx: &mut App) {
        let workspace = &content.workspace;
        if let Some(border_size) = *&workspace
            .active_pane_modifiers
            .and_then(|modifier| modifier.border_size)
        {
            self.active_pane_modifiers.border_size = Some(border_size);
        }

        if let Some(inactive_opacity) = *&workspace
            .active_pane_modifiers
            .and_then(|modifier| modifier.inactive_opacity)
        {
            self.active_pane_modifiers.inactive_opacity = Some(inactive_opacity);
        }

        self.bottom_dock_layout
            .merge_from(&workspace.bottom_dock_layout);
        self.pane_split_direction_horizontal
            .merge_from(&workspace.pane_split_direction_horizontal);
        self.pane_split_direction_vertical
            .merge_from(&workspace.pane_split_direction_vertical);
        self.centered_layout.merge_from(&workspace.centered_layout);
        self.confirm_quit.merge_from(&workspace.confirm_quit);
        self.show_call_status_icon
            .merge_from(&workspace.show_call_status_icon);
        self.autosave.merge_from(&workspace.autosave);
        self.restore_on_startup
            .merge_from(&workspace.restore_on_startup);
        self.restore_on_file_reopen
            .merge_from(&workspace.restore_on_file_reopen);
        self.drop_target_size
            .merge_from(&workspace.drop_target_size);
        self.use_system_path_prompts
            .merge_from(&workspace.use_system_path_prompts);
        self.use_system_prompts
            .merge_from(&workspace.use_system_prompts);
        self.command_aliases
            .extend(workspace.command_aliases.clone());
        if let Some(max_tabs) = workspace.max_tabs {
            self.max_tabs = Some(max_tabs);
        }
        self.when_closing_with_no_tabs
            .merge_from(&workspace.when_closing_with_no_tabs);
        self.on_last_window_closed
            .merge_from(&workspace.on_last_window_closed);
        self.resize_all_panels_in_dock.extend(
            workspace
                .resize_all_panels_in_dock
                .iter()
                .copied()
                .map(Into::<DockPosition>::into),
        );
        self.close_on_file_delete
            .merge_from(&workspace.close_on_file_delete);
        self.use_system_window_tabs
            .merge_from(&workspace.use_system_window_tabs);
        self.zoomed_padding.merge_from(&workspace.zoomed_padding);
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
    fn import_from_vscode(
        vscode: &settings::VsCodeSettings,
        current: &mut settings::SettingsContent,
    ) {
        if let Some(b) = vscode.read_enum("workbench.editor.showTabs", |s| match s {
            "multiple" => Some(true),
            "single" | "none" => Some(false),
            _ => None,
        }) {
            current.workspace.tab_bar.get_or_insert_default().show = Some(b);
        }
        if Some("hidden") == vscode.read_string("workbench.editor.editorActionsLocation") {
            current
                .workspace
                .tab_bar
                .get_or_insert_default()
                .show_tab_bar_buttons = Some(false)
        }
    }
}
