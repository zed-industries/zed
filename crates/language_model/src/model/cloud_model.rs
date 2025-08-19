use std::fmt;
use std::sync::Arc;

use anyhow::Result;
use client::Client;
use cloud_api_types::websocket_protocol::MessageToClient;
use cloud_llm_client::Plan;
use gpui::{App, AppContext as _, Context, Entity, EventEmitter, Global, ReadGlobal as _};
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

#[derive(Error, Debug)]
pub struct ModelRequestLimitReachedError {
    pub plan: Plan,
}

impl fmt::Display for ModelRequestLimitReachedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self.plan {
            Plan::ZedFree => "Model request limit reached. Upgrade to Zed Pro for more requests.",
            Plan::ZedPro => {
                "Model request limit reached. Upgrade to usage-based billing for more requests."
            }
            Plan::ZedProTrial => {
                "Model request limit reached. Upgrade to Zed Pro for more requests."
            }
        };

        write!(f, "{message}")
    }
}

#[derive(Error, Debug)]
pub struct ToolUseLimitReachedError;

impl fmt::Display for ToolUseLimitReachedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Consecutive tool use limit reached. Enable Burn Mode for unlimited tool use."
        )
    }
}

#[derive(Clone, Default)]
pub struct LlmApiToken(Arc<RwLock<Option<String>>>);

impl LlmApiToken {
    pub async fn acquire(&self, client: &Arc<Client>) -> Result<String> {
        let lock = self.0.upgradable_read().await;
        if let Some(token) = lock.as_ref() {
            Ok(token.to_string())
        } else {
            Self::fetch(RwLockUpgradableReadGuard::upgrade(lock).await, client).await
        }
    }

    pub async fn refresh(&self, client: &Arc<Client>) -> Result<String> {
        Self::fetch(self.0.write().await, client).await
    }

    async fn fetch(
        mut lock: RwLockWriteGuard<'_, Option<String>>,
        client: &Arc<Client>,
    ) -> Result<String> {
        let system_id = client
            .telemetry()
            .system_id()
            .map(|system_id| system_id.to_string());

        let response = client.cloud_client().create_llm_token(system_id).await?;
        *lock = Some(response.token.0.clone());
        Ok(response.token.0.clone())
    }
}

struct GlobalRefreshLlmTokenListener(Entity<RefreshLlmTokenListener>);

impl Global for GlobalRefreshLlmTokenListener {}

pub struct RefreshLlmTokenEvent;

pub struct RefreshLlmTokenListener;

impl EventEmitter<RefreshLlmTokenEvent> for RefreshLlmTokenListener {}

impl RefreshLlmTokenListener {
    pub fn register(client: Arc<Client>, cx: &mut App) {
        let listener = cx.new(|cx| RefreshLlmTokenListener::new(client, cx));
        cx.set_global(GlobalRefreshLlmTokenListener(listener));
    }

    pub fn global(cx: &App) -> Entity<Self> {
        GlobalRefreshLlmTokenListener::global(cx).0.clone()
    }

    fn new(client: Arc<Client>, cx: &mut Context<Self>) -> Self {
        client.add_message_to_client_handler({
            let this = cx.entity();
            move |message, cx| {
                Self::handle_refresh_llm_token(this.clone(), message, cx);
            }
        });

        Self
    }

    fn handle_refresh_llm_token(this: Entity<Self>, message: &MessageToClient, cx: &mut App) {
        match message {
            MessageToClient::UserUpdated => {
                this.update(cx, |_this, cx| cx.emit(RefreshLlmTokenEvent));
            }
        }
    }
}
