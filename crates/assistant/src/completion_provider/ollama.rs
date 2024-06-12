use crate::{
    assistant_settings::OllamaModel, CompletionProvider, LanguageModel, LanguageModelRequest, Role,
};
use anyhow::Result;
use futures::StreamExt as _;
use futures::{future::BoxFuture, stream::BoxStream, FutureExt};
use gpui::{AnyView, AppContext, Task};
use http::HttpClient;
use ollama::{
    get_models, stream_chat_completion, ChatMessage, ChatOptions, ChatRequest, Role as OllamaRole,
};
use std::sync::Arc;
use std::time::Duration;
use ui::{prelude::*, ButtonLike, ElevationIndex};

const OLLAMA_DOWNLOAD_URL: &str = "https://ollama.com/download";

pub struct OllamaCompletionProvider {
    api_url: String,
    model: OllamaModel,
    http_client: Arc<dyn HttpClient>,
    low_speed_timeout: Option<Duration>,
    settings_version: usize,
    available_models: Vec<OllamaModel>,
}

impl OllamaCompletionProvider {
    pub fn new(
        model: OllamaModel,
        api_url: String,
        http_client: Arc<dyn HttpClient>,
        low_speed_timeout: Option<Duration>,
        settings_version: usize,
    ) -> Self {
        Self {
            api_url,
            model,
            http_client,
            low_speed_timeout,
            settings_version,
            available_models: Default::default(),
        }
    }

    pub fn update(
        &mut self,
        model: OllamaModel,
        api_url: String,
        low_speed_timeout: Option<Duration>,
        settings_version: usize,
    ) {
        self.model = model;
        self.api_url = api_url;
        self.low_speed_timeout = low_speed_timeout;
        self.settings_version = settings_version;
    }

    pub fn available_models(&self) -> impl Iterator<Item = &OllamaModel> {
        self.available_models.iter()
    }

    pub fn settings_version(&self) -> usize {
        self.settings_version
    }

    pub fn is_authenticated(&self) -> bool {
        !self.available_models.is_empty()
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        if self.is_authenticated() {
            Task::ready(Ok(()))
        } else {
            self.fetch_models(cx)
        }
    }

    pub fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        self.fetch_models(cx)
    }

    pub fn fetch_models(&self, cx: &AppContext) -> Task<Result<()>> {
        let http_client = self.http_client.clone();
        let api_url = self.api_url.clone();

        // As a proxy for the server being "authenticated", we'll check if its up by fetching the models
        cx.spawn(|mut cx| async move {
            let models = get_models(http_client.as_ref(), &api_url, None).await?;

            let mut models: Vec<OllamaModel> = models
                .into_iter()
                // Since there is no metadata from the Ollama API
                // indicating which models are embedding models,
                // simply filter out models with "-embed" in their name
                .filter(|model| !model.name.contains("-embed"))
                .map(|model| OllamaModel::new(&model.name, &model.details.parameter_size))
                .collect();

            models.sort_by(|a, b| a.name.cmp(&b.name));

            cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                if let CompletionProvider::Ollama(provider) = provider {
                    provider.available_models = models;
                }
            })
        })
    }

    pub fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        cx.new_view(|cx| DownloadOllamaMessage::new(cx)).into()
    }

    pub fn model(&self) -> OllamaModel {
        self.model.clone()
    }

    pub fn count_tokens(
        &self,
        request: LanguageModelRequest,
        _cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        // There is no endpoint for this _yet_ in Ollama
        // see: https://github.com/ollama/ollama/issues/1716 and https://github.com/ollama/ollama/issues/3582
        let token_count = request
            .messages
            .iter()
            .map(|msg| msg.content.chars().count())
            .sum::<usize>()
            / 4;

        async move { Ok(token_count) }.boxed()
    }

    pub fn complete(
        &self,
        request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let request = self.to_ollama_request(request);

        let http_client = self.http_client.clone();
        let api_url = self.api_url.clone();
        let low_speed_timeout = self.low_speed_timeout;
        async move {
            let request =
                stream_chat_completion(http_client.as_ref(), &api_url, request, low_speed_timeout);
            let response = request.await?;
            let stream = response
                .filter_map(|response| async move {
                    match response {
                        Ok(delta) => {
                            let content = match delta.message {
                                ChatMessage::User { content } => content,
                                ChatMessage::Assistant { content } => content,
                                ChatMessage::System { content } => content,
                            };
                            Some(Ok(content))
                        }
                        Err(error) => Some(Err(error)),
                    }
                })
                .boxed();
            Ok(stream)
        }
        .boxed()
    }

    fn to_ollama_request(&self, request: LanguageModelRequest) -> ChatRequest {
        let model = match request.model {
            LanguageModel::Ollama(model) => model,
            _ => self.model(),
        };

        ChatRequest {
            model: model.name,
            messages: request
                .messages
                .into_iter()
                .map(|msg| match msg.role {
                    Role::User => ChatMessage::User {
                        content: msg.content,
                    },
                    Role::Assistant => ChatMessage::Assistant {
                        content: msg.content,
                    },
                    Role::System => ChatMessage::System {
                        content: msg.content,
                    },
                })
                .collect(),
            keep_alive: model.keep_alive,
            stream: true,
            options: Some(ChatOptions {
                num_ctx: Some(model.max_tokens),
                stop: Some(request.stop),
                temperature: Some(request.temperature),
                ..Default::default()
            }),
        }
    }
}

impl From<Role> for ollama::Role {
    fn from(val: Role) -> Self {
        match val {
            Role::User => OllamaRole::User,
            Role::Assistant => OllamaRole::Assistant,
            Role::System => OllamaRole::System,
        }
    }
}

struct DownloadOllamaMessage {}

impl DownloadOllamaMessage {
    pub fn new(_cx: &mut ViewContext<Self>) -> Self {
        Self {}
    }

    fn render_download_button(&self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        ButtonLike::new("download_ollama_button")
            .style(ButtonStyle::Filled)
            .size(ButtonSize::Large)
            .layer(ElevationIndex::ModalSurface)
            .child(Label::new("Get Ollama"))
            .on_click(move |_, cx| cx.open_url(OLLAMA_DOWNLOAD_URL))
    }
}

impl Render for DownloadOllamaMessage {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .p_4()
            .size_full()
            .child(Label::new("To use Ollama models via the assistant, Ollama must be running on your machine.").size(LabelSize::Large))
            .child(
                h_flex()
                    .w_full()
                    .p_4()
                    .justify_center()
                    .child(
                        self.render_download_button(cx)
                    )
            )
            .into_any()
    }
}
