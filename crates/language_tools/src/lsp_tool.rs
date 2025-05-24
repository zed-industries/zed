use std::sync::Arc;

use collections::HashSet;
use editor::{
    Editor,
    actions::{RestartLanguageServer, StopLanguageServer},
};
use gpui::{Corner, Entity, WeakEntity};
use language::CachedLspAdapter;
use lsp::LanguageServer;
use project::LspStore;
use ui::{ContextMenu, IconButtonShape, PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*};
use workspace::{StatusItemView, Workspace};

pub struct LspTool {
    active_editor: Option<WeakEntity<Editor>>,
    lsp_store: Entity<LspStore>,
    popover_menu_handle: PopoverMenuHandle<ContextMenu>,
}

impl LspTool {
    pub fn new(
        popover_menu_handle: PopoverMenuHandle<ContextMenu>,
        workspace: &Workspace,
        cx: &App,
    ) -> Self {
        let lsp_store = workspace.project().read(cx).lsp_store();
        Self {
            active_editor: None,
            popover_menu_handle,
            lsp_store,
        }
    }

    fn build_lsp_context_menu(
        &self,
        editor: WeakEntity<Editor>,
        applicable_language_servers: &[(Arc<CachedLspAdapter>, Arc<LanguageServer>)],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        ContextMenu::build(window, cx, move |menu, _, cx| {
            menu.separator()
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
                .entry(
                    "Stop all servers",
                    Some(Box::new(StopLanguageServer)),
                    move |window, cx| {
                        editor
                            .update(cx, |editor, cx| {
                                // TODO kb this will make the button disappear.
                                // We need a better method to get "all language servers and statuses"
                                editor.stop_language_server(&StopLanguageServer, window, cx);
                            })
                            .ok();
                    },
                )
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
                            .map(|(adapter, server)| (adapter.clone(), server.clone()))
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

        let popover_menu = PopoverMenu::new("lsp_servers")
            .menu(move |window, cx| {
                Some(lsp_tool.update(cx, |lsp_tool, cx| {
                    lsp_tool.build_lsp_context_menu(
                        editor.downgrade(),
                        &applicable_language_servers,
                        window,
                        cx,
                    )
                }))
            })
            .anchor(Corner::BottomRight)
            .with_handle(self.popover_menu_handle.clone())
            .trigger(icon_button);

        div().child(popover_menu.into_any_element())
    }
}
