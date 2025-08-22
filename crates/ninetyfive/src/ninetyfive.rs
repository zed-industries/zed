mod ninetyfive_completion_provider;

pub use ninetyfive_completion_provider::*;

use client::Client;
use gpui::{App, AppContext, Context, Entity, Global};
use language::language_settings::{all_language_settings, EditPredictionProvider};
use std::sync::Arc;

pub fn init(client: Arc<Client>, cx: &mut App) {
    log::info!("NinetyFive: Initializing...");

    let ninetyfive = cx.new(|_| NinetyFive::Starting);
    NinetyFive::set_global(ninetyfive.clone(), cx);

    let provider = all_language_settings(None, cx).edit_predictions.provider;
    if provider == EditPredictionProvider::NinetyFive {
        log::info!("NinetyFive: Provider selected, starting...");
        ninetyfive.update(cx, |ninetyfive, cx| ninetyfive.start(client.clone(), cx));
    }
}

#[derive(Debug)]
pub enum NinetyFive {
    Starting,
    Ready,
    Error { error: anyhow::Error },
}

#[derive(Clone)]
struct NinetyFiveGlobal(Entity<NinetyFive>);

impl Global for NinetyFiveGlobal {}

impl NinetyFive {
    pub fn global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<NinetyFiveGlobal>()
            .map(|model| model.0.clone())
    }

    pub fn set_global(ninetyfive: Entity<Self>, cx: &mut App) {
        cx.set_global(NinetyFiveGlobal(ninetyfive));
    }

    pub fn start(&mut self, _client: Arc<Client>, cx: &mut Context<Self>) {
        log::debug!("NinetyFive: Start called ");
        //TODO this would ideally do something more

        cx.spawn(async move |this, cx| {
            log::info!("NinetyFive: transitioning to Ready");
            this.update(cx, |this, cx| {
                *this = Self::Ready;
                cx.notify();
            })?;

            Ok::<(), anyhow::Error>(())
        })
        .detach_and_log_err(cx)
    }

    pub fn stop(&mut self) {
        log::debug!("NinetyFive: Stopping...");
        *self = Self::Starting;
    }

    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Ready)
    }
}
