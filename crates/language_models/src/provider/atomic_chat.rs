use anyhow::Result;
use atomic_chat::{ATOMIC_CHAT_API_URL, get_models, Model};
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{
    AnyView, App, AsyncApp, Context, CursorStyle, Entity, Render, Subscription, Task, TaskExt,
    Window,
};
use http_client::HttpClient;
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelToolChoice, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelId, LanguageModelName, RateLimiter, env_var,
};
use open_ai::{ResponseStreamEvent, stream_completion};
use settings::{Settings, SettingsStore, update_settings_file};
use std::collections::BTreeMap;
use std::sync::{Arc, LazyLock};

use crate::AllLanguageModelSettings;
use crate::provider::open_ai::{OpenAiEventMapper, into_open_ai};
use fs::Fs;
use menu;
use ui::{
    ButtonLike, ConfiguredApiCard, ElevationIndex, List, ListBulletItem, Tooltip, prelude::*,
};
use ui_input::InputField;

pub use settings::AtomicChatAvailableModel as AvailableModel;

const ATOMIC_CHAT_SITE: &str = "https://atomic.chat/";

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("atomic_chat");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Atomic Chat");

const API_KEY_ENV_VAR_NAME: &str = "ATOMIC_CHAT_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AtomicChatSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

pub struct AtomicChatLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
    available_models: Vec<Model>,
    fetch_model_task: Option<Task<Result<()>>>,
    _subscription: Subscription,
}

impl State {
    fn is_authenticated(&self) -> bool {
        !self.available_models.is_empty()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = AtomicChatLanguageModelProvider::api_url(cx).into();
        let task = self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );
        self.restart_fetch_models_task(cx);
        task
    }

    fn fetch_models(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let settings = &AllLanguageModelSettings::get_global(cx).atomic_chat;
        let http_client = self.http_client.clone();
        let api_url = settings.api_url.clone();
        let api_key = self.api_key_state.key(&api_url);

        cx.spawn(async move |this, cx| {
            let models = get_models(http_client.as_ref(), &api_url, api_key.as_deref()).await?;

            this.update(cx, |this, cx| {
                this.available_models = models;
                cx.notify();
            })
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        let task = self.fetch_models(cx);
        self.fetch_model_task.replace(task);
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = AtomicChatLanguageModelProvider::api_url(cx).into();
        let _task = self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        let fetch_models_task = self.fetch_models(cx);
        cx.spawn(async move |_this, _cx| {
            match fetch_models_task.await {
                Ok(()) => Ok(()),
                Err(err) => {
                    let mut connection_refused = false;
                    for cause in err.chain() {
                        if let Some(io_err) = cause.downcast_ref::<std::io::Error>() {
                            if io_err.kind() == std::io::ErrorKind::ConnectionRefused {
                                connection_refused = true;
                                break;
                            }
                        }
                    }
                    if connection_refused {
                        Err(AuthenticateError::ConnectionRefused)
                    } else {
                        Err(AuthenticateError::Other(err))
                    }
                }
            }
        })
    }
}

impl AtomicChatLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let this = Self {
            http_client: http_client.clone(),
            state: cx.new(|cx| {
                let subscription = cx.observe_global::<SettingsStore>({
                    let mut settings = AllLanguageModelSettings::get_global(cx).atomic_chat.clone();
                    move |this: &mut State, cx| {
                        let new_settings =
                            AllLanguageModelSettings::get_global(cx).atomic_chat.clone();
                        if settings != new_settings {
                            let credentials_provider = this.credentials_provider.clone();
                            let api_url = Self::api_url(cx).into();
                            this.api_key_state.handle_url_change(
                                api_url,
                                |this| &mut this.api_key_state,
                                credentials_provider,
                                cx,
                            );
                            settings = new_settings;
                            this.restart_fetch_models_task(cx);
                            cx.notify();
                        }
                    }
                });

                State {
                    api_key_state: ApiKeyState::new(
                        Self::api_url(cx).into(),
                        (*API_KEY_ENV_VAR).clone(),
                    ),
                    credentials_provider,
                    http_client,
                    available_models: Default::default(),
                    fetch_model_task: None,
                    _subscription: subscription,
                }
            }),
        };
        this.state
            .update(cx, |state, cx| state.restart_fetch_models_task(cx));
        this
    }

    fn api_url(cx: &App) -> String {
        AllLanguageModelSettings::get_global(cx)
            .atomic_chat
            .api_url
            .clone()
    }

    fn has_custom_url(cx: &App) -> bool {
        Self::api_url(cx) != ATOMIC_CHAT_API_URL
    }
}

impl LanguageModelProviderState for AtomicChatLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for AtomicChatLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiAtomicChat)
    }

    fn default_model(&self, _: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn default_fast_model(&self, _: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models: BTreeMap<String, Model> = BTreeMap::default();

        for model in self.state.read(cx).available_models.iter() {
            models.insert(model.name.clone(), model.clone());
        }

        for model in AllLanguageModelSettings::get_global(cx)
            .atomic_chat
            .available_models
            .iter()
        {
            models.insert(
                model.name.clone(),
                Model {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    supports_tool_calls: model.supports_tool_calls,
                    supports_images: model.supports_images,
                },
            );
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(AtomicChatLanguageModel {
                    id: LanguageModelId::from(model.name.clone()),
                    model,
                    http_client: self.http_client.clone(),
                    request_limiter: RateLimiter::new(4),
                    state: self.state.clone(),
                }) as Arc<dyn LanguageModel>
            })
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
        _window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), _window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }
}

pub struct AtomicChatLanguageModel {
    id: LanguageModelId,
    model: Model,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
    state: Entity<State>,
}

impl AtomicChatLanguageModel {
    fn stream_open_ai_completion(
        &self,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<BoxStream<'static, Result<ResponseStreamEvent>>, LanguageModelCompletionError>,
    > {
        let http_client = self.http_client.clone();
        let (api_key, api_url) = self.state.read_with(cx, |state, cx| {
            let api_url = AtomicChatLanguageModelProvider::api_url(cx);
            (state.api_key_state.key(&api_url), api_url)
        });

        let future = self.request_limiter.stream(async move {
            let api_key_str = api_key.unwrap_or_default();
            let stream = stream_completion(
                http_client.as_ref(),
                "atomic_chat",
                &api_url,
                &api_key_str,
                request,
            )
            .await
            .map_err(LanguageModelCompletionError::from)?;
            Ok(stream)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for AtomicChatLanguageModel {
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
        self.model.supports_tool_calls()
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        self.supports_tools()
            && matches!(
                choice,
                LanguageModelToolChoice::Auto
                    | LanguageModelToolChoice::Any
                    | LanguageModelToolChoice::None
            )
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_split_token_display(&self) -> bool {
        true
    }

    fn telemetry_id(&self) -> String {
        format!("atomic_chat/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
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
        let request = into_open_ai(
            request,
            self.model.name.as_str(),
            self.model.supports_tool_calls(),
            false,
            None,
            None,
            false,
        );
        let completions = self.stream_open_ai_completion(request, cx);
        async move {
            let mapper = OpenAiEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
        }
        .boxed()
    }
}

struct ConfigurationView {
    state: Entity<State>,
    api_key_editor: Entity<InputField>,
    api_url_editor: Entity<InputField>,
}

impl ConfigurationView {
    pub fn new(state: Entity<State>, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| InputField::new(_window, cx, "sk-...").label("API key"));

        let api_url_editor = cx.new(|cx| {
            let input = InputField::new(_window, cx, ATOMIC_CHAT_API_URL).label("API URL");
            input.set_text(&AtomicChatLanguageModelProvider::api_url(cx), _window, cx);
            input
        });

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        Self {
            state,
            api_key_editor,
            api_url_editor,
        }
    }

    fn retry_connection(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let has_api_url = AtomicChatLanguageModelProvider::has_custom_url(cx);
        let has_api_key = self
            .state
            .read_with(cx, |state, _| state.api_key_state.has_key());
        if !has_api_url {
            self.save_api_url(cx);
        }
        if !has_api_key {
            self.save_api_key(&Default::default(), _window, cx);
        }

        self.state.update(cx, |state, cx| {
            state.restart_fetch_models_task(cx);
        });
    }

    fn save_api_key(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
        if api_key.is_empty() {
            return;
        }

        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", _window, cx));

        let state = self.state.clone();
        cx.spawn_in(_window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_api_key(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", _window, cx));

        let state = self.state.clone();
        cx.spawn_in(_window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))
                .await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }

    fn save_api_url(&self, cx: &mut Context<Self>) {
        let api_url = self.api_url_editor.read(cx).text(cx).trim().to_string();
        let current_url = AtomicChatLanguageModelProvider::api_url(cx);
        if !api_url.is_empty() && api_url != current_url {
            self.state
                .update(cx, |state, cx| state.set_api_key(None, cx))
                .detach_and_log_err(cx);

            let fs = <dyn Fs>::global(cx);
            update_settings_file(fs, cx, move |settings, _| {
                settings
                    .language_models
                    .get_or_insert_default()
                    .atomic_chat
                    .get_or_insert_default()
                    .api_url = Some(api_url);
            });
        }
    }

    fn reset_api_url(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.api_url_editor
            .update(cx, |input, cx| input.set_text("", _window, cx));

        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
            .detach_and_log_err(cx);

        let fs = <dyn Fs>::global(cx);
        update_settings_file(fs, cx, |settings, _cx| {
            if let Some(settings) = settings
                .language_models
                .as_mut()
                .and_then(|models| models.atomic_chat.as_mut())
            {
                settings.api_url = Some(ATOMIC_CHAT_API_URL.into());
            }
        });
        cx.notify();
    }

    fn render_api_url_editor(&self, cx: &Context<Self>) -> impl IntoElement {
        let api_url = AtomicChatLanguageModelProvider::api_url(cx);
        let custom_api_url_set = api_url != ATOMIC_CHAT_API_URL;

        if custom_api_url_set {
            h_flex()
                .p_3()
                .justify_between()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().elevated_surface_background)
                .child(
                    h_flex()
                        .gap_2()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(v_flex().gap_1().child(Label::new(api_url))),
                )
                .child(
                    Button::new("reset-atomic-chat-api-url", "Reset API URL")
                        .label_size(LabelSize::Small)
                        .start_icon(Icon::new(IconName::Undo).size(IconSize::Small))
                        .layer(ElevationIndex::ModalSurface)
                        .on_click(
                            cx.listener(|this, _, _window, cx| this.reset_api_url(_window, cx)),
                        ),
                )
                .into_any_element()
        } else {
            v_flex()
                .on_action(cx.listener(|this, _: &menu::Confirm, _window, cx| {
                    this.save_api_url(cx);
                    cx.notify();
                }))
                .gap_2()
                .child(self.api_url_editor.clone())
                .into_any_element()
        }
    }

    fn render_api_key_editor(&self, cx: &Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let env_var_set = state.api_key_state.is_from_env_var();
        let configured_card_label = if env_var_set {
            format!("API key set in {API_KEY_ENV_VAR_NAME} environment variable.")
        } else {
            "API key configured".to_string()
        };

        if !state.api_key_state.has_key() {
            v_flex()
                .on_action(cx.listener(Self::save_api_key))
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(format!(
                        "Optional. Leave empty if your server does not require a key. You can also set {API_KEY_ENV_VAR_NAME}."
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
                .into_any_element()
        } else {
            ConfiguredApiCard::new(configured_card_label)
                .disabled(env_var_set)
                .on_click(cx.listener(|this, _, _window, cx| this.reset_api_key(_window, cx)))
                .when(env_var_set, |this| {
                    this.tooltip_label(format!(
                        "To reset your API key, unset the {API_KEY_ENV_VAR_NAME} environment variable."
                    ))
                })
                .into_any_element()
        }
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_authenticated = self.state.read(cx).is_authenticated();

        v_flex()
            .gap_2()
            .child(
                v_flex()
                    .gap_1()
                    .child(Label::new(
                        "Run local models through Atomic Chat’s OpenAI-compatible API.",
                    ))
                    .child(
                        List::new().child(ListBulletItem::new(
                            "Start Atomic Chat and enable the local API server (default: http://localhost:1337/v1).",
                        )),
                    )
                    .child(Label::new(
                        "Use a custom base URL if the app listens on another host or port.",
                    )),
            )
            .child(self.render_api_url_editor(cx))
            .child(self.render_api_key_editor(cx))
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .gap_2()
                    .child(
                        h_flex().w_full().gap_2().map(|this| {
                            if is_authenticated {
                                this.child(
                                    Button::new("atomic-chat-site", "atomic.chat")
                                        .style(ButtonStyle::Subtle)
                                        .end_icon(
                                            Icon::new(IconName::ArrowUpRight)
                                                .size(IconSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .on_click(move |_, _window, cx| {
                                            cx.open_url(ATOMIC_CHAT_SITE)
                                        })
                                        .into_any_element(),
                                )
                            } else {
                                this.child(
                                    Button::new("download-atomic-chat", "Get Atomic Chat")
                                        .style(ButtonStyle::Subtle)
                                        .end_icon(
                                            Icon::new(IconName::ArrowUpRight)
                                                .size(IconSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .on_click(move |_, _window, cx| {
                                            cx.open_url(ATOMIC_CHAT_SITE)
                                        })
                                        .into_any_element(),
                                )
                            }
                        }),
                    )
                    .map(|this| {
                        if is_authenticated {
                            this.child(
                                ButtonLike::new("atomic-chat-connected")
                                    .disabled(true)
                                    .cursor_style(CursorStyle::Arrow)
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(Icon::new(IconName::Check).color(Color::Success))
                                            .child(Label::new("Connected"))
                                            .into_any_element(),
                                    )
                                    .child(
                                        IconButton::new("refresh-atomic-chat-models", IconName::RotateCcw)
                                            .tooltip(Tooltip::text("Refresh Models"))
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.state.update(cx, |state, _| {
                                                    state.available_models.clear();
                                                });
                                                this.retry_connection(_window, cx);
                                            })),
                                    ),
                            )
                        } else {
                            this.child(
                                Button::new("retry-atomic-chat-models", "Connect")
                                    .start_icon(
                                        Icon::new(IconName::PlayFilled).size(IconSize::XSmall),
                                    )
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.retry_connection(_window, cx)
                                    })),
                            )
                        }
                    }),
            )
    }
}
