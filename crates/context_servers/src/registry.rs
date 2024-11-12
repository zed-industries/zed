use std::sync::Arc;

use anyhow::Result;
use collections::HashMap;
use gpui::{AppContext, AsyncAppContext, Global, Model, ReadGlobal, Task};
use parking_lot::RwLock;
use project::Project;

use crate::ContextServer;

pub type ContextServerFactory = Arc<
    dyn Fn(Model<Project>, &AsyncAppContext) -> Task<Result<Arc<dyn ContextServer>>>
        + Send
        + Sync
        + 'static,
>;

#[derive(Default)]
struct GlobalContextServerFactoryRegistry(Arc<ContextServerFactoryRegistry>);

impl Global for GlobalContextServerFactoryRegistry {}

#[derive(Default)]
struct ContextServerFactoryRegistryState {
    context_servers: HashMap<Arc<str>, ContextServerFactory>,
}

#[derive(Default)]
pub struct ContextServerFactoryRegistry {
    state: RwLock<ContextServerFactoryRegistryState>,
}

impl ContextServerFactoryRegistry {
    /// Returns the global [`ContextServerFactoryRegistry`].
    pub fn global(cx: &AppContext) -> Arc<Self> {
        GlobalContextServerFactoryRegistry::global(cx).0.clone()
    }

    /// Returns the global [`ContextServerFactoryRegistry`].
    ///
    /// Inserts a default [`ContextServerFactoryRegistry`] if one does not yet exist.
    pub fn default_global(cx: &mut AppContext) -> Arc<Self> {
        cx.default_global::<GlobalContextServerFactoryRegistry>()
            .0
            .clone()
    }

    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            state: RwLock::new(ContextServerFactoryRegistryState {
                context_servers: HashMap::default(),
            }),
        })
    }

    pub fn context_server_factories(&self) -> Vec<(Arc<str>, ContextServerFactory)> {
        self.state
            .read()
            .context_servers
            .iter()
            .map(|(id, factory)| (id.clone(), factory.clone()))
            .collect()
    }

    /// Registers the provided [`ContextServerFactory`].
    pub fn register_server_factory(&self, id: Arc<str>, factory: ContextServerFactory) {
        let mut state = self.state.write();
        state.context_servers.insert(id, factory);
    }

    /// Unregisters the [`ContextServerFactory`] for the server with the given ID.
    pub fn unregister_server_factory_by_id(&self, server_id: &str) {
        let mut state = self.state.write();
        state.context_servers.remove(server_id);
    }
}
