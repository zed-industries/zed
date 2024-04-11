use channel::{ChannelStore, DevServer, RemoteProject};
use client::{ChannelId, DevServerId, RemoteProjectId};
use editor::Editor;
use gpui::{
    AppContext, ClipboardItem, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    ScrollHandle, Task, View, ViewContext,
};
use rpc::proto::{self, CreateDevServerResponse, DevServerStatus};
use ui::{prelude::*, Indicator, List, ListHeader, ModalContent, ModalHeader, Tooltip};
use util::ResultExt;
use workspace::ModalView;

pub struct DevServerModal {
    mode: Mode,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    channel_store: Model<ChannelStore>,
    channel_id: ChannelId,
    remote_project_name_editor: View<Editor>,
    remote_project_path_editor: View<Editor>,
    dev_server_name_editor: View<Editor>,
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

impl DevServerModal {
    pub fn new(
        channel_store: Model<ChannelStore>,
        channel_id: ChannelId,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let name_editor = cx.new_view(|cx| Editor::single_line(cx));
        let path_editor = cx.new_view(|cx| Editor::single_line(cx));
        let dev_server_name_editor = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Dev server name", cx);
            editor
        });

        let focus_handle = cx.focus_handle();

        let subscriptions = [
            cx.observe(&channel_store, |_, _, cx| {
                cx.notify();
            }),
            cx.on_focus_out(&focus_handle, |_, _cx| { /* cx.emit(DismissEvent) */ }),
        ];

        Self {
            mode: Mode::Default,
            focus_handle,
            scroll_handle: ScrollHandle::new(),
            channel_store,
            channel_id,
            remote_project_name_editor: name_editor,
            remote_project_path_editor: path_editor,
            dev_server_name_editor,
            _subscriptions: subscriptions,
        }
    }

    pub fn create_remote_project(
        &mut self,
        dev_server_id: DevServerId,
        cx: &mut ViewContext<Self>,
    ) {
        let channel_id = self.channel_id;
        let name = self
            .remote_project_name_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();
        let path = self
            .remote_project_path_editor
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

        let create = self.channel_store.update(cx, |store, cx| {
            store.create_remote_project(channel_id, dev_server_id, name, path, cx)
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
            .dev_server_name_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();

        if name == "" {
            return;
        }

        let dev_server = self.channel_store.update(cx, |store, cx| {
            store.create_dev_server(self.channel_id, name.clone(), cx)
        });

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

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        match self.mode {
            Mode::Default => cx.emit(DismissEvent),
            Mode::CreateRemoteProject(_) | Mode::CreateDevServer(_) => {
                self.mode = Mode::Default;
                cx.notify();
            }
        }
    }

    fn render_dev_server(
        &mut self,
        dev_server: &DevServer,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let channel_store = self.channel_store.read(cx);
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
                                                DevServerStatus::Offline => Color::Deleted,
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
                                    cx.notify();
                                })),
                        ),
                    ),
            )
            .child(
                v_flex()
                    .w_full()
                    .bg(cx.theme().colors().title_bar_background)
                    .border()
                    .border_color(cx.theme().colors().border_variant)
                    .rounded_md()
                    .my_1()
                    .py_0p5()
                    .px_3()
                    .child(
                        List::new().empty_message("No projects.").children(
                            channel_store
                                .remote_projects_for_id(dev_server.channel_id)
                                .iter()
                                .filter_map(|remote_project| {
                                    if remote_project.dev_server_id == dev_server.id {
                                        Some(self.render_remote_project(remote_project, cx))
                                    } else {
                                        None
                                    }
                                }),
                        ),
                    ),
            )
        // .child(div().ml_8().child(
        //     Button::new(("add-project", dev_server_id.0), "Add Project").on_click(cx.listener(
        //         move |this, _, cx| {
        //             this.mode = Mode::CreateRemoteProject(CreateRemoteProject {
        //                 dev_server_id,
        //                 creating: None,
        //                 remote_project: None,
        //             });
        //             cx.notify();
        //         },
        //     )),
        // ))
    }

    fn render_remote_project(
        &mut self,
        project: &RemoteProject,
        _: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(Icon::new(IconName::FileTree))
            .child(Label::new(project.name.clone()))
            .child(Label::new(format!("({})", project.path.clone())).color(Color::Muted))
    }

    fn render_create_dev_server(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Mode::CreateDevServer(CreateDevServer {
            creating,
            dev_server,
        }) = &self.mode
        else {
            unreachable!()
        };

        self.dev_server_name_editor.update(cx, |editor, _| {
            editor.set_read_only(creating.is_some() || dev_server.is_some())
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
                                .on_click(cx.listener(|this, _: &gpui::ClickEvent, cx| {
                                    this.mode = Mode::Default;
                                    cx.notify();
                                })),
                        )
                        .child(Headline::new("Register dev server")),
                ),
            )
            .child(
                h_flex()
                    .ml_5()
                    .gap_2()
                    .child("Name")
                    .child(self.dev_server_name_editor.clone())
                    .on_action(
                        cx.listener(|this, _: &menu::Confirm, cx| this.create_dev_server(cx)),
                    )
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
                let channel_store = self.channel_store.read(cx);
                let status = channel_store
                    .find_dev_server_by_id(DevServerId(dev_server.dev_server_id))
                    .map(|server| server.status)
                    .unwrap_or(DevServerStatus::Offline);
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
                                Button::new("done", "Done").on_click(cx.listener(|this, _, cx| {
                                    this.mode = Mode::Default;
                                    cx.notify();
                                })),
                            )
                        }),
                )
            })
    }

    fn render_default(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let channel_store = self.channel_store.read(cx);
        let dev_servers = channel_store.dev_servers_for_id(self.channel_id);
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
                ModalHeader::new("Manage Remote Project")
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
                                        this.dev_server_name_editor
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

    fn render_create_project(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Mode::CreateRemoteProject(CreateRemoteProject {
            dev_server_id,
            creating,
            remote_project,
        }) = &self.mode
        else {
            unreachable!()
        };
        let channel_store = self.channel_store.read(cx);
        let (dev_server_name, dev_server_status) = channel_store
            .find_dev_server_by_id(*dev_server_id)
            .map(|server| (server.name.clone(), server.status))
            .unwrap_or((SharedString::from(""), DevServerStatus::Offline));
        v_flex()
            .px_1()
            .pt_0p5()
            .gap_px()
            .child(
                ModalHeader::new("Manage Remote Project")
                    .child(Headline::new("Manage Remote Projects")),
            )
            .child(
                h_flex()
                    .py_0p5()
                    .px_1()
                    .child(div().px_1().py_0p5().child(
                        IconButton::new("back", IconName::ArrowLeft).on_click(cx.listener(
                            |this, _, cx| {
                                this.mode = Mode::Default;
                                cx.notify()
                            },
                        )),
                    ))
                    .child("Add Project..."),
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
                                    DevServerStatus::Offline => Color::Deleted,
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
                    .child("Name")
                    .child(self.remote_project_name_editor.clone())
                    .on_action(cx.listener(|this, _: &menu::Confirm, cx| {
                        cx.focus_view(&this.remote_project_path_editor)
                    })),
            )
            .child(
                h_flex()
                    .ml_5()
                    .gap_2()
                    .child("Path")
                    .child(self.remote_project_path_editor.clone())
                    .on_action(
                        cx.listener(|this, _: &menu::Confirm, cx| this.create_dev_server(cx)),
                    )
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
                let channel_store = self.channel_store.read(cx);
                let status = channel_store
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
                                Button::new("done", "Done").on_click(cx.listener(|this, _, cx| {
                                    this.mode = Mode::Default;
                                    cx.notify();
                                })),
                            )
                        }),
                )
            })
    }
}
impl ModalView for DevServerModal {}

impl FocusableView for DevServerModal {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for DevServerModal {}

impl Render for DevServerModal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .elevation_3(cx)
            .key_context("DevServerModal")
            .on_action(cx.listener(Self::cancel))
            .pb_4()
            .w(rems(34.))
            .min_h(rems(20.))
            .max_h(rems(40.))
            .child(match &self.mode {
                Mode::Default => self.render_default(cx).into_any_element(),
                Mode::CreateRemoteProject(_) => self.render_create_project(cx).into_any_element(),
                Mode::CreateDevServer(_) => self.render_create_dev_server(cx).into_any_element(),
            })
    }
}
