use std::sync::Arc;

use anyhow::Result;
use context_server::ContextServerCommand;
use extension::{
    ContextServerConfiguration, Extension, ExtensionContextServerProxy, ExtensionHostProxy,
    ProjectDelegate,
};
use gpui::{App, AsyncApp, Entity, Task};
use postage::{stream::Stream as _, watch};

use crate::binary_downloads::BinaryDownloads;
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
    Ok(worktree_store.update(cx, |worktree_store, cx| {
        Arc::new(ExtensionProject {
            worktree_ids: worktree_store
                .visible_worktrees(cx)
                .map(|worktree| worktree.read(cx).id().to_proto())
                .collect(),
        })
    }))
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
            let extension_project = extension_project(worktree_store.clone(), cx)?;
            let waits = cx.update(|cx| {
                wait_until_downloads_allowed_for_all_visible_worktrees(&worktree_store, cx)
            });
            await_downloads_allowed(waits, &id).await;
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

/// Collects a wait channel for every visible worktree that currently disallows
/// binary downloads, plus the global default when there are no worktrees. The
/// `command` task awaits all of them so an extension-driven context server
/// downloads only after every relevant scope has been approved.
fn wait_until_downloads_allowed_for_all_visible_worktrees(
    worktree_store: &Entity<WorktreeStore>,
    cx: &mut App,
) -> Vec<watch::Receiver<bool>> {
    let Some(binary_downloads) = BinaryDownloads::try_get_global(cx) else {
        return Vec::new();
    };
    let worktree_ids: Vec<_> = worktree_store
        .read(cx)
        .visible_worktrees(cx)
        .map(|worktree| worktree.read(cx).id())
        .collect();
    binary_downloads.update(cx, |binary_downloads, cx| {
        if worktree_ids.is_empty() {
            return binary_downloads
                .wait_until_allowed(None, cx)
                .into_iter()
                .collect();
        }
        worktree_ids
            .into_iter()
            .filter_map(|worktree_id| binary_downloads.wait_until_allowed(Some(worktree_id), cx))
            .collect()
    })
}

async fn await_downloads_allowed(waits: Vec<watch::Receiver<bool>>, server_id: &str) {
    for mut wait in waits {
        if *wait.borrow() {
            continue;
        }
        log::info!(
            "Waiting for binary downloads approval before starting context server {server_id}"
        );
        while let Some(allowed) = wait.recv().await {
            if allowed {
                break;
            }
        }
        log::info!("Binary downloads allowed, starting context server {server_id}");
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
