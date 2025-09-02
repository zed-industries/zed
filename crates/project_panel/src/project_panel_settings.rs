use editor::ShowScrollbar;
use gpui::Pixels;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources, SettingsUi};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectPanelDockPosition {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ShowIndentGuides {
    Always,
    Never,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EntrySpacing {
    /// Comfortable spacing of entries.
    #[default]
    Comfortable,
    /// The standard spacing of entries.
    Standard,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct ProjectPanelSettings {
    pub button: bool,
    pub hide_gitignore: bool,
    pub default_width: Pixels,
    pub dock: ProjectPanelDockPosition,
    pub entry_spacing: EntrySpacing,
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
pub struct IndentGuidesSettingsContent {
    /// When to show the scrollbar in the project panel.
    pub show: Option<ShowIndentGuides>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarSettings {
    /// When to show the scrollbar in the project panel.
    ///
    /// Default: inherits editor scrollbar settings
    pub show: Option<ShowScrollbar>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarSettingsContent {
    /// When to show the scrollbar in the project panel.
    ///
    /// Default: inherits editor scrollbar settings
    pub show: Option<Option<ShowScrollbar>>,
}

/// Whether to indicate diagnostic errors and/or warnings in project panel items.
///
/// Default: all
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ShowDiagnostics {
    /// Never mark the diagnostic errors/warnings in the project panel.
    Off,
    /// Mark files containing only diagnostic errors in the project panel.
    Errors,
    #[default]
    /// Mark files containing diagnostic errors or warnings in the project panel.
    All,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug, SettingsUi)]
pub struct ProjectPanelSettingsContent {
    /// Whether to show the project panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Whether to hide gitignore files in the project panel.
    ///
    /// Default: false
    pub hide_gitignore: Option<bool>,
    /// Customize default width (in pixels) taken by project panel
    ///
    /// Default: 240
    pub default_width: Option<f32>,
    /// The position of project panel
    ///
    /// Default: left
    pub dock: Option<ProjectPanelDockPosition>,
    /// Spacing between worktree entries in the project panel.
    ///
    /// Default: comfortable
    pub entry_spacing: Option<EntrySpacing>,
    /// Whether to show file icons in the project panel.
    ///
    /// Default: true
    pub file_icons: Option<bool>,
    /// Whether to show folder icons or chevrons for directories in the project panel.
    ///
    /// Default: true
    pub folder_icons: Option<bool>,
    /// Whether to show the git status in the project panel.
    ///
    /// Default: true
    pub git_status: Option<bool>,
    /// Amount of indentation (in pixels) for nested items.
    ///
    /// Default: 20
    pub indent_size: Option<f32>,
    /// Whether to reveal it in the project panel automatically,
    /// when a corresponding project entry becomes active.
    /// Gitignored entries are never auto revealed.
    ///
    /// Default: true
    pub auto_reveal_entries: Option<bool>,
    /// Whether to fold directories automatically
    /// when directory has only one directory inside.
    ///
    /// Default: true
    pub auto_fold_dirs: Option<bool>,
    /// Whether the project panel should open on startup.
    ///
    /// Default: true
    pub starts_open: Option<bool>,
    /// Scrollbar-related settings
    pub scrollbar: Option<ScrollbarSettingsContent>,
    /// Which files containing diagnostic errors/warnings to mark in the project panel.
    ///
    /// Default: all
    pub show_diagnostics: Option<ShowDiagnostics>,
    /// Settings related to indent guides in the project panel.
    pub indent_guides: Option<IndentGuidesSettingsContent>,
    /// Whether to hide the root entry when only one folder is open in the window.
    ///
    /// Default: false
    pub hide_root: Option<bool>,
    /// Whether to stick parent directories at top of the project panel.
    ///
    /// Default: true
    pub sticky_scroll: Option<bool>,
    /// Whether to enable drag-and-drop operations in the project panel.
    ///
    /// Default: true
    pub drag_and_drop: Option<bool>,
}

impl Settings for ProjectPanelSettings {
    const KEY: Option<&'static str> = Some("project_panel");

    type FileContent = ProjectPanelSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::App,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut Self::FileContent) {
        vscode.bool_setting("explorer.excludeGitIgnore", &mut current.hide_gitignore);
        vscode.bool_setting("explorer.autoReveal", &mut current.auto_reveal_entries);
        vscode.bool_setting("explorer.compactFolders", &mut current.auto_fold_dirs);

        if Some(false) == vscode.read_bool("git.decorations.enabled") {
            current.git_status = Some(false);
        }
        if Some(false) == vscode.read_bool("problems.decorations.enabled") {
            current.show_diagnostics = Some(ShowDiagnostics::Off);
        }
        if let (Some(false), Some(false)) = (
            vscode.read_bool("explorer.decorations.badges"),
            vscode.read_bool("explorer.decorations.colors"),
        ) {
            current.git_status = Some(false);
            current.show_diagnostics = Some(ShowDiagnostics::Off);
        }
    }
}
