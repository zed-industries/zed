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

impl AssistantSettingsContent {
    pub fn set_dock(&mut self, dock: AssistantDockPosition) {
        self.dock = Some(dock);
    }

    pub fn set_model(&mut self, language_model: Arc<dyn LanguageModel>) {
        let model = language_model.id().0.to_string();
        let provider = language_model.provider_id().0.to_string();

        self.default_model = Some(LanguageModelSelection {
            provider: provider.into(),
            model,
        });
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

    pub fn set_always_allow_tool_actions(&mut self, allow: bool) {
        self.always_allow_tool_actions = Some(allow);
    }

    pub fn set_single_file_review(&mut self, allow: bool) {
        self.single_file_review = Some(allow);
    }

    pub fn set_profile(&mut self, profile_id: AgentProfileId) {
        self.default_profile = Some(profile_id);
    }

    pub fn create_profile(
        &mut self,
        profile_id: AgentProfileId,
        profile: AgentProfile,
    ) -> Result<()> {
        let profiles = self.profiles.get_or_insert_default();
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
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug, Default)]
pub struct AssistantSettingsContent {
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
            merge(&mut settings.default_model, value.default_model.clone());
            settings.inline_assistant_model = value
                .inline_assistant_model
                .clone()
                .or(settings.inline_assistant_model.take());
            settings.commit_message_model = value
                .commit_message_model
                .clone()
                .or(settings.commit_message_model.take());
            settings.thread_summary_model = value
                .thread_summary_model
                .clone()
                .or(settings.thread_summary_model.take());
            merge(
                &mut settings.inline_alternatives,
                value.inline_alternatives.clone(),
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
            merge(&mut settings.default_profile, value.default_profile.clone());
            merge(
                &mut settings.preferred_completion_mode,
                value.preferred_completion_mode,
            );

            settings
                .model_parameters
                .extend_from_slice(&value.model_parameters);

            if let Some(profiles) = value.profiles.clone() {
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
            current.enabled = Some(b);
            current.button = Some(b);
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

    // #[gpui::test]
    // async fn test_deserialize_assistant_settings_with_version(cx: &mut TestAppContext) {
    //     let fs = fs::FakeFs::new(cx.executor().clone());
    //     fs.create_dir(paths::settings_file().parent().unwrap())
    //         .await
    //         .unwrap();

    //     cx.update(|cx| {
    //         let test_settings = settings::SettingsStore::test(cx);
    //         cx.set_global(test_settings);
    //         AssistantSettings::register(cx);
    //     });

    //     cx.update(|cx| {
    //         assert!(!AssistantSettings::get_global(cx).using_outdated_settings_version);
    //         assert_eq!(
    //             AssistantSettings::get_global(cx).default_model,
    //             LanguageModelSelection {
    //                 provider: "zed.dev".into(),
    //                 model: "claude-3-7-sonnet-latest".into(),
    //             }
    //         );
    //     });

    //     cx.update(|cx| {
    //         settings::SettingsStore::global(cx).update_settings_file::<AssistantSettings>(
    //             fs.clone(),
    //             |settings, _| {
    //                 *settings = AssistantSettingsContent {
    //                     inner: Some(AssistantSettingsContentInner::for_v2(
    //                         AssistantSettingsContentV2 {
    //                             default_model: Some(LanguageModelSelection {
    //                                 provider: "test-provider".into(),
    //                                 model: "gpt-99".into(),
    //                             }),
    //                             inline_assistant_model: None,
    //                             commit_message_model: None,
    //                             thread_summary_model: None,
    //                             inline_alternatives: None,
    //                             enabled: None,
    //                             button: None,
    //                             dock: None,
    //                             default_width: None,
    //                             default_height: None,
    //                             default_profile: None,
    //                             profiles: None,
    //                             always_allow_tool_actions: None,
    //                             notify_when_agent_waiting: None,
    //                             stream_edits: None,
    //                             single_file_review: None,
    //                             model_parameters: Vec::new(),
    //                             preferred_completion_mode: None,
    //                         },
    //                     )),
    //                 }
    //             },
    //         );
    //     });

    //     cx.run_until_parked();

    //     let raw_settings_value = fs.load(paths::settings_file()).await.unwrap();
    //     assert!(raw_settings_value.contains(r#""version": "2""#));

    //     #[derive(Debug, Deserialize)]
    //     struct AssistantSettingsTest {
    //         agent: AssistantSettingsContent,
    //     }

    //     let assistant_settings: AssistantSettingsTest =
    //         serde_json_lenient::from_str(&raw_settings_value).unwrap();

    //     assert!(!assistant_settings.agent.is_version_outdated());
    // }

    // #[gpui::test]
    // async fn test_load_settings_from_old_key(cx: &mut TestAppContext) {
    //     let fs = fs::FakeFs::new(cx.executor().clone());
    //     fs.create_dir(paths::settings_file().parent().unwrap())
    //         .await
    //         .unwrap();

    //     cx.update(|cx| {
    //         let mut test_settings = settings::SettingsStore::test(cx);
    //         let user_settings_content = r#"{
    //         "assistant": {
    //             "enabled": true,
    //             "version": "2",
    //             "default_model": {
    //               "provider": "zed.dev",
    //               "model": "gpt-99"
    //             },
    //         }}"#;
    //         test_settings
    //             .set_user_settings(user_settings_content, cx)
    //             .unwrap();
    //         cx.set_global(test_settings);
    //         AssistantSettings::register(cx);
    //     });

    //     cx.run_until_parked();

    //     let assistant_settings = cx.update(|cx| AssistantSettings::get_global(cx).clone());
    //     assert!(assistant_settings.enabled);
    //     assert!(!assistant_settings.using_outdated_settings_version);
    //     assert_eq!(assistant_settings.default_model.model, "gpt-99");

    //     cx.update_global::<SettingsStore, _>(|settings_store, cx| {
    //         settings_store.update_user_settings::<AssistantSettings>(cx, |settings| {
    //             *settings = AssistantSettingsContent {
    //                 inner: Some(AssistantSettingsContentInner::for_v2(
    //                     AssistantSettingsContent {
    //                         enabled: Some(false),
    //                         default_model: Some(LanguageModelSelection {
    //                             provider: "xai".to_owned().into(),
    //                             model: "grok".to_owned(),
    //                         }),
    //                         ..Default::default()
    //                     },
    //                 )),
    //             };
    //         });
    //     });

    //     cx.run_until_parked();

    //     let settings = cx.update(|cx| SettingsStore::global(cx).raw_user_settings().clone());

    //     #[derive(Debug, Deserialize)]
    //     struct AssistantSettingsTest {
    //         assistant: AssistantSettingsContent,
    //         agent: Option<serde_json_lenient::Value>,
    //     }

    //     let assistant_settings: AssistantSettingsTest = serde_json::from_value(settings).unwrap();
    //     assert!(assistant_settings.agent.is_none());
    // }
}
