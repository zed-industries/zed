use std::fmt;
use std::sync::Arc;

use anyhow::Result;
use client::Client;
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, Global, ReadGlobal as _,
};
use proto::{Plan, TypedEnvelope};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use smol::lock::{RwLock, RwLockUpgradableReadGuard, RwLockWriteGuard};
use strum::EnumIter;
use thiserror::Error;

use crate::{LanguageModelAvailability, LanguageModelToolSchemaFormat};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "provider", rename_all = "lowercase")]
pub enum CloudModel {
    Anthropic(anthropic::Model),
    OpenAi(open_ai::Model),
    Google(google_ai::Model),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, EnumIter)]
pub enum ZedModel {
    #[serde(rename = "Qwen/Qwen2-7B-Instruct")]
    Qwen2_7bInstruct,
}

impl Default for CloudModel {
    fn default() -> Self {
        Self::Anthropic(anthropic::Model::default())
    }
}

impl CloudModel {
    pub fn id(&self) -> &str {
        match self {
            Self::Anthropic(model) => model.id(),
            Self::OpenAi(model) => model.id(),
            Self::Google(model) => model.id(),
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Anthropic(model) => model.display_name(),
            Self::OpenAi(model) => model.display_name(),
            Self::Google(model) => model.display_name(),
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Self::Anthropic(model) => model.max_token_count(),
            Self::OpenAi(model) => model.max_token_count(),
            Self::Google(model) => model.max_token_count(),
        }
    }

    /// Returns the availability of this model.
    pub fn availability(&self) -> LanguageModelAvailability {
        match self {
            Self::Anthropic(model) => match model {
                anthropic::Model::Claude3_5Sonnet
                | anthropic::Model::Claude3_7Sonnet
                | anthropic::Model::Claude3_7SonnetThinking => {
                    LanguageModelAvailability::RequiresPlan(Plan::Free)
                }
                anthropic::Model::Claude3Opus
                | anthropic::Model::Claude3Sonnet
                | anthropic::Model::Claude3Haiku
                | anthropic::Model::Claude3_5Haiku
                | anthropic::Model::Custom { .. } => {
                    LanguageModelAvailability::RequiresPlan(Plan::ZedPro)
                }
            },
            Self::OpenAi(model) => match model {
                open_ai::Model::ThreePointFiveTurbo
                | open_ai::Model::Four
                | open_ai::Model::FourTurbo
                | open_ai::Model::FourOmni
                | open_ai::Model::FourOmniMini
                | open_ai::Model::FourPointOne
                | open_ai::Model::FourPointOneMini
                | open_ai::Model::FourPointOneNano
                | open_ai::Model::O1Mini
                | open_ai::Model::O1Preview
                | open_ai::Model::O1
                | open_ai::Model::O3Mini
                | open_ai::Model::O3
                | open_ai::Model::O4Mini
                | open_ai::Model::Custom { .. } => {
                    LanguageModelAvailability::RequiresPlan(Plan::ZedPro)
                }
            },
            Self::Google(model) => match model {
                google_ai::Model::Gemini15Pro
                | google_ai::Model::Gemini15Flash
                | google_ai::Model::Gemini20Pro
                | google_ai::Model::Gemini20Flash
                | google_ai::Model::Gemini20FlashThinking
                | google_ai::Model::Gemini20FlashLite
                | google_ai::Model::Gemini25ProExp0325
                | google_ai::Model::Gemini25ProPreview0325
                | google_ai::Model::Custom { .. } => {
                    LanguageModelAvailability::RequiresPlan(Plan::ZedPro)
                }
            },
        }
    }

    pub fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        match self {
            Self::Anthropic(_) | Self::OpenAi(_) => LanguageModelToolSchemaFormat::JsonSchema,
            Self::Google(_) => LanguageModelToolSchemaFormat::JsonSchemaSubset,
        }
    }
}

#[derive(Error, Debug)]
pub struct PaymentRequiredError;

impl fmt::Display for PaymentRequiredError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Payment required to use this language model. Please upgrade your account."
        )
    }
}

#[derive(Error, Debug)]
pub struct MaxMonthlySpendReachedError;

impl fmt::Display for MaxMonthlySpendReachedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Maximum spending limit reached for this month. For more usage, increase your spending limit."
        )
    }
}

#[derive(Error, Debug)]
pub struct ModelRequestLimitReachedError {
    pub plan: Plan,
}

impl fmt::Display for ModelRequestLimitReachedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self.plan {
            Plan::Free => "Model request limit reached. Upgrade to Zed Pro for more requests.",
            Plan::ZedPro => {
                "Model request limit reached. Upgrade to usage-based billing for more requests."
            }
            Plan::ZedProTrial => {
                "Model request limit reached. Upgrade to Zed Pro for more requests."
            }
        };

        write!(f, "{message}")
    }
}

#[derive(Clone, Default)]
pub struct LlmApiToken(Arc<RwLock<Option<String>>>);

impl LlmApiToken {
    pub async fn acquire(&self, client: &Arc<Client>) -> Result<String> {
        let lock = self.0.upgradable_read().await;
        if let Some(token) = lock.as_ref() {
            Ok(token.to_string())
        } else {
            Self::fetch(RwLockUpgradableReadGuard::upgrade(lock).await, client).await
        }
    }

    pub async fn refresh(&self, client: &Arc<Client>) -> Result<String> {
        Self::fetch(self.0.write().await, client).await
    }

    async fn fetch(
        mut lock: RwLockWriteGuard<'_, Option<String>>,
        client: &Arc<Client>,
    ) -> Result<String> {
        let response = client.request(proto::GetLlmToken {}).await?;
        *lock = Some(response.token.clone());
        Ok(response.token.clone())
    }
}

struct GlobalRefreshLlmTokenListener(Entity<RefreshLlmTokenListener>);

impl Global for GlobalRefreshLlmTokenListener {}

pub struct RefreshLlmTokenEvent;

pub struct RefreshLlmTokenListener {
    _llm_token_subscription: client::Subscription,
}

impl EventEmitter<RefreshLlmTokenEvent> for RefreshLlmTokenListener {}

impl RefreshLlmTokenListener {
    pub fn register(client: Arc<Client>, cx: &mut App) {
        let listener = cx.new(|cx| RefreshLlmTokenListener::new(client, cx));
        cx.set_global(GlobalRefreshLlmTokenListener(listener));
    }

    pub fn global(cx: &App) -> Entity<Self> {
        GlobalRefreshLlmTokenListener::global(cx).0.clone()
    }

    fn new(client: Arc<Client>, cx: &mut Context<Self>) -> Self {
        Self {
            _llm_token_subscription: client
                .add_message_handler(cx.weak_entity(), Self::handle_refresh_llm_token),
        }
    }

    async fn handle_refresh_llm_token(
        this: Entity<Self>,
        _: TypedEnvelope<proto::RefreshLlmToken>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |_this, cx| cx.emit(RefreshLlmTokenEvent))
    }
}
