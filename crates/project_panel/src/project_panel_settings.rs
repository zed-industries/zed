use editor::EditorSettings;
use gpui::Pixels;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{DockSide, ProjectPanelEntrySpacing, Settings, ShowDiagnostics, ShowIndentGuides};
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
    pub hide_hidden: bool,
    pub drag_and_drop: bool,
    pub open_file_on_paste: bool,
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
            hide_hidden: project_panel.hide_hidden.unwrap(),
            drag_and_drop: project_panel.drag_and_drop.unwrap(),
            open_file_on_paste: project_panel.open_file_on_paste.unwrap(),
        }
    }
}
