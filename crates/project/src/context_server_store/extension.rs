use std::sync::Arc;
use std::path::Path;

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
            command.command = extension
                .path_from_extension(command.command.as_ref())
                .to_string_lossy()
                .to_string();

            // Only resolve relative paths through the extension.
            // This fixes a Windows issue where absolute paths (e.g., "C:\Users\...")
            // were being incorrectly joined with the extension's work directory,
            // resulting in invalid paths like "C:\C:\Users\...".
            let command_path = Path::new(&command.command);
            command.command = if command_path.is_absolute() {
                command.command
            } else {
                extension
                    .path_from_extension(command_path)
                    .to_string_lossy()
                    .to_string()
            };

            // Process arguments to resolve any paths.
            // Some extensions return Unix-style paths (e.g., "/C:/Users/...")
            // which need to be converted to proper Windows paths.
            let args: Vec<String> = command
                .args
                .iter()
                .map(|arg| {
                    // Check if the argument looks like a path
                    let arg_path = Path::new(arg);

                    // Handle Unix-style absolute paths on Windows (e.g., "/C:/...")
                    if cfg!(windows) && arg.starts_with('/') && arg.len() > 2 && arg.chars().nth(2) == Some(':') {
                        // Remove the leading slash for Windows absolute paths
                        let cleaned_arg = &arg[1..];
                        let cleaned_path = Path::new(cleaned_arg);

                        if cleaned_path.is_absolute() {
                            // Convert forward slashes to backslashes for Windows
                            cleaned_arg.replace('/', "\\")
                        } else {
                            extension
                                .path_from_extension(cleaned_path)
                                .to_string_lossy()
                                .replace('/', "\\")
                        }
                    } else if arg_path.is_absolute() {
                        // Already an absolute path, use as-is
                        arg.to_string()
                    } else if arg.contains('/') || arg.contains('\\') {
                        // Looks like a relative path, resolve it
                        extension
                            .path_from_extension(arg_path)
                            .to_string_lossy()
                            .to_string()
                    } else {
                        // Not a path, just a regular argument
                        arg.to_string()
                    }
                })
                .collect();

            log::info!("loaded command for context server {id}: {command:?}");

            Ok(ContextServerCommand {
                path: command.command,
                args: command.args,
                args,
                env: Some(command.env.into_iter().collect()),
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
