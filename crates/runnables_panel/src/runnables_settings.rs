use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RunnablesDockPosition {
    Left,
    #[default]
    Right,
}

#[derive(Serialize, Deserialize)]
pub struct RunnablesSettings {
    pub(crate) dock: RunnablesDockPosition,
    pub(crate) default_width: f32,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct RunnablesSettingsContent {
    /// The position of runnables panel
    ///
    /// Default: right
    pub dock: Option<RunnablesDockPosition>,
    /// Customise default width (in pixels) taken by runnables panel
    ///
    /// Default: 240
    pub default_width: Option<f32>,
}

impl Settings for RunnablesSettings {
    const KEY: Option<&'static str> = Some("runnables");

    type FileContent = RunnablesSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
