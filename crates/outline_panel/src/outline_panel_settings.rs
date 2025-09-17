use editor::EditorSettings;
use gpui::{App, Pixels};
pub use settings::{DockSide, Settings, ShowIndentGuides};
use ui::scrollbars::{ScrollbarVisibility, ShowScrollbar};
use util::MergeFrom;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OutlinePanelSettings {
    pub button: bool,
    pub default_width: Pixels,
    pub dock: LeftRightDockPosition,
    pub file_icons: bool,
    pub folder_icons: bool,
    pub git_status: bool,
    pub indent_size: f32,
    pub indent_guides: IndentGuidesSettings,
    pub auto_reveal_entries: bool,
    pub auto_fold_dirs: bool,
    pub scrollbar: ScrollbarSettings,
    pub expand_outlines_with_depth: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ScrollbarSettings {
    /// When to show the scrollbar in the project panel.
    ///
    /// Default: inherits editor scrollbar settings
    pub show: Option<ShowScrollbar>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IndentGuidesSettings {
    pub show: ShowIndentGuides,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IndentGuidesSettingsContent {
    /// When to show the scrollbar in the outline panel.
    pub show: Option<ShowIndentGuides>,
}

impl ScrollbarVisibility for OutlinePanelSettings {
    fn visibility(&self, cx: &App) -> ShowScrollbar {
        self.scrollbar
            .show
            .unwrap_or_else(|| EditorSettings::get_global(cx).scrollbar.show)
    }
}

impl Settings for OutlinePanelSettings {
    fn from_defaults(content: &settings::SettingsContent, _cx: &mut App) -> Self {
        let panel = content.outline_panel.as_ref().unwrap();
        Self {
            button: panel.button.unwrap(),
            default_width: panel.default_width.map(gpui::px).unwrap(),
            dock: panel.dock.unwrap(),
            file_icons: panel.file_icons.unwrap(),
            folder_icons: panel.folder_icons.unwrap(),
            git_status: panel.git_status.unwrap(),
            indent_size: panel.indent_size.unwrap(),
            indent_guides: IndentGuidesSettings {
                show: panel.indent_guides.unwrap().show.unwrap(),
            },
            auto_reveal_entries: panel.auto_reveal_entries.unwrap(),
            auto_fold_dirs: panel.auto_fold_dirs.unwrap(),
            scrollbar: ScrollbarSettings {
                show: panel.scrollbar.unwrap().show.flatten().map(Into::into),
            },
            expand_outlines_with_depth: panel.expand_outlines_with_depth.unwrap(),
        }
    }

    fn refine(&mut self, content: &settings::SettingsContent, cx: &mut App) {
        let Some(panel) = content.outline_panel.as_ref() else {
            return;
        };

        self.button.merge_from(&panel.button);
        self.default_width
            .merge_from(&panel.default_width.map(Pixels::from));
        self.dock.merge_from(&panel.dock.map(Into::into));
        self.file_icons.merge_from(&panel.file_icons);
        self.folder_icons.merge_from(&panel.folder_icons);
        self.git_status.merge_from(&panel.git_status);
        self.indent_size.merge_from(&panel.indent_size);

        if let Some(indent_guides) = panel.indent_guides.as_ref() {
            self.indent_guides.show.merge_from(&indent_guides.show);
        }

        self.auto_reveal_entries
            .merge_from(&panel.auto_reveal_entries);
        self.auto_fold_dirs.merge_from(&panel.auto_fold_dirs);

        if let Some(scrollbar) = panel.scrollbar.as_ref() {
            self.scrollbar.show.merge_from(&scrollbar.show.flatten());
        }
    }
    fn import_from_vscode(
        vscode: &settings::VsCodeSettings,
        current: &mut settings::SettingsContent,
    ) {
        if let Some(b) = vscode.read_bool("outline.icons") {
            let outline_panel = current.outline_panel.get_or_insert_default();
            outline_panel.file_icons = Some(b);
            outline_panel.folder_icons = Some(b);
        }

        if let Some(b) = vscode.read_bool("git.decorations.enabled") {
            let outline_panel = current.outline_panel.get_or_insert_default();
            outline_panel.git_status = Some(b);
        }
    }
}
