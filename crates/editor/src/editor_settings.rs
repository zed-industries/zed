use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;

#[derive(Deserialize)]
pub struct EditorSettings {
    pub cursor_blink: bool,
    pub hover_popover_enabled: bool,
    pub show_completions_on_input: bool,
    pub show_completion_documentation: bool,
    pub use_on_type_format: bool,
    pub scrollbar: Scrollbar,
    pub relative_line_numbers: bool,
    pub seed_search_query_from_cursor: SeedQuerySetting,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SeedQuerySetting {
    Always,
    Selection,
    Never,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Scrollbar {
    pub show: ShowScrollbar,
    pub git_diff: bool,
    pub selections: bool,
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
    pub show_completion_documentation: Option<bool>,
    pub use_on_type_format: Option<bool>,
    pub scrollbar: Option<ScrollbarContent>,
    pub relative_line_numbers: Option<bool>,
    pub seed_search_query_from_cursor: Option<SeedQuerySetting>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarContent {
    pub show: Option<ShowScrollbar>,
    pub git_diff: Option<bool>,
    pub selections: Option<bool>,
}

impl Settings for EditorSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = EditorSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
