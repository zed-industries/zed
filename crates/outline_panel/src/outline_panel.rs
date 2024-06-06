mod outline_panel_settings;

use std::{
    cmp,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use db::kvp::KEY_VALUE_STORE;
use editor::{
    items::{entry_git_aware_label_color, entry_label_color},
    Editor, EditorEvent, ExcerptId,
};
use file_icons::FileIcons;
use git::repository::GitFileStatus;
use gpui::{
    actions, anchored, deferred, div, px, uniform_list, Action, AnyElement, AppContext,
    AssetSource, AsyncWindowContext, ClipboardItem, DismissEvent, Div, ElementId, EntityId,
    EventEmitter, FocusHandle, FocusableView, InteractiveElement, IntoElement, KeyContext, Model,
    MouseButton, MouseDownEvent, ParentElement, Pixels, Point, Render, SharedString, Stateful,
    StatefulInteractiveElement, Styled, Subscription, Task, UniformListScrollHandle, View,
    ViewContext, VisualContext, WeakView, WindowContext,
};
use language::{OffsetRangeExt, OutlineItem, ToOffset};
use menu::{SelectFirst, SelectLast, SelectNext, SelectPrev};

use outline_panel_settings::{OutlinePanelDockPosition, OutlinePanelSettings};
use project::{EntryKind, Fs, Project};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use unicase::UniCase;
use util::{debug_panic, maybe, NumericPrefixWithSuffix, ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    item::ItemHandle,
    ui::{
        h_flex, v_flex, ActiveTheme, Color, ContextMenu, FluentBuilder, Icon, IconName, IconSize,
        Label, LabelCommon, ListItem, Selectable, Tooltip,
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

pub struct OutlinePanel {
    fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    project: Model<Project>,
    scroll_handle: UniformListScrollHandle,
    context_menu: Option<(View<ContextMenu>, Point<Pixels>, Subscription)>,
    workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
    pending_serialization: Task<Option<()>>,
    visible_entries: Vec<OutlinePanelEntry>,
    // TODO kb has to include files with outlines later?
    expanded_dir_ids: HashMap<WorktreeId, BTreeSet<ProjectEntryId>>,
    unfolded_dir_ids: HashSet<ProjectEntryId>,
    // Currently selected entry in a file tree
    selected_entry: Option<OutlinePanelEntry>,
    displayed_item: Option<DisplayedActiveItem>,
    _subscriptions: Vec<Subscription>,
}

#[derive(Clone, Debug, Eq, Hash)]
enum OutlinePanelEntry {
    ExternalFile(ExcerptId, Option<PathBuf>),
    Directory(WorktreeId, Entry),
    File(ExcerptId, WorktreeId, Entry),
    Outline(ExcerptId, OutlineItem<language::Anchor>),
}

impl PartialEq for OutlinePanelEntry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ExternalFile(id_a, path_a), Self::ExternalFile(id_b, path_b)) => {
                path_a == path_b && id_a == id_b
            }
            (Self::Directory(id_a, entry_a), Self::Directory(id_b, entry_b)) => {
                id_a == id_b && entry_a.id == entry_b.id
            }
            (Self::File(id_a, worktree_a, entry_a), Self::File(id_b, worktree_b, entry_b)) => {
                id_a == id_b && worktree_a == worktree_b && entry_a.id == entry_b.id
            }
            (Self::Outline(id_a, item_a), Self::Outline(id_b, item_b)) => {
                id_a == id_b && item_a == item_b
            }
            _ => false,
        }
    }
}
impl OutlinePanelEntry {
    fn abs_path(&self, project: &Model<Project>, cx: &AppContext) -> Option<PathBuf> {
        match self {
            Self::ExternalFile(_, path) => path.clone(),
            Self::Directory(worktree_id, entry) => project
                .read(cx)
                .worktree_for_id(*worktree_id, cx)?
                .read(cx)
                .absolutize(&entry.path)
                .ok(),
            Self::File(_, worktree_id, entry) => project
                .read(cx)
                .worktree_for_id(*worktree_id, cx)?
                .read(cx)
                .absolutize(&entry.path)
                .ok(),
            Self::Outline(..) => None,
        }
    }

    fn relative_path(&self, _: &AppContext) -> Option<&Path> {
        match self {
            Self::ExternalFile(_, path) => path.as_deref(),
            Self::Directory(_, entry) => Some(entry.path.as_ref()),
            Self::File(_, _, entry) => Some(entry.path.as_ref()),
            Self::Outline(..) => None,
        }
    }
}

struct DisplayedActiveItem {
    item_id: EntityId,
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
                            outline_panel.displayed_item = None;
                            outline_panel.visible_entries.clear();
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
                visible_entries: Vec::new(),
                expanded_dir_ids: HashMap::default(),
                unfolded_dir_ids: Default::default(),
                selected_entry: None,
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
            if let Some(editor) = workspace
                .active_item(cx)
                .and_then(|item| item.act_as::<Editor>(cx))
            {
                outline_panel.update_visible_entries(&editor, HashSet::default(), None, cx);
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
        let Some(editor) = self.workspace.upgrade().and_then(|workspace| {
            workspace
                .read(cx)
                .active_item(cx)
                .and_then(|item| item.act_as::<Editor>(cx))
        }) else {
            return;
        };
        if let Some(OutlinePanelEntry::Directory(_, selected_entry)) = &self.selected_entry {
            self.unfolded_dir_ids.insert(selected_entry.id);
            self.update_visible_entries(&editor, HashSet::default(), None, cx);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn fold_directory(&mut self, _: &FoldDirectory, cx: &mut ViewContext<Self>) {
        if let Some(selected_dir @ OutlinePanelEntry::Directory(..)) = &self.selected_entry {
            let Some(editor) = self.workspace.upgrade().and_then(|workspace| {
                workspace
                    .read(cx)
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
            }) else {
                return;
            };

            for folded_dir_entry_id in self
                .visible_entries
                .iter()
                .skip_while(|entry| entry != &selected_dir)
                .take_while(|entry| matches!(entry, OutlinePanelEntry::Directory(..)))
                .filter_map(|entry| {
                    if let OutlinePanelEntry::Directory(_, entry) = entry {
                        Some(entry.id)
                    } else {
                        None
                    }
                })
            {
                self.unfolded_dir_ids.remove(&folded_dir_entry_id);
            }

            self.update_visible_entries(&editor, HashSet::default(), None, cx);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if let Some(selection) = &self.selected_entry {
            let mut entries = self
                .visible_entries
                .iter()
                .skip_while(|entry| entry != &selection)
                .fuse();
            let _current_entry = entries.next();
            if let Some(next_entry) = entries.next() {
                self.selected_entry = Some(next_entry.clone());
                self.autoscroll(cx);
                cx.notify();
            }
        } else {
            self.select_first(&SelectFirst {}, cx);
        }
    }

    fn select_parent(&mut self, _: &SelectParent, cx: &mut ViewContext<Self>) {
        if let Some(selection) = &self.selected_entry {
            let parent_entry = self
                .visible_entries
                .iter()
                .rev()
                .skip_while(|entry| entry != &selection)
                .skip(1)
                .find(|entry| match selection {
                    OutlinePanelEntry::ExternalFile(..) => false,
                    OutlinePanelEntry::Directory(worktree_id, child_directory_entry) => {
                        if let OutlinePanelEntry::Directory(
                            directory_worktree_id,
                            directory_entry,
                        ) = entry
                        {
                            directory_worktree_id == worktree_id
                                && directory_contains(directory_entry, child_directory_entry)
                        } else {
                            false
                        }
                    }
                    OutlinePanelEntry::File(_, worktree_id, file_entry) => {
                        if let OutlinePanelEntry::Directory(
                            directory_worktree_id,
                            directory_entry,
                        ) = entry
                        {
                            directory_worktree_id == worktree_id
                                && directory_contains(directory_entry, file_entry)
                        } else {
                            false
                        }
                    }
                    OutlinePanelEntry::Outline(excerpt_id, _) => {
                        if let OutlinePanelEntry::File(file_excerpt_id, _, _) = entry {
                            excerpt_id == file_excerpt_id
                        } else {
                            false
                        }
                    }
                });

            if let Some(parent_entry) = parent_entry {
                self.selected_entry = Some(parent_entry.clone());
                self.autoscroll(cx);
                cx.notify();
            }
        } else {
            self.select_first(&SelectFirst {}, cx);
        }
    }

    fn select_first(&mut self, _: &SelectFirst, cx: &mut ViewContext<Self>) {
        if let Some(first_entry) = self.visible_entries.first() {
            self.selected_entry = Some(first_entry.clone());
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &SelectLast, cx: &mut ViewContext<Self>) {
        if let Some(last_entry) = self.visible_entries.last() {
            self.selected_entry = Some(last_entry.clone());
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn autoscroll(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(selected_entry) = &self.selected_entry {
            if let Some(index) = self
                .visible_entries
                .iter()
                .position(|entry| entry == selected_entry)
            {
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
        entry: &OutlinePanelEntry,
        cx: &mut ViewContext<Self>,
    ) {
        let project = self.project.read(cx);

        self.selected_entry = Some(entry.clone());
        let auto_fold_dirs = OutlinePanelSettings::get_global(cx).auto_fold_dirs;
        let is_foldable = auto_fold_dirs && self.is_foldable(entry);
        let is_unfoldable = auto_fold_dirs && self.is_unfoldable(entry);
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
                },
            )
        });

        cx.focus_view(&context_menu);
        let subscription = cx.subscribe(&context_menu, |outline_panel, _, _: &DismissEvent, cx| {
            outline_panel.context_menu.take();
            cx.notify();
        });
        self.context_menu = Some((context_menu, position, subscription));
        cx.notify();
    }

    fn is_unfoldable(&self, entry: &OutlinePanelEntry) -> bool {
        if let OutlinePanelEntry::Directory(_, entry) = entry {
            !self.unfolded_dir_ids.contains(&entry.id)
        } else {
            false
        }
    }

    fn is_foldable(&self, entry: &OutlinePanelEntry) -> bool {
        if let OutlinePanelEntry::Directory(_, entry) = entry {
            self.unfolded_dir_ids.contains(&entry.id)
        } else {
            false
        }
    }

    fn expand_selected_entry(&mut self, _: &ExpandSelectedEntry, cx: &mut ViewContext<Self>) {
        if let Some(OutlinePanelEntry::Directory(worktree_id, selected_dir_entry)) =
            &self.selected_entry
        {
            let expanded_dir_ids =
                if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(worktree_id) {
                    expanded_dir_ids
                } else {
                    return;
                };

            let entry_id = selected_dir_entry.id;
            if expanded_dir_ids.insert(entry_id) {
                let Some(editor) = self.workspace.upgrade().and_then(|workspace| {
                    workspace
                        .read(cx)
                        .active_item(cx)
                        .and_then(|item| item.act_as::<Editor>(cx))
                }) else {
                    return;
                };
                self.project.update(cx, |project, cx| {
                    project.expand_entry(*worktree_id, entry_id, cx);
                });
                self.update_visible_entries(&editor, HashSet::default(), None, cx);
                cx.notify()
            } else {
                self.select_next(&SelectNext, cx)
            }
        }
    }

    fn collapse_selected_entry(&mut self, _: &CollapseSelectedEntry, cx: &mut ViewContext<Self>) {
        if let Some(dir_entry @ OutlinePanelEntry::Directory(worktree_id, selected_dir_entry)) =
            &self.selected_entry
        {
            let expanded_dir_ids =
                if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(worktree_id) {
                    expanded_dir_ids
                } else {
                    return;
                };
            let Some(editor) = self.workspace.upgrade().and_then(|workspace| {
                workspace
                    .read(cx)
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
            }) else {
                return;
            };

            let entry_id = selected_dir_entry.id;
            expanded_dir_ids.remove(&entry_id);
            self.update_visible_entries(&editor, HashSet::default(), Some(dir_entry.clone()), cx);
            cx.notify();
        }
    }

    pub fn collapse_all_entries(&mut self, _: &CollapseAllEntries, cx: &mut ViewContext<Self>) {
        let Some(editor) = self.workspace.upgrade().and_then(|workspace| {
            workspace
                .read(cx)
                .active_item(cx)
                .and_then(|item| item.act_as::<Editor>(cx))
        }) else {
            return;
        };
        // By keeping entries for fully collapsed worktrees, we avoid expanding them within update_visible_entries
        // (which is it's default behaviour when there's no entry for a worktree in expanded_dir_ids).
        self.expanded_dir_ids
            .retain(|_, expanded_entries| expanded_entries.is_empty());
        self.update_visible_entries(&editor, HashSet::default(), None, cx);
        cx.notify();
    }

    fn toggle_expanded(&mut self, entry_id: ProjectEntryId, cx: &mut ViewContext<Self>) {
        if let Some(worktree_id) = self.project.read(cx).worktree_id_for_entry(entry_id, cx) {
            let Some(dir_entry_to_toggle) = self.visible_entries.iter().find(|entry| {
                if let OutlinePanelEntry::Directory(directory_worktree_id, directory_entry) = entry
                {
                    directory_worktree_id == &worktree_id && directory_entry.id == entry_id
                } else {
                    false
                }
            }) else {
                return;
            };

            if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(&worktree_id) {
                let Some(editor) = self.workspace.upgrade().and_then(|workspace| {
                    workspace
                        .read(cx)
                        .active_item(cx)
                        .and_then(|item| item.act_as::<Editor>(cx))
                }) else {
                    return;
                };

                self.project.update(cx, |project, cx| {
                    if !expanded_dir_ids.remove(&entry_id) {
                        project.expand_entry(worktree_id, entry_id, cx);
                        expanded_dir_ids.insert(entry_id);
                    }
                });
                self.update_visible_entries(
                    &editor,
                    HashSet::default(),
                    Some(dir_entry_to_toggle.clone()),
                    cx,
                );
                cx.focus(&self.focus_handle);
                cx.notify();
            }
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if let Some(selection) = &self.selected_entry {
            if let Some(previous_entry) = self
                .visible_entries
                .iter()
                .rev()
                .skip_while(|entry| entry != &selection)
                .skip(1)
                .next()
            {
                self.selected_entry = Some(previous_entry.clone());
                self.autoscroll(cx);
                cx.notify();
            }
        } else {
            self.select_first(&SelectFirst {}, cx);
        }
    }

    fn copy_path(&mut self, _: &CopyPath, cx: &mut ViewContext<Self>) {
        if let Some(clipboard_text) = self.selected_entry.as_ref().and_then(|entry| {
            entry
                .abs_path(&self.project, cx)
                .map(|p| p.to_string_lossy().to_string())
        }) {
            cx.write_to_clipboard(ClipboardItem::new(clipboard_text));
        }
    }

    fn copy_relative_path(&mut self, _: &CopyRelativePath, cx: &mut ViewContext<Self>) {
        if let Some(clipboard_text) = self.selected_entry.as_ref().and_then(|entry| {
            entry
                .relative_path(cx)
                .map(|p| p.to_string_lossy().to_string())
        }) {
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
        if let Some((selected_entry, abs_path)) = self
            .selected_entry
            .as_ref()
            .and_then(|entry| Some((entry, entry.abs_path(&self.project, cx)?)))
        {
            let working_directory = match selected_entry {
                OutlinePanelEntry::File(..) | OutlinePanelEntry::ExternalFile(..) => {
                    abs_path.parent().map(|p| p.to_owned())
                }
                OutlinePanelEntry::Directory(..) => Some(abs_path),
                OutlinePanelEntry::Outline(..) => None,
            };
            if let Some(working_directory) = working_directory {
                cx.dispatch_action(workspace::OpenTerminal { working_directory }.boxed_clone())
            }
        }
    }

    fn reveal_entry(
        &mut self,
        editor: &View<Editor>,
        project: Model<Project>,
        excerpt_id: ExcerptId,
        outline_item: Option<OutlineItem<language::Anchor>>,
        cx: &mut ViewContext<'_, Self>,
    ) {
        let file_entry_to_expand = match &outline_item {
            Some(outline_item) => self
                .visible_entries
                .iter()
                .rev()
                .skip_while(|entry| {
                    if let OutlinePanelEntry::Outline(visible_excerpt_id, visible_outline_item) =
                        entry
                    {
                        visible_excerpt_id != &excerpt_id || visible_outline_item != outline_item
                    } else {
                        true
                    }
                })
                .skip(1)
                .find(|entry| match entry {
                    OutlinePanelEntry::ExternalFile(file_excerpt_id, _)
                    | OutlinePanelEntry::File(file_excerpt_id, _, _) => {
                        file_excerpt_id == &excerpt_id
                    }
                    _ => false,
                }),
            None => self.visible_entries.iter().find(|entry| match entry {
                OutlinePanelEntry::ExternalFile(file_excerpt_id, _)
                | OutlinePanelEntry::File(file_excerpt_id, _, _) => file_excerpt_id == &excerpt_id,
                _ => false,
            }),
        };
        let Some(entry_to_select) = outline_item
            .map(|outline| OutlinePanelEntry::Outline(excerpt_id, outline))
            .or_else(|| file_entry_to_expand.cloned())
        else {
            return;
        };

        if self.selected_entry.as_ref() == Some(&entry_to_select) {
            return;
        }

        if let Some(OutlinePanelEntry::File(_, file_worktree_id, file_entry)) = file_entry_to_expand
        {
            if let Some(worktree) = project.read(cx).worktree_for_id(*file_worktree_id, cx) {
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

        self.update_visible_entries(&editor, HashSet::default(), Some(entry_to_select), cx);
        self.autoscroll(cx);
        cx.notify();
    }

    fn expand_entry(
        &mut self,
        worktree_id: WorktreeId,
        entry_id: ProjectEntryId,
        cx: &mut AppContext,
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
        rendered_entry: &OutlinePanelEntry,
        depth: usize,
        cx: &mut ViewContext<Self>,
    ) -> Stateful<Div> {
        let settings = OutlinePanelSettings::get_global(cx);
        let is_active = self.selected_entry.as_ref() == Some(rendered_entry);
        let (item_id, name, text_color, icon) = match rendered_entry {
            OutlinePanelEntry::File(_, worktree_id, entry) => {
                let name = self.entry_name(worktree_id, entry, cx);
                let color =
                    entry_git_aware_label_color(entry.git_status, entry.is_ignored, is_active);
                let icon = if settings.file_icons {
                    FileIcons::get_icon(&entry.path, cx)
                } else {
                    None
                }
                .map(Icon::from_path);
                (
                    ElementId::from(entry.id.to_proto() as usize),
                    name,
                    color,
                    icon,
                )
            }
            OutlinePanelEntry::Directory(worktree_id, entry) => {
                let name = self.entry_name(worktree_id, entry, cx);

                let is_expanded = self
                    .expanded_dir_ids
                    .get(worktree_id)
                    .map_or(false, |ids| ids.contains(&entry.id));
                let color =
                    entry_git_aware_label_color(entry.git_status, entry.is_ignored, is_active);
                let icon = if settings.folder_icons {
                    FileIcons::get_folder_icon(is_expanded, cx)
                } else {
                    FileIcons::get_chevron_icon(is_expanded, cx)
                }
                .map(Icon::from_path);
                (
                    ElementId::from(entry.id.to_proto() as usize),
                    name,
                    color,
                    icon,
                )
            }
            OutlinePanelEntry::ExternalFile(excerpt_id, file) => {
                let name = file.as_ref().map_or_else(
                    || "Untitled".to_string(),
                    |file_path| {
                        file_path
                            .file_name()
                            .unwrap_or_else(|| file_path.as_os_str())
                            .to_string_lossy()
                            .to_string()
                    },
                );
                let color = entry_label_color(is_active);
                let icon = if settings.file_icons {
                    file.as_deref()
                        .and_then(|path| FileIcons::get_icon(path, cx))
                } else {
                    None
                }
                .map(Icon::from_path);
                (
                    ElementId::from(excerpt_id.to_proto() as usize),
                    name,
                    color,
                    icon,
                )
            }
            OutlinePanelEntry::Outline(excerpt_id, outline) => {
                let name = outline.text.clone();
                let color = entry_label_color(is_active);
                (
                    ElementId::from(SharedString::from(format!(
                        "{:?}|{}",
                        excerpt_id, &outline.text,
                    ))),
                    name,
                    color,
                    None,
                )
            }
        };

        let clicked_entry = rendered_entry.clone();
        div()
            .id(item_id.clone())
            .child(
                ListItem::new(item_id)
                    .indent_level(depth)
                    .indent_step_size(px(settings.indent_size))
                    .selected(is_active)
                    .child(if let Some(icon) = icon {
                        h_flex().child(icon.color(text_color))
                    } else {
                        h_flex()
                            .size(IconSize::default().rems())
                            .invisible()
                            .flex_none()
                    })
                    .child(
                        h_flex()
                            .h_6()
                            .child(Label::new(name).single_line().color(text_color))
                            .ml_1(),
                    )
                    .on_click(
                        cx.listener(move |outline_panel, event: &gpui::ClickEvent, cx| {
                            if event.down.button == MouseButton::Right || event.down.first_mouse {
                                return;
                            }

                            // TODO kb
                            match &clicked_entry {
                                OutlinePanelEntry::ExternalFile(excerpt_id, _) => {}
                                OutlinePanelEntry::Directory(_, directory_entry) => {}
                                OutlinePanelEntry::File(excerpt_id, _, file_entry) => {}
                                OutlinePanelEntry::Outline(excerpt_id, outline) => {}
                            }
                            // if kind.is_dir() {
                            //     outline_panel.toggle_expanded(entry_id, cx);
                            // } else if outline_panel
                            //     .displayed_item
                            //     .as_ref()
                            //     .filter(|item| !item.entries.is_empty())
                            //     .is_some()
                            // {
                            //     if let Some(active_editor) = outline_panel.active_editor(cx) {
                            //         let active_multi_buffer =
                            //             active_editor.read(cx).buffer().clone();
                            //         let multi_buffer_snapshot =
                            //             active_multi_buffer.read(cx).snapshot(cx);
                            //         let scroll_target = outline_panel
                            //             .project
                            //             .update(cx, |project, cx| {
                            //                 project
                            //                     .path_for_entry(entry_id, cx)
                            //                     .and_then(|path| project.get_open_buffer(&path, cx))
                            //             })
                            //             .map(|buffer| {
                            //                 active_multi_buffer
                            //                     .read(cx)
                            //                     .excerpts_for_buffer(&buffer, cx)
                            //             })
                            //             .and_then(|excerpts| {
                            //                 let (excerpt_id, excerpt_range) = excerpts.first()?;
                            //                 multi_buffer_snapshot.anchor_in_excerpt(
                            //                     *excerpt_id,
                            //                     excerpt_range.context.start,
                            //                 )
                            //             });
                            //         if let Some(anchor) = scroll_target {
                            //             outline_panel.selected_entry = Some(SelectedEntry {
                            //                 worktree_id,
                            //                 entry_id,
                            //             });
                            //             active_editor.update(cx, |editor, cx| {
                            //                 editor.set_scroll_anchor(
                            //                     ScrollAnchor {
                            //                         offset: Point::new(
                            //                             0.0,
                            //                             -(editor.file_header_size() as f32),
                            //                         ),
                            //                         anchor,
                            //                     },
                            //                     cx,
                            //                 );
                            //             })
                            //         }
                            //     }
                            // }
                        }),
                    )
                    .on_secondary_mouse_down(cx.listener(
                        move |outline_panel, event: &MouseDownEvent, cx| {
                            // Stop propagation to prevent the catch-all context menu for the project
                            // panel from being deployed.
                            cx.stop_propagation();
                            if let Some(selection) = outline_panel
                                .selected_entry
                                .as_ref()
                                .or_else(|| outline_panel.visible_entries.last())
                                .cloned()
                            {
                                outline_panel.deploy_context_menu(event.position, &selection, cx);
                            }
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

    fn update_visible_entries(
        &mut self,
        active_editor: &View<Editor>,
        new_entries: HashSet<ExcerptId>,
        new_selected_entry: Option<OutlinePanelEntry>,
        cx: &mut ViewContext<Self>,
    ) -> Option<()> {
        self.visible_entries.clear();

        let auto_collapse_dirs = OutlinePanelSettings::get_global(cx).auto_fold_dirs;
        let project = self.project.read(cx);
        let displayed_multi_buffer = active_editor.read(cx).buffer().clone();
        let multi_buffer_snapshot = displayed_multi_buffer.read(cx).snapshot(cx);
        let mut new_workspace_entries = BTreeMap::<WorktreeId, HashSet<&Entry>>::new();
        let mut outline_entries = HashMap::default();
        let mut non_project_entries = Vec::default();
        let mut project_entries_excerpts = HashMap::default();
        for (excerpt_id, buffer_snapshot, excerpt_range) in multi_buffer_snapshot.excerpts() {
            let is_new = new_entries.contains(&excerpt_id);
            let mut outlines = buffer_snapshot
                .outline(None)
                .map(|outline| outline.items)
                .unwrap_or_default();
            outlines.retain(|outline| {
                range_contains(&excerpt_range.context, outline.range.start, buffer_snapshot)
                    || range_contains(&excerpt_range.context, outline.range.end, buffer_snapshot)
            });
            if !outlines.is_empty() {
                outline_entries.insert(excerpt_id, outlines);
            }

            if let Some(file) = project::File::from_dyn(buffer_snapshot.file()) {
                let worktree = file.worktree.read(cx);
                let worktree_snapshot = worktree.snapshot();
                let expanded_dir_ids = self.expanded_dir_ids.entry(worktree.id()).or_default();

                match file
                    .entry_id
                    .and_then(|project_id| worktree.entry_for_id(project_id))
                {
                    Some(entry) => {
                        let mut traversal =
                            worktree.traverse_from_path(true, true, true, entry.path.as_ref());

                        let mut current_entry = entry;
                        loop {
                            if entry.is_dir() {
                                if auto_collapse_dirs && !self.unfolded_dir_ids.contains(&entry.id)
                                {
                                    if let Some(root_path) = worktree_snapshot.root_entry() {
                                        let mut child_entries =
                                            worktree_snapshot.child_entries(&entry.path);
                                        if let Some(child) = child_entries.next() {
                                            if entry.path != root_path.path
                                                && child_entries.next().is_none()
                                                && child.kind.is_dir()
                                            {
                                                if traversal.back_to_parent() {
                                                    if let Some(parent_entry) = traversal.entry() {
                                                        current_entry = parent_entry;
                                                        continue;
                                                    }
                                                } else {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }

                                if is_new || worktree_snapshot.root_entry() == Some(entry) {
                                    expanded_dir_ids.insert(entry.id);
                                } else if !expanded_dir_ids.contains(&entry.id) {
                                    break;
                                }
                            }

                            if new_workspace_entries
                                .entry(worktree.id())
                                .or_default()
                                .insert(current_entry)
                            {
                                project_entries_excerpts.insert(current_entry.id, excerpt_id);
                                if traversal.back_to_parent() {
                                    if let Some(parent_entry) = traversal.entry() {
                                        current_entry = parent_entry;
                                        continue;
                                    }
                                }
                            }
                            break;
                        }
                    }
                    None => {
                        if let Some(abs_path) = file.worktree.read(cx).absolutize(&file.path).ok() {
                            non_project_entries
                                .push(OutlinePanelEntry::ExternalFile(excerpt_id, Some(abs_path)));
                        }
                    }
                }
            } else {
                non_project_entries.push(OutlinePanelEntry::ExternalFile(excerpt_id, None));
            }
        }

        non_project_entries.sort_by(|entry_a, entry_b| match (entry_a, entry_b) {
            (
                OutlinePanelEntry::ExternalFile(excerpt_a, path_a),
                OutlinePanelEntry::ExternalFile(excerpt_b, path_b),
            ) => path_a
                .cmp(&path_b)
                .then(excerpt_a.cmp(&excerpt_b, &multi_buffer_snapshot)),
            (OutlinePanelEntry::ExternalFile(..), _) => cmp::Ordering::Less,
            (_, OutlinePanelEntry::ExternalFile(..)) => cmp::Ordering::Greater,
            _ => cmp::Ordering::Equal,
        });

        let worktree_entries =
            new_workspace_entries
                .into_iter()
                .filter_map(|(worktree_id, entries)| {
                    let worktree_snapshot = project
                        .worktree_for_id(worktree_id, cx)?
                        .read(cx)
                        .snapshot();
                    let mut entries = entries.into_iter().cloned().collect::<Vec<_>>();
                    entries.sort_by(|entry_a, entry_b| {
                        let mut components_a = entry_a.path.components().peekable();
                        let mut components_b = entry_b.path.components().peekable();
                        loop {
                            match (components_a.next(), components_b.next()) {
                                (Some(component_a), Some(component_b)) => {
                                    let a_is_file =
                                        components_a.peek().is_none() && entry_a.is_file();
                                    let b_is_file =
                                        components_b.peek().is_none() && entry_b.is_file();
                                    let ordering = a_is_file.cmp(&b_is_file).then_with(|| {
                                        let maybe_numeric_ordering = maybe!({
                                            let num_and_remainder_a = Path::new(
                                                component_a.as_os_str(),
                                            )
                                            .file_stem()
                                            .and_then(|s| s.to_str())
                                            .and_then(
                                                NumericPrefixWithSuffix::from_numeric_prefixed_str,
                                            )?;
                                            let num_and_remainder_b = Path::new(
                                                component_b.as_os_str(),
                                            )
                                            .file_stem()
                                            .and_then(|s| s.to_str())
                                            .and_then(
                                                NumericPrefixWithSuffix::from_numeric_prefixed_str,
                                            )?;

                                            num_and_remainder_a.partial_cmp(&num_and_remainder_b)
                                        });

                                        maybe_numeric_ordering.unwrap_or_else(|| {
                                            let name_a = UniCase::new(
                                                component_a.as_os_str().to_string_lossy(),
                                            );
                                            let name_b = UniCase::new(
                                                component_b.as_os_str().to_string_lossy(),
                                            );

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
                    worktree_snapshot.propagate_git_statuses(&mut entries);
                    Some((worktree_id, entries))
                });

        let mut worktree_items_with_outlines = Vec::new();
        for (worktree_id, entries) in worktree_entries {
            for entry in entries {
                if entry.is_dir() {
                    worktree_items_with_outlines
                        .push(OutlinePanelEntry::Directory(worktree_id, entry))
                } else if let Some(excerpt_id) = project_entries_excerpts.remove(&entry.id) {
                    worktree_items_with_outlines.push(OutlinePanelEntry::File(
                        excerpt_id,
                        worktree_id,
                        entry,
                    ))
                }
            }
        }
        self.visible_entries = non_project_entries
            .into_iter()
            .chain(worktree_items_with_outlines.into_iter())
            .flat_map(|entry| {
                let excerpt_id = match &entry {
                    OutlinePanelEntry::ExternalFile(excerpt_id, _) => Some(*excerpt_id),
                    OutlinePanelEntry::File(excerpt_id, _, _) => Some(*excerpt_id),
                    OutlinePanelEntry::Directory(..) => None,
                    OutlinePanelEntry::Outline(..) => None,
                };
                let outlines =
                    excerpt_id.and_then(|excerpt_id| outline_entries.remove(&excerpt_id));
                Some(entry)
                    .into_iter()
                    .chain(outlines.into_iter().flatten().flat_map(move |outline| {
                        Some(OutlinePanelEntry::Outline(excerpt_id?, outline))
                    }))
            })
            .collect();

        if new_selected_entry.is_some() {
            self.selected_entry = new_selected_entry;
        }

        Some(())
    }

    fn replace_visible_entries(
        &mut self,
        new_active_editor: View<Editor>,
        cx: &mut ViewContext<Self>,
    ) {
        let new_entries =
            HashSet::from_iter(new_active_editor.read(cx).buffer().read(cx).excerpt_ids());
        self.displayed_item = Some(DisplayedActiveItem {
            item_id: new_active_editor.item_id(),
            _editor_subscrpiption: subscribe_for_editor_events(&new_active_editor, cx),
        });
        self.update_visible_entries(&new_active_editor, new_entries, None, cx);
        cx.notify();
    }
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

fn directory_contains(directory_entry: &Entry, chld_entry: &Entry) -> bool {
    debug_assert!(directory_entry.is_dir());
    let Some(relative_path) = chld_entry.path.strip_prefix(&directory_entry.path).ok() else {
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
                        // When deploying the context menu anywhere below the last project entry,
                        // act as if the user clicked the last visible element (as most of the empty space to click on is below).
                        if let Some(entry) = outline_panel
                            .selected_entry
                            .as_ref()
                            .or_else(|| outline_panel.visible_entries.last())
                            .cloned()
                        {
                            outline_panel.deploy_context_menu(event.position, &entry, cx);
                        }
                    }),
                )
                .track_focus(&self.focus_handle)
                .child(
                    uniform_list(cx.view().clone(), "entries", self.visible_entries.len(), {
                        |outline_panel, range, cx| {
                            let mut depth = 0;
                            let mut previous_entry = None::<&OutlinePanelEntry>;
                            outline_panel
                                .visible_entries
                                .iter()
                                .enumerate()
                                .filter_map(|(i, visible_item)| {
                                    if let OutlinePanelEntry::File(_, _, entry) = &visible_item {
                                        if entry
                                            .path
                                            .to_string_lossy()
                                            .to_string()
                                            .ends_with("main.rs")
                                        {
                                            dbg!(
                                                "!!!!!!!!!!!!!!!!!!!!!!!",
                                                &previous_entry,
                                                depth,
                                                &visible_item
                                            );
                                        }
                                    }

                                    match (previous_entry, visible_item) {
                                        (None, _) => {}

                                        (
                                            Some(OutlinePanelEntry::Directory(..)),
                                            OutlinePanelEntry::File(..),
                                        ) => depth += 1,

                                        (
                                            Some(OutlinePanelEntry::File(..)),
                                            OutlinePanelEntry::Outline(..),
                                        ) => depth += 1,
                                        (
                                            Some(OutlinePanelEntry::File(..)),
                                            OutlinePanelEntry::File(..),
                                        ) => {}

                                        (
                                            Some(OutlinePanelEntry::ExternalFile(..)),
                                            OutlinePanelEntry::Outline(..),
                                        ) => depth += 1,
                                        (Some(OutlinePanelEntry::ExternalFile(..)), _) => {}
                                        (Some(_), OutlinePanelEntry::ExternalFile(..)) => depth = 0,

                                        (
                                            Some(OutlinePanelEntry::Outline(_, previous_outline)),
                                            OutlinePanelEntry::Outline(_, outline),
                                        ) => match previous_outline.depth.cmp(&outline.depth) {
                                            cmp::Ordering::Less => depth += 1,
                                            cmp::Ordering::Greater => depth -= 1,
                                            cmp::Ordering::Equal => {}
                                        },
                                        // TODO kb next two are wrong, need to keep previous dir's depth?
                                        (
                                            Some(OutlinePanelEntry::Outline(..)),
                                            OutlinePanelEntry::Directory(..),
                                        ) => {
                                            depth -= 1;
                                        }
                                        (
                                            Some(OutlinePanelEntry::Outline(outline_excerpt_id, _)),
                                            OutlinePanelEntry::File(excerpt_id, ..)
                                            | OutlinePanelEntry::ExternalFile(excerpt_id, _),
                                        ) => {
                                            if excerpt_id == outline_excerpt_id {
                                                depth -= 1;
                                            } else {
                                                depth = 0;
                                            }
                                        }
                                        (
                                            Some(OutlinePanelEntry::Directory(..)),
                                            OutlinePanelEntry::Outline(..),
                                        ) => {
                                            debug_panic!(
                                                "Unexpected: outlines after a directory entry"
                                            );
                                            depth += 1;
                                        }

                                        (
                                            Some(OutlinePanelEntry::Directory(
                                                _,
                                                previous_directory,
                                            )),
                                            OutlinePanelEntry::Directory(_, directory),
                                        ) => {
                                            if directory.path.starts_with(&previous_directory.path)
                                            {
                                                depth += 1;
                                            } else {
                                                match directory.path.parent() {
                                                    Some(parent_path) => {
                                                        if !previous_directory
                                                            .path
                                                            .starts_with(parent_path)
                                                        {
                                                            depth -= 1;
                                                        }
                                                    }
                                                    None => depth = 0,
                                                }
                                            }
                                        }
                                        (
                                            Some(OutlinePanelEntry::File(_, _, file_entry)),
                                            OutlinePanelEntry::Directory(_, directory_entry),
                                        ) => {
                                            if let Some(file_parent) = file_entry.path.parent() {
                                                if !directory_entry.path.starts_with(file_parent) {
                                                    depth -= 1;
                                                }
                                            } else {
                                                depth -= 1;
                                            }
                                        }
                                    }
                                    previous_entry = Some(visible_item);

                                    if range.contains(&i) {
                                        Some(outline_panel.render_entry(visible_item, depth, cx))
                                    } else {
                                        None
                                    }
                                })
                                .collect()
                        }
                    })
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

fn subscribe_for_editor_events(
    editor: &View<Editor>,
    cx: &mut ViewContext<OutlinePanel>,
) -> Option<Subscription> {
    if OutlinePanelSettings::get_global(cx).auto_reveal_entries {
        Some(cx.subscribe(
            editor,
            |outline_panel, editor, e: &EditorEvent, cx| match e {
                EditorEvent::SelectionsChanged { local: true } => {
                    let (excerpt_id, outline_item) = location_for_selection(&editor, cx);
                    outline_panel.reveal_entry(
                        &editor,
                        outline_panel.project.clone(),
                        excerpt_id,
                        outline_item,
                        cx,
                    );
                    cx.notify();
                }
                EditorEvent::ExcerptsAdded { excerpts, .. } => {
                    outline_panel.update_visible_entries(
                        &editor,
                        excerpts.iter().map(|&(excerpt_id, _)| excerpt_id).collect(),
                        None,
                        cx,
                    );
                    cx.notify();
                }
                EditorEvent::ExcerptsRemoved { .. } => {
                    outline_panel.update_visible_entries(&editor, HashSet::default(), None, cx);
                    cx.notify();
                }
                EditorEvent::ExcerptsEdited { .. } => {}
                EditorEvent::ExcerptsExpanded { .. } => {}
                _ => {}
            },
        ))
    } else {
        None
    }
}

fn location_for_selection(
    editor: &View<Editor>,
    cx: &mut ViewContext<OutlinePanel>,
) -> (ExcerptId, Option<OutlineItem<language::Anchor>>) {
    let selection = editor
        .read(cx)
        .selections
        .newest::<language::Point>(cx)
        .head();
    let multi_buffer = editor.read(cx).buffer();
    let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);
    let selection = multi_buffer_snapshot.anchor_before(selection);
    let outline_item = multi_buffer_snapshot
        .buffer_for_excerpt(selection.excerpt_id)
        .and_then(|buffer_snapshot| {
            buffer_snapshot
                .outline(None)
                .into_iter()
                .flat_map(|outline| outline.items)
                .filter(|outline_item| {
                    range_contains(&outline_item.range, selection.text_anchor, buffer_snapshot)
                })
                .min_by_key(|outline| {
                    let range = outline.range.to_offset(&buffer_snapshot);
                    let cursor_offset = selection.text_anchor.to_offset(&buffer_snapshot) as isize;
                    let distance_to_closest_endpoint = cmp::min(
                        (range.start as isize - cursor_offset).abs(),
                        (range.end as isize - cursor_offset).abs(),
                    );
                    distance_to_closest_endpoint
                })
        });
    (selection.excerpt_id, outline_item)
}

fn range_contains(
    range: &Range<language::Anchor>,
    anchor: language::Anchor,
    buffer_snapshot: &language::BufferSnapshot,
) -> bool {
    range.start.cmp(&anchor, buffer_snapshot).is_le()
        && range.end.cmp(&anchor, buffer_snapshot).is_ge()
}

// TODO kb tests
