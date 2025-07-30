use anyhow::{Result, anyhow};
use futures::{FutureExt, SinkExt, StreamExt, channel::mpsc, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Context, Entity, Task};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, MessageContent, RateLimiter, Role, StopReason,
};
use mistralrs::{
    IsqType, Model as MistralModel, Response as MistralResponse, TextMessageRole, TextMessages,
    TextModelBuilder,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::{ButtonLike, IconName, Indicator, prelude::*};

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("local");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Local");
const DEFAULT_MODEL: &str = "Qwen/Qwen2.5-0.5B-Instruct";

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LocalSettings {
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableModel {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: u64,
}

pub struct LocalLanguageModelProvider {
    state: Entity<State>,
}

pub struct State {
    model: Option<Arc<MistralModel>>,
    status: ModelStatus,
}

#[derive(Clone, Debug, PartialEq)]
enum ModelStatus {
    NotLoaded,
    Loading,
    Loaded,
    Error(String),
}

impl State {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            model: None,
            status: ModelStatus::NotLoaded,
        }
    }

    fn is_authenticated(&self) -> bool {
        // Local models don't require authentication
        true
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        // Skip if already loaded or currently loading
        if matches!(self.status, ModelStatus::Loaded | ModelStatus::Loading) {
            return Task::ready(Ok(()));
        }

        self.status = ModelStatus::Loading;
        cx.notify();

        let background_executor = cx.background_executor().clone();
        cx.spawn(async move |this, cx| {
            eprintln!("Local model: Starting to load model");

            // Move the model loading to a background thread
            let model_result = background_executor
                .spawn(async move { load_mistral_model().await })
                .await;

            match model_result {
                Ok(model) => {
                    eprintln!("Local model: Model loaded successfully");
                    this.update(cx, |state, cx| {
                        state.model = Some(model);
                        state.status = ModelStatus::Loaded;
                        cx.notify();
                        eprintln!("Local model: Status updated to Loaded");
                    })?;
                    Ok(())
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    eprintln!("Local model: Failed to load model - {}", error_msg);
                    this.update(cx, |state, cx| {
                        state.status = ModelStatus::Error(error_msg.clone());
                        cx.notify();
                        eprintln!("Local model: Status updated to Failed");
                    })?;
                    Err(AuthenticateError::Other(anyhow!(
                        "Failed to load model: {}",
                        error_msg
                    )))
                }
            }
        })
    }
}

async fn load_mistral_model() -> Result<Arc<MistralModel>> {
    println!("\n\n\n\nLoading mistral model...\n\n\n");
    eprintln!("Starting to load model: {}", DEFAULT_MODEL);

    // Configure the model builder to use background threads for downloads
    eprintln!("Creating TextModelBuilder...");
    let builder = TextModelBuilder::new(DEFAULT_MODEL).with_isq(IsqType::Q4K);

    eprintln!("Building model (this should be quick for a 0.5B model)...");
    let start_time = std::time::Instant::now();

    match builder.build().await {
        Ok(model) => {
            let elapsed = start_time.elapsed();
            eprintln!("Model loaded successfully in {:?}", elapsed);
            Ok(Arc::new(model))
        }
        Err(e) => {
            eprintln!("Failed to load model: {:?}", e);
            Err(e)
        }
    }
}

impl LocalLanguageModelProvider {
    pub fn new(_http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(State::new);
        Self { state }
    }
}

impl LanguageModelProviderState for LocalLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for LocalLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconName {
        IconName::Ai
    }

    fn provided_models(&self, _cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        vec![Arc::new(LocalLanguageModel {
            state: self.state.clone(),
            request_limiter: RateLimiter::new(4),
        })]
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.provided_models(cx).into_iter().next()
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.default_model(cx)
    }

    fn is_authenticated(&self, _cx: &App) -> bool {
        // Local models don't require authentication
        true
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(&self, _window: &mut gpui::Window, cx: &mut App) -> AnyView {
        cx.new(|_cx| ConfigurationView {
            state: self.state.clone(),
        })
        .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| {
            state.model = None;
            state.status = ModelStatus::NotLoaded;
            cx.notify();
        });
        Task::ready(Ok(()))
    }
}

pub struct LocalLanguageModel {
    state: Entity<State>,
    request_limiter: RateLimiter,
}

impl LocalLanguageModel {
    fn to_mistral_messages(&self, request: &LanguageModelRequest) -> TextMessages {
        let mut messages = TextMessages::new();

        for message in &request.messages {
            let mut text_content = String::new();

            for content in &message.content {
                match content {
                    MessageContent::Text(text) => {
                        text_content.push_str(text);
                    }
                    MessageContent::Image { .. } => {
                        // For now, skip image content
                        continue;
                    }
                    MessageContent::ToolResult { .. } => {
                        // Skip tool results for now
                        continue;
                    }
                    MessageContent::Thinking { .. } => {
                        // Skip thinking content
                        continue;
                    }
                    MessageContent::RedactedThinking(_) => {
                        // Skip redacted thinking
                        continue;
                    }
                    MessageContent::ToolUse(_) => {
                        // Skip tool use
                        continue;
                    }
                }
            }

            if text_content.is_empty() {
                continue;
            }

            let role = match message.role {
                Role::User => TextMessageRole::User,
                Role::Assistant => TextMessageRole::Assistant,
                Role::System => TextMessageRole::System,
            };

            messages = messages.add_message(role, text_content);
        }

        messages
    }
}

impl LanguageModel for LocalLanguageModel {
    fn id(&self) -> LanguageModelId {
        LanguageModelId(DEFAULT_MODEL.into())
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName(DEFAULT_MODEL.into())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn telemetry_id(&self) -> String {
        format!("local/{}", DEFAULT_MODEL)
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn supports_images(&self) -> bool {
        false
    }

    fn supports_tool_choice(&self, _choice: LanguageModelToolChoice) -> bool {
        true
    }

    fn max_token_count(&self) -> u64 {
        128000 // Qwen2.5 supports 128k context
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        _cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        // Rough estimation: 1 token ≈ 4 characters
        let mut total_chars = 0;
        for message in request.messages {
            for content in message.content {
                match content {
                    MessageContent::Text(text) => total_chars += text.len(),
                    _ => {}
                }
            }
        }
        let tokens = (total_chars / 4) as u64;
        futures::future::ready(Ok(tokens)).boxed()
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
            LanguageModelCompletionError,
        >,
    > {
        let messages = self.to_mistral_messages(&request);
        let state = self.state.clone();
        let limiter = self.request_limiter.clone();

        cx.spawn(async move |cx| {
            let result: Result<
                BoxStream<
                    'static,
                    Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
                >,
                LanguageModelCompletionError,
            > = limiter
                .run(async move {
                    let model = cx
                        .read_entity(&state, |state, _| {
                            eprintln!(
                                "Local model: Checking if model is loaded: {:?}",
                                state.status
                            );
                            state.model.clone()
                        })
                        .map_err(|_| {
                            LanguageModelCompletionError::Other(anyhow!("App state dropped"))
                        })?
                        .ok_or_else(|| {
                            eprintln!("Local model: Model is not loaded!");
                            LanguageModelCompletionError::Other(anyhow!("Model not loaded"))
                        })?;

                    let (mut tx, rx) = mpsc::channel(32);

                    // Spawn a task to handle the stream
                    let _ = smol::spawn(async move {
                        let mut stream = match model.stream_chat_request(messages).await {
                            Ok(stream) => stream,
                            Err(e) => {
                                let _ = tx
                                    .send(Err(LanguageModelCompletionError::Other(anyhow!(
                                        "Failed to start stream: {}",
                                        e
                                    ))))
                                    .await;
                                return;
                            }
                        };

                        while let Some(response) = stream.next().await {
                            let event = match response {
                                MistralResponse::Chunk(chunk) => {
                                    if let Some(choice) = chunk.choices.first() {
                                        if let Some(content) = &choice.delta.content {
                                            Some(Ok(LanguageModelCompletionEvent::Text(
                                                content.clone(),
                                            )))
                                        } else if let Some(finish_reason) = &choice.finish_reason {
                                            let stop_reason = match finish_reason.as_str() {
                                                "stop" => StopReason::EndTurn,
                                                "length" => StopReason::MaxTokens,
                                                _ => StopReason::EndTurn,
                                            };
                                            Some(Ok(LanguageModelCompletionEvent::Stop(
                                                stop_reason,
                                            )))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                MistralResponse::Done(_response) => {
                                    // For now, we don't emit usage events since the format doesn't match
                                    None
                                }
                                _ => None,
                            };

                            if let Some(event) = event {
                                if tx.send(event).await.is_err() {
                                    break;
                                }
                            }
                        }
                    })
                    .detach();

                    Ok(rx.boxed())
                })
                .await;

            result
        })
        .boxed()
    }
}

struct ConfigurationView {
    state: Entity<State>,
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let status = self.state.read(cx).status.clone();

        div().size_full().child(
            div()
                .p_4()
                .child(
                    div()
                        .flex()
                        .gap_2()
                        .items_center()
                        .child(match &status {
                            ModelStatus::NotLoaded => Label::new("Model not loaded"),
                            ModelStatus::Loading => Label::new("Loading model..."),
                            ModelStatus::Loaded => Label::new("Model loaded"),
                            ModelStatus::Error(e) => Label::new(format!("Error: {}", e)),
                        })
                        .child(match &status {
                            ModelStatus::NotLoaded => Indicator::dot().color(Color::Disabled),
                            ModelStatus::Loading => Indicator::dot().color(Color::Modified),
                            ModelStatus::Loaded => Indicator::dot().color(Color::Success),
                            ModelStatus::Error(_) => Indicator::dot().color(Color::Error),
                        }),
                )
                .when(!matches!(status, ModelStatus::Loading), |this| {
                    this.child(
                        ButtonLike::new("load_model")
                            .child(Label::new(if matches!(status, ModelStatus::Loaded) {
                                "Reload Model"
                            } else {
                                "Load Model"
                            }))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.state.update(cx, |state, cx| {
                                    state.authenticate(cx).detach();
                                });
                            })),
                    )
                }),
        )
    }
}

#[cfg(test)]
mod tests;
