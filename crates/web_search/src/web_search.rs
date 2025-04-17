use anyhow::Result;
use collections::HashMap;
use gpui::{App, AppContext as _, Context, Entity, Global, SharedString, Task};
use std::sync::Arc;
use zed_llm_client::WebSearchResponse;

pub fn init(cx: &mut App) {
    let registry = cx.new(|_cx| WebSearchRegistry::default());
    cx.set_global(GlobalWebSearchRegistry(registry));
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct WebSearchProviderId(pub SharedString);

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
        let provider = Arc::new(provider);
        self.providers.insert(id.clone(), provider.clone());
        if self.active_provider.is_none() {
            self.active_provider = Some(provider);
        }
    }
}
