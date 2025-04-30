use std::sync::Arc;

use anyhow::Result;
use collections::HashMap;
use extension::ContextServerConfiguration;
use gpui::{App, AppContext as _, AsyncApp, Entity, Global, ReadGlobal, Task};
use project::Project;

use crate::ServerCommand;

pub trait ContextServerDescriptor {
    fn command(&self, project: Entity<Project>, cx: &AsyncApp) -> Task<Result<ServerCommand>>;
    fn configuration(
        &self,
        project: Entity<Project>,
        cx: &AsyncApp,
    ) -> Task<Result<Option<ContextServerConfiguration>>>;
}

struct GlobalContextServerDescriptorRegistry(Entity<ContextServerDescriptorRegistry>);

impl Global for GlobalContextServerDescriptorRegistry {}

#[derive(Default)]
pub struct ContextServerDescriptorRegistry {
    context_servers: HashMap<Arc<str>, Arc<dyn ContextServerDescriptor>>,
}

impl ContextServerDescriptorRegistry {
    /// Returns the global [`ContextServerDescriptorRegistry`].
    pub fn global(cx: &App) -> Entity<Self> {
        GlobalContextServerDescriptorRegistry::global(cx).0.clone()
    }

    /// Returns the global [`ContextServerDescriptorRegistry`].
    ///
    /// Inserts a default [`ContextServerDescriptorRegistry`] if one does not yet exist.
    pub fn default_global(cx: &mut App) -> Entity<Self> {
        if !cx.has_global::<GlobalContextServerDescriptorRegistry>() {
            let registry = cx.new(|_| Self::new());
            cx.set_global(GlobalContextServerDescriptorRegistry(registry));
        }
        cx.global::<GlobalContextServerDescriptorRegistry>()
            .0
            .clone()
    }

    pub fn new() -> Self {
        Self {
            context_servers: HashMap::default(),
        }
    }

    pub fn context_server_descriptors(&self) -> Vec<(Arc<str>, Arc<dyn ContextServerDescriptor>)> {
        self.context_servers
            .iter()
            .map(|(id, factory)| (id.clone(), factory.clone()))
            .collect()
    }

    pub fn context_server_descriptor(&self, id: &str) -> Option<Arc<dyn ContextServerDescriptor>> {
        self.context_servers.get(id).cloned()
    }

    /// Registers the provided [`ContextServerDescriptor`].
    pub fn register_context_server_descriptor(
        &mut self,
        id: Arc<str>,
        descriptor: Arc<dyn ContextServerDescriptor>,
    ) {
        self.context_servers.insert(id, descriptor);
    }

    /// Unregisters the [`ContextServerDescriptor`] for the server with the given ID.
    pub fn unregister_context_server_descriptor_by_id(&mut self, server_id: &str) {
        self.context_servers.remove(server_id);
    }
}
