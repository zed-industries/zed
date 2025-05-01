use std::sync::Arc;

use anyhow::Context as _;
use context_server::ContextServerDescriptorRegistry;
use extension::ExtensionManifest;
use language::LanguageRegistry;
use ui::prelude::*;
use util::ResultExt;
use workspace::Workspace;

use crate::{AssistantPanel, assistant_configuration::ConfigureContextServerModal};

pub(crate) fn init(language_registry: Arc<LanguageRegistry>, cx: &mut App) {
    cx.observe_new(move |_: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        if let Some(extension_events) = extension::ExtensionEvents::try_global(cx).as_ref() {
            cx.subscribe_in(extension_events, window, {
                let language_registry = language_registry.clone();
                move |workspace, _, event, window, cx| match event {
                    extension::Event::ExtensionInstalled(manifest) => {
                        show_configure_mcp_modal(
                            language_registry.clone(),
                            manifest,
                            workspace,
                            window,
                            cx,
                        );
                    }
                    extension::Event::ConfigureExtensionRequested(manifest) => {
                        if !manifest.context_servers.is_empty() {
                            show_configure_mcp_modal(
                                language_registry.clone(),
                                manifest,
                                workspace,
                                window,
                                cx,
                            );
                        }
                    }
                    _ => {}
                }
            })
            .detach();
        } else {
            log::info!(
                "No extension events global found. Skipping context server configuration wizard"
            );
        }
    })
    .detach();
}

fn show_configure_mcp_modal(
    language_registry: Arc<LanguageRegistry>,
    manifest: &Arc<ExtensionManifest>,
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<'_, Workspace>,
) {
    let Some(context_server_manager) = workspace.panel::<AssistantPanel>(cx).map(|panel| {
        panel
            .read(cx)
            .thread_store()
            .read(cx)
            .context_server_manager()
    }) else {
        return;
    };

    let registry = ContextServerDescriptorRegistry::global(cx).read(cx);
    let project = workspace.project().clone();
    let configuration_tasks = manifest
        .context_servers
        .keys()
        .cloned()
        .filter_map({
            |key| {
                let descriptor = registry.context_server_descriptor(&key)?;
                Some(cx.spawn({
                    let project = project.clone();
                    async move |_, cx| {
                        descriptor
                            .configuration(project, &cx)
                            .await
                            .context("Failed to resolve context server configuration")
                            .log_err()
                            .flatten()
                            .map(|config| (key, config))
                    }
                }))
            }
        })
        .collect::<Vec<_>>();

    let jsonc_language = language_registry.language_for_name("jsonc");

    cx.spawn_in(window, async move |this, cx| {
        let descriptors = futures::future::join_all(configuration_tasks).await;
        let jsonc_language = jsonc_language.await.ok();

        this.update_in(cx, |this, window, cx| {
            let modal = ConfigureContextServerModal::new(
                descriptors.into_iter().flatten(),
                jsonc_language,
                context_server_manager,
                language_registry,
                cx.entity().downgrade(),
                window,
                cx,
            );
            if let Some(modal) = modal {
                this.toggle_modal(window, cx, |_, _| modal);
            }
        })
    })
    .detach();
}
