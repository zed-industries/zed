use ai_onboarding::YoungAccountBanner;
use anyhow::{Context as _, Result};
use client::{Client, RefreshLlmTokenListener, UserStore, global_llm_token, zed_urls};
use cloud_api_types::{OrganizationId, Plan};
use cloud_llm_client::{CLIENT_SUPPORTS_X_AI_HEADER_NAME, ListModelsResponse};
use futures::StreamExt;
use gpui::{AnyElement, AnyView, App, Context, Entity, Subscription, Task};
use http_client::{AsyncBody, HttpClient, Method};
use language_model::{
    AuthenticateError, IconOrSvg, LanguageModel, LanguageModelId, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState, LlmApiToken,
    RateLimiter, ZED_CLOUD_PROVIDER_ID, ZED_CLOUD_PROVIDER_NAME,
};
use language_models_cloud::{CloudLanguageModel, CloudLlmTokenProvider};
use smol::io::AsyncReadExt;

use settings::SettingsStore;
pub use settings::ZedDotDevAvailableModel as AvailableModel;
pub use settings::ZedDotDevAvailableProvider as AvailableProvider;
use std::sync::Arc;
use ui::{TintColor, prelude::*};

const PROVIDER_ID: LanguageModelProviderId = ZED_CLOUD_PROVIDER_ID;
const PROVIDER_NAME: LanguageModelProviderName = ZED_CLOUD_PROVIDER_NAME;

struct ClientTokenProvider {
    client: Arc<Client>,
    llm_api_token: LlmApiToken,
    organization_id: Option<cloud_api_types::OrganizationId>,
}

impl CloudLlmTokenProvider for ClientTokenProvider {
    fn acquire_token(&self) -> futures::future::BoxFuture<'_, Result<String>> {
        let client = self.client.clone();
        let llm_api_token = self.llm_api_token.clone();
        let organization_id = self.organization_id.clone();
        Box::pin(async move {
            Ok(client
                .acquire_llm_token(&llm_api_token, organization_id)
                .await?)
        })
    }

    fn refresh_token(&self) -> futures::future::BoxFuture<'_, Result<String>> {
        let client = self.client.clone();
        let llm_api_token = self.llm_api_token.clone();
        let organization_id = self.organization_id.clone();
        Box::pin(async move {
            Ok(client
                .refresh_llm_token(&llm_api_token, organization_id)
                .await?)
        })
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ZedDotDevSettings {
    pub available_models: Vec<AvailableModel>,
}

pub struct CloudLanguageModelProvider {
    client: Arc<Client>,
    state: Entity<State>,
    _maintain_client_status: Task<()>,
}

pub struct State {
    client: Arc<Client>,
    llm_api_token: LlmApiToken,
    user_store: Entity<UserStore>,
    status: client::Status,
    models: Vec<Arc<cloud_llm_client::LanguageModel>>,
    default_model: Option<Arc<cloud_llm_client::LanguageModel>>,
    default_fast_model: Option<Arc<cloud_llm_client::LanguageModel>>,
    recommended_models: Vec<Arc<cloud_llm_client::LanguageModel>>,
    _user_store_subscription: Subscription,
    _settings_subscription: Subscription,
    _llm_token_subscription: Subscription,
}

impl State {
    fn new(
        client: Arc<Client>,
        user_store: Entity<UserStore>,
        status: client::Status,
        cx: &mut Context<Self>,
    ) -> Self {
        let refresh_llm_token_listener = RefreshLlmTokenListener::global(cx);
        let llm_api_token = global_llm_token(cx);
        Self {
            client: client.clone(),
            llm_api_token,
            user_store: user_store.clone(),
            status,
            models: Vec::new(),
            default_model: None,
            default_fast_model: None,
            recommended_models: Vec::new(),
            _user_store_subscription: cx.subscribe(
                &user_store,
                move |this, _user_store, event, cx| match event {
                    client::user::Event::PrivateUserInfoUpdated => {
                        let status = *client.status().borrow();
                        if status.is_signed_out() {
                            return;
                        }

                        let client = this.client.clone();
                        let llm_api_token = this.llm_api_token.clone();
                        let organization_id = this
                            .user_store
                            .read(cx)
                            .current_organization()
                            .map(|organization| organization.id.clone());
                        cx.spawn(async move |this, cx| {
                            let response =
                                Self::fetch_models(client, llm_api_token, organization_id).await?;
                            this.update(cx, |this, cx| this.update_models(response, cx))
                        })
                        .detach_and_log_err(cx);
                    }
                    _ => {}
                },
            ),
            _settings_subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            }),
            _llm_token_subscription: cx.subscribe(
                &refresh_llm_token_listener,
                move |this, _listener, _event, cx| {
                    let client = this.client.clone();
                    let llm_api_token = this.llm_api_token.clone();
                    let organization_id = this
                        .user_store
                        .read(cx)
                        .current_organization()
                        .map(|organization| organization.id.clone());
                    cx.spawn(async move |this, cx| {
                        let response =
                            Self::fetch_models(client, llm_api_token, organization_id).await?;
                        this.update(cx, |this, cx| {
                            this.update_models(response, cx);
                        })
                    })
                    .detach_and_log_err(cx);
                },
            ),
        }
    }

    fn is_signed_out(&self, cx: &App) -> bool {
        self.user_store.read(cx).current_user().is_none()
    }

    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.spawn(async move |state, cx| {
            client.sign_in_with_optional_connect(true, cx).await?;
            state.update(cx, |_, cx| cx.notify())
        })
    }

    fn update_models(&mut self, response: ListModelsResponse, cx: &mut Context<Self>) {
        let mut models = Vec::new();

        for model in response.models {
            models.push(Arc::new(model.clone()));
        }

        self.default_model = models
            .iter()
            .find(|model| {
                response
                    .default_model
                    .as_ref()
                    .is_some_and(|default_model_id| &model.id == default_model_id)
            })
            .cloned();
        self.default_fast_model = models
            .iter()
            .find(|model| {
                response
                    .default_fast_model
                    .as_ref()
                    .is_some_and(|default_fast_model_id| &model.id == default_fast_model_id)
            })
            .cloned();
        self.recommended_models = response
            .recommended_models
            .iter()
            .filter_map(|id| models.iter().find(|model| &model.id == id))
            .cloned()
            .collect();
        self.models = models;
        cx.notify();
    }

    async fn fetch_models(
        client: Arc<Client>,
        llm_api_token: LlmApiToken,
        organization_id: Option<OrganizationId>,
    ) -> Result<ListModelsResponse> {
        let http_client = &client.http_client();
        let token = client
            .acquire_llm_token(&llm_api_token, organization_id)
            .await?;

        let request = http_client::Request::builder()
            .method(Method::GET)
            .header(CLIENT_SUPPORTS_X_AI_HEADER_NAME, "true")
            .uri(http_client.build_zed_llm_url("/models", &[])?.as_ref())
            .header("Authorization", format!("Bearer {token}"))
            .body(AsyncBody::empty())?;
        let mut response = http_client
            .send(request)
            .await
            .context("failed to send list models request")?;

        if response.status().is_success() {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            Ok(serde_json::from_str(&body)?)
        } else {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            anyhow::bail!(
                "error listing models.\nStatus: {:?}\nBody: {body}",
                response.status(),
            );
        }
    }
}

impl CloudLanguageModelProvider {
    pub fn new(user_store: Entity<UserStore>, client: Arc<Client>, cx: &mut App) -> Self {
        let mut status_rx = client.status();
        let status = *status_rx.borrow();

        let state = cx.new(|cx| State::new(client.clone(), user_store.clone(), status, cx));

        let state_ref = state.downgrade();
        let maintain_client_status = cx.spawn(async move |cx| {
            while let Some(status) = status_rx.next().await {
                if let Some(this) = state_ref.upgrade() {
                    _ = this.update(cx, |this, cx| {
                        if this.status != status {
                            this.status = status;
                            cx.notify();
                        }
                    });
                } else {
                    break;
                }
            }
        });

        Self {
            client,
            state,
            _maintain_client_status: maintain_client_status,
        }
    }

    fn create_language_model(
        &self,
        model: Arc<cloud_llm_client::LanguageModel>,
        llm_api_token: LlmApiToken,
        organization_id: Option<cloud_api_types::OrganizationId>,
    ) -> Arc<dyn LanguageModel> {
        let token_provider = Arc::new(ClientTokenProvider {
            client: self.client.clone(),
            llm_api_token,
            organization_id,
        });
        Arc::new(CloudLanguageModel {
            id: LanguageModelId::from(model.id.0.to_string()),
            model,
            token_provider,
            http_client: self.client.http_client(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for CloudLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for CloudLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiZed)
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        let default_model = state.default_model.clone()?;
        let llm_api_token = state.llm_api_token.clone();
        let organization_id = state
            .user_store
            .read(cx)
            .current_organization()
            .map(|organization| organization.id.clone());
        Some(self.create_language_model(default_model, llm_api_token, organization_id))
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        let default_fast_model = state.default_fast_model.clone()?;
        let llm_api_token = state.llm_api_token.clone();
        let organization_id = state
            .user_store
            .read(cx)
            .current_organization()
            .map(|organization| organization.id.clone());
        Some(self.create_language_model(default_fast_model, llm_api_token, organization_id))
    }

    fn recommended_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        let llm_api_token = state.llm_api_token.clone();
        let organization_id = state
            .user_store
            .read(cx)
            .current_organization()
            .map(|organization| organization.id.clone());
        state
            .recommended_models
            .iter()
            .cloned()
            .map(|model| {
                self.create_language_model(model, llm_api_token.clone(), organization_id.clone())
            })
            .collect()
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        let llm_api_token = state.llm_api_token.clone();
        let organization_id = state
            .user_store
            .read(cx)
            .current_organization()
            .map(|organization| organization.id.clone());
        state
            .models
            .iter()
            .cloned()
            .map(|model| {
                self.create_language_model(model, llm_api_token.clone(), organization_id.clone())
            })
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        let state = self.state.read(cx);
        !state.is_signed_out(cx)
    }

    fn authenticate(&self, _cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        Task::ready(Ok(()))
    }

    fn configuration_view(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        _: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|_| ConfigurationView::new(self.state.clone()))
            .into()
    }

    fn reset_credentials(&self, _cx: &mut App) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }
}

#[derive(IntoElement, RegisterComponent)]
struct ZedAiConfiguration {
    is_connected: bool,
    plan: Option<Plan>,
    eligible_for_trial: bool,
    account_too_young: bool,
    sign_in_callback: Arc<dyn Fn(&mut Window, &mut App) + Send + Sync>,
}

impl RenderOnce for ZedAiConfiguration {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (subscription_text, has_paid_plan) = match self.plan {
            Some(Plan::ZedPro) => (
                "You have access to Zed's hosted models through your Pro subscription.",
                true,
            ),
            Some(Plan::ZedProTrial) => (
                "You have access to Zed's hosted models through your Pro trial.",
                false,
            ),
            Some(Plan::ZedStudent) => (
                "You have access to Zed's hosted models through your Student subscription.",
                true,
            ),
            Some(Plan::ZedBusiness) => (
                "You have access to Zed's hosted models through your Organization.",
                true,
            ),
            Some(Plan::ZedFree) | None => (
                if self.eligible_for_trial {
                    "Subscribe for access to Zed's hosted models. Start with a 14 day free trial."
                } else {
                    "Subscribe for access to Zed's hosted models."
                },
                false,
            ),
        };

        let manage_subscription_buttons = if has_paid_plan {
            Button::new("manage_settings", "Manage Subscription")
                .full_width()
                .label_size(LabelSize::Small)
                .style(ButtonStyle::Tinted(TintColor::Accent))
                .on_click(|_, _, cx| cx.open_url(&zed_urls::account_url(cx)))
                .into_any_element()
        } else if self.plan.is_none() || self.eligible_for_trial {
            Button::new("start_trial", "Start 14-day Free Pro Trial")
                .full_width()
                .style(ui::ButtonStyle::Tinted(ui::TintColor::Accent))
                .on_click(|_, _, cx| cx.open_url(&zed_urls::start_trial_url(cx)))
                .into_any_element()
        } else {
            Button::new("upgrade", "Upgrade to Pro")
                .full_width()
                .style(ui::ButtonStyle::Tinted(ui::TintColor::Accent))
                .on_click(|_, _, cx| cx.open_url(&zed_urls::upgrade_to_zed_pro_url(cx)))
                .into_any_element()
        };

        if !self.is_connected {
            return v_flex()
                .gap_2()
                .child(Label::new("Sign in to have access to Zed's complete agentic experience with hosted models."))
                .child(
                    Button::new("sign_in", "Sign In to use Zed AI")
                        .start_icon(Icon::new(IconName::Github).size(IconSize::Small).color(Color::Muted))
                        .full_width()
                        .on_click({
                            let callback = self.sign_in_callback.clone();
                            move |_, window, cx| (callback)(window, cx)
                        }),
                );
        }

        v_flex().gap_2().w_full().map(|this| {
            if self.account_too_young {
                this.child(YoungAccountBanner).child(
                    Button::new("upgrade", "Upgrade to Pro")
                        .style(ui::ButtonStyle::Tinted(ui::TintColor::Accent))
                        .full_width()
                        .on_click(|_, _, cx| cx.open_url(&zed_urls::upgrade_to_zed_pro_url(cx))),
                )
            } else {
                this.text_sm()
                    .child(subscription_text)
                    .child(manage_subscription_buttons)
            }
        })
    }
}

struct ConfigurationView {
    state: Entity<State>,
    sign_in_callback: Arc<dyn Fn(&mut Window, &mut App) + Send + Sync>,
}

impl ConfigurationView {
    fn new(state: Entity<State>) -> Self {
        let sign_in_callback = Arc::new({
            let state = state.clone();
            move |_window: &mut Window, cx: &mut App| {
                state.update(cx, |state, cx| {
                    state.authenticate(cx).detach_and_log_err(cx);
                });
            }
        });

        Self {
            state,
            sign_in_callback,
        }
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let user_store = state.user_store.read(cx);

        ZedAiConfiguration {
            is_connected: !state.is_signed_out(cx),
            plan: user_store.plan(),
            eligible_for_trial: user_store.trial_started_at().is_none(),
            account_too_young: user_store.account_too_young(),
            sign_in_callback: self.sign_in_callback.clone(),
        }
    }
}

impl Component for ZedAiConfiguration {
    fn name() -> &'static str {
        "AI Configuration Content"
    }

    fn sort_name() -> &'static str {
        "AI Configuration Content"
    }

    fn scope() -> ComponentScope {
        ComponentScope::Onboarding
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        fn configuration(
            is_connected: bool,
            plan: Option<Plan>,
            eligible_for_trial: bool,
            account_too_young: bool,
        ) -> AnyElement {
            ZedAiConfiguration {
                is_connected,
                plan,
                eligible_for_trial,
                account_too_young,
                sign_in_callback: Arc::new(|_, _| {}),
            }
            .into_any_element()
        }

        Some(
            v_flex()
                .p_4()
                .gap_4()
                .children(vec![
                    single_example("Not connected", configuration(false, None, false, false)),
                    single_example(
                        "Accept Terms of Service",
                        configuration(true, None, true, false),
                    ),
                    single_example(
                        "No Plan - Not eligible for trial",
                        configuration(true, None, false, false),
                    ),
                    single_example(
                        "No Plan - Eligible for trial",
                        configuration(true, None, true, false),
                    ),
                    single_example(
                        "Free Plan",
                        configuration(true, Some(Plan::ZedFree), true, false),
                    ),
                    single_example(
                        "Zed Pro Trial Plan",
                        configuration(true, Some(Plan::ZedProTrial), true, false),
                    ),
                    single_example(
                        "Zed Pro Plan",
                        configuration(true, Some(Plan::ZedPro), true, false),
                    ),
                ])
                .into_any_element(),
        )
    }
}
