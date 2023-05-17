mod font_size;
mod keymap_file;
mod settings_file;
mod settings_store;

use anyhow::{bail, Result};
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
use std::{borrow::Cow, str, sync::Arc};
use theme::{Theme, ThemeRegistry};
use util::ResultExt as _;

pub use font_size::{adjust_font_size_delta, font_size_for_setting};
pub use keymap_file::{keymap_file_json_schema, KeymapFileContent};
pub use settings_file::*;
pub use settings_store::{Setting, SettingsJsonSchemaParams, SettingsStore};

pub const DEFAULT_SETTINGS_ASSET_PATH: &str = "settings/default.json";
pub const INITIAL_USER_SETTINGS_ASSET_PATH: &str = "settings/initial_user_settings.json";

#[derive(Clone)]
pub struct Settings {
    pub buffer_font_family_name: String,
    pub buffer_font_features: fonts::Features,
    pub buffer_font_family: FamilyId,
    pub buffer_font_size: f32,
    pub active_pane_magnification: f32,
    pub confirm_quit: bool,
    pub show_call_status_icon: bool,
    pub autosave: Autosave,
    pub default_dock_anchor: DockAnchor,
    pub git: GitSettings,
    pub git_overrides: GitSettings,
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
    ) -> Result<Self> {
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
            confirm_quit: defaults.confirm_quit.unwrap(),
            show_call_status_icon: defaults.show_call_status_icon.unwrap(),
            autosave: defaults.autosave.unwrap(),
            default_dock_anchor: defaults.default_dock_anchor.unwrap(),
            git: defaults.git.unwrap(),
            git_overrides: Default::default(),
            theme: themes.get(defaults.theme.as_ref().unwrap()).unwrap(),
            base_keymap: Default::default(),
        };

        for value in user_values.into_iter().copied().cloned() {
            this.set_user_settings(value, themes.as_ref(), cx.font_cache());
        }

        Ok(this)
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Autosave {
    Off,
    AfterDelay { milliseconds: u64 },
    OnFocusChange,
    OnWindowChange,
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
    #[serde(default)]
    pub git: Option<GitSettings>,
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(default)]
    pub base_keymap: Option<BaseKeymap>,
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
            confirm_quit: defaults.confirm_quit.unwrap(),
            show_call_status_icon: defaults.show_call_status_icon.unwrap(),
            autosave: defaults.autosave.unwrap(),
            default_dock_anchor: defaults.default_dock_anchor.unwrap(),
            git: defaults.git.unwrap(),
            git_overrides: Default::default(),
            theme: themes.get(&defaults.theme.unwrap()).unwrap(),
            base_keymap: Default::default(),
        }
    }

    // Fill out the overrride and etc. settings from the user's settings.json
    fn set_user_settings(
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
        merge(&mut self.confirm_quit, data.confirm_quit);
        merge(&mut self.autosave, data.autosave);
        merge(&mut self.default_dock_anchor, data.default_dock_anchor);
        merge(&mut self.base_keymap, data.base_keymap);

        self.git_overrides = data.git.unwrap_or_default();
    }

    pub fn git_gutter(&self) -> GitGutter {
        self.git_overrides.git_gutter.unwrap_or_else(|| {
            self.git
                .git_gutter
                .expect("git_gutter should be some by setting setup")
        })
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
            confirm_quit: false,
            show_call_status_icon: true,
            autosave: Autosave::Off,
            default_dock_anchor: DockAnchor::Bottom,
            git: Default::default(),
            git_overrides: Default::default(),
            theme: gpui::fonts::with_font_cache(cx.font_cache().clone(), Default::default),
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

fn merge<T: Copy>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}
