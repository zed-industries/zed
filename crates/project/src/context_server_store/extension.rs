use std::sync::Arc;

use anyhow::Result;
use context_server::ContextServerCommand;
use extension::{
    ContextServerConfiguration, Extension, ExtensionContextServerProxy, ExtensionHostProxy,
    ProjectDelegate,
};
use gpui::{App, AsyncApp, Entity, Task};

use crate::worktree_store::WorktreeStore;

use super::registry::{self, ContextServerDescriptorRegistry};

pub fn init(cx: &mut App) {
    let proxy = ExtensionHostProxy::default_global(cx);
    proxy.register_context_server_proxy(ContextServerDescriptorRegistryProxy {
        context_server_factory_registry: ContextServerDescriptorRegistry::default_global(cx),
    });
}

struct ExtensionProject {
    worktree_ids: Vec<u64>,
}

impl ProjectDelegate for ExtensionProject {
    fn worktree_ids(&self) -> Vec<u64> {
        self.worktree_ids.clone()
    }
}

struct ContextServerDescriptor {
    id: Arc<str>,
    extension: Arc<dyn Extension>,
}

fn extension_project(
    worktree_store: Entity<WorktreeStore>,
    cx: &mut AsyncApp,
) -> Result<Arc<ExtensionProject>> {
    worktree_store.update(cx, |worktree_store, cx| {
        Arc::new(ExtensionProject {
            worktree_ids: worktree_store
                .visible_worktrees(cx)
                .map(|worktree| worktree.read(cx).id().to_proto())
                .collect(),
        })
    })
}

impl registry::ContextServerDescriptor for ContextServerDescriptor {
    fn command(
        &self,
        worktree_store: Entity<WorktreeStore>,
        cx: &AsyncApp,
    ) -> Task<Result<ContextServerCommand>> {
        let id = self.id.clone();
        let extension = self.extension.clone();
        cx.spawn(async move |cx| {
            let extension_project = extension_project(worktree_store, cx)?;
            let mut command = extension
                .context_server_command(id.clone(), extension_project.clone())
                .await?;
            command.command = extension.path_from_extension(&command.command);

            log::debug!("loaded command for context server {id}: {command:?}");

            Ok(ContextServerCommand {
                path: command.command,
                args: command.args,
                env: Some(command.env.into_iter().collect()),
                timeout: None,
            })
        })
    }

    fn configuration(
        &self,
        worktree_store: Entity<WorktreeStore>,
        cx: &AsyncApp,
    ) -> Task<Result<Option<ContextServerConfiguration>>> {
        let id = self.id.clone();
        let extension = self.extension.clone();
        cx.spawn(async move |cx| {
            let extension_project = extension_project(worktree_store, cx)?;
            let configuration = extension
                .context_server_configuration(id.clone(), extension_project)
                .await?;

            log::debug!("loaded configuration for context server {id}: {configuration:?}");

            Ok(configuration)
        })
    }
}

struct ContextServerDescriptorRegistryProxy {
    context_server_factory_registry: Entity<ContextServerDescriptorRegistry>,
}

impl ExtensionContextServerProxy for ContextServerDescriptorRegistryProxy {
    fn register_context_server(&self, extension: Arc<dyn Extension>, id: Arc<str>, cx: &mut App) {
        self.context_server_factory_registry
            .update(cx, |registry, cx| {
                registry.register_context_server_descriptor(
                    id.clone(),
                    Arc::new(ContextServerDescriptor { id, extension })
                        as Arc<dyn registry::ContextServerDescriptor>,
                    cx,
                )
            });
    }

    fn unregister_context_server(&self, server_id: Arc<str>, cx: &mut App) {
        self.context_server_factory_registry
            .update(cx, |registry, cx| {
                registry.unregister_context_server_descriptor_by_id(&server_id, cx)
            });
    }
}
