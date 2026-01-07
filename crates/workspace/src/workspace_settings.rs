use std::num::NonZeroUsize;

use crate::DockPosition;
use collections::HashMap;
use serde::Deserialize;
pub use settings::{
    AutosaveSetting, BottomDockLayout, EncodingDisplayOptions, InactiveOpacity,
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
    pub restore_on_file_reopen: bool,
    pub drop_target_size: f32,
    pub use_system_path_prompts: bool,
    pub use_system_prompts: bool,
    pub command_aliases: HashMap<String, String>,
    pub max_tabs: Option<NonZeroUsize>,
    pub when_closing_with_no_tabs: settings::CloseWindowWhenNoItems,
    pub on_last_window_closed: settings::OnLastWindowClosed,
    pub text_rendering_mode: settings::TextRenderingMode,
    pub resize_all_panels_in_dock: Vec<DockPosition>,
    pub close_on_file_delete: bool,
    pub use_system_window_tabs: bool,
    pub zoomed_padding: bool,
    pub window_decorations: settings::WindowDecorations,
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
            text_rendering_mode: workspace.text_rendering_mode.unwrap(),
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
            window_decorations: workspace.window_decorations.unwrap(),
        }
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
}

#[derive(Deserialize, RegisterSetting)]
pub struct StatusBarSettings {
    pub show: bool,
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
            active_language_button: status_bar.active_language_button.unwrap(),
            cursor_position_button: status_bar.cursor_position_button.unwrap(),
            line_endings_button: status_bar.line_endings_button.unwrap(),
            active_encoding_button: status_bar.active_encoding_button.unwrap(),
        }
    }
}
