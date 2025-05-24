use collections::HashSet;
use editor::Editor;
use gpui::{Entity, WeakEntity};
use project::LspStore;
use ui::{IconButtonShape, Tooltip, prelude::*};
use workspace::{StatusItemView, Workspace};

pub struct LspTool {
    active_editor: Option<WeakEntity<Editor>>,
    lsp_store: Entity<LspStore>,
}

impl LspTool {
    pub fn new(workspace: &Workspace, cx: &App) -> Self {
        let lsp_store = workspace.project().read(cx).lsp_store();
        Self {
            active_editor: None,
            lsp_store,
        }
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
        let applicable_language_servers = self.lsp_store.update(cx, |lsp_store, cx| {
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
        });
        dbg!(applicable_language_servers.len());
        if applicable_language_servers.is_empty() {
            return div();
        }

        div().child(
            IconButton::new("zed-lsp-tool-button", IconName::Bolt)
                .shape(IconButtonShape::Square)
                .icon_size(IconSize::XSmall)
                .indicator_border_color(Some(cx.theme().colors().status_bar_background))
                .tooltip(move |_, cx| Tooltip::simple("Language servers", cx))
                .on_click(cx.listener(move |_, _, _window, _cx| {
                    dbg!("????????");
                })),
        )
    }
}
