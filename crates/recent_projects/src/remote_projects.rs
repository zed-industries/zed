use gpui::{
    Action, AppContext, ClipboardItem, DismissEvent, EventEmitter, FocusHandle, FocusableView,
    Model, ScrollHandle, Task, View, ViewContext,
};
use remote_projects::{DevServer, DevServerId, OpenRemote, RemoteProject, RemoteProjectId};
use rpc::proto::{self, CreateDevServerResponse, DevServerStatus};
use ui::{prelude::*, Indicator, List, ListHeader, ListItem, ModalContent, ModalHeader, Tooltip};
use ui_text_field::{FieldLabelLayout, TextField};
use util::ResultExt;
use workspace::{notifications::DetachAndPromptErr, AppState, ModalView, Workspace};

pub struct RemoteProjects {
    mode: Mode,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    remote_project_store: Model<remote_projects::Store>,
    remote_project_name_input: View<TextField>,
    remote_project_path_input: View<TextField>,
    dev_server_name_input: View<TextField>,
    _subscriptions: [gpui::Subscription; 2],
}

#[derive(Default)]
struct CreateDevServer {
    creating: Option<Task<()>>,
    dev_server: Option<CreateDevServerResponse>,
}

struct CreateRemoteProject {
    dev_server_id: DevServerId,
    creating: Option<Task<()>>,
    remote_project: Option<proto::RemoteProject>,
}

enum Mode {
    Default,
    CreateRemoteProject(CreateRemoteProject),
    CreateDevServer(CreateDevServer),
}

impl RemoteProjects {
    pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(|workspace, _: &OpenRemote, cx| {
            workspace.toggle_modal(cx, |cx| Self::new(cx))
        });
    }
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let name_editor =
            cx.new_view(|cx| TextField::new(cx, "Name", "").with_label(FieldLabelLayout::Inline));
        let path_editor =
            cx.new_view(|cx| TextField::new(cx, "Path", "").with_label(FieldLabelLayout::Inline));
        let dev_server_name_editor = cx.new_view(|cx| TextField::new(cx, "", "Dev server name"));

        let focus_handle = cx.focus_handle();
        let remote_project_store = remote_projects::Store::global(cx);

        let subscriptions = [
            cx.observe(&remote_project_store, |_, _, cx| {
                cx.notify();
            }),
            cx.on_focus_out(&focus_handle, |_, _cx| { /* cx.emit(DismissEvent) */ }),
        ];

        Self {
            mode: Mode::Default,
            focus_handle,
            scroll_handle: ScrollHandle::new(),
            remote_project_store,
            remote_project_name_input: name_editor,
            remote_project_path_input: path_editor,
            dev_server_name_input: dev_server_name_editor,
            _subscriptions: subscriptions,
        }
    }

    pub fn create_remote_project(
        &mut self,
        dev_server_id: DevServerId,
        cx: &mut ViewContext<Self>,
    ) {
        let name = self
            .remote_project_name_input
            .read(cx)
            .editor()
            .read(cx)
            .text(cx)
            .trim()
            .to_string();
        let path = self
            .remote_project_path_input
            .read(cx)
            .editor()
            .read(cx)
            .text(cx)
            .trim()
            .to_string();

        if name == "" {
            return;
        }
        if path == "" {
            return;
        }

        let create = self.remote_project_store.update(cx, |store, cx| {
            store.create_remote_project(dev_server_id, name, path, cx)
        });

        let task = cx.spawn(|this, mut cx| async move {
            let result = create.await;
            if let Err(e) = &result {
                cx.prompt(
                    gpui::PromptLevel::Critical,
                    "Failed to create project",
                    Some(&format!("{:?}. Please try again.", e)),
                    &["Ok"],
                )
                .await
                .log_err();
            }
            this.update(&mut cx, |this, _| {
                this.mode = Mode::CreateRemoteProject(CreateRemoteProject {
                    dev_server_id,
                    creating: None,
                    remote_project: result.ok().and_then(|r| r.remote_project),
                });
            })
            .log_err();
        });

        self.mode = Mode::CreateRemoteProject(CreateRemoteProject {
            dev_server_id,
            creating: Some(task),
            remote_project: None,
        });
    }

    pub fn create_dev_server(&mut self, cx: &mut ViewContext<Self>) {
        let name = self
            .dev_server_name_input
            .read(cx)
            .editor()
            .read(cx)
            .text(cx)
            .trim()
            .to_string();

        if name == "" {
            return;
        }

        let dev_server = self
            .remote_project_store
            .update(cx, |store, cx| store.create_dev_server(name.clone(), cx));

        let task = cx.spawn(|this, mut cx| async move {
            match dev_server.await {
                Ok(dev_server) => {
                    this.update(&mut cx, |this, _| {
                        this.mode = Mode::CreateDevServer(CreateDevServer {
                            creating: None,
                            dev_server: Some(dev_server),
                        });
                    })
                    .log_err();
                }
                Err(e) => {
                    cx.prompt(
                        gpui::PromptLevel::Critical,
                        "Failed to create server",
                        Some(&format!("{:?}. Please try again.", e)),
                        &["Ok"],
                    )
                    .await
                    .log_err();
                    this.update(&mut cx, |this, _| {
                        this.mode = Mode::CreateDevServer(Default::default());
                    })
                    .log_err();
                }
            }
        });

        self.mode = Mode::CreateDevServer(CreateDevServer {
            creating: Some(task),
            dev_server: None,
        });
        cx.notify()
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        match self.mode {
            Mode::Default => {}
            Mode::CreateRemoteProject(CreateRemoteProject { dev_server_id, .. }) => {
                self.create_remote_project(dev_server_id, cx);
            }
            Mode::CreateDevServer(_) => {
                self.create_dev_server(cx);
            }
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        match self.mode {
            Mode::Default => cx.emit(DismissEvent),
            Mode::CreateRemoteProject(_) | Mode::CreateDevServer(_) => {
                self.mode = Mode::Default;
                self.focus_handle(cx).focus(cx);
                cx.notify();
            }
        }
    }

    fn render_dev_server(
        &mut self,
        dev_server: &DevServer,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let dev_server_id = dev_server.id;
        let status = dev_server.status;

        v_flex()
            .w_full()
            .child(
                h_flex()
                    .group("dev-server")
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                div()
                                    .id(("status", dev_server.id.0))
                                    .relative()
                                    .child(Icon::new(IconName::Server).size(IconSize::Small))
                                    .child(
                                        div().absolute().bottom_0().left(rems_from_px(8.0)).child(
                                            Indicator::dot().color(match status {
                                                DevServerStatus::Online => Color::Created,
                                                DevServerStatus::Offline => Color::Hidden,
                                            }),
                                        ),
                                    )
                                    .tooltip(move |cx| {
                                        Tooltip::text(
                                            match status {
                                                DevServerStatus::Online => "Online",
                                                DevServerStatus::Offline => "Offline",
                                            },
                                            cx,
                                        )
                                    }),
                            )
                            .child(dev_server.name.clone())
                            .child(
                                h_flex()
                                    .visible_on_hover("dev-server")
                                    .gap_1()
                                    .child(
                                        IconButton::new("edit-dev-server", IconName::Pencil)
                                            .disabled(true) //TODO implement this on the collab side
                                            .tooltip(|cx| {
                                                Tooltip::text("Coming Soon - Edit dev server", cx)
                                            }),
                                    )
                                    .child(
                                        IconButton::new("remove-dev-server", IconName::Trash)
                                            .disabled(true) //TODO implement this on the collab side
                                            .tooltip(|cx| {
                                                Tooltip::text("Coming Soon - Remove dev server", cx)
                                            }),
                                    ),
                            ),
                    )
                    .child(
                        h_flex().gap_1().child(
                            IconButton::new("add-remote-project", IconName::Plus)
                                .tooltip(|cx| Tooltip::text("Add a remote project", cx))
                                .on_click(cx.listener(move |this, _, cx| {
                                    this.mode = Mode::CreateRemoteProject(CreateRemoteProject {
                                        dev_server_id,
                                        creating: None,
                                        remote_project: None,
                                    });
                                    this.remote_project_name_input
                                        .read(cx)
                                        .focus_handle(cx)
                                        .focus(cx);
                                    cx.notify();
                                })),
                        ),
                    ),
            )
            .child(
                v_flex()
                    .w_full()
                    .bg(cx.theme().colors().title_bar_background) // todo: this should be distinct
                    .border()
                    .border_color(cx.theme().colors().border_variant)
                    .rounded_md()
                    .my_1()
                    .py_0p5()
                    .px_3()
                    .child(
                        List::new().empty_message("No projects.").children(
                            self.remote_project_store
                                .read(cx)
                                .remote_projects_for_server(dev_server.id)
                                .iter()
                                .map(|p| self.render_remote_project(p, cx)),
                        ),
                    ),
            )
    }

    fn render_remote_project(
        &mut self,
        project: &RemoteProject,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let remote_project_id = project.id;
        let project_id = project.project_id;
        let is_online = project_id.is_some();

        ListItem::new(("remote-project", remote_project_id.0))
            .start_slot(Icon::new(IconName::FileTree).when(!is_online, |icon| icon.color(Color::Muted)))
            .child(
                h_flex()
                    .child(Label::new(project.name.clone()).when(!is_online, |label| {
                        label.color(Color::Muted)
                    }))
                    .child(Label::new(format!("({})", project.path.clone())).color(Color::Muted)),
            )
            .on_click(cx.listener(move |_, _, cx| {
                if let Some(project_id) = project_id {
                    if let Some(app_state) = AppState::global(cx).upgrade() {
                        workspace::join_remote_project(project_id, app_state, cx)
                            .detach_and_prompt_err("Could not join project", cx, |_, _| None)
                    }
                } else {
                    cx.spawn(|_, mut cx| async move {
                        cx.prompt(gpui::PromptLevel::Critical, "This project is offline", Some("The `zed` instance running on this dev server is not connected. You will have to restart it."), &["Ok"]).await.log_err();
                    }).detach();
                }
            }))
    }

    fn render_create_dev_server(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Mode::CreateDevServer(CreateDevServer {
            creating,
            dev_server,
        }) = &self.mode
        else {
            unreachable!()
        };

        self.dev_server_name_input.update(cx, |input, cx| {
            input.editor().update(cx, |editor, _| {
                editor.set_read_only(creating.is_some() || dev_server.is_some())
            });
        });

        v_flex()
            .px_1()
            .pt_0p5()
            .gap_px()
            .child(
                v_flex().py_0p5().px_1().child(
                    h_flex()
                        .px_1()
                        .py_0p5()
                        .child(
                            IconButton::new("back", IconName::ArrowLeft)
                                .style(ButtonStyle::Transparent)
                                .on_click(cx.listener(|_, _: &gpui::ClickEvent, cx| {
                                    cx.dispatch_action(menu::Cancel.boxed_clone())
                                })),
                        )
                        .child(Headline::new("Register dev server").size(HeadlineSize::Small)),
                ),
            )
            .child(
                h_flex()
                    .ml_5()
                    .gap_2()
                    .child(self.dev_server_name_input.clone())
                    .when(creating.is_none() && dev_server.is_none(), |div| {
                        div.child(
                            Button::new("create-dev-server", "Create").on_click(cx.listener(
                                move |this, _, cx| {
                                    this.create_dev_server(cx);
                                },
                            )),
                        )
                    })
                    .when(creating.is_some() && dev_server.is_none(), |div| {
                        div.child(Button::new("create-dev-server", "Creating...").disabled(true))
                    }),
            )
            .when_some(dev_server.clone(), |div, dev_server| {
                let status = self
                    .remote_project_store
                    .read(cx)
                    .dev_server_status(DevServerId(dev_server.dev_server_id));

                let instructions = SharedString::from(format!(
                    "zed --dev-server-token {}",
                    dev_server.access_token
                ));
                div.child(
                    v_flex()
                        .ml_8()
                        .gap_2()
                        .child(Label::new(format!(
                            "Please log into `{}` and run:",
                            dev_server.name
                        )))
                        .child(instructions.clone())
                        .child(
                            IconButton::new("copy-access-token", IconName::Copy)
                                .on_click(cx.listener(move |_, _, cx| {
                                    cx.write_to_clipboard(ClipboardItem::new(
                                        instructions.to_string(),
                                    ))
                                }))
                                .icon_size(IconSize::Small)
                                .tooltip(|cx| Tooltip::text("Copy access token", cx)),
                        )
                        .when(status == DevServerStatus::Offline, |this| {
                            this.child(Label::new("Waiting for connection..."))
                        })
                        .when(status == DevServerStatus::Online, |this| {
                            this.child(Label::new("Connection established! 🎊")).child(
                                Button::new("done", "Done").on_click(cx.listener(|_, _, cx| {
                                    cx.dispatch_action(menu::Cancel.boxed_clone())
                                })),
                            )
                        }),
                )
            })
    }

    fn render_default(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let dev_servers = self.remote_project_store.read(cx).dev_servers();
        // let dev_servers = Vec::new();

        v_flex()
            .id("scroll-container")
            .h_full()
            .overflow_y_scroll()
            .track_scroll(&self.scroll_handle)
            .px_1()
            .pt_0p5()
            .gap_px()
            .child(
                ModalHeader::new("remote-projects")
                    .show_dismiss_button(true)
                    .child(Headline::new("Remote Projects").size(HeadlineSize::Small)),
            )
            .child(
                ModalContent::new().child(
                    List::new()
                        .empty_message("No dev servers registered.")
                        .header(Some(
                            ListHeader::new("Dev Servers").end_slot(
                                Button::new("register-dev-server-button", "New Server")
                                    .icon(IconName::Plus)
                                    .icon_position(IconPosition::Start)
                                    .tooltip(|cx| Tooltip::text("Register a new dev server", cx))
                                    .on_click(cx.listener(|this, _, cx| {
                                        this.mode = Mode::CreateDevServer(Default::default());
                                        this.dev_server_name_input
                                            .read(cx)
                                            .focus_handle(cx)
                                            .focus(cx);
                                        cx.notify();
                                    })),
                            ),
                        ))
                        .children(dev_servers.iter().map(|dev_server| {
                            self.render_dev_server(dev_server, cx).into_any_element()
                        })),
                ),
            )
    }

    fn render_create_remote_project(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Mode::CreateRemoteProject(CreateRemoteProject {
            dev_server_id,
            creating,
            remote_project,
        }) = &self.mode
        else {
            unreachable!()
        };

        let dev_server = self
            .remote_project_store
            .read(cx)
            .dev_server(*dev_server_id)
            .cloned();

        let (dev_server_name, dev_server_status) = dev_server
            .map(|server| (server.name, server.status))
            .unwrap_or((SharedString::from(""), DevServerStatus::Offline));

        v_flex()
            .px_1()
            .pt_0p5()
            .gap_px()
            .child(
                v_flex().py_0p5().px_1().child(
                    h_flex()
                        .px_1()
                        .py_0p5()
                        .child(
                            IconButton::new("back", IconName::ArrowLeft)
                                .style(ButtonStyle::Transparent)
                                .on_click(cx.listener(|_, _: &gpui::ClickEvent, cx| {
                                    cx.dispatch_action(menu::Cancel.boxed_clone())
                                })),
                        )
                        .child(Headline::new("Add remote project").size(HeadlineSize::Small)),
                ),
            )
            .child(
                h_flex()
                    .ml_5()
                    .gap_2()
                    .child(
                        div()
                            .id(("status", dev_server_id.0))
                            .relative()
                            .child(Icon::new(IconName::Server))
                            .child(div().absolute().bottom_0().left(rems_from_px(12.0)).child(
                                Indicator::dot().color(match dev_server_status {
                                    DevServerStatus::Online => Color::Created,
                                    DevServerStatus::Offline => Color::Hidden,
                                }),
                            ))
                            .tooltip(move |cx| {
                                Tooltip::text(
                                    match dev_server_status {
                                        DevServerStatus::Online => "Online",
                                        DevServerStatus::Offline => "Offline",
                                    },
                                    cx,
                                )
                            }),
                    )
                    .child(dev_server_name.clone()),
            )
            .child(
                h_flex()
                    .ml_5()
                    .gap_2()
                    .child(self.remote_project_name_input.clone())
                    .on_action(cx.listener(|this, _: &menu::Confirm, cx| {
                        cx.focus_view(&this.remote_project_path_input)
                    })),
            )
            .child(
                h_flex()
                    .ml_5()
                    .gap_2()
                    .child(self.remote_project_path_input.clone())
                    .when(creating.is_none() && remote_project.is_none(), |div| {
                        div.child(Button::new("create-remote-server", "Create").on_click({
                            let dev_server_id = *dev_server_id;
                            cx.listener(move |this, _, cx| {
                                this.create_remote_project(dev_server_id, cx)
                            })
                        }))
                    })
                    .when(creating.is_some(), |div| {
                        div.child(Button::new("create-dev-server", "Creating...").disabled(true))
                    }),
            )
            .when_some(remote_project.clone(), |div, remote_project| {
                let status = self
                    .remote_project_store
                    .read(cx)
                    .find_remote_project_by_id(RemoteProjectId(remote_project.id))
                    .map(|project| {
                        if project.project_id.is_some() {
                            DevServerStatus::Online
                        } else {
                            DevServerStatus::Offline
                        }
                    })
                    .unwrap_or(DevServerStatus::Offline);
                div.child(
                    v_flex()
                        .ml_5()
                        .ml_8()
                        .gap_2()
                        .when(status == DevServerStatus::Offline, |this| {
                            this.child(Label::new("Waiting for project..."))
                        })
                        .when(status == DevServerStatus::Online, |this| {
                            this.child(Label::new("Project online! 🎊")).child(
                                Button::new("done", "Done").on_click(cx.listener(|_, _, cx| {
                                    cx.dispatch_action(menu::Cancel.boxed_clone())
                                })),
                            )
                        }),
                )
            })
    }
}
impl ModalView for RemoteProjects {}

impl FocusableView for RemoteProjects {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for RemoteProjects {}

impl Render for RemoteProjects {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .elevation_3(cx)
            .key_context("DevServerModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .pb_4()
            .w(rems(34.))
            .min_h(rems(20.))
            .max_h(rems(40.))
            .child(match &self.mode {
                Mode::Default => self.render_default(cx).into_any_element(),
                Mode::CreateRemoteProject(_) => {
                    self.render_create_remote_project(cx).into_any_element()
                }
                Mode::CreateDevServer(_) => self.render_create_dev_server(cx).into_any_element(),
            })
    }
}
