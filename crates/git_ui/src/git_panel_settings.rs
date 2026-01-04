use editor::EditorSettings;
use gpui::Pixels;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{RegisterSetting, Settings, StatusStyle};
use ui::{
    px,
    scrollbars::{ScrollbarVisibility, ShowScrollbar},
};
use workspace::dock::DockPosition;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarSettings {
    pub show: Option<ShowScrollbar>,
}

const MIN_AUTO_FETCH_INTERVAL_SECS: u64 = 60;
const MAX_AUTO_FETCH_INTERVAL_SECS: u64 = 3600;
const DEFAULT_AUTO_FETCH_INTERVAL_SECS: u64 = 300;
const DEFAULT_COMMIT_HISTORY_COUNT: usize = 10;

#[derive(Debug, Clone, PartialEq, RegisterSetting)]
pub struct GitPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: Pixels,
    pub status_style: StatusStyle,
    pub scrollbar: ScrollbarSettings,
    pub fallback_branch_name: String,
    pub sort_by_path: bool,
    pub collapse_untracked_diff: bool,
    pub tree_view: bool,
    /// Whether to automatically fetch from remote repositories in the background.
    pub auto_fetch: bool,
    /// Interval in seconds between automatic fetches.
    pub auto_fetch_interval_secs: u64,
    /// Whether to show the commit history section in the git panel.
    pub show_commit_history: bool,
    /// Number of commits to show in the commit history section.
    pub commit_history_count: usize,
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
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let git_panel = content.git_panel.clone().unwrap();
        let auto_fetch_interval = git_panel
            .auto_fetch_interval_secs
            .unwrap_or(DEFAULT_AUTO_FETCH_INTERVAL_SECS)
            .clamp(MIN_AUTO_FETCH_INTERVAL_SECS, MAX_AUTO_FETCH_INTERVAL_SECS);

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
            tree_view: git_panel.tree_view.unwrap(),
            auto_fetch: git_panel.auto_fetch.unwrap_or(true),
            auto_fetch_interval_secs: auto_fetch_interval,
            show_commit_history: git_panel.show_commit_history.unwrap_or(true),
            commit_history_count: git_panel
                .commit_history_count
                .unwrap_or(DEFAULT_COMMIT_HISTORY_COUNT),
        }
    }
}
