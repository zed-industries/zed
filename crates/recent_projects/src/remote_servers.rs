use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Arc;

use editor::Editor;
use file_finder::OpenPathDelegate;
use futures::channel::oneshot;
use futures::future::Shared;
use futures::FutureExt;
use gpui::canvas;
use gpui::ClipboardItem;
use gpui::Task;
use gpui::WeakView;
use gpui::{
    AnyElement, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    PromptLevel, ScrollHandle,
};
use picker::Picker;
use project::Project;
use remote::ssh_session::ConnectionIdentifier;
use remote::SshConnectionOptions;
use remote::SshRemoteClient;
use settings::update_settings_file;
use settings::Settings;
use ui::Navigable;
use ui::NavigableEntry;
use ui::{
    prelude::*, IconButtonShape, List, ListItem, ListSeparator, Modal, ModalHeader, Scrollbar,
    ScrollbarState, Section, Tooltip,
};
use util::ResultExt;
use workspace::notifications::NotificationId;
use workspace::OpenOptions;
use workspace::Toast;
use workspace::{notifications::DetachAndPromptErr, ModalView, Workspace};

use crate::ssh_connections::connect_over_ssh;
use crate::ssh_connections::open_ssh_project;
use crate::ssh_connections::RemoteSettingsContent;
use crate::ssh_connections::SshConnection;
use crate::ssh_connections::SshConnectionHeader;
use crate::ssh_connections::SshConnectionModal;
use crate::ssh_connections::SshProject;
use crate::ssh_connections::SshPrompt;
use crate::ssh_connections::SshSettings;
use crate::OpenRemote;

mod navigation_base {}
pub struct RemoteServerProjects {
    mode: Mode,
    focus_handle: FocusHandle,
    workspace: WeakModel<Workspace>,
    retained_connections: Vec<Model<SshRemoteClient>>,
}

struct CreateRemoteServer {
    address_editor: Model<Editor>,
    address_error: Option<SharedString>,
    ssh_prompt: Option<Model<SshPrompt>>,
    _creating: Option<Task<Option<()>>>,
}

impl CreateRemoteServer {
    fn new(window: &mut gpui::Window, cx: &mut gpui::AppContext) -> Self {
        let address_editor = cx.new_model(Editor::single_line);
        address_editor.update(cx, |this, model, cx| {
            this.focus_handle(cx).focus(window);
        });
        Self {
            address_editor,
            address_error: None,
            ssh_prompt: None,
            _creating: None,
        }
    }
}

struct ProjectPicker {
    connection_string: SharedString,
    nickname: Option<SharedString>,
    picker: Model<Picker<OpenPathDelegate>>,
    _path_task: Shared<Task<Option<()>>>,
}

struct EditNicknameState {
    index: usize,
    editor: Model<Editor>,
}

impl EditNicknameState {
    fn new(index: usize, window: &mut gpui::Window, cx: &mut gpui::AppContext) -> Self {
        let this = Self {
            index,
            editor: cx.new_model(Editor::single_line),
        };
        let starting_text = SshSettings::get_global(cx)
            .ssh_connections()
            .nth(index)
            .and_then(|state| state.nickname.clone())
            .filter(|text| !text.is_empty());
        this.editor.update(cx, |this, model, cx| {
            this.set_placeholder_text("Add a nickname for this server", cx);
            if let Some(starting_text) = starting_text {
                this.set_text(starting_text, cx);
            }
        });
        this.editor.focus_handle(cx).focus(window);
        this
    }
}

impl FocusableView for ProjectPicker {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl ProjectPicker {
    fn new(
        ix: usize,
        connection: SshConnectionOptions,
        project: Model<Project>,
        workspace: WeakModel<Workspace>,
        model: &Model<RemoteServerProjects>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        let (tx, rx) = oneshot::channel();
        let lister = project::DirectoryLister::Project(project.clone());
        let query = lister.default_query(cx);
        let delegate = file_finder::OpenPathDelegate::new(tx, lister);

        let picker = cx.new_model(|model, cx| {
            let picker = Picker::uniform_list(delegate, model, cx)
                .width(rems(34.))
                .modal(false);
            picker.set_query(query, model, cx);
            picker
        });
        let connection_string = connection.connection_string().into();
        let nickname = connection.nickname.clone().map(|nick| nick.into());
        let _path_task = cx
            .spawn({
                let workspace = workspace.clone();
                move |this, mut cx| async move {
                    let Ok(Some(paths)) = rx.await else {
                        workspace
                            .update(&mut cx, |workspace, cx| {
                                let weak = model.downgrade();
                                workspace
                                    .toggle_modal(cx, |cx| RemoteServerProjects::new(cx, weak));
                            })
                            .log_err()?;
                        return None;
                    };

                    let app_state = workspace
                        .update(&mut cx, |workspace, _| workspace.app_state().clone())
                        .ok()?;
                    let options = cx
                        .update(|cx| (app_state.build_window_options)(None, cx))
                        .log_err()?;

                    cx.open_window(options, |cx| {
                        cx.activate_window();

                        let fs = app_state.fs.clone();
                        update_settings_file::<SshSettings>(fs, cx, {
                            let paths = paths
                                .iter()
                                .map(|path| path.to_string_lossy().to_string())
                                .collect();
                            move |setting, _| {
                                if let Some(server) = setting
                                    .ssh_connections
                                    .as_mut()
                                    .and_then(|connections| connections.get_mut(ix))
                                {
                                    server.projects.insert(SshProject { paths });
                                }
                            }
                        });

                        let tasks = paths
                            .into_iter()
                            .map(|path| {
                                project.update(cx, |project, model, cx| {
                                    project.find_or_create_worktree(&path, true, model, cx)
                                })
                            })
                            .collect::<Vec<_>>();
                        cx.spawn(|_| async move {
                            for task in tasks {
                                task.await?;
                            }
                            Ok(())
                        })
                        .detach_and_prompt_err(
                            "Failed to open path",
                            cx,
                            |_, _| None,
                        );

                        cx.new_model(|model, cx| {
                            let workspace =
                                Workspace::new(None, project.clone(), app_state.clone(), model, cx);

                            workspace
                                .client()
                                .telemetry()
                                .report_app_event("create ssh project".to_string());

                            workspace
                        })
                    })
                    .log_err();
                    this.update(&mut cx, |_, cx| {
                        model.emit(DismissEvent, cx);
                    })
                    .ok();
                    Some(())
                }
            })
            .shared();
        cx.new_model(|_, _| Self {
            _path_task,
            picker,
            connection_string,
            nickname,
        })
    }
}

impl gpui::Render for ProjectPicker {
    fn render(
        &mut self,
        model: &Model<Self>,
        window: &mut gpui::Window,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        v_flex()
            .child(
                SshConnectionHeader {
                    connection_string: self.connection_string.clone(),
                    paths: Default::default(),
                    nickname: self.nickname.clone(),
                }
                .render(model, window, cx),
            )
            .child(
                div()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(self.picker.clone()),
            )
    }
}

#[derive(Clone)]
struct ProjectEntry {
    open_folder: NavigableEntry,
    projects: Vec<(NavigableEntry, SshProject)>,
    configure: NavigableEntry,
    connection: SshConnection,
}

#[derive(Clone)]
struct DefaultState {
    scrollbar: ScrollbarState,
    add_new_server: NavigableEntry,
    servers: Vec<ProjectEntry>,
}
impl DefaultState {
    fn new(window: &Window, cx: &AppContext) -> Self {
        let handle = ScrollHandle::new();
        let scrollbar = ScrollbarState::new(handle.clone());
        let add_new_server = NavigableEntry::new(&handle, model, cx);
        let servers = SshSettings::get_global(cx)
            .ssh_connections()
            .map(|connection| {
                let open_folder = NavigableEntry::new(&handle, model, cx);
                let configure = NavigableEntry::new(&handle, model, cx);
                let projects = connection
                    .projects
                    .iter()
                    .map(|project| (NavigableEntry::new(&handle, model, cx), project.clone()))
                    .collect();
                ProjectEntry {
                    open_folder,
                    configure,
                    projects,
                    connection,
                }
            })
            .collect();
        Self {
            scrollbar,
            add_new_server,
            servers,
        }
    }
}

#[derive(Clone)]
struct ViewServerOptionsState {
    server_index: usize,
    connection: SshConnection,
    entries: [NavigableEntry; 4],
}
enum Mode {
    Default(DefaultState),
    ViewServerOptions(ViewServerOptionsState),
    EditNickname(EditNicknameState),
    ProjectPicker(Model<ProjectPicker>),
    CreateRemoteServer(CreateRemoteServer),
}

impl Mode {
    fn default_mode(window: &Window, cx: &AppContext) -> Self {
        Self::Default(DefaultState::new(cx))
    }
}
impl RemoteServerProjects {
    pub fn register(workspace: &mut Workspace, _: &Model<Workspace>, _: &mut AppContext) {
        workspace.register_action(model, |workspace, _: &OpenRemote, cx| {
            let handle = model.downgrade();
            workspace.toggle_modal(cx, |cx| Self::new(cx, handle))
        });
    }

    pub fn open(workspace: Model<Workspace>, window: &mut gpui::Window, cx: &mut gpui::AppContext) {
        workspace.update(cx, |workspace, model, cx| {
            let handle = model.downgrade();
            workspace.toggle_modal(cx, |cx| Self::new(cx, handle))
        })
    }

    pub fn new(model: &Model<Self>, cx: &mut AppContext, workspace: WeakModel<Workspace>) -> Self {
        let focus_handle = window.focus_handle();

        let mut base_style = window.text_style();
        base_style.refine(&gpui::TextStyleRefinement {
            color: Some(cx.theme().colors().editor_foreground),
            ..Default::default()
        });

        Self {
            mode: Mode::default_mode(cx),
            focus_handle,
            workspace,
            retained_connections: Vec::new(),
        }
    }

    pub fn project_picker(
        ix: usize,
        connection_options: remote::SshConnectionOptions,
        project: Model<Project>,
        model: &Model<Self>,
        cx: &mut AppContext,
        workspace: WeakModel<Workspace>,
    ) -> Self {
        let mut this = Self::new(cx, workspace.clone());
        this.mode = Mode::ProjectPicker(ProjectPicker::new(
            ix,
            connection_options,
            project,
            workspace,
            model,
            cx,
        ));
        model.notify(cx);

        this
    }

    fn create_ssh_server(
        &mut self,
        editor: Model<Editor>,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        let input = get_text(&editor, model, cx);
        if input.is_empty() {
            return;
        }

        let connection_options = match SshConnectionOptions::parse_command_line(&input) {
            Ok(c) => c,
            Err(e) => {
                self.mode = Mode::CreateRemoteServer(CreateRemoteServer {
                    address_editor: editor,
                    address_error: Some(format!("could not parse: {:?}", e).into()),
                    ssh_prompt: None,
                    _creating: None,
                });
                return;
            }
        };
        let ssh_prompt = cx.new_model(|model, cx| SshPrompt::new(&connection_options, model, cx));

        let connection = connect_over_ssh(
            ConnectionIdentifier::setup(),
            connection_options.clone(),
            ssh_prompt.clone(),
            model,
            cx,
        )
        .prompt_err("Failed to connect", cx, |_, _| None);

        let address_editor = editor.clone();
        let creating = cx.spawn(move |this, mut cx| async move {
            match connection.await {
                Some(Some(client)) => this
                    .update(&mut cx, |this, model, cx| {
                        let _ = this.workspace.update(cx, |workspace, model, _| {
                            workspace
                                .client()
                                .telemetry()
                                .report_app_event("create ssh server".to_string())
                        });
                        this.retained_connections.push(client);
                        this.add_ssh_server(connection_options, model, cx);
                        this.mode = Mode::default_mode(model, cx);
                        model.notify(cx)
                    })
                    .log_err(),
                _ => this
                    .update(&mut cx, |this, model, cx| {
                        address_editor.update(cx, |this, model, _| {
                            this.set_read_only(false);
                        });
                        this.mode = Mode::CreateRemoteServer(CreateRemoteServer {
                            address_editor,
                            address_error: None,
                            ssh_prompt: None,
                            _creating: None,
                        });
                        model.notify(cx)
                    })
                    .log_err(),
            };
            None
        });

        editor.update(cx, |this, model, _| {
            this.set_read_only(true);
        });
        self.mode = Mode::CreateRemoteServer(CreateRemoteServer {
            address_editor: editor,
            address_error: None,
            ssh_prompt: Some(ssh_prompt.clone()),
            _creating: Some(creating),
        });
    }

    fn view_server_options(
        &mut self,
        (server_index, connection): (usize, SshConnection),
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        self.mode = Mode::ViewServerOptions(ViewServerOptionsState {
            server_index,
            connection,
            entries: std::array::from_fn(|_| NavigableEntry::focusable(cx)),
        });
        self.focus_handle(cx).focus(model, cx);
        model.notify(cx);
    }

    fn create_ssh_project(
        &mut self,
        ix: usize,
        ssh_connection: SshConnection,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let connection_options = ssh_connection.into();
        workspace.update(cx, |_, model, cx| {
            cx.defer(move |workspace, cx| {
                workspace.toggle_modal(cx, |cx| {
                    SshConnectionModal::new(&connection_options, Vec::new(), model, cx)
                });
                let prompt = workspace
                    .active_modal::<SshConnectionModal>(cx)
                    .unwrap()
                    .read(cx)
                    .prompt
                    .clone();

                let connect = connect_over_ssh(
                    ConnectionIdentifier::setup(),
                    connection_options.clone(),
                    prompt,
                    model,
                    cx,
                )
                .prompt_err("Failed to connect", cx, |_, _| None);

                cx.spawn(move |workspace, mut cx| async move {
                    let session = connect.await;

                    workspace
                        .update(&mut cx, |workspace, cx| {
                            if let Some(prompt) = workspace.active_modal::<SshConnectionModal>(cx) {
                                prompt.update(cx, |prompt, model, cx| prompt.finished(cx))
                            }
                        })
                        .ok();

                    let Some(Some(session)) = session else {
                        workspace
                            .update(&mut cx, |workspace, cx| {
                                let weak = model.downgrade();
                                workspace
                                    .toggle_modal(cx, |cx| RemoteServerProjects::new(cx, weak));
                            })
                            .log_err();
                        return;
                    };

                    workspace
                        .update(&mut cx, |workspace, cx| {
                            let app_state = workspace.app_state().clone();
                            let weak = model.downgrade();
                            let project = project::Project::ssh(
                                session,
                                app_state.client.clone(),
                                app_state.node_runtime.clone(),
                                app_state.user_store.clone(),
                                app_state.languages.clone(),
                                app_state.fs.clone(),
                                cx,
                            );
                            workspace.toggle_modal(cx, |cx| {
                                RemoteServerProjects::project_picker(
                                    ix,
                                    connection_options,
                                    project,
                                    cx,
                                    weak,
                                )
                            });
                        })
                        .ok();
                })
                .detach()
            })
        })
    }

    fn confirm(&mut self, _: &menu::Confirm, model: &Model<Self>, cx: &mut AppContext) {
        match &self.mode {
            Mode::Default(_) | Mode::ViewServerOptions(_) => {}
            Mode::ProjectPicker(_) => {}
            Mode::CreateRemoteServer(state) => {
                if let Some(prompt) = state.ssh_prompt.as_ref() {
                    prompt.update(cx, |prompt, model, cx| {
                        prompt.confirm(cx);
                    });
                    return;
                }

                self.create_ssh_server(state.address_editor.clone(), model, cx);
            }
            Mode::EditNickname(state) => {
                let text = Some(state.editor.read(cx).text(cx)).filter(|text| !text.is_empty());
                let index = state.index;
                self.update_settings_file(cx, move |setting, _| {
                    if let Some(connections) = setting.ssh_connections.as_mut() {
                        if let Some(connection) = connections.get_mut(index) {
                            connection.nickname = text;
                        }
                    }
                });
                self.mode = Mode::default_mode(model, cx);
                self.focus_handle.focus(model, cx);
            }
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, model: &Model<Self>, cx: &mut AppContext) {
        match &self.mode {
            Mode::Default(_) => model.emit(DismissEvent, cx),
            Mode::CreateRemoteServer(state) if state.ssh_prompt.is_some() => {
                let new_state = CreateRemoteServer::new(model, cx);
                let old_prompt = state.address_editor.read(cx).text(model, cx);
                new_state.address_editor.update(cx, |this, model, cx| {
                    this.set_text(old_prompt, cx);
                });

                self.mode = Mode::CreateRemoteServer(new_state);
                model.notify(cx);
            }
            _ => {
                self.mode = Mode::default_mode(model, cx);
                self.focus_handle(cx).focus(model, cx);
                model.notify(cx);
            }
        }
    }

    fn render_ssh_connection(
        &mut self,
        ix: usize,
        ssh_server: ProjectEntry,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        let (main_label, aux_label) = if let Some(nickname) = ssh_server.connection.nickname.clone()
        {
            let aux_label = SharedString::from(format!("({})", ssh_server.connection.host));
            (nickname.into(), Some(aux_label))
        } else {
            (ssh_server.connection.host.clone(), None)
        };
        v_flex()
            .w_full()
            .child(ListSeparator)
            .child(
                h_flex()
                    .group("ssh-server")
                    .w_full()
                    .pt_0p5()
                    .px_3()
                    .gap_1()
                    .overflow_hidden()
                    .child(
                        div().max_w_96().overflow_hidden().text_ellipsis().child(
                            Label::new(main_label)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                    .children(
                        aux_label.map(|label| {
                            Label::new(label).size(LabelSize::Small).color(Color::Muted)
                        }),
                    ),
            )
            .child(
                List::new()
                    .empty_message("No projects.")
                    .children(ssh_server.projects.iter().enumerate().map(|(pix, p)| {
                        v_flex().gap_0p5().child(self.render_ssh_project(
                            ix,
                            &ssh_server,
                            pix,
                            p,
                            model,
                            cx,
                        ))
                    }))
                    .child(
                        h_flex()
                            .id(("new-remote-project-container", ix))
                            .track_focus(&ssh_server.open_folder.focus_handle)
                            .anchor_scroll(ssh_server.open_folder.scroll_anchor.clone())
                            .on_action(cx.listener({
                                let ssh_connection = ssh_server.clone();
                                move |this, _: &menu::Confirm, cx| {
                                    this.create_ssh_project(
                                        ix,
                                        ssh_connection.connection.clone(),
                                        cx,
                                    );
                                }
                            }))
                            .child(
                                ListItem::new(("new-remote-project", ix))
                                    .selected(
                                        ssh_server.open_folder.focus_handle.contains_focused(cx),
                                    )
                                    .inset(true)
                                    .spacing(ui::ListItemSpacing::Sparse)
                                    .start_slot(Icon::new(IconName::Plus).color(Color::Muted))
                                    .child(Label::new("Open Folder"))
                                    .on_click(cx.listener({
                                        let ssh_connection = ssh_server.clone();
                                        move |this, _, cx| {
                                            this.create_ssh_project(
                                                ix,
                                                ssh_connection.connection.clone(),
                                                cx,
                                            );
                                        }
                                    })),
                            ),
                    )
                    .child(
                        h_flex()
                            .id(("server-options-container", ix))
                            .track_focus(&ssh_server.configure.focus_handle)
                            .anchor_scroll(ssh_server.configure.scroll_anchor.clone())
                            .on_action(cx.listener({
                                let ssh_connection = ssh_server.clone();
                                move |this, _: &menu::Confirm, cx| {
                                    this.view_server_options(
                                        (ix, ssh_connection.connection.clone()),
                                        cx,
                                    );
                                }
                            }))
                            .child(
                                ListItem::new(("server-options", ix))
                                    .selected(
                                        ssh_server.configure.focus_handle.contains_focused(cx),
                                    )
                                    .inset(true)
                                    .spacing(ui::ListItemSpacing::Sparse)
                                    .start_slot(Icon::new(IconName::Settings).color(Color::Muted))
                                    .child(Label::new("View Server Options"))
                                    .on_click(cx.listener({
                                        let ssh_connection = ssh_server.clone();
                                        move |this, _, cx| {
                                            this.view_server_options(
                                                (ix, ssh_connection.connection.clone()),
                                                cx,
                                            );
                                        }
                                    })),
                            ),
                    ),
            )
    }

    fn render_ssh_project(
        &mut self,
        server_ix: usize,
        server: &ProjectEntry,
        ix: usize,
        (navigation, project): &(NavigableEntry, SshProject),
        model: &Model<Self>,
        cx: &AppContext,
    ) -> impl IntoElement {
        let server = server.clone();
        let element_id_base = SharedString::from(format!("remote-project-{server_ix}"));
        let container_element_id_base =
            SharedString::from(format!("remote-project-container-{element_id_base}"));

        let callback = Arc::new({
            let project = project.clone();
            move |this: &mut Self, model: &Model<Self>, cx: &mut AppContext| {
                let Some(app_state) = this
                    .workspace
                    .update(cx, |workspace, model, _| workspace.app_state().clone())
                    .log_err()
                else {
                    return;
                };
                let project = project.clone();
                let server = server.connection.clone();
                model.emit(DismissEvent, cx);
                cx.spawn(|_, mut cx| async move {
                    let result = open_ssh_project(
                        server.into(),
                        project.paths.into_iter().map(PathBuf::from).collect(),
                        app_state,
                        OpenOptions::default(),
                        &mut cx,
                    )
                    .await;
                    if let Err(e) = result {
                        log::error!("Failed to connect: {:?}", e);
                        cx.prompt(
                            gpui::PromptLevel::Critical,
                            "Failed to connect",
                            Some(&e.to_string()),
                            &["Ok"],
                        )
                        .await
                        .ok();
                    }
                })
                .detach();
            }
        });

        div()
            .id((container_element_id_base, ix))
            .track_focus(&navigation.focus_handle)
            .anchor_scroll(navigation.scroll_anchor.clone())
            .on_action(cx.listener({
                let callback = callback.clone();
                move |this, _: &menu::Confirm, model, cx| {
                    callback(this, model, cx);
                }
            }))
            .child(
                ListItem::new((element_id_base, ix))
                    .selected(navigation.focus_handle.contains_focused(cx))
                    .inset(true)
                    .spacing(ui::ListItemSpacing::Sparse)
                    .start_slot(
                        Icon::new(IconName::Folder)
                            .color(Color::Muted)
                            .size(IconSize::Small),
                    )
                    .child(Label::new(project.paths.join(", ")))
                    .on_click(model.listener(move |this, _, cx| callback(this, model, cx)))
                    .end_hover_slot::<AnyElement>(Some(
                        div()
                            .mr_2()
                            .child({
                                let project = project.clone();
                                // Right-margin to offset it from the Scrollbar
                                IconButton::new("remove-remote-project", IconName::TrashAlt)
                                    .icon_size(IconSize::Small)
                                    .shape(IconButtonShape::Square)
                                    .size(ButtonSize::Large)
                                    .tooltip(|window, cx| {
                                        Tooltip::text("Delete Remote Project", cx)
                                    })
                                    .on_click(model.listener(move |this, _, cx| {
                                        this.delete_ssh_project(server_ix, &project, cx)
                                    }))
                            })
                            .into_any_element(),
                    )),
            )
    }

    fn update_settings_file(
        &mut self,
        model: &Model<Self>,
        cx: &mut AppContext,
        f: impl FnOnce(&mut RemoteSettingsContent, &AppContext) + Send + Sync + 'static,
    ) {
        let Some(fs) = self
            .workspace
            .update(cx, |workspace, model, _| workspace.app_state().fs.clone())
            .log_err()
        else {
            return;
        };
        update_settings_file::<SshSettings>(fs, cx, move |setting, cx| f(setting, cx));
    }

    fn delete_ssh_server(&mut self, server: usize, model: &Model<Self>, cx: &mut AppContext) {
        self.update_settings_file(cx, move |setting, _| {
            if let Some(connections) = setting.ssh_connections.as_mut() {
                connections.remove(server);
            }
        });
    }

    fn delete_ssh_project(
        &mut self,
        server: usize,
        project: &SshProject,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        let project = project.clone();
        self.update_settings_file(cx, move |setting, _| {
            if let Some(server) = setting
                .ssh_connections
                .as_mut()
                .and_then(|connections| connections.get_mut(server))
            {
                server.projects.remove(&project);
            }
        });
    }

    fn add_ssh_server(
        &mut self,
        connection_options: remote::SshConnectionOptions,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        self.update_settings_file(cx, move |setting, _| {
            setting
                .ssh_connections
                .get_or_insert(Default::default())
                .push(SshConnection {
                    host: SharedString::from(connection_options.host),
                    username: connection_options.username,
                    port: connection_options.port,
                    projects: BTreeSet::<SshProject>::new(),
                    nickname: None,
                    args: connection_options.args.unwrap_or_default(),
                    upload_binary_over_ssh: None,
                })
        });
    }

    fn render_create_remote_server(
        &self,
        state: &CreateRemoteServer,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        let ssh_prompt = state.ssh_prompt.clone();

        state.address_editor.update(cx, |editor, model, cx| {
            if editor.text(cx).is_empty() {
                editor.set_placeholder_text("ssh user@example -p 2222", cx);
            }
        });

        let theme = cx.theme();

        v_flex()
            .track_focus(&self.focus_handle(cx))
            .id("create-remote-server")
            .overflow_hidden()
            .size_full()
            .flex_1()
            .child(
                div()
                    .p_2()
                    .border_b_1()
                    .border_color(theme.colors().border_variant)
                    .child(state.address_editor.clone()),
            )
            .child(
                h_flex()
                    .bg(theme.colors().editor_background)
                    .rounded_b_md()
                    .w_full()
                    .map(|this| {
                        if let Some(ssh_prompt) = ssh_prompt {
                            this.child(h_flex().w_full().child(ssh_prompt))
                        } else if let Some(address_error) = &state.address_error {
                            this.child(
                                h_flex().p_2().w_full().gap_2().child(
                                    Label::new(address_error.clone())
                                        .size(LabelSize::Small)
                                        .color(Color::Error),
                                ),
                            )
                        } else {
                            this.child(
                                h_flex()
                                    .p_2()
                                    .w_full()
                                    .gap_1()
                                    .child(
                                        Label::new(
                                            "Enter the command you use to SSH into this server.",
                                        )
                                        .color(Color::Muted)
                                        .size(LabelSize::Small),
                                    )
                                    .child(
                                        Button::new("learn-more", "Learn more…")
                                            .label_size(LabelSize::Small)
                                            .size(ButtonSize::None)
                                            .color(Color::Accent)
                                            .style(ButtonStyle::Transparent)
                                            .on_click(|_, cx| {
                                                cx.open_url(
                                                    "https://zed.dev/docs/remote-development",
                                                );
                                            }),
                                    ),
                            )
                        }
                    }),
            )
    }

    fn render_view_options(
        &mut self,
        ViewServerOptionsState {
            server_index,
            connection,
            entries,
        }: ViewServerOptionsState,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        let connection_string = connection.host.clone();

        let mut view = Navigable::new(
            div()
                .track_focus(&self.focus_handle(cx))
                .size_full()
                .child(
                    SshConnectionHeader {
                        connection_string: connection_string.clone(),
                        paths: Default::default(),
                        nickname: connection.nickname.clone().map(|s| s.into()),
                    }
                    .render(model, window, cx),
                )
                .child(
                    v_flex()
                        .pb_1()
                        .child(ListSeparator)
                        .child({
                            let label = if connection.nickname.is_some() {
                                "Edit Nickname"
                            } else {
                                "Add Nickname to Server"
                            };
                            div()
                                .id("ssh-options-add-nickname")
                                .track_focus(&entries[0].focus_handle)
                                .on_action(model.listener(move |this, _: &menu::Confirm, cx| {
                                    this.mode = Mode::EditNickname(EditNicknameState::new(
                                        server_index,
                                        model,
                                        cx,
                                    ));
                                    model.notify(cx);
                                }))
                                .child(
                                    ListItem::new("add-nickname")
                                        .selected(entries[0].focus_handle.contains_focused(cx))
                                        .inset(true)
                                        .spacing(ui::ListItemSpacing::Sparse)
                                        .start_slot(Icon::new(IconName::Pencil).color(Color::Muted))
                                        .child(Label::new(label))
                                        .on_click(model.listener(move |this, _, cx| {
                                            this.mode = Mode::EditNickname(EditNicknameState::new(
                                                server_index,
                                                model,
                                                cx,
                                            ));
                                            model.notify(cx);
                                        })),
                                )
                        })
                        .child({
                            let workspace = self.workspace.clone();
                            fn callback(
                                workspace: WeakModel<Workspace>,
                                connection_string: SharedString,
                                window: &mut gpui::Window,
                                cx: &mut gpui::AppContext,
                            ) {
                                cx.write_to_clipboard(ClipboardItem::new_string(
                                    connection_string.to_string(),
                                ));
                                workspace
                                    .update(cx, |this, model, cx| {
                                        struct SshServerAddressCopiedToClipboard;
                                        let notification = format!(
                                            "Copied server address ({}) to clipboard",
                                            connection_string
                                        );

                                        this.show_toast(
                                            Toast::new(
                                                NotificationId::composite::<
                                                    SshServerAddressCopiedToClipboard,
                                                >(
                                                    connection_string.clone()
                                                ),
                                                notification,
                                            )
                                            .autohide(),
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                            div()
                                .id("ssh-options-copy-server-address")
                                .track_focus(&entries[1].focus_handle)
                                .on_action({
                                    let connection_string = connection_string.clone();
                                    let workspace = self.workspace.clone();
                                    move |_: &menu::Confirm, cx| {
                                        callback(
                                            workspace.clone(),
                                            connection_string.clone(),
                                            model,
                                            cx,
                                        );
                                    }
                                })
                                .child(
                                    ListItem::new("copy-server-address")
                                        .selected(entries[1].focus_handle.contains_focused(cx))
                                        .inset(true)
                                        .spacing(ui::ListItemSpacing::Sparse)
                                        .start_slot(Icon::new(IconName::Copy).color(Color::Muted))
                                        .child(Label::new("Copy Server Address"))
                                        .end_hover_slot(
                                            Label::new(connection_string.clone())
                                                .color(Color::Muted),
                                        )
                                        .on_click({
                                            let connection_string = connection_string.clone();
                                            move |_, cx| {
                                                callback(
                                                    workspace.clone(),
                                                    connection_string.clone(),
                                                    model,
                                                    cx,
                                                );
                                            }
                                        }),
                                )
                        })
                        .child({
                            fn remove_ssh_server(
                                remote_servers: Model<RemoteServerProjects>,
                                index: usize,
                                connection_string: SharedString,
                                window: &mut gpui::Window,
                                cx: &mut gpui::AppContext,
                            ) {
                                let prompt_message =
                                    format!("Remove server `{}`?", connection_string);

                                let confirmation = cx.prompt(
                                    PromptLevel::Warning,
                                    &prompt_message,
                                    None,
                                    &["Yes, remove it", "No, keep it"],
                                );

                                cx.spawn(|mut cx| async move {
                                    if confirmation.await.ok() == Some(0) {
                                        remote_servers
                                            .update(&mut cx, |this, model, cx| {
                                                this.delete_ssh_server(index, cx);
                                            })
                                            .ok();
                                        remote_servers
                                            .update(&mut cx, |this, model, cx| {
                                                this.mode = Mode::default_mode(model, cx);
                                                model.notify(cx);
                                            })
                                            .ok();
                                    }
                                    anyhow::Ok(())
                                })
                                .detach_and_log_err(cx);
                            }
                            div()
                                .id("ssh-options-copy-server-address")
                                .track_focus(&entries[2].focus_handle)
                                .on_action(cx.listener({
                                    let connection_string = connection_string.clone();
                                    move |_, _: &menu::Confirm, cx| {
                                        remove_ssh_server(
                                            cx.view().clone(),
                                            server_index,
                                            connection_string.clone(),
                                            model,
                                            cx,
                                        );
                                        cx.focus_self();
                                    }
                                }))
                                .child(
                                    ListItem::new("remove-server")
                                        .selected(entries[2].focus_handle.contains_focused(cx))
                                        .inset(true)
                                        .spacing(ui::ListItemSpacing::Sparse)
                                        .start_slot(Icon::new(IconName::Trash).color(Color::Error))
                                        .child(Label::new("Remove Server").color(Color::Error))
                                        .on_click(model.listener(
                                            move |_, _, model, window, cx| {
                                                remove_ssh_server(
                                                    cx.view().clone(),
                                                    server_index,
                                                    connection_string.clone(),
                                                    model,
                                                    cx,
                                                );
                                                cx.focus_self();
                                            },
                                        )),
                                )
                        })
                        .child(ListSeparator)
                        .child({
                            div()
                                .id("ssh-options-copy-server-address")
                                .track_focus(&entries[3].focus_handle)
                                .on_action(cx.listener(|this, _: &menu::Confirm, cx| {
                                    this.mode = Mode::default_mode(model, cx);
                                    cx.focus_self();
                                    model.notify(cx);
                                }))
                                .child(
                                    ListItem::new("go-back")
                                        .selected(entries[3].focus_handle.contains_focused(cx))
                                        .inset(true)
                                        .spacing(ui::ListItemSpacing::Sparse)
                                        .start_slot(
                                            Icon::new(IconName::ArrowLeft).color(Color::Muted),
                                        )
                                        .child(Label::new("Go Back"))
                                        .on_click(model.listener(|this, model, _, cx| {
                                            this.mode = Mode::default_mode(model, cx);
                                            cx.focus_self();
                                            model.notify(cx)
                                        })),
                                )
                        }),
                )
                .into_any_element(),
        );
        for entry in entries {
            view = view.entry(entry);
        }

        view.render(model, window, cx).into_any_element()
    }

    fn render_edit_nickname(
        &self,
        state: &EditNicknameState,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        let Some(connection) = SshSettings::get_global(cx)
            .ssh_connections()
            .nth(state.index)
        else {
            return v_flex()
                .id("ssh-edit-nickname")
                .track_focus(&self.focus_handle(cx));
        };

        let connection_string = connection.host.clone();
        let nickname = connection.nickname.clone().map(|s| s.into());

        v_flex()
            .id("ssh-edit-nickname")
            .track_focus(&self.focus_handle(cx))
            .child(
                SshConnectionHeader {
                    connection_string,
                    paths: Default::default(),
                    nickname,
                }
                .render(model, window, cx),
            )
            .child(
                h_flex()
                    .p_2()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(state.editor.clone()),
            )
    }

    fn render_default(
        &mut self,
        mut state: DefaultState,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        if SshSettings::get_global(cx)
            .ssh_connections
            .as_ref()
            .map_or(false, |connections| {
                state
                    .servers
                    .iter()
                    .map(|server| &server.connection)
                    .ne(connections.iter())
            })
        {
            self.mode = Mode::default_mode(model, cx);
            if let Mode::Default(new_state) = &self.mode {
                state = new_state.clone();
            }
        }
        let scroll_state = state.scrollbar.parent_view(cx.view());
        let connect_button = div()
            .id("ssh-connect-new-server-container")
            .track_focus(&state.add_new_server.focus_handle)
            .anchor_scroll(state.add_new_server.scroll_anchor.clone())
            .child(
                ListItem::new("register-remove-server-button")
                    .selected(state.add_new_server.focus_handle.contains_focused(cx))
                    .inset(true)
                    .spacing(ui::ListItemSpacing::Sparse)
                    .start_slot(Icon::new(IconName::Plus).color(Color::Muted))
                    .child(Label::new("Connect New Server"))
                    .on_click(model.listener(|this, model, _, cx| {
                        let state = CreateRemoteServer::new(model, cx);
                        this.mode = Mode::CreateRemoteServer(state);

                        model.notify(cx);
                    })),
            )
            .on_action(cx.listener(|this, _: &menu::Confirm, cx| {
                let state = CreateRemoteServer::new(model, cx);
                this.mode = Mode::CreateRemoteServer(state);

                model.notify(cx);
            }));

        let ui::ScrollableHandle::NonUniform(scroll_handle) = scroll_state.scroll_handle() else {
            unreachable!()
        };

        let mut modal_section = Navigable::new(
            v_flex()
                .track_focus(&self.focus_handle(cx))
                .id("ssh-server-list")
                .overflow_y_scroll()
                .track_scroll(&scroll_handle)
                .size_full()
                .child(connect_button)
                .child(
                    List::new()
                        .empty_message(
                            v_flex()
                                .child(
                                    div().px_3().child(
                                        Label::new("No remote servers registered yet.")
                                            .color(Color::Muted),
                                    ),
                                )
                                .into_any_element(),
                        )
                        .children(state.servers.iter().enumerate().map(|(ix, connection)| {
                            self.render_ssh_connection(ix, connection.clone(), model, cx)
                                .into_any_element()
                        })),
                )
                .into_any_element(),
        )
        .entry(state.add_new_server.clone());

        for server in &state.servers {
            for (navigation_state, _) in &server.projects {
                modal_section = modal_section.entry(navigation_state.clone());
            }
            modal_section = modal_section
                .entry(server.open_folder.clone())
                .entry(server.configure.clone());
        }
        let mut modal_section = modal_section.render(model, window, cx).into_any_element();

        Modal::new("remote-projects", None)
            .header(
                ModalHeader::new()
                    .child(Headline::new("Remote Projects (beta)").size(HeadlineSize::XSmall)),
            )
            .section(
                Section::new().padded(false).child(
                    v_flex()
                        .min_h(rems(20.))
                        .size_full()
                        .relative()
                        .child(ListSeparator)
                        .child(
                            canvas(
                                |bounds, cx| {
                                    modal_section.prepaint_as_root(
                                        bounds.origin,
                                        bounds.size.into(),
                                        model,
                                        cx,
                                    );
                                    modal_section
                                },
                                |_, mut modal_section, cx| {
                                    modal_section.paint(cx);
                                },
                            )
                            .size_full(),
                        )
                        .child(
                            div()
                                .occlude()
                                .h_full()
                                .absolute()
                                .top_1()
                                .bottom_1()
                                .right_1()
                                .w(px(8.))
                                .children(Scrollbar::vertical(scroll_state)),
                        ),
                ),
            )
            .into_any_element()
    }
}

fn get_text(
    element: &Model<Editor>,
    window: &mut gpui::Window,
    cx: &mut gpui::AppContext,
) -> String {
    element.read(cx).text(cx).trim().to_string()
}

impl ModalView for RemoteServerProjects {}

impl FocusableView for RemoteServerProjects {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        match &self.mode {
            Mode::ProjectPicker(picker) => picker.focus_handle(cx),
            _ => self.focus_handle.clone(),
        }
    }
}

impl EventEmitter<DismissEvent> for RemoteServerProjects {}

impl Render for RemoteServerProjects {
    fn render(
        &mut self,
        model: &Model<Self>,
        window: &mut gpui::Window,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        div()
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("RemoteServerModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .capture_any_mouse_down(model.listener(|this, model, _, cx| {
                this.focus_handle(cx).focus(window);
            }))
            .on_mouse_down_out(model.listener(|this, model, _, cx| {
                if matches!(this.mode, Mode::Default(_)) {
                    model.emit(DismissEvent, cx)
                }
            }))
            .child(match &self.mode {
                Mode::Default(state) => self
                    .render_default(state.clone(), model, cx)
                    .into_any_element(),
                Mode::ViewServerOptions(state) => self
                    .render_view_options(state.clone(), model, cx)
                    .into_any_element(),
                Mode::ProjectPicker(element) => element.clone().into_any_element(),
                Mode::CreateRemoteServer(state) => self
                    .render_create_remote_server(state, model, cx)
                    .into_any_element(),
                Mode::EditNickname(state) => self
                    .render_edit_nickname(state, model, cx)
                    .into_any_element(),
            })
    }
}
