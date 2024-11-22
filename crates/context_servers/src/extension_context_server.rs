use std::sync::Arc;

use extension::{
    Extension, ExtensionChangeListeners, OnContextServerExtensionChange, ProjectDelegate,
};
use gpui::{AppContext, Model};

use crate::manager::ServerCommand;
use crate::ContextServerFactoryRegistry;

struct ExtensionProject {
    worktree_ids: Vec<u64>,
}

impl ProjectDelegate for ExtensionProject {
    fn worktree_ids(&self) -> Vec<u64> {
        self.worktree_ids.clone()
    }
}

pub fn init(cx: &AppContext) {
    let extension_change_listeners = ExtensionChangeListeners::global(cx);
    extension_change_listeners.register_context_server_listener(
        ExtensionIndexedDocsProviderListener {
            context_server_factory_registry: ContextServerFactoryRegistry::global(cx),
        },
    );
}

struct ExtensionIndexedDocsProviderListener {
    context_server_factory_registry: Model<ContextServerFactoryRegistry>,
}

impl OnContextServerExtensionChange for ExtensionIndexedDocsProviderListener {
    fn register(&self, extension: Arc<dyn Extension>, id: Arc<str>, cx: &mut AppContext) {
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
