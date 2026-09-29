use ai_onboarding::YoungAccountBanner;
use anyhow::{Result, anyhow};
use client::{
    Client, RefreshLlmTokenListener, TelemetrySettings, UserStore, global_llm_token, zed_urls,
};
use cloud_api_client::{ClientApiError, LlmApiToken};
use cloud_api_types::OrganizationId;
use cloud_api_types::Plan;
use futures::FutureExt;
use futures::StreamExt;
use futures::future::BoxFuture;

use gpui::{AnyElement, App, AppContext, AsyncApp, Context, Entity, Subscription, Task, TaskExt};
use language_model::{
    AuthenticateError, CompactionResult, FastModeConfirmation, IconOrSvg, InlineDescription,
    LanguageModel, LanguageModelClient, LanguageModelCompletionError,
    LanguageModelCompletionStream, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    ProviderSettingsView, ZED_CLOUD_PROVIDER_ID, ZED_CLOUD_PROVIDER_NAME,
};
use language_models_cloud::{
    CloudCatalog, CloudLlmTokenProvider, CloudModelProvider, language_model,
};
use parking_lot::Mutex;
use rand::{Rng as _, SeedableRng as _, rngs::StdRng};
use release_channel::AppVersion;

pub use settings::ZedDotDevAvailableModel as AvailableModel;
pub use settings::ZedDotDevAvailableProvider as AvailableProvider;
use settings::{Settings as _, SettingsStore};
use std::sync::Arc;
use std::time::Duration;
use ui::{TintColor, prelude::*};

const PROVIDER_ID: LanguageModelProviderId = ZED_CLOUD_PROVIDER_ID;
const PROVIDER_NAME: LanguageModelProviderName = ZED_CLOUD_PROVIDER_NAME;
const MODELS_REFRESH_DEBOUNCE: Duration = Duration::from_secs(5 * 60);

struct ClientTokenProvider {
    client: Arc<Client>,
    llm_api_token: LlmApiToken,
    /// The organization tokens are issued for, mirrored from the
    /// [`UserStore`] because token requests run off the main thread.
    organization_id: Mutex<Option<OrganizationId>>,
}

impl ClientTokenProvider {
    fn update_organization(&self, user_store: &UserStore) {
        *self.organization_id.lock() = user_store
            .current_organization()
            .map(|organization| organization.id.clone());
    }
}

impl CloudLlmTokenProvider for ClientTokenProvider {
    fn token(&self, force_refresh: bool) -> BoxFuture<'static, Result<String, ClientApiError>> {
        let client = self.client.clone();
        let llm_api_token = self.llm_api_token.clone();
        let organization_id = self.organization_id.lock().clone();
        Box::pin(async move {
            let organization_id = organization_id.ok_or(ClientApiError::NotSignedIn)?;
            if force_refresh {
                client
                    .refresh_llm_token(&llm_api_token, organization_id)
                    .await
            } else {
                client
                    .cached_llm_token(&llm_api_token, organization_id)
                    .await
            }
        })
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ZedDotDevSettings {
    pub available_models: Vec<AvailableModel>,
}

pub struct CloudLanguageModelProvider {
    state: Entity<State>,
    client: CloudModelProvider,
    _maintain_client_status: Task<()>,
}

pub struct State {
    client: Arc<Client>,
    user_store: Entity<UserStore>,
    status: client::Status,
    catalog: Entity<CloudCatalog>,
    pending_models_refresh: Option<Task<()>>,
    _user_store_subscription: Subscription,
    _settings_subscription: Subscription,
    _llm_token_subscription: Subscription,
    _catalog_subscription: Subscription,
    _cloud_reconnect_task: Task<()>,
}

impl State {
    fn new(
        client: Arc<Client>,
        user_store: Entity<UserStore>,
        status: client::Status,
        token_provider: Arc<ClientTokenProvider>,
        catalog: Entity<CloudCatalog>,
        cx: &mut Context<Self>,
    ) -> Self {
        let refresh_llm_token_listener = RefreshLlmTokenListener::global(cx);
        token_provider.update_organization(user_store.read(cx));

        let cloud_reconnect_task = cx.spawn({
            let client = client.clone();
            async move |this, cx| {
                let mut connection_id_rx = client.cloud_connection_id();
                while let Some(connection_id) = connection_id_rx.next().await {
                    // The initial value `0` means no connection has been
                    // established since this `Client` was created; only real
                    // reconnects trigger a refresh.
                    if connection_id == 0 {
                        continue;
                    }
                    if this
                        .update(cx, |this, cx| this.schedule_debounced_models_refresh(cx))
                        .is_err()
                    {
                        break;
                    }
                }
            }
        });

        Self {
            client: client.clone(),
            user_store: user_store.clone(),
            status,
            pending_models_refresh: None,
            _catalog_subscription: cx.observe(&catalog, |_, _, cx| cx.notify()),
            catalog,
            _user_store_subscription: cx.subscribe(
                &user_store,
                move |this, user_store, event, cx| {
                    // Signing in or out changes the organization without
                    // emitting `OrganizationChanged`, so re-read it on every
                    // event.
                    token_provider.update_organization(user_store.read(cx));
                    match event {
                        client::user::Event::PrivateUserInfoUpdated => {
                            let status = *client.status().borrow();
                            if status.is_signed_out() {
                                return;
                            }

                            this.refresh_models(cx);
                        }
                        _ => {}
                    }
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
            _cloud_reconnect_task: cloud_reconnect_task,
        }
    }

    fn is_signed_out(&self, cx: &App) -> bool {
        self.status.is_signed_out() || self.user_store.read(cx).current_user().is_none()
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
        self.catalog.update(cx, |catalog, cx| {
            catalog.refresh_models(cx).detach_and_log_err(cx);
        });
    }

    /// Schedules a model list refresh, replacing any previously scheduled
    /// refresh.
    fn schedule_debounced_models_refresh(&mut self, cx: &mut Context<Self>) {
        self.pending_models_refresh = Some(cx.spawn(async move |this, cx| {
            #[cfg(any(test, feature = "test-support"))]
            let mut rng = StdRng::seed_from_u64(0);
            #[cfg(not(any(test, feature = "test-support")))]
            let mut rng = StdRng::from_os_rng();
            let jitter = Duration::from_millis(
                rng.random_range(0..MODELS_REFRESH_DEBOUNCE.as_millis() as u64),
            );
            cx.background_executor()
                .timer(MODELS_REFRESH_DEBOUNCE + jitter)
                .await;
            this.update(cx, |this, cx| this.refresh_models(cx)).ok();
        }));
    }
}

impl CloudLanguageModelProvider {
    pub fn new(user_store: Entity<UserStore>, client: Arc<Client>, cx: &mut App) -> Self {
        let mut status_rx = client.status();
        let status = *status_rx.borrow();

        let token_provider = Arc::new(ClientTokenProvider {
            client: client.clone(),
            llm_api_token: global_llm_token(cx),
            organization_id: Mutex::new(None),
        });
        let cloud_client = CloudModelProvider::new(
            token_provider.clone(),
            client.http_client(),
            Some(AppVersion::global(cx)),
            cx,
        );
        let catalog = cloud_client.catalog().clone();
        let state = cx.new(|cx| {
            State::new(
                client.clone(),
                user_store.clone(),
                status,
                token_provider,
                catalog,
                cx,
            )
        });

        let state_ref = state.downgrade();
        let maintain_client_status = cx.spawn(async move |cx| {
            while let Some(status) = status_rx.next().await {
                if let Some(this) = state_ref.upgrade() {
                    _ = this.update(cx, |this, cx| {
                        if this.status != status {
                            this.status = status;
                            if status.is_signed_out() {
                                this.catalog.update(cx, |catalog, cx| {
                                    catalog.clear_models();
                                    cx.notify();
                                });
                            }
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
            client: cloud_client,
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

    fn default_model(&self, cx: &App) -> Option<LanguageModel> {
        let catalog = self.state.read(cx).catalog.read(cx);
        Some(language_model(catalog.default_model()?))
    }

    fn default_fast_model(&self, cx: &App) -> Option<LanguageModel> {
        let catalog = self.state.read(cx).catalog.read(cx);
        Some(language_model(catalog.default_fast_model()?))
    }

    fn recommended_models(&self, cx: &App) -> Vec<LanguageModel> {
        let catalog = self.state.read(cx).catalog.read(cx);
        catalog
            .recommended_models()
            .iter()
            .map(|model| language_model(model))
            .collect()
    }

    fn provided_models(&self, cx: &App) -> Vec<LanguageModel> {
        let catalog = self.state.read(cx).catalog.read(cx);
        catalog
            .models()
            .iter()
            .map(|model| language_model(model))
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
                    return Err(AuthenticateError::Other(anyhow!(
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

    fn settings_view(&self, cx: &mut App) -> Option<ProviderSettingsView> {
        let state = self.state.read(cx);
        let user_store = state.user_store.read(cx);
        let is_zed_model_provider_enabled = user_store
            .current_organization_configuration()
            .map_or(true, |config| config.is_zed_model_provider_enabled);
        let description = InlineDescription::Text(
            zed_ai_description(
                !state.is_signed_out(cx),
                user_store.plan(),
                is_zed_model_provider_enabled,
                user_store.trial_started_at().is_none(),
            )
            .into(),
        );

        let title = if state.is_signed_out(cx) {
            None
        } else {
            match state.user_store.read(cx).plan() {
                Some(Plan::ZedPro) => Some("Subscribed to Pro".into()),
                Some(Plan::ZedProTrial) => Some("Subscribed to Pro Trial".into()),
                Some(Plan::ZedStudent) => Some("Subscribed to Student".into()),
                Some(Plan::ZedBusiness) => Some("Subscribed to Business".into()),
                Some(Plan::ZedVip) => Some("Subscribed to VIP".into()),
                Some(Plan::ZedFree) | None => None,
            }
        };

        Some(ProviderSettingsView::Inline(
            language_model::InlineProviderSettings {
                title,
                description: Some(description),
                create_view: Arc::new({
                    let state = self.state.clone();
                    move |_window, cx| {
                        cx.new(|_| ConfigurationView::new(state.clone(), true))
                            .into()
                    }
                }),
            },
        ))
    }

    fn authentication_error_message(&self) -> SharedString {
        "Failed to sign in with your Zed account (401).".into()
    }

    fn missing_credentials_error_message(&self) -> SharedString {
        "You are not signed in to your Zed account. \
        Sign in to continue."
            .into()
    }

    fn fast_mode_confirmation(&self, _cx: &App) -> Option<FastModeConfirmation> {
        Some(FastModeConfirmation {
            title: "Enable Fast Mode for Zed?".into(),
            message: "Fast mode routes requests through the upstream provider's fast mode or priority tier. The \
                upstream provider's premium per-token pricing applies and is passed through to \
                your Zed billing."
                .into(),
        })
    }
}

impl CloudLanguageModelProvider {
    fn check_data_retention_consent(
        model: &LanguageModel,
        cx: &AsyncApp,
    ) -> Result<(), LanguageModelCompletionError> {
        let has_consent = cx.update(|cx| TelemetrySettings::get_global(cx).anthropic_retention);
        if model.requires_data_retention() && !has_consent {
            return Err(LanguageModelCompletionError::DataRetentionConsentRequired {
                model_name: model.name.0.to_string(),
            });
        }
        Ok(())
    }
}

impl LanguageModelClient for CloudLanguageModelProvider {
    fn stream_completion(
        &self,
        model: &LanguageModel,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelCompletionStream, LanguageModelCompletionError>>
    {
        if let Err(error) = Self::check_data_retention_consent(model, cx) {
            return async move { Err(error) }.boxed();
        }
        self.client.stream_completion(model, request, cx)
    }

    fn count_input_tokens(
        &self,
        model: &LanguageModel,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<Option<u64>, LanguageModelCompletionError>> {
        if let Err(error) = Self::check_data_retention_consent(model, cx) {
            return async move { Err(error) }.boxed();
        }
        self.client.count_input_tokens(model, request, cx)
    }

    fn compact(
        &self,
        model: &LanguageModel,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<CompactionResult, LanguageModelCompletionError>> {
        if let Err(error) = Self::check_data_retention_consent(model, cx) {
            return async move { Err(error) }.boxed();
        }
        self.client.compact(model, request, cx)
    }
}

#[derive(IntoElement, RegisterComponent)]
struct ZedAiConfiguration {
    is_connected: bool,
    plan: Option<Plan>,
    is_zed_model_provider_enabled: bool,
    eligible_for_trial: bool,
    account_too_young: bool,
    compact: bool,
    sign_in_callback: Arc<dyn Fn(&mut Window, &mut App) + Send + Sync>,
}

#[cfg(any(test, feature = "test-support"))]
pub mod test_support {
    use super::*;

    pub fn young_account_configuration() -> AnyElement {
        ZedAiConfiguration {
            is_connected: true,
            plan: Some(Plan::ZedBusiness),
            is_zed_model_provider_enabled: true,
            eligible_for_trial: false,
            account_too_young: true,
            compact: true,
            sign_in_callback: Arc::new(|_, _| {}),
        }
        .into_any_element()
    }
}

fn zed_ai_description(
    is_connected: bool,
    plan: Option<Plan>,
    is_zed_model_provider_enabled: bool,
    eligible_for_trial: bool,
) -> &'static str {
    if !is_connected {
        return "Sign in to have access to Zed's complete agentic experience with hosted models.";
    }

    match plan {
        Some(Plan::ZedPro) => {
            "You have access to Zed's hosted models through your Pro subscription."
        }
        Some(Plan::ZedProTrial) => {
            "Your Pro trial includes $5 of GPT Luna and unlimited edit predictions for 14 days from trial start."
        }
        Some(Plan::ZedStudent) => {
            "You have access to Zed's hosted models through your Student subscription."
        }
        Some(Plan::ZedBusiness) => {
            if is_zed_model_provider_enabled {
                "You have access to Zed's hosted models through your organization."
            } else {
                "Zed's hosted models are disabled by your organization's configuration."
            }
        }
        Some(Plan::ZedVip) => {
            "You have access to Zed's hosted models through your VIP subscription."
        }
        Some(Plan::ZedFree) | None => {
            if eligible_for_trial {
                "Start a free trial with $5 of GPT Luna and unlimited edit predictions for 14 days from trial start."
            } else {
                "Subscribe for access to Zed's hosted models."
            }
        }
    }
}

impl RenderOnce for ZedAiConfiguration {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let has_paid_plan = matches!(
            self.plan,
            Some(Plan::ZedPro | Plan::ZedStudent | Plan::ZedBusiness | Plan::ZedVip)
        );

        let description = zed_ai_description(
            self.is_connected,
            self.plan,
            self.is_zed_model_provider_enabled,
            self.eligible_for_trial,
        );

        let manage_subscription_buttons = if has_paid_plan {
            Button::new("manage_settings", "Manage Subscription")
                .when(!self.compact, |this| {
                    this.full_width().label_size(LabelSize::Small)
                })
                .when(self.compact, |this| this.size(ButtonSize::Medium))
                .style(ButtonStyle::Tinted(TintColor::Accent))
                .on_click(|_, _, cx| cx.open_url(&zed_urls::account_url(cx)))
                .into_any_element()
        } else if self.plan.is_none() || self.eligible_for_trial {
            Button::new("start_trial", "Start Free Trial")
                .when(!self.compact, |this| {
                    this.full_width().label_size(LabelSize::Small)
                })
                .when(self.compact, |this| this.size(ButtonSize::Medium))
                .style(ui::ButtonStyle::Tinted(ui::TintColor::Accent))
                .on_click(|_, _, cx| cx.open_url(&zed_urls::start_trial_url(cx)))
                .into_any_element()
        } else {
            Button::new("upgrade", "Upgrade to Pro")
                .when(!self.compact, |this| {
                    this.full_width().label_size(LabelSize::Small)
                })
                .when(self.compact, |this| this.size(ButtonSize::Medium))
                .style(ui::ButtonStyle::Tinted(ui::TintColor::Accent))
                .on_click(|_, _, cx| cx.open_url(&zed_urls::upgrade_to_zed_pro_url(cx)))
                .into_any_element()
        };

        if !self.is_connected {
            return v_flex()
                .gap_2()
                .when(!self.compact, |this| this.child(Label::new(description)))
                .child(
                    Button::new("sign_in", "Sign In to use Zed AI")
                        .start_icon(
                            Icon::new(IconName::Github)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        )
                        .when(!self.compact, |this| this.full_width())
                        .on_click({
                            let callback = self.sign_in_callback.clone();
                            move |_, window, cx| (callback)(window, cx)
                        }),
                );
        }

        v_flex()
            .gap_2()
            .debug_selector(|| "zed-ai-configuration".into())
            .when(!self.compact || self.account_too_young, |this| {
                this.w_full()
            })
            .map(|this| {
                if self.account_too_young {
                    this.child(YoungAccountBanner).child(
                        Button::new("upgrade", "Upgrade to Pro")
                            .style(ui::ButtonStyle::Tinted(ui::TintColor::Accent))
                            .when(!self.compact, |this| this.full_width())
                            .on_click(|_, _, cx| {
                                cx.open_url(&zed_urls::upgrade_to_zed_pro_url(cx))
                            }),
                    )
                } else {
                    this.when(!self.compact, |this| this.text_sm().child(description))
                        .child(manage_subscription_buttons)
                }
            })
    }
}

struct ConfigurationView {
    state: Entity<State>,
    compact: bool,
    sign_in_callback: Arc<dyn Fn(&mut Window, &mut App) + Send + Sync>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, compact: bool) -> Self {
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
            compact,
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
            compact: self.compact,
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

    fn test_cloud_model(
        model_id: cloud_llm_client::LanguageModelId,
    ) -> cloud_llm_client::LanguageModel {
        cloud_llm_client::LanguageModel {
            provider: cloud_llm_client::LanguageModelProvider::Anthropic,
            id: model_id,
            display_name: "Test Model".to_string(),
            is_latest: true,
            max_token_count: 200_000,
            max_token_count_in_max_mode: None,
            max_output_tokens: 8_192,
            supports_tools: true,
            supports_images: false,
            supports_thinking: false,
            supports_disabling_thinking: false,
            supports_fast_mode: false,
            supports_server_side_compaction: false,
            supported_effort_levels: Vec::new(),
            supports_streaming_tools: false,
            supports_parallel_tool_calls: false,
            is_disabled: false,
            disabled_reason: None,
        }
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

    #[gpui::test]
    async fn provided_models_surface_disabled_reason(cx: &mut TestAppContext) {
        let (_client, _user_store, provider) = cx.update(init_test);
        let model_id = cloud_llm_client::LanguageModelId(Arc::from("disabled-model"));
        let disabled_reason = "This model is temporarily unavailable.";

        cx.update(|cx| {
            let catalog = provider.state.read(cx).catalog.clone();
            catalog.update(cx, |catalog, cx| {
                let mut model = test_cloud_model(model_id.clone());
                model.is_disabled = true;
                model.disabled_reason = Some(disabled_reason.to_string());
                catalog.update_models(cloud_llm_client::ListModelsResponse {
                    models: vec![model],
                    default_model: Some(model_id.clone()),
                    default_fast_model: None,
                    recommended_models: vec![model_id],
                });
                cx.notify();
            });
        });

        let model = cx.read(|cx| {
            provider
                .provided_models(cx)
                .into_iter()
                .next()
                .expect("disabled model should be provided")
        });
        assert_eq!(
            model.is_disabled(),
            Some(language_model::DisabledReason::new(disabled_reason))
        );
    }

    #[gpui::test]
    async fn retention_consent_gates_counting_compaction_and_generation(cx: &mut TestAppContext) {
        let (_client, _user_store, provider) = cx.update(init_test);
        let model_id = cloud_llm_client::LanguageModelId(Arc::from("claude-fable-5-1"));
        let mut config = test_cloud_model(model_id);
        config.supports_server_side_compaction = true;
        let model = language_models_cloud::language_model(&config);
        assert!(model.requires_data_retention());
        cx.update(|cx| {
            let catalog = provider.state.read(cx).catalog.clone();
            catalog.update(cx, |catalog, _| {
                catalog.update_models(cloud_llm_client::ListModelsResponse {
                    models: vec![config],
                    default_model: None,
                    default_fast_model: None,
                    recommended_models: Vec::new(),
                })
            });
        });
        let request = LanguageModelRequest::default();

        assert!(matches!(
            provider
                .count_input_tokens(&model, request.clone(), &cx.to_async())
                .await,
            Err(LanguageModelCompletionError::DataRetentionConsentRequired { .. })
        ));
        assert!(matches!(
            provider
                .compact(&model, request.clone(), &cx.to_async())
                .await,
            Err(LanguageModelCompletionError::DataRetentionConsentRequired { .. })
        ));
        assert!(matches!(
            provider
                .stream_completion(&model, request.clone(), &cx.to_async())
                .await,
            Err(LanguageModelCompletionError::DataRetentionConsentRequired { .. })
        ));

        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .telemetry
                        .get_or_insert_default()
                        .anthropic_retention = Some(true);
                });
            });
        });
        // Past the consent gate, the signed-out provider cannot acquire a
        // token.
        assert!(matches!(
            provider
                .count_input_tokens(&model, request, &cx.to_async())
                .await,
            Err(LanguageModelCompletionError::ProviderRejection {
                category: language_model::ProviderErrorCategory::Authentication,
                ..
            })
        ));
    }

    #[gpui::test]
    async fn sign_out_hides_cached_cloud_models(cx: &mut TestAppContext) {
        let (client, _user_store, provider) = cx.update(init_test);
        let (authenticate_tx, authenticate_rx) = futures::channel::oneshot::channel();
        let (authenticated_user_tx, authenticated_user_rx) = futures::channel::oneshot::channel();
        override_authenticate(&client, authenticate_rx);
        respond_to_authenticated_user_after(&client, authenticated_user_rx);

        let sign_in_task = sign_in_until_authenticating(client.clone(), cx).await;
        authenticate_tx
            .send(Ok(Credentials {
                user_id: TEST_USER_ID,
                access_token: "token".to_string(),
            }))
            .expect("authenticate receiver dropped");
        authenticated_user_tx
            .send(())
            .expect("authenticated user receiver dropped");
        sign_in_task.await.expect("sign-in should complete");
        cx.executor().run_until_parked();

        let model_id = cloud_llm_client::LanguageModelId(Arc::from("test-model"));
        cx.update(|cx| {
            let catalog = provider.state.read(cx).catalog.clone();
            catalog.update(cx, |catalog, cx| {
                catalog.update_models(cloud_llm_client::ListModelsResponse {
                    models: vec![test_cloud_model(model_id.clone())],
                    default_model: Some(model_id.clone()),
                    default_fast_model: None,
                    recommended_models: vec![model_id],
                });
                cx.notify();
            });
        });

        assert!(cx.read(|cx| provider.is_authenticated(cx)));
        assert_eq!(cx.read(|cx| provider.provided_models(cx).len()), 1);
        assert!(cx.read(|cx| provider.default_model(cx).is_some()));
        assert_eq!(cx.read(|cx| provider.recommended_models(cx).len()), 1);

        cx.update(|cx| {
            cx.spawn({
                let client = client.clone();
                async move |cx| client.sign_out(cx).await
            })
        })
        .await;
        cx.executor().run_until_parked();

        assert!(!cx.read(|cx| provider.is_authenticated(cx)));
        assert!(cx.read(|cx| provider.provided_models(cx).is_empty()));
        assert!(cx.read(|cx| provider.default_model(cx).is_none()));
        assert!(cx.read(|cx| provider.recommended_models(cx).is_empty()));
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

    fn description() -> &'static str {
        "The configuration surface for Zed's hosted AI models, \
        showing the user's connection status, current plan, trial eligibility, \
        and entry points for enabling the Zed model provider."
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> AnyElement {
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
                compact: false,
                sign_in_callback: Arc::new(|_, _| {}),
            }
            .into_any_element()
        };

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
            .into_any_element()
    }
}
