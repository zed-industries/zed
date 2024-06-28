mod registry;
mod zed_cloud;

pub use registry::*;
pub use zed_cloud::*;

use anyhow::Result;
use futures::{future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, AppContext, SharedString, Task, WindowContext};
use schemars::schema::RootSchema;

pub trait LanguageModel {
    fn is_authenticated(&self, cx: &mut AppContext) -> bool;

    fn authenticate(&self, cx: &mut AppContext) -> Task<Result<()>>;

    fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView;

    fn reset_credentials(&self, cx: &mut AppContext) -> Task<Result<()>>;

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &mut AppContext,
    ) -> BoxFuture<'static, Result<usize>>;

    fn complete(
        &self,
        request: LanguageModelRequest,
        cx: &mut AppContext,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<LanguageModelOutput>>>>;
}

pub enum LanguageModelOutput {
    Text(String),
    ToolCall(ToolCall),
}

pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

pub struct LanguageModelRequest {
    pub tools: Vec<LanguageModelTool>,
    pub messages: Vec<LanguageModelRequestMessage>,
    pub stop: Vec<String>,
    pub temperature: f32,
}

pub struct LanguageModelTool {
    pub name: String,
    pub description: String,
    pub parameters: RootSchema,
}

pub struct LanguageModelRequestMessage {
    role: Role,
    content: String,
}

pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LanguageModelId(pub SharedString);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LanguageModelName(pub SharedString);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LanguageModelProviderName(SharedString);

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
