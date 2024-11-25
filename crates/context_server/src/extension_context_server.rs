use std::sync::Arc;

use extension::{Extension, ExtensionContextServerProxy, ExtensionHostProxy, ProjectDelegate};
use gpui::{AppContext, Model};

use crate::{ContextServerFactoryRegistry, ServerCommand};

struct ExtensionProject {
    worktree_ids: Vec<u64>,
}

impl ProjectDelegate for ExtensionProject {
    fn worktree_ids(&self) -> Vec<u64> {
        self.worktree_ids.clone()
    }
}

pub fn init(cx: &mut AppContext) {
    let proxy = ExtensionHostProxy::default_global(cx);
    proxy.register_context_server_proxy(ContextServerFactoryRegistryProxy {
        context_server_factory_registry: ContextServerFactoryRegistry::global(cx),
    });
}

struct ContextServerFactoryRegistryProxy {
    context_server_factory_registry: Model<ContextServerFactoryRegistry>,
}

impl ExtensionContextServerProxy for ContextServerFactoryRegistryProxy {
    fn register_context_server(
        &self,
        extension: Arc<dyn Extension>,
        id: Arc<str>,
        cx: &mut AppContext,
    ) {
        self.context_server_factory_registry
            .update(cx, |registry, _| {
                registry.register_server_factory(
                    id.clone(),
                    Arc::new({
                        move |project, cx| {
                            log::info!(
                                "loading command for context server {id} from extension {}",
                                extension.manifest().id
                            );

                            let id = id.clone();
                            let extension = extension.clone();
                            cx.spawn(|mut cx| async move {
                                let extension_project =
                                    project.update(&mut cx, |project, cx| {
                                        Arc::new(ExtensionProject {
                                            worktree_ids: project
                                                .visible_worktrees(cx)
                                                .map(|worktree| worktree.read(cx).id().to_proto())
                                                .collect(),
                                        })
                                    })?;

                                let command = extension
                                    .context_server_command(id.clone(), extension_project)
                                    .await?;

                                log::info!("loaded command for context server {id}: {command:?}");

                                Ok(ServerCommand {
                                    path: command.command,
                                    args: command.args,
                                    env: Some(command.env.into_iter().collect()),
                                })
                            })
                        }
                    }),
                )
            });
    }
}
