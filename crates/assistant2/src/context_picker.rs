mod completion_provider;
mod fetch_context_picker;
mod file_context_picker;
mod thread_context_picker;

use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use editor::display_map::{Crease, FoldId};
use editor::{Anchor, AnchorRangeExt as _, Editor, ExcerptId, FoldPlaceholder, ToOffset};
use file_context_picker::render_file_context_entry;
use gpui::{
    App, DismissEvent, Empty, Entity, EventEmitter, FocusHandle, Focusable, Task, WeakEntity,
};
use multi_buffer::MultiBufferRow;
use project::ProjectPath;
use thread_context_picker::{render_thread_context_entry, ThreadContextEntry};
use ui::{
    prelude::*, ButtonLike, ContextMenu, ContextMenuEntry, ContextMenuItem, Disclosure, TintColor,
};
use workspace::{notifications::NotifyResultExt, Workspace};

pub use crate::context_picker::completion_provider::ContextPickerCompletionProvider;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContextPickerMode {
    File,
    Fetch,
    Thread,
}

impl TryFrom<&str> for ContextPickerMode {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "file" => Ok(Self::File),
            "fetch" => Ok(Self::Fetch),
            "thread" => Ok(Self::Thread),
            _ => Err(format!("Invalid context picker mode: {}", value)),
        }
    }
}

impl ContextPickerMode {
    pub fn mention_prefix(&self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Fetch => "fetch",
            Self::Thread => "thread",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::File => "Files & Directories",
            Self::Fetch => "Fetch",
            Self::Thread => "Thread",
        }
    }

    pub fn icon(&self) -> IconName {
        match self {
            Self::File => IconName::File,
            Self::Fetch => IconName::Globe,
            Self::Thread => IconName::MessageCircle,
        }
    }
}

#[derive(Debug, Clone)]
enum ContextPickerState {
    Default(Entity<ContextMenu>),
    File(Entity<FileContextPicker>),
    Fetch(Entity<FetchContextPicker>),
    Thread(Entity<ThreadContextPicker>),
}

pub(super) struct ContextPicker {
    mode: ContextPickerState,
    workspace: WeakEntity<Workspace>,
    context_store: WeakEntity<ContextStore>,
    thread_store: Option<WeakEntity<ThreadStore>>,
    confirm_behavior: ConfirmBehavior,
}

impl ContextPicker {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        thread_store: Option<WeakEntity<ThreadStore>>,
        context_store: WeakEntity<ContextStore>,
        confirm_behavior: ConfirmBehavior,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        ContextPicker {
            mode: ContextPickerState::Default(ContextMenu::build(
                window,
                cx,
                |menu, _window, _cx| menu,
            )),
            workspace,
            context_store,
            thread_store,
            confirm_behavior,
        }
    }

    pub fn init(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.mode = ContextPickerState::Default(self.build_menu(window, cx));
        cx.notify();
    }

    fn build_menu(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Entity<ContextMenu> {
        let context_picker = cx.entity().clone();

        let menu = ContextMenu::build(window, cx, move |menu, _window, cx| {
            let recent = self.recent_entries(cx);
            let has_recent = !recent.is_empty();
            let recent_entries = recent
                .into_iter()
                .enumerate()
                .map(|(ix, entry)| self.recent_menu_item(context_picker.clone(), ix, entry));

            let modes = supported_context_picker_modes(&self.thread_store);

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
                .extend(modes.into_iter().map(|mode| {
                    let context_picker = context_picker.clone();

                    ContextMenuEntry::new(mode.label())
                        .icon(mode.icon())
                        .icon_size(IconSize::XSmall)
                        .icon_color(Color::Muted)
                        .handler(move |window, cx| {
                            context_picker.update(cx, |this, cx| this.select_mode(mode, window, cx))
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

    fn select_mode(
        &mut self,
        mode: ContextPickerMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let context_picker = cx.entity().downgrade();

        match mode {
            ContextPickerMode::File => {
                self.mode = ContextPickerState::File(cx.new(|cx| {
                    FileContextPicker::new(
                        context_picker.clone(),
                        self.workspace.clone(),
                        self.context_store.clone(),
                        self.confirm_behavior,
                        window,
                        cx,
                    )
                }));
            }
            ContextPickerMode::Fetch => {
                self.mode = ContextPickerState::Fetch(cx.new(|cx| {
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
            ContextPickerMode::Thread => {
                if let Some(thread_store) = self.thread_store.as_ref() {
                    self.mode = ContextPickerState::Thread(cx.new(|cx| {
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
                            false,
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
            context_store.add_file_from_path(project_path.clone(), true, cx)
        });

        cx.spawn_in(window, async move |_, cx| task.await.notify_async_err(cx))
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
        cx.spawn(async move |this, cx| {
            let thread = open_thread_task.await?;
            context_store.update(cx, |context_store, cx| {
                context_store.add_thread(thread, true, cx);
            })?;

            this.update(cx, |_this, cx| cx.notify())
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

        if let Some(active_path) = active_singleton_buffer_path(&workspace, cx) {
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
}

impl EventEmitter<DismissEvent> for ContextPicker {}

impl Focusable for ContextPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.mode {
            ContextPickerState::Default(menu) => menu.focus_handle(cx),
            ContextPickerState::File(file_picker) => file_picker.focus_handle(cx),
            ContextPickerState::Fetch(fetch_picker) => fetch_picker.focus_handle(cx),
            ContextPickerState::Thread(thread_picker) => thread_picker.focus_handle(cx),
        }
    }
}

impl Render for ContextPicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(px(400.))
            .min_w(px(400.))
            .map(|parent| match &self.mode {
                ContextPickerState::Default(menu) => parent.child(menu.clone()),
                ContextPickerState::File(file_picker) => parent.child(file_picker.clone()),
                ContextPickerState::Fetch(fetch_picker) => parent.child(fetch_picker.clone()),
                ContextPickerState::Thread(thread_picker) => parent.child(thread_picker.clone()),
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

fn supported_context_picker_modes(
    thread_store: &Option<WeakEntity<ThreadStore>>,
) -> Vec<ContextPickerMode> {
    let mut modes = vec![ContextPickerMode::File, ContextPickerMode::Fetch];
    if thread_store.is_some() {
        modes.push(ContextPickerMode::Thread);
    }
    modes
}

fn active_singleton_buffer_path(workspace: &Workspace, cx: &App) -> Option<PathBuf> {
    let active_item = workspace.active_item(cx)?;

    let editor = active_item.to_any().downcast::<Editor>().ok()?.read(cx);
    let buffer = editor.buffer().read(cx).as_singleton()?;

    let path = buffer.read(cx).file()?.path().to_path_buf();
    Some(path)
}

fn recent_context_picker_entries(
    context_store: Entity<ContextStore>,
    thread_store: Option<WeakEntity<ThreadStore>>,
    workspace: Entity<Workspace>,
    cx: &App,
) -> Vec<RecentEntry> {
    let mut recent = Vec::with_capacity(6);

    let mut current_files = context_store.read(cx).file_paths(cx);

    let workspace = workspace.read(cx);

    if let Some(active_path) = active_singleton_buffer_path(workspace, cx) {
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

    let mut current_threads = context_store.read(cx).thread_ids();

    if let Some(active_thread) = workspace
        .panel::<AssistantPanel>(cx)
        .map(|panel| panel.read(cx).active_thread(cx))
    {
        current_threads.insert(active_thread.read(cx).id().clone());
    }

    if let Some(thread_store) = thread_store.and_then(|thread_store| thread_store.upgrade()) {
        recent.extend(
            thread_store
                .read(cx)
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
        );
    }

    recent
}

pub(crate) fn insert_crease_for_mention(
    excerpt_id: ExcerptId,
    crease_start: text::Anchor,
    content_len: usize,
    crease_label: SharedString,
    crease_icon_path: SharedString,
    editor_entity: Entity<Editor>,
    window: &mut Window,
    cx: &mut App,
) {
    editor_entity.update(cx, |editor, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);

        let Some(start) = snapshot.anchor_in_excerpt(excerpt_id, crease_start) else {
            return;
        };

        let end = snapshot.anchor_before(start.to_offset(&snapshot) + content_len);

        let placeholder = FoldPlaceholder {
            render: render_fold_icon_button(
                crease_icon_path,
                crease_label,
                editor_entity.downgrade(),
            ),
            ..Default::default()
        };

        let render_trailer =
            move |_row, _unfold, _window: &mut Window, _cx: &mut App| Empty.into_any();

        let crease = Crease::inline(
            start..end,
            placeholder.clone(),
            fold_toggle("mention"),
            render_trailer,
        );

        editor.insert_creases(vec![crease.clone()], cx);
        editor.fold_creases(vec![crease], false, window, cx);
    });
}

fn render_fold_icon_button(
    icon_path: SharedString,
    label: SharedString,
    editor: WeakEntity<Editor>,
) -> Arc<dyn Send + Sync + Fn(FoldId, Range<Anchor>, &mut App) -> AnyElement> {
    Arc::new({
        move |fold_id, fold_range, cx| {
            let is_in_text_selection = editor.upgrade().is_some_and(|editor| {
                editor.update(cx, |editor, cx| {
                    let snapshot = editor
                        .buffer()
                        .update(cx, |multi_buffer, cx| multi_buffer.snapshot(cx));

                    let is_in_pending_selection = || {
                        editor
                            .selections
                            .pending
                            .as_ref()
                            .is_some_and(|pending_selection| {
                                pending_selection
                                    .selection
                                    .range()
                                    .includes(&fold_range, &snapshot)
                            })
                    };

                    let mut is_in_complete_selection = || {
                        editor
                            .selections
                            .disjoint_in_range::<usize>(fold_range.clone(), cx)
                            .into_iter()
                            .any(|selection| {
                                // This is needed to cover a corner case, if we just check for an existing
                                // selection in the fold range, having a cursor at the start of the fold
                                // marks it as selected. Non-empty selections don't cause this.
                                let length = selection.end - selection.start;
                                length > 0
                            })
                    };

                    is_in_pending_selection() || is_in_complete_selection()
                })
            });

            ButtonLike::new(fold_id)
                .style(ButtonStyle::Filled)
                .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                .toggle_state(is_in_text_selection)
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            Icon::from_path(icon_path.clone())
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        )
                        .child(
                            Label::new(label.clone())
                                .size(LabelSize::Small)
                                .single_line(),
                        ),
                )
                .into_any_element()
        }
    })
}

fn fold_toggle(
    name: &'static str,
) -> impl Fn(
    MultiBufferRow,
    bool,
    Arc<dyn Fn(bool, &mut Window, &mut App) + Send + Sync>,
    &mut Window,
    &mut App,
) -> AnyElement {
    move |row, is_folded, fold, _window, _cx| {
        Disclosure::new((name, row.0 as u64), !is_folded)
            .toggle_state(is_folded)
            .on_click(move |_e, window, cx| fold(!is_folded, window, cx))
            .into_any_element()
    }
}
