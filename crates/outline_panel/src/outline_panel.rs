mod outline_panel_settings;

use std::{
    cmp,
    ffi::OsStr,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use collections::{hash_map, BTreeMap, BTreeSet, HashMap, HashSet};
use db::kvp::KEY_VALUE_STORE;
use editor::{
    items::entry_git_aware_label_color, scroll::ScrollAnchor, Editor, EditorEvent, MultiBuffer,
};
use file_icons::FileIcons;
use git::repository::GitFileStatus;
use gpui::{
    actions, anchored, deferred, div, px, uniform_list, Action, AnyElement, AppContext,
    AssetSource, AsyncWindowContext, ClipboardItem, DismissEvent, Div, EntityId, EventEmitter,
    FocusHandle, FocusableView, InteractiveElement, IntoElement, KeyContext, Model, MouseButton,
    MouseDownEvent, ParentElement, Pixels, Point, Render, Stateful, StatefulInteractiveElement,
    Styled, Subscription, Task, UniformListScrollHandle, View, ViewContext, VisualContext,
    WeakView, WindowContext,
};
use menu::{SelectFirst, SelectLast, SelectNext, SelectPrev};

use language::Buffer;
use outline_panel_settings::{OutlinePanelDockPosition, OutlinePanelSettings};
use project::{EntryKind, Fs, Item, Project};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use unicase::UniCase;
use util::{maybe, NumericPrefixWithSuffix, ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    item::ItemHandle,
    ui::{
        h_flex, v_flex, ActiveTheme, Color, ContextMenu, FluentBuilder, Icon, IconName, IconSize,
        Label, LabelCommon, ListItem, Selectable, Tooltip,
    },
    OpenInTerminal, Workspace,
};
use worktree::{Entry, ProjectEntryId, Worktree, WorktreeId};

actions!(
    outline_panel,
    [
        ExpandSelectedEntry,
        CollapseSelectedEntry,
        CollapseAllEntries,
        CopyPath,
        CopyRelativePath,
        RevealInFinder,
        Open,
        ToggleFocus,
        UnfoldDirectory,
        FoldDirectory,
        SelectParent,
    ]
);

const OUTLINE_PANEL_KEY: &str = "OutlinePanel";

pub struct OutlinePanel {
    fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    project: Model<Project>,
    scroll_handle: UniformListScrollHandle,
    context_menu: Option<(View<ContextMenu>, Point<Pixels>, Subscription)>,
    workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
    pending_serialization: Task<Option<()>>,
    visible_entries: Vec<(WorktreeId, Vec<Entry>)>,
    last_worktree_root_id: Option<ProjectEntryId>,
    expanded_dir_ids: HashMap<WorktreeId, BTreeSet<ProjectEntryId>>,
    unfolded_dir_ids: HashSet<ProjectEntryId>,
    // Currently selected entry in a file tree
    selection: Option<SelectedEntry>,
    displayed_item: Option<DisplayedActiveItem>,
    _subscriptions: Vec<Subscription>,
}

struct DisplayedActiveItem {
    item_id: EntityId,
    _editor_subscrpiption: Option<Subscription>,
    entries: BTreeMap<WorktreeId, BTreeSet<ProjectEntryId>>,
}

#[derive(Debug)]
pub enum Event {
    Focus,
}

#[derive(Serialize, Deserialize)]
struct SerializedOutlinePanel {
    width: Option<Pixels>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SelectedEntry {
    worktree_id: WorktreeId,
    entry_id: ProjectEntryId,
}

struct ActiveItem {
    item_id: EntityId,
    editor: View<Editor>,
    buffer: ActiveBuffer,
}

enum ActiveBuffer {
    SingletonBuffer(Model<Buffer>),
    MultiBuffer(Model<MultiBuffer>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntryDetails {
    filename: String,
    icon: Option<Arc<str>>,
    path: Arc<Path>,
    depth: usize,
    kind: EntryKind,
    is_ignored: bool,
    is_expanded: bool,
    is_selected: bool,
    git_status: Option<GitFileStatus>,
    is_private: bool,
    worktree_id: WorktreeId,
    canonical_path: Option<PathBuf>,
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
                        if let Some(new_active_item) = outline_panel.active_item(cx) {
                            let mut new_entries = new_active_item
                                .editor
                                .project_entry_ids(cx)
                                .into_iter()
                                .filter_map(|entry_id| {
                                    let worktree_id = workspace
                                        .read(cx)
                                        .project()
                                        .read(cx)
                                        .worktree_for_entry(entry_id, cx)?
                                        .read(cx)
                                        .id();
                                    Some((worktree_id, entry_id))
                                })
                                .fold(
                                    BTreeMap::<WorktreeId, BTreeSet<ProjectEntryId>>::new(),
                                    |mut entries, (worktree_id, entry_id)| {
                                        entries.entry(worktree_id).or_default().insert(entry_id);
                                        entries
                                    },
                                );
                            let mut added_entries = HashSet::default();
                            if let Some(displayed_item) = &mut outline_panel.displayed_item {
                                if displayed_item.item_id != new_active_item.item_id {
                                    added_entries.extend(new_entries.values().flatten().copied());
                                    outline_panel.displayed_item = Some(DisplayedActiveItem {
                                        item_id: new_active_item.item_id,
                                        _editor_subscrpiption: subscribe_for_editor_changes(
                                            &new_active_item.editor,
                                            cx,
                                        ),
                                        entries: new_entries,
                                    });
                                } else {
                                    displayed_item.entries.retain(
                                        |old_workspace_id, old_entries| match new_entries
                                            .remove(old_workspace_id)
                                        {
                                            Some(mut new_entries) => {
                                                old_entries.retain(|old_entry| {
                                                    new_entries.remove(old_entry)
                                                });
                                                added_entries.extend(new_entries.iter().copied());
                                                old_entries.extend(new_entries);
                                                !old_entries.is_empty()
                                            }
                                            None => false,
                                        },
                                    );

                                    for (new_workspace_id, new_entries) in new_entries {
                                        added_entries.extend(new_entries.iter().copied());
                                        displayed_item
                                            .entries
                                            .insert(new_workspace_id, new_entries);
                                    }
                                }
                            } else {
                                added_entries.extend(new_entries.values().flatten().copied());
                                outline_panel.displayed_item = Some(DisplayedActiveItem {
                                    item_id: new_active_item.item_id,
                                    _editor_subscrpiption: subscribe_for_editor_changes(
                                        &new_active_item.editor,
                                        cx,
                                    ),
                                    entries: new_entries,
                                });
                            }
                            if !added_entries.is_empty() {
                                outline_panel.update_visible_entries(added_entries, None, cx);
                                cx.notify();
                            }
                        } else {
                            outline_panel.displayed_item = None;
                            outline_panel.update_visible_entries(HashSet::default(), None, cx);
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
                project: project.clone(),
                fs: workspace.app_state().fs.clone(),
                scroll_handle: UniformListScrollHandle::new(),
                focus_handle,
                visible_entries: Default::default(),
                last_worktree_root_id: None,
                expanded_dir_ids: HashMap::default(),
                unfolded_dir_ids: Default::default(),
                selection: None,
                context_menu: None,
                workspace: workspace.weak_handle(),
                width: None,
                pending_serialization: Task::ready(None),
                displayed_item: None,
                _subscriptions: vec![
                    settings_subscription,
                    icons_subscription,
                    focus_subscription,
                    workspace_subscription,
                ],
            };
            outline_panel.update_visible_entries(HashSet::default(), None, cx);
            outline_panel
        });

        outline_panel
    }

    fn active_item(&self, cx: &WindowContext) -> Option<ActiveItem> {
        let editor = self
            .workspace
            .upgrade()?
            .read(cx)
            .active_item(cx)?
            .act_as::<Editor>(cx)?;
        let item_id = editor.item_id();
        let multi_buffer = editor.read(cx).buffer().clone();
        let buffer = match multi_buffer.read(cx).as_singleton() {
            Some(singleton_buffer) => ActiveBuffer::SingletonBuffer(singleton_buffer),
            None => ActiveBuffer::MultiBuffer(multi_buffer),
        };
        Some(ActiveItem {
            item_id,
            editor,
            buffer,
        })
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background_executor().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        OUTLINE_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedOutlinePanel { width })?,
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

    pub fn selected_entry<'a>(
        &self,
        cx: &'a AppContext,
    ) -> Option<(&'a Worktree, &'a project::Entry)> {
        let (worktree, entry) = self.selected_entry_handle(cx)?;
        Some((worktree.read(cx), entry))
    }

    fn selected_entry_handle<'a>(
        &self,
        cx: &'a AppContext,
    ) -> Option<(Model<Worktree>, &'a project::Entry)> {
        let selection = self.selection?;
        let project = self.project.read(cx);
        let worktree = project.worktree_for_id(selection.worktree_id, cx)?;
        let entry = worktree.read(cx).entry_for_id(selection.entry_id)?;
        Some((worktree, entry))
    }

    fn unfold_directory(&mut self, _: &UnfoldDirectory, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            self.unfolded_dir_ids.insert(entry.id);

            let snapshot = worktree.snapshot();
            let mut parent_path = entry.path.parent();
            while let Some(path) = parent_path {
                if let Some(parent_entry) = worktree.entry_for_path(path) {
                    let mut children_iter = snapshot.child_entries(path);

                    if children_iter.by_ref().take(2).count() > 1 {
                        break;
                    }

                    self.unfolded_dir_ids.insert(parent_entry.id);
                    parent_path = path.parent();
                } else {
                    break;
                }
            }

            self.update_visible_entries(HashSet::default(), None, cx);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn fold_directory(&mut self, _: &FoldDirectory, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            self.unfolded_dir_ids.remove(&entry.id);

            let snapshot = worktree.snapshot();
            let mut path = &*entry.path;
            loop {
                let mut child_entries_iter = snapshot.child_entries(path);
                if let Some(child) = child_entries_iter.next() {
                    if child_entries_iter.next().is_none() && child.is_dir() {
                        self.unfolded_dir_ids.remove(&child.id);
                        path = &*child.path;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            self.update_visible_entries(HashSet::default(), None, cx);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if let Some(selection) = self.selection {
            let (mut worktree_ix, mut entry_ix, _) =
                self.index_for_selection(selection).unwrap_or_default();
            if let Some((_, worktree_entries)) = self.visible_entries.get(worktree_ix) {
                if entry_ix + 1 < worktree_entries.len() {
                    entry_ix += 1;
                } else {
                    worktree_ix += 1;
                    entry_ix = 0;
                }
            }

            if let Some((worktree_id, worktree_entries)) = self.visible_entries.get(worktree_ix) {
                if let Some(entry) = worktree_entries.get(entry_ix) {
                    let selection = SelectedEntry {
                        worktree_id: *worktree_id,
                        entry_id: entry.id,
                    };
                    self.selection = Some(selection);
                    self.autoscroll(cx);
                    cx.notify();
                }
            }
        } else {
            self.select_first(&SelectFirst {}, cx);
        }
    }

    fn select_parent(&mut self, _: &SelectParent, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            if let Some(parent) = entry.path.parent() {
                if let Some(parent_entry) = worktree.entry_for_path(parent) {
                    self.selection = Some(SelectedEntry {
                        worktree_id: worktree.id(),
                        entry_id: parent_entry.id,
                    });
                    self.autoscroll(cx);
                    cx.notify();
                }
            }
        } else {
            self.select_first(&SelectFirst {}, cx);
        }
    }

    fn select_first(&mut self, _: &SelectFirst, cx: &mut ViewContext<Self>) {
        let worktree = self
            .visible_entries
            .first()
            .and_then(|(worktree_id, _)| self.project.read(cx).worktree_for_id(*worktree_id, cx));
        if let Some(worktree) = worktree {
            let worktree = worktree.read(cx);
            let worktree_id = worktree.id();
            if let Some(root_entry) = worktree.root_entry() {
                let selection = SelectedEntry {
                    worktree_id,
                    entry_id: root_entry.id,
                };
                self.selection = Some(selection);
                self.autoscroll(cx);
                cx.notify();
            }
        }
    }

    fn select_last(&mut self, _: &SelectLast, cx: &mut ViewContext<Self>) {
        let worktree = self
            .visible_entries
            .last()
            .and_then(|(worktree_id, _)| self.project.read(cx).worktree_for_id(*worktree_id, cx));
        if let Some(worktree) = worktree {
            let worktree = worktree.read(cx);
            let worktree_id = worktree.id();
            if let Some(last_entry) = worktree.entries(true).last() {
                self.selection = Some(SelectedEntry {
                    worktree_id,
                    entry_id: last_entry.id,
                });
                self.autoscroll(cx);
                cx.notify();
            }
        }
    }

    fn autoscroll(&mut self, cx: &mut ViewContext<Self>) {
        if let Some((_, _, index)) = self.selection.and_then(|s| self.index_for_selection(s)) {
            self.scroll_handle.scroll_to_item(index);
            cx.notify();
        }
    }

    fn index_for_selection(&self, selection: SelectedEntry) -> Option<(usize, usize, usize)> {
        let mut entry_index = 0;
        let mut visible_entries_index = 0;
        for (worktree_index, (worktree_id, worktree_entries)) in
            self.visible_entries.iter().enumerate()
        {
            if *worktree_id == selection.worktree_id {
                for entry in worktree_entries {
                    if entry.id == selection.entry_id {
                        return Some((worktree_index, entry_index, visible_entries_index));
                    } else {
                        visible_entries_index += 1;
                        entry_index += 1;
                    }
                }
                break;
            } else {
                visible_entries_index += worktree_entries.len();
            }
        }
        None
    }

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        if !self.focus_handle.contains_focused(cx) {
            cx.emit(Event::Focus);
        }
    }

    fn deploy_context_menu(
        &mut self,
        position: Point<Pixels>,
        entry_id: ProjectEntryId,
        cx: &mut ViewContext<Self>,
    ) {
        let project = self.project.read(cx);

        let worktree_id = if let Some(id) = project.worktree_id_for_entry(entry_id, cx) {
            id
        } else {
            return;
        };

        self.selection = Some(SelectedEntry {
            worktree_id,
            entry_id,
        });

        if let Some((worktree, entry)) = self.selected_entry(cx) {
            let auto_fold_dirs = OutlinePanelSettings::get_global(cx).auto_fold_dirs;
            let is_root = Some(entry) == worktree.root_entry();
            let is_foldable = auto_fold_dirs && self.is_foldable(entry, worktree);
            let is_unfoldable = auto_fold_dirs && self.is_unfoldable(entry, worktree);
            let is_local = project.is_local();
            let is_read_only = project.is_read_only();

            let context_menu = ContextMenu::build(cx, |menu, _| {
                menu.context(self.focus_handle.clone()).when_else(
                    is_read_only,
                    |menu| menu.action("Copy Relative Path", Box::new(CopyRelativePath)),
                    |menu| {
                        menu.action("Reveal in Finder", Box::new(RevealInFinder))
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
                            .when(is_local & is_root, |menu| {
                                menu.separator()
                                    .action("Collapse All", Box::new(CollapseAllEntries))
                            })
                    },
                )
            });

            cx.focus_view(&context_menu);
            let subscription =
                cx.subscribe(&context_menu, |outline_panel, _, _: &DismissEvent, cx| {
                    outline_panel.context_menu.take();
                    cx.notify();
                });
            self.context_menu = Some((context_menu, position, subscription));
        }

        cx.notify();
    }

    fn is_unfoldable(&self, entry: &Entry, worktree: &Worktree) -> bool {
        if !entry.is_dir() || self.unfolded_dir_ids.contains(&entry.id) {
            return false;
        }

        if let Some(parent_path) = entry.path.parent() {
            let snapshot = worktree.snapshot();
            let mut child_entries = snapshot.child_entries(&parent_path);
            if let Some(child) = child_entries.next() {
                if child_entries.next().is_none() {
                    return child.kind.is_dir();
                }
            }
        };
        false
    }

    fn is_foldable(&self, entry: &Entry, worktree: &Worktree) -> bool {
        if entry.is_dir() {
            let snapshot = worktree.snapshot();

            let mut child_entries = snapshot.child_entries(&entry.path);
            if let Some(child) = child_entries.next() {
                if child_entries.next().is_none() {
                    return child.kind.is_dir();
                }
            }
        }
        false
    }

    fn expand_selected_entry(&mut self, _: &ExpandSelectedEntry, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            if entry.is_dir() {
                let worktree_id = worktree.id();
                let entry_id = entry.id;
                let expanded_dir_ids =
                    if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(&worktree_id) {
                        expanded_dir_ids
                    } else {
                        return;
                    };

                if expanded_dir_ids.insert(entry_id) {
                    self.project.update(cx, |project, cx| {
                        project.expand_entry(worktree_id, entry_id, cx);
                    });
                    self.update_visible_entries(HashSet::default(), None, cx);
                    cx.notify()
                } else {
                    self.select_next(&SelectNext, cx)
                }
            }
        }
    }

    fn collapse_selected_entry(&mut self, _: &CollapseSelectedEntry, cx: &mut ViewContext<Self>) {
        if let Some((worktree, mut entry)) = self.selected_entry(cx) {
            let worktree_id = worktree.id();
            let expanded_dir_ids =
                if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(&worktree_id) {
                    expanded_dir_ids
                } else {
                    return;
                };

            loop {
                let entry_id = entry.id;
                if expanded_dir_ids.remove(&entry_id) {
                    self.update_visible_entries(
                        HashSet::default(),
                        Some((worktree_id, entry_id)),
                        cx,
                    );
                    cx.notify();
                    break;
                } else if let Some(parent_entry) =
                    entry.path.parent().and_then(|p| worktree.entry_for_path(p))
                {
                    entry = parent_entry;
                } else {
                    break;
                }
            }
        }
    }

    pub fn collapse_all_entries(&mut self, _: &CollapseAllEntries, cx: &mut ViewContext<Self>) {
        // By keeping entries for fully collapsed worktrees, we avoid expanding them within update_visible_entries
        // (which is it's default behaviour when there's no entry for a worktree in expanded_dir_ids).
        self.expanded_dir_ids
            .retain(|_, expanded_entries| expanded_entries.is_empty());
        self.update_visible_entries(HashSet::default(), None, cx);
        cx.notify();
    }

    fn toggle_expanded(&mut self, entry_id: ProjectEntryId, cx: &mut ViewContext<Self>) {
        if let Some(worktree_id) = self.project.read(cx).worktree_id_for_entry(entry_id, cx) {
            if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(&worktree_id) {
                self.project.update(cx, |project, cx| {
                    if !expanded_dir_ids.remove(&entry_id) {
                        project.expand_entry(worktree_id, entry_id, cx);
                        expanded_dir_ids.insert(entry_id);
                    }
                });
                self.update_visible_entries(HashSet::default(), Some((worktree_id, entry_id)), cx);
                cx.focus(&self.focus_handle);
                cx.notify();
            }
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if let Some(selection) = self.selection {
            let (mut worktree_ix, mut entry_ix, _) =
                self.index_for_selection(selection).unwrap_or_default();
            if entry_ix > 0 {
                entry_ix -= 1;
            } else if worktree_ix > 0 {
                worktree_ix -= 1;
                entry_ix = self.visible_entries[worktree_ix].1.len() - 1;
            } else {
                return;
            }

            let (worktree_id, worktree_entries) = &self.visible_entries[worktree_ix];
            let selection = SelectedEntry {
                worktree_id: *worktree_id,
                entry_id: worktree_entries[entry_ix].id,
            };
            self.selection = Some(selection);
            self.autoscroll(cx);
            cx.notify();
        } else {
            self.select_first(&SelectFirst {}, cx);
        }
    }

    fn copy_path(&mut self, _: &CopyPath, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            cx.write_to_clipboard(ClipboardItem::new(
                worktree
                    .abs_path()
                    .join(&entry.path)
                    .to_string_lossy()
                    .to_string(),
            ));
        }
    }

    fn copy_relative_path(&mut self, _: &CopyRelativePath, cx: &mut ViewContext<Self>) {
        if let Some((_, entry)) = self.selected_entry(cx) {
            cx.write_to_clipboard(ClipboardItem::new(entry.path.to_string_lossy().to_string()));
        }
    }

    fn reveal_in_finder(&mut self, _: &RevealInFinder, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            cx.reveal_path(&worktree.abs_path().join(&entry.path));
        }
    }

    fn open_in_terminal(&mut self, _: &OpenInTerminal, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            let abs_path = worktree.abs_path().join(&entry.path);
            let working_directory = if entry.is_dir() {
                Some(abs_path)
            } else {
                if entry.is_symlink {
                    abs_path.canonicalize().ok()
                } else {
                    Some(abs_path)
                }
                .and_then(|path| Some(path.parent()?.to_path_buf()))
            };
            if let Some(working_directory) = working_directory {
                cx.dispatch_action(workspace::OpenTerminal { working_directory }.boxed_clone())
            }
        }
    }

    fn for_each_visible_entry(
        &self,
        range: Range<usize>,
        cx: &mut ViewContext<Self>,
        mut callback: impl FnMut(ProjectEntryId, EntryDetails, &mut ViewContext<Self>),
    ) {
        let mut ix = 0;
        for (worktree_id, visible_worktree_entries) in &self.visible_entries {
            if ix >= range.end {
                return;
            }

            if ix + visible_worktree_entries.len() <= range.start {
                ix += visible_worktree_entries.len();
                continue;
            }

            let end_ix = range.end.min(ix + visible_worktree_entries.len());
            let (git_status_setting, show_file_icons, show_folder_icons) = {
                let settings = OutlinePanelSettings::get_global(cx);
                (
                    settings.git_status,
                    settings.file_icons,
                    settings.folder_icons,
                )
            };
            if let Some(worktree) = self.project.read(cx).worktree_for_id(*worktree_id, cx) {
                let snapshot = worktree.read(cx).snapshot();
                let root_name = OsStr::new(snapshot.root_name());
                let expanded_entry_ids = self.expanded_dir_ids.get(&snapshot.id());

                let entry_range = range.start.saturating_sub(ix)..end_ix - ix;
                for entry in visible_worktree_entries[entry_range].iter() {
                    let status = git_status_setting.then(|| entry.git_status).flatten();
                    let is_expanded =
                        expanded_entry_ids.map_or(false, |ids| ids.contains(&entry.id));
                    let icon = match entry.kind {
                        EntryKind::File(_) => {
                            if show_file_icons {
                                FileIcons::get_icon(&entry.path, cx)
                            } else {
                                None
                            }
                        }
                        _ => {
                            if show_folder_icons {
                                FileIcons::get_folder_icon(is_expanded, cx)
                            } else {
                                FileIcons::get_chevron_icon(is_expanded, cx)
                            }
                        }
                    };

                    let (depth, difference) =
                        Self::calculate_depth_and_difference(entry, visible_worktree_entries);

                    let filename = match difference {
                        diff if diff > 1 => entry
                            .path
                            .iter()
                            .skip(entry.path.components().count() - diff)
                            .collect::<PathBuf>()
                            .to_str()
                            .unwrap_or_default()
                            .to_string(),
                        _ => entry
                            .path
                            .file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                            .unwrap_or_else(|| root_name.to_string_lossy().to_string()),
                    };
                    let selection = SelectedEntry {
                        worktree_id: snapshot.id(),
                        entry_id: entry.id,
                    };
                    let details = EntryDetails {
                        filename,
                        icon,
                        path: entry.path.clone(),
                        depth,
                        kind: entry.kind,
                        is_ignored: entry.is_ignored,
                        is_expanded,
                        is_selected: self.selection == Some(selection),
                        git_status: status,
                        is_private: entry.is_private,
                        worktree_id: *worktree_id,
                        canonical_path: entry.canonical_path.clone(),
                    };

                    callback(entry.id, details, cx);
                }
            }
            ix = end_ix;
        }
    }

    fn calculate_depth_and_difference(
        entry: &Entry,
        visible_worktree_entries: &[Entry],
    ) -> (usize, usize) {
        let visible_worktree_paths: HashSet<Arc<Path>> = visible_worktree_entries
            .iter()
            .map(|e| e.path.clone())
            .collect();

        let (depth, difference) = entry
            .path
            .ancestors()
            .skip(1) // Skip the entry itself
            .find_map(|ancestor| {
                if visible_worktree_paths.contains(ancestor) {
                    let parent_entry = visible_worktree_entries
                        .iter()
                        .find(|&e| &*e.path == ancestor)
                        .unwrap();

                    let entry_path_components_count = entry.path.components().count();
                    let parent_path_components_count = parent_entry.path.components().count();
                    let difference = entry_path_components_count - parent_path_components_count;
                    let depth = parent_entry
                        .path
                        .ancestors()
                        .skip(1)
                        .filter(|ancestor| visible_worktree_paths.contains(*ancestor))
                        .count();
                    Some((depth + 1, difference))
                } else {
                    None
                }
            })
            .unwrap_or((0, 0));

        (depth, difference)
    }

    fn reveal_entry(
        &mut self,
        project: Model<Project>,
        entry_id: ProjectEntryId,
        skip_ignored: bool,
        cx: &mut ViewContext<'_, Self>,
    ) {
        if let Some(worktree) = project.read(cx).worktree_for_entry(entry_id, cx) {
            let worktree = worktree.read(cx);
            if skip_ignored
                && worktree
                    .entry_for_id(entry_id)
                    .map_or(true, |entry| entry.is_ignored)
            {
                return;
            }

            let worktree_id = worktree.id();
            if self.selection
                == Some(SelectedEntry {
                    worktree_id,
                    entry_id,
                })
            {
                return;
            }

            self.expand_entry(worktree_id, entry_id, cx);
            self.update_visible_entries(HashSet::default(), Some((worktree_id, entry_id)), cx);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn expand_entry(
        &mut self,
        worktree_id: WorktreeId,
        entry_id: ProjectEntryId,
        cx: &mut ViewContext<Self>,
    ) {
        self.project.update(cx, |project, cx| {
            if let Some((worktree, expanded_dir_ids)) = project
                .worktree_for_id(worktree_id, cx)
                .zip(self.expanded_dir_ids.get_mut(&worktree_id))
            {
                project.expand_entry(worktree_id, entry_id, cx);
                let worktree = worktree.read(cx);

                if let Some(mut entry) = worktree.entry_for_id(entry_id) {
                    loop {
                        expanded_dir_ids.insert(entry.id);
                        if let Some(parent_entry) =
                            entry.path.parent().and_then(|p| worktree.entry_for_path(p))
                        {
                            entry = parent_entry;
                        } else {
                            break;
                        }
                    }
                }
            }
        });
    }

    fn render_entry(
        &self,
        entry_id: ProjectEntryId,
        details: EntryDetails,
        cx: &mut ViewContext<Self>,
    ) -> Stateful<Div> {
        let kind = details.kind;
        let settings = OutlinePanelSettings::get_global(cx);
        let is_active = self
            .selection
            .map_or(false, |selection| selection.entry_id == entry_id);
        let filename_text_color =
            entry_git_aware_label_color(details.git_status, details.is_ignored, false);
        let file_name = details.filename.clone();
        let icon = details.icon.clone();

        let canonical_path = details
            .canonical_path
            .as_ref()
            .map(|f| f.to_string_lossy().to_string());

        let depth = details.depth;
        let worktree_id = details.worktree_id;

        div()
            .id(entry_id.to_proto() as usize)
            .child(
                ListItem::new(entry_id.to_proto() as usize)
                    .indent_level(depth)
                    .indent_step_size(px(settings.indent_size))
                    .selected(is_active)
                    .when_some(canonical_path, |this, path| {
                        this.end_slot::<AnyElement>(
                            div()
                                .id("symlink_icon")
                                .tooltip(move |cx| {
                                    Tooltip::text(format!("{path} • Symbolic Link"), cx)
                                })
                                .child(
                                    Icon::new(IconName::ArrowUpRight)
                                        .size(IconSize::Indicator)
                                        .color(filename_text_color),
                                )
                                .into_any_element(),
                        )
                    })
                    .child(if let Some(icon) = &icon {
                        h_flex().child(Icon::from_path(icon.to_string()).color(filename_text_color))
                    } else {
                        h_flex()
                            .size(IconSize::default().rems())
                            .invisible()
                            .flex_none()
                    })
                    .child(
                        h_flex()
                            .h_6()
                            .child(
                                Label::new(file_name)
                                    .single_line()
                                    .color(filename_text_color),
                            )
                            .ml_1(),
                    )
                    .on_click(
                        cx.listener(move |outline_panel, event: &gpui::ClickEvent, cx| {
                            if event.down.button == MouseButton::Right || event.down.first_mouse {
                                return;
                            }
                            if let Some(selection) = outline_panel
                                .selection
                                .filter(|_| event.down.modifiers.shift)
                            {
                                let current_selection =
                                    outline_panel.index_for_selection(selection);
                                let target_selection =
                                    outline_panel.index_for_selection(SelectedEntry {
                                        entry_id,
                                        worktree_id,
                                    });
                                if let Some(((_, _, source_index), (_, _, target_index))) =
                                    current_selection.zip(target_selection)
                                {
                                    let range_start = source_index.min(target_index);
                                    let range_end = source_index.max(target_index) + 1; // Make the range inclusive.
                                    let mut new_selections = BTreeSet::new();
                                    outline_panel.for_each_visible_entry(
                                        range_start..range_end,
                                        cx,
                                        |entry_id, details, _| {
                                            new_selections.insert(SelectedEntry {
                                                entry_id,
                                                worktree_id: details.worktree_id,
                                            });
                                        },
                                    );

                                    outline_panel.selection = Some(SelectedEntry {
                                        entry_id,
                                        worktree_id,
                                    });
                                }
                            } else if kind.is_dir() {
                                outline_panel.toggle_expanded(entry_id, cx);
                            } else if outline_panel
                                .displayed_item
                                .as_ref()
                                .filter(|item| !item.entries.is_empty())
                                .is_some()
                            {
                                if let Some(active_item) = outline_panel.active_item(cx) {
                                    match active_item.buffer {
                                        ActiveBuffer::SingletonBuffer(_active_singleton_buffer) => {
                                            // TODO kb add outline entries and navigate to clicked outline's anchor in the buffer
                                        }
                                        ActiveBuffer::MultiBuffer(active_multi_buffer) => {
                                            let multi_buffer_snapshot =
                                                active_multi_buffer.read(cx).snapshot(cx);
                                            let scroll_target = outline_panel
                                                .project
                                                .update(cx, |project, cx| {
                                                    project.path_for_entry(entry_id, cx).and_then(
                                                        |path| project.get_open_buffer(&path, cx),
                                                    )
                                                })
                                                .map(|buffer| {
                                                    active_multi_buffer
                                                        .read(cx)
                                                        .excerpts_for_buffer(&buffer, cx)
                                                })
                                                .and_then(|excerpts| {
                                                    let (excerpt_id, excerpt_range) =
                                                        excerpts.first()?;
                                                    multi_buffer_snapshot.anchor_in_excerpt(
                                                        *excerpt_id,
                                                        excerpt_range.context.start,
                                                    )
                                                });
                                            if let Some(anchor) = scroll_target {
                                                outline_panel.selection = Some(SelectedEntry {
                                                    worktree_id,
                                                    entry_id,
                                                });
                                                active_item.editor.update(cx, |editor, cx| {
                                                    editor.set_scroll_anchor(
                                                        ScrollAnchor {
                                                            offset: Point::new(
                                                                0.0,
                                                                -(editor.file_header_size() as f32),
                                                            ),
                                                            anchor,
                                                        },
                                                        cx,
                                                    );
                                                })
                                            }
                                        }
                                    }
                                }
                            }
                        }),
                    )
                    .on_secondary_mouse_down(cx.listener(
                        move |this, event: &MouseDownEvent, cx| {
                            // Stop propagation to prevent the catch-all context menu for the project
                            // panel from being deployed.
                            cx.stop_propagation();
                            this.deploy_context_menu(event.position, entry_id, cx);
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
            .when(
                is_active && self.focus_handle.contains_focused(cx),
                |this| this.border_color(Color::Selected.color(cx)),
            )
    }

    fn update_visible_entries(
        &mut self,
        new_entries: HashSet<ProjectEntryId>,
        new_selected_entry: Option<(WorktreeId, ProjectEntryId)>,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(displayed_entries) = self
            .displayed_item
            .as_ref()
            .filter(|displayed_item| !displayed_item.entries.is_empty())
        else {
            return;
        };

        let auto_collapse_dirs = OutlinePanelSettings::get_global(cx).auto_fold_dirs;
        let project = self.project.read(cx);
        self.last_worktree_root_id = project
            .visible_worktrees(cx)
            .rev()
            .next()
            .and_then(|worktree| {
                let worktree = worktree.read(cx);
                match &self.displayed_item {
                    None => worktree.root_entry(),
                    Some(displayed_item) => {
                        if displayed_item.entries.is_empty()
                            || displayed_item.entries.contains_key(&worktree.id())
                        {
                            worktree.root_entry()
                        } else {
                            None
                        }
                    }
                }
            })
            .map(|entry| entry.id);

        self.visible_entries.clear();
        for worktree in project.visible_worktrees(cx) {
            let snapshot = worktree.read(cx).snapshot();
            let worktree_id = snapshot.id();

            let Some(displayed_worktree_entries) = displayed_entries
                .entries
                .get(&worktree_id)
                .filter(|entries| !entries.is_empty())
            else {
                continue;
            };

            let expanded_dir_ids = match self.expanded_dir_ids.entry(worktree_id) {
                hash_map::Entry::Occupied(e) => e.into_mut(),
                hash_map::Entry::Vacant(e) => {
                    // The first time a worktree's root entry becomes available,
                    // mark that root entry as expanded.
                    if let Some(entry) = snapshot.root_entry() {
                        e.insert(BTreeSet::from([entry.id]))
                    } else {
                        e.insert(BTreeSet::new())
                    }
                }
            };

            let mut visible_worktree_entries = Vec::new();
            let mut entry_iter = snapshot.entries(true);
            let mut maybe_applicable_folder_entries = Vec::new();
            while let Some(entry) = entry_iter.entry() {
                if !displayed_worktree_entries.contains(&entry.id) {
                    if entry.is_dir() {
                        maybe_applicable_folder_entries.push(entry.clone());
                    }
                    entry_iter.advance();
                    continue;
                }
                if auto_collapse_dirs
                    && entry.kind.is_dir()
                    && !self.unfolded_dir_ids.contains(&entry.id)
                {
                    if let Some(root_path) = snapshot.root_entry() {
                        let mut child_entries = snapshot.child_entries(&entry.path);
                        if let Some(child) = child_entries.next() {
                            if entry.path != root_path.path
                                && child_entries.next().is_none()
                                && child.kind.is_dir()
                            {
                                entry_iter.advance();
                                continue;
                            }
                        }
                    }
                }

                visible_worktree_entries.push(entry.clone());
                if !expanded_dir_ids.contains(&entry.id) && entry_iter.advance_to_sibling() {
                    continue;
                }
                entry_iter.advance();
            }

            // TODO kb slow and odd?
            let mut dir_entries_to_re_add = HashMap::default();
            visible_worktree_entries.retain(|entry| {
                let is_new_entry = new_entries.contains(&entry.id);
                let mut retain_visible_entry = true;
                let mut parent_dir_excluded = false;
                for parent_dir_entry in maybe_applicable_folder_entries
                    .iter()
                    .filter(|maybe_parent_entry| entry.path.starts_with(&maybe_parent_entry.path))
                {
                    let mut exclude_parent_dir = false;
                    if is_new_entry {
                        expanded_dir_ids.insert(parent_dir_entry.id);
                    } else if !expanded_dir_ids.contains(&parent_dir_entry.id) {
                        retain_visible_entry = false;
                        exclude_parent_dir = true;
                    }
                    if !parent_dir_excluded {
                        dir_entries_to_re_add
                            .entry(parent_dir_entry.id)
                            .or_insert_with(|| parent_dir_entry.clone());
                    }
                    if exclude_parent_dir {
                        parent_dir_excluded = true;
                    }
                }
                retain_visible_entry
            });
            visible_worktree_entries.extend(dir_entries_to_re_add.into_values());

            visible_worktree_entries.sort_by(|entry_a, entry_b| {
                let mut components_a = entry_a.path.components().peekable();
                let mut components_b = entry_b.path.components().peekable();
                loop {
                    match (components_a.next(), components_b.next()) {
                        (Some(component_a), Some(component_b)) => {
                            let a_is_file = components_a.peek().is_none() && entry_a.is_file();
                            let b_is_file = components_b.peek().is_none() && entry_b.is_file();
                            let ordering = a_is_file.cmp(&b_is_file).then_with(|| {
                                let maybe_numeric_ordering = maybe!({
                                    let num_and_remainder_a = Path::new(component_a.as_os_str())
                                        .file_stem()
                                        .and_then(|s| s.to_str())
                                        .and_then(
                                            NumericPrefixWithSuffix::from_numeric_prefixed_str,
                                        )?;
                                    let num_and_remainder_b = Path::new(component_b.as_os_str())
                                        .file_stem()
                                        .and_then(|s| s.to_str())
                                        .and_then(
                                            NumericPrefixWithSuffix::from_numeric_prefixed_str,
                                        )?;

                                    num_and_remainder_a.partial_cmp(&num_and_remainder_b)
                                });

                                maybe_numeric_ordering.unwrap_or_else(|| {
                                    let name_a =
                                        UniCase::new(component_a.as_os_str().to_string_lossy());
                                    let name_b =
                                        UniCase::new(component_b.as_os_str().to_string_lossy());

                                    name_a.cmp(&name_b)
                                })
                            });
                            if !ordering.is_eq() {
                                return ordering;
                            }
                        }
                        (Some(_), None) => break cmp::Ordering::Greater,
                        (None, Some(_)) => break cmp::Ordering::Less,
                        (None, None) => break cmp::Ordering::Equal,
                    }
                }
            });

            snapshot.propagate_git_statuses(&mut visible_worktree_entries);
            self.visible_entries
                .push((worktree_id, visible_worktree_entries));
        }

        if let Some((worktree_id, entry_id)) = new_selected_entry {
            self.selection = Some(SelectedEntry {
                worktree_id,
                entry_id,
            });
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
            .then(|| IconName::Code)
    }

    fn icon_tooltip(&self, _: &WindowContext) -> Option<&'static str> {
        Some("Outline Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn starts_open(&self, _: &WindowContext) -> bool {
        match &self.displayed_item {
            Some(displayed_item) => !displayed_item.entries.is_empty(),
            None => false,
        }
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
        if self.visible_entries.is_empty() {
            v_flex()
                .id("empty-outline_panel")
                .size_full()
                .p_4()
                .track_focus(&self.focus_handle)
                .child(Label::new("No active editor"))
        } else {
            h_flex()
                .id("outline-panel")
                .size_full()
                .relative()
                .key_context(self.dispatch_context(cx))
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
                        // When deploying the context menu anywhere below the last project entry,
                        // act as if the user clicked the root of the last worktree.
                        if let Some(entry_id) = outline_panel.last_worktree_root_id {
                            outline_panel.deploy_context_menu(event.position, entry_id, cx);
                        }
                    }),
                )
                .track_focus(&self.focus_handle)
                .child(
                    uniform_list(
                        cx.view().clone(),
                        "entries",
                        self.visible_entries
                            .iter()
                            .map(|(_, worktree_entries)| worktree_entries.len())
                            .sum(),
                        {
                            |outline_panel, range, cx| {
                                let mut items = Vec::new();
                                outline_panel.for_each_visible_entry(
                                    range,
                                    cx,
                                    |id, details, cx| {
                                        items.push(outline_panel.render_entry(id, details, cx));
                                    },
                                );
                                items
                            }
                        },
                    )
                    .size_full()
                    .track_scroll(self.scroll_handle.clone()),
                )
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

fn subscribe_for_editor_changes(
    editor: &View<Editor>,
    cx: &mut ViewContext<OutlinePanel>,
) -> Option<Subscription> {
    if OutlinePanelSettings::get_global(cx).auto_reveal_entries {
        Some(
            cx.subscribe(editor, |outline_panel, editor, e: &EditorEvent, cx| {
                if let EditorEvent::SelectionsChanged { local: true } = e {
                    if let Some(entry_id) = entry_id_for_selection(&editor, cx) {
                        outline_panel.reveal_entry(
                            outline_panel.project.clone(),
                            entry_id,
                            false,
                            cx,
                        );
                        return;
                    }
                }
                cx.propagate();
            }),
        )
    } else {
        None
    }
}

fn entry_id_for_selection(
    editor: &View<Editor>,
    cx: &mut ViewContext<OutlinePanel>,
) -> Option<ProjectEntryId> {
    let selection = editor
        .read(cx)
        .selections
        .newest::<language::Point>(cx)
        .head();
    let multi_buffer = editor.read(cx).buffer();
    let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);
    let (buffer_snapshot, _) = multi_buffer_snapshot.point_to_buffer_offset(selection)?;
    let buffer = multi_buffer.read(cx).buffer(buffer_snapshot.remote_id())?;
    buffer.read(cx).entry_id(cx)
}

// TODO kb tests
