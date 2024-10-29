mod git_panel_settings;

use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use collections::{HashMap, HashSet};
use db::kvp::KEY_VALUE_STORE;
use editor::{
    actions::Cancel,
    items::{entry_git_aware_label_color, entry_label_color},
    scroll::{Autoscroll, AutoscrollStrategy, ScrollAnchor},
    Editor, ExcerptId, ExcerptRange,
};
use file_icons::FileIcons;
use fuzzy::StringMatch;
use git2;
use gpui::{
    actions, div, impl_actions, prelude::FluentBuilder, px, Action, AnyElement, AppContext,
    AssetSource, AsyncWindowContext, Div, ElementId, EventEmitter, FocusHandle, FocusableView,
    InteractiveElement, IntoElement, KeyContext, Model, MouseButton, ParentElement, Pixels, Point,
    Render, Stateful, Styled, Subscription, Task, View, ViewContext, VisualContext, WeakView,
    WindowContext,
};

use git_panel_settings::{GitPanelDockPosition, GitPanelSettings};
use language::{BufferId, BufferSnapshot, OutlineItem};
use project::{Fs, Project};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use theme::{ActiveTheme, ThemeSettings};
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    ui::{
        h_flex, v_flex, Color, ContextMenu, HighlightedLabel, Icon, IconName, IconSize, Label,
        LabelCommon, ListItem, Selectable, StyledExt, StyledTypography,
    },
    ItemHandle, WeakItemHandle, Workspace,
};
use worktree::{Entry, ProjectEntryId, WorktreeId};

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct Open {
    change_selection: bool,
}

impl_actions!(outline_panel, [Open]);

actions!(
    outline_panel,
    [RevealInFileManager, SelectParent, ToggleFocus,]
);

const OUTLINE_PANEL_KEY: &str = "GitPanel";

#[derive(Debug, Clone)]
struct GitStatus {
    branch: BranchInfo,
    files: Vec<FileStatus>,
}

#[derive(Debug)]
enum SelectedEntry {
    Invalidated(Option<PanelEntry>),
    Valid(PanelEntry),
    None,
}

#[derive(Clone, Debug)]
enum PanelEntry {
    Fs(FsEntry),
    FoldedDirs(WorktreeId, Vec<Entry>),
    Outline(OutlineEntry),
}

impl PartialEq for PanelEntry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Fs(a), Self::Fs(b)) => a == b,
            (Self::FoldedDirs(a1, a2), Self::FoldedDirs(b1, b2)) => a1 == b1 && a2 == b2,

            _ => false,
        }
    }
}

impl Eq for PanelEntry {}

type Outline = OutlineItem<language::Anchor>;

#[derive(Clone, Debug, PartialEq, Eq)]
enum OutlineEntry {
    Excerpt(BufferId, ExcerptId, ExcerptRange<language::Anchor>),
    Outline(BufferId, ExcerptId, Outline),
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum CollapsedEntry {
    Dir(WorktreeId, ProjectEntryId),
    File(WorktreeId, BufferId),
    ExternalFile(BufferId),
    Excerpt(BufferId, ExcerptId),
}

#[derive(Debug, Clone)]
struct BranchInfo {
    current_branch: String,
}

#[derive(Debug, Clone)]
struct FileStatus {
    path: String,
    status: GitFileStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum GitFileStatus {
    Modified,
    Added,
    Deleted,
    Renamed(String), // Contains the old path
    Untracked,
}

struct ActiveItem {
    item_handle: Box<dyn WeakItemHandle>,
    active_editor: WeakView<Editor>,
    _buffer_search_subscription: Subscription,
    _editor_subscrpiption: Subscription,
}

pub struct GitPanel {
    fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    project: Model<Project>,
    workspace: WeakView<Workspace>,
    active: bool,
    context_menu: Option<(View<ContextMenu>, Point<Pixels>, Subscription)>,
    focus_handle: FocusHandle,
    pending_serialization: Task<Option<()>>,
    collapsed_entries: HashSet<CollapsedEntry>,
    unfolded_dirs: HashMap<WorktreeId, BTreeSet<ProjectEntryId>>,
    selected_entry: SelectedEntry,
    active_item: Option<ActiveItem>,
    _subscriptions: Vec<Subscription>,
    filter_editor: View<Editor>,
    git_status: Option<GitStatus>,
    refresh_task: Task<()>,
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
    GitPanelSettings::register(cx);
}

pub fn init(assets: impl AssetSource, cx: &mut AppContext) {
    init_settings(cx);
    file_icons::init(assets, cx);

    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, cx| {
            workspace.toggle_panel_focus::<GitPanel>(cx);
        });
    })
    .detach();
}

impl GitPanel {
    pub async fn load(
        workspace: WeakView<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<View<Self>> {
        let serialized_panel = cx
            .background_executor()
            .spawn(async move { KEY_VALUE_STORE.read_kvp(OUTLINE_PANEL_KEY) })
            .await
            .context("loading git panel")
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

    fn get_workspace_root_path(&self, cx: &AppContext) -> Option<PathBuf> {
        let project = self.project.read(cx);
        project
            .worktrees(cx)
            .next() // Get first worktree
            .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
    }

    fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let filter_editor = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Filter...", cx);
            editor
        });
        let project = workspace.project().clone();
        let workspace_handle = cx.view().downgrade();

        let git_panel = cx.new_view(|cx| {
            let icons_subscription = cx.observe_global::<FileIcons>(|_, cx| {
                cx.notify();
            });

            let mut git_panel_settings = *GitPanelSettings::get_global(cx);
            let mut current_theme = ThemeSettings::get_global(cx).clone();
            let settings_subscription = cx.observe_global::<SettingsStore>(move |_, cx| {
                let new_settings = GitPanelSettings::get_global(cx);
                let new_theme = ThemeSettings::get_global(cx);
                if &current_theme != new_theme {
                    git_panel_settings = *new_settings;
                    current_theme = new_theme.clone();
                } else if &git_panel_settings != new_settings {
                    git_panel_settings = *new_settings;
                    cx.notify();
                }
            });

            let focus_handle = cx.focus_handle();

            let mut git_panel = Self {
                active: false,
                fs: workspace.app_state().fs.clone(),
                workspace: workspace_handle,
                project,
                filter_editor,
                focus_handle,
                context_menu: None,
                width: None,
                unfolded_dirs: HashMap::default(),
                pending_serialization: Task::ready(None),
                collapsed_entries: HashSet::default(),
                selected_entry: SelectedEntry::None,
                active_item: None,
                _subscriptions: vec![settings_subscription, icons_subscription],
                git_status: None,
                refresh_task: Task::ready(()),
            };

            if git_panel.active {
                git_panel.refresh_git_status(cx);
            }

            git_panel
        });

        git_panel
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

    fn dispatch_context(&self, cx: &ViewContext<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("GitPanel");
        dispatch_context.add("menu");

        dispatch_context
    }

    fn refresh_git_status(&mut self, cx: &mut ViewContext<Self>) {
        let workspace_path = match self.get_workspace_root_path(cx) {
            Some(path) => path,
            None => {
                self.git_status = None;
                cx.notify();
                return;
            }
        };
        self.refresh_task = cx.spawn(
            |panel: WeakView<GitPanel>, mut cx: AsyncWindowContext| async move {
                // Create a new repository instance
                if let Ok(repo) = git2::Repository::open(&workspace_path) {
                    // Get branch information
                    if let Ok(head) = repo.head() {
                        let branch_name = head.shorthand().unwrap_or("HEAD detached").to_string();

                        // Get status of files
                        let mut files = Vec::new();
                        if let Ok(statuses) = repo.statuses(None) {
                            for entry in statuses.iter() {
                                let status = entry.status();
                                let path = entry.path().unwrap_or("").to_string();

                                let file_status = if status.is_wt_modified() {
                                    GitFileStatus::Modified
                                } else if status.is_wt_new() {
                                    GitFileStatus::Added
                                } else if status.is_wt_deleted() {
                                    GitFileStatus::Deleted
                                } else if status.is_wt_renamed() {
                                    GitFileStatus::Renamed(
                                        entry
                                            .head_to_index()
                                            .unwrap()
                                            .old_file()
                                            .path()
                                            .unwrap()
                                            .to_string_lossy()
                                            .into(),
                                    )
                                } else if status.is_ignored() {
                                    continue;
                                } else {
                                    GitFileStatus::Untracked
                                };

                                files.push(FileStatus {
                                    path,
                                    status: file_status,
                                });
                            }
                        }

                        let git_status = GitStatus {
                            branch: BranchInfo {
                                current_branch: branch_name,
                            },
                            files,
                        };

                        panel
                            .update(&mut cx, |panel, cx| {
                                panel.git_status = Some(git_status);
                                cx.notify();
                            })
                            .ok();
                    }
                }
            },
        );
    }

    fn clear_git_status(&mut self, cx: &mut ViewContext<Self>) {
        self.git_status = None;
        cx.notify();
    }

    fn force_refresh_git_status(&mut self, cx: &mut ViewContext<Self>) {
        self.clear_git_status(cx);
        self.refresh_git_status(cx);
    }

    fn selected_entry(&self) -> Option<&PanelEntry> {
        match &self.selected_entry {
            SelectedEntry::Invalidated(entry) => entry.as_ref(),
            SelectedEntry::Valid(entry) => Some(entry),
            SelectedEntry::None => None,
        }
    }

    fn select_entry(&mut self, entry: PanelEntry, focus: bool, cx: &mut ViewContext<Self>) {
        if focus {
            self.focus_handle.focus(cx);
        }
        self.selected_entry = SelectedEntry::Valid(entry);
        cx.notify();
    }

    fn open(&mut self, open: &Open, cx: &mut ViewContext<Self>) {
        if let Some(selected_entry) = self.selected_entry().cloned() {
            self.open_entry(&selected_entry, open.change_selection, cx);
        }
    }

    fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        if self.context_menu.is_some() {
            self.context_menu.take();
            cx.notify();
        }
    }

    fn active_item(&self) -> Option<Box<dyn ItemHandle>> {
        self.active_item.as_ref()?.item_handle.upgrade()
    }

    fn active_editor(&self) -> Option<View<Editor>> {
        self.active_item.as_ref()?.active_editor.upgrade()
    }

    fn open_entry(
        &mut self,
        entry: &PanelEntry,
        change_selection: bool,
        cx: &mut ViewContext<GitPanel>,
    ) {
        let Some(active_editor) = self.active_editor() else {
            return;
        };
        let active_multi_buffer = active_editor.read(cx).buffer().clone();
        let multi_buffer_snapshot = active_multi_buffer.read(cx).snapshot(cx);
        let offset_from_top = if active_multi_buffer.read(cx).is_singleton() {
            Point::default()
        } else {
            Point::new(0.0, -(active_editor.read(cx).file_header_size() as f32))
        };

        let scroll_target = match entry {
            PanelEntry::FoldedDirs(..) | PanelEntry::Fs(FsEntry::Directory(..)) => None,
            PanelEntry::Fs(FsEntry::ExternalFile(buffer_id, _)) => {
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
                Some(offset_from_top).zip(scroll_target)
            }
            PanelEntry::Fs(FsEntry::File(_, file_entry, ..)) => {
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
                Some(offset_from_top).zip(scroll_target)
            }
            PanelEntry::Outline(OutlineEntry::Outline(_, excerpt_id, outline)) => {
                let scroll_target = multi_buffer_snapshot
                    .anchor_in_excerpt(*excerpt_id, outline.range.start)
                    .or_else(|| {
                        multi_buffer_snapshot.anchor_in_excerpt(*excerpt_id, outline.range.end)
                    });
                Some(Point::default()).zip(scroll_target)
            }
            PanelEntry::Outline(OutlineEntry::Excerpt(_, excerpt_id, excerpt_range)) => {
                let scroll_target = multi_buffer_snapshot
                    .anchor_in_excerpt(*excerpt_id, excerpt_range.context.start);
                Some(Point::default()).zip(scroll_target)
            }
        };

        if let Some((offset, anchor)) = scroll_target {
            let activate = self
                .workspace
                .update(cx, |workspace, cx| match self.active_item() {
                    Some(active_item) => {
                        workspace.activate_item(active_item.as_ref(), true, change_selection, cx)
                    }
                    None => workspace.activate_item(&active_editor, true, change_selection, cx),
                });

            if activate.is_ok() {
                self.select_entry(entry.clone(), true, cx);
                if change_selection {
                    active_editor.update(cx, |editor, cx| {
                        editor.change_selections(
                            Some(Autoscroll::Strategy(AutoscrollStrategy::Top)),
                            cx,
                            |s| s.select_ranges(Some(anchor..anchor)),
                        );
                    });
                    active_editor.focus_handle(cx).focus(cx);
                } else {
                    active_editor.update(cx, |editor, cx| {
                        editor.set_scroll_anchor(ScrollAnchor { offset, anchor }, cx);
                    });
                    self.focus_handle.focus(cx);
                }
            }
        }
    }

    fn render_entry(
        &self,
        rendered_entry: &FsEntry,
        depth: usize,
        string_match: Option<&StringMatch>,
        cx: &mut ViewContext<Self>,
    ) -> Stateful<Div> {
        let settings = GitPanelSettings::get_global(cx);
        let is_active = match self.selected_entry() {
            Some(PanelEntry::Fs(selected_entry)) => selected_entry == rendered_entry,
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
                    HighlightedLabel::new(
                        name,
                        string_match
                            .map(|string_match| string_match.positions.clone())
                            .unwrap_or_default(),
                    )
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
                    HighlightedLabel::new(
                        name,
                        string_match
                            .map(|string_match| string_match.positions.clone())
                            .unwrap_or_default(),
                    )
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
                    HighlightedLabel::new(
                        name,
                        string_match
                            .map(|string_match| string_match.positions.clone())
                            .unwrap_or_default(),
                    )
                    .color(color)
                    .into_any_element(),
                    icon.unwrap_or_else(empty_icon),
                )
            }
        };

        self.entry_element(
            PanelEntry::Fs(rendered_entry.clone()),
            item_id,
            depth,
            Some(icon),
            is_active,
            label_element,
            cx,
        )
    }

    fn dir_names_string(
        &self,
        entries: &[Entry],
        worktree_id: WorktreeId,
        cx: &AppContext,
    ) -> String {
        let dir_names_segment = entries
            .iter()
            .map(|entry| self.entry_name(&worktree_id, entry, cx))
            .collect::<PathBuf>();
        dir_names_segment.to_string_lossy().to_string()
    }

    fn render_folded_dirs(
        &self,
        worktree_id: WorktreeId,
        dir_entries: &[Entry],
        depth: usize,
        string_match: Option<&StringMatch>,
        cx: &mut ViewContext<GitPanel>,
    ) -> Stateful<Div> {
        let settings = GitPanelSettings::get_global(cx);
        let is_active = match self.selected_entry() {
            Some(PanelEntry::FoldedDirs(selected_worktree_id, selected_entries)) => {
                selected_worktree_id == &worktree_id && selected_entries == dir_entries
            }
            _ => false,
        };
        let (item_id, label_element, icon) = {
            let name = self.dir_names_string(dir_entries, worktree_id, cx);

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
                HighlightedLabel::new(
                    name,
                    string_match
                        .map(|string_match| string_match.positions.clone())
                        .unwrap_or_default(),
                )
                .color(color)
                .into_any_element(),
                icon.unwrap_or_else(empty_icon),
            )
        };

        self.entry_element(
            PanelEntry::FoldedDirs(worktree_id, dir_entries.to_vec()),
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
        rendered_entry: PanelEntry,
        item_id: ElementId,
        depth: usize,
        icon_element: Option<AnyElement>,
        is_active: bool,
        label_element: gpui::AnyElement,
        cx: &mut ViewContext<GitPanel>,
    ) -> Stateful<Div> {
        let settings = GitPanelSettings::get_global(cx);
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
                            let change_selection = event.down.click_count > 1;
                            outline_panel.open_entry(&clicked_entry, change_selection, cx);
                        })
                    }),
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

    fn entry_name(&self, worktree_id: &WorktreeId, entry: &Entry, cx: &AppContext) -> String {
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

    fn buffer_snapshot_for_id(
        &self,
        buffer_id: BufferId,
        cx: &AppContext,
    ) -> Option<BufferSnapshot> {
        let editor = self.active_editor()?;
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
}

impl Panel for GitPanel {
    fn persistent_name() -> &'static str {
        "Outline Panel"
    }

    fn position(&self, cx: &WindowContext) -> DockPosition {
        match GitPanelSettings::get_global(cx).dock {
            GitPanelDockPosition::Left => DockPosition::Left,
            GitPanelDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<GitPanelSettings>(
            self.fs.clone(),
            cx,
            move |settings, _| {
                let dock = match position {
                    DockPosition::Left | DockPosition::Bottom => GitPanelDockPosition::Left,
                    DockPosition::Right => GitPanelDockPosition::Right,
                };
                settings.dock = Some(dock);
            },
        );
    }

    fn size(&self, cx: &WindowContext) -> Pixels {
        self.width
            .unwrap_or_else(|| GitPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, cx: &WindowContext) -> Option<IconName> {
        GitPanelSettings::get_global(cx)
            .button
            .then_some(IconName::Git)
    }

    fn icon_tooltip(&self, _: &WindowContext) -> Option<&'static str> {
        Some("Git Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn starts_open(&self, _: &WindowContext) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        cx.spawn(|git_panel, mut cx| async move {
            git_panel
                .update(&mut cx, |git_panel, cx| {
                    git_panel.active = active;

                    if active {
                        // Force immediate refresh when panel becomes active
                        git_panel.force_refresh_git_status(cx);
                    } else {
                        git_panel.clear_git_status(cx);
                    }

                    git_panel.serialize(cx);
                })
                .ok();
        })
        .detach()
    }
}

impl FocusableView for GitPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.filter_editor.focus_handle(cx).clone()
    }
}

impl EventEmitter<Event> for GitPanel {}

impl EventEmitter<PanelEvent> for GitPanel {}

impl Render for GitPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let git_panel = v_flex()
            .id("git-panel")
            .size_full()
            .relative()
            .key_context(self.dispatch_context(cx));

        // Branch information section
        let header = if let Some(git_status) = &self.git_status {
            v_flex()
                .gap_2()
                .p_2()
                .child(h_flex().gap_1().child(Label::new(format!(
                    "Branch: {}",
                    git_status.branch.current_branch
                ))))
                .child(horizontal_separator(cx))
        } else {
            v_flex()
                .gap_2()
                .p_2()
                .child(Label::new("No Git repository found"))
                .child(horizontal_separator(cx))
        };

        // Files section
        let files_section = if let Some(git_status) = &self.git_status {
            let mut current_folder: Option<String> = None;
            let mut folder_items = Vec::new();
            let mut file_items = Vec::new();

            // Get the worktree_id from the project
            let worktree_id = self
                .project
                .read(cx)
                .worktrees(cx)
                .next()
                .map(|worktree| worktree.read(cx).id());

            // Group files by folder
            for file in &git_status.files {
                let path = std::path::Path::new(&file.path);
                if let Some(parent) = path.parent() {
                    let folder_path = parent.to_string_lossy().to_string();

                    // If we're in a new folder, add the previous folder's contents
                    if current_folder.as_ref() != Some(&folder_path) {
                        if let Some(current) = current_folder.take() {
                            folder_items.push(
                                v_flex()
                                    .gap_1()
                                    .pl_4()
                                    .child(h_flex().gap_1().child(Label::new(current)))
                                    .children(file_items.drain(..)),
                            );
                        }
                        current_folder = Some(folder_path);
                    }
                }

                // Status icon based on file status
                let status_icon = match file.status {
                    GitFileStatus::Modified => "M",
                    GitFileStatus::Added => "A",
                    GitFileStatus::Deleted => "D",
                    GitFileStatus::Renamed(_) => "R",
                    GitFileStatus::Untracked => "?",
                };

                // Create clickable file item
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                let file_path = file.path.clone();
                let worktree_id = worktree_id.clone();

                let file_item = ListItem::new(ElementId::from("file"))
                    .child(
                        h_flex()
                            .gap_2()
                            .pl_6()
                            .child(status_icon)
                            .child(Label::new(file_name.to_string())),
                    )
                    .on_click(
                        cx.listener(move |git_panel, _event: &gpui::ClickEvent, cx| {
                            if let (Some(workspace), Some(worktree_id)) =
                                (git_panel.workspace.upgrade(), worktree_id)
                            {
                                workspace.update(cx, |workspace, cx| {
                                    // Create a ProjectPath from WorktreeId and path
                                    let project_path = project::ProjectPath::from((
                                        worktree_id,
                                        std::path::PathBuf::from(&file_path),
                                    ));
                                    workspace.open_path(project_path, None, true, cx)
                                });
                            }
                        }),
                    );

                file_items.push(file_item);
            }

            // Add the last folder if any
            if let Some(current) = current_folder {
                folder_items.push(
                    v_flex()
                        .gap_1()
                        .pl_4()
                        .child(h_flex().gap_1().child(Label::new(current)))
                        .children(file_items.drain(..)),
                );
            }

            v_flex().gap_1().p_2().children(folder_items)
        } else {
            v_flex()
        };

        git_panel.child(header).child(files_section)
    }
}

fn empty_icon() -> AnyElement {
    h_flex()
        .size(IconSize::default().rems())
        .invisible()
        .flex_none()
        .into_any_element()
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

// Helper function for horizontal separators
fn horizontal_separator(cx: &mut WindowContext) -> Div {
    div().mx_2().border_primary(cx).border_t_1()
}
