mod completion_provider;
mod fetch_context_picker;
mod file_context_picker;
mod rules_context_picker;
mod symbol_context_picker;
mod thread_context_picker;

use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Result, anyhow};
pub use completion_provider::ContextPickerCompletionProvider;
use editor::display_map::{Crease, CreaseId, CreaseMetadata, FoldId};
use editor::{Anchor, AnchorRangeExt as _, Editor, ExcerptId, FoldPlaceholder, ToOffset};
use fetch_context_picker::FetchContextPicker;
use file_context_picker::FileContextPicker;
use file_context_picker::render_file_context_entry;
use gpui::{
    App, DismissEvent, Empty, Entity, EventEmitter, FocusHandle, Focusable, Subscription, Task,
    WeakEntity,
};
use language::Buffer;
use multi_buffer::MultiBufferRow;
use project::{Entry, ProjectPath};
use prompt_store::{PromptStore, UserPromptId};
use rules_context_picker::{RulesContextEntry, RulesContextPicker};
use symbol_context_picker::SymbolContextPicker;
use thread_context_picker::{ThreadContextEntry, ThreadContextPicker, render_thread_context_entry};
use ui::{
    ButtonLike, ContextMenu, ContextMenuEntry, ContextMenuItem, Disclosure, TintColor, prelude::*,
};
use uuid::Uuid;
use workspace::{Workspace, notifications::NotifyResultExt};

use crate::AssistantPanel;
use crate::context::RULES_ICON;
use crate::context_store::ContextStore;
use crate::thread::ThreadId;
use crate::thread_store::ThreadStore;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContextPickerEntry {
    Mode(ContextPickerMode),
    Action(ContextPickerAction),
}

impl ContextPickerEntry {
    pub fn keyword(&self) -> &'static str {
        match self {
            Self::Mode(mode) => mode.keyword(),
            Self::Action(action) => action.keyword(),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Mode(mode) => mode.label(),
            Self::Action(action) => action.label(),
        }
    }

    pub fn icon(&self) -> IconName {
        match self {
            Self::Mode(mode) => mode.icon(),
            Self::Action(action) => action.icon(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContextPickerMode {
    File,
    Symbol,
    Fetch,
    Thread,
    Rules,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContextPickerAction {
    AddSelections,
}

impl ContextPickerAction {
    pub fn keyword(&self) -> &'static str {
        match self {
            Self::AddSelections => "selection",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::AddSelections => "Selection",
        }
    }

    pub fn icon(&self) -> IconName {
        match self {
            Self::AddSelections => IconName::Context,
        }
    }
}

impl TryFrom<&str> for ContextPickerMode {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "file" => Ok(Self::File),
            "symbol" => Ok(Self::Symbol),
            "fetch" => Ok(Self::Fetch),
            "thread" => Ok(Self::Thread),
            "rules" => Ok(Self::Rules),
            _ => Err(format!("Invalid context picker mode: {}", value)),
        }
    }
}

impl ContextPickerMode {
    pub fn keyword(&self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Symbol => "symbol",
            Self::Fetch => "fetch",
            Self::Thread => "thread",
            Self::Rules => "rules",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::File => "Files & Directories",
            Self::Symbol => "Symbols",
            Self::Fetch => "Fetch",
            Self::Thread => "Threads",
            Self::Rules => "Rules",
        }
    }

    pub fn icon(&self) -> IconName {
        match self {
            Self::File => IconName::File,
            Self::Symbol => IconName::Code,
            Self::Fetch => IconName::Globe,
            Self::Thread => IconName::MessageBubbles,
            Self::Rules => RULES_ICON,
        }
    }
}

#[derive(Debug, Clone)]
enum ContextPickerState {
    Default(Entity<ContextMenu>),
    File(Entity<FileContextPicker>),
    Symbol(Entity<SymbolContextPicker>),
    Fetch(Entity<FetchContextPicker>),
    Thread(Entity<ThreadContextPicker>),
    Rules(Entity<RulesContextPicker>),
}

pub(super) struct ContextPicker {
    mode: ContextPickerState,
    workspace: WeakEntity<Workspace>,
    context_store: WeakEntity<ContextStore>,
    thread_store: Option<WeakEntity<ThreadStore>>,
    prompt_store: Option<Entity<PromptStore>>,
    _subscriptions: Vec<Subscription>,
}

impl ContextPicker {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        thread_store: Option<WeakEntity<ThreadStore>>,
        context_store: WeakEntity<ContextStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions = context_store
            .upgrade()
            .map(|context_store| {
                cx.observe(&context_store, |this, _, cx| this.notify_current_picker(cx))
            })
            .into_iter()
            .chain(
                thread_store
                    .as_ref()
                    .and_then(|thread_store| thread_store.upgrade())
                    .map(|thread_store| {
                        cx.observe(&thread_store, |this, _, cx| this.notify_current_picker(cx))
                    }),
            )
            .collect::<Vec<Subscription>>();

        let prompt_store = thread_store.as_ref().and_then(|thread_store| {
            thread_store
                .read_with(cx, |thread_store, _cx| thread_store.prompt_store().clone())
                .ok()
                .flatten()
        });

        ContextPicker {
            mode: ContextPickerState::Default(ContextMenu::build(
                window,
                cx,
                |menu, _window, _cx| menu,
            )),
            workspace,
            context_store,
            thread_store,
            prompt_store,
            _subscriptions: subscriptions,
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

            let entries = self
                .workspace
                .upgrade()
                .map(|workspace| {
                    available_context_picker_entries(
                        &self.prompt_store,
                        &self.thread_store,
                        &workspace,
                        cx,
                    )
                })
                .unwrap_or_default();

            menu.when(has_recent, |menu| {
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
            .extend(entries.into_iter().map(|entry| {
                let context_picker = context_picker.clone();

                ContextMenuEntry::new(entry.label())
                    .icon(entry.icon())
                    .icon_size(IconSize::XSmall)
                    .icon_color(Color::Muted)
                    .handler(move |window, cx| {
                        context_picker.update(cx, |this, cx| this.select_entry(entry, window, cx))
                    })
            }))
            .keep_open_on_confirm(true)
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

    fn select_entry(
        &mut self,
        entry: ContextPickerEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let context_picker = cx.entity().downgrade();

        match entry {
            ContextPickerEntry::Mode(mode) => match mode {
                ContextPickerMode::File => {
                    self.mode = ContextPickerState::File(cx.new(|cx| {
                        FileContextPicker::new(
                            context_picker.clone(),
                            self.workspace.clone(),
                            self.context_store.clone(),
                            window,
                            cx,
                        )
                    }));
                }
                ContextPickerMode::Symbol => {
                    self.mode = ContextPickerState::Symbol(cx.new(|cx| {
                        SymbolContextPicker::new(
                            context_picker.clone(),
                            self.workspace.clone(),
                            self.context_store.clone(),
                            window,
                            cx,
                        )
                    }));
                }
                ContextPickerMode::Rules => {
                    if let Some(prompt_store) = self.prompt_store.as_ref() {
                        self.mode = ContextPickerState::Rules(cx.new(|cx| {
                            RulesContextPicker::new(
                                prompt_store.clone(),
                                context_picker.clone(),
                                self.context_store.clone(),
                                window,
                                cx,
                            )
                        }));
                    }
                }
                ContextPickerMode::Fetch => {
                    self.mode = ContextPickerState::Fetch(cx.new(|cx| {
                        FetchContextPicker::new(
                            context_picker.clone(),
                            self.workspace.clone(),
                            self.context_store.clone(),
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
                                window,
                                cx,
                            )
                        }));
                    }
                }
            },
            ContextPickerEntry::Action(action) => match action {
                ContextPickerAction::AddSelections => {
                    if let Some((context_store, workspace)) =
                        self.context_store.upgrade().zip(self.workspace.upgrade())
                    {
                        add_selections_as_context(&context_store, &workspace, cx);
                    }

                    cx.emit(DismissEvent);
                }
            },
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
                let worktree_id = project_path.worktree_id;
                let path = project_path.path.clone();

                ContextMenuItem::custom_entry(
                    move |_window, cx| {
                        render_file_context_entry(
                            ElementId::named_usize("ctx-recent", ix),
                            worktree_id,
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
        let Some(workspace) = self.workspace.upgrade() else {
            return vec![];
        };

        let Some(context_store) = self.context_store.upgrade() else {
            return vec![];
        };

        recent_context_picker_entries(
            context_store,
            self.thread_store.clone(),
            workspace,
            None,
            cx,
        )
    }

    fn notify_current_picker(&mut self, cx: &mut Context<Self>) {
        match &self.mode {
            ContextPickerState::Default(entity) => entity.update(cx, |_, cx| cx.notify()),
            ContextPickerState::File(entity) => entity.update(cx, |_, cx| cx.notify()),
            ContextPickerState::Symbol(entity) => entity.update(cx, |_, cx| cx.notify()),
            ContextPickerState::Fetch(entity) => entity.update(cx, |_, cx| cx.notify()),
            ContextPickerState::Thread(entity) => entity.update(cx, |_, cx| cx.notify()),
            ContextPickerState::Rules(entity) => entity.update(cx, |_, cx| cx.notify()),
        }
    }
}

impl EventEmitter<DismissEvent> for ContextPicker {}

impl Focusable for ContextPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.mode {
            ContextPickerState::Default(menu) => menu.focus_handle(cx),
            ContextPickerState::File(file_picker) => file_picker.focus_handle(cx),
            ContextPickerState::Symbol(symbol_picker) => symbol_picker.focus_handle(cx),
            ContextPickerState::Fetch(fetch_picker) => fetch_picker.focus_handle(cx),
            ContextPickerState::Thread(thread_picker) => thread_picker.focus_handle(cx),
            ContextPickerState::Rules(user_rules_picker) => user_rules_picker.focus_handle(cx),
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
                ContextPickerState::Symbol(symbol_picker) => parent.child(symbol_picker.clone()),
                ContextPickerState::Fetch(fetch_picker) => parent.child(fetch_picker.clone()),
                ContextPickerState::Thread(thread_picker) => parent.child(thread_picker.clone()),
                ContextPickerState::Rules(user_rules_picker) => {
                    parent.child(user_rules_picker.clone())
                }
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

fn available_context_picker_entries(
    prompt_store: &Option<Entity<PromptStore>>,
    thread_store: &Option<WeakEntity<ThreadStore>>,
    workspace: &Entity<Workspace>,
    cx: &mut App,
) -> Vec<ContextPickerEntry> {
    let mut entries = vec![
        ContextPickerEntry::Mode(ContextPickerMode::File),
        ContextPickerEntry::Mode(ContextPickerMode::Symbol),
    ];

    let has_selection = workspace
        .read(cx)
        .active_item(cx)
        .and_then(|item| item.downcast::<Editor>())
        .map_or(false, |editor| {
            editor.update(cx, |editor, cx| editor.has_non_empty_selection(cx))
        });
    if has_selection {
        entries.push(ContextPickerEntry::Action(
            ContextPickerAction::AddSelections,
        ));
    }

    if thread_store.is_some() {
        entries.push(ContextPickerEntry::Mode(ContextPickerMode::Thread));
    }

    if prompt_store.is_some() {
        entries.push(ContextPickerEntry::Mode(ContextPickerMode::Rules));
    }

    entries.push(ContextPickerEntry::Mode(ContextPickerMode::Fetch));

    entries
}

fn recent_context_picker_entries(
    context_store: Entity<ContextStore>,
    thread_store: Option<WeakEntity<ThreadStore>>,
    workspace: Entity<Workspace>,
    exclude_path: Option<ProjectPath>,
    cx: &App,
) -> Vec<RecentEntry> {
    let mut recent = Vec::with_capacity(6);
    let mut current_files = context_store.read(cx).file_paths(cx);
    current_files.extend(exclude_path);
    let workspace = workspace.read(cx);
    let project = workspace.project().read(cx);

    recent.extend(
        workspace
            .recent_navigation_history_iter(cx)
            .filter(|(path, _)| !current_files.contains(path))
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

    let current_threads = context_store.read(cx).thread_ids();

    let active_thread_id = workspace
        .panel::<AssistantPanel>(cx)
        .map(|panel| panel.read(cx).active_thread(cx).read(cx).id());

    if let Some(thread_store) = thread_store.and_then(|thread_store| thread_store.upgrade()) {
        recent.extend(
            thread_store
                .read(cx)
                .reverse_chronological_threads()
                .into_iter()
                .filter(|thread| {
                    Some(&thread.id) != active_thread_id && !current_threads.contains(&thread.id)
                })
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

fn add_selections_as_context(
    context_store: &Entity<ContextStore>,
    workspace: &Entity<Workspace>,
    cx: &mut App,
) {
    let selection_ranges = selection_ranges(workspace, cx);
    context_store.update(cx, |context_store, cx| {
        for (buffer, range) in selection_ranges {
            context_store.add_selection(buffer, range, cx);
        }
    })
}

fn selection_ranges(
    workspace: &Entity<Workspace>,
    cx: &mut App,
) -> Vec<(Entity<Buffer>, Range<text::Anchor>)> {
    let Some(editor) = workspace
        .read(cx)
        .active_item(cx)
        .and_then(|item| item.act_as::<Editor>(cx))
    else {
        return Vec::new();
    };

    editor.update(cx, |editor, cx| {
        let selections = editor.selections.all_adjusted(cx);

        let buffer = editor.buffer().clone().read(cx);
        let snapshot = buffer.snapshot(cx);

        selections
            .into_iter()
            .map(|s| snapshot.anchor_after(s.start)..snapshot.anchor_before(s.end))
            .flat_map(|range| {
                let (start_buffer, start) = buffer.text_anchor_for_position(range.start, cx)?;
                let (end_buffer, end) = buffer.text_anchor_for_position(range.end, cx)?;
                if start_buffer != end_buffer {
                    return None;
                }
                Some((start_buffer, start..end))
            })
            .collect::<Vec<_>>()
    })
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
) -> Option<CreaseId> {
    editor_entity.update(cx, |editor, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);

        let start = snapshot.anchor_in_excerpt(excerpt_id, crease_start)?;

        let start = start.bias_right(&snapshot);
        let end = snapshot.anchor_before(start.to_offset(&snapshot) + content_len);

        let crease = crease_for_mention(
            crease_label,
            crease_icon_path,
            start..end,
            editor_entity.downgrade(),
        );

        let ids = editor.insert_creases(vec![crease.clone()], cx);
        editor.fold_creases(vec![crease], false, window, cx);
        Some(ids[0])
    })
}

pub fn crease_for_mention(
    label: SharedString,
    icon_path: SharedString,
    range: Range<Anchor>,
    editor_entity: WeakEntity<Editor>,
) -> Crease<Anchor> {
    let placeholder = FoldPlaceholder {
        render: render_fold_icon_button(icon_path.clone(), label.clone(), editor_entity),
        merge_adjacent: false,
        ..Default::default()
    };

    let render_trailer = move |_row, _unfold, _window: &mut Window, _cx: &mut App| Empty.into_any();

    Crease::inline(
        range,
        placeholder.clone(),
        fold_toggle("mention"),
        render_trailer,
    )
    .with_metadata(CreaseMetadata { icon_path, label })
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
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                        .child(
                            Label::new(label.clone())
                                .size(LabelSize::Small)
                                .buffer_font(cx)
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

pub enum MentionLink {
    File(ProjectPath, Entry),
    Symbol(ProjectPath, String),
    Selection(ProjectPath, Range<usize>),
    Fetch(String),
    Thread(ThreadId),
    Rules(UserPromptId),
}

impl MentionLink {
    const FILE: &str = "@file";
    const SYMBOL: &str = "@symbol";
    const SELECTION: &str = "@selection";
    const THREAD: &str = "@thread";
    const FETCH: &str = "@fetch";
    const RULES: &str = "@rules";

    const SEPARATOR: &str = ":";

    pub fn is_valid(url: &str) -> bool {
        url.starts_with(Self::FILE)
            || url.starts_with(Self::SYMBOL)
            || url.starts_with(Self::FETCH)
            || url.starts_with(Self::SELECTION)
            || url.starts_with(Self::THREAD)
            || url.starts_with(Self::RULES)
    }

    pub fn for_file(file_name: &str, full_path: &str) -> String {
        format!("[@{}]({}:{})", file_name, Self::FILE, full_path)
    }

    pub fn for_symbol(symbol_name: &str, full_path: &str) -> String {
        format!(
            "[@{}]({}:{}:{})",
            symbol_name,
            Self::SYMBOL,
            full_path,
            symbol_name
        )
    }

    pub fn for_selection(file_name: &str, full_path: &str, line_range: Range<usize>) -> String {
        format!(
            "[@{} ({}-{})]({}:{}:{}-{})",
            file_name,
            line_range.start,
            line_range.end,
            Self::SELECTION,
            full_path,
            line_range.start,
            line_range.end
        )
    }

    pub fn for_thread(thread: &ThreadContextEntry) -> String {
        format!("[@{}]({}:{})", thread.summary, Self::THREAD, thread.id)
    }

    pub fn for_fetch(url: &str) -> String {
        format!("[@{}]({}:{})", url, Self::FETCH, url)
    }

    pub fn for_rules(rules: &RulesContextEntry) -> String {
        format!("[@{}]({}:{})", rules.title, Self::RULES, rules.prompt_id.0)
    }

    pub fn try_parse(link: &str, workspace: &Entity<Workspace>, cx: &App) -> Option<Self> {
        fn extract_project_path_from_link(
            path: &str,
            workspace: &Entity<Workspace>,
            cx: &App,
        ) -> Option<ProjectPath> {
            let path = PathBuf::from(path);
            let worktree_name = path.iter().next()?;
            let path: PathBuf = path.iter().skip(1).collect();
            let worktree_id = workspace
                .read(cx)
                .visible_worktrees(cx)
                .find(|worktree| worktree.read(cx).root_name() == worktree_name)
                .map(|worktree| worktree.read(cx).id())?;
            Some(ProjectPath {
                worktree_id,
                path: path.into(),
            })
        }

        let (prefix, argument) = link.split_once(Self::SEPARATOR)?;
        match prefix {
            Self::FILE => {
                let project_path = extract_project_path_from_link(argument, workspace, cx)?;
                let entry = workspace
                    .read(cx)
                    .project()
                    .read(cx)
                    .entry_for_path(&project_path, cx)?;
                Some(MentionLink::File(project_path, entry))
            }
            Self::SYMBOL => {
                let (path, symbol) = argument.split_once(Self::SEPARATOR)?;
                let project_path = extract_project_path_from_link(path, workspace, cx)?;
                Some(MentionLink::Symbol(project_path, symbol.to_string()))
            }
            Self::SELECTION => {
                let (path, line_args) = argument.split_once(Self::SEPARATOR)?;
                let project_path = extract_project_path_from_link(path, workspace, cx)?;

                let line_range = {
                    let (start, end) = line_args
                        .trim_start_matches('(')
                        .trim_end_matches(')')
                        .split_once('-')?;
                    start.parse::<usize>().ok()?..end.parse::<usize>().ok()?
                };

                Some(MentionLink::Selection(project_path, line_range))
            }
            Self::THREAD => {
                let thread_id = ThreadId::from(argument);
                Some(MentionLink::Thread(thread_id))
            }
            Self::FETCH => Some(MentionLink::Fetch(argument.to_string())),
            Self::RULES => {
                let prompt_id = UserPromptId(Uuid::try_parse(argument).ok()?);
                Some(MentionLink::Rules(prompt_id))
            }
            _ => None,
        }
    }
}
