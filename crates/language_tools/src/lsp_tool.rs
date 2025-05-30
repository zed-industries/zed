use std::sync::{Arc, Weak};

use client::proto;
use collections::HashSet;
use editor::{
    Editor,
    actions::{RestartLanguageServer, StopLanguageServer},
};
use gpui::{Corner, DismissEvent, Entity, Focusable, Subscription, Task, WeakEntity};
use language::CachedLspAdapter;
use lsp::{LanguageServer, LanguageServerName};
use picker::{Picker, PickerDelegate, popover_menu::PickerPopoverMenu};
use project::LspStore;
use ui::{Context, IconButtonShape, KeyBinding, Tooltip, Window, prelude::*};
use workspace::{StatusItemView, Workspace};

pub struct LspTool {
    lsp_picker: Entity<Picker<LspPickerDelegate>>,
    lsp_store: Entity<LspStore>,
    _subscrtiptions: Vec<Subscription>,
}

struct LspPickerDelegate {
    active_editor: Option<WeakEntity<Editor>>,
    // TODO kb remove, and only set the LSP items
    // based on the events subscribed for in LspTool
    lsp_store: Entity<LspStore>,
    applicable_language_servers: Vec<(Weak<CachedLspAdapter>, Weak<LanguageServer>)>,
    selected_index: usize,
    items: Vec<LspItem>,
}

enum LspItem {
    Header(LanguageServerName, Option<LspStatus>),
    Item(),
}

struct LspStatus {
    message: SharedString,
    status: i32,
}

impl PickerDelegate for LspPickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.items.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix;
        cx.notify();
    }

    fn update_matches(
        &mut self,
        _: String,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let Some(editor) = self
            .active_editor
            .as_ref()
            .and_then(|editor| editor.upgrade())
        else {
            return Task::ready(());
        };

        let buffers = editor.read(cx).buffer().read(cx).all_buffers();
        let mut server_ids = HashSet::default();
        let mut new_items = Vec::new();
        self.applicable_language_servers = self.lsp_store.update(cx, |lsp_store, cx| {
            buffers
                .iter()
                .flat_map(|buffer| {
                    buffer.update(cx, |buffer, cx| {
                        lsp_store
                            .language_servers_for_local_buffer(buffer, cx)
                            .filter(|(_, server)| server_ids.insert(server.server_id()))
                            .map(|(adapter, server)| {
                                // TODO kb fill it properly
                                new_items.push(LspItem::Header(adapter.name(), None));
                                new_items.push(LspItem::Item());
                                (Arc::downgrade(adapter), Arc::downgrade(server))
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .collect()
        });
        self.items = new_items;
        self.selected_index = 0;

        Task::ready(())
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        Arc::default()
    }

    fn confirm(&mut self, _: bool, _: &mut Window, _: &mut Context<Picker<Self>>) {}

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        _: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        Some(
            match self.items.get(ix)? {
                LspItem::Header(language_server_name, lsp_status) => v_flex()
                    .justify_center()
                    .child(
                        h_flex()
                            .w_full()
                            .justify_center()
                            .child(Label::new(language_server_name.0.clone()).color(Color::Muted)),
                    )
                    .when_some(lsp_status.as_ref(), |header, lsp_status| {
                        header.child(
                            Label::new(format!("TODO kb status: {}", lsp_status.message))
                                .color(Color::Warning),
                        )
                    }),
                LspItem::Item() => h_flex()
                    .gap_2()
                    .justify_between()
                    .child(Button::new("open-server-log", "Open Log").on_click(
                        move |_, window, cx| {
                            dbg!("open log");
                        },
                    ))
                    .child(Button::new("restart-server", "Restart").on_click(
                        move |_, window, cx| {
                            dbg!("restart");
                        },
                    ))
                    .child(Button::new("disable-server", "Disable").on_click(
                        move |_, window, cx| {
                            dbg!("disable");
                        },
                    )),
            }
            .into_any_element(),
        )
    }

    fn can_select(
        &mut self,
        _ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> bool {
        true
    }

    fn render_editor(
        &self,
        _: &Entity<Editor>,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Div {
        div()
    }

    fn render_footer(
        &self,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let editor = self.active_editor.clone()?;

        Some(
            h_flex()
                .w_full()
                .justify_between()
                .child(
                    Button::new("restart-all-servers", "Restart all servers")
                        .key_binding(KeyBinding::for_action(&RestartLanguageServer, window, cx))
                        .on_click({
                            let editor = editor.clone();
                            move |_, window, cx| {
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
                        }),
                )
                .child(
                    Button::new("stop-all-servers", "Stop all servers")
                        .key_binding(KeyBinding::for_action(&StopLanguageServer, window, cx))
                        .on_click({
                            let editor = editor.clone();
                            move |_, window, cx| {
                                editor
                                    .update(cx, |editor, cx| {
                                        editor.stop_language_server(
                                            &StopLanguageServer,
                                            window,
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                        }),
                )
                .into_any_element(),
        )
    }
}

impl LspTool {
    pub fn new(workspace: &Workspace, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let lsp_store = workspace.project().read(cx).lsp_store();
        let lsp_store_subscription = cx.subscribe_in(&lsp_store, window, |lsp_tool, lsp_store, e, window, cx| {
            let updated = match e {
                project::LspStoreEvent::LanguageServerAdded(
                    language_server_id,
                    language_server_name,
                    worktree_id,
                ) => {
                    dbg!((
                        "added",
                        language_server_id,
                        language_server_name,
                        worktree_id
                    ));
                    true
                }
                project::LspStoreEvent::LanguageServerRemoved(language_server_id) => {
                    dbg!(("removed", language_server_id));
                    true
                }
                project::LspStoreEvent::LanguageServerUpdate {
                    language_server_id,
                    name,
                    message: proto::update_language_server::Variant::StatusUpdate(status_update),
                } => {
                    dbg!((language_server_id, name, status_update));
                    true
                }
                // TODO kb events are sent twice
                project::LspStoreEvent::LanguageServerUpdate {
                    language_server_id,
                    name,
                    message: proto::update_language_server::Variant::RegisteredForBuffer(update),
                } => {
                    dbg!((language_server_id, name, update));
                    true
                }
                // TODO kb move custom r-a status thing here too
                _ => false,
            };

            if updated {
                lsp_tool.lsp_picker.update(cx, |lsp_picker, cx| {
                    lsp_picker.refresh(window, cx);
                })
            }
        });

        let delegate = LspPickerDelegate {
            active_editor: None,
            lsp_store: lsp_store.clone(),
            selected_index: 0,
            applicable_language_servers: Vec::new(),
            items: Vec::new(),
        };
        let lsp_picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

        Self {
            lsp_picker,
            lsp_store,
            _subscrtiptions: vec![lsp_store_subscription],
        }
    }
}

impl StatusItemView for LspTool {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn workspace::ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.lsp_picker.update(cx, |picker, cx| {
            picker.delegate.active_editor = active_pane_item
                .and_then(|item| item.downcast::<Editor>().map(|editor| editor.downgrade()));
            picker.delegate.applicable_language_servers.clear();
            picker.refresh(window, cx);
        });
    }
}

impl Render for LspTool {
    // TODO kb add a setting to remove this button out of the status bar
    // TODO kb add scrollbar + max width and height
    // TODO kb does not disappear when clicked
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        if self
            .lsp_picker
            .read(cx)
            .delegate
            .applicable_language_servers
            .is_empty()
        {
            return div();
        }

        div().child(
            PickerPopoverMenu::new(
                self.lsp_picker.clone(),
                IconButton::new("zed-lsp-tool-button", IconName::Bolt)
                    .shape(IconButtonShape::Square)
                    .icon_size(IconSize::XSmall)
                    .indicator_border_color(Some(cx.theme().colors().status_bar_background)),
                move |_, cx| Tooltip::simple("Language servers", cx),
                Corner::BottomRight,
                cx,
            )
            .render(window, cx),
        )
    }
}
