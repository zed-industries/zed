use super::{Client, UserStore};
use cloud_api_types::websocket_protocol::MessageToClient;
use cloud_llm_client::{EXPIRED_LLM_TOKEN_HEADER_NAME, OUTDATED_LLM_TOKEN_HEADER_NAME};
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, Global, ReadGlobal as _, Subscription,
};
use language_model::LlmApiToken;
use std::sync::Arc;

pub trait NeedsLlmTokenRefresh {
    /// Returns whether the LLM token needs to be refreshed.
    fn needs_llm_token_refresh(&self) -> bool;
}

impl NeedsLlmTokenRefresh for http_client::Response<http_client::AsyncBody> {
    fn needs_llm_token_refresh(&self) -> bool {
        self.headers().get(EXPIRED_LLM_TOKEN_HEADER_NAME).is_some()
            || self.headers().get(OUTDATED_LLM_TOKEN_HEADER_NAME).is_some()
    }
}

enum TokenRefreshMode {
    Refresh,
    ClearAndRefresh,
}

pub fn global_llm_token(cx: &App) -> LlmApiToken {
    RefreshLlmTokenListener::global(cx)
        .read(cx)
        .llm_api_token
        .clone()
}

struct GlobalRefreshLlmTokenListener(Entity<RefreshLlmTokenListener>);

impl Global for GlobalRefreshLlmTokenListener {}

pub struct LlmTokenRefreshedEvent;

pub struct RefreshLlmTokenListener {
    client: Arc<Client>,
    user_store: Entity<UserStore>,
    llm_api_token: LlmApiToken,
    _subscription: Subscription,
}

impl EventEmitter<LlmTokenRefreshedEvent> for RefreshLlmTokenListener {}

impl RefreshLlmTokenListener {
    pub fn register(client: Arc<Client>, user_store: Entity<UserStore>, cx: &mut App) {
        let listener = cx.new(|cx| RefreshLlmTokenListener::new(client, user_store, cx));
        cx.set_global(GlobalRefreshLlmTokenListener(listener));
    }

    pub fn global(cx: &App) -> Entity<Self> {
        GlobalRefreshLlmTokenListener::global(cx).0.clone()
    }

    fn new(client: Arc<Client>, user_store: Entity<UserStore>, cx: &mut Context<Self>) -> Self {
        client.add_message_to_client_handler({
            let this = cx.weak_entity();
            move |message, cx| {
                if let Some(this) = this.upgrade() {
                    Self::handle_refresh_llm_token(this, message, cx);
                }
            }
        });

        let subscription = cx.subscribe(&user_store, |this, _user_store, event, cx| {
            if matches!(event, super::user::Event::OrganizationChanged) {
                this.refresh(TokenRefreshMode::ClearAndRefresh, cx);
            }
        });

        Self {
            client,
            user_store,
            llm_api_token: LlmApiToken::default(),
            _subscription: subscription,
        }
    }

    fn refresh(&self, mode: TokenRefreshMode, cx: &mut Context<Self>) {
        let client = self.client.clone();
        let llm_api_token = self.llm_api_token.clone();
        let organization_id = self
            .user_store
            .read(cx)
            .current_organization()
            .map(|organization| organization.id.clone());
        cx.spawn(async move |this, cx| {
            match mode {
                TokenRefreshMode::Refresh => {
                    client
                        .refresh_llm_token(&llm_api_token, organization_id)
                        .await?;
                }
                TokenRefreshMode::ClearAndRefresh => {
                    client
                        .clear_and_refresh_llm_token(&llm_api_token, organization_id)
                        .await?;
                }
            }
            this.update(cx, |_this, cx| cx.emit(LlmTokenRefreshedEvent))
        })
        .detach_and_log_err(cx);
    }

    fn handle_refresh_llm_token(this: Entity<Self>, message: &MessageToClient, cx: &mut App) {
        match message {
            MessageToClient::UserUpdated => {
                this.update(cx, |this, cx| this.refresh(TokenRefreshMode::Refresh, cx));
            }
        }
    }
}
