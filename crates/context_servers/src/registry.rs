use std::sync::Arc;

use anyhow::Result;
use collections::BTreeMap;
use futures::future::BoxFuture;
use gpui::{AppContext, AsyncAppContext, Context, Global, Model, ReadGlobal, Task};
use project::Project;

use crate::manager::ServerCommand;

pub type ContextServerFactory = Arc<
    dyn Fn(Model<Project>, &AsyncAppContext) -> BoxFuture<Result<ServerCommand>>
        + Send
        + Sync
        + 'static,
>;

struct GlobalContextServerFactoryRegistry(Model<ContextServerFactoryRegistry>);

impl Global for GlobalContextServerFactoryRegistry {}

#[derive(Default)]
pub struct ContextServerFactoryRegistry {
    pub context_servers: BTreeMap<Arc<str>, ContextServerFactory>,
}

impl ContextServerFactoryRegistry {
    /// Returns the global [`ContextServerFactoryRegistry`].
    pub fn global(cx: &mut AppContext) -> Model<Self> {
        if !cx.has_global::<GlobalContextServerFactoryRegistry>() {
            let registry = cx.new_model(|_| ContextServerFactoryRegistry::new());
            cx.set_global(GlobalContextServerFactoryRegistry(registry));
        }
        GlobalContextServerFactoryRegistry::global(cx).0.clone()
    }

    pub fn new() -> Self {
        Self {
            context_servers: Default::default(),
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
