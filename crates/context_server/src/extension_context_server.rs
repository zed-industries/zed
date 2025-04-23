use std::sync::Arc;

use extension::{
    ContextServer, Extension, ExtensionContextServerProxy, ExtensionHostProxy, ProjectDelegate,
};
use gpui::{App, Entity};

use crate::{ContextServerFactoryRegistry, ServerCommand, manager::SetupInstructions};

struct ExtensionProject {
    worktree_ids: Vec<u64>,
}

impl ProjectDelegate for ExtensionProject {
    fn worktree_ids(&self) -> Vec<u64> {
        self.worktree_ids.clone()
    }
}

pub fn init(cx: &mut App) {
    let proxy = ExtensionHostProxy::default_global(cx);
    proxy.register_context_server_proxy(ContextServerFactoryRegistryProxy {
        context_server_factory_registry: ContextServerFactoryRegistry::global(cx),
    });
}

struct ContextServerFactoryRegistryProxy {
    context_server_factory_registry: Entity<ContextServerFactoryRegistry>,
}

impl ExtensionContextServerProxy for ContextServerFactoryRegistryProxy {
    fn register_context_server(
        &self,
        extension: Arc<dyn Extension>,
        context_server: ContextServer,
        cx: &mut App,
    ) {
        self.context_server_factory_registry
            .update(cx, |registry, _| {
                registry.register_server_factory(
                    context_server.id.clone(),
                    Arc::new({
                        move |project, cx| {
                            log::info!(
                                "loading command for context server {} from extension {}",
                                context_server.id,
                                extension.manifest().id
                            );

                            let id = context_server.id.clone();
                            let extension = extension.clone();
                            cx.spawn({
                                let setup_instructions = SetupInstructions {
                                    installation_instructions: context_server
                                        .setup
                                        .installation_instructions
                                        .clone(),
                                    settings: context_server.setup.settings_hint.clone(),
                                };
                                async move |cx| {
                                    let extension_project = project.update(cx, |project, cx| {
                                        Arc::new(ExtensionProject {
                                            worktree_ids: project
                                                .visible_worktrees(cx)
                                                .map(|worktree| worktree.read(cx).id().to_proto())
                                                .collect(),
                                        })
                                    })?;

                                    let mut command = extension
                                        .context_server_command(id.clone(), extension_project)
                                        .await?;
                                    command.command = extension
                                        .path_from_extension(command.command.as_ref())
                                        .to_string_lossy()
                                        .to_string();

                                    log::info!(
                                        "loaded command for context server {id}: {command:?}"
                                    );
                                    let command = ServerCommand {
                                        path: command.command,
                                        args: command.args,
                                        env: Some(command.env.into_iter().collect()),
                                    };
                                    Ok((command, setup_instructions.clone()))
                                }
                            })
                        }
                    }),
                )
            });
    }
}
