mod keymap_file;
pub mod settings_file;
pub mod watched_json;

use anyhow::{bail, Result};
use gpui::{
    font_cache::{FamilyId, FontCache},
    AssetSource,
};
use schemars::{
    gen::{SchemaGenerator, SchemaSettings},
    schema::{InstanceType, ObjectValidation, Schema, SchemaObject, SingleOrVec},
    JsonSchema,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use sqlez::{
    bindable::{Bind, Column, StaticColumnCount},
    statement::Statement,
};
use std::{collections::HashMap, num::NonZeroU32, str, sync::Arc};
use theme::{Theme, ThemeRegistry};
use tree_sitter::Query;
use util::ResultExt as _;

pub use keymap_file::{keymap_file_json_schema, KeymapFileContent};
pub use watched_json::watch_files;

#[derive(Clone)]
pub struct Settings {
    pub buffer_font_family: FamilyId,
    pub default_buffer_font_size: f32,
    pub buffer_font_size: f32,
    pub active_pane_magnification: f32,
    pub cursor_blink: bool,
    pub confirm_quit: bool,
    pub hover_popover_enabled: bool,
    pub show_completions_on_input: bool,
    pub show_call_status_icon: bool,
    pub vim_mode: bool,
    pub autosave: Autosave,
    pub default_dock_anchor: DockAnchor,
    pub editor_defaults: EditorSettings,
    pub editor_overrides: EditorSettings,
    pub git: GitSettings,
    pub git_overrides: GitSettings,
    pub journal_defaults: JournalSettings,
    pub journal_overrides: JournalSettings,
    pub terminal_defaults: TerminalSettings,
    pub terminal_overrides: TerminalSettings,
    pub language_defaults: HashMap<Arc<str>, EditorSettings>,
    pub language_overrides: HashMap<Arc<str>, EditorSettings>,
    pub lsp: HashMap<Arc<str>, LspSettings>,
    pub theme: Arc<Theme>,
    pub telemetry_defaults: TelemetrySettings,
    pub telemetry_overrides: TelemetrySettings,
    pub auto_update: bool,
    pub base_keymap: BaseKeymap,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub enum BaseKeymap {
    #[default]
    VSCode,
    JetBrains,
    Sublime,
    Atom,
}

impl BaseKeymap {
    pub const OPTIONS: [(&'static str, Self); 4] = [
        ("VSCode (Default)", Self::VSCode),
        ("Atom", Self::Atom),
        ("JetBrains", Self::JetBrains),
        ("Sublime", Self::Sublime),
    ];

    pub fn asset_path(&self) -> Option<&'static str> {
        match self {
            BaseKeymap::JetBrains => Some("keymaps/jetbrains.json"),
            BaseKeymap::Sublime => Some("keymaps/sublime_text.json"),
            BaseKeymap::Atom => Some("keymaps/atom.json"),
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

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct TelemetrySettings {
    diagnostics: Option<bool>,
    metrics: Option<bool>,
}

impl TelemetrySettings {
    pub fn metrics(&self) -> bool {
        self.metrics.unwrap()
    }

    pub fn diagnostics(&self) -> bool {
        self.diagnostics.unwrap()
    }

    pub fn set_metrics(&mut self, value: bool) {
        self.metrics = Some(value);
    }

    pub fn set_diagnostics(&mut self, value: bool) {
        self.diagnostics = Some(value);
    }
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
    pub env: Option<HashMap<String, String>>,
    pub blinking: Option<TerminalBlink>,
    pub alternate_scroll: Option<AlternateScroll>,
    pub option_as_meta: Option<bool>,
    pub copy_on_select: Option<bool>,
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
    pub projects_online_by_default: Option<bool>,
    #[serde(default)]
    pub buffer_font_family: Option<String>,
    #[serde(default)]
    pub buffer_font_size: Option<f32>,
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
    pub vim_mode: Option<bool>,
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
    pub telemetry: TelemetrySettings,
    #[serde(default)]
    pub auto_update: Option<bool>,
    #[serde(default)]
    pub base_keymap: Option<BaseKeymap>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LspSettings {
    pub initialization_options: Option<Value>,
}

impl Settings {
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

        let defaults: SettingsFileContent = parse_json_with_comments(
            str::from_utf8(assets.load("settings/default.json").unwrap().as_ref()).unwrap(),
        )
        .unwrap();

        Self {
            buffer_font_family: font_cache
                .load_family(&[defaults.buffer_font_family.as_ref().unwrap()])
                .unwrap(),
            buffer_font_size: defaults.buffer_font_size.unwrap(),
            active_pane_magnification: defaults.active_pane_magnification.unwrap(),
            default_buffer_font_size: defaults.buffer_font_size.unwrap(),
            confirm_quit: defaults.confirm_quit.unwrap(),
            cursor_blink: defaults.cursor_blink.unwrap(),
            hover_popover_enabled: defaults.hover_popover_enabled.unwrap(),
            show_completions_on_input: defaults.show_completions_on_input.unwrap(),
            show_call_status_icon: defaults.show_call_status_icon.unwrap(),
            vim_mode: defaults.vim_mode.unwrap(),
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
            },
            editor_overrides: Default::default(),
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
            telemetry_defaults: defaults.telemetry,
            telemetry_overrides: Default::default(),
            auto_update: defaults.auto_update.unwrap(),
            base_keymap: Default::default(),
        }
    }

    // Fill out the overrride and etc. settings from the user's settings.json
    pub fn set_user_settings(
        &mut self,
        data: SettingsFileContent,
        theme_registry: &ThemeRegistry,
        font_cache: &FontCache,
    ) {
        if let Some(value) = &data.buffer_font_family {
            if let Some(id) = font_cache.load_family(&[value]).log_err() {
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
        merge(&mut self.vim_mode, data.vim_mode);
        merge(&mut self.autosave, data.autosave);
        merge(&mut self.default_dock_anchor, data.default_dock_anchor);
        merge(&mut self.base_keymap, data.base_keymap);

        // Ensure terminal font is loaded, so we can request it in terminal_element layout
        if let Some(terminal_font) = &data.terminal.font_family {
            font_cache.load_family(&[terminal_font]).log_err();
        }

        self.editor_overrides = data.editor;
        self.git_overrides = data.git.unwrap_or_default();
        self.journal_overrides = data.journal;
        self.terminal_defaults.font_size = data.terminal.font_size;
        self.terminal_overrides.copy_on_select = data.terminal.copy_on_select;
        self.terminal_overrides = data.terminal;
        self.language_overrides = data.languages;
        self.telemetry_overrides = data.telemetry;
        self.lsp = data.lsp;
        merge(&mut self.auto_update, data.auto_update);
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

    pub fn tab_size(&self, language: Option<&str>) -> NonZeroU32 {
        self.language_setting(language, |settings| settings.tab_size)
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

    fn terminal_setting<F, R: Default + Clone>(&self, f: F) -> R
    where
        F: Fn(&TerminalSettings) -> Option<&R>,
    {
        f(&self.terminal_overrides)
            .or_else(|| f(&self.terminal_defaults))
            .cloned()
            .unwrap_or_else(|| R::default())
    }

    pub fn telemetry(&self) -> TelemetrySettings {
        TelemetrySettings {
            diagnostics: Some(self.telemetry_diagnostics()),
            metrics: Some(self.telemetry_metrics()),
        }
    }

    pub fn telemetry_diagnostics(&self) -> bool {
        self.telemetry_overrides
            .diagnostics
            .or(self.telemetry_defaults.diagnostics)
            .expect("missing default")
    }

    pub fn telemetry_metrics(&self) -> bool {
        self.telemetry_overrides
            .metrics
            .or(self.telemetry_defaults.metrics)
            .expect("missing default")
    }

    pub fn terminal_scroll(&self) -> AlternateScroll {
        self.terminal_setting(|terminal_setting| terminal_setting.alternate_scroll.as_ref())
    }

    pub fn terminal_shell(&self) -> Shell {
        self.terminal_setting(|terminal_setting| terminal_setting.shell.as_ref())
    }

    pub fn terminal_env(&self) -> HashMap<String, String> {
        self.terminal_setting(|terminal_setting| terminal_setting.env.as_ref())
    }

    pub fn terminal_strategy(&self) -> WorkingDirectory {
        self.terminal_setting(|terminal_setting| terminal_setting.working_directory.as_ref())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &gpui::AppContext) -> Settings {
        Settings {
            buffer_font_family: cx.font_cache().load_family(&["Monaco"]).unwrap(),
            buffer_font_size: 14.,
            active_pane_magnification: 1.,
            default_buffer_font_size: 14.,
            confirm_quit: false,
            cursor_blink: true,
            hover_popover_enabled: true,
            show_completions_on_input: true,
            show_call_status_icon: true,
            vim_mode: false,
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
            },
            editor_overrides: Default::default(),
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
            telemetry_defaults: TelemetrySettings {
                diagnostics: Some(true),
                metrics: Some(true),
            },
            telemetry_overrides: Default::default(),
            auto_update: true,
            base_keymap: Default::default(),
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

pub fn settings_file_json_schema(
    theme_names: Vec<String>,
    language_names: &[String],
) -> serde_json::Value {
    let settings = SchemaSettings::draft07().with(|settings| {
        settings.option_add_null_type = false;
    });
    let generator = SchemaGenerator::new(settings);
    let mut root_schema = generator.into_root_schema_for::<SettingsFileContent>();

    // Create a schema for a theme name.
    let theme_name_schema = SchemaObject {
        instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
        enum_values: Some(theme_names.into_iter().map(Value::String).collect()),
        ..Default::default()
    };

    // Create a schema for a 'languages overrides' object, associating editor
    // settings with specific langauges.
    assert!(root_schema.definitions.contains_key("EditorSettings"));
    let languages_object_schema = SchemaObject {
        instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Object))),
        object: Some(Box::new(ObjectValidation {
            properties: language_names
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

    serde_json::to_value(root_schema).unwrap()
}

fn merge<T: Copy>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}

pub fn parse_json_with_comments<T: DeserializeOwned>(content: &str) -> Result<T> {
    Ok(serde_json::from_reader(
        json_comments::CommentSettings::c_style().strip_comments(content.as_bytes()),
    )?)
}

fn write_settings_key(settings_content: &mut String, key_path: &[&str], new_value: &Value) {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_json::language()).unwrap();
    let tree = parser.parse(&settings_content, None).unwrap();

    let mut cursor = tree_sitter::QueryCursor::new();

    let query = Query::new(
        tree_sitter_json::language(),
        "
            (pair
                key: (string) @key
                value: (_) @value)
        ",
    )
    .unwrap();

    let mut depth = 0;
    let mut first_key_start = None;
    let mut existing_value_range = 0..settings_content.len();
    let matches = cursor.matches(&query, tree.root_node(), settings_content.as_bytes());
    for mat in matches {
        if mat.captures.len() != 2 {
            continue;
        }

        let key_range = mat.captures[0].node.byte_range();
        let value_range = mat.captures[1].node.byte_range();

        if key_range.start > existing_value_range.end {
            break;
        }

        first_key_start.get_or_insert_with(|| key_range.start);

        let found_key = settings_content
            .get(key_range.clone())
            .map(|key_text| key_text == format!("\"{}\"", key_path[depth]))
            .unwrap_or(false);

        if found_key {
            existing_value_range = value_range;
            depth += 1;

            if depth == key_path.len() {
                break;
            } else {
                first_key_start = None;
            }
        }
    }

    // We found the exact key we want, insert the new value
    if depth == key_path.len() {
        let new_val = serde_json::to_string_pretty(new_value)
            .expect("Could not serialize new json field to string");
        settings_content.replace_range(existing_value_range, &new_val);
    } else {
        // We have key paths, construct the sub objects
        let new_key = key_path[depth];

        // We don't have the key, construct the nested objects
        let mut new_value = serde_json::to_value(new_value).unwrap();
        for key in key_path[(depth + 1)..].iter().rev() {
            new_value = serde_json::json!({ key.to_string(): new_value });
        }

        if let Some(first_key_start) = first_key_start {
            let mut row = 0;
            let mut column = 0;
            for (ix, char) in settings_content.char_indices() {
                if ix == first_key_start {
                    break;
                }
                if char == '\n' {
                    row += 1;
                    column = 0;
                } else {
                    column += char.len_utf8();
                }
            }

            if row > 0 {
                let new_val = to_pretty_json(&new_value, column, column);
                let content = format!(r#""{new_key}": {new_val},"#);
                settings_content.insert_str(first_key_start, &content);

                settings_content.insert_str(
                    first_key_start + content.len(),
                    &format!("\n{:width$}", ' ', width = column),
                )
            } else {
                let new_val = serde_json::to_string(&new_value).unwrap();
                let mut content = format!(r#""{new_key}": {new_val},"#);
                content.push(' ');
                settings_content.insert_str(first_key_start, &content);
            }
        } else {
            new_value = serde_json::json!({ new_key.to_string(): new_value });
            let indent_prefix_len = 4 * depth;
            let new_val = to_pretty_json(&new_value, 4, indent_prefix_len);

            settings_content.replace_range(existing_value_range, &new_val);
            if depth == 0 {
                settings_content.push('\n');
            }
        }
    }
}

fn to_pretty_json(
    value: &serde_json::Value,
    indent_size: usize,
    indent_prefix_len: usize,
) -> String {
    const SPACES: [u8; 32] = [b' '; 32];

    debug_assert!(indent_size <= SPACES.len());
    debug_assert!(indent_prefix_len <= SPACES.len());

    let mut output = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(
        &mut output,
        serde_json::ser::PrettyFormatter::with_indent(&SPACES[0..indent_size.min(SPACES.len())]),
    );

    value.serialize(&mut ser).unwrap();
    let text = String::from_utf8(output).unwrap();

    let mut adjusted_text = String::new();
    for (i, line) in text.split('\n').enumerate() {
        if i > 0 {
            adjusted_text.push_str(str::from_utf8(&SPACES[0..indent_prefix_len]).unwrap());
        }
        adjusted_text.push_str(line);
        adjusted_text.push('\n');
    }
    adjusted_text.pop();
    adjusted_text
}

pub fn update_settings_file(
    mut text: String,
    old_file_content: SettingsFileContent,
    update: impl FnOnce(&mut SettingsFileContent),
) -> String {
    let mut new_file_content = old_file_content.clone();

    update(&mut new_file_content);

    let old_object = to_json_object(old_file_content);
    let new_object = to_json_object(new_file_content);

    fn apply_changes_to_json_text(
        old_object: &serde_json::Map<String, Value>,
        new_object: &serde_json::Map<String, Value>,
        current_key_path: Vec<&str>,
        json_text: &mut String,
    ) {
        for (key, old_value) in old_object.iter() {
            // We know that these two are from the same shape of object, so we can just unwrap
            let new_value = new_object.get(key).unwrap();
            if old_value != new_value {
                match new_value {
                    Value::Bool(_) | Value::Number(_) | Value::String(_) => {
                        let mut key_path = current_key_path.clone();
                        key_path.push(key);
                        write_settings_key(json_text, &key_path, &new_value);
                    }
                    Value::Object(new_sub_object) => {
                        let mut key_path = current_key_path.clone();
                        key_path.push(key);
                        if let Value::Object(old_sub_object) = old_value {
                            apply_changes_to_json_text(
                                old_sub_object,
                                new_sub_object,
                                key_path,
                                json_text,
                            );
                        } else {
                            unimplemented!("This function doesn't support changing values from simple values to objects yet");
                        }
                    }
                    Value::Null | Value::Array(_) => {
                        unimplemented!("We only support objects and simple values");
                    }
                }
            }
        }
    }

    apply_changes_to_json_text(&old_object, &new_object, vec![], &mut text);

    text
}

fn to_json_object(settings_file: SettingsFileContent) -> serde_json::Map<String, Value> {
    let tmp = serde_json::to_value(settings_file).unwrap();
    match tmp {
        Value::Object(map) => map,
        _ => unreachable!("SettingsFileContent represents a JSON map"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use unindent::Unindent;

    fn assert_new_settings<S1: Into<String>, S2: Into<String>>(
        old_json: S1,
        update: fn(&mut SettingsFileContent),
        expected_new_json: S2,
    ) {
        let old_json = old_json.into();
        let old_content: SettingsFileContent = serde_json::from_str(&old_json).unwrap_or_default();
        let new_json = update_settings_file(old_json, old_content, update);
        assert_eq!(new_json, expected_new_json.into());
    }

    #[test]
    fn test_update_telemetry_setting_multiple_fields() {
        assert_new_settings(
            r#"
                {
                    "telemetry": {
                        "metrics": false,
                        "diagnostics": false
                    }
                }
            "#
            .unindent(),
            |settings| {
                settings.telemetry.set_diagnostics(true);
                settings.telemetry.set_metrics(true);
            },
            r#"
                {
                    "telemetry": {
                        "metrics": true,
                        "diagnostics": true
                    }
                }
            "#
            .unindent(),
        );
    }

    #[test]
    fn test_update_telemetry_setting_weird_formatting() {
        assert_new_settings(
            r#"{
                "telemetry":   { "metrics": false, "diagnostics": true }
            }"#
            .unindent(),
            |settings| settings.telemetry.set_diagnostics(false),
            r#"{
                "telemetry":   { "metrics": false, "diagnostics": false }
            }"#
            .unindent(),
        );
    }

    #[test]
    fn test_update_telemetry_setting_other_fields() {
        assert_new_settings(
            r#"
                {
                    "telemetry": {
                        "metrics": false,
                        "diagnostics": true
                    }
                }
            "#
            .unindent(),
            |settings| settings.telemetry.set_diagnostics(false),
            r#"
                {
                    "telemetry": {
                        "metrics": false,
                        "diagnostics": false
                    }
                }
            "#
            .unindent(),
        );
    }

    #[test]
    fn test_update_telemetry_setting_empty_telemetry() {
        assert_new_settings(
            r#"
                {
                    "telemetry": {}
                }
            "#
            .unindent(),
            |settings| settings.telemetry.set_diagnostics(false),
            r#"
                {
                    "telemetry": {
                        "diagnostics": false
                    }
                }
            "#
            .unindent(),
        );
    }

    #[test]
    fn test_update_telemetry_setting_pre_existing() {
        assert_new_settings(
            r#"
                {
                    "telemetry": {
                        "diagnostics": true
                    }
                }
            "#
            .unindent(),
            |settings| settings.telemetry.set_diagnostics(false),
            r#"
                {
                    "telemetry": {
                        "diagnostics": false
                    }
                }
            "#
            .unindent(),
        );
    }

    #[test]
    fn test_update_telemetry_setting() {
        assert_new_settings(
            "{}",
            |settings| settings.telemetry.set_diagnostics(true),
            r#"
                {
                    "telemetry": {
                        "diagnostics": true
                    }
                }
            "#
            .unindent(),
        );
    }

    #[test]
    fn test_update_object_empty_doc() {
        assert_new_settings(
            "",
            |settings| settings.telemetry.set_diagnostics(true),
            r#"
                {
                    "telemetry": {
                        "diagnostics": true
                    }
                }
            "#
            .unindent(),
        );
    }

    #[test]
    fn test_write_theme_into_settings_with_theme() {
        assert_new_settings(
            r#"
                {
                    "theme": "One Dark"
                }
            "#
            .unindent(),
            |settings| settings.theme = Some("summerfruit-light".to_string()),
            r#"
                {
                    "theme": "summerfruit-light"
                }
            "#
            .unindent(),
        );
    }

    #[test]
    fn test_write_theme_into_empty_settings() {
        assert_new_settings(
            r#"
                {
                }
            "#
            .unindent(),
            |settings| settings.theme = Some("summerfruit-light".to_string()),
            r#"
                {
                    "theme": "summerfruit-light"
                }
            "#
            .unindent(),
        );
    }

    #[test]
    fn write_key_no_document() {
        assert_new_settings(
            "",
            |settings| settings.theme = Some("summerfruit-light".to_string()),
            r#"
                {
                    "theme": "summerfruit-light"
                }
            "#
            .unindent(),
        );
    }

    #[test]
    fn test_write_theme_into_single_line_settings_without_theme() {
        assert_new_settings(
            r#"{ "a": "", "ok": true }"#,
            |settings| settings.theme = Some("summerfruit-light".to_string()),
            r#"{ "theme": "summerfruit-light", "a": "", "ok": true }"#,
        );
    }

    #[test]
    fn test_write_theme_pre_object_whitespace() {
        assert_new_settings(
            r#"          { "a": "", "ok": true }"#,
            |settings| settings.theme = Some("summerfruit-light".to_string()),
            r#"          { "theme": "summerfruit-light", "a": "", "ok": true }"#.unindent(),
        );
    }

    #[test]
    fn test_write_theme_into_multi_line_settings_without_theme() {
        assert_new_settings(
            r#"
                {
                    "a": "b"
                }
            "#
            .unindent(),
            |settings| settings.theme = Some("summerfruit-light".to_string()),
            r#"
                {
                    "theme": "summerfruit-light",
                    "a": "b"
                }
            "#
            .unindent(),
        );
    }
}
