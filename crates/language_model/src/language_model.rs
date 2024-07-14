pub mod providers;
pub mod registry;

use anyhow::Result;
use client::Client;
use futures::{future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, AppContext, Model, Task};
use providers::{
    anthropic::AnthropicLanguageModelProvider, cloud::CloudLanguageModelProvider,
    ollama::OllamaLanguageModelProvider, open_ai::OpenAiLanguageModelProvider,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    sync::Arc,
};
use ui::{Context, SharedString, WindowContext};

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    registry::init(client, cx);
}

#[derive(Debug, Clone)]
pub struct LanguageModelRequest {
    pub messages: Vec<LanguageModelRequestMessage>,
    pub stop: Vec<String>,
    pub temperature: f32,
}

#[derive(Clone)]
pub struct ProvidedLanguageModel {
    pub id: LanguageModelId,
    pub name: LanguageModelName,
}

#[derive(Clone)]
pub struct AvailableLanguageModel {
    pub provider: LanguageModelProviderName,
    pub model: ProvidedLanguageModel,
}

pub trait LanguageModel: Send + Sync {
    fn id(&self) -> LanguageModelId;
    fn name(&self) -> LanguageModelName;
    fn provider_name(&self) -> LanguageModelProviderName;
    fn telemetry_id(&self) -> String;

    fn max_token_count(&self) -> usize;

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>>;

    fn complete(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>;
}

pub trait LanguageModelProvider: 'static {
    fn name(&self, cx: &AppContext) -> LanguageModelProviderName;
    fn provided_models(&self, cx: &AppContext) -> Vec<ProvidedLanguageModel>;
    fn is_authenticated(&self, cx: &AppContext) -> bool;
    fn authenticate(&self, cx: &AppContext) -> Task<Result<()>>;
    fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView;
    fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>>;
    fn model(&self, id: LanguageModelId, cx: &AppContext) -> Result<Arc<dyn LanguageModel>>;
}

pub trait LanguageModelProviderState: 'static {
    fn subscribe<T: 'static>(&self, cx: &mut gpui::ModelContext<T>) -> gpui::Subscription;
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LanguageModelRequestMessage {
    pub role: Role,
    pub content: String,
}

impl LanguageModelRequestMessage {
    pub fn to_proto(&self) -> proto::LanguageModelRequestMessage {
        proto::LanguageModelRequestMessage {
            role: match self.role {
                Role::User => proto::LanguageModelRole::LanguageModelUser,
                Role::Assistant => proto::LanguageModelRole::LanguageModelAssistant,
                Role::System => proto::LanguageModelRole::LanguageModelSystem,
            } as i32,
            content: self.content.clone(),
            tool_calls: Vec::new(),
            tool_call_id: None,
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

impl Role {
    pub fn cycle(&mut self) {
        *self = match self {
            Role::User => Role::Assistant,
            Role::Assistant => Role::System,
            Role::System => Role::User,
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
            Role::System => write!(f, "system"),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LanguageModelId(pub SharedString);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LanguageModelName(pub SharedString);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LanguageModelProviderName(pub SharedString);

impl From<String> for LanguageModelId {
    fn from(value: String) -> Self {
        Self(SharedString::from(value))
    }
}

impl From<String> for LanguageModelName {
    fn from(value: String) -> Self {
        Self(SharedString::from(value))
    }
}

impl From<String> for LanguageModelProviderName {
    fn from(value: String) -> Self {
        Self(SharedString::from(value))
    }
}
