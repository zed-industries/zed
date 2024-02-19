use gpui::Pixels;
pub use open_ai::Model as OpenAiModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
pub enum ZedDotDevModel {
    #[serde(rename = "gpt-3.5-turbo-0613")]
    GptThreePointFiveTurbo,
    #[serde(rename = "gpt-4-0613")]
    GptFour,
    #[serde(rename = "gpt-4-1106-preview")]
    #[default]
    GptFourTurbo,
}

impl ZedDotDevModel {
    pub fn id(&self) -> &'static str {
        match self {
            Self::GptThreePointFiveTurbo => "gpt-3.5-turbo-0613",
            Self::GptFour => "gpt-4-0613",
            Self::GptFourTurbo => "gpt-4-1106-preview",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::GptThreePointFiveTurbo => "gpt-3.5-turbo",
            Self::GptFour => "gpt-4",
            Self::GptFourTurbo => "gpt-4-turbo",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            Self::GptThreePointFiveTurbo => Self::GptFour,
            Self::GptFour => Self::GptFourTurbo,
            Self::GptFourTurbo => Self::GptThreePointFiveTurbo,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssistantDockPosition {
    Left,
    #[default]
    Right,
    Bottom,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum AssistantProvider {
    #[serde(rename = "zed.dev")]
    ZedDotDev {
        #[serde(default)]
        default_model: ZedDotDevModel,
    },
    #[serde(rename = "openai")]
    OpenAi {
        #[serde(default)]
        default_model: OpenAiModel,
        #[serde(default = "open_ai_url")]
        api_url: String,
    },
}

impl Default for AssistantProvider {
    fn default() -> Self {
        Self::ZedDotDev {
            default_model: ZedDotDevModel::default(),
        }
    }
}

fn open_ai_url() -> String {
    "https://api.openai.com/v1".into()
}

#[derive(Default, Deserialize, Serialize)]
pub struct AssistantSettings {
    pub button: bool,
    pub dock: AssistantDockPosition,
    pub default_width: Pixels,
    pub default_height: Pixels,
    pub provider: AssistantProvider,
}

/// Assistant panel settings
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct AssistantSettingsContent {
    /// Whether to show the assistant panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Where to dock the assistant.
    ///
    /// Default: right
    pub dock: Option<AssistantDockPosition>,
    /// Default width in pixels when the assistant is docked to the left or right.
    ///
    /// Default: 640
    pub default_width: Option<f32>,
    /// Default height in pixels when the assistant is docked to the bottom.
    ///
    /// Default: 320
    pub default_height: Option<f32>,
    /// The provider of the assistant service.
    ///
    /// This can either be the internal `zed.dev` service or an external `openai` service,
    /// each with their respective default models and configurations.
    pub provider: Option<AssistantProvider>,
}

impl Settings for AssistantSettings {
    const KEY: Option<&'static str> = Some("assistant");

    type FileContent = AssistantSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        let mut settings = AssistantSettings::default();

        for value in [default_value].iter().chain(user_values) {
            merge(&mut settings.button, value.button);
            merge(&mut settings.dock, value.dock);
            merge(
                &mut settings.default_width,
                value.default_width.map(Into::into),
            );
            merge(
                &mut settings.default_height,
                value.default_height.map(Into::into),
            );
            if let Some(provider) = value.provider.clone() {
                match (&mut settings.provider, provider) {
                    (
                        AssistantProvider::ZedDotDev { default_model },
                        AssistantProvider::ZedDotDev {
                            default_model: default_model_override,
                        },
                    ) => {
                        *default_model = default_model_override;
                    }
                    (
                        AssistantProvider::OpenAi {
                            default_model,
                            api_url,
                        },
                        AssistantProvider::OpenAi {
                            default_model: default_model_override,
                            api_url: api_url_override,
                        },
                    ) => {
                        *default_model = default_model_override;
                        *api_url = api_url_override;
                    }
                    (merged, provider_override) => {
                        *merged = provider_override;
                    }
                }
            }
        }

        Ok(settings)
    }
}

fn merge<T: Copy>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}
