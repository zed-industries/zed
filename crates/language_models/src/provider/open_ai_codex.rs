use anyhow::{Context as _, Result, anyhow};
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, Window};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, RateLimiter,
};
use open_ai::codex::{
    CODEX_API_URL, CODEX_OAUTH_CREDENTIALS_KEY, CodexOAuthSession, create_codex_authorization_flow,
    exchange_codex_authorization_code, now_ms, parse_codex_authorization_callback_path,
    refresh_codex_session,
};
use open_ai::responses::{
    Request as ResponseRequest, ResponseInputContent, ResponseInputItem, ResponseTextConfig,
    ResponseTextVerbosity, StreamEvent, stream_codex_response,
};
use settings::Settings;
use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::Arc,
    time::Duration,
};
use strum::IntoEnumIterator;
use ui::{Button, ButtonStyle, ConfiguredApiCard, TintColor, prelude::*};
use util::ResultExt;

use crate::provider::open_ai::{OpenAiResponseEventMapper, into_open_ai_response};

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("openai-codex");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("OpenAI Codex");

const OAUTH_CALLBACK_TIMEOUT: Duration = Duration::from_secs(300);

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenAiCodexSettings {
    pub available_models: Vec<settings::OpenAiCodexAvailableModel>,
}

pub struct OpenAiCodexLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    session: Option<CodexOAuthSession>,
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.session.is_some()
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let http_client = self.http_client.clone();
        cx.spawn(async move |this, cx| {
            match load_codex_session(credentials_provider.clone(), http_client, cx).await {
                Ok(Some(session)) => {
                    this.update(cx, |this, cx| {
                        this.session = Some(session);
                        cx.notify();
                    })
                    .map_err(AuthenticateError::Other)?;
                    Ok(())
                }
                Ok(None) => Err(AuthenticateError::CredentialsNotFound),
                Err(error) => Err(AuthenticateError::Other(error)),
            }
        })
    }

    fn login(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let http_client = self.http_client.clone();
        cx.spawn(async move |this, cx| {
            let flow = create_codex_authorization_flow().map_err(AuthenticateError::Other)?;
            let code_task = cx.background_spawn(wait_for_authorization_code(flow.state.clone()));
            cx.update(|cx| cx.open_url(&flow.url));
            let code = code_task.await.map_err(AuthenticateError::Other)?;
            let session =
                exchange_codex_authorization_code(http_client.as_ref(), &code, &flow.verifier)
                    .await
                    .map_err(AuthenticateError::Other)?;
            store_codex_session(credentials_provider.as_ref(), &session, cx)
                .await
                .map_err(AuthenticateError::Other)?;
            this.update(cx, |this, cx| {
                this.session = Some(session);
                cx.notify();
            })
            .map_err(AuthenticateError::Other)?;
            Ok(())
        })
    }

    fn authenticated_session(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Task<Result<CodexOAuthSession, LanguageModelCompletionError>> {
        let credentials_provider = self.credentials_provider.clone();
        let http_client = self.http_client.clone();
        cx.spawn(async move |this, cx| {
            let Some(session) = load_codex_session(credentials_provider, http_client, cx)
                .await
                .map_err(LanguageModelCompletionError::Other)?
            else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };

            this.update(cx, |this, cx| {
                this.session = Some(session.clone());
                cx.notify();
            })
            .map_err(LanguageModelCompletionError::Other)?;

            Ok(session)
        })
    }

    fn reset_credentials(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        cx.spawn(async move |this, cx| {
            credentials_provider
                .delete_credentials(CODEX_OAUTH_CREDENTIALS_KEY, cx)
                .await?;
            this.update(cx, |this, cx| {
                this.session = None;
                cx.notify();
            })
        })
    }
}

impl OpenAiCodexLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|_| State {
            session: None,
            credentials_provider,
            http_client: http_client.clone(),
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: open_ai::Model) -> Arc<dyn LanguageModel> {
        Arc::new(OpenAiCodexLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    fn settings(cx: &App) -> &OpenAiCodexSettings {
        &crate::AllLanguageModelSettings::get_global(cx).openai_codex
    }
}

impl LanguageModelProviderState for OpenAiCodexLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for OpenAiCodexLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiOpenAi)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(open_ai::Model::FiveCodex))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(open_ai::Model::FiveCodex))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        for model in open_ai::Model::iter() {
            if !matches!(model, open_ai::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        for model in &Self::settings(cx).available_models {
            models.insert(
                model.name.clone(),
                open_ai::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    max_output_tokens: model.max_output_tokens,
                    max_completion_tokens: model.max_completion_tokens,
                    reasoning_effort: Some(model.reasoning_effort),
                    supports_chat_completions: false,
                    supports_images: model.supports_images,
                },
            );
        }

        models
            .into_values()
            .map(|model| self.create_language_model(model))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.reset_credentials(cx))
    }
}

pub struct OpenAiCodexLanguageModel {
    id: LanguageModelId,
    model: open_ai::Model,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OpenAiCodexLanguageModel {
    fn stream_response(
        &self,
        mut request: ResponseRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<StreamEvent>>>> {
        let http_client = self.http_client.clone();
        let state = self.state.clone();
        let session_task =
            cx.update(|cx| state.update(cx, |state, cx| state.authenticated_session(cx)));

        request.store = Some(false);
        request.instructions = take_system_instructions(&mut request.input);
        request.include = vec!["reasoning.encrypted_content".to_string()];
        request.text = Some(ResponseTextConfig {
            verbosity: ResponseTextVerbosity::Low,
        });

        let future = self.request_limiter.stream(async move {
            let session = session_task.await?;
            let provider_name = PROVIDER_NAME.0.to_string();
            let user_agent = codex_user_agent();
            let request = stream_codex_response(
                http_client.as_ref(),
                &provider_name,
                CODEX_API_URL,
                &session.access_token,
                &session.account_id,
                &user_agent,
                request,
            );
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for OpenAiCodexLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images()
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        matches!(
            choice,
            LanguageModelToolChoice::Auto
                | LanguageModelToolChoice::Any
                | LanguageModelToolChoice::None
        )
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_thinking(&self) -> bool {
        self.model.reasoning_effort().is_some()
    }

    fn supports_split_token_display(&self) -> bool {
        true
    }

    fn telemetry_id(&self) -> String {
        format!("openai-codex/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens()
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
            >,
            LanguageModelCompletionError,
        >,
    > {
        let request = into_open_ai_response(
            request,
            self.model.id(),
            self.model.supports_parallel_tool_calls(),
            self.model.supports_prompt_cache_key(),
            self.max_output_tokens(),
            self.model.reasoning_effort(),
        );
        let completions = self.stream_response(request, cx);
        async move {
            let mapper = OpenAiResponseEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
        }
        .boxed()
    }
}

fn take_system_instructions(input: &mut Vec<ResponseInputItem>) -> Option<String> {
    let mut instructions = Vec::new();
    let mut next_input = Vec::with_capacity(input.len());

    for item in input.drain(..) {
        match item {
            ResponseInputItem::Message(message) if message.role == open_ai::Role::System => {
                for content in message.content {
                    if let ResponseInputContent::Text { text } = content
                        && !text.trim().is_empty()
                    {
                        instructions.push(text);
                    }
                }
            }
            item => next_input.push(item),
        }
    }

    *input = next_input;

    if instructions.is_empty() {
        None
    } else {
        Some(instructions.join("\n\n"))
    }
}

struct ConfigurationView {
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
    login_task: Option<Task<()>>,
    last_error: Option<SharedString>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn_in(window, {
            let state = state.clone();
            async move |this, cx| {
                let result = state.update(cx, |state, cx| state.authenticate(cx)).await;
                this.update(cx, |this, cx| {
                    if let Err(AuthenticateError::Other(error)) = result {
                        this.last_error = Some(error.to_string().into());
                    }
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            state,
            load_credentials_task,
            login_task: None,
            last_error: None,
        }
    }

    fn login(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let state = self.state.clone();
        self.last_error = None;
        self.login_task = Some(cx.spawn_in(window, async move |this, cx| {
            let result = state.update(cx, |state, cx| state.login(cx)).await;
            this.update(cx, |this, cx| {
                if let Err(error) = result {
                    this.last_error = Some(error.to_string().into());
                }
                this.login_task = None;
                cx.notify();
            })
            .log_err();
        }));
        cx.notify();
    }

    fn reset_credentials(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let state = self.state.clone();
        self.login_task = Some(cx.spawn_in(window, async move |this, cx| {
            let result = state
                .update(cx, |state, cx| state.reset_credentials(cx))
                .await;
            this.update(cx, |this, cx| {
                if let Err(error) = result {
                    this.last_error = Some(error.to_string().into());
                }
                this.login_task = None;
                cx.notify();
            })
            .log_err();
        }));
        cx.notify();
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_authenticated = self.state.read(cx).is_authenticated();
        let is_busy = self.login_task.is_some();

        v_flex()
            .gap_2()
            .when_some(self.last_error.clone(), |this, error| {
                this.child(Label::new(error).size(LabelSize::Small).color(Color::Error))
            })
            .map(|this| {
                if is_authenticated {
                    this.child(
                        ConfiguredApiCard::new("Signed in with ChatGPT for Codex").on_click(
                            cx.listener(|this, _, window, cx| this.reset_credentials(window, cx)),
                        ),
                    )
                } else {
                    this.child(
                        Label::new(
                            "Sign in with ChatGPT to use Codex models through your ChatGPT plan.",
                        )
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                    )
                    .child(
                        Button::new(
                            "openai-codex-sign-in",
                            if is_busy {
                                "Signing in…"
                            } else {
                                "Sign in with ChatGPT"
                            },
                        )
                        .style(ButtonStyle::Tinted(TintColor::Accent))
                        .full_width()
                        .disabled(is_busy)
                        .on_click(cx.listener(|this, _, window, cx| this.login(window, cx))),
                    )
                }
            })
            .into_any_element()
    }
}

async fn load_codex_session(
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
    cx: &AsyncApp,
) -> Result<Option<CodexOAuthSession>> {
    let Some((_, bytes)) = credentials_provider
        .read_credentials(CODEX_OAUTH_CREDENTIALS_KEY, cx)
        .await?
    else {
        return Ok(None);
    };

    let mut session: CodexOAuthSession =
        serde_json::from_slice(&bytes).context("failed to parse OpenAI Codex credentials")?;
    if session.expires_at_ms <= now_ms() + Duration::from_secs(60).as_millis() as u64 {
        session = refresh_codex_session(http_client.as_ref(), &session.refresh_token).await?;
        store_codex_session(credentials_provider.as_ref(), &session, cx).await?;
    }
    Ok(Some(session))
}

async fn store_codex_session(
    credentials_provider: &dyn CredentialsProvider,
    session: &CodexOAuthSession,
    cx: &AsyncApp,
) -> Result<()> {
    let bytes = serde_json::to_vec(session)?;
    credentials_provider
        .write_credentials(CODEX_OAUTH_CREDENTIALS_KEY, "oauth", &bytes, cx)
        .await
}

async fn wait_for_authorization_code(state: String) -> Result<String> {
    let listener = TcpListener::bind("127.0.0.1:1455")
        .context("failed to start OpenAI Codex OAuth callback server on 127.0.0.1:1455")?;
    listener.set_nonblocking(true)?;
    let started_at = std::time::Instant::now();
    let mut stream = loop {
        match listener.accept() {
            Ok((stream, _)) => break stream,
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                if started_at.elapsed() >= OAUTH_CALLBACK_TIMEOUT {
                    return Err(anyhow!("Timed out waiting for OpenAI Codex OAuth callback"));
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(error) => return Err(error.into()),
        }
    };
    stream.set_nonblocking(false)?;
    let mut buffer = [0_u8; 4096];
    let bytes_read = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request_line = request
        .lines()
        .next()
        .ok_or_else(|| anyhow!("OAuth callback did not include an HTTP request line"))?;
    let path = request_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| anyhow!("OAuth callback request line was malformed"))?;
    let code = parse_codex_authorization_callback_path(path, &state);
    let (status, body) = if code.is_ok() {
        (
            "200 OK",
            "OpenAI Codex authentication completed. You can close this window.",
        )
    } else {
        ("400 Bad Request", "OpenAI Codex authentication failed.")
    };
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )?;

    code
}

fn codex_user_agent() -> String {
    format!(
        "Zed ({}/{}; language_models {})",
        std::env::consts::OS,
        std::env::consts::ARCH,
        env!("CARGO_PKG_VERSION")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_ai::responses::ResponseMessageItem;

    #[test]
    fn extracts_system_messages_as_instructions() {
        let mut input = vec![
            ResponseInputItem::Message(ResponseMessageItem {
                role: open_ai::Role::System,
                content: vec![ResponseInputContent::Text {
                    text: "First instruction".to_string(),
                }],
            }),
            ResponseInputItem::Message(ResponseMessageItem {
                role: open_ai::Role::User,
                content: vec![ResponseInputContent::Text {
                    text: "User message".to_string(),
                }],
            }),
            ResponseInputItem::Message(ResponseMessageItem {
                role: open_ai::Role::System,
                content: vec![ResponseInputContent::Text {
                    text: "Second instruction".to_string(),
                }],
            }),
        ];

        assert_eq!(
            take_system_instructions(&mut input).as_deref(),
            Some("First instruction\n\nSecond instruction")
        );
        assert_eq!(input.len(), 1);
        assert!(matches!(
            &input[0],
            ResponseInputItem::Message(message) if message.role == open_ai::Role::User
        ));
    }
}
