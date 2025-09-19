use editor::EditorSettings;
use gpui::Pixels;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsContent, StatusStyle};
use ui::{
    px,
    scrollbars::{ScrollbarVisibility, ShowScrollbar},
};
use workspace::dock::DockPosition;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarSettingsContent {
    /// When to show the scrollbar in the git panel.
    ///
    /// Default: inherits editor scrollbar settings
    pub show: Option<Option<ShowScrollbar>>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarSettings {
    pub show: Option<ShowScrollbar>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GitPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: Pixels,
    pub status_style: StatusStyle,
    pub scrollbar: ScrollbarSettings,
    pub fallback_branch_name: String,
    pub sort_by_path: bool,
    pub collapse_untracked_diff: bool,
}

impl ScrollbarVisibility for GitPanelSettings {
    fn visibility(&self, cx: &ui::App) -> ShowScrollbar {
        // TODO: This PR should have defined Editor's `scrollbar.axis`
        // as an Option<ScrollbarAxis>, not a ScrollbarAxes as it would allow you to
        // `.unwrap_or(EditorSettings::get_global(cx).scrollbar.show)`.
        //
        // Once this is fixed we can extend the GitPanelSettings with a `scrollbar.axis`
        // so we can show each axis based on the settings.
        //
        // We should fix this. PR: https://github.com/zed-industries/zed/pull/19495
        self.scrollbar
            .show
            .unwrap_or_else(|| EditorSettings::get_global(cx).scrollbar.show)
    }
}

impl Settings for GitPanelSettings {
    fn from_settings(content: &settings::SettingsContent, _cx: &mut ui::App) -> Self {
        let git_panel = content.git_panel.clone().unwrap();
        Self {
            button: git_panel.button.unwrap(),
            dock: git_panel.dock.unwrap().into(),
            default_width: px(git_panel.default_width.unwrap()),
            status_style: git_panel.status_style.unwrap(),
            scrollbar: ScrollbarSettings {
                show: git_panel.scrollbar.unwrap().show.map(Into::into),
            },
            fallback_branch_name: git_panel.fallback_branch_name.unwrap(),
            sort_by_path: git_panel.sort_by_path.unwrap(),
            collapse_untracked_diff: git_panel.collapse_untracked_diff.unwrap(),
        }
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut SettingsContent) {
        if let Some(git_enabled) = vscode.read_bool("git.enabled") {
            current.git_panel.get_or_insert_default().button = Some(git_enabled);
        }
        if let Some(default_branch) = vscode.read_string("git.defaultBranchName") {
            current
                .git_panel
                .get_or_insert_default()
                .fallback_branch_name = Some(default_branch.to_string());
        }
    }
}
