use anyhow::Context as _;
use collections::HashSet;
use editor::EditorSettings;
use gpui::{App, Pixels, ReadGlobal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{
    DockSide, ProjectPanelEntrySpacing, ProjectPanelSortMode, RegisterSetting, Settings,
    SettingsLocation, SettingsStore, ShowDiagnostics, ShowIndentGuides,
};
use std::path::Path;
use ui::{
    px,
    scrollbars::{ScrollbarVisibility, ShowScrollbar},
};
use util::{ResultExt, paths::PathStyle, rel_path::RelPath};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, RegisterSetting)]
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
    pub bold_folder_labels: bool,
    pub starts_open: bool,
    pub scrollbar: ScrollbarSettings,
    pub show_diagnostics: ShowDiagnostics,
    pub hide_root: bool,
    pub hide_hidden: bool,
    pub drag_and_drop: bool,
    pub auto_open: AutoOpenSettings,
    pub sort_mode: ProjectPanelSortMode,
    pub diagnostic_badges: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, RegisterSetting)]
pub struct ProjectPanelExclusionSettings {
    pub show_excluded: bool,
    excluded_entries: Vec<String>,
    excluded_entries_lookup: HashSet<String>,
}

impl ProjectPanelExclusionSettings {
    pub fn for_worktree<'a>(worktree_id: settings::WorktreeId, cx: &'a App) -> &'a Self {
        SettingsStore::global(cx).get(Some(SettingsLocation {
            worktree_id,
            path: RelPath::empty(),
        }))
    }

    pub fn excluded_entries(&self) -> &[String] {
        &self.excluded_entries
    }

    pub fn is_path_excluded(&self, path: &RelPath) -> bool {
        self.nearest_excluded_entry(path).is_some()
    }

    pub fn nearest_excluded_entry<'a>(&'a self, path: &RelPath) -> Option<&'a str> {
        path.ancestors().find_map(|ancestor| {
            self.excluded_entries_lookup
                .get(ancestor.as_unix_str())
                .map(String::as_str)
        })
    }
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
    /// Whether to allow horizontal scrolling in the project panel.
    /// When false, the view is locked to the leftmost position and long file names are clipped.
    ///
    /// Default: true
    pub horizontal_scroll: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct AutoOpenSettings {
    pub on_create: bool,
    pub on_paste: bool,
    pub on_drop: bool,
}

impl AutoOpenSettings {
    #[inline]
    pub fn should_open_on_create(self) -> bool {
        self.on_create
    }

    #[inline]
    pub fn should_open_on_paste(self) -> bool {
        self.on_paste
    }

    #[inline]
    pub fn should_open_on_drop(self) -> bool {
        self.on_drop
    }
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
            git_status: project_panel.git_status.unwrap()
                && content
                    .git
                    .as_ref()
                    .unwrap()
                    .enabled
                    .unwrap()
                    .is_git_status_enabled(),
            indent_size: project_panel.indent_size.unwrap(),
            indent_guides: IndentGuidesSettings {
                show: project_panel.indent_guides.unwrap().show.unwrap(),
            },
            sticky_scroll: project_panel.sticky_scroll.unwrap(),
            auto_reveal_entries: project_panel.auto_reveal_entries.unwrap(),
            auto_fold_dirs: project_panel.auto_fold_dirs.unwrap(),
            bold_folder_labels: project_panel.bold_folder_labels.unwrap(),
            starts_open: project_panel.starts_open.unwrap(),
            scrollbar: {
                let scrollbar = project_panel.scrollbar.unwrap();
                ScrollbarSettings {
                    show: scrollbar.show.map(Into::into),
                    horizontal_scroll: scrollbar.horizontal_scroll.unwrap(),
                }
            },
            show_diagnostics: project_panel.show_diagnostics.unwrap(),
            hide_root: project_panel.hide_root.unwrap(),
            hide_hidden: project_panel.hide_hidden.unwrap(),
            drag_and_drop: project_panel.drag_and_drop.unwrap(),
            auto_open: {
                let auto_open = project_panel.auto_open.unwrap();
                AutoOpenSettings {
                    on_create: auto_open.on_create.unwrap(),
                    on_paste: auto_open.on_paste.unwrap(),
                    on_drop: auto_open.on_drop.unwrap(),
                }
            },
            sort_mode: project_panel.sort_mode.unwrap(),
            diagnostic_badges: project_panel.diagnostic_badges.unwrap(),
        }
    }
}

impl Settings for ProjectPanelExclusionSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let project_panel = content.project.project_panel.clone().unwrap_or_default();
        let mut excluded_entries = project_panel
            .excluded_entries
            .unwrap_or_default()
            .into_iter()
            .filter_map(|path| {
                RelPath::new(Path::new(&path), PathStyle::local())
                    .with_context(|| {
                        format!("Failed to parse project panel excluded entry path {path:?}")
                    })
                    .log_err()
                    .map(|path| path.into_owned().as_unix_str().to_string())
            })
            .collect::<Vec<_>>();
        excluded_entries.sort();
        excluded_entries.dedup();

        Self {
            show_excluded: project_panel.show_excluded.unwrap_or(false),
            excluded_entries_lookup: excluded_entries.iter().cloned().collect(),
            excluded_entries,
        }
    }
}
