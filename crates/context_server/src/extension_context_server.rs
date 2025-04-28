use std::sync::Arc;

use anyhow::Result;
use extension::{
    ContextServerConfiguration, Extension, ExtensionContextServerProxy, ExtensionHostProxy,
    ProjectDelegate,
};
use gpui::{App, AsyncApp, Entity, Task};
use project::Project;

use crate::{ContextServerDescriptorRegistry, ServerCommand, registry};

pub fn init(cx: &mut App) {
    let proxy = ExtensionHostProxy::default_global(cx);
    proxy.register_context_server_proxy(ContextServerDescriptorRegistryProxy {
        context_server_factory_registry: ContextServerDescriptorRegistry::global(cx),
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

fn extension_project(project: Entity<Project>, cx: &mut AsyncApp) -> Result<Arc<ExtensionProject>> {
    project.update(cx, |project, cx| {
        Arc::new(ExtensionProject {
            worktree_ids: project
                .visible_worktrees(cx)
                .map(|worktree| worktree.read(cx).id().to_proto())
                .collect(),
        })
    })
}

impl registry::ContextServerDescriptor for ContextServerDescriptor {
    fn command(&self, project: Entity<Project>, cx: &AsyncApp) -> Task<Result<ServerCommand>> {
        let id = self.id.clone();
        let extension = self.extension.clone();
        cx.spawn(async move |cx| {
            let extension_project = extension_project(project, cx)?;
            let mut command = extension
                .context_server_command(id.clone(), extension_project.clone())
                .await?;
            command.command = extension
                .path_from_extension(command.command.as_ref())
                .to_string_lossy()
                .to_string();

            log::info!("loaded command for context server {id}: {command:?}");

            Ok(ServerCommand {
                path: command.command,
                args: command.args,
                env: Some(command.env.into_iter().collect()),
            })
        })
    }

    fn configuration(
        &self,
        project: Entity<Project>,
        cx: &AsyncApp,
    ) -> Task<Result<Option<ContextServerConfiguration>>> {
        let id = self.id.clone();
        let extension = self.extension.clone();
        cx.spawn(async move |cx| {
            let extension_project = extension_project(project, cx)?;
            let configuration = extension
                .context_server_configuration(id.clone(), extension_project)
                .await?;

            log::info!("loaded configration for context server {id}: {configuration:?}");

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
            .update(cx, |registry, _| {
                registry.register_context_server_descriptor(
                    id.clone(),
                    Arc::new(ContextServerDescriptor { id, extension })
                        as Arc<dyn registry::ContextServerDescriptor>,
                )
            });
    }
}
