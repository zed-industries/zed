use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Setting;

#[derive(Deserialize)]
pub struct EditorSettings {
    pub cursor_blink: bool,
    pub hover_popover_enabled: bool,
    pub show_completions_on_input: bool,
    pub use_on_type_format: bool,
    pub scrollbar: Scrollbar,
    pub inlay_hints: InlayHints,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Scrollbar {
    pub show: ShowScrollbar,
    pub git_diff: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct InlayHints {
    pub enabled: bool,
    pub show_type_hints: bool,
    pub show_parameter_hints: bool,
    pub show_other_hints: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ShowScrollbar {
    Auto,
    System,
    Always,
    Never,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct EditorSettingsContent {
    pub cursor_blink: Option<bool>,
    pub hover_popover_enabled: Option<bool>,
    pub show_completions_on_input: Option<bool>,
    pub use_on_type_format: Option<bool>,
    pub scrollbar: Option<ScrollbarContent>,
    pub inlay_hints: Option<InlayHintsContent>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarContent {
    pub show: Option<ShowScrollbar>,
    pub git_diff: Option<bool>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct InlayHintsContent {
    pub enabled: Option<bool>,
    pub show_type_hints: Option<bool>,
    pub show_parameter_hints: Option<bool>,
    pub show_other_hints: Option<bool>,
}

impl Setting for EditorSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = EditorSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
