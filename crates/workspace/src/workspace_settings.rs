use std::num::NonZeroUsize;

use crate::DockPosition;
use anyhow::Result;
use collections::HashMap;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};

#[derive(Deserialize)]
pub struct WorkspaceSettings {
    pub active_pane_modifiers: ActivePanelModifiers,
    pub bottom_dock_layout: BottomDockLayout,
    pub pane_split_direction_horizontal: PaneSplitDirectionHorizontal,
    pub pane_split_direction_vertical: PaneSplitDirectionVertical,
    pub centered_layout: CenteredLayoutSettings,
    pub confirm_quit: bool,
    pub show_call_status_icon: bool,
    pub autosave: AutosaveSetting,
    pub restore_on_startup: RestoreOnStartupBehavior,
    pub restore_on_file_reopen: bool,
    pub drop_target_size: f32,
    pub use_system_path_prompts: bool,
    pub use_system_prompts: bool,
    pub command_aliases: HashMap<String, String>,
    pub max_tabs: Option<NonZeroUsize>,
    pub when_closing_with_no_tabs: CloseWindowWhenNoItems,
    pub on_last_window_closed: OnLastWindowClosed,
    pub resize_all_panels_in_dock: Vec<DockPosition>,
    pub close_on_file_delete: bool,
    pub use_system_window_tabs: bool,
    pub zoomed_padding: bool,
}

#[derive(Copy, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OnLastWindowClosed {
    /// Match platform conventions by default, so don't quit on macOS, and quit on other platforms
    #[default]
    PlatformDefault,
    /// Quit the application the last window is closed
    QuitApp,
}

impl OnLastWindowClosed {
    pub fn is_quit_app(&self) -> bool {
        match self {
            OnLastWindowClosed::PlatformDefault => false,
            OnLastWindowClosed::QuitApp => true,
        }
    }
}

#[derive(Deserialize)]
pub struct TabBarSettings {
    pub show: bool,
    pub show_nav_history_buttons: bool,
    pub show_tab_bar_buttons: bool,
}
impl Settings for WorkspaceSettings {
    type FileContent = WorkspaceSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut Self::FileContent) {
        if vscode
            .read_bool("accessibility.dimUnfocused.enabled")
            .unwrap_or_default()
            && let Some(opacity) = vscode
                .read_value("accessibility.dimUnfocused.opacity")
                .and_then(|v| v.as_f64())
        {
            if let Some(settings) = current.active_pane_modifiers.as_mut() {
                settings.inactive_opacity = Some(opacity as f32)
            } else {
                current.active_pane_modifiers = Some(ActivePanelModifiers {
                    inactive_opacity: Some(opacity as f32),
                    ..Default::default()
                })
            }
        }

        vscode.enum_setting(
            "window.confirmBeforeClose",
            &mut current.confirm_quit,
            |s| match s {
                "always" | "keyboardOnly" => Some(true),
                "never" => Some(false),
                _ => None,
            },
        );

        vscode.bool_setting(
            "workbench.editor.restoreViewState",
            &mut current.restore_on_file_reopen,
        );

        if let Some(b) = vscode.read_bool("window.closeWhenEmpty") {
            current.when_closing_with_no_tabs = Some(if b {
                CloseWindowWhenNoItems::CloseWindow
            } else {
                CloseWindowWhenNoItems::KeepWindowOpen
            })
        }

        if let Some(b) = vscode.read_bool("files.simpleDialog.enable") {
            current.use_system_path_prompts = Some(!b);
        }

        vscode.enum_setting("files.autoSave", &mut current.autosave, |s| match s {
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
        });

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
            current.max_tabs = Some(n)
        }

        vscode.bool_setting("window.nativeTabs", &mut current.use_system_window_tabs);

        // some combination of "window.restoreWindows" and "workbench.startupEditor" might
        // map to our "restore_on_startup"

        // there doesn't seem to be a way to read whether the bottom dock's "justified"
        // setting is enabled in vscode. that'd be our equivalent to "bottom_dock_layout"
    }
}

impl Settings for TabBarSettings {
    type FileContent = TabBarSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut Self::FileContent) {
        vscode.enum_setting(
            "workbench.editor.showTabs",
            &mut current.show,
            |s| match s {
                "multiple" => Some(true),
                "single" | "none" => Some(false),
                _ => None,
            },
        );
        if Some("hidden") == vscode.read_string("workbench.editor.editorActionsLocation") {
            current.show_tab_bar_buttons = Some(false)
        }
    }
}
