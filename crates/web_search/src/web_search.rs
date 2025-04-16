use anyhow::Result;
use collections::HashMap;
use gpui::{App, AppContext as _, Context, Entity, Global, SharedString, Task};
use serde::{Deserialize, Serialize};
use std::{ops::Range, sync::Arc};

pub fn init(cx: &mut App) {
    let registry = cx.new(|_cx| WebSearchRegistry::default());
    cx.set_global(GlobalWebSearchRegistry(registry));
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct WebSearchProviderId(pub SharedString);

#[derive(Serialize, Deserialize, Clone)]
pub struct WebSearchResponse {
    pub summary: SharedString,
    pub citations: Vec<WebSearchCitation>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WebSearchCitation {
    pub title: SharedString,
    pub url: SharedString,
    pub range: Option<Range<usize>>,
}

pub trait WebSearchProvider {
    fn id(&self) -> WebSearchProviderId;
    fn search(&self, query: String, cx: &mut App) -> Task<Result<WebSearchResponse>>;
}

struct GlobalWebSearchRegistry(Entity<WebSearchRegistry>);

impl Global for GlobalWebSearchRegistry {}

#[derive(Default)]
pub struct WebSearchRegistry {
    providers: HashMap<WebSearchProviderId, Arc<dyn WebSearchProvider>>,
    active_provider: Option<Arc<dyn WebSearchProvider>>,
}

impl WebSearchRegistry {
    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalWebSearchRegistry>().0.clone()
    }

    pub fn read_global(cx: &App) -> &Self {
        cx.global::<GlobalWebSearchRegistry>().0.read(cx)
    }

    pub fn providers(&self) -> impl Iterator<Item = &Arc<dyn WebSearchProvider>> {
        self.providers.values()
    }

    pub fn active_provider(&self) -> Option<Arc<dyn WebSearchProvider>> {
        self.active_provider.clone()
    }

    pub fn set_active_provider(&mut self, provider: Arc<dyn WebSearchProvider>) {
        self.active_provider = Some(provider.clone());
        self.providers.insert(provider.id(), provider);
    }

    pub fn register_provider<T: WebSearchProvider + 'static>(
        &mut self,
        provider: T,
        _cx: &mut Context<Self>,
    ) {
        let id = provider.id();
        self.providers.insert(id.clone(), Arc::new(provider));
    }
}
