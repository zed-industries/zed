mod directory_context_picker;
mod fetch_context_picker;
mod file_context_picker;
mod thread_context_picker;

use std::path::PathBuf;
use std::sync::Arc;

use editor::Editor;
use file_context_picker::render_file_context_entry;
use gpui::{
    AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, View, WeakModel, WeakView,
};
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
    Default(View<ContextMenu>),
    File(View<FileContextPicker>),
    Directory(View<DirectoryContextPicker>),
    Fetch(View<FetchContextPicker>),
    Thread(View<ThreadContextPicker>),
}

pub(super) struct ContextPicker {
    mode: ContextPickerMode,
    workspace: WeakView<Workspace>,
    editor: WeakView<Editor>,
    context_store: WeakModel<ContextStore>,
    thread_store: Option<WeakModel<ThreadStore>>,
    confirm_behavior: ConfirmBehavior,
}

impl ContextPicker {
    pub fn new(
        workspace: WeakView<Workspace>,
        thread_store: Option<WeakModel<ThreadStore>>,
        context_store: WeakModel<ContextStore>,
        editor: WeakView<Editor>,
        confirm_behavior: ConfirmBehavior,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        ContextPicker {
            mode: ContextPickerMode::Default(ContextMenu::build(cx, |menu, _cx| menu)),
            workspace,
            context_store,
            thread_store,
            editor,
            confirm_behavior,
        }
    }

    pub fn init(&mut self, cx: &mut ViewContext<Self>) {
        self.mode = ContextPickerMode::Default(self.build_menu(cx));
        cx.notify();
    }

    fn build_menu(&mut self, cx: &mut ViewContext<Self>) -> View<ContextMenu> {
        let context_picker = cx.view().clone();

        let menu = ContextMenu::build(cx, move |menu, cx| {
            let kind_entry = |kind: &'static ContextKind| {
                let context_picker = context_picker.clone();

                ContextMenuEntry::new(kind.label())
                    .icon(kind.icon())
                    .handler(move |cx| {
                        context_picker.update(cx, |this, cx| this.select_kind(*kind, cx))
                    })
            };

            let recent = self.recent_entries(cx);
            let has_recent = !recent.is_empty();
            let recent_entries = recent
                .into_iter()
                .enumerate()
                .map(|(ix, entry)| self.recent_menu_item(context_picker.clone(), ix, entry));

            let menu = menu
                .when(has_recent, |menu| {
                    menu.custom_row(|_| {
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
                .extend(ContextKind::all().into_iter().map(kind_entry));

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

    fn select_kind(&mut self, kind: ContextKind, cx: &mut ViewContext<Self>) {
        let context_picker = cx.view().downgrade();

        match kind {
            ContextKind::File => {
                self.mode = ContextPickerMode::File(cx.new_view(|cx| {
                    FileContextPicker::new(
                        context_picker.clone(),
                        self.workspace.clone(),
                        self.editor.clone(),
                        self.context_store.clone(),
                        self.confirm_behavior,
                        cx,
                    )
                }));
            }
            ContextKind::Directory => {
                self.mode = ContextPickerMode::Directory(cx.new_view(|cx| {
                    DirectoryContextPicker::new(
                        context_picker.clone(),
                        self.workspace.clone(),
                        self.context_store.clone(),
                        self.confirm_behavior,
                        cx,
                    )
                }));
            }
            ContextKind::FetchedUrl => {
                self.mode = ContextPickerMode::Fetch(cx.new_view(|cx| {
                    FetchContextPicker::new(
                        context_picker.clone(),
                        self.workspace.clone(),
                        self.context_store.clone(),
                        self.confirm_behavior,
                        cx,
                    )
                }));
            }
            ContextKind::Thread => {
                if let Some(thread_store) = self.thread_store.as_ref() {
                    self.mode = ContextPickerMode::Thread(cx.new_view(|cx| {
                        ThreadContextPicker::new(
                            thread_store.clone(),
                            context_picker.clone(),
                            self.context_store.clone(),
                            self.confirm_behavior,
                            cx,
                        )
                    }));
                }
            }
        }

        cx.notify();
        cx.focus_self();
    }

    fn recent_menu_item(
        &self,
        context_picker: View<ContextPicker>,
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
                    move |cx| {
                        render_file_context_entry(
                            ElementId::NamedInteger("ctx-recent".into(), ix),
                            &path,
                            &path_prefix,
                            context_store.clone(),
                            cx,
                        )
                        .into_any()
                    },
                    move |cx| {
                        context_picker.update(cx, |this, cx| {
                            this.add_recent_file(project_path.clone(), cx);
                        })
                    },
                )
            }
            RecentEntry::Thread(thread) => {
                let context_store = self.context_store.clone();
                let view_thread = thread.clone();

                ContextMenuItem::custom_entry(
                    move |cx| {
                        render_thread_context_entry(&view_thread, context_store.clone(), cx)
                            .into_any()
                    },
                    move |cx| {
                        context_picker.update(cx, |this, cx| {
                            this.add_recent_thread(thread.clone(), cx);
                        })
                    },
                )
            }
        }
    }

    fn add_recent_file(&self, project_path: ProjectPath, cx: &mut ViewContext<Self>) {
        let Some(context_store) = self.context_store.upgrade() else {
            return;
        };

        let task = context_store.update(cx, |context_store, cx| {
            context_store.add_file_from_path(project_path.clone(), cx)
        });

        cx.spawn(|_, mut cx| async move { task.await.notify_async_err(&mut cx) })
            .detach();

        cx.notify();
    }

    fn add_recent_thread(&self, thread: ThreadContextEntry, cx: &mut ViewContext<Self>) {
        let Some(context_store) = self.context_store.upgrade() else {
            return;
        };

        let Some(thread) = self
            .thread_store
            .clone()
            .and_then(|this| this.upgrade())
            .and_then(|this| this.update(cx, |this, cx| this.open_thread(&thread.id, cx)))
        else {
            return;
        };

        context_store.update(cx, |context_store, cx| {
            context_store.add_thread(thread, cx);
        });

        cx.notify();
    }

    fn recent_entries(&self, cx: &mut WindowContext) -> Vec<RecentEntry> {
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

        thread_store.update(cx, |thread_store, cx| {
            recent.extend(
                thread_store
                    .threads(cx)
                    .into_iter()
                    .filter(|thread| !current_threads.contains(thread.read(cx).id()))
                    .take(2)
                    .map(|thread| {
                        let thread = thread.read(cx);

                        RecentEntry::Thread(ThreadContextEntry {
                            id: thread.id().clone(),
                            summary: thread.summary_or_default(),
                        })
                    }),
            )
        });

        recent
    }

    fn active_singleton_buffer_path(workspace: &Workspace, cx: &AppContext) -> Option<PathBuf> {
        let active_item = workspace.active_item(cx)?;

        let editor = active_item.to_any().downcast::<Editor>().ok()?.read(cx);
        let buffer = editor.buffer().read(cx).as_singleton()?;

        let path = buffer.read(cx).file()?.path().to_path_buf();
        Some(path)
    }
}

impl EventEmitter<DismissEvent> for ContextPicker {}

impl FocusableView for ContextPicker {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
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
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
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
