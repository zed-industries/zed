mod keymap_file;
mod settings_file;
mod settings_store;

use anyhow::bail;
use gpui::{
    font_cache::{FamilyId, FontCache},
    fonts, AppContext, AssetSource,
};
use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, ObjectValidation, Schema, SchemaObject, SingleOrVec},
    JsonSchema,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlez::{
    bindable::{Bind, Column, StaticColumnCount},
    statement::Statement,
};
use std::{borrow::Cow, collections::HashMap, num::NonZeroU32, path::Path, str, sync::Arc};
use theme::{Theme, ThemeRegistry};
use util::ResultExt as _;

pub use keymap_file::{keymap_file_json_schema, KeymapFileContent};
pub use settings_file::*;
pub use settings_store::{Setting, SettingsJsonSchemaParams, SettingsStore};

pub const DEFAULT_SETTINGS_ASSET_PATH: &str = "settings/default.json";
pub const INITIAL_USER_SETTINGS_ASSET_PATH: &str = "settings/initial_user_settings.json";

#[derive(Clone)]
pub struct Settings {
    pub features: Features,
    pub buffer_font_family_name: String,
    pub buffer_font_features: fonts::Features,
    pub buffer_font_family: FamilyId,
    pub default_buffer_font_size: f32,
    pub buffer_font_size: f32,
    pub active_pane_magnification: f32,
    pub cursor_blink: bool,
    pub confirm_quit: bool,
    pub hover_popover_enabled: bool,
    pub show_completions_on_input: bool,
    pub show_call_status_icon: bool,
    pub autosave: Autosave,
    pub default_dock_anchor: DockAnchor,
    pub editor_defaults: EditorSettings,
    pub editor_overrides: EditorSettings,
    pub git: GitSettings,
    pub git_overrides: GitSettings,
    pub copilot: CopilotSettings,
    pub journal_defaults: JournalSettings,
    pub journal_overrides: JournalSettings,
    pub terminal_defaults: TerminalSettings,
    pub terminal_overrides: TerminalSettings,
    pub language_defaults: HashMap<Arc<str>, EditorSettings>,
    pub language_overrides: HashMap<Arc<str>, EditorSettings>,
    pub lsp: HashMap<Arc<str>, LspSettings>,
    pub theme: Arc<Theme>,
    pub base_keymap: BaseKeymap,
}

impl Setting for Settings {
    const KEY: Option<&'static str> = None;

    type FileContent = SettingsFileContent;

    fn load(
        defaults: &Self::FileContent,
        user_values: &[&Self::FileContent],
        cx: &AppContext,
    ) -> Self {
        let buffer_font_features = defaults.buffer_font_features.clone().unwrap();
        let themes = cx.global::<Arc<ThemeRegistry>>();

        let mut this = Self {
            buffer_font_family: cx
                .font_cache()
                .load_family(
                    &[defaults.buffer_font_family.as_ref().unwrap()],
                    &buffer_font_features,
                )
                .unwrap(),
            buffer_font_family_name: defaults.buffer_font_family.clone().unwrap(),
            buffer_font_features,
            buffer_font_size: defaults.buffer_font_size.unwrap(),
            active_pane_magnification: defaults.active_pane_magnification.unwrap(),
            default_buffer_font_size: defaults.buffer_font_size.unwrap(),
            confirm_quit: defaults.confirm_quit.unwrap(),
            cursor_blink: defaults.cursor_blink.unwrap(),
            hover_popover_enabled: defaults.hover_popover_enabled.unwrap(),
            show_completions_on_input: defaults.show_completions_on_input.unwrap(),
            show_call_status_icon: defaults.show_call_status_icon.unwrap(),
            autosave: defaults.autosave.unwrap(),
            default_dock_anchor: defaults.default_dock_anchor.unwrap(),
            editor_defaults: EditorSettings {
                tab_size: defaults.editor.tab_size,
                hard_tabs: defaults.editor.hard_tabs,
                soft_wrap: defaults.editor.soft_wrap,
                preferred_line_length: defaults.editor.preferred_line_length,
                remove_trailing_whitespace_on_save: defaults
                    .editor
                    .remove_trailing_whitespace_on_save,
                ensure_final_newline_on_save: defaults.editor.ensure_final_newline_on_save,
                format_on_save: defaults.editor.format_on_save.clone(),
                formatter: defaults.editor.formatter.clone(),
                enable_language_server: defaults.editor.enable_language_server,
                show_copilot_suggestions: defaults.editor.show_copilot_suggestions,
                show_whitespaces: defaults.editor.show_whitespaces,
            },
            editor_overrides: Default::default(),
            copilot: CopilotSettings {
                disabled_globs: defaults
                    .copilot
                    .clone()
                    .unwrap()
                    .disabled_globs
                    .unwrap()
                    .into_iter()
                    .map(|s| glob::Pattern::new(&s).unwrap())
                    .collect(),
            },
            git: defaults.git.unwrap(),
            git_overrides: Default::default(),
            journal_defaults: defaults.journal.clone(),
            journal_overrides: Default::default(),
            terminal_defaults: defaults.terminal.clone(),
            terminal_overrides: Default::default(),
            language_defaults: defaults.languages.clone(),
            language_overrides: Default::default(),
            lsp: defaults.lsp.clone(),
            theme: themes.get(defaults.theme.as_ref().unwrap()).unwrap(),
            base_keymap: Default::default(),
            features: Features {
                copilot: defaults.features.copilot.unwrap(),
            },
        };

        for value in user_values.into_iter().copied().cloned() {
            this.set_user_settings(value, themes.as_ref(), cx.font_cache());
        }

        this
    }

    fn json_schema(
        generator: &mut SchemaGenerator,
        params: &SettingsJsonSchemaParams,
    ) -> schemars::schema::RootSchema {
        let mut root_schema = generator.root_schema_for::<SettingsFileContent>();

        // Create a schema for a theme name.
        let theme_name_schema = SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            enum_values: Some(
                params
                    .theme_names
                    .iter()
                    .cloned()
                    .map(Value::String)
                    .collect(),
            ),
            ..Default::default()
        };

        // Create a schema for a 'languages overrides' object, associating editor
        // settings with specific langauges.
        assert!(root_schema.definitions.contains_key("EditorSettings"));

        let languages_object_schema = SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Object))),
            object: Some(Box::new(ObjectValidation {
                properties: params
                    .language_names
                    .iter()
                    .map(|name| {
                        (
                            name.clone(),
                            Schema::new_ref("#/definitions/EditorSettings".into()),
                        )
                    })
                    .collect(),
                ..Default::default()
            })),
            ..Default::default()
        };

        // Add these new schemas as definitions, and modify properties of the root
        // schema to reference them.
        root_schema.definitions.extend([
            ("ThemeName".into(), theme_name_schema.into()),
            ("Languages".into(), languages_object_schema.into()),
        ]);
        let root_schema_object = &mut root_schema.schema.object.as_mut().unwrap();

        root_schema_object.properties.extend([
            (
                "theme".to_owned(),
                Schema::new_ref("#/definitions/ThemeName".into()),
            ),
            (
                "languages".to_owned(),
                Schema::new_ref("#/definitions/Languages".into()),
            ),
            // For backward compatibility
            (
                "language_overrides".to_owned(),
                Schema::new_ref("#/definitions/Languages".into()),
            ),
        ]);

        root_schema
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub enum BaseKeymap {
    #[default]
    VSCode,
    JetBrains,
    SublimeText,
    Atom,
    TextMate,
}

impl BaseKeymap {
    pub const OPTIONS: [(&'static str, Self); 5] = [
        ("VSCode (Default)", Self::VSCode),
        ("Atom", Self::Atom),
        ("JetBrains", Self::JetBrains),
        ("Sublime Text", Self::SublimeText),
        ("TextMate", Self::TextMate),
    ];

    pub fn asset_path(&self) -> Option<&'static str> {
        match self {
            BaseKeymap::JetBrains => Some("keymaps/jetbrains.json"),
            BaseKeymap::SublimeText => Some("keymaps/sublime_text.json"),
            BaseKeymap::Atom => Some("keymaps/atom.json"),
            BaseKeymap::TextMate => Some("keymaps/textmate.json"),
            BaseKeymap::VSCode => None,
        }
    }

    pub fn names() -> impl Iterator<Item = &'static str> {
        Self::OPTIONS.iter().map(|(name, _)| *name)
    }

    pub fn from_names(option: &str) -> BaseKeymap {
        Self::OPTIONS
            .iter()
            .copied()
            .find_map(|(name, value)| (name == option).then(|| value))
            .unwrap_or_default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct CopilotSettings {
    pub disabled_globs: Vec<glob::Pattern>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct CopilotSettingsContent {
    #[serde(default)]
    pub disabled_globs: Option<Vec<String>>,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct GitSettings {
    pub git_gutter: Option<GitGutter>,
    pub gutter_debounce: Option<u64>,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GitGutter {
    #[default]
    TrackedFiles,
    Hide,
}

pub struct GitGutterConfig {}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct EditorSettings {
    pub tab_size: Option<NonZeroU32>,
    pub hard_tabs: Option<bool>,
    pub soft_wrap: Option<SoftWrap>,
    pub preferred_line_length: Option<u32>,
    pub format_on_save: Option<FormatOnSave>,
    pub remove_trailing_whitespace_on_save: Option<bool>,
    pub ensure_final_newline_on_save: Option<bool>,
    pub formatter: Option<Formatter>,
    pub enable_language_server: Option<bool>,
    pub show_copilot_suggestions: Option<bool>,
    pub show_whitespaces: Option<ShowWhitespaces>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SoftWrap {
    None,
    EditorWidth,
    PreferredLineLength,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FormatOnSave {
    On,
    Off,
    LanguageServer,
    External {
        command: String,
        arguments: Vec<String>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Formatter {
    LanguageServer,
    External {
        command: String,
        arguments: Vec<String>,
    },
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Autosave {
    Off,
    AfterDelay { milliseconds: u64 },
    OnFocusChange,
    OnWindowChange,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct JournalSettings {
    pub path: Option<String>,
    pub hour_format: Option<HourFormat>,
}

impl Default for JournalSettings {
    fn default() -> Self {
        Self {
            path: Some("~".into()),
            hour_format: Some(Default::default()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HourFormat {
    Hour12,
    Hour24,
}

impl Default for HourFormat {
    fn default() -> Self {
        Self::Hour12
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct TerminalSettings {
    pub shell: Option<Shell>,
    pub working_directory: Option<WorkingDirectory>,
    pub font_size: Option<f32>,
    pub font_family: Option<String>,
    pub line_height: Option<TerminalLineHeight>,
    pub font_features: Option<fonts::Features>,
    pub env: Option<HashMap<String, String>>,
    pub blinking: Option<TerminalBlink>,
    pub alternate_scroll: Option<AlternateScroll>,
    pub option_as_meta: Option<bool>,
    pub copy_on_select: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum TerminalLineHeight {
    #[default]
    Comfortable,
    Standard,
    Custom(f32),
}

impl TerminalLineHeight {
    fn value(&self) -> f32 {
        match self {
            TerminalLineHeight::Comfortable => 1.618,
            TerminalLineHeight::Standard => 1.3,
            TerminalLineHeight::Custom(line_height) => *line_height,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TerminalBlink {
    Off,
    TerminalControlled,
    On,
}

impl Default for TerminalBlink {
    fn default() -> Self {
        TerminalBlink::TerminalControlled
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Shell {
    System,
    Program(String),
    WithArguments { program: String, args: Vec<String> },
}

impl Default for Shell {
    fn default() -> Self {
        Shell::System
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AlternateScroll {
    On,
    Off,
}

impl Default for AlternateScroll {
    fn default() -> Self {
        AlternateScroll::On
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WorkingDirectory {
    CurrentProjectDirectory,
    FirstProjectDirectory,
    AlwaysHome,
    Always { directory: String },
}

impl Default for WorkingDirectory {
    fn default() -> Self {
        Self::CurrentProjectDirectory
    }
}

impl TerminalSettings {
    fn line_height(&self) -> Option<f32> {
        self.line_height
            .to_owned()
            .map(|line_height| line_height.value())
    }
}

#[derive(PartialEq, Eq, Debug, Default, Copy, Clone, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DockAnchor {
    #[default]
    Bottom,
    Right,
    Expanded,
}

impl StaticColumnCount for DockAnchor {}
impl Bind for DockAnchor {
    fn bind(&self, statement: &Statement, start_index: i32) -> anyhow::Result<i32> {
        match self {
            DockAnchor::Bottom => "Bottom",
            DockAnchor::Right => "Right",
            DockAnchor::Expanded => "Expanded",
        }
        .bind(statement, start_index)
    }
}

impl Column for DockAnchor {
    fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
        String::column(statement, start_index).and_then(|(anchor_text, next_index)| {
            Ok((
                match anchor_text.as_ref() {
                    "Bottom" => DockAnchor::Bottom,
                    "Right" => DockAnchor::Right,
                    "Expanded" => DockAnchor::Expanded,
                    _ => bail!("Stored dock anchor is incorrect"),
                },
                next_index,
            ))
        })
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct SettingsFileContent {
    #[serde(default)]
    pub buffer_font_family: Option<String>,
    #[serde(default)]
    pub buffer_font_size: Option<f32>,
    #[serde(default)]
    pub buffer_font_features: Option<fonts::Features>,
    #[serde(default)]
    pub copilot: Option<CopilotSettingsContent>,
    #[serde(default)]
    pub active_pane_magnification: Option<f32>,
    #[serde(default)]
    pub cursor_blink: Option<bool>,
    #[serde(default)]
    pub confirm_quit: Option<bool>,
    #[serde(default)]
    pub hover_popover_enabled: Option<bool>,
    #[serde(default)]
    pub show_completions_on_input: Option<bool>,
    #[serde(default)]
    pub show_call_status_icon: Option<bool>,
    #[serde(default)]
    pub autosave: Option<Autosave>,
    #[serde(default)]
    pub default_dock_anchor: Option<DockAnchor>,
    #[serde(flatten)]
    pub editor: EditorSettings,
    #[serde(default)]
    pub journal: JournalSettings,
    #[serde(default)]
    pub terminal: TerminalSettings,
    #[serde(default)]
    pub git: Option<GitSettings>,
    #[serde(default)]
    #[serde(alias = "language_overrides")]
    pub languages: HashMap<Arc<str>, EditorSettings>,
    #[serde(default)]
    pub lsp: HashMap<Arc<str>, LspSettings>,
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(default)]
    pub base_keymap: Option<BaseKeymap>,
    #[serde(default)]
    pub features: FeaturesContent,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LspSettings {
    pub initialization_options: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Features {
    pub copilot: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FeaturesContent {
    pub copilot: Option<bool>,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ShowWhitespaces {
    #[default]
    Selection,
    None,
    All,
}

impl Settings {
    pub fn initial_user_settings_content(assets: &'static impl AssetSource) -> Cow<'static, str> {
        match assets.load(INITIAL_USER_SETTINGS_ASSET_PATH).unwrap() {
            Cow::Borrowed(s) => Cow::Borrowed(str::from_utf8(s).unwrap()),
            Cow::Owned(s) => Cow::Owned(String::from_utf8(s).unwrap()),
        }
    }

    /// Fill out the settings corresponding to the default.json file, overrides will be set later
    pub fn defaults(
        assets: impl AssetSource,
        font_cache: &FontCache,
        themes: &ThemeRegistry,
    ) -> Self {
        #[track_caller]
        fn required<T>(value: Option<T>) -> Option<T> {
            assert!(value.is_some(), "missing default setting value");
            value
        }

        let defaults: SettingsFileContent = settings_store::parse_json_with_comments(
            str::from_utf8(assets.load(DEFAULT_SETTINGS_ASSET_PATH).unwrap().as_ref()).unwrap(),
        )
        .unwrap();

        let buffer_font_features = defaults.buffer_font_features.unwrap();
        Self {
            buffer_font_family: font_cache
                .load_family(
                    &[defaults.buffer_font_family.as_ref().unwrap()],
                    &buffer_font_features,
                )
                .unwrap(),
            buffer_font_family_name: defaults.buffer_font_family.unwrap(),
            buffer_font_features,
            buffer_font_size: defaults.buffer_font_size.unwrap(),
            active_pane_magnification: defaults.active_pane_magnification.unwrap(),
            default_buffer_font_size: defaults.buffer_font_size.unwrap(),
            confirm_quit: defaults.confirm_quit.unwrap(),
            cursor_blink: defaults.cursor_blink.unwrap(),
            hover_popover_enabled: defaults.hover_popover_enabled.unwrap(),
            show_completions_on_input: defaults.show_completions_on_input.unwrap(),
            show_call_status_icon: defaults.show_call_status_icon.unwrap(),
            autosave: defaults.autosave.unwrap(),
            default_dock_anchor: defaults.default_dock_anchor.unwrap(),
            editor_defaults: EditorSettings {
                tab_size: required(defaults.editor.tab_size),
                hard_tabs: required(defaults.editor.hard_tabs),
                soft_wrap: required(defaults.editor.soft_wrap),
                preferred_line_length: required(defaults.editor.preferred_line_length),
                remove_trailing_whitespace_on_save: required(
                    defaults.editor.remove_trailing_whitespace_on_save,
                ),
                ensure_final_newline_on_save: required(
                    defaults.editor.ensure_final_newline_on_save,
                ),
                format_on_save: required(defaults.editor.format_on_save),
                formatter: required(defaults.editor.formatter),
                enable_language_server: required(defaults.editor.enable_language_server),
                show_copilot_suggestions: required(defaults.editor.show_copilot_suggestions),
                show_whitespaces: required(defaults.editor.show_whitespaces),
            },
            editor_overrides: Default::default(),
            copilot: CopilotSettings {
                disabled_globs: defaults
                    .copilot
                    .unwrap()
                    .disabled_globs
                    .unwrap()
                    .into_iter()
                    .map(|s| glob::Pattern::new(&s).unwrap())
                    .collect(),
            },
            git: defaults.git.unwrap(),
            git_overrides: Default::default(),
            journal_defaults: defaults.journal,
            journal_overrides: Default::default(),
            terminal_defaults: defaults.terminal,
            terminal_overrides: Default::default(),
            language_defaults: defaults.languages,
            language_overrides: Default::default(),
            lsp: defaults.lsp.clone(),
            theme: themes.get(&defaults.theme.unwrap()).unwrap(),
            base_keymap: Default::default(),
            features: Features {
                copilot: defaults.features.copilot.unwrap(),
            },
        }
    }

    // Fill out the overrride and etc. settings from the user's settings.json
    pub fn set_user_settings(
        &mut self,
        data: SettingsFileContent,
        theme_registry: &ThemeRegistry,
        font_cache: &FontCache,
    ) {
        let mut family_changed = false;
        if let Some(value) = data.buffer_font_family {
            self.buffer_font_family_name = value;
            family_changed = true;
        }
        if let Some(value) = data.buffer_font_features {
            self.buffer_font_features = value;
            family_changed = true;
        }
        if family_changed {
            if let Some(id) = font_cache
                .load_family(&[&self.buffer_font_family_name], &self.buffer_font_features)
                .log_err()
            {
                self.buffer_font_family = id;
            }
        }

        if let Some(value) = &data.theme {
            if let Some(theme) = theme_registry.get(value).log_err() {
                self.theme = theme;
            }
        }

        merge(&mut self.buffer_font_size, data.buffer_font_size);
        merge(
            &mut self.active_pane_magnification,
            data.active_pane_magnification,
        );
        merge(&mut self.default_buffer_font_size, data.buffer_font_size);
        merge(&mut self.cursor_blink, data.cursor_blink);
        merge(&mut self.confirm_quit, data.confirm_quit);
        merge(&mut self.hover_popover_enabled, data.hover_popover_enabled);
        merge(
            &mut self.show_completions_on_input,
            data.show_completions_on_input,
        );
        merge(&mut self.autosave, data.autosave);
        merge(&mut self.default_dock_anchor, data.default_dock_anchor);
        merge(&mut self.base_keymap, data.base_keymap);
        merge(&mut self.features.copilot, data.features.copilot);

        if let Some(copilot) = data.copilot {
            if let Some(disabled_globs) = copilot.disabled_globs {
                self.copilot.disabled_globs = disabled_globs
                    .into_iter()
                    .filter_map(|s| glob::Pattern::new(&s).ok())
                    .collect()
            }
        }
        self.editor_overrides = data.editor;
        self.git_overrides = data.git.unwrap_or_default();
        self.journal_overrides = data.journal;
        self.terminal_defaults.font_size = data.terminal.font_size;
        self.terminal_overrides.copy_on_select = data.terminal.copy_on_select;
        self.terminal_overrides = data.terminal;
        self.language_overrides = data.languages;
        self.lsp = data.lsp;
    }

    pub fn with_language_defaults(
        mut self,
        language_name: impl Into<Arc<str>>,
        overrides: EditorSettings,
    ) -> Self {
        self.language_defaults
            .insert(language_name.into(), overrides);
        self
    }

    pub fn features(&self) -> &Features {
        &self.features
    }

    pub fn show_copilot_suggestions(&self, language: Option<&str>, path: Option<&Path>) -> bool {
        if !self.features.copilot {
            return false;
        }

        if !self.copilot_enabled_for_language(language) {
            return false;
        }

        if let Some(path) = path {
            if !self.copilot_enabled_for_path(path) {
                return false;
            }
        }

        true
    }

    pub fn copilot_enabled_for_path(&self, path: &Path) -> bool {
        !self
            .copilot
            .disabled_globs
            .iter()
            .any(|glob| glob.matches_path(path))
    }

    pub fn copilot_enabled_for_language(&self, language: Option<&str>) -> bool {
        self.language_setting(language, |settings| settings.show_copilot_suggestions)
    }

    pub fn tab_size(&self, language: Option<&str>) -> NonZeroU32 {
        self.language_setting(language, |settings| settings.tab_size)
    }

    pub fn show_whitespaces(&self, language: Option<&str>) -> ShowWhitespaces {
        self.language_setting(language, |settings| settings.show_whitespaces)
    }

    pub fn hard_tabs(&self, language: Option<&str>) -> bool {
        self.language_setting(language, |settings| settings.hard_tabs)
    }

    pub fn soft_wrap(&self, language: Option<&str>) -> SoftWrap {
        self.language_setting(language, |settings| settings.soft_wrap)
    }

    pub fn preferred_line_length(&self, language: Option<&str>) -> u32 {
        self.language_setting(language, |settings| settings.preferred_line_length)
    }

    pub fn remove_trailing_whitespace_on_save(&self, language: Option<&str>) -> bool {
        self.language_setting(language, |settings| {
            settings.remove_trailing_whitespace_on_save.clone()
        })
    }

    pub fn ensure_final_newline_on_save(&self, language: Option<&str>) -> bool {
        self.language_setting(language, |settings| {
            settings.ensure_final_newline_on_save.clone()
        })
    }

    pub fn format_on_save(&self, language: Option<&str>) -> FormatOnSave {
        self.language_setting(language, |settings| settings.format_on_save.clone())
    }

    pub fn formatter(&self, language: Option<&str>) -> Formatter {
        self.language_setting(language, |settings| settings.formatter.clone())
    }

    pub fn enable_language_server(&self, language: Option<&str>) -> bool {
        self.language_setting(language, |settings| settings.enable_language_server)
    }

    fn language_setting<F, R>(&self, language: Option<&str>, f: F) -> R
    where
        F: Fn(&EditorSettings) -> Option<R>,
    {
        None.or_else(|| language.and_then(|l| self.language_overrides.get(l).and_then(&f)))
            .or_else(|| f(&self.editor_overrides))
            .or_else(|| language.and_then(|l| self.language_defaults.get(l).and_then(&f)))
            .or_else(|| f(&self.editor_defaults))
            .expect("missing default")
    }

    pub fn git_gutter(&self) -> GitGutter {
        self.git_overrides.git_gutter.unwrap_or_else(|| {
            self.git
                .git_gutter
                .expect("git_gutter should be some by setting setup")
        })
    }

    fn terminal_setting<F, R>(&self, f: F) -> R
    where
        F: Fn(&TerminalSettings) -> Option<R>,
    {
        None.or_else(|| f(&self.terminal_overrides))
            .or_else(|| f(&self.terminal_defaults))
            .expect("missing default")
    }

    pub fn terminal_line_height(&self) -> f32 {
        self.terminal_setting(|terminal_setting| terminal_setting.line_height())
    }

    pub fn terminal_scroll(&self) -> AlternateScroll {
        self.terminal_setting(|terminal_setting| terminal_setting.alternate_scroll.to_owned())
    }

    pub fn terminal_shell(&self) -> Shell {
        self.terminal_setting(|terminal_setting| terminal_setting.shell.to_owned())
    }

    pub fn terminal_env(&self) -> HashMap<String, String> {
        self.terminal_setting(|terminal_setting| terminal_setting.env.to_owned())
    }

    pub fn terminal_strategy(&self) -> WorkingDirectory {
        self.terminal_setting(|terminal_setting| terminal_setting.working_directory.to_owned())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &gpui::AppContext) -> Settings {
        Settings {
            buffer_font_family_name: "Monaco".to_string(),
            buffer_font_features: Default::default(),
            buffer_font_family: cx
                .font_cache()
                .load_family(&["Monaco"], &Default::default())
                .unwrap(),
            buffer_font_size: 14.,
            active_pane_magnification: 1.,
            default_buffer_font_size: 14.,
            confirm_quit: false,
            cursor_blink: true,
            hover_popover_enabled: true,
            show_completions_on_input: true,
            show_call_status_icon: true,
            autosave: Autosave::Off,
            default_dock_anchor: DockAnchor::Bottom,
            editor_defaults: EditorSettings {
                tab_size: Some(4.try_into().unwrap()),
                hard_tabs: Some(false),
                soft_wrap: Some(SoftWrap::None),
                preferred_line_length: Some(80),
                remove_trailing_whitespace_on_save: Some(true),
                ensure_final_newline_on_save: Some(true),
                format_on_save: Some(FormatOnSave::On),
                formatter: Some(Formatter::LanguageServer),
                enable_language_server: Some(true),
                show_copilot_suggestions: Some(true),
                show_whitespaces: Some(ShowWhitespaces::None),
            },
            editor_overrides: Default::default(),
            copilot: Default::default(),
            journal_defaults: Default::default(),
            journal_overrides: Default::default(),
            terminal_defaults: Default::default(),
            terminal_overrides: Default::default(),
            git: Default::default(),
            git_overrides: Default::default(),
            language_defaults: Default::default(),
            language_overrides: Default::default(),
            lsp: Default::default(),
            theme: gpui::fonts::with_font_cache(cx.font_cache().clone(), Default::default),
            base_keymap: Default::default(),
            features: Features { copilot: true },
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_async(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let settings = Self::test(cx);
            cx.set_global(settings);
        });
    }
}

fn merge<T: Copy>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}
