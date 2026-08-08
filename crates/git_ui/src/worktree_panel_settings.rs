use gpui::{App, Pixels};
use settings::{GitPanelSortBy, RegisterSetting, Settings, StatusStyle};
use ui::px;
use workspace::dock::DockPosition;

use crate::git_panel_settings::GitPanelSettings;

#[derive(Debug, Clone, PartialEq, RegisterSetting)]
pub struct WorktreePanelSettings {
    pub button: bool,
    /// `None` means "wherever the git panel is", so the two panels' buttons sit
    /// together in the status bar without having to be configured twice.
    pub dock: Option<DockPosition>,
    pub default_width: Pixels,
}

impl Settings for WorktreePanelSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let worktree_panel = content.worktree_panel.clone().unwrap();
        Self {
            button: worktree_panel.button.unwrap(),
            dock: worktree_panel.dock.map(Into::into),
            default_width: px(worktree_panel.default_width.unwrap()),
        }
    }
}

/// How the worktree panel draws its rows.
///
/// The panel deliberately has no appearance settings of its own: it shows the
/// same kind of rows as the git panel, so it reads the git panel's settings and
/// the two stay consistent without being configured twice.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct WorktreeRowStyle {
    pub status_style: StatusStyle,
    pub file_icons: bool,
    pub diff_stats: bool,
    pub sort_by: GitPanelSortBy,
}

impl WorktreeRowStyle {
    pub fn get(cx: &App) -> Self {
        let git_panel = GitPanelSettings::get_global(cx);
        Self {
            status_style: git_panel.status_style,
            file_icons: git_panel.file_icons,
            diff_stats: git_panel.diff_stats,
            sort_by: git_panel.sort_by,
        }
    }
}

/// The side the panel docks on, falling back to the git panel's side.
pub(crate) fn worktree_panel_dock(cx: &App) -> DockPosition {
    WorktreePanelSettings::get_global(cx)
        .dock
        .unwrap_or_else(|| GitPanelSettings::get_global(cx).dock)
}
