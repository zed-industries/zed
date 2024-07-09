mod outline_panel_settings;

use std::{
    cmp,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use anyhow::Context;
use collections::{hash_map, BTreeSet, HashMap, HashSet};
use db::kvp::KEY_VALUE_STORE;
use editor::{
    display_map::ToDisplayPoint,
    items::{entry_git_aware_label_color, entry_label_color},
    scroll::ScrollAnchor,
    DisplayPoint, Editor, EditorEvent, ExcerptId, ExcerptRange,
};
use file_icons::FileIcons;
use gpui::{
    actions, anchored, deferred, div, px, uniform_list, Action, AnyElement, AppContext,
    AssetSource, AsyncWindowContext, ClipboardItem, DismissEvent, Div, ElementId, EntityId,
    EventEmitter, FocusHandle, FocusableView, InteractiveElement, IntoElement, KeyContext, Model,
    MouseButton, MouseDownEvent, ParentElement, Pixels, Point, Render, SharedString, Stateful,
    Styled, Subscription, Task, UniformListScrollHandle, View, ViewContext, VisualContext,
    WeakView, WindowContext,
};
use itertools::Itertools;
use language::{BufferId, BufferSnapshot, OffsetRangeExt, OutlineItem};
use menu::{SelectFirst, SelectLast, SelectNext, SelectPrev};

use outline_panel_settings::{OutlinePanelDockPosition, OutlinePanelSettings};
use project::{File, Fs, Item, Project};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use util::{RangeExt, ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    item::ItemHandle,
    ui::{
        h_flex, v_flex, ActiveTheme, Color, ContextMenu, FluentBuilder, Icon, IconName, IconSize,
        Label, LabelCommon, ListItem, Selectable, Spacing, StyledTypography,
    },
    OpenInTerminal, Workspace,
};
use worktree::{Entry, ProjectEntryId, WorktreeId};

actions!(
    outline_panel,
    [
        ExpandSelectedEntry,
        CollapseSelectedEntry,
        CollapseAllEntries,
        CopyPath,
        CopyRelativePath,
        RevealInFileManager,
        Open,
        ToggleFocus,
        UnfoldDirectory,
        FoldDirectory,
        SelectParent,
    ]
);

const OUTLINE_PANEL_KEY: &str = "OutlinePanel";
const UPDATE_DEBOUNCE_MILLIS: u64 = 80;

type Outline = OutlineItem<language::Anchor>;

pub struct OutlinePanel {
    fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    project: Model<Project>,
    active: bool,
    scroll_handle: UniformListScrollHandle,
    context_menu: Option<(View<ContextMenu>, Point<Pixels>, Subscription)>,
    focus_handle: FocusHandle,
    pending_serialization: Task<Option<()>>,
    fs_entries_depth: HashMap<(WorktreeId, ProjectEntryId), usize>,
    fs_entries: Vec<FsEntry>,
    collapsed_entries: HashSet<CollapsedEntry>,
    unfolded_dirs: HashMap<WorktreeId, BTreeSet<ProjectEntryId>>,
    last_visible_range: Range<usize>,
    selected_entry: Option<EntryOwned>,
    active_item: Option<ActiveItem>,
    _subscriptions: Vec<Subscription>,
    loading_outlines: bool,
    update_task: Task<()>,
    outline_fetch_tasks: HashMap<(BufferId, ExcerptId), Task<()>>,
    excerpts: HashMap<BufferId, HashMap<ExcerptId, Excerpt>>,
    cached_entries_with_depth: Option<Vec<(usize, EntryOwned)>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum CollapsedEntry {
    Dir(WorktreeId, ProjectEntryId),
    File(WorktreeId, BufferId),
    ExternalFile(BufferId),
    Excerpt(BufferId, ExcerptId),
}

#[derive(Debug)]
struct Excerpt {
    range: ExcerptRange<language::Anchor>,
    outlines: ExcerptOutlines,
}

impl Excerpt {
    fn invalidate_outlines(&mut self) {
        if let ExcerptOutlines::Outlines(valid_outlines) = &mut self.outlines {
            self.outlines = ExcerptOutlines::Invalidated(std::mem::take(valid_outlines));
        }
    }

    fn iter_outlines(&self) -> impl Iterator<Item = &Outline> {
        match &self.outlines {
            ExcerptOutlines::Outlines(outlines) => outlines.iter(),
            ExcerptOutlines::Invalidated(outlines) => outlines.iter(),
            ExcerptOutlines::NotFetched => [].iter(),
        }
    }

    fn should_fetch_outlines(&self) -> bool {
        match &self.outlines {
            ExcerptOutlines::Outlines(_) => false,
            ExcerptOutlines::Invalidated(_) => true,
            ExcerptOutlines::NotFetched => true,
        }
    }
}

#[derive(Debug)]
enum ExcerptOutlines {
    Outlines(Vec<Outline>),
    Invalidated(Vec<Outline>),
    NotFetched,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum EntryOwned {
    Entry(FsEntry),
    FoldedDirs(WorktreeId, Vec<Entry>),
    Excerpt(BufferId, ExcerptId, ExcerptRange<language::Anchor>),
    Outline(BufferId, ExcerptId, Outline),
}

impl EntryOwned {
    fn to_ref_entry(&self) -> EntryRef<'_> {
        match self {
            Self::Entry(entry) => EntryRef::Entry(entry),
            Self::FoldedDirs(worktree_id, dirs) => EntryRef::FoldedDirs(*worktree_id, dirs),
            Self::Excerpt(buffer_id, excerpt_id, range) => {
                EntryRef::Excerpt(*buffer_id, *excerpt_id, range)
            }
            Self::Outline(buffer_id, excerpt_id, outline) => {
                EntryRef::Outline(*buffer_id, *excerpt_id, outline)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntryRef<'a> {
    Entry(&'a FsEntry),
    FoldedDirs(WorktreeId, &'a [Entry]),
    Excerpt(BufferId, ExcerptId, &'a ExcerptRange<language::Anchor>),
    Outline(BufferId, ExcerptId, &'a Outline),
}

impl EntryRef<'_> {
    fn to_owned_entry(&self) -> EntryOwned {
        match self {
            &Self::Entry(entry) => EntryOwned::Entry(entry.clone()),
            &Self::FoldedDirs(worktree_id, dirs) => {
                EntryOwned::FoldedDirs(worktree_id, dirs.to_vec())
            }
            &Self::Excerpt(buffer_id, excerpt_id, range) => {
                EntryOwned::Excerpt(buffer_id, excerpt_id, range.clone())
            }
            &Self::Outline(buffer_id, excerpt_id, outline) => {
                EntryOwned::Outline(buffer_id, excerpt_id, outline.clone())
            }
        }
    }
}

#[derive(Clone, Debug, Eq)]
enum FsEntry {
    ExternalFile(BufferId, Vec<ExcerptId>),
    Directory(WorktreeId, Entry),
    File(WorktreeId, Entry, BufferId, Vec<ExcerptId>),
}

impl PartialEq for FsEntry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ExternalFile(id_a, _), Self::ExternalFile(id_b, _)) => id_a == id_b,
            (Self::Directory(id_a, entry_a), Self::Directory(id_b, entry_b)) => {
                id_a == id_b && entry_a.id == entry_b.id
            }
            (
                Self::File(worktree_a, entry_a, id_a, ..),
                Self::File(worktree_b, entry_b, id_b, ..),
            ) => worktree_a == worktree_b && entry_a.id == entry_b.id && id_a == id_b,
            _ => false,
        }
    }
}

struct ActiveItem {
    item_id: EntityId,
    active_editor: WeakView<Editor>,
    _editor_subscrpiption: Subscription,
}

#[derive(Debug)]
pub enum Event {
    Focus,
}

#[derive(Serialize, Deserialize)]
struct SerializedOutlinePanel {
    width: Option<Pixels>,
    active: Option<bool>,
}

pub fn init_settings(cx: &mut AppContext) {
    OutlinePanelSettings::register(cx);
}

pub fn init(assets: impl AssetSource, cx: &mut AppContext) {
    init_settings(cx);
    file_icons::init(assets, cx);

    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, cx| {
            workspace.toggle_panel_focus::<OutlinePanel>(cx);
        });
    })
    .detach();
}

impl OutlinePanel {
    pub async fn load(
        workspace: WeakView<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<View<Self>> {
        let serialized_panel = cx
            .background_executor()
            .spawn(async move { KEY_VALUE_STORE.read_kvp(OUTLINE_PANEL_KEY) })
            .await
            .context("loading outline panel")
            .log_err()
            .flatten()
            .map(|panel| serde_json::from_str::<SerializedOutlinePanel>(&panel))
            .transpose()
            .log_err()
            .flatten();

        workspace.update(&mut cx, |workspace, cx| {
            let panel = Self::new(workspace, cx);
            if let Some(serialized_panel) = serialized_panel {
                panel.update(cx, |panel, cx| {
                    panel.width = serialized_panel.width.map(|px| px.round());
                    panel.active = serialized_panel.active.unwrap_or(false);
                    cx.notify();
                });
            }
            panel
        })
    }

    fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let project = workspace.project().clone();
        let outline_panel = cx.new_view(|cx| {
            let focus_handle = cx.focus_handle();
            let focus_subscription = cx.on_focus(&focus_handle, Self::focus_in);
            let workspace_subscription = cx.subscribe(
                &workspace
                    .weak_handle()
                    .upgrade()
                    .expect("have a &mut Workspace"),
                move |outline_panel, workspace, event, cx| {
                    if let workspace::Event::ActiveItemChanged = event {
                        if let Some(new_active_editor) = workspace
                            .read(cx)
                            .active_item(cx)
                            .and_then(|item| item.act_as::<Editor>(cx))
                        {
                            let active_editor_updated = outline_panel
                                .active_item
                                .as_ref()
                                .map_or(true, |active_item| {
                                    active_item.item_id != new_active_editor.item_id()
                                });
                            if active_editor_updated {
                                outline_panel.replace_visible_entries(new_active_editor, cx);
                            }
                        } else {
                            outline_panel.clear_previous();
                            cx.notify();
                        }
                    }
                },
            );

            let icons_subscription = cx.observe_global::<FileIcons>(|_, cx| {
                cx.notify();
            });

            let mut outline_panel_settings = *OutlinePanelSettings::get_global(cx);
            let settings_subscription = cx.observe_global::<SettingsStore>(move |_, cx| {
                let new_settings = *OutlinePanelSettings::get_global(cx);
                if outline_panel_settings != new_settings {
                    outline_panel_settings = new_settings;
                    cx.notify();
                }
            });

            let mut outline_panel = Self {
                active: false,
                project: project.clone(),
                fs: workspace.app_state().fs.clone(),
                scroll_handle: UniformListScrollHandle::new(),
                focus_handle,
                fs_entries: Vec::new(),
                fs_entries_depth: HashMap::default(),
                collapsed_entries: HashSet::default(),
                unfolded_dirs: HashMap::default(),
                selected_entry: None,
                context_menu: None,
                width: None,
                active_item: None,
                pending_serialization: Task::ready(None),
                loading_outlines: false,
                update_task: Task::ready(()),
                outline_fetch_tasks: HashMap::default(),
                excerpts: HashMap::default(),
                last_visible_range: 0..0,
                cached_entries_with_depth: None,
                _subscriptions: vec![
                    settings_subscription,
                    icons_subscription,
                    focus_subscription,
                    workspace_subscription,
                ],
            };
            if let Some(editor) = workspace
                .active_item(cx)
                .and_then(|item| item.act_as::<Editor>(cx))
            {
                outline_panel.replace_visible_entries(editor, cx);
            }
            outline_panel
        });

        outline_panel
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        let active = Some(self.active);
        self.pending_serialization = cx.background_executor().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        OUTLINE_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedOutlinePanel { width, active })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    fn dispatch_context(&self, _: &ViewContext<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("OutlinePanel");
        dispatch_context.add("menu");
        dispatch_context
    }

    fn unfold_directory(&mut self, _: &UnfoldDirectory, cx: &mut ViewContext<Self>) {
        let Some(editor) = self
            .active_item
            .as_ref()
            .and_then(|item| item.active_editor.upgrade())
        else {
            return;
        };
        if let Some(EntryOwned::FoldedDirs(worktree_id, entries)) = &self.selected_entry {
            self.unfolded_dirs
                .entry(*worktree_id)
                .or_default()
                .extend(entries.iter().map(|entry| entry.id));
            self.update_fs_entries(&editor, HashSet::default(), None, None, false, cx);
        }
    }

    fn fold_directory(&mut self, _: &FoldDirectory, cx: &mut ViewContext<Self>) {
        let Some(editor) = self
            .active_item
            .as_ref()
            .and_then(|item| item.active_editor.upgrade())
        else {
            return;
        };

        let (worktree_id, entry) = match &self.selected_entry {
            Some(EntryOwned::Entry(FsEntry::Directory(worktree_id, entry))) => {
                (worktree_id, Some(entry))
            }
            Some(EntryOwned::FoldedDirs(worktree_id, entries)) => (worktree_id, entries.last()),
            _ => return,
        };
        let Some(entry) = entry else {
            return;
        };
        let unfolded_dirs = self.unfolded_dirs.get_mut(worktree_id);
        let worktree = self
            .project
            .read(cx)
            .worktree_for_id(*worktree_id, cx)
            .map(|w| w.read(cx).snapshot());
        let Some((worktree, unfolded_dirs)) = worktree.zip(unfolded_dirs) else {
            return;
        };

        unfolded_dirs.remove(&entry.id);
        let mut parent = entry.path.parent();
        while let Some(parent_path) = parent {
            let removed = worktree.entry_for_path(parent_path).map_or(false, |entry| {
                if worktree.root_entry().map(|entry| entry.id) == Some(entry.id) {
                    false
                } else {
                    unfolded_dirs.remove(&entry.id)
                }
            });

            if removed {
                parent = parent_path.parent();
            } else {
                break;
            }
        }
        for child_dir in worktree
            .child_entries(&entry.path)
            .filter(|entry| entry.is_dir())
        {
            let removed = unfolded_dirs.remove(&child_dir.id);
            if !removed {
                break;
            }
        }

        self.update_fs_entries(&editor, HashSet::default(), None, None, false, cx);
    }

    fn open(&mut self, _: &Open, cx: &mut ViewContext<Self>) {
        if let Some(selected_entry) = self.selected_entry.clone() {
            self.open_entry(&selected_entry, cx);
        }
    }

    fn open_entry(&mut self, entry: &EntryOwned, cx: &mut ViewContext<OutlinePanel>) {
        let Some(active_editor) = self
            .active_item
            .as_ref()
            .and_then(|item| item.active_editor.upgrade())
        else {
            return;
        };
        let active_multi_buffer = active_editor.read(cx).buffer().clone();
        let multi_buffer_snapshot = active_multi_buffer.read(cx).snapshot(cx);
        let offset_from_top = if active_multi_buffer.read(cx).is_singleton() {
            Point::default()
        } else {
            Point::new(0.0, -(active_editor.read(cx).file_header_size() as f32))
        };

        self.toggle_expanded(entry, cx);
        match entry {
            EntryOwned::FoldedDirs(..) | EntryOwned::Entry(FsEntry::Directory(..)) => {}
            EntryOwned::Entry(FsEntry::ExternalFile(buffer_id, _)) => {
                let scroll_target = multi_buffer_snapshot.excerpts().find_map(
                    |(excerpt_id, buffer_snapshot, excerpt_range)| {
                        if &buffer_snapshot.remote_id() == buffer_id {
                            multi_buffer_snapshot
                                .anchor_in_excerpt(excerpt_id, excerpt_range.context.start)
                        } else {
                            None
                        }
                    },
                );
                if let Some(anchor) = scroll_target {
                    self.selected_entry = Some(entry.clone());
                    active_editor.update(cx, |editor, cx| {
                        editor.set_scroll_anchor(
                            ScrollAnchor {
                                offset: offset_from_top,
                                anchor,
                            },
                            cx,
                        );
                    })
                }
            }
            EntryOwned::Entry(FsEntry::File(_, file_entry, ..)) => {
                let scroll_target = self
                    .project
                    .update(cx, |project, cx| {
                        project
                            .path_for_entry(file_entry.id, cx)
                            .and_then(|path| project.get_open_buffer(&path, cx))
                    })
                    .map(|buffer| {
                        active_multi_buffer
                            .read(cx)
                            .excerpts_for_buffer(&buffer, cx)
                    })
                    .and_then(|excerpts| {
                        let (excerpt_id, excerpt_range) = excerpts.first()?;
                        multi_buffer_snapshot
                            .anchor_in_excerpt(*excerpt_id, excerpt_range.context.start)
                    });
                if let Some(anchor) = scroll_target {
                    self.selected_entry = Some(entry.clone());
                    active_editor.update(cx, |editor, cx| {
                        editor.set_scroll_anchor(
                            ScrollAnchor {
                                offset: offset_from_top,
                                anchor,
                            },
                            cx,
                        );
                    })
                }
            }
            EntryOwned::Outline(_, excerpt_id, outline) => {
                let scroll_target = multi_buffer_snapshot
                    .anchor_in_excerpt(*excerpt_id, outline.range.start)
                    .or_else(|| {
                        multi_buffer_snapshot.anchor_in_excerpt(*excerpt_id, outline.range.end)
                    });
                if let Some(anchor) = scroll_target {
                    self.selected_entry = Some(entry.clone());
                    active_editor.update(cx, |editor, cx| {
                        editor.set_scroll_anchor(
                            ScrollAnchor {
                                offset: Point::default(),
                                anchor,
                            },
                            cx,
                        );
                    })
                }
            }
            EntryOwned::Excerpt(_, excerpt_id, excerpt_range) => {
                let scroll_target = multi_buffer_snapshot
                    .anchor_in_excerpt(*excerpt_id, excerpt_range.context.start);
                if let Some(anchor) = scroll_target {
                    self.selected_entry = Some(entry.clone());
                    active_editor.update(cx, |editor, cx| {
                        editor.set_scroll_anchor(
                            ScrollAnchor {
                                offset: Point::default(),
                                anchor,
                            },
                            cx,
                        );
                    })
                }
            }
        }
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if let Some(entry_to_select) = self.selected_entry.clone().and_then(|selected_entry| {
            self.entries_with_depths(cx)
                .iter()
                .map(|(_, entry)| entry)
                .skip_while(|entry| entry != &&selected_entry)
                .skip(1)
                .next()
                .cloned()
        }) {
            self.selected_entry = Some(entry_to_select);
            self.autoscroll(cx);
            cx.notify();
        } else {
            self.select_first(&SelectFirst {}, cx)
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if let Some(entry_to_select) = self.selected_entry.clone().and_then(|selected_entry| {
            self.entries_with_depths(cx)
                .iter()
                .rev()
                .map(|(_, entry)| entry)
                .skip_while(|entry| entry != &&selected_entry)
                .skip(1)
                .next()
                .cloned()
        }) {
            self.selected_entry = Some(entry_to_select);
            self.autoscroll(cx);
            cx.notify();
        } else {
            self.select_first(&SelectFirst {}, cx)
        }
    }

    fn select_parent(&mut self, _: &SelectParent, cx: &mut ViewContext<Self>) {
        if let Some(entry_to_select) = self.selected_entry.clone().and_then(|selected_entry| {
            let mut previous_entries = self
                .entries_with_depths(cx)
                .iter()
                .rev()
                .map(|(_, entry)| entry)
                .skip_while(|entry| entry != &&selected_entry)
                .skip(1);
            match &selected_entry {
                EntryOwned::Entry(fs_entry) => match fs_entry {
                    FsEntry::ExternalFile(..) => None,
                    FsEntry::File(worktree_id, entry, ..)
                    | FsEntry::Directory(worktree_id, entry) => {
                        entry.path.parent().and_then(|parent_path| {
                            previous_entries.find(|entry| match entry {
                                EntryOwned::Entry(FsEntry::Directory(
                                    dir_worktree_id,
                                    dir_entry,
                                )) => {
                                    dir_worktree_id == worktree_id
                                        && dir_entry.path.as_ref() == parent_path
                                }
                                EntryOwned::FoldedDirs(dirs_worktree_id, dirs) => {
                                    dirs_worktree_id == worktree_id
                                        && dirs
                                            .first()
                                            .map_or(false, |dir| dir.path.as_ref() == parent_path)
                                }
                                _ => false,
                            })
                        })
                    }
                },
                EntryOwned::FoldedDirs(worktree_id, entries) => entries
                    .first()
                    .and_then(|entry| entry.path.parent())
                    .and_then(|parent_path| {
                        previous_entries.find(|entry| {
                            if let EntryOwned::Entry(FsEntry::Directory(
                                dir_worktree_id,
                                dir_entry,
                            )) = entry
                            {
                                dir_worktree_id == worktree_id
                                    && dir_entry.path.as_ref() == parent_path
                            } else {
                                false
                            }
                        })
                    }),
                EntryOwned::Excerpt(excerpt_buffer_id, excerpt_id, _) => {
                    previous_entries.find(|entry| match entry {
                        EntryOwned::Entry(FsEntry::File(_, _, file_buffer_id, file_excerpts)) => {
                            file_buffer_id == excerpt_buffer_id
                                && file_excerpts.contains(&excerpt_id)
                        }
                        EntryOwned::Entry(FsEntry::ExternalFile(file_buffer_id, file_excerpts)) => {
                            file_buffer_id == excerpt_buffer_id
                                && file_excerpts.contains(&excerpt_id)
                        }
                        _ => false,
                    })
                }
                EntryOwned::Outline(outline_buffer_id, outline_excerpt_id, _) => previous_entries
                    .find(|entry| {
                        if let EntryOwned::Excerpt(excerpt_buffer_id, excerpt_id, _) = entry {
                            outline_buffer_id == excerpt_buffer_id
                                && outline_excerpt_id == excerpt_id
                        } else {
                            false
                        }
                    }),
            }
        }) {
            self.selected_entry = Some(entry_to_select.clone());
            self.autoscroll(cx);
            cx.notify();
        } else {
            self.select_first(&SelectFirst {}, cx);
        }
    }

    fn select_first(&mut self, _: &SelectFirst, cx: &mut ViewContext<Self>) {
        if let Some((_, first_entry)) = self.entries_with_depths(cx).iter().next() {
            self.selected_entry = Some(first_entry.clone());
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &SelectLast, cx: &mut ViewContext<Self>) {
        if let Some(new_selection) = self
            .entries_with_depths(cx)
            .iter()
            .rev()
            .map(|(_, entry)| entry)
            .next()
        {
            self.selected_entry = Some(new_selection.clone());
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn autoscroll(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(selected_entry) = self.selected_entry.clone() {
            let index = self
                .entries_with_depths(cx)
                .iter()
                .position(|(_, entry)| entry == &selected_entry);
            if let Some(index) = index {
                self.scroll_handle.scroll_to_item(index);
                cx.notify();
            }
        }
    }

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        if !self.focus_handle.contains_focused(cx) {
            cx.emit(Event::Focus);
        }
    }

    fn deploy_context_menu(
        &mut self,
        position: Point<Pixels>,
        entry: EntryRef<'_>,
        cx: &mut ViewContext<Self>,
    ) {
        self.selected_entry = Some(entry.to_owned_entry());
        let is_root = match entry {
            EntryRef::Entry(FsEntry::File(worktree_id, entry, ..))
            | EntryRef::Entry(FsEntry::Directory(worktree_id, entry)) => self
                .project
                .read(cx)
                .worktree_for_id(*worktree_id, cx)
                .map(|worktree| {
                    worktree.read(cx).root_entry().map(|entry| entry.id) == Some(entry.id)
                })
                .unwrap_or(false),
            EntryRef::FoldedDirs(worktree_id, entries) => entries
                .first()
                .and_then(|entry| {
                    self.project
                        .read(cx)
                        .worktree_for_id(worktree_id, cx)
                        .map(|worktree| {
                            worktree.read(cx).root_entry().map(|entry| entry.id) == Some(entry.id)
                        })
                })
                .unwrap_or(false),
            EntryRef::Entry(FsEntry::ExternalFile(..)) => false,
            EntryRef::Excerpt(..) => {
                cx.notify();
                return;
            }
            EntryRef::Outline(..) => {
                cx.notify();
                return;
            }
        };
        let auto_fold_dirs = OutlinePanelSettings::get_global(cx).auto_fold_dirs;
        let is_foldable = auto_fold_dirs && !is_root && self.is_foldable(entry);
        let is_unfoldable = auto_fold_dirs && !is_root && self.is_unfoldable(entry);

        let context_menu = ContextMenu::build(cx, |menu, _| {
            menu.context(self.focus_handle.clone())
                .when(cfg!(target_os = "macos"), |menu| {
                    menu.action("Reveal in Finder", Box::new(RevealInFileManager))
                })
                .when(cfg!(not(target_os = "macos")), |menu| {
                    menu.action("Reveal in File Manager", Box::new(RevealInFileManager))
                })
                .action("Open in Terminal", Box::new(OpenInTerminal))
                .when(is_unfoldable, |menu| {
                    menu.action("Unfold Directory", Box::new(UnfoldDirectory))
                })
                .when(is_foldable, |menu| {
                    menu.action("Fold Directory", Box::new(FoldDirectory))
                })
                .separator()
                .action("Copy Path", Box::new(CopyPath))
                .action("Copy Relative Path", Box::new(CopyRelativePath))
        });
        cx.focus_view(&context_menu);
        let subscription = cx.subscribe(&context_menu, |outline_panel, _, _: &DismissEvent, cx| {
            outline_panel.context_menu.take();
            cx.notify();
        });
        self.context_menu = Some((context_menu, position, subscription));
        cx.notify();
    }

    fn is_unfoldable(&self, entry: EntryRef) -> bool {
        matches!(entry, EntryRef::FoldedDirs(..))
    }

    fn is_foldable(&self, entry: EntryRef) -> bool {
        let (directory_worktree, directory_entry) = match entry {
            EntryRef::Entry(FsEntry::Directory(directory_worktree, directory_entry)) => {
                (*directory_worktree, Some(directory_entry))
            }
            EntryRef::FoldedDirs(directory_worktree, entries) => {
                (directory_worktree, entries.last())
            }
            _ => return false,
        };
        let Some(directory_entry) = directory_entry else {
            return false;
        };

        if self
            .unfolded_dirs
            .get(&directory_worktree)
            .map_or(false, |unfolded_dirs| {
                unfolded_dirs.contains(&directory_entry.id)
            })
        {
            return true;
        }

        let child_entries = self
            .fs_entries
            .iter()
            .skip_while(|entry| {
                if let FsEntry::Directory(worktree_id, entry) = entry {
                    worktree_id != &directory_worktree || entry.id != directory_entry.id
                } else {
                    true
                }
            })
            .skip(1)
            .filter(|next_entry| match next_entry {
                FsEntry::ExternalFile(..) => false,
                FsEntry::Directory(worktree_id, entry) | FsEntry::File(worktree_id, entry, ..) => {
                    worktree_id == &directory_worktree
                        && entry.path.parent() == Some(directory_entry.path.as_ref())
                }
            })
            .collect::<Vec<_>>();

        child_entries.len() == 1 && matches!(child_entries.first(), Some(FsEntry::Directory(..)))
    }

    fn expand_selected_entry(&mut self, _: &ExpandSelectedEntry, cx: &mut ViewContext<Self>) {
        let Some(editor) = self
            .active_item
            .as_ref()
            .and_then(|item| item.active_editor.upgrade())
        else {
            return;
        };

        let entry_to_expand = match &self.selected_entry {
            Some(EntryOwned::FoldedDirs(worktree_id, dir_entries)) => dir_entries
                .last()
                .map(|entry| CollapsedEntry::Dir(*worktree_id, entry.id)),
            Some(EntryOwned::Entry(FsEntry::Directory(worktree_id, dir_entry))) => {
                Some(CollapsedEntry::Dir(*worktree_id, dir_entry.id))
            }
            Some(EntryOwned::Entry(FsEntry::File(worktree_id, _, buffer_id, _))) => {
                Some(CollapsedEntry::File(*worktree_id, *buffer_id))
            }
            Some(EntryOwned::Entry(FsEntry::ExternalFile(buffer_id, _))) => {
                Some(CollapsedEntry::ExternalFile(*buffer_id))
            }
            Some(EntryOwned::Excerpt(buffer_id, excerpt_id, _)) => {
                Some(CollapsedEntry::Excerpt(*buffer_id, *excerpt_id))
            }
            None | Some(EntryOwned::Outline(..)) => None,
        };
        let Some(collapsed_entry) = entry_to_expand else {
            return;
        };
        let expanded = self.collapsed_entries.remove(&collapsed_entry);
        if expanded {
            if let CollapsedEntry::Dir(worktree_id, dir_entry_id) = collapsed_entry {
                self.project.update(cx, |project, cx| {
                    project.expand_entry(worktree_id, dir_entry_id, cx);
                });
            }
            self.update_fs_entries(&editor, HashSet::default(), None, None, false, cx);
        } else {
            self.select_next(&SelectNext, cx)
        }
    }

    fn collapse_selected_entry(&mut self, _: &CollapseSelectedEntry, cx: &mut ViewContext<Self>) {
        let Some(editor) = self
            .active_item
            .as_ref()
            .and_then(|item| item.active_editor.upgrade())
        else {
            return;
        };
        match &self.selected_entry {
            Some(
                dir_entry @ EntryOwned::Entry(FsEntry::Directory(worktree_id, selected_dir_entry)),
            ) => {
                self.collapsed_entries
                    .insert(CollapsedEntry::Dir(*worktree_id, selected_dir_entry.id));
                self.update_fs_entries(
                    &editor,
                    HashSet::default(),
                    Some(dir_entry.clone()),
                    None,
                    false,
                    cx,
                );
            }
            Some(file_entry @ EntryOwned::Entry(FsEntry::File(worktree_id, _, buffer_id, _))) => {
                self.collapsed_entries
                    .insert(CollapsedEntry::File(*worktree_id, *buffer_id));
                self.update_fs_entries(
                    &editor,
                    HashSet::default(),
                    Some(file_entry.clone()),
                    None,
                    false,
                    cx,
                );
            }
            Some(file_entry @ EntryOwned::Entry(FsEntry::ExternalFile(buffer_id, _))) => {
                self.collapsed_entries
                    .insert(CollapsedEntry::ExternalFile(*buffer_id));
                self.update_fs_entries(
                    &editor,
                    HashSet::default(),
                    Some(file_entry.clone()),
                    None,
                    false,
                    cx,
                );
            }
            Some(dirs_entry @ EntryOwned::FoldedDirs(worktree_id, dir_entries)) => {
                if let Some(dir_entry) = dir_entries.last() {
                    if self
                        .collapsed_entries
                        .insert(CollapsedEntry::Dir(*worktree_id, dir_entry.id))
                    {
                        self.update_fs_entries(
                            &editor,
                            HashSet::default(),
                            Some(dirs_entry.clone()),
                            None,
                            false,
                            cx,
                        );
                    }
                }
            }
            Some(excerpt_entry @ EntryOwned::Excerpt(buffer_id, excerpt_id, _)) => {
                if self
                    .collapsed_entries
                    .insert(CollapsedEntry::Excerpt(*buffer_id, *excerpt_id))
                {
                    self.update_fs_entries(
                        &editor,
                        HashSet::default(),
                        Some(excerpt_entry.clone()),
                        None,
                        false,
                        cx,
                    );
                }
            }
            None | Some(EntryOwned::Outline(..)) => {}
        }
    }

    pub fn collapse_all_entries(&mut self, _: &CollapseAllEntries, cx: &mut ViewContext<Self>) {
        let Some(editor) = self
            .active_item
            .as_ref()
            .and_then(|item| item.active_editor.upgrade())
        else {
            return;
        };

        let new_entries = self
            .entries_with_depths(cx)
            .iter()
            .flat_map(|(_, entry)| match entry {
                EntryOwned::Entry(FsEntry::Directory(worktree_id, entry)) => {
                    Some(CollapsedEntry::Dir(*worktree_id, entry.id))
                }
                EntryOwned::Entry(FsEntry::File(worktree_id, _, buffer_id, _)) => {
                    Some(CollapsedEntry::File(*worktree_id, *buffer_id))
                }
                EntryOwned::Entry(FsEntry::ExternalFile(buffer_id, _)) => {
                    Some(CollapsedEntry::ExternalFile(*buffer_id))
                }
                EntryOwned::FoldedDirs(worktree_id, entries) => {
                    Some(CollapsedEntry::Dir(*worktree_id, entries.last()?.id))
                }
                EntryOwned::Excerpt(buffer_id, excerpt_id, _) => {
                    Some(CollapsedEntry::Excerpt(*buffer_id, *excerpt_id))
                }
                EntryOwned::Outline(..) => None,
            })
            .collect::<Vec<_>>();
        self.collapsed_entries.extend(new_entries);
        self.update_fs_entries(&editor, HashSet::default(), None, None, false, cx);
    }

    fn toggle_expanded(&mut self, entry: &EntryOwned, cx: &mut ViewContext<Self>) {
        let Some(editor) = self
            .active_item
            .as_ref()
            .and_then(|item| item.active_editor.upgrade())
        else {
            return;
        };

        match entry {
            EntryOwned::Entry(FsEntry::Directory(worktree_id, dir_entry)) => {
                let entry_id = dir_entry.id;
                let collapsed_entry = CollapsedEntry::Dir(*worktree_id, entry_id);
                if self.collapsed_entries.remove(&collapsed_entry) {
                    self.project
                        .update(cx, |project, cx| {
                            project.expand_entry(*worktree_id, entry_id, cx)
                        })
                        .unwrap_or_else(|| Task::ready(Ok(())))
                        .detach_and_log_err(cx);
                } else {
                    self.collapsed_entries.insert(collapsed_entry);
                }
            }
            EntryOwned::Entry(FsEntry::File(worktree_id, _, buffer_id, _)) => {
                let collapsed_entry = CollapsedEntry::File(*worktree_id, *buffer_id);
                if !self.collapsed_entries.remove(&collapsed_entry) {
                    self.collapsed_entries.insert(collapsed_entry);
                }
            }
            EntryOwned::Entry(FsEntry::ExternalFile(buffer_id, _)) => {
                let collapsed_entry = CollapsedEntry::ExternalFile(*buffer_id);
                if !self.collapsed_entries.remove(&collapsed_entry) {
                    self.collapsed_entries.insert(collapsed_entry);
                }
            }
            EntryOwned::FoldedDirs(worktree_id, dir_entries) => {
                if let Some(entry_id) = dir_entries.first().map(|entry| entry.id) {
                    let collapsed_entry = CollapsedEntry::Dir(*worktree_id, entry_id);
                    if self.collapsed_entries.remove(&collapsed_entry) {
                        self.project
                            .update(cx, |project, cx| {
                                project.expand_entry(*worktree_id, entry_id, cx)
                            })
                            .unwrap_or_else(|| Task::ready(Ok(())))
                            .detach_and_log_err(cx);
                    } else {
                        self.collapsed_entries.insert(collapsed_entry);
                    }
                }
            }
            EntryOwned::Excerpt(buffer_id, excerpt_id, _) => {
                let collapsed_entry = CollapsedEntry::Excerpt(*buffer_id, *excerpt_id);
                if !self.collapsed_entries.remove(&collapsed_entry) {
                    self.collapsed_entries.insert(collapsed_entry);
                }
            }
            EntryOwned::Outline(..) => return,
        }

        self.update_fs_entries(
            &editor,
            HashSet::default(),
            Some(entry.clone()),
            None,
            false,
            cx,
        );
    }

    fn copy_path(&mut self, _: &CopyPath, cx: &mut ViewContext<Self>) {
        if let Some(clipboard_text) = self
            .selected_entry
            .as_ref()
            .and_then(|entry| self.abs_path(&entry, cx))
            .map(|p| p.to_string_lossy().to_string())
        {
            cx.write_to_clipboard(ClipboardItem::new(clipboard_text));
        }
    }

    fn copy_relative_path(&mut self, _: &CopyRelativePath, cx: &mut ViewContext<Self>) {
        if let Some(clipboard_text) = self
            .selected_entry
            .as_ref()
            .and_then(|entry| match entry {
                EntryOwned::Entry(entry) => self.relative_path(&entry, cx),
                EntryOwned::FoldedDirs(_, dirs) => {
                    dirs.last().map(|entry| entry.path.to_path_buf())
                }
                EntryOwned::Excerpt(..) | EntryOwned::Outline(..) => None,
            })
            .map(|p| p.to_string_lossy().to_string())
        {
            cx.write_to_clipboard(ClipboardItem::new(clipboard_text));
        }
    }

    fn reveal_in_finder(&mut self, _: &RevealInFileManager, cx: &mut ViewContext<Self>) {
        if let Some(abs_path) = self
            .selected_entry
            .as_ref()
            .and_then(|entry| self.abs_path(&entry, cx))
        {
            cx.reveal_path(&abs_path);
        }
    }

    fn open_in_terminal(&mut self, _: &OpenInTerminal, cx: &mut ViewContext<Self>) {
        let selected_entry = self.selected_entry.as_ref();
        let abs_path = selected_entry.and_then(|entry| self.abs_path(&entry, cx));
        let working_directory = if let (
            Some(abs_path),
            Some(EntryOwned::Entry(FsEntry::File(..) | FsEntry::ExternalFile(..))),
        ) = (&abs_path, selected_entry)
        {
            abs_path.parent().map(|p| p.to_owned())
        } else {
            abs_path
        };

        if let Some(working_directory) = working_directory {
            cx.dispatch_action(workspace::OpenTerminal { working_directory }.boxed_clone())
        }
    }

    fn reveal_entry_for_selection(
        &mut self,
        editor: &View<Editor>,
        cx: &mut ViewContext<'_, Self>,
    ) {
        if !OutlinePanelSettings::get_global(cx).auto_reveal_entries {
            return;
        }
        let Some(entry_with_selection) = self.location_for_editor_selection(editor, cx) else {
            self.selected_entry = None;
            cx.notify();
            return;
        };
        let related_buffer_entry = match entry_with_selection {
            EntryOwned::Entry(FsEntry::File(worktree_id, _, buffer_id, _)) => {
                let project = self.project.read(cx);
                let entry_id = project
                    .buffer_for_id(buffer_id)
                    .and_then(|buffer| buffer.read(cx).entry_id(cx));
                project
                    .worktree_for_id(worktree_id, cx)
                    .zip(entry_id)
                    .and_then(|(worktree, entry_id)| {
                        let entry = worktree.read(cx).entry_for_id(entry_id)?.clone();
                        Some((worktree, entry))
                    })
            }
            EntryOwned::Outline(buffer_id, excerpt_id, _)
            | EntryOwned::Excerpt(buffer_id, excerpt_id, _) => {
                self.collapsed_entries
                    .remove(&CollapsedEntry::ExternalFile(buffer_id));
                self.collapsed_entries
                    .remove(&CollapsedEntry::Excerpt(buffer_id, excerpt_id));
                let project = self.project.read(cx);
                let entry_id = project
                    .buffer_for_id(buffer_id)
                    .and_then(|buffer| buffer.read(cx).entry_id(cx));

                entry_id.and_then(|entry_id| {
                    project
                        .worktree_for_entry(entry_id, cx)
                        .and_then(|worktree| {
                            let worktree_id = worktree.read(cx).id();
                            self.collapsed_entries
                                .remove(&CollapsedEntry::File(worktree_id, buffer_id));
                            let entry = worktree.read(cx).entry_for_id(entry_id)?.clone();
                            Some((worktree, entry))
                        })
                })
            }
            EntryOwned::Entry(FsEntry::ExternalFile(..)) => None,
            _ => return,
        };
        if let Some((worktree, buffer_entry)) = related_buffer_entry {
            let worktree_id = worktree.read(cx).id();
            let mut dirs_to_expand = Vec::new();
            {
                let mut traversal = worktree.read(cx).traverse_from_path(
                    true,
                    true,
                    true,
                    buffer_entry.path.as_ref(),
                );
                let mut current_entry = buffer_entry;
                loop {
                    if current_entry.is_dir() {
                        if self
                            .collapsed_entries
                            .remove(&CollapsedEntry::Dir(worktree_id, current_entry.id))
                        {
                            dirs_to_expand.push(current_entry.id);
                        }
                    }

                    if traversal.back_to_parent() {
                        if let Some(parent_entry) = traversal.entry() {
                            current_entry = parent_entry.clone();
                            continue;
                        }
                    }
                    break;
                }
            }
            for dir_to_expand in dirs_to_expand {
                self.project
                    .update(cx, |project, cx| {
                        project.expand_entry(worktree_id, dir_to_expand, cx)
                    })
                    .unwrap_or_else(|| Task::ready(Ok(())))
                    .detach_and_log_err(cx)
            }
        }

        self.update_fs_entries(
            &editor,
            HashSet::default(),
            Some(entry_with_selection),
            None,
            false,
            cx,
        );
    }

    fn render_excerpt(
        &self,
        buffer_id: BufferId,
        excerpt_id: ExcerptId,
        range: &ExcerptRange<language::Anchor>,
        depth: usize,
        cx: &mut ViewContext<OutlinePanel>,
    ) -> Option<Stateful<Div>> {
        let item_id = ElementId::from(excerpt_id.to_proto() as usize);
        let is_active = match &self.selected_entry {
            Some(EntryOwned::Excerpt(selected_buffer_id, selected_excerpt_id, _)) => {
                selected_buffer_id == &buffer_id && selected_excerpt_id == &excerpt_id
            }
            _ => false,
        };
        let has_outlines = self
            .excerpts
            .get(&buffer_id)
            .and_then(|excerpts| match &excerpts.get(&excerpt_id)?.outlines {
                ExcerptOutlines::Outlines(outlines) => Some(outlines),
                ExcerptOutlines::Invalidated(outlines) => Some(outlines),
                ExcerptOutlines::NotFetched => None,
            })
            .map_or(false, |outlines| !outlines.is_empty());
        let is_expanded = !self
            .collapsed_entries
            .contains(&CollapsedEntry::Excerpt(buffer_id, excerpt_id));
        let color = entry_git_aware_label_color(None, false, is_active);
        let icon = if has_outlines {
            FileIcons::get_chevron_icon(is_expanded, cx)
                .map(|icon_path| Icon::from_path(icon_path).color(color).into_any_element())
        } else {
            None
        }
        .unwrap_or_else(empty_icon);

        let buffer_snapshot = self.buffer_snapshot_for_id(buffer_id, cx)?;
        let excerpt_range = range.context.to_point(&buffer_snapshot);
        let label_element = Label::new(format!(
            "Lines {}-{}",
            excerpt_range.start.row + 1,
            excerpt_range.end.row + 1,
        ))
        .single_line()
        .color(color)
        .into_any_element();

        Some(self.entry_element(
            EntryRef::Excerpt(buffer_id, excerpt_id, range),
            item_id,
            depth,
            Some(icon),
            is_active,
            label_element,
            cx,
        ))
    }

    fn render_outline(
        &self,
        buffer_id: BufferId,
        excerpt_id: ExcerptId,
        rendered_outline: &Outline,
        depth: usize,
        cx: &mut ViewContext<Self>,
    ) -> Stateful<Div> {
        let (item_id, label_element) = (
            ElementId::from(SharedString::from(format!(
                "{buffer_id:?}|{excerpt_id:?}{:?}|{:?}",
                rendered_outline.range, &rendered_outline.text,
            ))),
            language::render_item(&rendered_outline, None, cx).into_any_element(),
        );
        let is_active = match &self.selected_entry {
            Some(EntryOwned::Outline(selected_buffer_id, selected_excerpt_id, selected_entry)) => {
                selected_buffer_id == &buffer_id
                    && selected_excerpt_id == &excerpt_id
                    && selected_entry == rendered_outline
            }
            _ => false,
        };
        let icon = if self.is_singleton_active(cx) {
            None
        } else {
            Some(empty_icon())
        };
        self.entry_element(
            EntryRef::Outline(buffer_id, excerpt_id, rendered_outline),
            item_id,
            depth,
            icon,
            is_active,
            label_element,
            cx,
        )
    }

    fn render_entry(
        &self,
        rendered_entry: &FsEntry,
        depth: usize,
        cx: &mut ViewContext<Self>,
    ) -> Stateful<Div> {
        let settings = OutlinePanelSettings::get_global(cx);
        let is_active = match &self.selected_entry {
            Some(EntryOwned::Entry(selected_entry)) => selected_entry == rendered_entry,
            _ => false,
        };
        let (item_id, label_element, icon) = match rendered_entry {
            FsEntry::File(worktree_id, entry, ..) => {
                let name = self.entry_name(worktree_id, entry, cx);
                let color =
                    entry_git_aware_label_color(entry.git_status, entry.is_ignored, is_active);
                let icon = if settings.file_icons {
                    FileIcons::get_icon(&entry.path, cx)
                        .map(|icon_path| Icon::from_path(icon_path).color(color).into_any_element())
                } else {
                    None
                };
                (
                    ElementId::from(entry.id.to_proto() as usize),
                    Label::new(name)
                        .single_line()
                        .color(color)
                        .into_any_element(),
                    icon.unwrap_or_else(empty_icon),
                )
            }
            FsEntry::Directory(worktree_id, entry) => {
                let name = self.entry_name(worktree_id, entry, cx);

                let is_expanded = !self
                    .collapsed_entries
                    .contains(&CollapsedEntry::Dir(*worktree_id, entry.id));
                let color =
                    entry_git_aware_label_color(entry.git_status, entry.is_ignored, is_active);
                let icon = if settings.folder_icons {
                    FileIcons::get_folder_icon(is_expanded, cx)
                } else {
                    FileIcons::get_chevron_icon(is_expanded, cx)
                }
                .map(Icon::from_path)
                .map(|icon| icon.color(color).into_any_element());
                (
                    ElementId::from(entry.id.to_proto() as usize),
                    Label::new(name)
                        .single_line()
                        .color(color)
                        .into_any_element(),
                    icon.unwrap_or_else(empty_icon),
                )
            }
            FsEntry::ExternalFile(buffer_id, ..) => {
                let color = entry_label_color(is_active);
                let (icon, name) = match self.buffer_snapshot_for_id(*buffer_id, cx) {
                    Some(buffer_snapshot) => match buffer_snapshot.file() {
                        Some(file) => {
                            let path = file.path();
                            let icon = if settings.file_icons {
                                FileIcons::get_icon(path.as_ref(), cx)
                            } else {
                                None
                            }
                            .map(Icon::from_path)
                            .map(|icon| icon.color(color).into_any_element());
                            (icon, file_name(path.as_ref()))
                        }
                        None => (None, "Untitled".to_string()),
                    },
                    None => (None, "Unknown buffer".to_string()),
                };
                (
                    ElementId::from(buffer_id.to_proto() as usize),
                    Label::new(name)
                        .single_line()
                        .color(color)
                        .into_any_element(),
                    icon.unwrap_or_else(empty_icon),
                )
            }
        };

        self.entry_element(
            EntryRef::Entry(rendered_entry),
            item_id,
            depth,
            Some(icon),
            is_active,
            label_element,
            cx,
        )
    }

    fn render_folded_dirs(
        &self,
        worktree_id: WorktreeId,
        dir_entries: &[Entry],
        depth: usize,
        cx: &mut ViewContext<OutlinePanel>,
    ) -> Stateful<Div> {
        let settings = OutlinePanelSettings::get_global(cx);
        let is_active = match &self.selected_entry {
            Some(EntryOwned::FoldedDirs(selected_worktree_id, selected_entries)) => {
                selected_worktree_id == &worktree_id && selected_entries == dir_entries
            }
            _ => false,
        };
        let (item_id, label_element, icon) = {
            let name = dir_entries.iter().fold(String::new(), |mut name, entry| {
                if !name.is_empty() {
                    name.push(std::path::MAIN_SEPARATOR)
                }
                name.push_str(&self.entry_name(&worktree_id, entry, cx));
                name
            });

            let is_expanded = dir_entries.iter().all(|dir| {
                !self
                    .collapsed_entries
                    .contains(&CollapsedEntry::Dir(worktree_id, dir.id))
            });
            let is_ignored = dir_entries.iter().any(|entry| entry.is_ignored);
            let git_status = dir_entries.first().and_then(|entry| entry.git_status);
            let color = entry_git_aware_label_color(git_status, is_ignored, is_active);
            let icon = if settings.folder_icons {
                FileIcons::get_folder_icon(is_expanded, cx)
            } else {
                FileIcons::get_chevron_icon(is_expanded, cx)
            }
            .map(Icon::from_path)
            .map(|icon| icon.color(color).into_any_element());
            (
                ElementId::from(
                    dir_entries
                        .last()
                        .map(|entry| entry.id.to_proto())
                        .unwrap_or_else(|| worktree_id.to_proto()) as usize,
                ),
                Label::new(name)
                    .single_line()
                    .color(color)
                    .into_any_element(),
                icon.unwrap_or_else(empty_icon),
            )
        };

        self.entry_element(
            EntryRef::FoldedDirs(worktree_id, dir_entries),
            item_id,
            depth,
            Some(icon),
            is_active,
            label_element,
            cx,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn entry_element(
        &self,
        rendered_entry: EntryRef<'_>,
        item_id: ElementId,
        depth: usize,
        icon_element: Option<AnyElement>,
        is_active: bool,
        label_element: gpui::AnyElement,
        cx: &mut ViewContext<OutlinePanel>,
    ) -> Stateful<Div> {
        let settings = OutlinePanelSettings::get_global(cx);
        let rendered_entry = rendered_entry.to_owned_entry();
        div()
            .text_ui(cx)
            .id(item_id.clone())
            .child(
                ListItem::new(item_id)
                    .indent_level(depth)
                    .indent_step_size(px(settings.indent_size))
                    .selected(is_active)
                    .when_some(icon_element, |list_item, icon_element| {
                        list_item.child(h_flex().child(icon_element))
                    })
                    .child(h_flex().h_6().child(label_element).ml_1())
                    .on_click({
                        let clicked_entry = rendered_entry.clone();
                        cx.listener(move |outline_panel, event: &gpui::ClickEvent, cx| {
                            if event.down.button == MouseButton::Right || event.down.first_mouse {
                                return;
                            }
                            outline_panel.open_entry(&clicked_entry, cx);
                        })
                    })
                    .on_secondary_mouse_down(cx.listener(
                        move |outline_panel, event: &MouseDownEvent, cx| {
                            // Stop propagation to prevent the catch-all context menu for the project
                            // panel from being deployed.
                            cx.stop_propagation();
                            outline_panel.deploy_context_menu(
                                event.position,
                                rendered_entry.to_ref_entry(),
                                cx,
                            )
                        },
                    )),
            )
            .border_1()
            .border_r_2()
            .rounded_none()
            .hover(|style| {
                if is_active {
                    style
                } else {
                    let hover_color = cx.theme().colors().ghost_element_hover;
                    style.bg(hover_color).border_color(hover_color)
                }
            })
            .when(is_active && self.focus_handle.contains_focused(cx), |div| {
                div.border_color(Color::Selected.color(cx))
            })
    }

    fn entry_name(
        &self,
        worktree_id: &WorktreeId,
        entry: &Entry,
        cx: &ViewContext<OutlinePanel>,
    ) -> String {
        let name = match self.project.read(cx).worktree_for_id(*worktree_id, cx) {
            Some(worktree) => {
                let worktree = worktree.read(cx);
                match worktree.snapshot().root_entry() {
                    Some(root_entry) => {
                        if root_entry.id == entry.id {
                            file_name(worktree.abs_path().as_ref())
                        } else {
                            let path = worktree.absolutize(entry.path.as_ref()).ok();
                            let path = path.as_deref().unwrap_or_else(|| entry.path.as_ref());
                            file_name(path)
                        }
                    }
                    None => {
                        let path = worktree.absolutize(entry.path.as_ref()).ok();
                        let path = path.as_deref().unwrap_or_else(|| entry.path.as_ref());
                        file_name(path)
                    }
                }
            }
            None => file_name(entry.path.as_ref()),
        };
        name
    }

    fn update_fs_entries(
        &mut self,
        active_editor: &View<Editor>,
        new_entries: HashSet<ExcerptId>,
        new_selected_entry: Option<EntryOwned>,
        debounce: Option<Duration>,
        prefetch: bool,
        cx: &mut ViewContext<Self>,
    ) {
        if !self.active {
            return;
        }

        let auto_fold_dirs = OutlinePanelSettings::get_global(cx).auto_fold_dirs;
        let active_multi_buffer = active_editor.read(cx).buffer().clone();
        let multi_buffer_snapshot = active_multi_buffer.read(cx).snapshot(cx);
        let mut new_collapsed_entries = self.collapsed_entries.clone();
        let mut new_unfolded_dirs = self.unfolded_dirs.clone();
        let mut root_entries = HashSet::default();
        let mut new_excerpts = HashMap::<BufferId, HashMap<ExcerptId, Excerpt>>::default();
        let buffer_excerpts = multi_buffer_snapshot.excerpts().fold(
            HashMap::default(),
            |mut buffer_excerpts, (excerpt_id, buffer_snapshot, excerpt_range)| {
                let buffer_id = buffer_snapshot.remote_id();
                let file = File::from_dyn(buffer_snapshot.file());
                let entry_id = file.and_then(|file| file.project_entry_id(cx));
                let worktree = file.map(|file| file.worktree.read(cx).snapshot());
                let is_new =
                    new_entries.contains(&excerpt_id) || !self.excerpts.contains_key(&buffer_id);
                buffer_excerpts
                    .entry(buffer_id)
                    .or_insert_with(|| (is_new, Vec::new(), entry_id, worktree))
                    .1
                    .push(excerpt_id);

                let outlines = match self
                    .excerpts
                    .get(&buffer_id)
                    .and_then(|excerpts| excerpts.get(&excerpt_id))
                {
                    Some(old_excerpt) => match &old_excerpt.outlines {
                        ExcerptOutlines::Outlines(outlines) => {
                            ExcerptOutlines::Outlines(outlines.clone())
                        }
                        ExcerptOutlines::Invalidated(_) => ExcerptOutlines::NotFetched,
                        ExcerptOutlines::NotFetched => ExcerptOutlines::NotFetched,
                    },
                    None => {
                        new_collapsed_entries
                            .insert(CollapsedEntry::Excerpt(buffer_id, excerpt_id));
                        ExcerptOutlines::NotFetched
                    }
                };
                new_excerpts.entry(buffer_id).or_default().insert(
                    excerpt_id,
                    Excerpt {
                        range: excerpt_range,
                        outlines,
                    },
                );
                buffer_excerpts
            },
        );

        self.loading_outlines = true;
        self.update_task = cx.spawn(|outline_panel, mut cx| async move {
            if let Some(debounce) = debounce {
                cx.background_executor().timer(debounce).await;
            }
            let Some((new_collapsed_entries, new_unfolded_dirs, new_fs_entries, new_depth_map)) =
                cx.background_executor()
                    .spawn(async move {
                        let mut processed_external_buffers = HashSet::default();
                        let mut new_worktree_entries =
                            HashMap::<WorktreeId, (worktree::Snapshot, HashSet<Entry>)>::default();
                        let mut worktree_excerpts = HashMap::<
                            WorktreeId,
                            HashMap<ProjectEntryId, (BufferId, Vec<ExcerptId>)>,
                        >::default();
                        let mut external_excerpts = HashMap::default();

                        for (buffer_id, (is_new, excerpts, entry_id, worktree)) in buffer_excerpts {
                            if is_new {
                                match &worktree {
                                    Some(worktree) => {
                                        new_collapsed_entries
                                            .insert(CollapsedEntry::File(worktree.id(), buffer_id));
                                    }
                                    None => {
                                        new_collapsed_entries
                                            .insert(CollapsedEntry::ExternalFile(buffer_id));
                                    }
                                }

                                for excerpt_id in &excerpts {
                                    new_collapsed_entries
                                        .insert(CollapsedEntry::Excerpt(buffer_id, *excerpt_id));
                                }
                            }

                            if let Some(worktree) = worktree {
                                let worktree_id = worktree.id();
                                let unfolded_dirs =
                                    new_unfolded_dirs.entry(worktree_id).or_default();

                                match entry_id.and_then(|id| worktree.entry_for_id(id)).cloned() {
                                    Some(entry) => {
                                        let mut traversal = worktree.traverse_from_path(
                                            true,
                                            true,
                                            true,
                                            entry.path.as_ref(),
                                        );

                                        let mut entries_to_add = HashSet::default();
                                        worktree_excerpts
                                            .entry(worktree_id)
                                            .or_default()
                                            .insert(entry.id, (buffer_id, excerpts));
                                        let mut current_entry = entry;
                                        loop {
                                            if current_entry.is_dir() {
                                                let is_root =
                                                    worktree.root_entry().map(|entry| entry.id)
                                                        == Some(current_entry.id);
                                                if is_root {
                                                    root_entries.insert(current_entry.id);
                                                    if auto_fold_dirs {
                                                        unfolded_dirs.insert(current_entry.id);
                                                    }
                                                }

                                                if is_new {
                                                    new_collapsed_entries.remove(
                                                        &CollapsedEntry::Dir(
                                                            worktree_id,
                                                            current_entry.id,
                                                        ),
                                                    );
                                                } else if new_collapsed_entries.contains(
                                                    &CollapsedEntry::Dir(
                                                        worktree_id,
                                                        current_entry.id,
                                                    ),
                                                ) {
                                                    entries_to_add.clear();
                                                }
                                            }

                                            let new_entry_added =
                                                entries_to_add.insert(current_entry);
                                            if new_entry_added && traversal.back_to_parent() {
                                                if let Some(parent_entry) = traversal.entry() {
                                                    current_entry = parent_entry.clone();
                                                    continue;
                                                }
                                            }
                                            break;
                                        }
                                        new_worktree_entries
                                            .entry(worktree_id)
                                            .or_insert_with(|| {
                                                (worktree.clone(), HashSet::default())
                                            })
                                            .1
                                            .extend(entries_to_add);
                                    }
                                    None => {
                                        if processed_external_buffers.insert(buffer_id) {
                                            external_excerpts
                                                .entry(buffer_id)
                                                .or_insert_with(|| Vec::new())
                                                .extend(excerpts);
                                        }
                                    }
                                }
                            } else if processed_external_buffers.insert(buffer_id) {
                                external_excerpts
                                    .entry(buffer_id)
                                    .or_insert_with(|| Vec::new())
                                    .extend(excerpts);
                            }
                        }

                        #[derive(Clone, Copy, Default)]
                        struct Children {
                            files: usize,
                            dirs: usize,
                        }
                        let mut children_count =
                            HashMap::<WorktreeId, HashMap<PathBuf, Children>>::default();

                        let worktree_entries = new_worktree_entries
                            .into_iter()
                            .map(|(worktree_id, (worktree_snapshot, entries))| {
                                let mut entries = entries.into_iter().collect::<Vec<_>>();
                                // For a proper git status propagation, we have to keep the entries sorted lexicographically.
                                entries.sort_by(|a, b| a.path.as_ref().cmp(b.path.as_ref()));
                                worktree_snapshot.propagate_git_statuses(&mut entries);
                                project::sort_worktree_entries(&mut entries);
                                (worktree_id, entries)
                            })
                            .flat_map(|(worktree_id, entries)| {
                                {
                                    entries
                                        .into_iter()
                                        .filter_map(|entry| {
                                            if auto_fold_dirs {
                                                if let Some(parent) = entry.path.parent() {
                                                    let children = children_count
                                                        .entry(worktree_id)
                                                        .or_default()
                                                        .entry(parent.to_path_buf())
                                                        .or_default();
                                                    if entry.is_dir() {
                                                        children.dirs += 1;
                                                    } else {
                                                        children.files += 1;
                                                    }
                                                }
                                            }

                                            if entry.is_dir() {
                                                Some(FsEntry::Directory(worktree_id, entry))
                                            } else {
                                                let (buffer_id, excerpts) = worktree_excerpts
                                                    .get_mut(&worktree_id)
                                                    .and_then(|worktree_excerpts| {
                                                        worktree_excerpts.remove(&entry.id)
                                                    })?;
                                                Some(FsEntry::File(
                                                    worktree_id,
                                                    entry,
                                                    buffer_id,
                                                    excerpts,
                                                ))
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                }
                            })
                            .collect::<Vec<_>>();

                        let mut visited_dirs = Vec::new();
                        let mut new_depth_map = HashMap::default();
                        let new_visible_entries = external_excerpts
                            .into_iter()
                            .sorted_by_key(|(id, _)| *id)
                            .map(|(buffer_id, excerpts)| FsEntry::ExternalFile(buffer_id, excerpts))
                            .chain(worktree_entries)
                            .filter(|visible_item| {
                                match visible_item {
                                    FsEntry::Directory(worktree_id, dir_entry) => {
                                        let parent_id = back_to_common_visited_parent(
                                            &mut visited_dirs,
                                            worktree_id,
                                            dir_entry,
                                        );

                                        visited_dirs.push((dir_entry.id, dir_entry.path.clone()));
                                        let depth = if root_entries.contains(&dir_entry.id) {
                                            0
                                        } else if auto_fold_dirs {
                                            let (parent_folded, parent_depth) = match parent_id {
                                                Some((worktree_id, id)) => (
                                                    new_unfolded_dirs.get(&worktree_id).map_or(
                                                        true,
                                                        |unfolded_dirs| {
                                                            !unfolded_dirs.contains(&id)
                                                        },
                                                    ),
                                                    new_depth_map
                                                        .get(&(worktree_id, id))
                                                        .copied()
                                                        .unwrap_or(0),
                                                ),

                                                None => (false, 0),
                                            };

                                            let children = children_count
                                                .get(&worktree_id)
                                                .and_then(|children_count| {
                                                    children_count
                                                        .get(&dir_entry.path.to_path_buf())
                                                })
                                                .copied()
                                                .unwrap_or_default();
                                            let folded = if children.dirs > 1
                                                || (children.dirs == 1 && children.files > 0)
                                                || (children.dirs == 0
                                                    && visited_dirs
                                                        .last()
                                                        .map(|(parent_dir_id, _)| {
                                                            root_entries.contains(parent_dir_id)
                                                        })
                                                        .unwrap_or(true))
                                            {
                                                new_unfolded_dirs
                                                    .entry(*worktree_id)
                                                    .or_default()
                                                    .insert(dir_entry.id);
                                                false
                                            } else {
                                                new_unfolded_dirs.get(&worktree_id).map_or(
                                                    true,
                                                    |unfolded_dirs| {
                                                        !unfolded_dirs.contains(&dir_entry.id)
                                                    },
                                                )
                                            };

                                            if parent_folded && folded {
                                                parent_depth
                                            } else {
                                                parent_depth + 1
                                            }
                                        } else {
                                            parent_id
                                                .and_then(|(worktree_id, id)| {
                                                    new_depth_map.get(&(worktree_id, id)).copied()
                                                })
                                                .unwrap_or(0)
                                                + 1
                                        };
                                        new_depth_map.insert((*worktree_id, dir_entry.id), depth);
                                    }
                                    FsEntry::File(worktree_id, file_entry, ..) => {
                                        let parent_id = back_to_common_visited_parent(
                                            &mut visited_dirs,
                                            worktree_id,
                                            file_entry,
                                        );
                                        let depth = if root_entries.contains(&file_entry.id) {
                                            0
                                        } else {
                                            parent_id
                                                .and_then(|(worktree_id, id)| {
                                                    new_depth_map.get(&(worktree_id, id)).copied()
                                                })
                                                .unwrap_or(0)
                                                + 1
                                        };
                                        new_depth_map.insert((*worktree_id, file_entry.id), depth);
                                    }
                                    FsEntry::ExternalFile(..) => {
                                        visited_dirs.clear();
                                    }
                                }

                                true
                            })
                            .collect::<Vec<_>>();

                        anyhow::Ok((
                            new_collapsed_entries,
                            new_unfolded_dirs,
                            new_visible_entries,
                            new_depth_map,
                        ))
                    })
                    .await
                    .log_err()
            else {
                return;
            };

            outline_panel
                .update(&mut cx, |outline_panel, cx| {
                    outline_panel.loading_outlines = false;
                    outline_panel.excerpts = new_excerpts;
                    outline_panel.collapsed_entries = new_collapsed_entries;
                    outline_panel.unfolded_dirs = new_unfolded_dirs;
                    outline_panel.fs_entries = new_fs_entries;
                    outline_panel.fs_entries_depth = new_depth_map;
                    outline_panel.cached_entries_with_depth = None;
                    if new_selected_entry.is_some() {
                        outline_panel.selected_entry = new_selected_entry;
                    }
                    if prefetch {
                        let range = if outline_panel.last_visible_range.is_empty() {
                            0..(outline_panel.entries_with_depths(cx).len() / 4).min(50)
                        } else {
                            outline_panel.last_visible_range.clone()
                        };
                        outline_panel.fetch_outlines(&range, cx);
                    }

                    outline_panel.autoscroll(cx);
                    cx.notify();
                })
                .ok();
        });
    }

    fn replace_visible_entries(
        &mut self,
        new_active_editor: View<Editor>,
        cx: &mut ViewContext<Self>,
    ) {
        let new_selected_entry = self.location_for_editor_selection(&new_active_editor, cx);
        self.clear_previous();
        self.active_item = Some(ActiveItem {
            item_id: new_active_editor.item_id(),
            _editor_subscrpiption: subscribe_for_editor_events(&new_active_editor, cx),
            active_editor: new_active_editor.downgrade(),
        });
        let new_entries =
            HashSet::from_iter(new_active_editor.read(cx).buffer().read(cx).excerpt_ids());
        self.update_fs_entries(
            &new_active_editor,
            new_entries,
            new_selected_entry,
            None,
            true,
            cx,
        );
    }

    fn clear_previous(&mut self) {
        self.collapsed_entries.clear();
        self.unfolded_dirs.clear();
        self.last_visible_range = 0..0;
        self.selected_entry = None;
        self.update_task = Task::ready(());
        self.active_item = None;
        self.fs_entries.clear();
        self.fs_entries_depth.clear();
        self.outline_fetch_tasks.clear();
        self.excerpts.clear();
        self.cached_entries_with_depth = None;
    }

    fn location_for_editor_selection(
        &self,
        editor: &View<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> Option<EntryOwned> {
        let selection = editor
            .read(cx)
            .selections
            .newest::<language::Point>(cx)
            .head();
        let editor_snapshot = editor.update(cx, |editor, cx| editor.snapshot(cx));
        let multi_buffer = editor.read(cx).buffer();
        let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);
        let (excerpt_id, buffer, _) = editor
            .read(cx)
            .buffer()
            .read(cx)
            .excerpt_containing(selection, cx)?;
        let buffer_id = buffer.read(cx).remote_id();
        let selection_display_point = selection.to_display_point(&editor_snapshot);

        let excerpt_outlines = self
            .excerpts
            .get(&buffer_id)
            .and_then(|excerpts| excerpts.get(&excerpt_id))
            .into_iter()
            .flat_map(|excerpt| excerpt.iter_outlines())
            .flat_map(|outline| {
                let start = multi_buffer_snapshot
                    .anchor_in_excerpt(excerpt_id, outline.range.start)?
                    .to_display_point(&editor_snapshot);
                let end = multi_buffer_snapshot
                    .anchor_in_excerpt(excerpt_id, outline.range.end)?
                    .to_display_point(&editor_snapshot);
                Some((start..end, outline))
            })
            .collect::<Vec<_>>();

        let mut matching_outline_indices = Vec::new();
        let mut children = HashMap::default();
        let mut parents_stack = Vec::<(&Range<DisplayPoint>, &&Outline, usize)>::new();

        for (i, (outline_range, outline)) in excerpt_outlines.iter().enumerate() {
            if outline_range
                .to_inclusive()
                .contains(&selection_display_point)
            {
                matching_outline_indices.push(i);
            } else if (outline_range.start.row()..outline_range.end.row())
                .to_inclusive()
                .contains(&selection_display_point.row())
            {
                matching_outline_indices.push(i);
            }

            while let Some((parent_range, parent_outline, _)) = parents_stack.last() {
                if parent_outline.depth >= outline.depth
                    || !parent_range.contains(&outline_range.start)
                {
                    parents_stack.pop();
                } else {
                    break;
                }
            }
            if let Some((_, _, parent_index)) = parents_stack.last_mut() {
                children
                    .entry(*parent_index)
                    .or_insert_with(Vec::new)
                    .push(i);
            }
            parents_stack.push((outline_range, outline, i));
        }

        let outline_item = matching_outline_indices
            .into_iter()
            .flat_map(|i| Some((i, excerpt_outlines.get(i)?)))
            .filter(|(i, _)| {
                children
                    .get(i)
                    .map(|children| {
                        children.iter().all(|child_index| {
                            excerpt_outlines
                                .get(*child_index)
                                .map(|(child_range, _)| child_range.start > selection_display_point)
                                .unwrap_or(false)
                        })
                    })
                    .unwrap_or(true)
            })
            .min_by_key(|(_, (outline_range, outline))| {
                let distance_from_start = if outline_range.start > selection_display_point {
                    outline_range.start - selection_display_point
                } else {
                    selection_display_point - outline_range.start
                };
                let distance_from_end = if outline_range.end > selection_display_point {
                    outline_range.end - selection_display_point
                } else {
                    selection_display_point - outline_range.end
                };

                (
                    cmp::Reverse(outline.depth),
                    distance_from_start + distance_from_end,
                )
            })
            .map(|(_, (_, outline))| *outline)
            .cloned();

        let closest_container = match outline_item {
            Some(outline) => EntryOwned::Outline(buffer_id, excerpt_id, outline),
            None => self
                .cached_entries_with_depth
                .iter()
                .flatten()
                .rev()
                .find_map(|(_, entry)| match entry {
                    EntryOwned::Excerpt(entry_buffer_id, entry_excerpt_id, _) => {
                        if entry_buffer_id == &buffer_id && entry_excerpt_id == &excerpt_id {
                            Some(entry.clone())
                        } else {
                            None
                        }
                    }
                    EntryOwned::Entry(
                        FsEntry::ExternalFile(file_buffer_id, file_excerpts)
                        | FsEntry::File(_, _, file_buffer_id, file_excerpts),
                    ) => {
                        if file_buffer_id == &buffer_id && file_excerpts.contains(&excerpt_id) {
                            Some(entry.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                })?,
        };
        Some(closest_container)
    }

    fn fetch_outlines(&mut self, range: &Range<usize>, cx: &mut ViewContext<Self>) {
        let range_len = range.len();
        let half_range = range_len / 2;
        let entries = self.entries_with_depths(cx);
        let expanded_range =
            range.start.saturating_sub(half_range)..(range.end + half_range).min(entries.len());

        let excerpt_fetch_ranges = self.excerpt_fetch_ranges(expanded_range, cx);
        if excerpt_fetch_ranges.is_empty() {
            return;
        }

        let syntax_theme = cx.theme().syntax().clone();
        for (buffer_id, (buffer_snapshot, excerpt_ranges)) in excerpt_fetch_ranges {
            for (excerpt_id, excerpt_range) in excerpt_ranges {
                let syntax_theme = syntax_theme.clone();
                let buffer_snapshot = buffer_snapshot.clone();
                self.outline_fetch_tasks.insert(
                    (buffer_id, excerpt_id),
                    cx.spawn(|outline_panel, mut cx| async move {
                        let fetched_outlines = cx
                            .background_executor()
                            .spawn(async move {
                                buffer_snapshot
                                    .outline_items_containing(
                                        excerpt_range.context,
                                        false,
                                        Some(&syntax_theme),
                                    )
                                    .unwrap_or_default()
                            })
                            .await;
                        outline_panel
                            .update(&mut cx, |outline_panel, cx| {
                                if let Some(excerpt) = outline_panel
                                    .excerpts
                                    .entry(buffer_id)
                                    .or_default()
                                    .get_mut(&excerpt_id)
                                {
                                    excerpt.outlines = ExcerptOutlines::Outlines(fetched_outlines);
                                }
                                outline_panel.cached_entries_with_depth = None;
                                cx.notify();
                            })
                            .ok();
                    }),
                );
            }
        }
    }

    fn entries_with_depths(&mut self, cx: &AppContext) -> &[(usize, EntryOwned)] {
        let is_singleton = self.is_singleton_active(cx);
        self.cached_entries_with_depth.get_or_insert_with(|| {
            let auto_fold_dirs = OutlinePanelSettings::get_global(cx).auto_fold_dirs;
            let mut folded_dirs_entry = None::<(usize, WorktreeId, Vec<Entry>)>;
            let mut entries = Vec::new();

            for entry in &self.fs_entries {
                let depth = match entry {
                    FsEntry::Directory(worktree_id, dir_entry) => {
                        let depth = self
                            .fs_entries_depth
                            .get(&(*worktree_id, dir_entry.id))
                            .copied()
                            .unwrap_or(0);
                        if auto_fold_dirs {
                            let folded = self
                                .unfolded_dirs
                                .get(worktree_id)
                                .map_or(true, |unfolded_dirs| {
                                    !unfolded_dirs.contains(&dir_entry.id)
                                });
                            if folded {
                                if let Some((folded_depth, folded_worktree_id, mut folded_dirs)) =
                                    folded_dirs_entry.take()
                                {
                                    if worktree_id == &folded_worktree_id
                                        && dir_entry.path.parent()
                                            == folded_dirs.last().map(|entry| entry.path.as_ref())
                                    {
                                        folded_dirs.push(dir_entry.clone());
                                        folded_dirs_entry =
                                            Some((folded_depth, folded_worktree_id, folded_dirs))
                                    } else {
                                        entries.push((
                                            folded_depth,
                                            EntryOwned::FoldedDirs(folded_worktree_id, folded_dirs),
                                        ));
                                        folded_dirs_entry =
                                            Some((depth, *worktree_id, vec![dir_entry.clone()]))
                                    }
                                } else {
                                    folded_dirs_entry =
                                        Some((depth, *worktree_id, vec![dir_entry.clone()]))
                                }

                                continue;
                            }
                        }
                        depth
                    }
                    FsEntry::ExternalFile(..) => 0,
                    FsEntry::File(worktree_id, file_entry, ..) => self
                        .fs_entries_depth
                        .get(&(*worktree_id, file_entry.id))
                        .copied()
                        .unwrap_or(0),
                };
                if let Some((folded_depth, worktree_id, folded_dirs)) = folded_dirs_entry.take() {
                    entries.push((
                        folded_depth,
                        EntryOwned::FoldedDirs(worktree_id, folded_dirs),
                    ));
                }

                entries.push((depth, EntryOwned::Entry(entry.clone())));

                let excerpts_to_consider = match entry {
                    FsEntry::File(worktree_id, _, buffer_id, entry_excerpts) => {
                        if is_singleton
                            || !self
                                .collapsed_entries
                                .contains(&CollapsedEntry::File(*worktree_id, *buffer_id))
                        {
                            Some((*buffer_id, entry_excerpts))
                        } else {
                            None
                        }
                    }
                    FsEntry::ExternalFile(buffer_id, entry_excerpts) => {
                        if is_singleton
                            || !self
                                .collapsed_entries
                                .contains(&CollapsedEntry::ExternalFile(*buffer_id))
                        {
                            Some((*buffer_id, entry_excerpts))
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                if let Some((buffer_id, entry_excerpts)) = excerpts_to_consider {
                    if let Some(excerpts) = self.excerpts.get(&buffer_id) {
                        for &entry_excerpt in entry_excerpts {
                            let Some(excerpt) = excerpts.get(&entry_excerpt) else {
                                continue;
                            };
                            let excerpt_depth = depth + 1;
                            entries.push((
                                excerpt_depth,
                                EntryOwned::Excerpt(
                                    buffer_id,
                                    entry_excerpt,
                                    excerpt.range.clone(),
                                ),
                            ));

                            let mut outline_base_depth = excerpt_depth + 1;
                            if is_singleton {
                                outline_base_depth = 0;
                                entries.clear();
                            } else if self
                                .collapsed_entries
                                .contains(&CollapsedEntry::Excerpt(buffer_id, entry_excerpt))
                            {
                                continue;
                            }

                            for outline in excerpt.iter_outlines() {
                                entries.push((
                                    outline_base_depth + outline.depth,
                                    EntryOwned::Outline(buffer_id, entry_excerpt, outline.clone()),
                                ));
                            }
                            if is_singleton && entries.is_empty() {
                                entries.push((0, EntryOwned::Entry(entry.clone())));
                            }
                        }
                    }
                }
            }
            if let Some((folded_depth, worktree_id, folded_dirs)) = folded_dirs_entry.take() {
                entries.push((
                    folded_depth,
                    EntryOwned::FoldedDirs(worktree_id, folded_dirs),
                ));
            }
            entries
        })
    }

    fn is_singleton_active(&self, cx: &AppContext) -> bool {
        self.active_item
            .as_ref()
            .and_then(|active_item| {
                Some(
                    active_item
                        .active_editor
                        .upgrade()?
                        .read(cx)
                        .buffer()
                        .read(cx)
                        .is_singleton(),
                )
            })
            .unwrap_or(false)
    }

    fn invalidate_outlines(&mut self, ids: &[ExcerptId]) {
        self.outline_fetch_tasks.clear();
        let mut ids = ids.into_iter().collect::<HashSet<_>>();
        for excerpts in self.excerpts.values_mut() {
            ids.retain(|id| {
                if let Some(excerpt) = excerpts.get_mut(id) {
                    excerpt.invalidate_outlines();
                    false
                } else {
                    true
                }
            });
            if ids.is_empty() {
                break;
            }
        }
    }

    fn excerpt_fetch_ranges(
        &self,
        entry_range: Range<usize>,
        cx: &AppContext,
    ) -> HashMap<
        BufferId,
        (
            BufferSnapshot,
            HashMap<ExcerptId, ExcerptRange<language::Anchor>>,
        ),
    > {
        match self.cached_entries_with_depth.as_ref() {
            Some(entries) => entries.get(entry_range).into_iter().flatten().fold(
                HashMap::default(),
                |mut excerpts_to_fetch, (_, entry)| {
                    match entry {
                        EntryOwned::Entry(FsEntry::File(_, _, buffer_id, file_excerpts))
                        | EntryOwned::Entry(FsEntry::ExternalFile(buffer_id, file_excerpts)) => {
                            let excerpts = self.excerpts.get(&buffer_id);
                            for &file_excerpt in file_excerpts {
                                if let Some(excerpt) = excerpts
                                    .and_then(|excerpts| excerpts.get(&file_excerpt))
                                    .filter(|excerpt| excerpt.should_fetch_outlines())
                                {
                                    match excerpts_to_fetch.entry(*buffer_id) {
                                        hash_map::Entry::Occupied(mut o) => {
                                            o.get_mut()
                                                .1
                                                .insert(file_excerpt, excerpt.range.clone());
                                        }
                                        hash_map::Entry::Vacant(v) => {
                                            if let Some(buffer_snapshot) =
                                                self.buffer_snapshot_for_id(*buffer_id, cx)
                                            {
                                                v.insert((buffer_snapshot, HashMap::default()))
                                                    .1
                                                    .insert(file_excerpt, excerpt.range.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        EntryOwned::Excerpt(buffer_id, excerpt_id, _) => {
                            if let Some(excerpt) = self
                                .excerpts
                                .get(&buffer_id)
                                .and_then(|excerpts| excerpts.get(&excerpt_id))
                                .filter(|excerpt| excerpt.should_fetch_outlines())
                            {
                                match excerpts_to_fetch.entry(*buffer_id) {
                                    hash_map::Entry::Occupied(mut o) => {
                                        o.get_mut().1.insert(*excerpt_id, excerpt.range.clone());
                                    }
                                    hash_map::Entry::Vacant(v) => {
                                        if let Some(buffer_snapshot) =
                                            self.buffer_snapshot_for_id(*buffer_id, cx)
                                        {
                                            v.insert((buffer_snapshot, HashMap::default()))
                                                .1
                                                .insert(*excerpt_id, excerpt.range.clone());
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    excerpts_to_fetch
                },
            ),
            None => HashMap::default(),
        }
    }

    fn buffer_snapshot_for_id(
        &self,
        buffer_id: BufferId,
        cx: &AppContext,
    ) -> Option<BufferSnapshot> {
        let editor = self.active_item.as_ref()?.active_editor.upgrade()?;
        Some(
            editor
                .read(cx)
                .buffer()
                .read(cx)
                .buffer(buffer_id)?
                .read(cx)
                .snapshot(),
        )
    }

    fn abs_path(&self, entry: &EntryOwned, cx: &AppContext) -> Option<PathBuf> {
        match entry {
            EntryOwned::Entry(
                FsEntry::File(_, _, buffer_id, _) | FsEntry::ExternalFile(buffer_id, _),
            ) => self
                .buffer_snapshot_for_id(*buffer_id, cx)
                .and_then(|buffer_snapshot| {
                    let file = File::from_dyn(buffer_snapshot.file())?;
                    file.worktree.read(cx).absolutize(&file.path).ok()
                }),
            EntryOwned::Entry(FsEntry::Directory(worktree_id, entry)) => self
                .project
                .read(cx)
                .worktree_for_id(*worktree_id, cx)?
                .read(cx)
                .absolutize(&entry.path)
                .ok(),
            EntryOwned::FoldedDirs(worktree_id, dirs) => dirs.last().and_then(|entry| {
                self.project
                    .read(cx)
                    .worktree_for_id(*worktree_id, cx)
                    .and_then(|worktree| worktree.read(cx).absolutize(&entry.path).ok())
            }),
            EntryOwned::Excerpt(..) | EntryOwned::Outline(..) => None,
        }
    }

    fn relative_path(&self, entry: &FsEntry, cx: &AppContext) -> Option<PathBuf> {
        match entry {
            FsEntry::ExternalFile(buffer_id, _) => self
                .buffer_snapshot_for_id(*buffer_id, cx)
                .and_then(|buffer_snapshot| Some(buffer_snapshot.file()?.path().to_path_buf())),
            FsEntry::Directory(_, entry) => Some(entry.path.to_path_buf()),
            FsEntry::File(_, entry, ..) => Some(entry.path.to_path_buf()),
        }
    }
}

fn back_to_common_visited_parent(
    visited_dirs: &mut Vec<(ProjectEntryId, Arc<Path>)>,
    worktree_id: &WorktreeId,
    new_entry: &Entry,
) -> Option<(WorktreeId, ProjectEntryId)> {
    while let Some((visited_dir_id, visited_path)) = visited_dirs.last() {
        match new_entry.path.parent() {
            Some(parent_path) => {
                if parent_path == visited_path.as_ref() {
                    return Some((*worktree_id, *visited_dir_id));
                }
            }
            None => {
                break;
            }
        }
        visited_dirs.pop();
    }
    None
}

fn file_name(path: &Path) -> String {
    let mut current_path = path;
    loop {
        if let Some(file_name) = current_path.file_name() {
            return file_name.to_string_lossy().into_owned();
        }
        match current_path.parent() {
            Some(parent) => current_path = parent,
            None => return path.to_string_lossy().into_owned(),
        }
    }
}

impl Panel for OutlinePanel {
    fn persistent_name() -> &'static str {
        "Outline Panel"
    }

    fn position(&self, cx: &WindowContext) -> DockPosition {
        match OutlinePanelSettings::get_global(cx).dock {
            OutlinePanelDockPosition::Left => DockPosition::Left,
            OutlinePanelDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<OutlinePanelSettings>(
            self.fs.clone(),
            cx,
            move |settings| {
                let dock = match position {
                    DockPosition::Left | DockPosition::Bottom => OutlinePanelDockPosition::Left,
                    DockPosition::Right => OutlinePanelDockPosition::Right,
                };
                settings.dock = Some(dock);
            },
        );
    }

    fn size(&self, cx: &WindowContext) -> Pixels {
        self.width
            .unwrap_or_else(|| OutlinePanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, cx: &WindowContext) -> Option<IconName> {
        OutlinePanelSettings::get_global(cx)
            .button
            .then(|| IconName::ListTree)
    }

    fn icon_tooltip(&self, _: &WindowContext) -> Option<&'static str> {
        Some("Outline Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn starts_open(&self, _: &WindowContext) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        let old_active = self.active;
        self.active = active;
        if active && old_active != active {
            if let Some(active_editor) = self
                .active_item
                .as_ref()
                .and_then(|item| item.active_editor.upgrade())
            {
                if self.active_item.as_ref().map(|item| item.item_id)
                    == Some(active_editor.item_id())
                {
                    let new_selected_entry = self.location_for_editor_selection(&active_editor, cx);
                    self.update_fs_entries(
                        &active_editor,
                        HashSet::default(),
                        new_selected_entry,
                        None,
                        true,
                        cx,
                    )
                } else {
                    self.replace_visible_entries(active_editor, cx);
                }
            }
        }
        self.serialize(cx);
    }
}

impl FocusableView for OutlinePanel {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<Event> for OutlinePanel {}

impl EventEmitter<PanelEvent> for OutlinePanel {}

impl Render for OutlinePanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let project = self.project.read(cx);
        if self.fs_entries.is_empty() {
            let header = if self.loading_outlines {
                "Loading outlines"
            } else {
                "No outlines available"
            };
            v_flex()
                .id("empty-outline_panel")
                .justify_center()
                .size_full()
                .p_4()
                .track_focus(&self.focus_handle)
                .child(h_flex().justify_center().child(Label::new(header)))
                .child(
                    h_flex()
                        .pt(Spacing::Small.rems(cx))
                        .justify_center()
                        .child({
                            let keystroke = match self.position(cx) {
                                DockPosition::Left => {
                                    cx.keystroke_text_for(&workspace::ToggleLeftDock)
                                }
                                DockPosition::Bottom => {
                                    cx.keystroke_text_for(&workspace::ToggleBottomDock)
                                }
                                DockPosition::Right => {
                                    cx.keystroke_text_for(&workspace::ToggleRightDock)
                                }
                            };
                            Label::new(format!("Toggle this panel with {keystroke}",))
                        }),
                )
        } else {
            h_flex()
                .id("outline-panel")
                .size_full()
                .relative()
                .key_context(self.dispatch_context(cx))
                .on_action(cx.listener(Self::open))
                .on_action(cx.listener(Self::select_next))
                .on_action(cx.listener(Self::select_prev))
                .on_action(cx.listener(Self::select_first))
                .on_action(cx.listener(Self::select_last))
                .on_action(cx.listener(Self::select_parent))
                .on_action(cx.listener(Self::expand_selected_entry))
                .on_action(cx.listener(Self::collapse_selected_entry))
                .on_action(cx.listener(Self::collapse_all_entries))
                .on_action(cx.listener(Self::copy_path))
                .on_action(cx.listener(Self::copy_relative_path))
                .on_action(cx.listener(Self::unfold_directory))
                .on_action(cx.listener(Self::fold_directory))
                .when(project.is_local(), |el| {
                    el.on_action(cx.listener(Self::reveal_in_finder))
                        .on_action(cx.listener(Self::open_in_terminal))
                })
                .on_mouse_down(
                    MouseButton::Right,
                    cx.listener(move |outline_panel, event: &MouseDownEvent, cx| {
                        if let Some(entry) = outline_panel.selected_entry.clone() {
                            outline_panel.deploy_context_menu(
                                event.position,
                                entry.to_ref_entry(),
                                cx,
                            )
                        } else if let Some(entry) = outline_panel.fs_entries.first().cloned() {
                            outline_panel.deploy_context_menu(
                                event.position,
                                EntryRef::Entry(&entry),
                                cx,
                            )
                        }
                    }),
                )
                .track_focus(&self.focus_handle)
                .child({
                    let items_len = self.entries_with_depths(cx).len();
                    uniform_list(cx.view().clone(), "entries", items_len, {
                        move |outline_panel, range, cx| {
                            outline_panel.last_visible_range = range.clone();
                            outline_panel.fetch_outlines(&range, cx);
                            let entries = outline_panel.entries_with_depths(cx).get(range);
                            entries
                                .map(|entries| entries.to_vec())
                                .unwrap_or_default()
                                .into_iter()
                                .filter_map(|(depth, entry)| match entry {
                                    EntryOwned::Entry(entry) => {
                                        Some(outline_panel.render_entry(&entry, depth, cx))
                                    }
                                    EntryOwned::FoldedDirs(worktree_id, entries) => {
                                        Some(outline_panel.render_folded_dirs(
                                            worktree_id,
                                            &entries,
                                            depth,
                                            cx,
                                        ))
                                    }
                                    EntryOwned::Excerpt(buffer_id, excerpt_id, excerpt) => {
                                        outline_panel.render_excerpt(
                                            buffer_id, excerpt_id, &excerpt, depth, cx,
                                        )
                                    }
                                    EntryOwned::Outline(buffer_id, excerpt_id, outline) => {
                                        Some(outline_panel.render_outline(
                                            buffer_id, excerpt_id, &outline, depth, cx,
                                        ))
                                    }
                                })
                                .collect()
                        }
                    })
                    .size_full()
                    .track_scroll(self.scroll_handle.clone())
                })
                .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                    deferred(
                        anchored()
                            .position(*position)
                            .anchor(gpui::AnchorCorner::TopLeft)
                            .child(menu.clone()),
                    )
                    .with_priority(1)
                }))
        }
    }
}

fn subscribe_for_editor_events(
    editor: &View<Editor>,
    cx: &mut ViewContext<OutlinePanel>,
) -> Subscription {
    let debounce = Some(Duration::from_millis(UPDATE_DEBOUNCE_MILLIS));
    cx.subscribe(
        editor,
        move |outline_panel, editor, e: &EditorEvent, cx| match e {
            EditorEvent::SelectionsChanged { local: true } => {
                outline_panel.reveal_entry_for_selection(&editor, cx);
                cx.notify();
            }
            EditorEvent::ExcerptsAdded { excerpts, .. } => {
                outline_panel.update_fs_entries(
                    &editor,
                    excerpts.iter().map(|&(excerpt_id, _)| excerpt_id).collect(),
                    None,
                    debounce,
                    false,
                    cx,
                );
            }
            EditorEvent::ExcerptsRemoved { ids } => {
                let mut ids = ids.into_iter().collect::<HashSet<_>>();
                for excerpts in outline_panel.excerpts.values_mut() {
                    excerpts.retain(|excerpt_id, _| !ids.remove(excerpt_id));
                    if ids.is_empty() {
                        break;
                    }
                }
                outline_panel.update_fs_entries(
                    &editor,
                    HashSet::default(),
                    None,
                    debounce,
                    false,
                    cx,
                );
            }
            EditorEvent::ExcerptsExpanded { ids } => {
                outline_panel.invalidate_outlines(ids);
                outline_panel.update_fs_entries(
                    &editor,
                    HashSet::default(),
                    None,
                    debounce,
                    true,
                    cx,
                );
            }
            EditorEvent::ExcerptsEdited { ids } => {
                outline_panel.invalidate_outlines(ids);
                outline_panel.update_fs_entries(
                    &editor,
                    HashSet::default(),
                    None,
                    debounce,
                    true,
                    cx,
                );
            }
            EditorEvent::Reparsed(buffer_id) => {
                if let Some(excerpts) = outline_panel.excerpts.get_mut(buffer_id) {
                    for (_, excerpt) in excerpts {
                        excerpt.invalidate_outlines();
                    }
                }
                outline_panel.update_fs_entries(
                    &editor,
                    HashSet::default(),
                    None,
                    debounce,
                    true,
                    cx,
                );
            }
            _ => {}
        },
    )
}

fn empty_icon() -> AnyElement {
    h_flex()
        .size(IconSize::default().rems())
        .invisible()
        .flex_none()
        .into_any_element()
}
