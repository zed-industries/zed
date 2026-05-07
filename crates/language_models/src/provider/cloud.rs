use ai_onboarding::YoungAccountBanner;
use anyhow::Result;
use client::{Client, RefreshLlmTokenListener, UserStore, global_llm_token, zed_urls};
use cloud_api_client::LlmApiToken;
use cloud_api_types::OrganizationId;
use cloud_api_types::Plan;
use futures::FutureExt;
use futures::StreamExt;
use futures::future::BoxFuture;
use gpui::{AnyElement, AnyView, App, AppContext, Context, Entity, Subscription, Task, TaskExt};
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

    fn sign_in(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let client = self.client.clone();
        let mut current_user = self.user_store.read(cx).watch_current_user();
        cx.spawn(async move |state, cx| {
            client.sign_in_with_optional_connect(true, cx).await?;
            while current_user.borrow().is_none() {
                current_user.next().await;
            }
            state.update(cx, |_, cx| {
                cx.notify();
            })
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

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated(cx) {
            return Task::ready(Ok(()));
        }
        let mut status = self.state.read(cx).client.status();
        let mut current_user = self.state.read(cx).user_store.read(cx).watch_current_user();
        if !status.borrow().is_signing_in() {
            return Task::ready(Ok(()));
        }
        cx.background_spawn(async move {
            while status.borrow().is_signing_in() {
                status.next().await;
            }
            while current_user.borrow().is_none() {
                let current_status = *status.borrow();
                if !matches!(
                    current_status,
                    client::Status::Authenticated
                        | client::Status::Reauthenticated
                        | client::Status::Connected { .. }
                ) {
                    return Err(AuthenticateError::Other(anyhow::anyhow!(
                        "sign-in did not complete: {current_status:?}"
                    )));
                }
                futures::select_biased! {
                    _ = current_user.next().fuse() => {},
                    _ = status.next().fuse() => {},
                }
            }
            Ok(())
        })
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
    is_zed_model_provider_enabled: bool,
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
                if self.is_zed_model_provider_enabled {
                    "You have access to Zed's hosted models through your organization."
                } else {
                    "Zed's hosted models are disabled by your organization's configuration."
                },
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
                    state.sign_in(cx).detach_and_log_err(cx);
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

        let is_zed_model_provider_enabled = user_store
            .current_organization_configuration()
            .map_or(true, |config| config.is_zed_model_provider_enabled);

        ZedAiConfiguration {
            is_connected: !state.is_signed_out(cx),
            plan: user_store.plan(),
            is_zed_model_provider_enabled,
            eligible_for_trial: user_store.trial_started_at().is_none(),
            account_too_young: user_store.account_too_young(),
            sign_in_callback: self.sign_in_callback.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::{Credentials, test::make_get_authenticated_user_response};
    use clock::FakeSystemClock;
    use feature_flags::FeatureFlagAppExt as _;
    use gpui::TestAppContext;
    use http_client::{FakeHttpClient, Method, Response};
    use std::sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    };

    const TEST_USER_ID: u64 = 42;

    fn init_test(cx: &mut App) -> (Arc<Client>, Entity<UserStore>, CloudLanguageModelProvider) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        cx.set_global(db::AppDatabase::test_new());
        let app_version = AppVersion::global(cx);
        release_channel::init_test(app_version, release_channel::ReleaseChannel::Dev, cx);
        gpui_tokio::init(cx);
        cx.update_flags(false, Vec::new());

        let client = Client::new(
            Arc::new(FakeSystemClock::new()),
            FakeHttpClient::with_404_response(),
            cx,
        );
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        RefreshLlmTokenListener::register(client.clone(), user_store.clone(), cx);
        let provider = CloudLanguageModelProvider::new(user_store.clone(), client.clone(), cx);

        (client, user_store, provider)
    }

    fn override_authenticate(
        client: &Arc<Client>,
        authenticate_rx: futures::channel::oneshot::Receiver<anyhow::Result<Credentials>>,
    ) {
        let authenticate_rx = Arc::new(Mutex::new(Some(authenticate_rx)));
        client.override_authenticate(move |cx| {
            let authenticate_rx = authenticate_rx.clone();
            cx.background_spawn(async move {
                let authenticate_rx = authenticate_rx
                    .lock()
                    .expect("authenticate receiver lock poisoned")
                    .take()
                    .expect("authenticate receiver already used");
                authenticate_rx.await?
            })
        });
    }

    fn respond_to_authenticated_user_after(
        client: &Arc<Client>,
        authenticated_user_rx: futures::channel::oneshot::Receiver<()>,
    ) {
        let authenticated_user_rx = Arc::new(Mutex::new(Some(authenticated_user_rx)));
        client
            .http_client()
            .as_fake()
            .replace_handler(move |old_handler, request| {
                let authenticated_user_rx = authenticated_user_rx.clone();
                async move {
                    if request.method() == Method::GET && request.uri().path() == "/client/users/me"
                    {
                        let authenticated_user_rx = authenticated_user_rx
                            .lock()
                            .expect("authenticated user receiver lock poisoned")
                            .take();
                        if let Some(authenticated_user_rx) = authenticated_user_rx {
                            authenticated_user_rx.await.ok();
                        }

                        return Ok(Response::builder()
                            .status(200)
                            .body(
                                serde_json::to_string(&make_get_authenticated_user_response(
                                    TEST_USER_ID as i32,
                                    format!("user-{TEST_USER_ID}"),
                                ))
                                .expect("failed to serialize authenticated user response")
                                .into(),
                            )
                            .expect("failed to build authenticated user response"));
                    }

                    old_handler(request).await
                }
            });
    }

    async fn sign_in_until_authenticating(
        client: Arc<Client>,
        cx: &mut TestAppContext,
    ) -> Task<anyhow::Result<Credentials>> {
        let mut status = client.status();
        let sign_in_task = cx.update(|cx| {
            cx.spawn({
                let client = client.clone();
                async move |cx| client.sign_in(false, cx).await
            })
        });

        while !status.borrow().is_signing_in() {
            status.next().await;
        }

        sign_in_task
    }

    #[gpui::test]
    async fn provider_authenticate_does_not_start_sign_in_when_signed_out(cx: &mut TestAppContext) {
        let (client, _user_store, provider) = cx.update(init_test);
        let authenticate_calls = Arc::new(AtomicUsize::new(0));
        client.override_authenticate({
            let authenticate_calls = authenticate_calls.clone();
            move |_| {
                authenticate_calls.fetch_add(1, Ordering::SeqCst);
                Task::ready(Err(anyhow::anyhow!(
                    "provider authenticate should not start sign-in"
                )))
            }
        });

        assert!(!cx.read(|cx| provider.is_authenticated(cx)));
        assert!(matches!(
            *client.status().borrow(),
            client::Status::SignedOut
        ));

        cx.update(|cx| provider.authenticate(cx))
            .now_or_never()
            .expect("authenticate should return immediately when signed out")
            .expect("authenticate should not fail when no sign-in is in progress");
        cx.executor().run_until_parked();

        assert_eq!(authenticate_calls.load(Ordering::SeqCst), 0);
        assert!(matches!(
            *client.status().borrow(),
            client::Status::SignedOut
        ));
        assert!(!cx.read(|cx| provider.is_authenticated(cx)));
    }

    #[gpui::test]
    async fn provider_authenticate_waits_for_current_user(cx: &mut TestAppContext) {
        let (client, _user_store, provider) = cx.update(init_test);
        let (authenticate_tx, authenticate_rx) = futures::channel::oneshot::channel();
        let (authenticated_user_tx, authenticated_user_rx) = futures::channel::oneshot::channel();
        override_authenticate(&client, authenticate_rx);
        respond_to_authenticated_user_after(&client, authenticated_user_rx);

        let sign_in_task = sign_in_until_authenticating(client.clone(), cx).await;
        let authenticate_task = cx.update(|cx| provider.authenticate(cx));
        authenticate_tx
            .send(Ok(Credentials {
                user_id: TEST_USER_ID,
                access_token: "token".to_string(),
            }))
            .expect("authenticate receiver dropped");

        cx.executor().run_until_parked();
        assert!(!cx.read(|cx| provider.is_authenticated(cx)));

        authenticated_user_tx
            .send(())
            .expect("authenticated user receiver dropped");
        sign_in_task
            .await
            .expect("sign-in should complete after user response");
        authenticate_task
            .await
            .expect("provider authentication should complete after current user is populated");
        assert!(cx.read(|cx| provider.is_authenticated(cx)));

        cx.update(|cx| provider.authenticate(cx))
            .now_or_never()
            .expect("already-authenticated provider should authenticate immediately")
            .unwrap();
    }

    #[gpui::test]
    async fn provider_authenticate_returns_error_when_sign_in_fails(cx: &mut TestAppContext) {
        let (client, _user_store, provider) = cx.update(init_test);
        let (authenticate_tx, authenticate_rx) = futures::channel::oneshot::channel();
        override_authenticate(&client, authenticate_rx);

        let sign_in_task = sign_in_until_authenticating(client.clone(), cx).await;
        let authenticate_task = cx.update(|cx| provider.authenticate(cx));
        authenticate_tx
            .send(Err(anyhow::anyhow!("test authentication failed")))
            .expect("authenticate receiver dropped");

        sign_in_task
            .await
            .expect_err("sign-in should report authentication failure");
        let error = authenticate_task
            .await
            .expect_err("provider authentication should fail when sign-in fails");
        assert!(error.to_string().contains("AuthenticationError"));
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
        struct PreviewConfiguration {
            plan: Option<Plan>,
            is_connected: bool,
            is_zed_model_provider_enabled: bool,
            eligible_for_trial: bool,
        }

        let configuration = |config: PreviewConfiguration| -> AnyElement {
            ZedAiConfiguration {
                is_connected: config.is_connected,
                plan: config.plan,
                is_zed_model_provider_enabled: config.is_zed_model_provider_enabled,
                eligible_for_trial: config.eligible_for_trial,
                account_too_young: false,
                sign_in_callback: Arc::new(|_, _| {}),
            }
            .into_any_element()
        };

        Some(
            v_flex()
                .p_4()
                .gap_4()
                .children(vec![
                    single_example(
                        "Not connected",
                        configuration(PreviewConfiguration {
                            plan: None,
                            is_connected: false,
                            is_zed_model_provider_enabled: true,
                            eligible_for_trial: false,
                        }),
                    ),
                    single_example(
                        "Accept Terms of Service",
                        configuration(PreviewConfiguration {
                            plan: None,
                            is_connected: true,
                            is_zed_model_provider_enabled: true,
                            eligible_for_trial: true,
                        }),
                    ),
                    single_example(
                        "No Plan - Not eligible for trial",
                        configuration(PreviewConfiguration {
                            plan: None,
                            is_connected: true,
                            is_zed_model_provider_enabled: true,
                            eligible_for_trial: false,
                        }),
                    ),
                    single_example(
                        "No Plan - Eligible for trial",
                        configuration(PreviewConfiguration {
                            plan: None,
                            is_connected: true,
                            is_zed_model_provider_enabled: true,
                            eligible_for_trial: true,
                        }),
                    ),
                    single_example(
                        "Free Plan",
                        configuration(PreviewConfiguration {
                            plan: Some(Plan::ZedFree),
                            is_connected: true,
                            is_zed_model_provider_enabled: true,
                            eligible_for_trial: true,
                        }),
                    ),
                    single_example(
                        "Zed Pro Trial Plan",
                        configuration(PreviewConfiguration {
                            plan: Some(Plan::ZedProTrial),
                            is_connected: true,
                            is_zed_model_provider_enabled: true,
                            eligible_for_trial: true,
                        }),
                    ),
                    single_example(
                        "Zed Pro Plan",
                        configuration(PreviewConfiguration {
                            plan: Some(Plan::ZedPro),
                            is_connected: true,
                            is_zed_model_provider_enabled: true,
                            eligible_for_trial: true,
                        }),
                    ),
                    single_example(
                        "Business Plan - Zed models enabled",
                        configuration(PreviewConfiguration {
                            plan: Some(Plan::ZedBusiness),
                            is_connected: true,
                            is_zed_model_provider_enabled: true,
                            eligible_for_trial: false,
                        }),
                    ),
                    single_example(
                        "Business Plan - Zed models disabled",
                        configuration(PreviewConfiguration {
                            plan: Some(Plan::ZedBusiness),
                            is_connected: true,
                            is_zed_model_provider_enabled: false,
                            eligible_for_trial: false,
                        }),
                    ),
                ])
                .into_any_element(),
        )
    }
}
