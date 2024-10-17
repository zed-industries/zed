use editor::Editor;
use gpui::{
    prelude::*, AnyElement, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView,
    Render, ScrollHandle, View, ViewContext, WeakView,
};
use settings::Settings;
use ui::{
    prelude::*, v_flex, Button, Color, Divider, Icon, IconButton, IconName, IconSize, IntoElement,
    Label, List, ListItem, Modal, ModalHeader, Section, SharedString, Tooltip,
};
use workspace::{ModalView, Workspace};

use crate::{
    jupyter_settings::{JupyterServer, JupyterSettingsContent},
    JupyterSettings,
};

gpui::actions!(repl, [ConnectJupyterServer]);
pub struct JupyterServers {
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    workspace: WeakView<Workspace>,
    server_list: Vec<JupyterServer>,
    new_server_url: View<Editor>,
    new_server_nickname: View<Editor>,
    mode: JupyterServerMode,
    selectable_items: SelectableItemList,
}

enum JupyterServerMode {
    Default,
    CreateServer,
    EditServer(usize),
}

struct SelectableItemList {
    items: Vec<Box<dyn Fn(&mut JupyterServers, &mut ViewContext<JupyterServers>)>>,
    active_item: Option<usize>,
}

impl SelectableItemList {
    fn next(&mut self, _cx: &mut ViewContext<JupyterServers>) {
        if let Some(active_item) = self.active_item.as_mut() {
            *active_item = (*active_item + 1) % self.items.len();
        } else if !self.items.is_empty() {
            self.active_item = Some(0);
        }
    }

    fn prev(&mut self, _cx: &mut ViewContext<JupyterServers>) {
        if let Some(active_item) = self.active_item.as_mut() {
            *active_item = (*active_item + self.items.len() - 1) % self.items.len();
        } else if !self.items.is_empty() {
            self.active_item = Some(self.items.len() - 1);
        }
    }
}

impl JupyterServers {
    fn add_server(&mut self, cx: &mut ViewContext<Self>) {
        let added_server = self.new_server_url.read(cx).text(cx);
        if !added_server.is_empty() {
            let nickname = self.new_server_nickname.read(cx).text(cx);

            let nickname = if nickname.is_empty() {
                None
            } else {
                Some(nickname.to_string())
            };

            let new_server = JupyterServer {
                url: added_server.clone(),
                nickname,
            };
            self.server_list.push(new_server);

            self.new_server_url.update(cx, |editor, cx| {
                editor.clear(cx);
            });

            self.new_server_nickname.update(cx, |editor, cx| {
                editor.clear(cx);
            });

            self.update_settings(cx);
        }
    }

    fn remove_server(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        self.server_list.remove(index);
        self.update_settings(cx);
    }

    fn update_settings(&self, cx: &mut ViewContext<Self>) {
        todo!();
    }

    pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        //
        workspace.register_action(|workspace, _: &ConnectJupyterServer, cx| {
            let handle = cx.view().downgrade();
            workspace.toggle_modal(cx, |cx| Self::new(cx, handle));
        });
    }
    pub fn new(cx: &mut ViewContext<Self>, workspace: WeakView<Workspace>) -> Self {
        let focus_handle = cx.focus_handle();

        let settings = JupyterSettings::get_global(cx);
        let server_list = settings.jupyter_servers.clone();

        let new_server_url = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Server URL", cx);
            editor
        });

        let new_server_nickname = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Nickname (optional)", cx);
            editor
        });

        Self {
            focus_handle,
            scroll_handle: ScrollHandle::new(),
            workspace,
            server_list,
            new_server_url,
            new_server_nickname,
            mode: JupyterServerMode::Default,
            selectable_items: SelectableItemList {
                items: Vec::new(),
                active_item: None,
            },
        }
    }

    fn connect_to_server(&self, server: JupyterServer, cx: &AppContext) {
        todo!()
    }

    fn next_item(&mut self, _: &menu::SelectNext, cx: &mut ViewContext<Self>) {
        if let JupyterServerMode::Default = self.mode {
            self.selectable_items.next(cx);
            cx.notify();
        }
    }

    fn prev_item(&mut self, _: &menu::SelectPrev, cx: &mut ViewContext<Self>) {
        if let JupyterServerMode::Default = self.mode {
            self.selectable_items.prev(cx);
            cx.notify();
        }
    }
    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        match self.mode {
            JupyterServerMode::Default => {
                if let Some(active_item) = self.selectable_items.active_item {
                    // if active_item < self.selectable_items.items.len() {
                    //     let item = self.selectable_items.items[active_item].clone();
                    //     item(self, cx);
                    // }
                }
            }
            JupyterServerMode::CreateServer => {
                self.add_server(cx);
                self.mode = JupyterServerMode::Default;
                cx.notify();
            }
            JupyterServerMode::EditServer(_) => {
                // TODO: Implement edit confirmation
                self.mode = JupyterServerMode::Default;
                cx.notify();
            }
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        match self.mode {
            JupyterServerMode::Default => cx.emit(DismissEvent),
            _ => {
                self.mode = JupyterServerMode::Default;
                cx.notify();
            }
        }
    }
}

impl ModalView for JupyterServers {}

impl EventEmitter<DismissEvent> for JupyterServers {}

impl FocusableView for JupyterServers {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
impl Render for JupyterServers {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .elevation_3(cx)
            .key_context("JupyterServersModal")
            .on_mouse_down_out(cx.listener(|_, _, cx| cx.emit(DismissEvent)))
            .child(
                Modal::new("jupyter-servers", Some(self.scroll_handle.clone()))
                    .header(ModalHeader::new().child(Label::new("Jupyter Servers")))
                    .child(
                        v_flex()
                            .size_full()
                            .p_4()
                            .border_1()
                            .border_color(cx.theme().colors().border_variant)
                            .child(Section::new().child(
                                List::new().empty_message("No servers added yet.").children(
                                    self.server_list.iter().enumerate().map(|(index, server)| {
                                        let name = server
                                            .nickname
                                            .clone()
                                            .unwrap_or_else(|| server.url.clone());

                                        ListItem::new(SharedString::from(format!(
                                            "jupyter-server-{name}"
                                        )))
                                        .inset(true)
                                        .spacing(ui::ListItemSpacing::Sparse)
                                        .start_slot(
                                            Icon::new(IconName::Server)
                                                .color(Color::Muted)
                                                .size(IconSize::Small),
                                        )
                                        .child(Label::new(name))
                                        .on_click(cx.listener(move |this, _, cx| {
                                            this.connect_to_server(
                                                this.server_list[index].clone(),
                                                cx,
                                            );
                                        }))
                                        .end_hover_slot::<AnyElement>(Some(
                                            h_flex()
                                                .gap_2()
                                                .child(
                                                    IconButton::new(
                                                        "edit-jupyter-server",
                                                        IconName::Settings,
                                                    )
                                                    .icon_size(IconSize::Small)
                                                    .on_click(cx.listener(move |this, _, cx| {
                                                        // TODO: Implement edit server logic
                                                        eprintln!(
                                                            "Editing server: {:?}",
                                                            this.server_list[index]
                                                        );
                                                    }))
                                                    .tooltip(|cx| Tooltip::text("Edit Server", cx)),
                                                )
                                                .child(
                                                    IconButton::new(
                                                        "remove-jupyter-server",
                                                        IconName::Trash,
                                                    )
                                                    .icon_size(IconSize::Small)
                                                    .on_click(cx.listener(move |this, _, cx| {
                                                        this.remove_server(index, cx);
                                                    }))
                                                    .tooltip(|cx| {
                                                        Tooltip::text("Delete Server", cx)
                                                    }),
                                                )
                                                .into_any_element(),
                                        ))
                                    }),
                                ),
                            ))
                            .child(
                                Section::new().child(
                                    v_flex()
                                        .gap_2()
                                        .child(self.new_server_url.clone())
                                        .child(self.new_server_nickname.clone())
                                        .child(Button::new("add-server", "Add Server").on_click(
                                            cx.listener(|this, _, cx| this.add_server(cx)),
                                        )),
                                ),
                            ),
                    ),
            )
    }
}
