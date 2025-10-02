use std::sync::Arc;

use context_server::ContextServerId;
use extension::ExtensionManifest;
use fs::Fs;
use gpui::WeakEntity;
use language::LanguageRegistry;
use settings::update_settings_file;
use ui::prelude::*;
use util::ResultExt;
use workspace::Workspace;

use crate::agent_configuration::ConfigureContextServerModal;

pub(crate) fn init(language_registry: Arc<LanguageRegistry>, fs: Arc<dyn Fs>, cx: &mut App) {
    cx.observe_new(move |_: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        if let Some(extension_events) = extension::ExtensionEvents::try_global(cx).as_ref() {
            cx.subscribe_in(extension_events, window, {
                let language_registry = language_registry.clone();
                let fs = fs.clone();
                move |_, _, event, window, cx| match event {
                    extension::Event::ExtensionInstalled(manifest) => {
                        show_configure_mcp_modal(
                            language_registry.clone(),
                            manifest,
                            cx.weak_entity(),
                            window,
                            cx,
                        );
                    }
                    extension::Event::ExtensionUninstalled(manifest) => {
                        remove_context_server_settings(
                            manifest.context_servers.keys().cloned().collect(),
                            fs.clone(),
                            cx,
                        );
                    }
                    extension::Event::ConfigureExtensionRequested(manifest) => {
                        if !manifest.context_servers.is_empty() {
                            show_configure_mcp_modal(
                                language_registry.clone(),
                                manifest,
                                cx.weak_entity(),
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

fn remove_context_server_settings(
    context_server_ids: Vec<Arc<str>>,
    fs: Arc<dyn Fs>,
    cx: &mut App,
) {
    update_settings_file(fs, cx, move |settings, _| {
        settings
            .project
            .context_servers
            .retain(|server_id, _| !context_server_ids.contains(server_id));
    });
}

fn show_configure_mcp_modal(
    language_registry: Arc<LanguageRegistry>,
    manifest: &Arc<ExtensionManifest>,
    workspace: WeakEntity<Workspace>,
    window: &mut Window,
    cx: &mut Context<'_, Workspace>,
) {
    if !window.is_window_active() {
        return;
    }

    let ids = manifest.context_servers.keys().cloned().collect::<Vec<_>>();
    if ids.is_empty() {
        return;
    }

    window
        .spawn(cx, async move |cx| {
            for id in ids {
                let Some(task) = cx
                    .update(|window, cx| {
                        ConfigureContextServerModal::show_modal_for_existing_server(
                            ContextServerId(id.clone()),
                            language_registry.clone(),
                            workspace.clone(),
                            window,
                            cx,
                        )
                    })
                    .ok()
                else {
                    continue;
                };
                task.await.log_err();
            }
        })
        .detach();
}
