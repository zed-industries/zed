use std::sync::Arc;

use collections::HashMap;
use gpui::{AppContext, BackgroundExecutor, Global, ReadGlobal, UpdateGlobal};
use parking_lot::RwLock;

use crate::{IndexedDocsProvider, IndexedDocsStore, ProviderId};

struct GlobalIndexedDocsRegistry(Arc<IndexedDocsRegistry>);

impl Global for GlobalIndexedDocsRegistry {}

pub struct IndexedDocsRegistry {
    executor: BackgroundExecutor,
    stores_by_provider: RwLock<HashMap<ProviderId, Arc<IndexedDocsStore>>>,
}

impl IndexedDocsRegistry {
    pub fn global(cx: &AppContext) -> Arc<Self> {
        GlobalIndexedDocsRegistry::global(cx).0.clone()
    }

    pub fn init_global(cx: &mut AppContext) {
        GlobalIndexedDocsRegistry::set_global(
            cx,
            GlobalIndexedDocsRegistry(Arc::new(Self::new(cx.background_executor().clone()))),
        );
    }

    pub fn new(executor: BackgroundExecutor) -> Self {
        Self {
            executor,
            stores_by_provider: RwLock::new(HashMap::default()),
        }
    }

    pub fn list_providers(&self) -> Vec<ProviderId> {
        self.stores_by_provider
            .read()
            .keys()
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn register_provider(
        &self,
        provider: Box<dyn IndexedDocsProvider + Send + Sync + 'static>,
    ) {
        self.stores_by_provider.write().insert(
            provider.id(),
            Arc::new(IndexedDocsStore::new(provider, self.executor.clone())),
        );
    }

    pub fn get_provider_store(&self, provider_id: ProviderId) -> Option<Arc<IndexedDocsStore>> {
        self.stores_by_provider.read().get(&provider_id).cloned()
    }
}
