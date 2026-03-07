use std::fmt;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use client::Client;
use client::UserStore;
use cloud_api_client::ClientApiError;
use cloud_api_types::OrganizationId;
use cloud_api_types::websocket_protocol::MessageToClient;
use cloud_llm_client::{EXPIRED_LLM_TOKEN_HEADER_NAME, OUTDATED_LLM_TOKEN_HEADER_NAME};
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, Global, ReadGlobal as _, Subscription,
};
use smol::lock::{RwLock, RwLockUpgradableReadGuard, RwLockWriteGuard};
use thiserror::Error;

#[derive(Error, Debug)]
pub struct PaymentRequiredError;

impl fmt::Display for PaymentRequiredError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Payment required to use this language model. Please upgrade your account."
        )
    }
}

#[derive(Clone, Default)]
pub struct LlmApiToken(Arc<RwLock<Option<String>>>);

impl LlmApiToken {
    pub async fn acquire(
        &self,
        client: &Arc<Client>,
        organization_id: Option<OrganizationId>,
    ) -> Result<String> {
        let lock = self.0.upgradable_read().await;
        if let Some(token) = lock.as_ref() {
            Ok(token.to_string())
        } else {
            Self::fetch(
                RwLockUpgradableReadGuard::upgrade(lock).await,
                client,
                organization_id,
            )
            .await
        }
    }

    pub async fn refresh(
        &self,
        client: &Arc<Client>,
        organization_id: Option<OrganizationId>,
    ) -> Result<String> {
        Self::fetch(self.0.write().await, client, organization_id).await
    }

    async fn fetch(
        mut lock: RwLockWriteGuard<'_, Option<String>>,
        client: &Arc<Client>,
        organization_id: Option<OrganizationId>,
    ) -> Result<String> {
        let system_id = client
            .telemetry()
            .system_id()
            .map(|system_id| system_id.to_string());

        let result = client
            .cloud_client()
            .create_llm_token(system_id, organization_id)
            .await;
        match result {
            Ok(response) => {
                *lock = Some(response.token.0.clone());
                Ok(response.token.0)
            }
            Err(err) => match err {
                ClientApiError::Unauthorized => {
                    client.request_sign_out();
                    Err(err).context("Failed to create LLM token")
                }
                ClientApiError::Other(err) => Err(err),
            },
        }
    }
}

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

struct GlobalRefreshLlmTokenListener(Entity<RefreshLlmTokenListener>);

impl Global for GlobalRefreshLlmTokenListener {}

pub struct RefreshLlmTokenEvent;

pub struct RefreshLlmTokenListener {
    _subscription: Subscription,
}

impl EventEmitter<RefreshLlmTokenEvent> for RefreshLlmTokenListener {}

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
            let this = cx.entity();
            move |message, cx| {
                Self::handle_refresh_llm_token(this.clone(), message, cx);
            }
        });

        let subscription = cx.subscribe(&user_store, |_this, _user_store, event, cx| {
            if matches!(event, client::user::Event::OrganizationChanged) {
                cx.emit(RefreshLlmTokenEvent);
            }
        });

        Self {
            _subscription: subscription,
        }
    }

    fn handle_refresh_llm_token(this: Entity<Self>, message: &MessageToClient, cx: &mut App) {
        match message {
            MessageToClient::UserUpdated => {
                this.update(cx, |_this, cx| cx.emit(RefreshLlmTokenEvent));
            }
        }
    }
}
