use std::time::Duration;

use dev_server_projects::{DevServer, DevServerId, DevServerProject, DevServerProjectId};
use editor::Editor;
use feature_flags::FeatureFlagAppExt;
use feature_flags::FeatureFlagViewExt;
use gpui::{
    percentage, Action, Animation, AnimationExt, AnyElement, AppContext, ClipboardItem,
    DismissEvent, EventEmitter, FocusHandle, FocusableView, Model, ScrollHandle, Transformation,
    View, ViewContext,
};
use rpc::{
    proto::{CreateDevServerResponse, DevServerStatus, RegenerateDevServerTokenResponse},
    ErrorCode, ErrorExt,
};
use settings::Settings;
use theme::ThemeSettings;
use ui::{prelude::*, Indicator, List, ListHeader, ListItem, ModalContent, ModalHeader, Tooltip};
use ui_text_field::{FieldLabelLayout, TextField};
use util::ResultExt;
use workspace::{notifications::DetachAndPromptErr, AppState, ModalView, Workspace, WORKSPACE_DB};

use crate::OpenRemote;

pub struct DevServerProjects {
    mode: Mode,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    dev_server_store: Model<dev_server_projects::Store>,
    project_path_input: View<Editor>,
    dev_server_name_input: View<TextField>,
    rename_dev_server_input: View<TextField>,
    _subscription: gpui::Subscription,
}

#[derive(Default, Clone)]
struct CreateDevServer {
    creating: bool,
    dev_server: Option<CreateDevServerResponse>,
}

#[derive(Clone)]
struct EditDevServer {
    dev_server_id: DevServerId,
    state: EditDevServerState,
}

#[derive(Clone, PartialEq)]
enum EditDevServerState {
    Default,
    RenamingDevServer,
    RegeneratingToken,
    RegeneratedToken(RegenerateDevServerTokenResponse),
}

#[derive(Clone)]
struct CreateDevServerProject {
    dev_server_id: DevServerId,
    creating: bool,
}

enum Mode {
    Default(Option<CreateDevServerProject>),
    CreateDevServer(CreateDevServer),
    EditDevServer(EditDevServer),
}

impl DevServerProjects {
    pub fn register(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        cx.observe_flag::<feature_flags::Remoting, _>(|enabled, workspace, _| {
            if enabled {
                Self::register_open_remote_action(workspace);
            }
        })
        .detach();

        if cx.has_flag::<feature_flags::Remoting>() {
            Self::register_open_remote_action(workspace);
        }
    }

    fn register_open_remote_action(workspace: &mut Workspace) {
        workspace.register_action(|workspace, _: &OpenRemote, cx| {
            workspace.toggle_modal(cx, |cx| Self::new(cx))
        });
    }

    pub fn open(workspace: View<Workspace>, cx: &mut WindowContext) {
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_modal(cx, |cx| Self::new(cx))
        })
    }

    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let project_path_input = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Project path", cx);
            editor
        });
        let dev_server_name_input =
            cx.new_view(|cx| TextField::new(cx, "Name", "").with_label(FieldLabelLayout::Stacked));
        let rename_dev_server_input =
            cx.new_view(|cx| TextField::new(cx, "Name", "").with_label(FieldLabelLayout::Stacked));

        let focus_handle = cx.focus_handle();
        let dev_server_store = dev_server_projects::Store::global(cx);

        let subscription = cx.observe(&dev_server_store, |_, _, cx| {
            cx.notify();
        });

        Self {
            mode: Mode::Default(None),
            focus_handle,
            scroll_handle: ScrollHandle::new(),
            dev_server_store,
            project_path_input,
            dev_server_name_input,
            rename_dev_server_input,
            _subscription: subscription,
        }
    }

    pub fn create_dev_server_project(
        &mut self,
        dev_server_id: DevServerId,
        cx: &mut ViewContext<Self>,
    ) {
        let path = self.project_path_input.read(cx).text(cx).trim().to_string();

        if path == "" {
            return;
        }

        if self
            .dev_server_store
            .read(cx)
            .projects_for_server(dev_server_id)
            .iter()
            .any(|p| p.path == path)
        {
            cx.spawn(|_, mut cx| async move {
                cx.prompt(
                    gpui::PromptLevel::Critical,
                    "Failed to create project",
                    Some(&format!(
                        "Project {} already exists for this dev server.",
                        path
                    )),
                    &["Ok"],
                )
                .await
            })
            .detach_and_log_err(cx);
            return;
        }

        let create = {
            let path = path.clone();
            self.dev_server_store.update(cx, |store, cx| {
                store.create_dev_server_project(dev_server_id, path, cx)
            })
        };

        cx.spawn(|this, mut cx| async move {
            let result = create.await;
            this.update(&mut cx, |this, cx| {
                if result.is_ok() {
                    this.project_path_input.update(cx, |editor, cx| {
                        editor.set_text("", cx);
                    });
                    this.mode = Mode::Default(None);
                } else {
                    this.mode = Mode::Default(Some(CreateDevServerProject {
                        dev_server_id,
                        creating: false,
                    }));
                }
            })
            .log_err();
            result
        })
        .detach_and_prompt_err("Failed to create project", cx, move |e, _| {
            match e.error_code() {
                ErrorCode::DevServerOffline => Some(
                    "The dev server is offline. Please log in and check it is connected."
                        .to_string(),
                ),
                ErrorCode::DevServerProjectPathDoesNotExist => {
                    Some(format!("The path `{}` does not exist on the server.", path))
                }
                _ => None,
            }
        });

        self.mode = Mode::Default(Some(CreateDevServerProject {
            dev_server_id,
            creating: true,
        }));
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
            .dev_server_store
            .update(cx, |store, cx| store.create_dev_server(name.clone(), cx));

        cx.spawn(|this, mut cx| async move {
            let result = dev_server.await;

            this.update(&mut cx, |this, cx| match &result {
                Ok(dev_server) => {
                    this.focus_handle.focus(cx);
                    this.mode = Mode::CreateDevServer(CreateDevServer {
                        creating: false,
                        dev_server: Some(dev_server.clone()),
                    });
                }
                Err(_) => {
                    this.mode = Mode::CreateDevServer(Default::default());
                }
            })
            .log_err();
            result
        })
        .detach_and_prompt_err("Failed to create server", cx, |_, _| None);

        self.mode = Mode::CreateDevServer(CreateDevServer {
            creating: true,
            dev_server: None,
        });
        cx.notify()
    }

    fn rename_dev_server(&mut self, id: DevServerId, cx: &mut ViewContext<Self>) {
        let name = self
            .rename_dev_server_input
            .read(cx)
            .editor()
            .read(cx)
            .text(cx)
            .trim()
            .to_string();

        let Some(dev_server) = self.dev_server_store.read(cx).dev_server(id) else {
            return;
        };

        if name.is_empty() || dev_server.name == name {
            return;
        }

        let request = self
            .dev_server_store
            .update(cx, |store, cx| store.rename_dev_server(id, name, cx));

        self.mode = Mode::EditDevServer(EditDevServer {
            dev_server_id: id,
            state: EditDevServerState::RenamingDevServer,
        });

        cx.spawn(|this, mut cx| async move {
            request.await?;
            this.update(&mut cx, move |this, cx| {
                this.mode = Mode::EditDevServer(EditDevServer {
                    dev_server_id: id,
                    state: EditDevServerState::Default,
                });
                cx.notify();
            })
        })
        .detach_and_prompt_err("Failed to rename dev server", cx, |_, _| None);
    }

    fn refresh_dev_server_token(&mut self, id: DevServerId, cx: &mut ViewContext<Self>) {
        let answer = cx.prompt(
            gpui::PromptLevel::Warning,
            "Are you sure?",
            Some("This will invalidate the existing dev server token."),
            &["Generate", "Cancel"],
        );
        cx.spawn(|this, mut cx| async move {
            let answer = answer.await?;

            if answer != 0 {
                return Ok(());
            }

            let response = this
                .update(&mut cx, move |this, cx| {
                    let request = this
                        .dev_server_store
                        .update(cx, |store, cx| store.regenerate_dev_server_token(id, cx));
                    this.mode = Mode::EditDevServer(EditDevServer {
                        dev_server_id: id,
                        state: EditDevServerState::RegeneratingToken,
                    });
                    cx.notify();
                    request
                })?
                .await?;

            this.update(&mut cx, move |this, cx| {
                this.mode = Mode::EditDevServer(EditDevServer {
                    dev_server_id: id,
                    state: EditDevServerState::RegeneratedToken(response),
                });
                cx.notify();
            })
            .log_err();

            Ok(())
        })
        .detach_and_prompt_err("Failed to delete dev server", cx, |_, _| None);
    }

    fn delete_dev_server(&mut self, id: DevServerId, cx: &mut ViewContext<Self>) {
        let answer = cx.prompt(
            gpui::PromptLevel::Destructive,
            "Are you sure?",
            Some("This will delete the dev server and all of its remote projects."),
            &["Delete", "Cancel"],
        );

        cx.spawn(|this, mut cx| async move {
            let answer = answer.await?;

            if answer != 0 {
                return Ok(());
            }

            let project_ids: Vec<DevServerProjectId> = this.update(&mut cx, |this, cx| {
                this.dev_server_store.update(cx, |store, _| {
                    store
                        .projects_for_server(id)
                        .into_iter()
                        .map(|project| project.id)
                        .collect()
                })
            })?;

            this.update(&mut cx, |this, cx| {
                this.dev_server_store
                    .update(cx, |store, cx| store.delete_dev_server(id, cx))
            })?
            .await?;

            for id in project_ids {
                WORKSPACE_DB
                    .delete_workspace_by_dev_server_project_id(id)
                    .await
                    .log_err();
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to delete dev server", cx, |_, _| None);
    }

    fn delete_dev_server_project(
        &mut self,
        id: DevServerProjectId,
        path: &str,
        cx: &mut ViewContext<Self>,
    ) {
        let answer = cx.prompt(
            gpui::PromptLevel::Destructive,
            format!("Delete \"{}\"?", path).as_str(),
            Some("This will delete the remote project. You can always re-add it later."),
            &["Delete", "Cancel"],
        );

        cx.spawn(|this, mut cx| async move {
            let answer = answer.await?;

            if answer != 0 {
                return Ok(());
            }

            this.update(&mut cx, |this, cx| {
                this.dev_server_store
                    .update(cx, |store, cx| store.delete_dev_server_project(id, cx))
            })?
            .await?;

            WORKSPACE_DB
                .delete_workspace_by_dev_server_project_id(id)
                .await
                .log_err();

            Ok(())
        })
        .detach_and_prompt_err("Failed to delete dev server project", cx, |_, _| None);
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        match &self.mode {
            Mode::Default(None) => {}
            Mode::Default(Some(create_project)) => {
                self.create_dev_server_project(create_project.dev_server_id, cx);
            }
            Mode::CreateDevServer(state) => {
                if !state.creating && state.dev_server.is_none() {
                    self.create_dev_server(cx);
                }
            }
            Mode::EditDevServer(edit_dev_server) => {
                if self
                    .rename_dev_server_input
                    .read(cx)
                    .editor()
                    .read(cx)
                    .is_focused(cx)
                {
                    self.rename_dev_server(edit_dev_server.dev_server_id, cx);
                }
            }
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        match self.mode {
            Mode::Default(None) => cx.emit(DismissEvent),
            _ => {
                self.mode = Mode::Default(None);
                self.focus_handle(cx).focus(cx);
                cx.notify();
            }
        }
    }

    fn render_dev_server(
        &mut self,
        dev_server: &DevServer,
        mut create_project: Option<CreateDevServerProject>,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let dev_server_id = dev_server.id;
        let status = dev_server.status;
        let dev_server_name = dev_server.name.clone();
        if create_project
            .as_ref()
            .is_some_and(|cp| cp.dev_server_id != dev_server.id)
        {
            create_project = None;
        }

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
                            .child(dev_server_name.clone())
                            .child(
                                h_flex()
                                    .visible_on_hover("dev-server")
                                    .gap_1()
                                    .child(
                                        IconButton::new("edit-dev-server", IconName::Pencil)
                                            .on_click(cx.listener(move |this, _, cx| {
                                                this.mode = Mode::EditDevServer(EditDevServer {
                                                    dev_server_id,
                                                    state: EditDevServerState::Default,
                                                });
                                                let dev_server_name = dev_server_name.clone();
                                                this.rename_dev_server_input.update(
                                                    cx,
                                                    move |input, cx| {
                                                        input.editor().update(
                                                            cx,
                                                            move |editor, cx| {
                                                                editor.set_text(dev_server_name, cx)
                                                            },
                                                        )
                                                    },
                                                )
                                            }))
                                            .tooltip(|cx| Tooltip::text("Edit dev server", cx)),
                                    )
                                    .child({
                                        let dev_server_id = dev_server.id;
                                        IconButton::new("remove-dev-server", IconName::Trash)
                                            .on_click(cx.listener(move |this, _, cx| {
                                                this.delete_dev_server(dev_server_id, cx)
                                            }))
                                            .tooltip(|cx| Tooltip::text("Remove dev server", cx))
                                    }),
                            ),
                    )
                    .child(
                        h_flex().gap_1().child(
                            IconButton::new(
                                ("add-remote-project", dev_server_id.0),
                                IconName::Plus,
                            )
                            .tooltip(|cx| Tooltip::text("Add a remote project", cx))
                            .on_click(cx.listener(
                                move |this, _, cx| {
                                    if let Mode::Default(project) = &mut this.mode {
                                        *project = Some(CreateDevServerProject {
                                            dev_server_id,
                                            creating: false,
                                        });
                                    }
                                    this.project_path_input.read(cx).focus_handle(cx).focus(cx);
                                    cx.notify();
                                },
                            )),
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
                        List::new()
                            .empty_message("No projects.")
                            .children(
                                self.dev_server_store
                                    .read(cx)
                                    .projects_for_server(dev_server.id)
                                    .iter()
                                    .map(|p| self.render_dev_server_project(p, cx)),
                            )
                            .when_some(create_project, |el, create_project| {
                                el.child(self.render_create_new_project(&create_project, cx))
                            }),
                    ),
            )
    }

    fn render_create_new_project(
        &mut self,
        create_project: &CreateDevServerProject,
        _: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        ListItem::new("create-remote-project")
            .start_slot(Icon::new(IconName::FileTree).color(Color::Muted))
            .child(self.project_path_input.clone())
            .child(
                div()
                    .w(IconSize::Medium.rems())
                    .when(create_project.creating, |el| {
                        el.child(
                            Icon::new(IconName::ArrowCircle)
                                .size(IconSize::Medium)
                                .with_animation(
                                    "arrow-circle",
                                    Animation::new(Duration::from_secs(2)).repeat(),
                                    |icon, delta| {
                                        icon.transform(Transformation::rotate(percentage(delta)))
                                    },
                                ),
                        )
                    }),
            )
    }

    fn render_dev_server_project(
        &mut self,
        project: &DevServerProject,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let dev_server_project_id = project.id;
        let project_id = project.project_id;
        let is_online = project_id.is_some();
        let project_path = project.path.clone();

        ListItem::new(("remote-project", dev_server_project_id.0))
            .start_slot(Icon::new(IconName::FileTree).when(!is_online, |icon| icon.color(Color::Muted)))
            .child(
                    Label::new(project.path.clone())
            )
            .on_click(cx.listener(move |_, _, cx| {
                if let Some(project_id) = project_id {
                    if let Some(app_state) = AppState::global(cx).upgrade() {
                        workspace::join_dev_server_project(project_id, app_state, None, cx)
                            .detach_and_prompt_err("Could not join project", cx, |_, _| None)
                    }
                } else {
                    cx.spawn(|_, mut cx| async move {
                        cx.prompt(gpui::PromptLevel::Critical, "This project is offline", Some("The `zed` instance running on this dev server is not connected. You will have to restart it."), &["Ok"]).await.log_err();
                    }).detach();
                }
            }))
            .end_hover_slot::<AnyElement>(Some(IconButton::new("remove-remote-project", IconName::Trash)
                .on_click(cx.listener(move |this, _, cx| {
                    this.delete_dev_server_project(dev_server_project_id, &project_path, cx)
                }))
                .tooltip(|cx| Tooltip::text("Delete remote project", cx)).into_any_element()))
    }

    fn render_create_dev_server(
        &mut self,
        state: CreateDevServer,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let CreateDevServer {
            creating,
            dev_server,
        } = state;

        self.dev_server_name_input.update(cx, |input, cx| {
            input.set_disabled(creating || dev_server.is_some(), cx);
        });

        v_flex()
            .id("scroll-container")
            .h_full()
            .overflow_y_scroll()
            .track_scroll(&self.scroll_handle)
            .px_1()
            .pt_0p5()
            .gap_px()
            .child(
                ModalHeader::new("create-dev-server")
                    .show_back_button(true)
                    .child(Headline::new("New dev server").size(HeadlineSize::Small)),
            )
            .child(
                ModalContent::new().child(
                    v_flex()
                        .w_full()
                        .child(
                            h_flex()
                                .pb_2()
                                .items_end()
                                .w_full()
                                .px_2()
                                .border_b_1()
                                .border_color(cx.theme().colors().border)
                                .child(
                                    div()
                                        .pl_2()
                                        .max_w(rems(16.))
                                        .child(self.dev_server_name_input.clone()),
                                )
                                .child(
                                    div()
                                        .pl_1()
                                        .pb(px(3.))
                                        .when(!creating && dev_server.is_none(), |div| {
                                            div.child(Button::new("create-dev-server", "Create").on_click(
                                                cx.listener(move |this, _, cx| {
                                                    this.create_dev_server(cx);
                                                }),
                                            ))
                                        })
                                        .when(creating && dev_server.is_none(), |div| {
                                            div.child(
                                                Button::new("create-dev-server", "Creating...")
                                                    .disabled(true),
                                            )
                                        }),
                                )
                        )
                        .when(dev_server.is_none(), |div| {
                            div.px_2().child(Label::new("Once you have created a dev server, you will be given a command to run on the server to register it.").color(Color::Muted))
                        })
                        .when_some(dev_server.clone(), |div, dev_server| {
                            let status = self
                                .dev_server_store
                                .read(cx)
                                .dev_server_status(DevServerId(dev_server.dev_server_id));

                            div.child(
                                 Self::render_dev_server_token_instructions(&dev_server.access_token, &dev_server.name, status, cx)
                            )
                        }),
                )
            )
    }

    fn render_dev_server_token_instructions(
        access_token: &str,
        dev_server_name: &str,
        status: DevServerStatus,
        cx: &mut ViewContext<Self>,
    ) -> Div {
        let instructions = SharedString::from(format!("zed --dev-server-token {}", access_token));

        v_flex()
            .pl_2()
            .pt_2()
            .gap_2()
            .child(
                h_flex()
                    .justify_between()
                    .w_full()
                    .child(Label::new(format!(
                        "Please log into `{}` and run:",
                        dev_server_name
                    )))
                    .child(
                        Button::new("copy-access-token", "Copy Instructions")
                            .icon(Some(IconName::Copy))
                            .icon_size(IconSize::Small)
                            .on_click({
                                let instructions = instructions.clone();
                                cx.listener(move |_, _, cx| {
                                    cx.write_to_clipboard(ClipboardItem::new(
                                        instructions.to_string(),
                                    ))
                                })
                            }),
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
                    .font_family(ThemeSettings::get_global(cx).buffer_font.family.clone())
                    .child(Label::new(instructions)),
            )
            .when(status == DevServerStatus::Offline, |this| {
                this.child(Self::render_loading_spinner("Waiting for connection…"))
            })
            .when(status == DevServerStatus::Online, |this| {
                this.child(Label::new("🎊 Connection established!")).child(
                    h_flex()
                        .justify_end()
                        .child(Button::new("done", "Done").on_click(
                            cx.listener(|_, _, cx| cx.dispatch_action(menu::Cancel.boxed_clone())),
                        )),
                )
            })
    }

    fn render_loading_spinner(label: impl Into<SharedString>) -> Div {
        h_flex()
            .gap_2()
            .child(
                Icon::new(IconName::ArrowCircle)
                    .size(IconSize::Medium)
                    .with_animation(
                        "arrow-circle",
                        Animation::new(Duration::from_secs(2)).repeat(),
                        |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                    ),
            )
            .child(Label::new(label))
    }

    fn render_edit_dev_server(
        &mut self,
        edit_dev_server: EditDevServer,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let dev_server_id = edit_dev_server.dev_server_id;
        let dev_server = self
            .dev_server_store
            .read(cx)
            .dev_server(dev_server_id)
            .cloned();

        let dev_server_name = dev_server
            .as_ref()
            .map(|dev_server| dev_server.name.clone())
            .unwrap_or_default();

        let dev_server_status = dev_server
            .map(|dev_server| dev_server.status)
            .unwrap_or(DevServerStatus::Offline);

        let disabled = matches!(
            edit_dev_server.state,
            EditDevServerState::RenamingDevServer | EditDevServerState::RegeneratingToken
        );
        self.rename_dev_server_input.update(cx, |input, cx| {
            input.set_disabled(disabled, cx);
        });

        let rename_dev_server_input_text = self
            .rename_dev_server_input
            .read(cx)
            .editor()
            .read(cx)
            .text(cx);

        let content = v_flex().w_full().gap_2().child(
            h_flex()
                .pb_2()
                .border_b_1()
                .border_color(cx.theme().colors().border)
                .items_end()
                .w_full()
                .px_2()
                .child(
                    div()
                        .pl_2()
                        .max_w(rems(16.))
                        .child(self.rename_dev_server_input.clone()),
                )
                .child(
                    div()
                        .pl_1()
                        .pb(px(3.))
                        .when(
                            edit_dev_server.state != EditDevServerState::RenamingDevServer,
                            |div| {
                                div.child(
                                    Button::new("rename-dev-server", "Rename")
                                        .disabled(
                                            rename_dev_server_input_text.trim().is_empty()
                                                || rename_dev_server_input_text == dev_server_name,
                                        )
                                        .on_click(cx.listener(move |this, _, cx| {
                                            this.rename_dev_server(dev_server_id, cx);
                                            cx.notify();
                                        })),
                                )
                            },
                        )
                        .when(
                            edit_dev_server.state == EditDevServerState::RenamingDevServer,
                            |div| {
                                div.child(
                                    Button::new("rename-dev-server", "Renaming...").disabled(true),
                                )
                            },
                        ),
                ),
        );

        let content = content.child(match edit_dev_server.state {
            EditDevServerState::RegeneratingToken => {
                Self::render_loading_spinner("Generating token...")
            }
            EditDevServerState::RegeneratedToken(response) => {
                Self::render_dev_server_token_instructions(
                    &response.access_token,
                    &dev_server_name,
                    dev_server_status,
                    cx,
                )
            }
            _ => h_flex().items_end().w_full().child(
                Button::new("regenerate-dev-server-token", "Generate new access token")
                    .icon(IconName::Update)
                    .on_click(cx.listener(move |this, _, cx| {
                        this.refresh_dev_server_token(dev_server_id, cx);
                        cx.notify();
                    })),
            ),
        });

        v_flex()
            .id("scroll-container")
            .h_full()
            .overflow_y_scroll()
            .track_scroll(&self.scroll_handle)
            .px_1()
            .pt_0p5()
            .gap_px()
            .child(
                ModalHeader::new("edit-dev-server")
                    .show_back_button(true)
                    .child(
                        Headline::new(format!("Edit {}", &dev_server_name))
                            .size(HeadlineSize::Small),
                    ),
            )
            .child(ModalContent::new().child(v_flex().w_full().child(content)))
    }

    fn render_default(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let dev_servers = self.dev_server_store.read(cx).dev_servers();

        let Mode::Default(create_dev_server_project) = &self.mode else {
            unreachable!()
        };
        let create_dev_server_project = create_dev_server_project.clone();

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

                                        this.dev_server_name_input.update(cx, |input, cx| {
                                            input.editor().update(cx, |editor, cx| {
                                                editor.set_text("", cx);
                                            });
                                            input.focus_handle(cx).focus(cx)
                                        });

                                        cx.notify();
                                    })),
                            ),
                        ))
                        .children(dev_servers.iter().map(|dev_server| {
                            self.render_dev_server(
                                dev_server,
                                create_dev_server_project.clone(),
                                cx,
                            )
                            .into_any_element()
                        })),
                ),
            )
    }
}
impl ModalView for DevServerProjects {}

impl FocusableView for DevServerProjects {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for DevServerProjects {}

impl Render for DevServerProjects {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .elevation_3(cx)
            .key_context("DevServerModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .on_mouse_down_out(cx.listener(|this, _, cx| {
                if matches!(this.mode, Mode::Default(None)) {
                    cx.emit(DismissEvent)
                }
            }))
            .pb_4()
            .w(rems(34.))
            .min_h(rems(20.))
            .max_h(rems(40.))
            .child(match &self.mode {
                Mode::Default(_) => self.render_default(cx).into_any_element(),
                Mode::CreateDevServer(state) => self
                    .render_create_dev_server(state.clone(), cx)
                    .into_any_element(),
                Mode::EditDevServer(state) => self
                    .render_edit_dev_server(state.clone(), cx)
                    .into_any_element(),
            })
    }
}
