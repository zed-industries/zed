mod codestral_completion_provider;
mod input_excerpt;

pub use codestral_completion_provider::*;

use client::Client;
use gpui::{actions, App, AppContext, Context, Entity, Global};
use language::language_settings::all_language_settings;
use serde::{Deserialize, Serialize};
use settings::SettingsStore;
use std::sync::Arc;

actions!(
    codestral,
    [
        /// Signs out of Codestral.
        SignOut
    ]
);

pub fn init(client: Arc<Client>, cx: &mut App) {
    log::info!("Codestral: Initializing...");
    let codestral = cx.new(|_| Codestral::Starting);
    Codestral::set_global(codestral.clone(), cx);

    let mut provider = all_language_settings(None, cx).edit_predictions.provider;
    if provider == language::language_settings::EditPredictionProvider::Codestral {
        log::info!("Codestral: Provider selected, starting...");
        codestral.update(cx, |codestral, cx| codestral.start(client.clone(), cx));
    }

    cx.observe_global::<SettingsStore>(move |cx| {
        let new_provider = all_language_settings(None, cx).edit_predictions.provider;
        if new_provider != provider {
            provider = new_provider;
            if provider == language::language_settings::EditPredictionProvider::Codestral {
                log::info!("Codestral: Provider selected, starting...");
                codestral.update(cx, |codestral, cx| codestral.start(client.clone(), cx));
            } else {
                log::info!("Codestral: Provider deselected, stopping...");
                codestral.update(cx, |codestral, _cx| codestral.stop());
            }
        }
    })
    .detach();

    cx.on_action(|_: &SignOut, cx| {
        if let Some(codestral) = Codestral::global(cx) {
            codestral.update(cx, |codestral, cx| codestral.sign_out(cx));
        }
    });
}

#[derive(Debug)]
pub enum Codestral {
    Starting,
    Authenticating,
    Ready { api_key: String },
    Error { error: anyhow::Error },
}

#[derive(Clone)]
struct CodestralGlobal(Entity<Codestral>);

impl Global for CodestralGlobal {}

impl Codestral {
    pub fn global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<CodestralGlobal>()
            .map(|model| model.0.clone())
    }

    pub fn set_global(codestral: Entity<Self>, cx: &mut App) {
        cx.set_global(CodestralGlobal(codestral));
    }

    pub fn start(&mut self, _client: Arc<Client>, cx: &mut Context<Self>) {
        if let Self::Starting = self {
            log::debug!("Codestral: Transitioning from Starting to Authenticating");
            *self = Self::Authenticating;

            cx.spawn(async move |this, cx| {
                // Try to get API key from settings first
                let api_key = cx.update(|cx| {
                    let settings = all_language_settings(None, cx);
                    settings.edit_predictions.codestral.api_key.clone()
                })?;

                if let Some(api_key) = api_key {
                    log::info!("Codestral: API key configured, transitioning to Ready");
                    this.update(cx, |this, cx| {
                        *this = Self::Ready { api_key };
                        cx.notify();
                    })?;
                } else {
                    let error_msg =
                        "No API key configured. Please add your Codestral API key to settings.";
                    log::error!("Codestral: {}", error_msg);
                    this.update(cx, |this, cx| {
                        *this = Self::Error {
                            error: anyhow::anyhow!(error_msg),
                        };
                        cx.notify();
                    })?;
                }
                Ok::<(), anyhow::Error>(())
            })
            .detach_and_log_err(cx)
        } else {
            log::debug!("Codestral: Start called but already in state: {:?}", self);
        }
    }

    pub fn stop(&mut self) {
        log::info!("Codestral: Stopping...");
        *self = Self::Starting;
    }

    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Ready { .. })
    }

    pub fn api_key(&self) -> Option<&str> {
        match self {
            Self::Ready { api_key } => Some(api_key),
            _ => None,
        }
    }

    pub fn sign_out(&mut self, cx: &mut Context<Self>) {
        log::info!("Codestral: Signing out...");
        *self = Self::Starting;
        cx.notify();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodestralRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct CodestralResponse {
    pub id: String,
    pub object: String,
    pub model: String,
    pub usage: Usage,
    pub created: u64,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub content: String,
    pub role: String,
}
