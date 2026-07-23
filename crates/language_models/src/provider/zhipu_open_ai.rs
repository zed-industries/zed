use anyhow::Result;
use collections::IndexMap;
use credentials_provider::CredentialsProvider;
use fs::Fs;

use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{App, AppContext, AsyncApp, Context, Entity, SharedString, Task, Window};
use http_client::{CustomHeaders, HttpClient};
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, InlineDescription, LanguageModel,
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelEffortLevel,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, ProviderSettingsView, RateLimiter, SubPageProviderSettings, env_var,
};
use menu;
use open_ai::{ResponseStreamEvent, stream_completion};
pub use settings::ZhipuAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore, update_settings_file};
use std::sync::{Arc, LazyLock};
use util::ResultExt;

use crate::provider::open_ai::{OpenAiEventMapper, into_open_ai};

use ui::{
    Button, ButtonStyle as ZedButtonStyle, ElevationIndex, FluentBuilder, Icon, IconName, IconSize,
    Label, LabelSize, TintColor, Tooltip, prelude::*,
};
use ui_input::InputField;
use zhipu::openai_api_url;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("zhipu");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Zhipu AI");

const API_KEY_ENV_VAR_NAME: &str = "ZHIPU_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ZhipuOpenAiSettings {
    pub api_url: String,
    pub region: zhipu::Region,
    pub billing: zhipu::BillingType,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
}

pub struct ZhipuOpenAiLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = ZhipuOpenAiLanguageModelProvider::api_url(cx);
        self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        )
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = ZhipuOpenAiLanguageModelProvider::api_url(cx);
        self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        )
    }
}

impl ZhipuOpenAiLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let credentials_provider = this.credentials_provider.clone();
                let api_url = Self::api_url(cx);
                this.api_key_state.handle_url_change(
                    api_url,
                    |this| &mut this.api_key_state,
                    credentials_provider,
                    cx,
                );
                cx.notify();
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
                credentials_provider,
            }
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: zhipu::ZhipuModel) -> Arc<dyn LanguageModel> {
        Arc::new(ZhipuOpenAiLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    fn settings(cx: &App) -> &ZhipuOpenAiSettings {
        &crate::AllLanguageModelSettings::get_global(cx).zhipu
    }

    fn api_url(cx: &App) -> SharedString {
        let settings = Self::settings(cx);
        let api_url = &settings.api_url;
        if api_url.is_empty() {
            openai_api_url(settings.region, settings.billing).into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

impl LanguageModelProviderState for ZhipuOpenAiLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for ZhipuOpenAiLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiZhipu)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(zhipu::ZhipuModel::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(zhipu::ZhipuModel::default_fast()))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = IndexMap::default();

        models.insert("glm-5.2", zhipu::ZhipuModel::Glm5_2);
        models.insert("glm-5.1", zhipu::ZhipuModel::Glm5_1);
        models.insert("glm-5-turbo", zhipu::ZhipuModel::Glm5Turbo);

        for available_model in &Self::settings(cx).available_models {
            models.insert(
                &available_model.name,
                zhipu::ZhipuModel::Custom {
                    name: available_model.name.clone(),
                    display_name: available_model.display_name.clone(),
                    max_tokens: available_model.max_tokens,
                    max_output_tokens: available_model.max_output_tokens,
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

    fn settings_view(&self, _cx: &mut App) -> Option<ProviderSettingsView> {
        let state = self.state.clone();
        Some(ProviderSettingsView::SubPage(
            SubPageProviderSettings::new(move |window, cx| {
                cx.new(|cx| ConfigurationView::new(state.clone(), window, cx))
                    .into()
            })
            .description(InlineDescription::Text(
                "Configure region and API key for Zhipu AI. \
                 For international users use api.z.ai, for China users use open.bigmodel.cn."
                    .into(),
            )),
        ))
    }

    fn set_api_key(&self, api_key: Option<String>, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(api_key, cx))
    }
}

pub struct ZhipuOpenAiLanguageModel {
    id: LanguageModelId,
    model: zhipu::ZhipuModel,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl ZhipuOpenAiLanguageModel {
    fn stream_completion(
        &self,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<BoxStream<'static, Result<ResponseStreamEvent>>, LanguageModelCompletionError>,
    > {
        let http_client = self.http_client.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = ZhipuOpenAiLanguageModelProvider::api_url(cx);
            let extra_headers = ZhipuOpenAiLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let provider_name = PROVIDER_NAME.0.to_string();
            let request = stream_completion(
                http_client.as_ref(),
                &provider_name,
                &api_url,
                &api_key,
                request,
                &extra_headers,
            );
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for ZhipuOpenAiLanguageModel {
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

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_thinking(&self) -> bool {
        self.model.supports_thinking()
    }

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        if !self.supports_thinking() {
            return Vec::new();
        }

        vec![
            LanguageModelEffortLevel {
                name: "Medium".into(),
                value: "medium".into(),
                is_default: true,
            },
            LanguageModelEffortLevel {
                name: "High".into(),
                value: "high".into(),
                is_default: false,
            },
        ]
    }

    fn supports_tool_choice(&self, _choice: LanguageModelToolChoice) -> bool {
        true
    }

    fn supports_images(&self) -> bool {
        false
    }

    fn telemetry_id(&self) -> String {
        format!("zhipu/{}", self.model.id())
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
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
            LanguageModelCompletionError,
        >,
    > {
        let request = match into_open_ai(
            request,
            self.model.id(),
            false,
            false,
            self.max_output_tokens(),
            open_ai::completion::ChatCompletionMaxTokensParameter::MaxCompletionTokens,
            None,
            false,
        ) {
            Ok(request) => request,
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };
        let completions = self.stream_completion(request, cx);
        async move {
            let mapper = OpenAiEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
        }
        .boxed()
    }
}

// ── ConfigurationView ───────────────────────────────────────────────────────

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    pub fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| InputField::new(window, cx, "Enter your Zhipu API key"));

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn_in(window, {
            let state = state.clone();
            async move |this, cx| {
                let task = state.update(cx, |state, cx| state.authenticate(cx));
                match task.await {
                    Ok(()) | Err(AuthenticateError::CredentialsNotFound) => {}
                    Err(error) => {
                        log::error!("Failed to load Zhipu API credentials: {error}");
                    }
                }
                this.update(cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            api_key_editor,
            state,
            load_credentials_task,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
        if api_key.is_empty() {
            return;
        }

        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))
                .await
        })
        .detach_and_log_err(cx);
    }
}

impl gpui::prelude::Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        if self.load_credentials_task.is_some() {
            return div()
                .child(Label::new("Loading credentials…"))
                .into_any_element();
        }

        let settings = ZhipuOpenAiLanguageModelProvider::settings(cx);
        let api_url = ZhipuOpenAiLanguageModelProvider::api_url(cx);
        let current_region = settings.region;
        let current_billing = settings.billing;
        let is_authenticated = self.state.read(cx).is_authenticated();
        let env_var_set = self.state.read(cx).api_key_state.is_from_env_var();
        let env_var_name = self.state.read(cx).api_key_state.env_var_name().clone();

        let on_region_click = move |region: zhipu::Region, _window: &mut Window, cx: &mut App| {
            let fs = <dyn Fs>::global(cx);
            update_settings_file(fs, cx, move |settings, _| {
                let zhipu = settings
                    .language_models
                    .get_or_insert_default()
                    .zhipu
                    .get_or_insert_default();
                zhipu.region = Some(match region {
                    zhipu::Region::International => settings::ZhipuRegion::International,
                    zhipu::Region::China => settings::ZhipuRegion::China,
                });
            });
        };

        let on_billing_click =
            move |billing: zhipu::BillingType, _window: &mut Window, cx: &mut App| {
                let fs = <dyn Fs>::global(cx);
                update_settings_file(fs, cx, move |settings, _| {
                    let zhipu = settings
                        .language_models
                        .get_or_insert_default()
                        .zhipu
                        .get_or_insert_default();
                    zhipu.billing = Some(match billing {
                        zhipu::BillingType::Standard => settings::ZhipuBillingType::Standard,
                        zhipu::BillingType::CodingPlan => settings::ZhipuBillingType::CodingPlan,
                    });
                });
            };

        let is_international = current_region == zhipu::Region::International;
        let is_china = current_region == zhipu::Region::China;
        let is_standard = current_billing == zhipu::BillingType::Standard;
        let is_coding_plan = current_billing == zhipu::BillingType::CodingPlan;

        let api_key_section: gpui::AnyElement = if !is_authenticated {
            v_flex()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new(
                    "To use Zhipu AI models, you need to add an API key.",
                ))
                .child(
                    div()
                        .pt(DynamicSpacing::Base04.rems(cx))
                        .child(self.api_key_editor.clone()),
                )
                .child(
                    Label::new(format!(
                        "You can also set the {env_var_name} environment variable and restart Zed.",
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
                .into_any()
        } else {
            h_flex()
                .mt_1()
                .p_1()
                .justify_between()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().background)
                .child(
                    h_flex()
                        .flex_1()
                        .min_w_0()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(
                            div()
                                .w_full()
                                .overflow_x_hidden()
                                .text_ellipsis()
                                .child(Label::new(if env_var_set {
                                    format!("API key set in {env_var_name} environment variable")
                                } else {
                                    "API key configured".to_string()
                                })),
                        ),
                )
                .child(
                    h_flex().flex_shrink_0().child(
                        Button::new("reset-api-key", "Reset Key")
                            .label_size(LabelSize::Small)
                            .start_icon(Icon::new(IconName::Undo).size(IconSize::Small))
                            .layer(ElevationIndex::ModalSurface)
                            .when(env_var_set, |this| {
                                this.tooltip(Tooltip::text(format!(
                                    "To reset your API key, unset the {env_var_name} environment variable.",
                                )))
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.reset_api_key(window, cx)
                            })),
                    ),
                )
                .into_any()
        };

        v_flex()
            .size_full()
            .gap_4()
            .child(
                v_flex()
                    .gap_1()
                    .child(Label::new("Region").size(LabelSize::Small))
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Button::new("region-international", "Z.AI (International)")
                                    .toggle_state(is_international)
                                    .selected_style(ZedButtonStyle::Tinted(TintColor::Accent))
                                    .on_click(move |_, window, cx| {
                                        on_region_click(zhipu::Region::International, window, cx);
                                    }),
                            )
                            .child(
                                Button::new("region-china", "Zhipu (China)")
                                    .toggle_state(is_china)
                                    .selected_style(ZedButtonStyle::Tinted(TintColor::Accent))
                                    .on_click(move |_, window, cx| {
                                        on_region_click(zhipu::Region::China, window, cx);
                                    }),
                            ),
                    ),
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(Label::new("Billing").size(LabelSize::Small))
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Button::new("billing-coding-plan", "Coding Plan")
                                    .toggle_state(is_coding_plan)
                                    .selected_style(ZedButtonStyle::Tinted(TintColor::Accent))
                                    .on_click(move |_, window, cx| {
                                        on_billing_click(
                                            zhipu::BillingType::CodingPlan,
                                            window,
                                            cx,
                                        );
                                    }),
                            )
                            .child(
                                Button::new("billing-payg", "Pay-as-you-go")
                                    .toggle_state(is_standard)
                                    .selected_style(ZedButtonStyle::Tinted(TintColor::Accent))
                                    .on_click(move |_, window, cx| {
                                        on_billing_click(zhipu::BillingType::Standard, window, cx);
                                    }),
                            ),
                    ),
            )
            .child(
                Label::new(format!("API URL: {api_url}"))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(api_key_section)
            .into_any_element()
    }
}
