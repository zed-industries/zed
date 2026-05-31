use crate::{LanguageModelProviderId, LanguageModelProviderName};

pub const ANTHROPIC_PROVIDER_ID: LanguageModelProviderId =
    LanguageModelProviderId::new("anthropic");
pub const ANTHROPIC_PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("Anthropic");

pub const OPEN_AI_PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("openai");
pub const OPEN_AI_PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("OpenAI");

pub const GOOGLE_PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("google");
pub const GOOGLE_PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("Google AI");

pub const X_AI_PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("x_ai");
pub const X_AI_PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("xAI");

pub const ZED_CLOUD_PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("zed.dev");
pub const ZED_CLOUD_PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("Zed");
