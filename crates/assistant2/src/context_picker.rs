mod directory_context_picker;
mod fetch_context_picker;
mod file_context_picker;
mod thread_context_picker;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use editor::Editor;
use file_context_picker::render_file_context_entry;
use gpui::{App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Task, WeakEntity};
use project::ProjectPath;
use thread_context_picker::{render_thread_context_entry, ThreadContextEntry};
use ui::{prelude::*, ContextMenu, ContextMenuEntry, ContextMenuItem};
use workspace::{notifications::NotifyResultExt, Workspace};

use crate::context::ContextKind;
use crate::context_picker::directory_context_picker::DirectoryContextPicker;
use crate::context_picker::fetch_context_picker::FetchContextPicker;
use crate::context_picker::file_context_picker::FileContextPicker;
use crate::context_picker::thread_context_picker::ThreadContextPicker;
use crate::context_store::ContextStore;
use crate::thread_store::ThreadStore;
use crate::AssistantPanel;

#[derive(Debug, Clone, Copy)]
pub enum ConfirmBehavior {
    KeepOpen,
    Close,
}

#[derive(Debug, Clone)]
enum ContextPickerMode {
    Default(Entity<ContextMenu>),
    File(Entity<FileContextPicker>),
    Directory(Entity<DirectoryContextPicker>),
    Fetch(Entity<FetchContextPicker>),
    Thread(Entity<ThreadContextPicker>),
}

pub(super) struct ContextPicker {
    mode: ContextPickerMode,
    workspace: WeakEntity<Workspace>,
    editor: WeakEntity<Editor>,
    context_store: WeakEntity<ContextStore>,
    thread_store: Option<WeakEntity<ThreadStore>>,
    confirm_behavior: ConfirmBehavior,
}

impl ContextPicker {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        thread_store: Option<WeakEntity<ThreadStore>>,
        context_store: WeakEntity<ContextStore>,
        editor: WeakEntity<Editor>,
        confirm_behavior: ConfirmBehavior,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        ContextPicker {
            mode: ContextPickerMode::Default(ContextMenu::build(
                window,
                cx,
                |menu, _window, _cx| menu,
            )),
            workspace,
            context_store,
            thread_store,
            editor,
            confirm_behavior,
        }
    }

    pub fn init(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.mode = ContextPickerMode::Default(self.build_menu(window, cx));
        cx.notify();
    }

    fn build_menu(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Entity<ContextMenu> {
        let context_picker = cx.model().clone();

        let menu = ContextMenu::build(window, cx, move |menu, _window, cx| {
            let recent = self.recent_entries(cx);
            let has_recent = !recent.is_empty();
            let recent_entries = recent
                .into_iter()
                .enumerate()
                .map(|(ix, entry)| self.recent_menu_item(context_picker.clone(), ix, entry));

            let mut context_kinds = vec![
                ContextKind::File,
                ContextKind::Directory,
                ContextKind::FetchedUrl,
            ];
            if self.allow_threads() {
                context_kinds.push(ContextKind::Thread);
            }

            let menu = menu
                .when(has_recent, |menu| {
                    menu.custom_row(|_, _| {
                        div()
                            .mb_1()
                            .child(
                                Label::new("Recent")
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                            )
                            .into_any_element()
                    })
                })
                .extend(recent_entries)
                .when(has_recent, |menu| menu.separator())
                .extend(context_kinds.into_iter().map(|kind| {
                    let context_picker = context_picker.clone();

                    ContextMenuEntry::new(kind.label())
                        .icon(kind.icon())
                        .icon_size(IconSize::XSmall)
                        .icon_color(Color::Muted)
                        .handler(move |window, cx| {
                            context_picker.update(cx, |this, cx| this.select_kind(kind, window, cx))
                        })
                }));

            match self.confirm_behavior {
                ConfirmBehavior::KeepOpen => menu.keep_open_on_confirm(),
                ConfirmBehavior::Close => menu,
            }
        });

        cx.subscribe(&menu, move |_, _, _: &DismissEvent, cx| {
            cx.emit(DismissEvent);
        })
        .detach();

        menu
    }

    /// Whether threads are allowed as context.
    pub fn allow_threads(&self) -> bool {
        self.thread_store.is_some()
    }

    fn select_kind(&mut self, kind: ContextKind, window: &mut Window, cx: &mut Context<Self>) {
        let context_picker = cx.model().downgrade();

        match kind {
            ContextKind::File => {
                self.mode = ContextPickerMode::File(cx.new(|cx| {
                    FileContextPicker::new(
                        context_picker.clone(),
                        self.workspace.clone(),
                        self.editor.clone(),
                        self.context_store.clone(),
                        self.confirm_behavior,
                        window,
                        cx,
                    )
                }));
            }
            ContextKind::Directory => {
                self.mode = ContextPickerMode::Directory(cx.new(|cx| {
                    DirectoryContextPicker::new(
                        context_picker.clone(),
                        self.workspace.clone(),
                        self.context_store.clone(),
                        self.confirm_behavior,
                        window,
                        cx,
                    )
                }));
            }
            ContextKind::FetchedUrl => {
                self.mode = ContextPickerMode::Fetch(cx.new(|cx| {
                    FetchContextPicker::new(
                        context_picker.clone(),
                        self.workspace.clone(),
                        self.context_store.clone(),
                        self.confirm_behavior,
                        window,
                        cx,
                    )
                }));
            }
            ContextKind::Thread => {
                if let Some(thread_store) = self.thread_store.as_ref() {
                    self.mode = ContextPickerMode::Thread(cx.new(|cx| {
                        ThreadContextPicker::new(
                            thread_store.clone(),
                            context_picker.clone(),
                            self.context_store.clone(),
                            self.confirm_behavior,
                            window,
                            cx,
                        )
                    }));
                }
            }
        }

        cx.notify();
        cx.focus_self(window);
    }

    fn recent_menu_item(
        &self,
        context_picker: Entity<ContextPicker>,
        ix: usize,
        entry: RecentEntry,
    ) -> ContextMenuItem {
        match entry {
            RecentEntry::File {
                project_path,
                path_prefix,
            } => {
                let context_store = self.context_store.clone();
                let path = project_path.path.clone();

                ContextMenuItem::custom_entry(
                    move |_window, cx| {
                        render_file_context_entry(
                            ElementId::NamedInteger("ctx-recent".into(), ix),
                            &path,
                            &path_prefix,
                            context_store.clone(),
                            cx,
                        )
                        .into_any()
                    },
                    move |window, cx| {
                        context_picker.update(cx, |this, cx| {
                            this.add_recent_file(project_path.clone(), window, cx);
                        })
                    },
                )
            }
            RecentEntry::Thread(thread) => {
                let context_store = self.context_store.clone();
                let view_thread = thread.clone();

                ContextMenuItem::custom_entry(
                    move |_window, cx| {
                        render_thread_context_entry(&view_thread, context_store.clone(), cx)
                            .into_any()
                    },
                    move |_window, cx| {
                        context_picker.update(cx, |this, cx| {
                            this.add_recent_thread(thread.clone(), cx)
                                .detach_and_log_err(cx);
                        })
                    },
                )
            }
        }
    }

    fn add_recent_file(
        &self,
        project_path: ProjectPath,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(context_store) = self.context_store.upgrade() else {
            return;
        };

        let task = context_store.update(cx, |context_store, cx| {
            context_store.add_file_from_path(project_path.clone(), cx)
        });

        cx.spawn_in(window, |_, mut cx| async move {
            task.await.notify_async_err(&mut cx)
        })
        .detach();

        cx.notify();
    }

    fn add_recent_thread(
        &self,
        thread: ThreadContextEntry,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(context_store) = self.context_store.upgrade() else {
            return Task::ready(Err(anyhow!("context store not available")));
        };

        let Some(thread_store) = self
            .thread_store
            .as_ref()
            .and_then(|thread_store| thread_store.upgrade())
        else {
            return Task::ready(Err(anyhow!("thread store not available")));
        };

        let open_thread_task = thread_store.update(cx, |this, cx| this.open_thread(&thread.id, cx));
        cx.spawn(|this, mut cx| async move {
            let thread = open_thread_task.await?;
            context_store.update(&mut cx, |context_store, cx| {
                context_store.add_thread(thread, cx);
            })?;

            this.update(&mut cx, |_this, cx| cx.notify())
        })
    }

    fn recent_entries(&self, cx: &mut App) -> Vec<RecentEntry> {
        let Some(workspace) = self.workspace.upgrade().map(|w| w.read(cx)) else {
            return vec![];
        };

        let Some(context_store) = self.context_store.upgrade().map(|cs| cs.read(cx)) else {
            return vec![];
        };

        let mut recent = Vec::with_capacity(6);

        let mut current_files = context_store.file_paths(cx);

        if let Some(active_path) = Self::active_singleton_buffer_path(&workspace, cx) {
            current_files.insert(active_path);
        }

        let project = workspace.project().read(cx);

        recent.extend(
            workspace
                .recent_navigation_history_iter(cx)
                .filter(|(path, _)| !current_files.contains(&path.path.to_path_buf()))
                .take(4)
                .filter_map(|(project_path, _)| {
                    project
                        .worktree_for_id(project_path.worktree_id, cx)
                        .map(|worktree| RecentEntry::File {
                            project_path,
                            path_prefix: worktree.read(cx).root_name().into(),
                        })
                }),
        );

        let mut current_threads = context_store.thread_ids();

        if let Some(active_thread) = workspace
            .panel::<AssistantPanel>(cx)
            .map(|panel| panel.read(cx).active_thread(cx))
        {
            current_threads.insert(active_thread.read(cx).id().clone());
        }

        let Some(thread_store) = self
            .thread_store
            .as_ref()
            .and_then(|thread_store| thread_store.upgrade())
        else {
            return recent;
        };

        thread_store.update(cx, |thread_store, _cx| {
            recent.extend(
                thread_store
                    .threads()
                    .into_iter()
                    .filter(|thread| !current_threads.contains(&thread.id))
                    .take(2)
                    .map(|thread| {
                        RecentEntry::Thread(ThreadContextEntry {
                            id: thread.id,
                            summary: thread.summary,
                        })
                    }),
            )
        });

        recent
    }

    fn active_singleton_buffer_path(workspace: &Workspace, cx: &App) -> Option<PathBuf> {
        let active_item = workspace.active_item(cx)?;

        let editor = active_item.to_any().downcast::<Editor>().ok()?.read(cx);
        let buffer = editor.buffer().read(cx).as_singleton()?;

        let path = buffer.read(cx).file()?.path().to_path_buf();
        Some(path)
    }
}

impl EventEmitter<DismissEvent> for ContextPicker {}

impl Focusable for ContextPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.mode {
            ContextPickerMode::Default(menu) => menu.focus_handle(cx),
            ContextPickerMode::File(file_picker) => file_picker.focus_handle(cx),
            ContextPickerMode::Directory(directory_picker) => directory_picker.focus_handle(cx),
            ContextPickerMode::Fetch(fetch_picker) => fetch_picker.focus_handle(cx),
            ContextPickerMode::Thread(thread_picker) => thread_picker.focus_handle(cx),
        }
    }
}

impl Render for ContextPicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(px(400.))
            .min_w(px(400.))
            .map(|parent| match &self.mode {
                ContextPickerMode::Default(menu) => parent.child(menu.clone()),
                ContextPickerMode::File(file_picker) => parent.child(file_picker.clone()),
                ContextPickerMode::Directory(directory_picker) => {
                    parent.child(directory_picker.clone())
                }
                ContextPickerMode::Fetch(fetch_picker) => parent.child(fetch_picker.clone()),
                ContextPickerMode::Thread(thread_picker) => parent.child(thread_picker.clone()),
            })
    }
}
enum RecentEntry {
    File {
        project_path: ProjectPath,
        path_prefix: Arc<str>,
    },
    Thread(ThreadContextEntry),
}
