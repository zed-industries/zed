use std::sync::Arc;

use anyhow::Result;
use collections::HashMap;
use gpui::{AppContext, AsyncAppContext, Context, Global, Model, ReadGlobal, Task};
use project::Project;

use crate::manager::ServerCommand;

pub type ContextServerFactory = Arc<
    dyn Fn(Model<Project>, &AsyncAppContext) -> Task<Result<ServerCommand>> + Send + Sync + 'static,
>;

struct GlobalContextServerFactoryRegistry(Model<ContextServerFactoryRegistry>);

impl Global for GlobalContextServerFactoryRegistry {}

#[derive(Default)]
pub struct ContextServerFactoryRegistry {
    context_servers: HashMap<Arc<str>, ContextServerFactory>,
}

impl ContextServerFactoryRegistry {
    /// Returns the global [`ContextServerFactoryRegistry`].
    pub fn global(cx: &AppContext) -> Model<Self> {
        GlobalContextServerFactoryRegistry::global(cx).0.clone()
    }

    /// Returns the global [`ContextServerFactoryRegistry`].
    ///
    /// Inserts a default [`ContextServerFactoryRegistry`] if one does not yet exist.
    pub fn default_global(cx: &mut AppContext) -> Model<Self> {
        if !cx.has_global::<GlobalContextServerFactoryRegistry>() {
            let registry = cx.new_model(|_| Self::new());
            cx.set_global(GlobalContextServerFactoryRegistry(registry));
        }
        cx.global::<GlobalContextServerFactoryRegistry>().0.clone()
    }

    pub fn new() -> Self {
        Self {
            context_servers: HashMap::default(),
        }
    }

    pub fn context_server_factories(&self) -> Vec<(Arc<str>, ContextServerFactory)> {
        self.context_servers
            .iter()
            .map(|(id, factory)| (id.clone(), factory.clone()))
            .collect()
    }

    /// Registers the provided [`ContextServerFactory`].
    pub fn register_server_factory(&mut self, id: Arc<str>, factory: ContextServerFactory) {
        self.context_servers.insert(id, factory);
    }

    /// Unregisters the [`ContextServerFactory`] for the server with the given ID.
    pub fn unregister_server_factory_by_id(&mut self, server_id: &str) {
        self.context_servers.remove(server_id);
    }
}
