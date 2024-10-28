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
    PromptLevel, ScrollHandle, View, ViewContext,
};
use picker::Picker;
use project::Project;
use remote::SshConnectionOptions;
use remote::SshRemoteClient;
use settings::update_settings_file;
use settings::Settings;
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

pub struct RemoteServerProjects {
    mode: Mode,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    workspace: WeakView<Workspace>,
    selectable_items: SelectableItemList,
    retained_connections: Vec<Model<SshRemoteClient>>,
}

struct CreateRemoteServer {
    address_editor: View<Editor>,
    address_error: Option<SharedString>,
    ssh_prompt: Option<View<SshPrompt>>,
    _creating: Option<Task<Option<()>>>,
}

impl CreateRemoteServer {
    fn new(cx: &mut WindowContext<'_>) -> Self {
        let address_editor = cx.new_view(Editor::single_line);
        address_editor.update(cx, |this, cx| {
            this.focus_handle(cx).focus(cx);
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
    picker: View<Picker<OpenPathDelegate>>,
    _path_task: Shared<Task<Option<()>>>,
}

type SelectedItemCallback =
    Box<dyn Fn(&mut RemoteServerProjects, &mut ViewContext<RemoteServerProjects>) + 'static>;

/// Used to implement keyboard navigation for SSH modal.
#[derive(Default)]
struct SelectableItemList {
    items: Vec<SelectedItemCallback>,
    active_item: Option<usize>,
}

struct EditNicknameState {
    index: usize,
    editor: View<Editor>,
}

impl EditNicknameState {
    fn new(index: usize, cx: &mut WindowContext<'_>) -> Self {
        let this = Self {
            index,
            editor: cx.new_view(Editor::single_line),
        };
        let starting_text = SshSettings::get_global(cx)
            .ssh_connections()
            .nth(index)
            .and_then(|state| state.nickname.clone())
            .filter(|text| !text.is_empty());
        this.editor.update(cx, |this, cx| {
            this.set_placeholder_text("Add a nickname for this server", cx);
            if let Some(starting_text) = starting_text {
                this.set_text(starting_text, cx);
            }
        });
        this.editor.focus_handle(cx).focus(cx);
        this
    }
}

impl SelectableItemList {
    fn reset(&mut self) {
        self.items.clear();
    }

    fn reset_selection(&mut self) {
        self.active_item.take();
    }

    fn prev(&mut self, _: &mut WindowContext<'_>) {
        match self.active_item.as_mut() {
            Some(active_index) => {
                *active_index = active_index.checked_sub(1).unwrap_or(self.items.len() - 1)
            }
            None => {
                self.active_item = Some(self.items.len() - 1);
            }
        }
    }

    fn next(&mut self, _: &mut WindowContext<'_>) {
        match self.active_item.as_mut() {
            Some(active_index) => {
                if *active_index + 1 < self.items.len() {
                    *active_index += 1;
                } else {
                    *active_index = 0;
                }
            }
            None => {
                self.active_item = Some(0);
            }
        }
    }

    fn add_item(&mut self, callback: SelectedItemCallback) {
        self.items.push(callback)
    }

    fn is_selected(&self) -> bool {
        self.active_item == self.items.len().checked_sub(1)
    }

    fn confirm(
        &self,
        remote_modal: &mut RemoteServerProjects,
        cx: &mut ViewContext<RemoteServerProjects>,
    ) {
        if let Some(active_item) = self.active_item.and_then(|ix| self.items.get(ix)) {
            active_item(remote_modal, cx);
        }
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
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<RemoteServerProjects>,
    ) -> View<Self> {
        let (tx, rx) = oneshot::channel();
        let lister = project::DirectoryLister::Project(project.clone());
        let query = lister.default_query(cx);
        let delegate = file_finder::OpenPathDelegate::new(tx, lister);

        let picker = cx.new_view(|cx| {
            let picker = Picker::uniform_list(delegate, cx)
                .width(rems(34.))
                .modal(false);
            picker.set_query(query, cx);
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
                                let weak = cx.view().downgrade();
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
                                    server.projects.push(SshProject { paths })
                                }
                            }
                        });

                        let tasks = paths
                            .into_iter()
                            .map(|path| {
                                project.update(cx, |project, cx| {
                                    project.find_or_create_worktree(&path, true, cx)
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

                        cx.new_view(|cx| {
                            let workspace =
                                Workspace::new(None, project.clone(), app_state.clone(), cx);

                            workspace
                                .client()
                                .telemetry()
                                .report_app_event("create ssh project".to_string());

                            workspace
                        })
                    })
                    .log_err();
                    this.update(&mut cx, |_, cx| {
                        cx.emit(DismissEvent);
                    })
                    .ok();
                    Some(())
                }
            })
            .shared();
        cx.new_view(|_| Self {
            _path_task,
            picker,
            connection_string,
            nickname,
        })
    }
}

impl gpui::Render for ProjectPicker {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .child(
                SshConnectionHeader {
                    connection_string: self.connection_string.clone(),
                    paths: Default::default(),
                    nickname: self.nickname.clone(),
                }
                .render(cx),
            )
            .child(
                div()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(self.picker.clone()),
            )
    }
}
enum Mode {
    Default(ScrollbarState),
    ViewServerOptions(usize, SshConnection),
    EditNickname(EditNicknameState),
    ProjectPicker(View<ProjectPicker>),
    CreateRemoteServer(CreateRemoteServer),
}

impl Mode {
    fn default_mode() -> Self {
        let handle = ScrollHandle::new();
        Self::Default(ScrollbarState::new(handle))
    }
}
impl RemoteServerProjects {
    pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(|workspace, _: &OpenRemote, cx| {
            let handle = cx.view().downgrade();
            workspace.toggle_modal(cx, |cx| Self::new(cx, handle))
        });
    }

    pub fn open(workspace: View<Workspace>, cx: &mut WindowContext) {
        workspace.update(cx, |workspace, cx| {
            let handle = cx.view().downgrade();
            workspace.toggle_modal(cx, |cx| Self::new(cx, handle))
        })
    }

    pub fn new(cx: &mut ViewContext<Self>, workspace: WeakView<Workspace>) -> Self {
        let focus_handle = cx.focus_handle();

        let mut base_style = cx.text_style();
        base_style.refine(&gpui::TextStyleRefinement {
            color: Some(cx.theme().colors().editor_foreground),
            ..Default::default()
        });

        Self {
            mode: Mode::default_mode(),
            focus_handle,
            scroll_handle: ScrollHandle::new(),
            workspace,
            selectable_items: Default::default(),
            retained_connections: Vec::new(),
        }
    }

    fn next_item(&mut self, _: &menu::SelectNext, cx: &mut ViewContext<Self>) {
        if !matches!(self.mode, Mode::Default(_) | Mode::ViewServerOptions(_, _)) {
            return;
        }

        self.selectable_items.next(cx);
    }

    fn prev_item(&mut self, _: &menu::SelectPrev, cx: &mut ViewContext<Self>) {
        if !matches!(self.mode, Mode::Default(_) | Mode::ViewServerOptions(_, _)) {
            return;
        }
        self.selectable_items.prev(cx);
    }

    pub fn project_picker(
        ix: usize,
        connection_options: remote::SshConnectionOptions,
        project: Model<Project>,
        cx: &mut ViewContext<Self>,
        workspace: WeakView<Workspace>,
    ) -> Self {
        let mut this = Self::new(cx, workspace.clone());
        this.mode = Mode::ProjectPicker(ProjectPicker::new(
            ix,
            connection_options,
            project,
            workspace,
            cx,
        ));
        cx.notify();

        this
    }

    fn create_ssh_server(&mut self, editor: View<Editor>, cx: &mut ViewContext<Self>) {
        let input = get_text(&editor, cx);
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
        let ssh_prompt = cx.new_view(|cx| SshPrompt::new(&connection_options, cx));

        let connection = connect_over_ssh(
            connection_options.remote_server_identifier(),
            connection_options.clone(),
            ssh_prompt.clone(),
            cx,
        )
        .prompt_err("Failed to connect", cx, |_, _| None);

        let address_editor = editor.clone();
        let creating = cx.spawn(move |this, mut cx| async move {
            match connection.await {
                Some(Some(client)) => this
                    .update(&mut cx, |this, cx| {
                        let _ = this.workspace.update(cx, |workspace, _| {
                            workspace
                                .client()
                                .telemetry()
                                .report_app_event("create ssh server".to_string())
                        });
                        this.retained_connections.push(client);
                        this.add_ssh_server(connection_options, cx);
                        this.mode = Mode::default_mode();
                        this.selectable_items.reset_selection();
                        cx.notify()
                    })
                    .log_err(),
                _ => this
                    .update(&mut cx, |this, cx| {
                        address_editor.update(cx, |this, _| {
                            this.set_read_only(false);
                        });
                        this.mode = Mode::CreateRemoteServer(CreateRemoteServer {
                            address_editor,
                            address_error: None,
                            ssh_prompt: None,
                            _creating: None,
                        });
                        cx.notify()
                    })
                    .log_err(),
            };
            None
        });

        editor.update(cx, |this, _| {
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
        (index, connection): (usize, SshConnection),
        cx: &mut ViewContext<Self>,
    ) {
        self.selectable_items.reset_selection();
        self.mode = Mode::ViewServerOptions(index, connection);
        cx.notify();
    }

    fn create_ssh_project(
        &mut self,
        ix: usize,
        ssh_connection: SshConnection,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let connection_options = ssh_connection.into();
        workspace.update(cx, |_, cx| {
            cx.defer(move |workspace, cx| {
                workspace.toggle_modal(cx, |cx| {
                    SshConnectionModal::new(&connection_options, Vec::new(), cx)
                });
                let prompt = workspace
                    .active_modal::<SshConnectionModal>(cx)
                    .unwrap()
                    .read(cx)
                    .prompt
                    .clone();

                let connect = connect_over_ssh(
                    connection_options.remote_server_identifier(),
                    connection_options.clone(),
                    prompt,
                    cx,
                )
                .prompt_err("Failed to connect", cx, |_, _| None);

                cx.spawn(move |workspace, mut cx| async move {
                    let session = connect.await;

                    workspace
                        .update(&mut cx, |workspace, cx| {
                            if let Some(prompt) = workspace.active_modal::<SshConnectionModal>(cx) {
                                prompt.update(cx, |prompt, cx| prompt.finished(cx))
                            }
                        })
                        .ok();

                    let Some(Some(session)) = session else {
                        workspace
                            .update(&mut cx, |workspace, cx| {
                                let weak = cx.view().downgrade();
                                workspace
                                    .toggle_modal(cx, |cx| RemoteServerProjects::new(cx, weak));
                            })
                            .log_err();
                        return;
                    };

                    workspace
                        .update(&mut cx, |workspace, cx| {
                            let app_state = workspace.app_state().clone();
                            let weak = cx.view().downgrade();
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

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        match &self.mode {
            Mode::Default(_) | Mode::ViewServerOptions(_, _) => {
                let items = std::mem::take(&mut self.selectable_items);
                items.confirm(self, cx);
                self.selectable_items = items;
            }
            Mode::ProjectPicker(_) => {}
            Mode::CreateRemoteServer(state) => {
                if let Some(prompt) = state.ssh_prompt.as_ref() {
                    prompt.update(cx, |prompt, cx| {
                        prompt.confirm(cx);
                    });
                    return;
                }

                self.create_ssh_server(state.address_editor.clone(), cx);
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
                self.mode = Mode::default_mode();
                self.selectable_items.reset_selection();
                self.focus_handle.focus(cx);
            }
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        match &self.mode {
            Mode::Default(_) => cx.emit(DismissEvent),
            Mode::CreateRemoteServer(state) if state.ssh_prompt.is_some() => {
                let new_state = CreateRemoteServer::new(cx);
                let old_prompt = state.address_editor.read(cx).text(cx);
                new_state.address_editor.update(cx, |this, cx| {
                    this.set_text(old_prompt, cx);
                });

                self.mode = Mode::CreateRemoteServer(new_state);
                self.selectable_items.reset_selection();
                cx.notify();
            }
            _ => {
                self.mode = Mode::default_mode();
                self.selectable_items.reset_selection();
                self.focus_handle(cx).focus(cx);
                cx.notify();
            }
        }
    }

    fn render_ssh_connection(
        &mut self,
        ix: usize,
        ssh_connection: SshConnection,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let (main_label, aux_label) = if let Some(nickname) = ssh_connection.nickname.clone() {
            let aux_label = SharedString::from(format!("({})", ssh_connection.host));
            (nickname.into(), Some(aux_label))
        } else {
            (ssh_connection.host.clone(), None)
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
                    .children(ssh_connection.projects.iter().enumerate().map(|(pix, p)| {
                        v_flex().gap_0p5().child(self.render_ssh_project(
                            ix,
                            &ssh_connection,
                            pix,
                            p,
                            cx,
                        ))
                    }))
                    .child(h_flex().map(|this| {
                        self.selectable_items.add_item(Box::new({
                            let ssh_connection = ssh_connection.clone();
                            move |this, cx| {
                                this.create_ssh_project(ix, ssh_connection.clone(), cx);
                            }
                        }));
                        let is_selected = self.selectable_items.is_selected();
                        this.child(
                            ListItem::new(("new-remote-project", ix))
                                .selected(is_selected)
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::Plus).color(Color::Muted))
                                .child(Label::new("Open Folder"))
                                .on_click(cx.listener({
                                    let ssh_connection = ssh_connection.clone();
                                    move |this, _, cx| {
                                        this.create_ssh_project(ix, ssh_connection.clone(), cx);
                                    }
                                })),
                        )
                    }))
                    .child(h_flex().map(|this| {
                        self.selectable_items.add_item(Box::new({
                            let ssh_connection = ssh_connection.clone();
                            move |this, cx| {
                                this.view_server_options((ix, ssh_connection.clone()), cx);
                            }
                        }));
                        let is_selected = self.selectable_items.is_selected();
                        this.child(
                            ListItem::new(("server-options", ix))
                                .selected(is_selected)
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::Settings).color(Color::Muted))
                                .child(Label::new("View Server Options"))
                                .on_click(cx.listener({
                                    let ssh_connection = ssh_connection.clone();
                                    move |this, _, cx| {
                                        this.view_server_options((ix, ssh_connection.clone()), cx);
                                    }
                                })),
                        )
                    })),
            )
    }

    fn render_ssh_project(
        &mut self,
        server_ix: usize,
        server: &SshConnection,
        ix: usize,
        project: &SshProject,
        cx: &ViewContext<Self>,
    ) -> impl IntoElement {
        let server = server.clone();

        let element_id_base = SharedString::from(format!("remote-project-{server_ix}"));
        let callback = Arc::new({
            let project = project.clone();
            move |this: &mut Self, cx: &mut ViewContext<Self>| {
                let Some(app_state) = this
                    .workspace
                    .update(cx, |workspace, _| workspace.app_state().clone())
                    .log_err()
                else {
                    return;
                };
                let project = project.clone();
                let server = server.clone();
                cx.emit(DismissEvent);
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
        self.selectable_items.add_item(Box::new({
            let callback = callback.clone();
            move |this, cx| callback(this, cx)
        }));
        let is_selected = self.selectable_items.is_selected();

        ListItem::new((element_id_base, ix))
            .inset(true)
            .selected(is_selected)
            .spacing(ui::ListItemSpacing::Sparse)
            .start_slot(
                Icon::new(IconName::Folder)
                    .color(Color::Muted)
                    .size(IconSize::Small),
            )
            .child(Label::new(project.paths.join(", ")))
            .on_click(cx.listener(move |this, _, cx| callback(this, cx)))
            .end_hover_slot::<AnyElement>(Some(
                div()
                    .mr_2()
                    .child(
                        // Right-margin to offset it from the Scrollbar
                        IconButton::new("remove-remote-project", IconName::TrashAlt)
                            .icon_size(IconSize::Small)
                            .shape(IconButtonShape::Square)
                            .size(ButtonSize::Large)
                            .tooltip(|cx| Tooltip::text("Delete Remote Project", cx))
                            .on_click(cx.listener(move |this, _, cx| {
                                this.delete_ssh_project(server_ix, ix, cx)
                            })),
                    )
                    .into_any_element(),
            ))
    }

    fn update_settings_file(
        &mut self,
        cx: &mut ViewContext<Self>,
        f: impl FnOnce(&mut RemoteSettingsContent, &AppContext) + Send + Sync + 'static,
    ) {
        let Some(fs) = self
            .workspace
            .update(cx, |workspace, _| workspace.app_state().fs.clone())
            .log_err()
        else {
            return;
        };
        update_settings_file::<SshSettings>(fs, cx, move |setting, cx| f(setting, cx));
    }

    fn delete_ssh_server(&mut self, server: usize, cx: &mut ViewContext<Self>) {
        self.update_settings_file(cx, move |setting, _| {
            if let Some(connections) = setting.ssh_connections.as_mut() {
                connections.remove(server);
            }
        });
    }

    fn delete_ssh_project(&mut self, server: usize, project: usize, cx: &mut ViewContext<Self>) {
        self.update_settings_file(cx, move |setting, _| {
            if let Some(server) = setting
                .ssh_connections
                .as_mut()
                .and_then(|connections| connections.get_mut(server))
            {
                server.projects.remove(project);
            }
        });
    }

    fn add_ssh_server(
        &mut self,
        connection_options: remote::SshConnectionOptions,
        cx: &mut ViewContext<Self>,
    ) {
        self.update_settings_file(cx, move |setting, _| {
            setting
                .ssh_connections
                .get_or_insert(Default::default())
                .push(SshConnection {
                    host: SharedString::from(connection_options.host),
                    username: connection_options.username,
                    port: connection_options.port,
                    projects: vec![],
                    nickname: None,
                    args: connection_options.args.unwrap_or_default(),
                    upload_binary_over_ssh: None,
                })
        });
    }

    fn render_create_remote_server(
        &self,
        state: &CreateRemoteServer,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let ssh_prompt = state.ssh_prompt.clone();

        state.address_editor.update(cx, |editor, cx| {
            if editor.text(cx).is_empty() {
                editor.set_placeholder_text("ssh user@example -p 2222", cx);
            }
        });

        let theme = cx.theme();

        v_flex()
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
        index: usize,
        connection: SshConnection,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let connection_string = connection.host.clone();

        div()
            .size_full()
            .child(
                SshConnectionHeader {
                    connection_string: connection_string.clone(),
                    paths: Default::default(),
                    nickname: connection.nickname.clone().map(|s| s.into()),
                }
                .render(cx),
            )
            .child(
                v_flex()
                    .pb_1()
                    .child(ListSeparator)
                    .child({
                        self.selectable_items.add_item(Box::new({
                            move |this, cx| {
                                this.mode = Mode::EditNickname(EditNicknameState::new(index, cx));
                                cx.notify();
                            }
                        }));
                        let is_selected = self.selectable_items.is_selected();
                        let label = if connection.nickname.is_some() {
                            "Edit Nickname"
                        } else {
                            "Add Nickname to Server"
                        };
                        ListItem::new("add-nickname")
                            .selected(is_selected)
                            .inset(true)
                            .spacing(ui::ListItemSpacing::Sparse)
                            .start_slot(Icon::new(IconName::Pencil).color(Color::Muted))
                            .child(Label::new(label))
                            .on_click(cx.listener(move |this, _, cx| {
                                this.mode = Mode::EditNickname(EditNicknameState::new(index, cx));
                                cx.notify();
                            }))
                    })
                    .child({
                        let workspace = self.workspace.clone();
                        fn callback(
                            workspace: WeakView<Workspace>,
                            connection_string: SharedString,
                            cx: &mut WindowContext<'_>,
                        ) {
                            cx.write_to_clipboard(ClipboardItem::new_string(
                                connection_string.to_string(),
                            ));
                            workspace
                                .update(cx, |this, cx| {
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
                        self.selectable_items.add_item(Box::new({
                            let workspace = workspace.clone();
                            let connection_string = connection_string.clone();
                            move |_, cx| {
                                callback(workspace.clone(), connection_string.clone(), cx);
                            }
                        }));
                        let is_selected = self.selectable_items.is_selected();
                        ListItem::new("copy-server-address")
                            .selected(is_selected)
                            .inset(true)
                            .spacing(ui::ListItemSpacing::Sparse)
                            .start_slot(Icon::new(IconName::Copy).color(Color::Muted))
                            .child(Label::new("Copy Server Address"))
                            .end_hover_slot(
                                Label::new(connection_string.clone()).color(Color::Muted),
                            )
                            .on_click({
                                let connection_string = connection_string.clone();
                                move |_, cx| {
                                    callback(workspace.clone(), connection_string.clone(), cx);
                                }
                            })
                    })
                    .child({
                        fn remove_ssh_server(
                            remote_servers: View<RemoteServerProjects>,
                            index: usize,
                            connection_string: SharedString,
                            cx: &mut WindowContext<'_>,
                        ) {
                            let prompt_message = format!("Remove server `{}`?", connection_string);

                            let confirmation = cx.prompt(
                                PromptLevel::Warning,
                                &prompt_message,
                                None,
                                &["Yes, remove it", "No, keep it"],
                            );

                            cx.spawn(|mut cx| async move {
                                if confirmation.await.ok() == Some(0) {
                                    remote_servers
                                        .update(&mut cx, |this, cx| {
                                            this.delete_ssh_server(index, cx);
                                            this.mode = Mode::default_mode();
                                            cx.notify();
                                        })
                                        .ok();
                                }
                                anyhow::Ok(())
                            })
                            .detach_and_log_err(cx);
                        }
                        self.selectable_items.add_item(Box::new({
                            let connection_string = connection_string.clone();
                            move |_, cx| {
                                remove_ssh_server(
                                    cx.view().clone(),
                                    index,
                                    connection_string.clone(),
                                    cx,
                                );
                            }
                        }));
                        let is_selected = self.selectable_items.is_selected();
                        ListItem::new("remove-server")
                            .selected(is_selected)
                            .inset(true)
                            .spacing(ui::ListItemSpacing::Sparse)
                            .start_slot(Icon::new(IconName::Trash).color(Color::Error))
                            .child(Label::new("Remove Server").color(Color::Error))
                            .on_click(cx.listener(move |_, _, cx| {
                                remove_ssh_server(
                                    cx.view().clone(),
                                    index,
                                    connection_string.clone(),
                                    cx,
                                );
                            }))
                    })
                    .child(ListSeparator)
                    .child({
                        self.selectable_items.add_item(Box::new({
                            move |this, cx| {
                                this.mode = Mode::default_mode();
                                cx.notify();
                            }
                        }));
                        let is_selected = self.selectable_items.is_selected();
                        ListItem::new("go-back")
                            .selected(is_selected)
                            .inset(true)
                            .spacing(ui::ListItemSpacing::Sparse)
                            .start_slot(Icon::new(IconName::ArrowLeft).color(Color::Muted))
                            .child(Label::new("Go Back"))
                            .on_click(cx.listener(|this, _, cx| {
                                this.mode = Mode::default_mode();
                                cx.notify()
                            }))
                    }),
            )
    }

    fn render_edit_nickname(
        &self,
        state: &EditNicknameState,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let Some(connection) = SshSettings::get_global(cx)
            .ssh_connections()
            .nth(state.index)
        else {
            return v_flex();
        };

        let connection_string = connection.host.clone();
        let nickname = connection.nickname.clone().map(|s| s.into());

        v_flex()
            .child(
                SshConnectionHeader {
                    connection_string,
                    paths: Default::default(),
                    nickname,
                }
                .render(cx),
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
        scroll_state: ScrollbarState,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let scroll_state = scroll_state.parent_view(cx.view());
        let ssh_connections = SshSettings::get_global(cx)
            .ssh_connections()
            .collect::<Vec<_>>();
        self.selectable_items.add_item(Box::new(|this, cx| {
            this.mode = Mode::CreateRemoteServer(CreateRemoteServer::new(cx));
            cx.notify();
        }));

        let is_selected = self.selectable_items.is_selected();

        let connect_button = ListItem::new("register-remove-server-button")
            .selected(is_selected)
            .inset(true)
            .spacing(ui::ListItemSpacing::Sparse)
            .start_slot(Icon::new(IconName::Plus).color(Color::Muted))
            .child(Label::new("Connect New Server"))
            .on_click(cx.listener(|this, _, cx| {
                let state = CreateRemoteServer::new(cx);
                this.mode = Mode::CreateRemoteServer(state);

                cx.notify();
            }));

        let ui::ScrollableHandle::NonUniform(scroll_handle) = scroll_state.scroll_handle() else {
            unreachable!()
        };

        let mut modal_section = v_flex()
            .id("ssh-server-list")
            .overflow_y_scroll()
            .track_scroll(&scroll_handle)
            .size_full()
            .child(connect_button)
            .child(
                List::new()
                    .empty_message(
                        v_flex()
                            .child(div().px_3().child(
                                Label::new("No remote servers registered yet.").color(Color::Muted),
                            ))
                            .into_any_element(),
                    )
                    .children(ssh_connections.iter().cloned().enumerate().map(
                        |(ix, connection)| {
                            self.render_ssh_connection(ix, connection, cx)
                                .into_any_element()
                        },
                    )),
            )
            .into_any_element();

        Modal::new("remote-projects", Some(self.scroll_handle.clone()))
            .header(
                ModalHeader::new()
                    .child(Headline::new("Remote Projects (alpha)").size(HeadlineSize::XSmall)),
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
    }
}

fn get_text(element: &View<Editor>, cx: &mut WindowContext) -> String {
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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.selectable_items.reset();
        div()
            .track_focus(&self.focus_handle)
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("RemoteServerModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::prev_item))
            .on_action(cx.listener(Self::next_item))
            .capture_any_mouse_down(cx.listener(|this, _, cx| {
                this.focus_handle(cx).focus(cx);
            }))
            .on_mouse_down_out(cx.listener(|this, _, cx| {
                if matches!(this.mode, Mode::Default(_)) {
                    cx.emit(DismissEvent)
                }
            }))
            .child(match &self.mode {
                Mode::Default(state) => self.render_default(state.clone(), cx).into_any_element(),
                Mode::ViewServerOptions(index, connection) => self
                    .render_view_options(*index, connection.clone(), cx)
                    .into_any_element(),
                Mode::ProjectPicker(element) => element.clone().into_any_element(),
                Mode::CreateRemoteServer(state) => self
                    .render_create_remote_server(state, cx)
                    .into_any_element(),
                Mode::EditNickname(state) => {
                    self.render_edit_nickname(state, cx).into_any_element()
                }
            })
    }
}
