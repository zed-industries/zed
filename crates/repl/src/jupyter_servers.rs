use gpui::{
    prelude::*, AnyElement, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView,
    Refineable, Render, ScrollHandle, ViewContext, WeakView,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use ui::{
    prelude::*, v_flex, ActiveTheme, Color, Divider, Icon, IconButton, IconName, IconSize,
    IntoElement, Label, ListItem, SharedString, Tooltip,
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
    new_server_url: String,
    new_server_nickname: String,
}

impl JupyterServers {
    fn add_server(&mut self, cx: &mut ViewContext<Self>) {
        if !self.new_server_url.is_empty() {
            let new_server = JupyterServer {
                url: self.new_server_url.clone(),
                nickname: if self.new_server_nickname.is_empty() {
                    None
                } else {
                    Some(self.new_server_nickname.clone())
                },
            };
            self.server_list.push(new_server);
            self.new_server_url.clear();
            self.new_server_nickname.clear();
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

        Self {
            focus_handle,
            scroll_handle: ScrollHandle::new(),
            workspace,
            server_list,
            new_server_url: String::new(),
            new_server_nickname: String::new(),
        }
    }

    fn connect_to_server(&self, server: JupyterServer, cx: &AppContext) {
        todo!()
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
        v_flex()
            .gap_3()
            .p_4()
            .child(Label::new("Jupyter Servers"))
            .child(
                v_flex()
                    .gap_2()
                    .children(self.server_list.iter().enumerate().map(|(index, server)| {
                        let name = server
                            .nickname
                            .clone()
                            .unwrap_or_else(|| server.url.clone());

                        ListItem::new(SharedString::from(format!("jupyter-server-{name}")))
                            .inset(true)
                            .spacing(ui::ListItemSpacing::Sparse)
                            .start_slot(
                                Icon::new(IconName::Server)
                                    .color(Color::Muted)
                                    .size(IconSize::Small),
                            )
                            .child(Label::new(name))
                            .on_click(cx.listener(move |this, _, cx| {
                                this.connect_to_server(this.server_list[index].clone(), cx);
                            }))
                            .end_hover_slot::<AnyElement>(Some(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        IconButton::new("edit-jupyter-server", IconName::Settings)
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
                                        IconButton::new("remove-jupyter-server", IconName::Trash)
                                            .icon_size(IconSize::Small)
                                            .on_click(cx.listener(move |this, _, cx| {
                                                this.remove_server(index, cx);
                                            }))
                                            .tooltip(|cx| Tooltip::text("Delete Server", cx)),
                                    )
                                    .into_any_element(),
                            ))
                    })),
            )
            .child(Divider::horizontal())
            .child(
                v_flex()
                    .gap_2()
                    .child(Label::new("Add New Server"))
                    // .child(
                    //     TextInput::new("new-server-url")
                    //         .placeholder("Server URL")
                    //         .bind(cx, &mut self.new_server_url),
                    // )
                    // .child(
                    //     TextInput::new("new-server-nickname")
                    //         .placeholder("Nickname (optional)")
                    //         .bind(cx, &mut self.new_server_nickname),
                    // )
                    .child(
                        Button::new("add-server", "Add Server")
                            .on_click(cx.listener(|this, _, cx| this.add_server(cx))),
                    ),
            )
    }
}
