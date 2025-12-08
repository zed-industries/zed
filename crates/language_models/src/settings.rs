use std::sync::Arc;

use collections::HashMap;
use settings::RegisterSetting;

use crate::provider::{
    bedrock::AmazonBedrockSettings, cloud::ZedDotDevSettings, deepseek::DeepSeekSettings,
    google::GoogleSettings, lmstudio::LmStudioSettings, mistral::MistralSettings,
    ollama::OllamaSettings, open_ai::OpenAiSettings, open_ai_compatible::OpenAiCompatibleSettings,
    vercel::VercelSettings, x_ai::XAiSettings,
};

pub use settings::OpenRouterAvailableModel as AvailableModel;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenRouterSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

#[derive(Debug, RegisterSetting)]
pub struct AllLanguageModelSettings {
    pub bedrock: AmazonBedrockSettings,
    pub deepseek: DeepSeekSettings,
    pub google: GoogleSettings,
    pub lmstudio: LmStudioSettings,
    pub mistral: MistralSettings,
    pub ollama: OllamaSettings,
    pub open_router: OpenRouterSettings,
    pub openai: OpenAiSettings,
    pub openai_compatible: HashMap<Arc<str>, OpenAiCompatibleSettings>,
    pub vercel: VercelSettings,
    pub x_ai: XAiSettings,
    pub zed_dot_dev: ZedDotDevSettings,
}

impl settings::Settings for AllLanguageModelSettings {
    const PRESERVED_KEYS: Option<&'static [&'static str]> = Some(&["version"]);

    fn from_settings(content: &settings::SettingsContent) -> Self {
        let language_models = content.language_models.clone().unwrap();
        let bedrock = language_models.bedrock.unwrap();
        let deepseek = language_models.deepseek.unwrap();
        let google = language_models.google.unwrap();
        let lmstudio = language_models.lmstudio.unwrap();
        let mistral = language_models.mistral.unwrap();
        let ollama = language_models.ollama.unwrap();
        let open_router = language_models.open_router.unwrap();
        let openai = language_models.openai.unwrap();
        let openai_compatible = language_models.openai_compatible.unwrap();
        let vercel = language_models.vercel.unwrap();
        let x_ai = language_models.x_ai.unwrap();
        let zed_dot_dev = language_models.zed_dot_dev.unwrap();
        Self {
            bedrock: AmazonBedrockSettings {
                available_models: bedrock.available_models.unwrap_or_default(),
                region: bedrock.region,
                endpoint: bedrock.endpoint_url,
                profile_name: bedrock.profile,
                role_arn: None,
                authentication_method: bedrock.authentication_method.map(Into::into),
                allow_global: bedrock.allow_global,
            },
            deepseek: DeepSeekSettings {
                api_url: deepseek.api_url.unwrap(),
                available_models: deepseek.available_models.unwrap_or_default(),
            },
            google: GoogleSettings {
                api_url: google.api_url.unwrap(),
                available_models: google.available_models.unwrap_or_default(),
            },
            lmstudio: LmStudioSettings {
                api_url: lmstudio.api_url.unwrap(),
                available_models: lmstudio.available_models.unwrap_or_default(),
            },
            mistral: MistralSettings {
                api_url: mistral.api_url.unwrap(),
                available_models: mistral.available_models.unwrap_or_default(),
            },
            ollama: OllamaSettings {
                api_url: ollama.api_url.unwrap(),
                available_models: ollama.available_models.unwrap_or_default(),
            },
            open_router: OpenRouterSettings {
                api_url: open_router.api_url.unwrap(),
                available_models: open_router.available_models.unwrap_or_default(),
            },
            openai: OpenAiSettings {
                api_url: openai.api_url.unwrap(),
                available_models: openai.available_models.unwrap_or_default(),
            },
            openai_compatible: openai_compatible
                .into_iter()
                .map(|(key, value)| {
                    (
                        key,
                        OpenAiCompatibleSettings {
                            api_url: value.api_url,
                            available_models: value.available_models,
                        },
                    )
                })
                .collect(),
            vercel: VercelSettings {
                api_url: vercel.api_url.unwrap(),
                available_models: vercel.available_models.unwrap_or_default(),
            },
            x_ai: XAiSettings {
                api_url: x_ai.api_url.unwrap(),
                available_models: x_ai.available_models.unwrap_or_default(),
            },
            zed_dot_dev: ZedDotDevSettings {
                available_models: zed_dot_dev.available_models.unwrap_or_default(),
            },
        }
    }
}
