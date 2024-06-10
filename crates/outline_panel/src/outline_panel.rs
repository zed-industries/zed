mod outline_panel_settings;

use std::{
    cmp,
    hash::Hash,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use anyhow::Context;
use collections::{hash_map, BTreeSet, HashMap, HashSet};
use db::kvp::KEY_VALUE_STORE;
use editor::{
    items::{entry_git_aware_label_color, entry_label_color},
    scroll::ScrollAnchor,
    Editor, EditorEvent, ExcerptId,
};
use file_icons::FileIcons;
use git::repository::GitFileStatus;
use gpui::{
    actions, anchored, deferred, div, px, uniform_list, Action, AppContext, AssetSource,
    AsyncWindowContext, ClipboardItem, DismissEvent, Div, ElementId, EntityId, EventEmitter,
    FocusHandle, FocusableView, InteractiveElement, IntoElement, KeyContext, Model, MouseButton,
    MouseDownEvent, ParentElement, Pixels, Point, Render, SharedString, Stateful, Styled,
    Subscription, Task, UniformListScrollHandle, View, ViewContext, VisualContext, WeakView,
    WindowContext,
};
use language::{BufferId, OffsetRangeExt, OutlineItem};
use menu::{SelectFirst, SelectLast, SelectNext, SelectPrev};

use outline_panel_settings::{OutlinePanelDockPosition, OutlinePanelSettings};
use project::{EntryKind, File, Fs, Project};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use unicase::UniCase;
use util::{maybe, NumericPrefixWithSuffix, ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    item::ItemHandle,
    ui::{
        h_flex, v_flex, ActiveTheme, Color, ContextMenu, FluentBuilder, Icon, IconName, IconSize,
        Label, LabelCommon, ListItem, Selectable,
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
        RevealInFinder,
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
    fs_entries_depth: Vec<usize>,
    fs_entries: Vec<FsEntry>,
    collapsed_dirs: HashMap<WorktreeId, BTreeSet<ProjectEntryId>>,
    unfolded_dirs: HashMap<WorktreeId, BTreeSet<ProjectEntryId>>,
    last_visible_range: Range<usize>,
    selected_entry: Option<EntryOwned>,
    displayed_item: Option<DisplayedActiveItem>,
    _subscriptions: Vec<Subscription>,
    update_task: Task<()>,
    outline_fetch_tasks: Vec<Task<()>>,
    outlines: HashMap<OutlinesContainer, Vec<Outline>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum EntryOwned {
    Entry(FsEntry),
    FoldedDirs(WorktreeId, Vec<Entry>),
    Outline(OutlinesContainer, Outline),
}

impl EntryOwned {
    fn to_ref_entry(&self) -> EntryRef<'_> {
        match self {
            Self::Entry(entry) => EntryRef::Entry(entry),
            Self::FoldedDirs(worktree_id, dirs) => EntryRef::FoldedDirs(*worktree_id, dirs),
            Self::Outline(container, outline) => EntryRef::Outline(*container, outline),
        }
    }

    fn abs_path(&self, project: &Model<Project>, cx: &AppContext) -> Option<PathBuf> {
        match self {
            Self::Entry(entry) => entry.abs_path(project, cx),
            Self::FoldedDirs(worktree_id, dirs) => dirs.last().and_then(|entry| {
                project
                    .read(cx)
                    .worktree_for_id(*worktree_id, cx)
                    .and_then(|worktree| worktree.read(cx).absolutize(&entry.path).ok())
            }),
            Self::Outline(..) => None,
        }
    }

    fn outlines_container(&self) -> Option<OutlinesContainer> {
        match self {
            Self::Entry(entry) => entry.outlines_container(),
            Self::FoldedDirs(..) => None,
            Self::Outline(container, _) => Some(*container),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntryRef<'a> {
    Entry(&'a FsEntry),
    FoldedDirs(WorktreeId, &'a [Entry]),
    Outline(OutlinesContainer, &'a Outline),
}

impl EntryRef<'_> {
    fn to_owned_entry(&self) -> EntryOwned {
        match self {
            &Self::Entry(entry) => EntryOwned::Entry(entry.clone()),
            &Self::FoldedDirs(worktree_id, dirs) => {
                EntryOwned::FoldedDirs(worktree_id, dirs.to_vec())
            }
            &Self::Outline(container, outline) => EntryOwned::Outline(container, outline.clone()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum OutlinesContainer {
    ExternalFile(BufferId),
    File(WorktreeId, ProjectEntryId),
}

#[derive(Clone, Debug, Eq)]
enum FsEntry {
    ExternalFile(BufferId),
    Directory(WorktreeId, Entry),
    File(WorktreeId, Entry),
}

impl PartialEq for FsEntry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ExternalFile(id_a), Self::ExternalFile(id_b)) => id_a == id_b,
            (Self::Directory(id_a, entry_a), Self::Directory(id_b, entry_b)) => {
                id_a == id_b && entry_a.id == entry_b.id
            }
            (Self::File(worktree_a, entry_a), Self::File(worktree_b, entry_b)) => {
                worktree_a == worktree_b && entry_a.id == entry_b.id
            }
            _ => false,
        }
    }
}

impl Hash for FsEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::ExternalFile(buffer_id) => buffer_id.hash(state),
            Self::Directory(worktree_id, entry) => {
                worktree_id.hash(state);
                entry.id.hash(state);
            }
            Self::File(worktree_id, entry) => {
                worktree_id.hash(state);
                entry.id.hash(state);
            }
        }
    }
}

impl FsEntry {
    fn abs_path(&self, project: &Model<Project>, cx: &AppContext) -> Option<PathBuf> {
        match self {
            Self::ExternalFile(buffer_id) => project
                .read(cx)
                .buffer_for_id(*buffer_id)
                .and_then(|buffer| File::from_dyn(buffer.read(cx).file()))
                .and_then(|file| file.worktree.read(cx).absolutize(&file.path).ok()),
            Self::Directory(worktree_id, entry) => project
                .read(cx)
                .worktree_for_id(*worktree_id, cx)?
                .read(cx)
                .absolutize(&entry.path)
                .ok(),
            Self::File(worktree_id, entry) => project
                .read(cx)
                .worktree_for_id(*worktree_id, cx)?
                .read(cx)
                .absolutize(&entry.path)
                .ok(),
        }
    }

    fn relative_path<'a>(
        &'a self,
        project: &Model<Project>,
        cx: &'a AppContext,
    ) -> Option<&'a Path> {
        match self {
            Self::ExternalFile(buffer_id) => project
                .read(cx)
                .buffer_for_id(*buffer_id)
                .and_then(|buffer| buffer.read(cx).file())
                .map(|file| file.path().as_ref()),
            Self::Directory(_, entry) => Some(entry.path.as_ref()),
            Self::File(_, entry) => Some(entry.path.as_ref()),
        }
    }

    fn outlines_container(&self) -> Option<OutlinesContainer> {
        match self {
            Self::ExternalFile(buffer_id) => Some(OutlinesContainer::ExternalFile(*buffer_id)),
            Self::File(worktree_id, entry) => Some(OutlinesContainer::File(*worktree_id, entry.id)),
            Self::Directory(..) => None,
        }
    }
}

struct DisplayedActiveItem {
    item_id: EntityId,
    active_editor: WeakView<Editor>,
    _editor_subscrpiption: Option<Subscription>,
}

#[derive(Debug)]
pub enum Event {
    Focus,
}

#[derive(Serialize, Deserialize)]
struct SerializedOutlinePanel {
    width: Option<Pixels>,
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
                        if let Some(new_active_editor) = workspace
                            .read(cx)
                            .active_item(cx)
                            .and_then(|item| item.act_as::<Editor>(cx))
                        {
                            let active_editor_updated = outline_panel
                                .displayed_item
                                .as_ref()
                                .map_or(true, |displayed_item| {
                                    displayed_item.item_id != new_active_editor.item_id()
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
                fs_entries_depth: Vec::new(),
                collapsed_dirs: HashMap::default(),
                unfolded_dirs: HashMap::default(),
                selected_entry: None,
                context_menu: None,
                width: None,
                displayed_item: None,
                pending_serialization: Task::ready(None),
                update_task: Task::ready(()),
                outline_fetch_tasks: Vec::new(),
                outlines: HashMap::default(),
                last_visible_range: 0..0,
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

    fn unfold_directory(&mut self, _: &UnfoldDirectory, cx: &mut ViewContext<Self>) {
        let Some(editor) = self
            .displayed_item
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
            .displayed_item
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

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if let Some(selected_entry) = &self.selected_entry {
            let outline_to_select = match selected_entry {
                EntryOwned::Entry(entry) => entry.outlines_container().and_then(|container| {
                    let next_outline = self.outlines.get(&container)?.first()?.clone();
                    Some((container, next_outline))
                }),
                EntryOwned::FoldedDirs(..) => None,
                EntryOwned::Outline(container, outline) => self
                    .outlines
                    .get(container)
                    .and_then(|outlines| {
                        outlines.iter().skip_while(|o| o != &outline).skip(1).next()
                    })
                    .map(|outline| (*container, outline.clone())),
            }
            .map(|(container, outline)| EntryOwned::Outline(container, outline));

            let entry_to_select = outline_to_select.or_else(|| {
                match selected_entry {
                    EntryOwned::Entry(entry) => self
                        .fs_entries
                        .iter()
                        .skip_while(|e| e != &entry)
                        .skip(1)
                        .next(),
                    EntryOwned::FoldedDirs(worktree_id, dirs) => self
                        .fs_entries
                        .iter()
                        .skip_while(|e| {
                            if let FsEntry::Directory(dir_worktree_id, dir_entry) = e {
                                dir_worktree_id != worktree_id || dirs.last() != Some(dir_entry)
                            } else {
                                true
                            }
                        })
                        .skip(1)
                        .next(),
                    EntryOwned::Outline(container, _) => self
                        .fs_entries
                        .iter()
                        .skip_while(|entry| entry.outlines_container().as_ref() != Some(container))
                        .skip(1)
                        .next(),
                }
                .cloned()
                .map(EntryOwned::Entry)
            });

            if let Some(entry_to_select) = entry_to_select {
                self.selected_entry = Some(entry_to_select);
                self.autoscroll(cx);
                cx.notify();
            }
        } else {
            self.select_first(&SelectFirst {}, cx)
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if let Some(selected_entry) = &self.selected_entry {
            let outline_to_select = match selected_entry {
                EntryOwned::Entry(entry) => {
                    let previous_entry = self
                        .fs_entries
                        .iter()
                        .rev()
                        .skip_while(|e| e != &entry)
                        .skip(1)
                        .next();
                    previous_entry
                        .and_then(|entry| entry.outlines_container())
                        .and_then(|container| {
                            let previous_outline = self.outlines.get(&container)?.last()?.clone();
                            Some((container, previous_outline))
                        })
                }
                EntryOwned::FoldedDirs(worktree_id, dirs) => {
                    let previous_entry = self
                        .fs_entries
                        .iter()
                        .rev()
                        .skip_while(|e| {
                            if let FsEntry::Directory(dir_worktree_id, dir_entry) = e {
                                dir_worktree_id != worktree_id || dirs.first() != Some(dir_entry)
                            } else {
                                true
                            }
                        })
                        .skip(1)
                        .next();
                    previous_entry
                        .and_then(|entry| entry.outlines_container())
                        .and_then(|container| {
                            let previous_outline = self.outlines.get(&container)?.last()?.clone();
                            Some((container, previous_outline))
                        })
                }
                EntryOwned::Outline(container, outline) => self
                    .outlines
                    .get(container)
                    .and_then(|outlines| {
                        outlines
                            .iter()
                            .rev()
                            .skip_while(|o| o != &outline)
                            .skip(1)
                            .next()
                    })
                    .map(|outline| (*container, outline.clone())),
            }
            .map(|(container, outline)| EntryOwned::Outline(container, outline));

            let entry_to_select = outline_to_select.or_else(|| {
                match selected_entry {
                    EntryOwned::Entry(entry) => self
                        .fs_entries
                        .iter()
                        .rev()
                        .skip_while(|e| e != &entry)
                        .skip(1)
                        .next(),
                    EntryOwned::FoldedDirs(worktree_id, dirs) => self
                        .fs_entries
                        .iter()
                        .rev()
                        .skip_while(|e| {
                            if let FsEntry::Directory(dir_worktree_id, dir_entry) = e {
                                dir_worktree_id != worktree_id || dirs.first() != Some(dir_entry)
                            } else {
                                true
                            }
                        })
                        .skip(1)
                        .next(),
                    EntryOwned::Outline(container, _) => self
                        .fs_entries
                        .iter()
                        .rev()
                        .find(|entry| entry.outlines_container().as_ref() == Some(container)),
                }
                .cloned()
                .map(EntryOwned::Entry)
            });

            if let Some(entry_to_select) = entry_to_select {
                self.selected_entry = Some(entry_to_select);
                self.autoscroll(cx);
                cx.notify();
            }
        } else {
            self.select_first(&SelectFirst {}, cx);
        }
    }

    fn select_parent(&mut self, _: &SelectParent, cx: &mut ViewContext<Self>) {
        if let Some(selected_entry) = &self.selected_entry {
            let parent_entry = match selected_entry {
                EntryOwned::Entry(entry) => self
                    .fs_entries
                    .iter()
                    .rev()
                    .skip_while(|e| e != &entry)
                    .skip(1)
                    .find(|entry_before_current| match (entry, entry_before_current) {
                        (
                            FsEntry::File(worktree_id, entry)
                            | FsEntry::Directory(worktree_id, entry),
                            FsEntry::Directory(parent_worktree_id, parent_entry),
                        ) => {
                            parent_worktree_id == worktree_id
                                && directory_contains(parent_entry, entry)
                        }
                        _ => false,
                    }),
                EntryOwned::FoldedDirs(worktree_id, dirs) => self
                    .fs_entries
                    .iter()
                    .rev()
                    .skip_while(|e| {
                        if let FsEntry::Directory(dir_worktree_id, dir_entry) = e {
                            dir_worktree_id != worktree_id || dirs.first() != Some(dir_entry)
                        } else {
                            true
                        }
                    })
                    .skip(1)
                    .find(
                        |entry_before_current| match (dirs.first(), entry_before_current) {
                            (Some(entry), FsEntry::Directory(parent_worktree_id, parent_entry)) => {
                                parent_worktree_id == worktree_id
                                    && directory_contains(parent_entry, entry)
                            }
                            _ => false,
                        },
                    ),
                EntryOwned::Outline(container, _) => self
                    .fs_entries
                    .iter()
                    .find(|entry| entry.outlines_container().as_ref() == Some(container)),
            }
            .cloned()
            .map(EntryOwned::Entry);
            if let Some(parent_entry) = parent_entry {
                self.selected_entry = Some(parent_entry);
                self.autoscroll(cx);
                cx.notify();
            }
        } else {
            self.select_first(&SelectFirst {}, cx);
        }
    }

    fn select_first(&mut self, _: &SelectFirst, cx: &mut ViewContext<Self>) {
        if let Some(first_entry) = self.fs_entries.first().cloned().map(EntryOwned::Entry) {
            self.selected_entry = Some(first_entry);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &SelectLast, cx: &mut ViewContext<Self>) {
        if let Some(new_selection) = self.fs_entries.last().map(|last_entry| {
            last_entry
                .outlines_container()
                .and_then(|container| {
                    let outline = self.outlines.get(&container)?.last()?;
                    Some((container, outline.clone()))
                })
                .map(|(container, outline)| EntryOwned::Outline(container, outline))
                .unwrap_or_else(|| EntryOwned::Entry(last_entry.clone()))
        }) {
            self.selected_entry = Some(new_selection);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn autoscroll(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(selected_entry) = &self.selected_entry {
            let index = self
                .entries_with_depths(cx)
                .into_iter()
                .position(|(_, entry)| &entry == selected_entry);
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
            EntryRef::Entry(FsEntry::File(worktree_id, entry))
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
            EntryRef::Outline(_, _) => {
                cx.notify();
                return;
            }
        };
        let auto_fold_dirs = OutlinePanelSettings::get_global(cx).auto_fold_dirs;
        let is_foldable = auto_fold_dirs && !is_root && self.is_foldable(entry);
        let is_unfoldable = auto_fold_dirs && !is_root && self.is_unfoldable(entry);

        let context_menu = ContextMenu::build(cx, |menu, _| {
            menu.context(self.focus_handle.clone())
                .action("Copy Relative Path", Box::new(CopyRelativePath))
                .action("Reveal in Finder", Box::new(RevealInFinder))
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
                FsEntry::ExternalFile(_) => false,
                FsEntry::Directory(worktree_id, entry) | FsEntry::File(worktree_id, entry) => {
                    worktree_id == &directory_worktree
                        && entry.path.parent() == Some(directory_entry.path.as_ref())
                }
            })
            .collect::<Vec<_>>();

        child_entries.len() == 1 && matches!(child_entries.first(), Some(FsEntry::Directory(..)))
    }

    fn expand_selected_entry(&mut self, _: &ExpandSelectedEntry, cx: &mut ViewContext<Self>) {
        let Some(editor) = self
            .displayed_item
            .as_ref()
            .and_then(|item| item.active_editor.upgrade())
        else {
            return;
        };
        if let Some(EntryOwned::Entry(FsEntry::Directory(worktree_id, selected_dir_entry))) =
            &self.selected_entry
        {
            let expanded = self
                .collapsed_dirs
                .get_mut(worktree_id)
                .map_or(false, |hidden_dirs| {
                    hidden_dirs.remove(&selected_dir_entry.id)
                });
            if expanded {
                self.project.update(cx, |project, cx| {
                    project.expand_entry(*worktree_id, selected_dir_entry.id, cx);
                });
                self.update_fs_entries(&editor, HashSet::default(), None, None, false, cx);
            } else {
                self.select_next(&SelectNext, cx)
            }
        }
    }

    fn collapse_selected_entry(&mut self, _: &CollapseSelectedEntry, cx: &mut ViewContext<Self>) {
        let Some(editor) = self
            .displayed_item
            .as_ref()
            .and_then(|item| item.active_editor.upgrade())
        else {
            return;
        };
        if let Some(
            dir_entry @ EntryOwned::Entry(FsEntry::Directory(worktree_id, selected_dir_entry)),
        ) = &self.selected_entry
        {
            self.collapsed_dirs
                .entry(*worktree_id)
                .or_default()
                .insert(selected_dir_entry.id);
            self.update_fs_entries(
                &editor,
                HashSet::default(),
                Some(dir_entry.clone()),
                None,
                false,
                cx,
            );
        }
    }

    pub fn collapse_all_entries(&mut self, _: &CollapseAllEntries, cx: &mut ViewContext<Self>) {
        let Some(editor) = self
            .displayed_item
            .as_ref()
            .and_then(|item| item.active_editor.upgrade())
        else {
            return;
        };

        self.fs_entries_depth
            .iter()
            .enumerate()
            .filter(|(_, depth)| depth == &&0)
            .filter_map(|(i, _)| self.fs_entries.get(i))
            .filter_map(|entry| match entry {
                FsEntry::Directory(worktree_id, dir_entry) => Some((*worktree_id, dir_entry)),
                _ => None,
            })
            .for_each(|(worktree_id, dir_entry)| {
                self.collapsed_dirs
                    .entry(worktree_id)
                    .or_default()
                    .insert(dir_entry.id);
            });
        self.update_fs_entries(&editor, HashSet::default(), None, None, false, cx);
    }

    fn toggle_expanded(&mut self, entry: &EntryOwned, cx: &mut ViewContext<Self>) {
        let Some(editor) = self
            .displayed_item
            .as_ref()
            .and_then(|item| item.active_editor.upgrade())
        else {
            return;
        };

        match entry {
            EntryOwned::Entry(FsEntry::Directory(worktree_id, dir_entry)) => {
                let entry_id = dir_entry.id;
                match self.collapsed_dirs.entry(*worktree_id) {
                    hash_map::Entry::Occupied(mut o) => {
                        let collapsed_dir_ids = o.get_mut();
                        if collapsed_dir_ids.remove(&entry_id) {
                            self.project
                                .update(cx, |project, cx| {
                                    project.expand_entry(*worktree_id, entry_id, cx)
                                })
                                .unwrap_or_else(|| Task::ready(Ok(())))
                                .detach_and_log_err(cx);
                        } else {
                            collapsed_dir_ids.insert(entry_id);
                        }
                    }
                    hash_map::Entry::Vacant(v) => {
                        v.insert(BTreeSet::new()).insert(entry_id);
                    }
                }
            }
            EntryOwned::FoldedDirs(worktree_id, dir_entries) => {
                if let Some(entry_id) = dir_entries.first().map(|entry| entry.id) {
                    match self.collapsed_dirs.entry(*worktree_id) {
                        hash_map::Entry::Occupied(mut o) => {
                            let collapsed_dir_ids = o.get_mut();
                            if collapsed_dir_ids.remove(&entry_id) {
                                self.project
                                    .update(cx, |project, cx| {
                                        project.expand_entry(*worktree_id, entry_id, cx)
                                    })
                                    .unwrap_or_else(|| Task::ready(Ok(())))
                                    .detach_and_log_err(cx);
                            } else {
                                collapsed_dir_ids.insert(entry_id);
                            }
                        }
                        hash_map::Entry::Vacant(v) => {
                            v.insert(BTreeSet::new()).insert(entry_id);
                        }
                    }
                }
            }
            _ => return,
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
            .and_then(|entry| entry.abs_path(&self.project, cx))
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
                EntryOwned::Entry(entry) => entry.relative_path(&self.project, cx),
                EntryOwned::FoldedDirs(_, dirs) => dirs.last().map(|entry| entry.path.as_ref()),
                EntryOwned::Outline(..) => None,
            })
            .map(|p| p.to_string_lossy().to_string())
        {
            cx.write_to_clipboard(ClipboardItem::new(clipboard_text));
        }
    }

    fn reveal_in_finder(&mut self, _: &RevealInFinder, cx: &mut ViewContext<Self>) {
        if let Some(abs_path) = self
            .selected_entry
            .as_ref()
            .and_then(|entry| entry.abs_path(&self.project, cx))
        {
            cx.reveal_path(&abs_path);
        }
    }

    fn open_in_terminal(&mut self, _: &OpenInTerminal, cx: &mut ViewContext<Self>) {
        let selected_entry = self.selected_entry.as_ref();
        let abs_path = selected_entry.and_then(|entry| entry.abs_path(&self.project, cx));
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
        let Some((container, outline_item)) = self.location_for_editor_selection(editor, cx) else {
            return;
        };

        let file_entry_to_expand = self
            .fs_entries
            .iter()
            .find(|entry| match (entry, &container) {
                (
                    FsEntry::ExternalFile(buffer_id),
                    OutlinesContainer::ExternalFile(container_buffer_id),
                ) => buffer_id == container_buffer_id,
                (
                    FsEntry::File(file_worktree_id, file_entry),
                    OutlinesContainer::File(worktree_id, id),
                ) => file_worktree_id == worktree_id && &file_entry.id == id,
                _ => false,
            });
        let Some(entry_to_select) = outline_item
            .map(|outline| EntryOwned::Outline(container, outline))
            .or_else(|| Some(EntryOwned::Entry(file_entry_to_expand.cloned()?)))
        else {
            return;
        };

        if self.selected_entry.as_ref() == Some(&entry_to_select) {
            return;
        }

        if let Some(FsEntry::File(file_worktree_id, file_entry)) = file_entry_to_expand {
            if let Some(worktree) = self.project.read(cx).worktree_for_id(*file_worktree_id, cx) {
                let parent_entry = {
                    let mut traversal = worktree.read(cx).traverse_from_path(
                        true,
                        true,
                        true,
                        file_entry.path.as_ref(),
                    );
                    if traversal.back_to_parent() {
                        traversal.entry()
                    } else {
                        None
                    }
                    .cloned()
                };
                if let Some(directory_entry) = parent_entry {
                    self.expand_entry(worktree.read(cx).id(), directory_entry.id, cx);
                }
            }
        }

        self.update_fs_entries(
            &editor,
            HashSet::default(),
            Some(entry_to_select),
            None,
            false,
            cx,
        );
    }

    fn expand_entry(
        &mut self,
        worktree_id: WorktreeId,
        entry_id: ProjectEntryId,
        cx: &mut AppContext,
    ) {
        if let Some(collapsed_dir_ids) = self.collapsed_dirs.get_mut(&worktree_id) {
            if collapsed_dir_ids.remove(&entry_id) {
                self.project
                    .update(cx, |project, cx| {
                        project.expand_entry(worktree_id, entry_id, cx)
                    })
                    .unwrap_or_else(|| Task::ready(Ok(())))
                    .detach_and_log_err(cx)
            }
        }
    }

    fn render_outline(
        &self,
        container: OutlinesContainer,
        rendered_outline: &Outline,
        depth: usize,
        cx: &mut ViewContext<Self>,
    ) -> Stateful<Div> {
        let (item_id, label_element) = (
            ElementId::from(SharedString::from(format!(
                "{:?}|{:?}",
                rendered_outline.range, &rendered_outline.text,
            ))),
            language::render_item(&rendered_outline, None, cx).into_any_element(),
        );
        let is_active = match &self.selected_entry {
            Some(EntryOwned::Outline(selected_container, selected_entry)) => {
                selected_container == &container && selected_entry == rendered_outline
            }
            _ => false,
        };

        self.entry_element(
            EntryRef::Outline(container, rendered_outline),
            item_id,
            depth,
            None,
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
            FsEntry::File(worktree_id, entry) => {
                let name = self.entry_name(worktree_id, entry, cx);
                let color =
                    entry_git_aware_label_color(entry.git_status, entry.is_ignored, is_active);
                let icon = if settings.file_icons {
                    FileIcons::get_icon(&entry.path, cx)
                } else {
                    None
                }
                .map(Icon::from_path)
                .map(|icon| icon.color(color));
                (
                    ElementId::from(entry.id.to_proto() as usize),
                    Label::new(name)
                        .single_line()
                        .color(color)
                        .into_any_element(),
                    icon,
                )
            }
            FsEntry::Directory(worktree_id, entry) => {
                let name = self.entry_name(worktree_id, entry, cx);

                let is_expanded = self
                    .collapsed_dirs
                    .get(worktree_id)
                    .map_or(true, |ids| !ids.contains(&entry.id));
                let color =
                    entry_git_aware_label_color(entry.git_status, entry.is_ignored, is_active);
                let icon = if settings.folder_icons {
                    FileIcons::get_folder_icon(is_expanded, cx)
                } else {
                    FileIcons::get_chevron_icon(is_expanded, cx)
                }
                .map(Icon::from_path)
                .map(|icon| icon.color(color));
                (
                    ElementId::from(entry.id.to_proto() as usize),
                    Label::new(name)
                        .single_line()
                        .color(color)
                        .into_any_element(),
                    icon,
                )
            }
            FsEntry::ExternalFile(buffer_id) => {
                let color = entry_label_color(is_active);
                let (icon, name) = match self.project.read(cx).buffer_for_id(*buffer_id) {
                    Some(buffer) => match buffer.read(cx).file() {
                        Some(file) => {
                            let path = file.path();
                            let icon = if settings.file_icons {
                                FileIcons::get_icon(path.as_ref(), cx)
                            } else {
                                None
                            }
                            .map(Icon::from_path)
                            .map(|icon| icon.color(color));
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
                    icon,
                )
            }
        };

        self.entry_element(
            EntryRef::Entry(rendered_entry),
            item_id,
            depth,
            icon,
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

            let is_expanded =
                self.collapsed_dirs
                    .get(&worktree_id)
                    .map_or(true, |collapsed_dirs| {
                        dir_entries
                            .iter()
                            .all(|dir| !collapsed_dirs.contains(&dir.id))
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
            .map(|icon| icon.color(color));
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
                icon,
            )
        };

        self.entry_element(
            EntryRef::FoldedDirs(worktree_id, dir_entries),
            item_id,
            depth,
            icon,
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
        icon: Option<Icon>,
        is_active: bool,
        label_element: gpui::AnyElement,
        cx: &mut ViewContext<OutlinePanel>,
    ) -> Stateful<Div> {
        let settings = OutlinePanelSettings::get_global(cx);
        let rendered_entry = rendered_entry.to_owned_entry();
        div()
            .id(item_id.clone())
            .child(
                ListItem::new(item_id)
                    .indent_level(depth)
                    .indent_step_size(px(settings.indent_size))
                    .selected(is_active)
                    .child(if let Some(icon) = icon {
                        h_flex().child(icon)
                    } else {
                        h_flex()
                            .size(IconSize::default().rems())
                            .invisible()
                            .flex_none()
                    })
                    .child(h_flex().h_6().child(label_element).ml_1())
                    .on_click({
                        let clicked_entry = rendered_entry.clone();
                        cx.listener(move |outline_panel, event: &gpui::ClickEvent, cx| {
                            if event.down.button == MouseButton::Right || event.down.first_mouse {
                                return;
                            }

                            let Some(active_editor) = outline_panel
                                .displayed_item
                                .as_ref()
                                .and_then(|item| item.active_editor.upgrade())
                            else {
                                return;
                            };
                            let active_multi_buffer = active_editor.read(cx).buffer().clone();
                            let multi_buffer_snapshot = active_multi_buffer.read(cx).snapshot(cx);

                            match &clicked_entry {
                                EntryOwned::Entry(FsEntry::ExternalFile(buffer_id)) => {
                                    let scroll_target = multi_buffer_snapshot.excerpts().find_map(
                                        |(excerpt_id, buffer_snapshot, excerpt_range)| {
                                            if &buffer_snapshot.remote_id() == buffer_id {
                                                multi_buffer_snapshot.anchor_in_excerpt(
                                                    excerpt_id,
                                                    excerpt_range.context.start,
                                                )
                                            } else {
                                                None
                                            }
                                        },
                                    );
                                    if let Some(anchor) = scroll_target {
                                        outline_panel.selected_entry = Some(clicked_entry.clone());
                                        active_editor.update(cx, |editor, cx| {
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
                                entry @ EntryOwned::Entry(FsEntry::Directory(..)) => {
                                    outline_panel.toggle_expanded(entry, cx);
                                }
                                entry @ EntryOwned::FoldedDirs(..) => {
                                    outline_panel.toggle_expanded(entry, cx);
                                }
                                EntryOwned::Entry(FsEntry::File(_, file_entry)) => {
                                    let scroll_target = outline_panel
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
                                            multi_buffer_snapshot.anchor_in_excerpt(
                                                *excerpt_id,
                                                excerpt_range.context.start,
                                            )
                                        });
                                    if let Some(anchor) = scroll_target {
                                        outline_panel.selected_entry = Some(clicked_entry.clone());
                                        active_editor.update(cx, |editor, cx| {
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
                                EntryOwned::Outline(_, outline) => {
                                    let Some(full_buffer_snapshot) = outline
                                        .range
                                        .start
                                        .buffer_id
                                        .and_then(|buffer_id| {
                                            active_multi_buffer.read(cx).buffer(buffer_id)
                                        })
                                        .or_else(|| {
                                            outline.range.end.buffer_id.and_then(|buffer_id| {
                                                active_multi_buffer.read(cx).buffer(buffer_id)
                                            })
                                        })
                                        .map(|buffer| buffer.read(cx).snapshot())
                                    else {
                                        return;
                                    };
                                    let outline_offset_range =
                                        outline.range.to_offset(&full_buffer_snapshot);
                                    let scroll_target = multi_buffer_snapshot
                                        .excerpts()
                                        .filter(|(_, buffer_snapshot, _)| {
                                            let buffer_id = buffer_snapshot.remote_id();
                                            Some(buffer_id) == outline.range.start.buffer_id
                                                || Some(buffer_id) == outline.range.end.buffer_id
                                        })
                                        .min_by_key(|(_, _, excerpt_range)| {
                                            let excerpt_offeset_range = excerpt_range
                                                .context
                                                .to_offset(&full_buffer_snapshot);
                                            ((outline_offset_range.start / 2
                                                + outline_offset_range.end / 2)
                                                as isize
                                                - (excerpt_offeset_range.start / 2
                                                    + excerpt_offeset_range.end / 2)
                                                    as isize)
                                                .abs()
                                        })
                                        .and_then(
                                            |(excerpt_id, excerpt_snapshot, excerpt_range)| {
                                                let location = if outline
                                                    .range
                                                    .start
                                                    .is_valid(excerpt_snapshot)
                                                {
                                                    outline.range.start
                                                } else {
                                                    excerpt_range.context.start
                                                };
                                                multi_buffer_snapshot
                                                    .anchor_in_excerpt(excerpt_id, location)
                                            },
                                        );
                                    if let Some(anchor) = scroll_target {
                                        outline_panel.selected_entry = Some(clicked_entry.clone());
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

        let displayed_multi_buffer = active_editor.read(cx).buffer().clone();
        let multi_buffer_snapshot = displayed_multi_buffer.read(cx).snapshot(cx);
        let mut new_collapsed_dirs = self.collapsed_dirs.clone();
        let mut new_unfolded_dirs = self.unfolded_dirs.clone();
        let excerpts = multi_buffer_snapshot
            .excerpts()
            .map(|(excerpt_id, buffer_snapshot, _)| {
                let file = File::from_dyn(buffer_snapshot.file());
                let entry_id = file.and_then(|file| file.project_entry_id(cx));
                let worktree = file.map(|file| file.worktree.read(cx).snapshot());
                (excerpt_id, buffer_snapshot.remote_id(), entry_id, worktree)
            })
            .collect::<Vec<_>>();

        self.update_task = cx.spawn(|outline_panel, mut cx| async move {
            if let Some(debounce) = debounce {
                cx.background_executor().timer(debounce).await;
            }
            let Some((new_collapsed_dirs, new_unfolded_dirs, new_fs_entries, new_depth_map)) = cx
                .background_executor()
                .spawn(async move {
                    let mut processed_excernal_buffers = HashSet::default();
                    let mut new_worktree_entries =
                        HashMap::<WorktreeId, (worktree::Snapshot, HashSet<Entry>)>::default();
                    let mut external_entries = Vec::default();

                    for (excerpt_id, buffer_id, file_entry_id, worktree) in excerpts {
                        let is_new = new_entries.contains(&excerpt_id);
                        if let Some(worktree) = worktree {
                            let collapsed_dirs =
                                new_collapsed_dirs.entry(worktree.id()).or_default();
                            let unfolded_dirs = new_unfolded_dirs.entry(worktree.id()).or_default();

                            match file_entry_id
                                .and_then(|id| worktree.entry_for_id(id))
                                .cloned()
                            {
                                Some(entry) => {
                                    let mut traversal = worktree.traverse_from_path(
                                        true,
                                        true,
                                        true,
                                        entry.path.as_ref(),
                                    );

                                    let mut entries_to_add = HashSet::default();
                                    let mut current_entry = entry;
                                    loop {
                                        if current_entry.is_dir() {
                                            if worktree.root_entry().map(|entry| entry.id)
                                                == Some(current_entry.id)
                                            {
                                                unfolded_dirs.insert(current_entry.id);
                                            }
                                            if is_new {
                                                collapsed_dirs.remove(&current_entry.id);
                                            } else if collapsed_dirs.contains(&current_entry.id) {
                                                entries_to_add.clear();
                                            }
                                        }

                                        let new_entry_added = entries_to_add.insert(current_entry);
                                        if new_entry_added && traversal.back_to_parent() {
                                            if let Some(parent_entry) = traversal.entry() {
                                                current_entry = parent_entry.clone();
                                                continue;
                                            }
                                        }
                                        break;
                                    }
                                    new_worktree_entries
                                        .entry(worktree.id())
                                        .or_insert_with(|| (worktree.clone(), HashSet::default()))
                                        .1
                                        .extend(entries_to_add);
                                }
                                None => {
                                    if processed_excernal_buffers.insert(buffer_id) {
                                        external_entries.push(FsEntry::ExternalFile(buffer_id));
                                    }
                                }
                            }
                        } else if processed_excernal_buffers.insert(buffer_id) {
                            external_entries.push(FsEntry::ExternalFile(buffer_id));
                        }
                    }

                    external_entries.sort_by(|entry_a, entry_b| match (entry_a, entry_b) {
                        (
                            FsEntry::ExternalFile(buffer_id_a),
                            FsEntry::ExternalFile(buffer_id_b),
                        ) => buffer_id_a.cmp(&buffer_id_b),
                        (FsEntry::ExternalFile(..), _) => cmp::Ordering::Less,
                        (_, FsEntry::ExternalFile(..)) => cmp::Ordering::Greater,
                        _ => cmp::Ordering::Equal,
                    });

                    let worktree_entries = new_worktree_entries
                        .into_iter()
                        .map(|(worktree_id, (worktree_snapshot, entries))| {
                            let mut entries = entries.into_iter().collect::<Vec<_>>();
                            sort_worktree_entries(&mut entries);
                            worktree_snapshot.propagate_git_statuses(&mut entries);
                            (worktree_id, entries)
                        })
                        .flat_map(|(worktree_id, entries)| {
                            entries.into_iter().map(move |entry| {
                                if entry.is_dir() {
                                    FsEntry::Directory(worktree_id, entry)
                                } else {
                                    FsEntry::File(worktree_id, entry)
                                }
                            })
                        });

                    let mut depth = 0;
                    let mut parent_entry_stack = Vec::new();
                    let mut new_depth_map = Vec::new();
                    let mut fold_started = false;
                    let new_visible_entries = external_entries
                        .into_iter()
                        .chain(worktree_entries)
                        .filter(|visible_item| {
                            match visible_item {
                                FsEntry::Directory(worktree_id, dir_entry) => {
                                    while !parent_entry_stack.is_empty()
                                        && !dir_entry
                                            .path
                                            .starts_with(parent_entry_stack.last().unwrap())
                                    {
                                        fold_started = false;
                                        parent_entry_stack.pop();
                                        if depth > 0 {
                                            depth -= 1;
                                        }
                                    }

                                    let folded = new_unfolded_dirs
                                        .get(worktree_id)
                                        .map_or(true, |unfolded_dirs| {
                                            !unfolded_dirs.contains(&dir_entry.id)
                                        });

                                    parent_entry_stack.push(dir_entry.path.clone());
                                    new_depth_map.push(depth);

                                    if folded {
                                        if !fold_started {
                                            depth += 1;
                                        }
                                        fold_started = true;
                                    } else {
                                        fold_started = false;
                                        depth += 1;
                                    }
                                }
                                FsEntry::File(_, file_entry) => {
                                    fold_started = false;
                                    while !parent_entry_stack.is_empty()
                                        && !file_entry
                                            .path
                                            .starts_with(parent_entry_stack.last().unwrap())
                                    {
                                        parent_entry_stack.pop();
                                        if depth > 0 {
                                            depth -= 1;
                                        }
                                    }
                                    new_depth_map.push(depth);
                                }
                                FsEntry::ExternalFile(..) => {
                                    fold_started = false;
                                    depth = 0;
                                    parent_entry_stack.clear();
                                    new_depth_map.push(depth);
                                }
                            }

                            true
                        })
                        .collect::<Vec<_>>();

                    anyhow::Ok((
                        new_collapsed_dirs,
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
                    outline_panel.collapsed_dirs = new_collapsed_dirs;
                    outline_panel.unfolded_dirs = new_unfolded_dirs;
                    outline_panel.fs_entries = new_fs_entries;
                    outline_panel.fs_entries_depth = new_depth_map;
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
        self.clear_previous();
        self.displayed_item = Some(DisplayedActiveItem {
            item_id: new_active_editor.item_id(),
            _editor_subscrpiption: subscribe_for_editor_events(&new_active_editor, cx),
            active_editor: new_active_editor.downgrade(),
        });
        let new_entries =
            HashSet::from_iter(new_active_editor.read(cx).buffer().read(cx).excerpt_ids());
        self.update_fs_entries(&new_active_editor, new_entries, None, None, true, cx);
    }

    fn clear_previous(&mut self) {
        self.collapsed_dirs.clear();
        self.unfolded_dirs.clear();
        self.last_visible_range = 0..0;
        self.selected_entry = None;
        self.update_task = Task::ready(());
        self.displayed_item = None;
        self.fs_entries.clear();
        self.fs_entries_depth.clear();
        self.outline_fetch_tasks.clear();
        self.outlines.clear();
    }

    fn location_for_editor_selection(
        &self,
        editor: &View<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> Option<(OutlinesContainer, Option<Outline>)> {
        let selection = editor
            .read(cx)
            .selections
            .newest::<language::Point>(cx)
            .head();
        let multi_buffer = editor.read(cx).buffer();
        let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);
        let selection = multi_buffer_snapshot.anchor_before(selection);
        let buffer_snapshot = multi_buffer_snapshot.buffer_for_excerpt(selection.excerpt_id)?;

        let container = match File::from_dyn(buffer_snapshot.file())
            .and_then(|file| Some(file.worktree.read(cx).id()).zip(file.entry_id))
        {
            Some((worktree_id, id)) => OutlinesContainer::File(worktree_id, id),
            None => OutlinesContainer::ExternalFile(buffer_snapshot.remote_id()),
        };

        let outline_item = self
            .outlines
            .get(&container)
            .into_iter()
            .flatten()
            .filter(|outline| {
                outline.range.start.buffer_id == selection.buffer_id
                    || outline.range.end.buffer_id == selection.buffer_id
            })
            .filter(|outline_item| {
                range_contains(&outline_item.range, selection.text_anchor, buffer_snapshot)
            })
            .min_by_key(|outline| {
                let range = outline.range.start.offset..outline.range.end.offset;
                let cursor_offset = selection.text_anchor.offset as isize;
                let distance_to_closest_endpoint = cmp::min(
                    (range.start as isize - cursor_offset).abs(),
                    (range.end as isize - cursor_offset).abs(),
                );
                distance_to_closest_endpoint
            })
            .cloned();

        Some((container, outline_item))
    }

    fn fetch_outlines(&mut self, range: &Range<usize>, cx: &mut ViewContext<Self>) {
        let Some(editor) = self
            .displayed_item
            .as_ref()
            .and_then(|item| item.active_editor.upgrade())
        else {
            return;
        };

        let range_len = range.len();
        let half_range = range_len / 2;
        let entries = self.entries_with_depths(cx);
        let expanded_range =
            range.start.saturating_sub(half_range)..(range.end + half_range).min(entries.len());
        let containers = entries
            .get(expanded_range)
            .into_iter()
            .flatten()
            .flat_map(|(_, entry)| entry.outlines_container())
            .collect::<Vec<_>>();
        let fetch_outlines_for = containers
            .into_iter()
            .filter(|container| match self.outlines.entry(*container) {
                hash_map::Entry::Occupied(_) => false,
                hash_map::Entry::Vacant(v) => {
                    v.insert(Vec::new());
                    true
                }
            })
            .collect::<HashSet<_>>();

        let outlines_to_fetch = editor
            .read(cx)
            .buffer()
            .read(cx)
            .snapshot(cx)
            .excerpts()
            .filter_map(|(_, buffer_snapshot, excerpt_range)| {
                let container = match File::from_dyn(buffer_snapshot.file()) {
                    Some(file) => {
                        let entry_id = file.project_entry_id(cx);
                        let worktree_id = file.worktree.read(cx).id();
                        entry_id.map(|entry_id| OutlinesContainer::File(worktree_id, entry_id))
                    }
                    None => Some(OutlinesContainer::ExternalFile(buffer_snapshot.remote_id())),
                }?;
                Some((container, (buffer_snapshot.clone(), excerpt_range)))
            })
            .filter(|(container, _)| fetch_outlines_for.contains(container))
            .collect::<Vec<_>>();
        if outlines_to_fetch.is_empty() {
            return;
        }

        let syntax_theme = cx.theme().syntax().clone();
        self.outline_fetch_tasks
            .push(cx.spawn(|outline_panel, mut cx| async move {
                let mut processed_outlines =
                    HashMap::<OutlinesContainer, HashSet<Outline>>::default();
                let fetched_outlines = cx
                    .background_executor()
                    .spawn(async move {
                        outlines_to_fetch
                            .into_iter()
                            .map(|(container, (buffer_snapshot, excerpt_range))| {
                                (
                                    container,
                                    buffer_snapshot
                                        .outline_items_containing(
                                            excerpt_range.context,
                                            false,
                                            Some(&syntax_theme),
                                        )
                                        .unwrap_or_default(),
                                )
                            })
                            .fold(
                                HashMap::default(),
                                |mut outlines, (container, new_outlines)| {
                                    outlines
                                        .entry(container)
                                        .or_insert_with(Vec::new)
                                        .extend(new_outlines);
                                    outlines
                                },
                            )
                    })
                    .await;
                outline_panel
                    .update(&mut cx, |outline_panel, cx| {
                        for (container, fetched_outlines) in fetched_outlines {
                            let existing_outlines =
                                outline_panel.outlines.entry(container).or_default();
                            let processed_outlines =
                                processed_outlines.entry(container).or_default();
                            processed_outlines.extend(existing_outlines.iter().cloned());
                            for fetched_outline in fetched_outlines {
                                if processed_outlines.insert(fetched_outline.clone()) {
                                    existing_outlines.push(fetched_outline);
                                }
                            }
                        }
                        cx.notify();
                    })
                    .ok();
            }));
    }

    fn entries_with_depths(&self, cx: &AppContext) -> Vec<(usize, EntryOwned)> {
        let auto_fold_dirs = OutlinePanelSettings::get_global(cx).auto_fold_dirs;
        let mut folded_dirs_entry = None::<(usize, WorktreeId, Vec<Entry>)>;
        let mut entries = Vec::new();

        for (i, entry) in self.fs_entries.iter().enumerate() {
            let mut depth = *self.fs_entries_depth.get(i).unwrap_or(&0);

            if auto_fold_dirs {
                if let FsEntry::Directory(worktree_id, dir_entry) = entry {
                    let folded = self
                        .unfolded_dirs
                        .get(worktree_id)
                        .map_or(true, |unfolded_dirs| !unfolded_dirs.contains(&dir_entry.id));
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
                            folded_dirs_entry = Some((depth, *worktree_id, vec![dir_entry.clone()]))
                        }

                        continue;
                    }
                }
            }
            if let Some((folded_depth, worktree_id, folded_dirs)) = folded_dirs_entry.take() {
                entries.push((
                    folded_depth,
                    EntryOwned::FoldedDirs(worktree_id, folded_dirs),
                ));
            }

            entries.push((depth, EntryOwned::Entry(entry.clone())));
            let mut outline_depth = None::<usize>;
            entries.extend(
                entry
                    .outlines_container()
                    .and_then(|container| Some((container, self.outlines.get(&container)?)))
                    .into_iter()
                    .flat_map(|(container, outlines)| {
                        outlines.iter().map(move |outline| (container, outline))
                    })
                    .map(move |(container, outline)| {
                        if let Some(outline_depth) = outline_depth {
                            match outline_depth.cmp(&outline.depth) {
                                cmp::Ordering::Less => depth += 1,
                                cmp::Ordering::Equal => {}
                                cmp::Ordering::Greater => depth -= 1,
                            };
                        }
                        outline_depth = Some(outline.depth);
                        (depth, EntryOwned::Outline(container, outline.clone()))
                    }),
            )
        }
        if let Some((folded_depth, worktree_id, folded_dirs)) = folded_dirs_entry.take() {
            entries.push((
                folded_depth,
                EntryOwned::FoldedDirs(worktree_id, folded_dirs),
            ));
        }

        entries
    }
}

fn sort_worktree_entries(entries: &mut Vec<Entry>) {
    entries.sort_by(|entry_a, entry_b| {
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
                                .and_then(NumericPrefixWithSuffix::from_numeric_prefixed_str)?;
                            let num_and_remainder_b = Path::new(component_b.as_os_str())
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .and_then(NumericPrefixWithSuffix::from_numeric_prefixed_str)?;

                            num_and_remainder_a.partial_cmp(&num_and_remainder_b)
                        });

                        maybe_numeric_ordering.unwrap_or_else(|| {
                            let name_a = UniCase::new(component_a.as_os_str().to_string_lossy());
                            let name_b = UniCase::new(component_b.as_os_str().to_string_lossy());

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

fn directory_contains(directory_entry: &Entry, child_entry: &Entry) -> bool {
    debug_assert!(directory_entry.is_dir());
    let Some(relative_path) = child_entry.path.strip_prefix(&directory_entry.path).ok() else {
        return false;
    };
    relative_path.iter().count() == 1
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
        self.displayed_item.is_some()
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        let old_active = self.active;
        self.active = active;
        if active && old_active != active {
            if let Some(active_editor) = self
                .displayed_item
                .as_ref()
                .and_then(|item| item.active_editor.upgrade())
            {
                self.replace_visible_entries(active_editor, cx);
            }
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
        if self.fs_entries.is_empty() {
            v_flex()
                .id("empty-outline_panel")
                .size_full()
                .p_4()
                .track_focus(&self.focus_handle)
                .child(Label::new("No editor outlines available"))
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
                    let entries = self.entries_with_depths(cx);
                    uniform_list(cx.view().clone(), "entries", entries.len(), {
                        move |outline_panel, range, cx| {
                            outline_panel.last_visible_range = range.clone();
                            outline_panel.fetch_outlines(&range, cx);
                            entries
                                .get(range)
                                .into_iter()
                                .flatten()
                                .map(|(depth, dipslayed_item)| match dipslayed_item {
                                    EntryOwned::Entry(entry) => {
                                        outline_panel.render_entry(entry, *depth, cx)
                                    }
                                    EntryOwned::FoldedDirs(worktree_id, entries) => outline_panel
                                        .render_folded_dirs(*worktree_id, entries, *depth, cx),
                                    EntryOwned::Outline(container, outline) => outline_panel
                                        .render_outline(*container, outline, *depth, cx),
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
) -> Option<Subscription> {
    if OutlinePanelSettings::get_global(cx).auto_reveal_entries {
        let debounce = Some(Duration::from_millis(UPDATE_DEBOUNCE_MILLIS));
        Some(cx.subscribe(
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
                EditorEvent::ExcerptsRemoved { .. } => {
                    outline_panel.update_fs_entries(
                        &editor,
                        HashSet::default(),
                        None,
                        debounce,
                        false,
                        cx,
                    );
                }
                EditorEvent::ExcerptsExpanded { .. } => {
                    outline_panel.update_fs_entries(
                        &editor,
                        HashSet::default(),
                        None,
                        debounce,
                        true,
                        cx,
                    );
                }
                EditorEvent::Reparsed => {
                    outline_panel.outline_fetch_tasks.clear();
                    outline_panel.outlines.clear();
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
        ))
    } else {
        None
    }
}

fn range_contains(
    range: &Range<language::Anchor>,
    anchor: language::Anchor,
    buffer_snapshot: &language::BufferSnapshot,
) -> bool {
    range.start.cmp(&anchor, buffer_snapshot).is_le()
        && range.end.cmp(&anchor, buffer_snapshot).is_ge()
}
