use ai_onboarding::YoungAccountBanner;
use anyhow::Result;
use client::{Client, RefreshLlmTokenListener, UserStore, global_llm_token, zed_urls};
use cloud_api_client::LlmApiToken;
use cloud_api_types::OrganizationId;
use cloud_api_types::Plan;
use futures::StreamExt;
use futures::future::BoxFuture;
use gpui::{AnyElement, AnyView, App, AppContext, Context, Entity, Subscription, Task};
use language_model::{
    AuthenticateError, IconOrSvg, LanguageModel, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, ZED_CLOUD_PROVIDER_ID,
    ZED_CLOUD_PROVIDER_NAME,
};
use language_models_cloud::{CloudLlmTokenProvider, CloudModelProvider};
use release_channel::AppVersion;

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
    user_store: Entity<UserStore>,
}

impl CloudLlmTokenProvider for ClientTokenProvider {
    type AuthContext = Option<OrganizationId>;

    fn auth_context(&self, cx: &impl AppContext) -> Self::AuthContext {
        self.user_store.read_with(cx, |user_store, _| {
            user_store
                .current_organization()
                .map(|organization| organization.id.clone())
        })
    }

    fn acquire_token(
        &self,
        organization_id: Self::AuthContext,
    ) -> BoxFuture<'static, Result<String>> {
        let client = self.client.clone();
        let llm_api_token = self.llm_api_token.clone();
        Box::pin(async move {
            client
                .acquire_llm_token(&llm_api_token, organization_id)
                .await
        })
    }

    fn refresh_token(
        &self,
        organization_id: Self::AuthContext,
    ) -> BoxFuture<'static, Result<String>> {
        let client = self.client.clone();
        let llm_api_token = self.llm_api_token.clone();
        Box::pin(async move {
            client
                .refresh_llm_token(&llm_api_token, organization_id)
                .await
        })
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ZedDotDevSettings {
    pub available_models: Vec<AvailableModel>,
}

pub struct CloudLanguageModelProvider {
    state: Entity<State>,
    _maintain_client_status: Task<()>,
}

pub struct State {
    client: Arc<Client>,
    user_store: Entity<UserStore>,
    status: client::Status,
    provider: Entity<CloudModelProvider<ClientTokenProvider>>,
    _user_store_subscription: Subscription,
    _settings_subscription: Subscription,
    _llm_token_subscription: Subscription,
    _provider_subscription: Subscription,
}

impl State {
    fn new(
        client: Arc<Client>,
        user_store: Entity<UserStore>,
        status: client::Status,
        cx: &mut Context<Self>,
    ) -> Self {
        let refresh_llm_token_listener = RefreshLlmTokenListener::global(cx);
        let token_provider = Arc::new(ClientTokenProvider {
            client: client.clone(),
            llm_api_token: global_llm_token(cx),
            user_store: user_store.clone(),
        });

        let provider = cx.new(|cx| {
            CloudModelProvider::new(
                token_provider.clone(),
                client.http_client(),
                Some(AppVersion::global(cx)),
            )
        });

        Self {
            client: client.clone(),
            user_store: user_store.clone(),
            status,
            _provider_subscription: cx.observe(&provider, |_, _, cx| cx.notify()),
            provider,
            _user_store_subscription: cx.subscribe(
                &user_store,
                move |this, _user_store, event, cx| match event {
                    client::user::Event::PrivateUserInfoUpdated => {
                        let status = *client.status().borrow();
                        if status.is_signed_out() {
                            return;
                        }

                        this.refresh_models(cx);
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
                    this.refresh_models(cx);
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

    fn refresh_models(&mut self, cx: &mut Context<Self>) {
        self.provider.update(cx, |provider, cx| {
            provider.refresh_models(cx).detach_and_log_err(cx);
        });
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
            state,
            _maintain_client_status: maintain_client_status,
        }
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
        let provider = state.provider.read(cx);
        let model = provider.default_model()?;
        Some(provider.create_model(model))
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        let provider = state.provider.read(cx);
        let model = provider.default_fast_model()?;
        Some(provider.create_model(model))
    }

    fn recommended_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        let provider = state.provider.read(cx);
        provider
            .recommended_models()
            .iter()
            .map(|model| provider.create_model(model))
            .collect()
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        let provider = state.provider.read(cx);
        provider
            .models()
            .iter()
            .map(|model| provider.create_model(model))
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
