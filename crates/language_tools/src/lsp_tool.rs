use std::sync::{Arc, Weak};

use client::proto;
use collections::HashSet;
use editor::{
    Editor,
    actions::{RestartLanguageServer, StopLanguageServer},
};
use gpui::{Corner, Entity, Subscription, WeakEntity};
use language::CachedLspAdapter;
use lsp::{LanguageServer, LanguageServerName};
use project::LspStore;
use ui::{ContextMenu, IconButtonShape, PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*};
use workspace::{StatusItemView, Workspace};

pub struct LspTool {
    active_editor: Option<WeakEntity<Editor>>,
    lsp_store: Entity<LspStore>,
    popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    selected_language_server: Option<(
        LanguageServerName,
        Weak<CachedLspAdapter>,
        Weak<LanguageServer>,
    )>,
    _subscrtiptions: Vec<Subscription>,
}

impl LspTool {
    pub fn new(
        popover_menu_handle: PopoverMenuHandle<ContextMenu>,
        workspace: &Workspace,
        cx: &mut Context<Self>,
    ) -> Self {
        let lsp_store = workspace.project().read(cx).lsp_store();
        let lsp_store_subscription = cx.subscribe(&lsp_store, |lsp_tool, lsp_store, e, cx| {
            match e {
                project::LspStoreEvent::LanguageServerAdded(
                    language_server_id,
                    language_server_name,
                    worktree_id,
                ) => {
                    //
                }
                project::LspStoreEvent::LanguageServerRemoved(language_server_id) => {
                    //
                }
                project::LspStoreEvent::LanguageServerUpdate {
                    language_server_id,
                    name,
                    message: proto::update_language_server::Variant::StatusUpdate(status_update),
                } => {
                    //
                    dbg!(status_update);
                }
                project::LspStoreEvent::LanguageServerUpdate {
                    language_server_id,
                    name,
                    message: proto::update_language_server::Variant::AssociationUpdate(association_update),
                } => {
                    //
                    dbg!(association_update);
                }
                project::LspStoreEvent::LanguageServerLog(
                    language_server_id,
                    language_server_log_type,
                    _,
                ) => {
                    //
                }
                project::LspStoreEvent::LanguageServerPrompt(language_server_prompt_request) => {
                    //
                }
                project::LspStoreEvent::Notification(_) => {
                    //
                }
                _ => {}
            }
        });

        Self {
            lsp_store,
            popover_menu_handle,
            active_editor: None,
            selected_language_server: None,
            _subscrtiptions: vec![lsp_store_subscription],
        }
    }

    fn build_language_servers_list(
        &self,
        editor: WeakEntity<Editor>,
        applicable_language_servers: Arc<Vec<(Weak<CachedLspAdapter>, Weak<LanguageServer>)>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let lsp_tool = cx.entity();
        let selected_language_server = self.selected_language_server.clone();
        dbg!(selected_language_server.is_some());

        ContextMenu::build_persistent(window, cx, move |mut menu, _, cx| {
            if let Some((selected_adapter, selected_server)) = selected_language_server
                .as_ref()
                .and_then(|(_, adapter, server)| Some((adapter.upgrade()?, server.upgrade()?)))
            {
                let editor = editor.clone();
                menu = menu
                    .header(selected_adapter.name.0.clone())
                    .custom_row(|_, _| Label::new("Status: TODO kb").into_any_element())
                    .separator()
                    .entry("Open log", None, {
                        let editor = editor.clone();
                        move |_, _| {
                            dbg!("open log");
                        }
                    })
                    .separator()
                    .entry("Restart", None, {
                        let editor = editor.clone();
                        move |_, _| {
                            dbg!("Restart");
                        }
                    })
                    .entry("Disable", None, {
                        let editor = editor.clone();
                        move |_, _| {
                            dbg!("Disable");
                        }
                    })
                    .separator()
                    .separator()
            }

            for (adapter, server) in applicable_language_servers.iter().cloned() {
                let Some(upgraded_adapter) = adapter.upgrade() else {
                    continue;
                };
                let server_name = upgraded_adapter.name();
                let context_menu = cx.entity();
                menu = menu.custom_entry(
                    {
                        let server_name = server_name.clone();
                        let selected_language_server = selected_language_server.clone();
                        move |_, cx| {
                            h_flex()
                                .when(
                                    Some(&server_name)
                                        == selected_language_server.as_ref().map(
                                            |(selected_server_name, _, _)| selected_server_name,
                                        ),
                                    |entry| entry.bg(cx.theme().colors().element_hover),
                                )
                                .child(Label::new(server_name.0.clone()))
                                .child(IconButton::new("server-details", IconName::ChevronRight))
                                .w_full()
                                .justify_between()
                                .into_any_element()
                        }
                    },
                    {
                        let lsp_tool = lsp_tool.clone();
                        let server_name = server_name.clone();
                        let context_menu = context_menu.clone();
                        move |window, cx| {
                            let adapter = adapter.clone();
                            let server = server.clone();
                            let server_name = server_name.clone();
                            let context_menu = context_menu.clone();
                            lsp_tool.update(cx, move |lsp_tool, cx| {
                                if lsp_tool
                                    .selected_language_server
                                    .as_ref()
                                    .map(|(selected_server_name, _, _)| selected_server_name)
                                    == Some(&server_name)
                                {
                                    lsp_tool.selected_language_server = None;
                                } else {
                                    lsp_tool.selected_language_server =
                                        Some((server_name, adapter, server));
                                }

                                // TODO kb why does it not work
                                context_menu.update(cx, |context_menu, cx| {
                                    context_menu.rebuild(window, cx);
                                    cx.notify();
                                });
                                cx.notify();
                            });
                        }
                    },
                );
            }

            menu.keep_open_on_confirm(true)
                .separator()
                .entry(
                    "Restart all servers",
                    Some(Box::new(RestartLanguageServer)),
                    {
                        let editor = editor.clone();
                        move |window, cx| {
                            editor
                                .update(cx, |editor, cx| {
                                    editor.restart_language_server(
                                        &RestartLanguageServer,
                                        window,
                                        cx,
                                    );
                                })
                                .ok();
                        }
                    },
                )
                .entry("Stop all servers", Some(Box::new(StopLanguageServer)), {
                    let editor = editor.clone();
                    move |window, cx| {
                        editor
                            .update(cx, |editor, cx| {
                                // TODO kb this will make the button disappear.
                                // We need a better method to get "all language servers and statuses"
                                editor.stop_language_server(&StopLanguageServer, window, cx);
                            })
                            .ok();
                    }
                })
        })
    }
}

impl StatusItemView for LspTool {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn workspace::ItemHandle>,
        _window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) {
        self.active_editor = active_pane_item
            .and_then(|item| item.downcast::<Editor>().map(|editor| editor.downgrade()));
        cx.notify();
    }
}

impl Render for LspTool {
    // TODO kb won't work for remote clients for now
    // TODO kb add a setting to remove this button out of the status bar
    fn render(
        &mut self,
        _window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        let Some(editor) = self
            .active_editor
            .as_ref()
            .and_then(|editor| editor.upgrade())
        else {
            return div();
        };

        let buffers = editor.read(cx).buffer().read(cx).all_buffers();
        let mut server_ids = HashSet::default();
        let applicable_language_servers = Arc::new(self.lsp_store.update(cx, |lsp_store, cx| {
            buffers
                .iter()
                .flat_map(|buffer| {
                    buffer.update(cx, |buffer, cx| {
                        lsp_store
                            .language_servers_for_local_buffer(buffer, cx)
                            .filter(|(_, server)| server_ids.insert(server.server_id()))
                            .map(|(adapter, server)| {
                                (Arc::downgrade(adapter), Arc::downgrade(server))
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .collect::<Vec<_>>()
        }));
        if applicable_language_servers.is_empty() {
            return div();
        }

        let icon_button = IconButton::new("zed-lsp-tool-button", IconName::Bolt)
            .shape(IconButtonShape::Square)
            .icon_size(IconSize::XSmall)
            .indicator_border_color(Some(cx.theme().colors().status_bar_background))
            .tooltip(move |_, cx| Tooltip::simple("Language servers", cx));

        let lsp_tool = cx.entity().clone();
        let language_servers_list = PopoverMenu::new("language_servers")
            .menu(move |window, cx| {
                Some(lsp_tool.update(cx, |lsp_tool, cx| {
                    lsp_tool.build_language_servers_list(
                        editor.downgrade(),
                        applicable_language_servers.clone(),
                        window,
                        cx,
                    )
                }))
            })
            .anchor(Corner::BottomRight)
            .with_handle(self.popover_menu_handle.clone())
            .trigger(icon_button);

        div().child(language_servers_list)
    }
}
