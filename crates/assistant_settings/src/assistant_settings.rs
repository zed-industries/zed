mod agent_profile;

use std::sync::Arc;

use ::open_ai::Model as OpenAiModel;
use anthropic::Model as AnthropicModel;
use anyhow::{Result, bail};
use collections::IndexMap;
use deepseek::Model as DeepseekModel;
use gpui::{App, Pixels, SharedString};
use language_model::{CloudModel, LanguageModel};
use lmstudio::Model as LmStudioModel;
use ollama::Model as OllamaModel;
use schemars::{JsonSchema, schema::Schema};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

pub use crate::agent_profile::*;

pub fn init(cx: &mut App) {
    AssistantSettings::register(cx);
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssistantDockPosition {
    Left,
    #[default]
    Right,
    Bottom,
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NotifyWhenAgentWaiting {
    #[default]
    PrimaryScreen,
    AllScreens,
    Never,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum AssistantProviderContentV1 {
    #[serde(rename = "zed.dev")]
    ZedDotDev { default_model: Option<CloudModel> },
    #[serde(rename = "openai")]
    OpenAi {
        default_model: Option<OpenAiModel>,
        api_url: Option<String>,
        available_models: Option<Vec<OpenAiModel>>,
    },
    #[serde(rename = "anthropic")]
    Anthropic {
        default_model: Option<AnthropicModel>,
        api_url: Option<String>,
    },
    #[serde(rename = "ollama")]
    Ollama {
        default_model: Option<OllamaModel>,
        api_url: Option<String>,
    },
    #[serde(rename = "lmstudio")]
    LmStudio {
        default_model: Option<LmStudioModel>,
        api_url: Option<String>,
    },
    #[serde(rename = "deepseek")]
    DeepSeek {
        default_model: Option<DeepseekModel>,
        api_url: Option<String>,
    },
}

#[derive(Default, Clone, Debug)]
pub struct AssistantSettings {
    pub enabled: bool,
    pub button: bool,
    pub dock: AssistantDockPosition,
    pub default_width: Pixels,
    pub default_height: Pixels,
    pub default_model: LanguageModelSelection,
    pub inline_assistant_model: Option<LanguageModelSelection>,
    pub commit_message_model: Option<LanguageModelSelection>,
    pub thread_summary_model: Option<LanguageModelSelection>,
    pub inline_alternatives: Vec<LanguageModelSelection>,
    pub using_outdated_settings_version: bool,
    pub enable_experimental_live_diffs: bool,
    pub default_profile: AgentProfileId,
    pub profiles: IndexMap<AgentProfileId, AgentProfile>,
    pub always_allow_tool_actions: bool,
    pub notify_when_agent_waiting: NotifyWhenAgentWaiting,
    pub stream_edits: bool,
    pub single_file_review: bool,
    pub model_parameters: Vec<LanguageModelParameters>,
    pub preferred_completion_mode: CompletionMode,
}

impl AssistantSettings {
    pub fn temperature_for_model(model: &Arc<dyn LanguageModel>, cx: &App) -> Option<f32> {
        let settings = Self::get_global(cx);
        settings
            .model_parameters
            .iter()
            .rfind(|setting| setting.matches(model))
            .and_then(|m| m.temperature)
    }

    pub fn stream_edits(&self, _cx: &App) -> bool {
        // TODO: Remove the `stream_edits` setting.
        true
    }

    pub fn are_live_diffs_enabled(&self, _cx: &App) -> bool {
        false
    }

    pub fn set_inline_assistant_model(&mut self, provider: String, model: String) {
        self.inline_assistant_model = Some(LanguageModelSelection {
            provider: provider.into(),
            model,
        });
    }

    pub fn set_commit_message_model(&mut self, provider: String, model: String) {
        self.commit_message_model = Some(LanguageModelSelection {
            provider: provider.into(),
            model,
        });
    }

    pub fn set_thread_summary_model(&mut self, provider: String, model: String) {
        self.thread_summary_model = Some(LanguageModelSelection {
            provider: provider.into(),
            model,
        });
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct LanguageModelParameters {
    pub provider: Option<LanguageModelProviderSetting>,
    pub model: Option<SharedString>,
    pub temperature: Option<f32>,
}

impl LanguageModelParameters {
    pub fn matches(&self, model: &Arc<dyn LanguageModel>) -> bool {
        if let Some(provider) = &self.provider {
            if provider.0 != model.provider_id().0 {
                return false;
            }
        }
        if let Some(setting_model) = &self.model {
            if *setting_model != model.id().0 {
                return false;
            }
        }
        true
    }
}

/// Assistant panel settings
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct AssistantSettingsContent {
    #[serde(flatten)]
    pub inner: Option<AssistantSettingsContentInner>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum AssistantSettingsContentInner {
    Versioned(Box<VersionedAssistantSettingsContent>),
    Legacy(LegacyAssistantSettingsContent),
}

impl AssistantSettingsContentInner {
    fn for_v2(content: AssistantSettingsContentV2) -> Self {
        AssistantSettingsContentInner::Versioned(Box::new(VersionedAssistantSettingsContent::V2(
            content,
        )))
    }
}

impl JsonSchema for AssistantSettingsContent {
    fn schema_name() -> String {
        VersionedAssistantSettingsContent::schema_name()
    }

    fn json_schema(r#gen: &mut schemars::r#gen::SchemaGenerator) -> Schema {
        VersionedAssistantSettingsContent::json_schema(r#gen)
    }

    fn is_referenceable() -> bool {
        VersionedAssistantSettingsContent::is_referenceable()
    }
}

impl AssistantSettingsContent {
    pub fn is_version_outdated(&self) -> bool {
        match &self.inner {
            Some(AssistantSettingsContentInner::Versioned(settings)) => match **settings {
                VersionedAssistantSettingsContent::V1(_) => true,
                VersionedAssistantSettingsContent::V2(_) => false,
            },
            Some(AssistantSettingsContentInner::Legacy(_)) => true,
            None => false,
        }
    }

    fn upgrade(&self) -> AssistantSettingsContentV2 {
        match &self.inner {
            Some(AssistantSettingsContentInner::Versioned(settings)) => match **settings {
                VersionedAssistantSettingsContent::V1(ref settings) => AssistantSettingsContentV2 {
                    enabled: settings.enabled,
                    button: settings.button,
                    dock: settings.dock,
                    default_width: settings.default_width,
                    default_height: settings.default_width,
                    default_model: settings
                        .provider
                        .clone()
                        .and_then(|provider| match provider {
                            AssistantProviderContentV1::ZedDotDev { default_model } => {
                                default_model.map(|model| LanguageModelSelection {
                                    provider: "zed.dev".into(),
                                    model: model.id().to_string(),
                                })
                            }
                            AssistantProviderContentV1::OpenAi { default_model, .. } => {
                                default_model.map(|model| LanguageModelSelection {
                                    provider: "openai".into(),
                                    model: model.id().to_string(),
                                })
                            }
                            AssistantProviderContentV1::Anthropic { default_model, .. } => {
                                default_model.map(|model| LanguageModelSelection {
                                    provider: "anthropic".into(),
                                    model: model.id().to_string(),
                                })
                            }
                            AssistantProviderContentV1::Ollama { default_model, .. } => {
                                default_model.map(|model| LanguageModelSelection {
                                    provider: "ollama".into(),
                                    model: model.id().to_string(),
                                })
                            }
                            AssistantProviderContentV1::LmStudio { default_model, .. } => {
                                default_model.map(|model| LanguageModelSelection {
                                    provider: "lmstudio".into(),
                                    model: model.id().to_string(),
                                })
                            }
                            AssistantProviderContentV1::DeepSeek { default_model, .. } => {
                                default_model.map(|model| LanguageModelSelection {
                                    provider: "deepseek".into(),
                                    model: model.id().to_string(),
                                })
                            }
                        }),
                    inline_assistant_model: None,
                    commit_message_model: None,
                    thread_summary_model: None,
                    inline_alternatives: None,
                    enable_experimental_live_diffs: None,
                    default_profile: None,
                    profiles: None,
                    always_allow_tool_actions: None,
                    notify_when_agent_waiting: None,
                    stream_edits: None,
                    single_file_review: None,
                    model_parameters: Vec::new(),
                    preferred_completion_mode: None,
                },
                VersionedAssistantSettingsContent::V2(ref settings) => settings.clone(),
            },
            Some(AssistantSettingsContentInner::Legacy(settings)) => AssistantSettingsContentV2 {
                enabled: None,
                button: settings.button,
                dock: settings.dock,
                default_width: settings.default_width,
                default_height: settings.default_height,
                default_model: Some(LanguageModelSelection {
                    provider: "openai".into(),
                    model: settings
                        .default_open_ai_model
                        .clone()
                        .unwrap_or_default()
                        .id()
                        .to_string(),
                }),
                inline_assistant_model: None,
                commit_message_model: None,
                thread_summary_model: None,
                inline_alternatives: None,
                enable_experimental_live_diffs: None,
                default_profile: None,
                profiles: None,
                always_allow_tool_actions: None,
                notify_when_agent_waiting: None,
                stream_edits: None,
                single_file_review: None,
                model_parameters: Vec::new(),
                preferred_completion_mode: None,
            },
            None => AssistantSettingsContentV2::default(),
        }
    }

    pub fn set_dock(&mut self, dock: AssistantDockPosition) {
        match &mut self.inner {
            Some(AssistantSettingsContentInner::Versioned(settings)) => match **settings {
                VersionedAssistantSettingsContent::V1(ref mut settings) => {
                    settings.dock = Some(dock);
                }
                VersionedAssistantSettingsContent::V2(ref mut settings) => {
                    settings.dock = Some(dock);
                }
            },
            Some(AssistantSettingsContentInner::Legacy(settings)) => {
                settings.dock = Some(dock);
            }
            None => {
                self.inner = Some(AssistantSettingsContentInner::for_v2(
                    AssistantSettingsContentV2 {
                        dock: Some(dock),
                        ..Default::default()
                    },
                ))
            }
        }
    }

    pub fn set_model(&mut self, language_model: Arc<dyn LanguageModel>) {
        let model = language_model.id().0.to_string();
        let provider = language_model.provider_id().0.to_string();

        match &mut self.inner {
            Some(AssistantSettingsContentInner::Versioned(settings)) => match **settings {
                VersionedAssistantSettingsContent::V1(ref mut settings) => {
                    match provider.as_ref() {
                        "zed.dev" => {
                            log::warn!("attempted to set zed.dev model on outdated settings");
                        }
                        "anthropic" => {
                            let api_url = match &settings.provider {
                                Some(AssistantProviderContentV1::Anthropic { api_url, .. }) => {
                                    api_url.clone()
                                }
                                _ => None,
                            };
                            settings.provider = Some(AssistantProviderContentV1::Anthropic {
                                default_model: AnthropicModel::from_id(&model).ok(),
                                api_url,
                            });
                        }
                        "ollama" => {
                            let api_url = match &settings.provider {
                                Some(AssistantProviderContentV1::Ollama { api_url, .. }) => {
                                    api_url.clone()
                                }
                                _ => None,
                            };
                            settings.provider = Some(AssistantProviderContentV1::Ollama {
                                default_model: Some(ollama::Model::new(
                                    &model,
                                    None,
                                    None,
                                    language_model.supports_tools(),
                                )),
                                api_url,
                            });
                        }
                        "lmstudio" => {
                            let api_url = match &settings.provider {
                                Some(AssistantProviderContentV1::LmStudio { api_url, .. }) => {
                                    api_url.clone()
                                }
                                _ => None,
                            };
                            settings.provider = Some(AssistantProviderContentV1::LmStudio {
                                default_model: Some(lmstudio::Model::new(&model, None, None)),
                                api_url,
                            });
                        }
                        "openai" => {
                            let (api_url, available_models) = match &settings.provider {
                                Some(AssistantProviderContentV1::OpenAi {
                                    api_url,
                                    available_models,
                                    ..
                                }) => (api_url.clone(), available_models.clone()),
                                _ => (None, None),
                            };
                            settings.provider = Some(AssistantProviderContentV1::OpenAi {
                                default_model: OpenAiModel::from_id(&model).ok(),
                                api_url,
                                available_models,
                            });
                        }
                        "deepseek" => {
                            let api_url = match &settings.provider {
                                Some(AssistantProviderContentV1::DeepSeek { api_url, .. }) => {
                                    api_url.clone()
                                }
                                _ => None,
                            };
                            settings.provider = Some(AssistantProviderContentV1::DeepSeek {
                                default_model: DeepseekModel::from_id(&model).ok(),
                                api_url,
                            });
                        }
                        _ => {}
                    }
                }
                VersionedAssistantSettingsContent::V2(ref mut settings) => {
                    settings.default_model = Some(LanguageModelSelection {
                        provider: provider.into(),
                        model,
                    });
                }
            },
            Some(AssistantSettingsContentInner::Legacy(settings)) => {
                if let Ok(model) = OpenAiModel::from_id(&language_model.id().0) {
                    settings.default_open_ai_model = Some(model);
                }
            }
            None => {
                self.inner = Some(AssistantSettingsContentInner::for_v2(
                    AssistantSettingsContentV2 {
                        default_model: Some(LanguageModelSelection {
                            provider: provider.into(),
                            model,
                        }),
                        ..Default::default()
                    },
                ));
            }
        }
    }

    pub fn set_inline_assistant_model(&mut self, provider: String, model: String) {
        self.v2_setting(|setting| {
            setting.inline_assistant_model = Some(LanguageModelSelection {
                provider: provider.into(),
                model,
            });
            Ok(())
        })
        .ok();
    }

    pub fn set_commit_message_model(&mut self, provider: String, model: String) {
        self.v2_setting(|setting| {
            setting.commit_message_model = Some(LanguageModelSelection {
                provider: provider.into(),
                model,
            });
            Ok(())
        })
        .ok();
    }

    pub fn v2_setting(
        &mut self,
        f: impl FnOnce(&mut AssistantSettingsContentV2) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        match self.inner.get_or_insert_with(|| {
            AssistantSettingsContentInner::for_v2(AssistantSettingsContentV2 {
                ..Default::default()
            })
        }) {
            AssistantSettingsContentInner::Versioned(boxed) => {
                if let VersionedAssistantSettingsContent::V2(ref mut settings) = **boxed {
                    f(settings)
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }

    pub fn set_thread_summary_model(&mut self, provider: String, model: String) {
        self.v2_setting(|setting| {
            setting.thread_summary_model = Some(LanguageModelSelection {
                provider: provider.into(),
                model,
            });
            Ok(())
        })
        .ok();
    }

    pub fn set_always_allow_tool_actions(&mut self, allow: bool) {
        self.v2_setting(|setting| {
            setting.always_allow_tool_actions = Some(allow);
            Ok(())
        })
        .ok();
    }

    pub fn set_single_file_review(&mut self, allow: bool) {
        self.v2_setting(|setting| {
            setting.single_file_review = Some(allow);
            Ok(())
        })
        .ok();
    }

    pub fn set_profile(&mut self, profile_id: AgentProfileId) {
        self.v2_setting(|setting| {
            setting.default_profile = Some(profile_id);
            Ok(())
        })
        .ok();
    }

    pub fn create_profile(
        &mut self,
        profile_id: AgentProfileId,
        profile: AgentProfile,
    ) -> Result<()> {
        self.v2_setting(|settings| {
            let profiles = settings.profiles.get_or_insert_default();
            if profiles.contains_key(&profile_id) {
                bail!("profile with ID '{profile_id}' already exists");
            }

            profiles.insert(
                profile_id,
                AgentProfileContent {
                    name: profile.name.into(),
                    tools: profile.tools,
                    enable_all_context_servers: Some(profile.enable_all_context_servers),
                    context_servers: profile
                        .context_servers
                        .into_iter()
                        .map(|(server_id, preset)| {
                            (
                                server_id,
                                ContextServerPresetContent {
                                    tools: preset.tools,
                                },
                            )
                        })
                        .collect(),
                },
            );

            Ok(())
        })
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(tag = "version")]
pub enum VersionedAssistantSettingsContent {
    #[serde(rename = "1")]
    V1(AssistantSettingsContentV1),
    #[serde(rename = "2")]
    V2(AssistantSettingsContentV2),
}

impl Default for VersionedAssistantSettingsContent {
    fn default() -> Self {
        Self::V2(AssistantSettingsContentV2 {
            enabled: None,
            button: None,
            dock: None,
            default_width: None,
            default_height: None,
            default_model: None,
            inline_assistant_model: None,
            commit_message_model: None,
            thread_summary_model: None,
            inline_alternatives: None,
            enable_experimental_live_diffs: None,
            default_profile: None,
            profiles: None,
            always_allow_tool_actions: None,
            notify_when_agent_waiting: None,
            stream_edits: None,
            single_file_review: None,
            model_parameters: Vec::new(),
            preferred_completion_mode: None,
        })
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug, Default)]
pub struct AssistantSettingsContentV2 {
    /// Whether the Assistant is enabled.
    ///
    /// Default: true
    enabled: Option<bool>,
    /// Whether to show the assistant panel button in the status bar.
    ///
    /// Default: true
    button: Option<bool>,
    /// Where to dock the assistant.
    ///
    /// Default: right
    dock: Option<AssistantDockPosition>,
    /// Default width in pixels when the assistant is docked to the left or right.
    ///
    /// Default: 640
    default_width: Option<f32>,
    /// Default height in pixels when the assistant is docked to the bottom.
    ///
    /// Default: 320
    default_height: Option<f32>,
    /// The default model to use when creating new chats and for other features when a specific model is not specified.
    default_model: Option<LanguageModelSelection>,
    /// Model to use for the inline assistant. Defaults to default_model when not specified.
    inline_assistant_model: Option<LanguageModelSelection>,
    /// Model to use for generating git commit messages. Defaults to default_model when not specified.
    commit_message_model: Option<LanguageModelSelection>,
    /// Model to use for generating thread summaries. Defaults to default_model when not specified.
    thread_summary_model: Option<LanguageModelSelection>,
    /// Additional models with which to generate alternatives when performing inline assists.
    inline_alternatives: Option<Vec<LanguageModelSelection>>,
    /// Enable experimental live diffs in the assistant panel.
    ///
    /// Default: false
    enable_experimental_live_diffs: Option<bool>,
    /// The default profile to use in the Agent.
    ///
    /// Default: write
    default_profile: Option<AgentProfileId>,
    /// The available agent profiles.
    pub profiles: Option<IndexMap<AgentProfileId, AgentProfileContent>>,
    /// Whenever a tool action would normally wait for your confirmation
    /// that you allow it, always choose to allow it.
    ///
    /// Default: false
    always_allow_tool_actions: Option<bool>,
    /// Where to show a popup notification when the agent is waiting for user input.
    ///
    /// Default: "primary_screen"
    notify_when_agent_waiting: Option<NotifyWhenAgentWaiting>,
    /// Whether to stream edits from the agent as they are received.
    ///
    /// Default: false
    stream_edits: Option<bool>,
    /// Whether to display agent edits in single-file editors in addition to the review multibuffer pane.
    ///
    /// Default: true
    single_file_review: Option<bool>,
    /// Additional parameters for language model requests. When making a request
    /// to a model, parameters will be taken from the last entry in this list
    /// that matches the model's provider and name. In each entry, both provider
    /// and model are optional, so that you can specify parameters for either
    /// one.
    ///
    /// Default: []
    #[serde(default)]
    model_parameters: Vec<LanguageModelParameters>,

    /// What completion mode to enable for new threads
    ///
    /// Default: normal
    preferred_completion_mode: Option<CompletionMode>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum CompletionMode {
    #[default]
    Normal,
    Max,
}

impl From<CompletionMode> for zed_llm_client::CompletionMode {
    fn from(value: CompletionMode) -> Self {
        match value {
            CompletionMode::Normal => zed_llm_client::CompletionMode::Normal,
            CompletionMode::Max => zed_llm_client::CompletionMode::Max,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct LanguageModelSelection {
    pub provider: LanguageModelProviderSetting,
    pub model: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LanguageModelProviderSetting(pub String);

impl JsonSchema for LanguageModelProviderSetting {
    fn schema_name() -> String {
        "LanguageModelProviderSetting".into()
    }

    fn json_schema(_: &mut schemars::r#gen::SchemaGenerator) -> Schema {
        schemars::schema::SchemaObject {
            enum_values: Some(vec![
                "anthropic".into(),
                "bedrock".into(),
                "google".into(),
                "lmstudio".into(),
                "ollama".into(),
                "openai".into(),
                "zed.dev".into(),
                "copilot_chat".into(),
                "deepseek".into(),
            ]),
            ..Default::default()
        }
        .into()
    }
}

impl From<String> for LanguageModelProviderSetting {
    fn from(provider: String) -> Self {
        Self(provider)
    }
}

impl From<&str> for LanguageModelProviderSetting {
    fn from(provider: &str) -> Self {
        Self(provider.to_string())
    }
}

impl Default for LanguageModelSelection {
    fn default() -> Self {
        Self {
            provider: LanguageModelProviderSetting("openai".to_string()),
            model: "gpt-4".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentProfileContent {
    pub name: Arc<str>,
    #[serde(default)]
    pub tools: IndexMap<Arc<str>, bool>,
    /// Whether all context servers are enabled by default.
    pub enable_all_context_servers: Option<bool>,
    #[serde(default)]
    pub context_servers: IndexMap<Arc<str>, ContextServerPresetContent>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ContextServerPresetContent {
    pub tools: IndexMap<Arc<str>, bool>,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
pub struct AssistantSettingsContentV1 {
    /// Whether the Assistant is enabled.
    ///
    /// Default: true
    enabled: Option<bool>,
    /// Whether to show the assistant panel button in the status bar.
    ///
    /// Default: true
    button: Option<bool>,
    /// Where to dock the assistant.
    ///
    /// Default: right
    dock: Option<AssistantDockPosition>,
    /// Default width in pixels when the assistant is docked to the left or right.
    ///
    /// Default: 640
    default_width: Option<f32>,
    /// Default height in pixels when the assistant is docked to the bottom.
    ///
    /// Default: 320
    default_height: Option<f32>,
    /// The provider of the assistant service.
    ///
    /// This can be "openai", "anthropic", "ollama", "lmstudio", "deepseek", "zed.dev"
    /// each with their respective default models and configurations.
    provider: Option<AssistantProviderContentV1>,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
pub struct LegacyAssistantSettingsContent {
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
    /// The default OpenAI model to use when creating new chats.
    ///
    /// Default: gpt-4-1106-preview
    pub default_open_ai_model: Option<OpenAiModel>,
    /// OpenAI API base URL to use when creating new chats.
    ///
    /// Default: <https://api.openai.com/v1>
    pub openai_api_url: Option<String>,
}

impl Settings for AssistantSettings {
    const KEY: Option<&'static str> = Some("agent");

    const FALLBACK_KEY: Option<&'static str> = Some("assistant");

    const PRESERVED_KEYS: Option<&'static [&'static str]> = Some(&["version"]);

    type FileContent = AssistantSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::App,
    ) -> anyhow::Result<Self> {
        let mut settings = AssistantSettings::default();

        for value in sources.defaults_and_customizations() {
            if value.is_version_outdated() {
                settings.using_outdated_settings_version = true;
            }

            let value = value.upgrade();
            merge(&mut settings.enabled, value.enabled);
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
            merge(&mut settings.default_model, value.default_model);
            settings.inline_assistant_model = value
                .inline_assistant_model
                .or(settings.inline_assistant_model.take());
            settings.commit_message_model = value
                .commit_message_model
                .or(settings.commit_message_model.take());
            settings.thread_summary_model = value
                .thread_summary_model
                .or(settings.thread_summary_model.take());
            merge(&mut settings.inline_alternatives, value.inline_alternatives);
            merge(
                &mut settings.enable_experimental_live_diffs,
                value.enable_experimental_live_diffs,
            );
            merge(
                &mut settings.always_allow_tool_actions,
                value.always_allow_tool_actions,
            );
            merge(
                &mut settings.notify_when_agent_waiting,
                value.notify_when_agent_waiting,
            );
            merge(&mut settings.stream_edits, value.stream_edits);
            merge(&mut settings.single_file_review, value.single_file_review);
            merge(&mut settings.default_profile, value.default_profile);
            merge(
                &mut settings.preferred_completion_mode,
                value.preferred_completion_mode,
            );

            settings
                .model_parameters
                .extend_from_slice(&value.model_parameters);

            if let Some(profiles) = value.profiles {
                settings
                    .profiles
                    .extend(profiles.into_iter().map(|(id, profile)| {
                        (
                            id,
                            AgentProfile {
                                name: profile.name.into(),
                                tools: profile.tools,
                                enable_all_context_servers: profile
                                    .enable_all_context_servers
                                    .unwrap_or_default(),
                                context_servers: profile
                                    .context_servers
                                    .into_iter()
                                    .map(|(context_server_id, preset)| {
                                        (
                                            context_server_id,
                                            ContextServerPreset {
                                                tools: preset.tools.clone(),
                                            },
                                        )
                                    })
                                    .collect(),
                            },
                        )
                    }));
            }
        }

        Ok(settings)
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut Self::FileContent) {
        if let Some(b) = vscode
            .read_value("chat.agent.enabled")
            .and_then(|b| b.as_bool())
        {
            match &mut current.inner {
                Some(AssistantSettingsContentInner::Versioned(versioned)) => {
                    match versioned.as_mut() {
                        VersionedAssistantSettingsContent::V1(setting) => {
                            setting.enabled = Some(b);
                            setting.button = Some(b);
                        }

                        VersionedAssistantSettingsContent::V2(setting) => {
                            setting.enabled = Some(b);
                            setting.button = Some(b);
                        }
                    }
                }
                Some(AssistantSettingsContentInner::Legacy(setting)) => setting.button = Some(b),
                None => {
                    current.inner = Some(AssistantSettingsContentInner::for_v2(
                        AssistantSettingsContentV2 {
                            enabled: Some(b),
                            button: Some(b),
                            ..Default::default()
                        },
                    ));
                }
            }
        }
    }
}

fn merge<T>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}

#[cfg(test)]
mod tests {
    use fs::Fs;
    use gpui::{ReadGlobal, TestAppContext};
    use settings::SettingsStore;

    use super::*;

    #[gpui::test]
    async fn test_deserialize_assistant_settings_with_version(cx: &mut TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor().clone());
        fs.create_dir(paths::settings_file().parent().unwrap())
            .await
            .unwrap();

        cx.update(|cx| {
            let test_settings = settings::SettingsStore::test(cx);
            cx.set_global(test_settings);
            AssistantSettings::register(cx);
        });

        cx.update(|cx| {
            assert!(!AssistantSettings::get_global(cx).using_outdated_settings_version);
            assert_eq!(
                AssistantSettings::get_global(cx).default_model,
                LanguageModelSelection {
                    provider: "zed.dev".into(),
                    model: "claude-3-7-sonnet-latest".into(),
                }
            );
        });

        cx.update(|cx| {
            settings::SettingsStore::global(cx).update_settings_file::<AssistantSettings>(
                fs.clone(),
                |settings, _| {
                    *settings = AssistantSettingsContent {
                        inner: Some(AssistantSettingsContentInner::for_v2(
                            AssistantSettingsContentV2 {
                                default_model: Some(LanguageModelSelection {
                                    provider: "test-provider".into(),
                                    model: "gpt-99".into(),
                                }),
                                inline_assistant_model: None,
                                commit_message_model: None,
                                thread_summary_model: None,
                                inline_alternatives: None,
                                enabled: None,
                                button: None,
                                dock: None,
                                default_width: None,
                                default_height: None,
                                enable_experimental_live_diffs: None,
                                default_profile: None,
                                profiles: None,
                                always_allow_tool_actions: None,
                                notify_when_agent_waiting: None,
                                stream_edits: None,
                                single_file_review: None,
                                model_parameters: Vec::new(),
                                preferred_completion_mode: None,
                            },
                        )),
                    }
                },
            );
        });

        cx.run_until_parked();

        let raw_settings_value = fs.load(paths::settings_file()).await.unwrap();
        assert!(raw_settings_value.contains(r#""version": "2""#));

        #[derive(Debug, Deserialize)]
        struct AssistantSettingsTest {
            agent: AssistantSettingsContent,
        }

        let assistant_settings: AssistantSettingsTest =
            serde_json_lenient::from_str(&raw_settings_value).unwrap();

        assert!(!assistant_settings.agent.is_version_outdated());
    }

    #[gpui::test]
    async fn test_load_settings_from_old_key(cx: &mut TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor().clone());
        fs.create_dir(paths::settings_file().parent().unwrap())
            .await
            .unwrap();

        cx.update(|cx| {
            let mut test_settings = settings::SettingsStore::test(cx);
            let user_settings_content = r#"{
            "assistant": {
                "enabled": true,
                "version": "2",
                "default_model": {
                  "provider": "zed.dev",
                  "model": "gpt-99"
                },
            }}"#;
            test_settings
                .set_user_settings(user_settings_content, cx)
                .unwrap();
            cx.set_global(test_settings);
            AssistantSettings::register(cx);
        });

        cx.run_until_parked();

        let assistant_settings = cx.update(|cx| AssistantSettings::get_global(cx).clone());
        assert!(assistant_settings.enabled);
        assert!(!assistant_settings.using_outdated_settings_version);
        assert_eq!(assistant_settings.default_model.model, "gpt-99");

        cx.update_global::<SettingsStore, _>(|settings_store, cx| {
            settings_store.update_user_settings::<AssistantSettings>(cx, |settings| {
                *settings = AssistantSettingsContent {
                    inner: Some(AssistantSettingsContentInner::for_v2(
                        AssistantSettingsContentV2 {
                            enabled: Some(false),
                            default_model: Some(LanguageModelSelection {
                                provider: "xai".to_owned().into(),
                                model: "grok".to_owned(),
                            }),
                            ..Default::default()
                        },
                    )),
                };
            });
        });

        cx.run_until_parked();

        let settings = cx.update(|cx| SettingsStore::global(cx).raw_user_settings().clone());

        #[derive(Debug, Deserialize)]
        struct AssistantSettingsTest {
            assistant: AssistantSettingsContent,
            agent: Option<serde_json_lenient::Value>,
        }

        let assistant_settings: AssistantSettingsTest = serde_json::from_value(settings).unwrap();
        assert!(assistant_settings.agent.is_none());
    }
}
