use editor::EditorSettings;
use gpui::Pixels;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{
    DockSide, ProjectPanelEntrySpacing, Settings, SettingsContent, ShowDiagnostics,
    ShowIndentGuides,
};
use ui::{
    px,
    scrollbars::{ScrollbarVisibility, ShowScrollbar},
};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct ProjectPanelSettings {
    pub button: bool,
    pub hide_gitignore: bool,
    pub default_width: Pixels,
    pub dock: DockSide,
    pub entry_spacing: ProjectPanelEntrySpacing,
    pub file_icons: bool,
    pub folder_icons: bool,
    pub git_status: bool,
    pub indent_size: f32,
    pub indent_guides: IndentGuidesSettings,
    pub sticky_scroll: bool,
    pub auto_reveal_entries: bool,
    pub auto_fold_dirs: bool,
    pub starts_open: bool,
    pub scrollbar: ScrollbarSettings,
    pub show_diagnostics: ShowDiagnostics,
    pub hide_root: bool,
    pub drag_and_drop: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct IndentGuidesSettings {
    pub show: ShowIndentGuides,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarSettings {
    /// When to show the scrollbar in the project panel.
    ///
    /// Default: inherits editor scrollbar settings
    pub show: Option<ShowScrollbar>,
}

impl ScrollbarVisibility for ProjectPanelSettings {
    fn visibility(&self, cx: &ui::App) -> ShowScrollbar {
        self.scrollbar
            .show
            .unwrap_or_else(|| EditorSettings::get_global(cx).scrollbar.show)
    }
}

impl Settings for ProjectPanelSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let project_panel = content.project_panel.clone().unwrap();
        Self {
            button: project_panel.button.unwrap(),
            hide_gitignore: project_panel.hide_gitignore.unwrap(),
            default_width: px(project_panel.default_width.unwrap()),
            dock: project_panel.dock.unwrap(),
            entry_spacing: project_panel.entry_spacing.unwrap(),
            file_icons: project_panel.file_icons.unwrap(),
            folder_icons: project_panel.folder_icons.unwrap(),
            git_status: project_panel.git_status.unwrap(),
            indent_size: project_panel.indent_size.unwrap(),
            indent_guides: IndentGuidesSettings {
                show: project_panel.indent_guides.unwrap().show.unwrap(),
            },
            sticky_scroll: project_panel.sticky_scroll.unwrap(),
            auto_reveal_entries: project_panel.auto_reveal_entries.unwrap(),
            auto_fold_dirs: project_panel.auto_fold_dirs.unwrap(),
            starts_open: project_panel.starts_open.unwrap(),
            scrollbar: ScrollbarSettings {
                show: project_panel.scrollbar.unwrap().show.map(Into::into),
            },
            show_diagnostics: project_panel.show_diagnostics.unwrap(),
            hide_root: project_panel.hide_root.unwrap(),
            drag_and_drop: project_panel.drag_and_drop.unwrap(),
        }
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut SettingsContent) {
        if let Some(hide_gitignore) = vscode.read_bool("explorer.excludeGitIgnore") {
            current.project_panel.get_or_insert_default().hide_gitignore = Some(hide_gitignore);
        }
        if let Some(auto_reveal) = vscode.read_bool("explorer.autoReveal") {
            current
                .project_panel
                .get_or_insert_default()
                .auto_reveal_entries = Some(auto_reveal);
        }
        if let Some(compact_folders) = vscode.read_bool("explorer.compactFolders") {
            current.project_panel.get_or_insert_default().auto_fold_dirs = Some(compact_folders);
        }

        if Some(false) == vscode.read_bool("git.decorations.enabled") {
            current.project_panel.get_or_insert_default().git_status = Some(false);
        }
        if Some(false) == vscode.read_bool("problems.decorations.enabled") {
            current
                .project_panel
                .get_or_insert_default()
                .show_diagnostics = Some(ShowDiagnostics::Off);
        }
        if let (Some(false), Some(false)) = (
            vscode.read_bool("explorer.decorations.badges"),
            vscode.read_bool("explorer.decorations.colors"),
        ) {
            current.project_panel.get_or_insert_default().git_status = Some(false);
            current
                .project_panel
                .get_or_insert_default()
                .show_diagnostics = Some(ShowDiagnostics::Off);
        }
    }
}
