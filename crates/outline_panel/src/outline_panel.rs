mod outline_panel_settings;

use std::{
    cell::OnceCell,
    cmp,
    ops::Range,
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
    u32,
};

use anyhow::Context;
use collections::{hash_map, BTreeSet, HashMap, HashSet};
use db::kvp::KEY_VALUE_STORE;
use editor::{
    display_map::ToDisplayPoint,
    items::{entry_git_aware_label_color, entry_label_color},
    scroll::{Autoscroll, ScrollAnchor},
    AnchorRangeExt, Bias, DisplayPoint, Editor, EditorEvent, EditorMode, ExcerptId, ExcerptRange,
    MultiBufferSnapshot, RangeToAnchorExt,
};
use file_icons::FileIcons;
use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    actions, anchored, deferred, div, impl_actions, px, uniform_list, Action, AnyElement,
    AppContext, AssetSource, AsyncWindowContext, ClipboardItem, DismissEvent, Div, ElementId,
    EventEmitter, FocusHandle, FocusableView, HighlightStyle, InteractiveElement, IntoElement,
    KeyContext, Model, MouseButton, MouseDownEvent, ParentElement, Pixels, Point, Render,
    SharedString, Stateful, Styled, Subscription, Task, UniformListScrollHandle, View, ViewContext,
    VisualContext, WeakView, WindowContext,
};
use itertools::Itertools;
use language::{BufferId, BufferSnapshot, OffsetRangeExt, OutlineItem};
use menu::{Cancel, SelectFirst, SelectLast, SelectNext, SelectPrev};

use outline_panel_settings::{OutlinePanelDockPosition, OutlinePanelSettings};
use project::{File, Fs, Item, Project};
use search::{BufferSearchBar, ProjectSearchView};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use theme::SyntaxTheme;
use util::{RangeExt, ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    item::ItemHandle,
    searchable::{SearchEvent, SearchableItem},
    ui::{
        h_flex, v_flex, ActiveTheme, ButtonCommon, Clickable, Color, ContextMenu, FluentBuilder,
        HighlightedLabel, Icon, IconButton, IconButtonShape, IconName, IconSize, Label,
        LabelCommon, ListItem, Selectable, Spacing, StyledExt, StyledTypography, Tooltip,
    },
    OpenInTerminal, Workspace,
};
use worktree::{Entry, ProjectEntryId, WorktreeId};

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct Open {
    change_selection: bool,
}

impl_actions!(outline_panel, [Open]);

actions!(
    outline_panel,
    [
        CollapseAllEntries,
        CollapseSelectedEntry,
        CopyPath,
        CopyRelativePath,
        ExpandAllEntries,
        ExpandSelectedEntry,
        FoldDirectory,
        ToggleActiveEditorPin,
        RevealInFileManager,
        SelectParent,
        ToggleFocus,
        UnfoldDirectory,
    ]
);

const OUTLINE_PANEL_KEY: &str = "OutlinePanel";
const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);

type Outline = OutlineItem<language::Anchor>;

pub struct OutlinePanel {
    fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    project: Model<Project>,
    workspace: View<Workspace>,
    active: bool,
    pinned: bool,
    scroll_handle: UniformListScrollHandle,
    context_menu: Option<(View<ContextMenu>, Point<Pixels>, Subscription)>,
    focus_handle: FocusHandle,
    pending_serialization: Task<Option<()>>,
    fs_entries_depth: HashMap<(WorktreeId, ProjectEntryId), usize>,
    fs_entries: Vec<FsEntry>,
    fs_children_count: HashMap<WorktreeId, HashMap<Arc<Path>, FsChildren>>,
    collapsed_entries: HashSet<CollapsedEntry>,
    unfolded_dirs: HashMap<WorktreeId, BTreeSet<ProjectEntryId>>,
    selected_entry: SelectedEntry,
    active_item: Option<ActiveItem>,
    _subscriptions: Vec<Subscription>,
    updating_fs_entries: bool,
    fs_entries_update_task: Task<()>,
    cached_entries_update_task: Task<()>,
    reveal_selection_task: Task<anyhow::Result<()>>,
    outline_fetch_tasks: HashMap<(BufferId, ExcerptId), Task<()>>,
    excerpts: HashMap<BufferId, HashMap<ExcerptId, Excerpt>>,
    cached_entries: Vec<CachedEntry>,
    filter_editor: View<Editor>,
    mode: ItemsDisplayMode,
    search: Option<(SearchKind, String)>,
    search_matches: Vec<Range<editor::Anchor>>,
}

#[derive(Debug)]
enum SelectedEntry {
    Invalidated(Option<PanelEntry>),
    Valid(PanelEntry),
    None,
}

impl SelectedEntry {
    fn invalidate(&mut self) {
        match std::mem::replace(self, SelectedEntry::None) {
            Self::Valid(entry) => *self = Self::Invalidated(Some(entry)),
            Self::None => *self = Self::Invalidated(None),
            other => *self = other,
        }
    }

    fn is_invalidated(&self) -> bool {
        matches!(self, Self::Invalidated(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ItemsDisplayMode {
    Search,
    Outline,
}

#[derive(Debug, Clone, Copy, Default)]
struct FsChildren {
    files: usize,
    dirs: usize,
}

impl FsChildren {
    fn may_be_fold_part(&self) -> bool {
        self.dirs == 0 || (self.dirs == 1 && self.files == 0)
    }
}

#[derive(Clone, Debug)]
struct CachedEntry {
    depth: usize,
    string_match: Option<StringMatch>,
    entry: PanelEntry,
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

#[derive(Clone, Debug)]
enum PanelEntry {
    Fs(FsEntry),
    FoldedDirs(WorktreeId, Vec<Entry>),
    Outline(OutlineEntry),
    Search(SearchEntry),
}

#[derive(Clone, Debug)]
struct SearchEntry {
    match_range: Range<editor::Anchor>,
    same_line_matches: Vec<Range<editor::Anchor>>,
    kind: SearchKind,
    render_data: Option<OnceCell<SearchData>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum SearchKind {
    Project,
    Buffer,
}

#[derive(Clone, Debug)]
struct SearchData {
    context_range: Range<editor::Anchor>,
    context_text: String,
    highlight_ranges: Vec<(Range<usize>, HighlightStyle)>,
    search_match_indices: Vec<Range<usize>>,
}

impl PartialEq for PanelEntry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Fs(a), Self::Fs(b)) => a == b,
            (Self::FoldedDirs(a1, a2), Self::FoldedDirs(b1, b2)) => a1 == b1 && a2 == b2,
            (Self::Outline(a), Self::Outline(b)) => a == b,
            (
                Self::Search(SearchEntry {
                    match_range: match_range_a,
                    kind: kind_a,
                    ..
                }),
                Self::Search(SearchEntry {
                    match_range: match_range_b,
                    kind: kind_b,
                    ..
                }),
            ) => match_range_a == match_range_b && kind_a == kind_b,
            _ => false,
        }
    }
}

impl Eq for PanelEntry {}

impl SearchData {
    fn new(
        kind: SearchKind,
        match_range: &Range<editor::Anchor>,
        multi_buffer_snapshot: &MultiBufferSnapshot,
        theme: &SyntaxTheme,
    ) -> Self {
        let match_point_range = match_range.to_point(&multi_buffer_snapshot);
        let entire_row_range_start = language::Point::new(match_point_range.start.row, 0);
        let entire_row_range_end = multi_buffer_snapshot.clip_point(
            language::Point::new(match_point_range.end.row, u32::MAX),
            Bias::Right,
        );
        let entire_row_range =
            (entire_row_range_start..entire_row_range_end).to_anchors(&multi_buffer_snapshot);
        let entire_row_offset_range = entire_row_range.to_offset(&multi_buffer_snapshot);
        let match_offset_range = match_range.to_offset(&multi_buffer_snapshot);
        let mut search_match_indices = vec![
            match_offset_range.start - entire_row_offset_range.start
                ..match_offset_range.end - entire_row_offset_range.start,
        ];

        let mut left_whitespaces_count = 0;
        let mut non_whitespace_symbol_occurred = false;
        let mut offset = entire_row_offset_range.start;
        let mut entire_row_text = String::new();
        let mut highlight_ranges = Vec::new();
        for mut chunk in multi_buffer_snapshot.chunks(
            entire_row_offset_range.start..entire_row_offset_range.end,
            true,
        ) {
            if !non_whitespace_symbol_occurred {
                for c in chunk.text.chars() {
                    if c.is_whitespace() {
                        left_whitespaces_count += 1;
                    } else {
                        non_whitespace_symbol_occurred = true;
                        break;
                    }
                }
            }

            if chunk.text.len() > entire_row_offset_range.end - offset {
                chunk.text = &chunk.text[0..(entire_row_offset_range.end - offset)];
                offset = entire_row_offset_range.end;
            } else {
                offset += chunk.text.len();
            }
            let style = chunk
                .syntax_highlight_id
                .and_then(|highlight| highlight.style(theme));
            if let Some(style) = style {
                let start = entire_row_text.len();
                let end = start + chunk.text.len();
                highlight_ranges.push((start..end, style));
            }
            entire_row_text.push_str(chunk.text);
            if offset >= entire_row_offset_range.end {
                break;
            }
        }

        if let SearchKind::Buffer = kind {
            left_whitespaces_count = 0;
        }
        highlight_ranges.iter_mut().for_each(|(range, _)| {
            range.start = range.start.saturating_sub(left_whitespaces_count);
            range.end = range.end.saturating_sub(left_whitespaces_count);
        });
        search_match_indices.iter_mut().for_each(|range| {
            range.start = range.start.saturating_sub(left_whitespaces_count);
            range.end = range.end.saturating_sub(left_whitespaces_count);
        });
        let trimmed_row_offset_range =
            entire_row_offset_range.start + left_whitespaces_count..entire_row_offset_range.end;
        let trimmed_text = entire_row_text[left_whitespaces_count..].to_owned();
        Self {
            highlight_ranges,
            search_match_indices,
            context_range: trimmed_row_offset_range.to_anchors(&multi_buffer_snapshot),
            context_text: trimmed_text,
        }
    }
}

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

struct ActiveItem {
    active_editor: WeakView<Editor>,
    _buffer_search_subscription: Subscription,
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
        let workspace_handle = cx.view().clone();
        let outline_panel = cx.new_view(|cx| {
            let filter_editor = cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text("Filter...", cx);
                editor
            });
            let filter_update_subscription =
                cx.subscribe(&filter_editor, |outline_panel: &mut Self, _, event, cx| {
                    if let editor::EditorEvent::BufferEdited = event {
                        outline_panel.update_cached_entries(Some(UPDATE_DEBOUNCE), cx);
                    }
                });

            let focus_handle = cx.focus_handle();
            let focus_subscription = cx.on_focus(&focus_handle, Self::focus_in);
            let workspace_subscription = cx.subscribe(
                &workspace
                    .weak_handle()
                    .upgrade()
                    .expect("have a &mut Workspace"),
                move |outline_panel, workspace, event, cx| {
                    if let workspace::Event::ActiveItemChanged = event {
                        if let Some(new_active_editor) =
                            workspace_active_editor(workspace.read(cx), cx)
                        {
                            if outline_panel.should_replace_active_editor(&new_active_editor) {
                                outline_panel.replace_active_editor(new_active_editor, cx);
                            }
                        } else {
                            outline_panel.clear_previous(cx);
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
                mode: ItemsDisplayMode::Outline,
                active: false,
                pinned: false,
                workspace: workspace_handle,
                project,
                fs: workspace.app_state().fs.clone(),
                scroll_handle: UniformListScrollHandle::new(),
                focus_handle,
                filter_editor,
                fs_entries: Vec::new(),
                search_matches: Vec::new(),
                search: None,
                fs_entries_depth: HashMap::default(),
                fs_children_count: HashMap::default(),
                collapsed_entries: HashSet::default(),
                unfolded_dirs: HashMap::default(),
                selected_entry: SelectedEntry::None,
                context_menu: None,
                width: None,
                active_item: None,
                pending_serialization: Task::ready(None),
                updating_fs_entries: false,
                fs_entries_update_task: Task::ready(()),
                cached_entries_update_task: Task::ready(()),
                reveal_selection_task: Task::ready(Ok(())),
                outline_fetch_tasks: HashMap::default(),
                excerpts: HashMap::default(),
                cached_entries: Vec::new(),
                _subscriptions: vec![
                    settings_subscription,
                    icons_subscription,
                    focus_subscription,
                    workspace_subscription,
                    filter_update_subscription,
                ],
            };
            if let Some(editor) = workspace_active_editor(workspace, cx) {
                outline_panel.replace_active_editor(editor, cx);
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
        if let Some(PanelEntry::FoldedDirs(worktree_id, entries)) = self.selected_entry().cloned() {
            self.unfolded_dirs
                .entry(worktree_id)
                .or_default()
                .extend(entries.iter().map(|entry| entry.id));
            self.update_cached_entries(None, cx);
        }
    }

    fn fold_directory(&mut self, _: &FoldDirectory, cx: &mut ViewContext<Self>) {
        let (worktree_id, entry) = match self.selected_entry().cloned() {
            Some(PanelEntry::Fs(FsEntry::Directory(worktree_id, entry))) => {
                (worktree_id, Some(entry))
            }
            Some(PanelEntry::FoldedDirs(worktree_id, entries)) => {
                (worktree_id, entries.last().cloned())
            }
            _ => return,
        };
        let Some(entry) = entry else {
            return;
        };
        let unfolded_dirs = self.unfolded_dirs.get_mut(&worktree_id);
        let worktree = self
            .project
            .read(cx)
            .worktree_for_id(worktree_id, cx)
            .map(|w| w.read(cx).snapshot());
        let Some((_, unfolded_dirs)) = worktree.zip(unfolded_dirs) else {
            return;
        };

        unfolded_dirs.remove(&entry.id);
        self.update_cached_entries(None, cx);
    }

    fn open(&mut self, open: &Open, cx: &mut ViewContext<Self>) {
        if self.filter_editor.focus_handle(cx).is_focused(cx) {
            cx.propagate()
        } else if let Some(selected_entry) = self.selected_entry().cloned() {
            self.open_entry(&selected_entry, open.change_selection, cx);
        }
    }

    fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        if self.filter_editor.focus_handle(cx).is_focused(cx) {
            self.focus_handle.focus(cx);
        } else {
            self.filter_editor.focus_handle(cx).focus(cx);
        }

        if self.context_menu.is_some() {
            self.context_menu.take();
            cx.notify();
        }
    }

    fn open_entry(
        &mut self,
        entry: &PanelEntry,
        change_selection: bool,
        cx: &mut ViewContext<OutlinePanel>,
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

        self.toggle_expanded(entry, cx);
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
            PanelEntry::Search(SearchEntry { match_range, .. }) => {
                Some((Point::default(), match_range.start))
            }
        };

        if let Some((offset, anchor)) = scroll_target {
            self.select_entry(entry.clone(), true, cx);
            if change_selection {
                active_editor.update(cx, |editor, cx| {
                    editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                        s.select_ranges(Some(anchor..anchor))
                    });
                });
                active_editor.focus_handle(cx).focus(cx);
            } else {
                active_editor.update(cx, |editor, cx| {
                    editor.set_scroll_anchor(ScrollAnchor { offset, anchor }, cx);
                });
                self.focus_handle.focus(cx);
            }

            if let PanelEntry::Search(_) = entry {
                if let Some(active_project_search) =
                    self.active_project_search(Some(&active_editor), cx)
                {
                    self.workspace.update(cx, |workspace, cx| {
                        workspace.activate_item(&active_project_search, true, change_selection, cx)
                    });
                }
            } else {
                self.workspace.update(cx, |workspace, cx| {
                    workspace.activate_item(&active_editor, true, change_selection, cx)
                });
            };
        }
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if let Some(entry_to_select) = self.selected_entry().and_then(|selected_entry| {
            self.cached_entries
                .iter()
                .map(|cached_entry| &cached_entry.entry)
                .skip_while(|entry| entry != &selected_entry)
                .skip(1)
                .next()
                .cloned()
        }) {
            self.select_entry(entry_to_select, true, cx);
        } else {
            self.select_first(&SelectFirst {}, cx)
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if let Some(entry_to_select) = self.selected_entry().and_then(|selected_entry| {
            self.cached_entries
                .iter()
                .rev()
                .map(|cached_entry| &cached_entry.entry)
                .skip_while(|entry| entry != &selected_entry)
                .skip(1)
                .next()
                .cloned()
        }) {
            self.select_entry(entry_to_select, true, cx);
        } else {
            self.select_first(&SelectFirst {}, cx)
        }
    }

    fn select_parent(&mut self, _: &SelectParent, cx: &mut ViewContext<Self>) {
        if let Some(entry_to_select) = self.selected_entry().and_then(|selected_entry| {
            let mut previous_entries = self
                .cached_entries
                .iter()
                .rev()
                .map(|cached_entry| &cached_entry.entry)
                .skip_while(|entry| entry != &selected_entry)
                .skip(1);
            match &selected_entry {
                PanelEntry::Fs(fs_entry) => match fs_entry {
                    FsEntry::ExternalFile(..) => None,
                    FsEntry::File(worktree_id, entry, ..)
                    | FsEntry::Directory(worktree_id, entry) => {
                        entry.path.parent().and_then(|parent_path| {
                            previous_entries.find(|entry| match entry {
                                PanelEntry::Fs(FsEntry::Directory(dir_worktree_id, dir_entry)) => {
                                    dir_worktree_id == worktree_id
                                        && dir_entry.path.as_ref() == parent_path
                                }
                                PanelEntry::FoldedDirs(dirs_worktree_id, dirs) => {
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
                PanelEntry::FoldedDirs(worktree_id, entries) => entries
                    .first()
                    .and_then(|entry| entry.path.parent())
                    .and_then(|parent_path| {
                        previous_entries.find(|entry| {
                            if let PanelEntry::Fs(FsEntry::Directory(dir_worktree_id, dir_entry)) =
                                entry
                            {
                                dir_worktree_id == worktree_id
                                    && dir_entry.path.as_ref() == parent_path
                            } else {
                                false
                            }
                        })
                    }),
                PanelEntry::Outline(OutlineEntry::Excerpt(excerpt_buffer_id, excerpt_id, _)) => {
                    previous_entries.find(|entry| match entry {
                        PanelEntry::Fs(FsEntry::File(_, _, file_buffer_id, file_excerpts)) => {
                            file_buffer_id == excerpt_buffer_id
                                && file_excerpts.contains(&excerpt_id)
                        }
                        PanelEntry::Fs(FsEntry::ExternalFile(file_buffer_id, file_excerpts)) => {
                            file_buffer_id == excerpt_buffer_id
                                && file_excerpts.contains(&excerpt_id)
                        }
                        _ => false,
                    })
                }
                PanelEntry::Outline(OutlineEntry::Outline(
                    outline_buffer_id,
                    outline_excerpt_id,
                    _,
                )) => previous_entries.find(|entry| {
                    if let PanelEntry::Outline(OutlineEntry::Excerpt(
                        excerpt_buffer_id,
                        excerpt_id,
                        _,
                    )) = entry
                    {
                        outline_buffer_id == excerpt_buffer_id && outline_excerpt_id == excerpt_id
                    } else {
                        false
                    }
                }),
                PanelEntry::Search(_) => {
                    previous_entries.find(|entry| !matches!(entry, PanelEntry::Search(_)))
                }
            }
        }) {
            self.select_entry(entry_to_select.clone(), true, cx);
        } else {
            self.select_first(&SelectFirst {}, cx);
        }
    }

    fn select_first(&mut self, _: &SelectFirst, cx: &mut ViewContext<Self>) {
        if let Some(first_entry) = self.cached_entries.iter().next() {
            self.select_entry(first_entry.entry.clone(), true, cx);
        }
    }

    fn select_last(&mut self, _: &SelectLast, cx: &mut ViewContext<Self>) {
        if let Some(new_selection) = self
            .cached_entries
            .iter()
            .rev()
            .map(|cached_entry| &cached_entry.entry)
            .next()
        {
            self.select_entry(new_selection.clone(), true, cx);
        }
    }

    fn autoscroll(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(selected_entry) = self.selected_entry() {
            let index = self
                .cached_entries
                .iter()
                .position(|cached_entry| &cached_entry.entry == selected_entry);
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
        entry: PanelEntry,
        cx: &mut ViewContext<Self>,
    ) {
        self.select_entry(entry.clone(), true, cx);
        let is_root = match &entry {
            PanelEntry::Fs(FsEntry::File(worktree_id, entry, ..))
            | PanelEntry::Fs(FsEntry::Directory(worktree_id, entry)) => self
                .project
                .read(cx)
                .worktree_for_id(*worktree_id, cx)
                .map(|worktree| {
                    worktree.read(cx).root_entry().map(|entry| entry.id) == Some(entry.id)
                })
                .unwrap_or(false),
            PanelEntry::FoldedDirs(worktree_id, entries) => entries
                .first()
                .and_then(|entry| {
                    self.project
                        .read(cx)
                        .worktree_for_id(*worktree_id, cx)
                        .map(|worktree| {
                            worktree.read(cx).root_entry().map(|entry| entry.id) == Some(entry.id)
                        })
                })
                .unwrap_or(false),
            PanelEntry::Fs(FsEntry::ExternalFile(..)) => false,
            PanelEntry::Outline(..) => {
                cx.notify();
                return;
            }
            PanelEntry::Search(_) => {
                cx.notify();
                return;
            }
        };
        let auto_fold_dirs = OutlinePanelSettings::get_global(cx).auto_fold_dirs;
        let is_foldable = auto_fold_dirs && !is_root && self.is_foldable(&entry);
        let is_unfoldable = auto_fold_dirs && !is_root && self.is_unfoldable(&entry);

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

    fn is_unfoldable(&self, entry: &PanelEntry) -> bool {
        matches!(entry, PanelEntry::FoldedDirs(..))
    }

    fn is_foldable(&self, entry: &PanelEntry) -> bool {
        let (directory_worktree, directory_entry) = match entry {
            PanelEntry::Fs(FsEntry::Directory(directory_worktree, directory_entry)) => {
                (*directory_worktree, Some(directory_entry))
            }
            _ => return false,
        };
        let Some(directory_entry) = directory_entry else {
            return false;
        };

        if self
            .unfolded_dirs
            .get(&directory_worktree)
            .map_or(true, |unfolded_dirs| {
                !unfolded_dirs.contains(&directory_entry.id)
            })
        {
            return false;
        }

        let children = self
            .fs_children_count
            .get(&directory_worktree)
            .and_then(|entries| entries.get(&directory_entry.path))
            .copied()
            .unwrap_or_default();

        children.may_be_fold_part() && children.dirs > 0
    }

    fn expand_selected_entry(&mut self, _: &ExpandSelectedEntry, cx: &mut ViewContext<Self>) {
        let entry_to_expand = match self.selected_entry() {
            Some(PanelEntry::FoldedDirs(worktree_id, dir_entries)) => dir_entries
                .last()
                .map(|entry| CollapsedEntry::Dir(*worktree_id, entry.id)),
            Some(PanelEntry::Fs(FsEntry::Directory(worktree_id, dir_entry))) => {
                Some(CollapsedEntry::Dir(*worktree_id, dir_entry.id))
            }
            Some(PanelEntry::Fs(FsEntry::File(worktree_id, _, buffer_id, _))) => {
                Some(CollapsedEntry::File(*worktree_id, *buffer_id))
            }
            Some(PanelEntry::Fs(FsEntry::ExternalFile(buffer_id, _))) => {
                Some(CollapsedEntry::ExternalFile(*buffer_id))
            }
            Some(PanelEntry::Outline(OutlineEntry::Excerpt(buffer_id, excerpt_id, _))) => {
                Some(CollapsedEntry::Excerpt(*buffer_id, *excerpt_id))
            }
            None | Some(PanelEntry::Search(_)) | Some(PanelEntry::Outline(..)) => None,
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
            self.update_cached_entries(None, cx);
        } else {
            self.select_next(&SelectNext, cx)
        }
    }

    fn collapse_selected_entry(&mut self, _: &CollapseSelectedEntry, cx: &mut ViewContext<Self>) {
        let Some(selected_entry) = self.selected_entry().cloned() else {
            return;
        };
        match &selected_entry {
            PanelEntry::Fs(FsEntry::Directory(worktree_id, selected_dir_entry)) => {
                self.collapsed_entries
                    .insert(CollapsedEntry::Dir(*worktree_id, selected_dir_entry.id));
                self.select_entry(selected_entry, true, cx);
                self.update_cached_entries(None, cx);
            }
            PanelEntry::Fs(FsEntry::File(worktree_id, _, buffer_id, _)) => {
                self.collapsed_entries
                    .insert(CollapsedEntry::File(*worktree_id, *buffer_id));
                self.select_entry(selected_entry, true, cx);
                self.update_cached_entries(None, cx);
            }
            PanelEntry::Fs(FsEntry::ExternalFile(buffer_id, _)) => {
                self.collapsed_entries
                    .insert(CollapsedEntry::ExternalFile(*buffer_id));
                self.select_entry(selected_entry, true, cx);
                self.update_cached_entries(None, cx);
            }
            PanelEntry::FoldedDirs(worktree_id, dir_entries) => {
                if let Some(dir_entry) = dir_entries.last() {
                    if self
                        .collapsed_entries
                        .insert(CollapsedEntry::Dir(*worktree_id, dir_entry.id))
                    {
                        self.select_entry(selected_entry, true, cx);
                        self.update_cached_entries(None, cx);
                    }
                }
            }
            PanelEntry::Outline(OutlineEntry::Excerpt(buffer_id, excerpt_id, _)) => {
                if self
                    .collapsed_entries
                    .insert(CollapsedEntry::Excerpt(*buffer_id, *excerpt_id))
                {
                    self.select_entry(selected_entry, true, cx);
                    self.update_cached_entries(None, cx);
                }
            }
            PanelEntry::Search(_) | PanelEntry::Outline(..) => {}
        }
    }

    pub fn expand_all_entries(&mut self, _: &ExpandAllEntries, cx: &mut ViewContext<Self>) {
        let expanded_entries =
            self.fs_entries
                .iter()
                .fold(HashSet::default(), |mut entries, fs_entry| {
                    match fs_entry {
                        FsEntry::ExternalFile(buffer_id, _) => {
                            entries.insert(CollapsedEntry::ExternalFile(*buffer_id));
                            entries.extend(self.excerpts.get(buffer_id).into_iter().flat_map(
                                |excerpts| {
                                    excerpts.iter().map(|(excerpt_id, _)| {
                                        CollapsedEntry::Excerpt(*buffer_id, *excerpt_id)
                                    })
                                },
                            ));
                        }
                        FsEntry::Directory(worktree_id, entry) => {
                            entries.insert(CollapsedEntry::Dir(*worktree_id, entry.id));
                        }
                        FsEntry::File(worktree_id, _, buffer_id, _) => {
                            entries.insert(CollapsedEntry::File(*worktree_id, *buffer_id));
                            entries.extend(self.excerpts.get(buffer_id).into_iter().flat_map(
                                |excerpts| {
                                    excerpts.iter().map(|(excerpt_id, _)| {
                                        CollapsedEntry::Excerpt(*buffer_id, *excerpt_id)
                                    })
                                },
                            ));
                        }
                    }
                    entries
                });
        self.collapsed_entries
            .retain(|entry| !expanded_entries.contains(entry));
        self.update_cached_entries(None, cx);
    }

    pub fn collapse_all_entries(&mut self, _: &CollapseAllEntries, cx: &mut ViewContext<Self>) {
        let new_entries = self
            .cached_entries
            .iter()
            .flat_map(|cached_entry| match &cached_entry.entry {
                PanelEntry::Fs(FsEntry::Directory(worktree_id, entry)) => {
                    Some(CollapsedEntry::Dir(*worktree_id, entry.id))
                }
                PanelEntry::Fs(FsEntry::File(worktree_id, _, buffer_id, _)) => {
                    Some(CollapsedEntry::File(*worktree_id, *buffer_id))
                }
                PanelEntry::Fs(FsEntry::ExternalFile(buffer_id, _)) => {
                    Some(CollapsedEntry::ExternalFile(*buffer_id))
                }
                PanelEntry::FoldedDirs(worktree_id, entries) => {
                    Some(CollapsedEntry::Dir(*worktree_id, entries.last()?.id))
                }
                PanelEntry::Outline(OutlineEntry::Excerpt(buffer_id, excerpt_id, _)) => {
                    Some(CollapsedEntry::Excerpt(*buffer_id, *excerpt_id))
                }
                PanelEntry::Search(_) | PanelEntry::Outline(..) => None,
            })
            .collect::<Vec<_>>();
        self.collapsed_entries.extend(new_entries);
        self.update_cached_entries(None, cx);
    }

    fn toggle_expanded(&mut self, entry: &PanelEntry, cx: &mut ViewContext<Self>) {
        match entry {
            PanelEntry::Fs(FsEntry::Directory(worktree_id, dir_entry)) => {
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
            PanelEntry::Fs(FsEntry::File(worktree_id, _, buffer_id, _)) => {
                let collapsed_entry = CollapsedEntry::File(*worktree_id, *buffer_id);
                if !self.collapsed_entries.remove(&collapsed_entry) {
                    self.collapsed_entries.insert(collapsed_entry);
                }
            }
            PanelEntry::Fs(FsEntry::ExternalFile(buffer_id, _)) => {
                let collapsed_entry = CollapsedEntry::ExternalFile(*buffer_id);
                if !self.collapsed_entries.remove(&collapsed_entry) {
                    self.collapsed_entries.insert(collapsed_entry);
                }
            }
            PanelEntry::FoldedDirs(worktree_id, dir_entries) => {
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
            PanelEntry::Outline(OutlineEntry::Excerpt(buffer_id, excerpt_id, _)) => {
                let collapsed_entry = CollapsedEntry::Excerpt(*buffer_id, *excerpt_id);
                if !self.collapsed_entries.remove(&collapsed_entry) {
                    self.collapsed_entries.insert(collapsed_entry);
                }
            }
            PanelEntry::Search(_) | PanelEntry::Outline(..) => return,
        }

        self.select_entry(entry.clone(), true, cx);
        self.update_cached_entries(None, cx);
    }

    fn copy_path(&mut self, _: &CopyPath, cx: &mut ViewContext<Self>) {
        if let Some(clipboard_text) = self
            .selected_entry()
            .and_then(|entry| self.abs_path(&entry, cx))
            .map(|p| p.to_string_lossy().to_string())
        {
            cx.write_to_clipboard(ClipboardItem::new_string(clipboard_text));
        }
    }

    fn copy_relative_path(&mut self, _: &CopyRelativePath, cx: &mut ViewContext<Self>) {
        if let Some(clipboard_text) = self
            .selected_entry()
            .and_then(|entry| match entry {
                PanelEntry::Fs(entry) => self.relative_path(&entry, cx),
                PanelEntry::FoldedDirs(_, dirs) => dirs.last().map(|entry| entry.path.clone()),
                PanelEntry::Search(_) | PanelEntry::Outline(..) => None,
            })
            .map(|p| p.to_string_lossy().to_string())
        {
            cx.write_to_clipboard(ClipboardItem::new_string(clipboard_text));
        }
    }

    fn reveal_in_finder(&mut self, _: &RevealInFileManager, cx: &mut ViewContext<Self>) {
        if let Some(abs_path) = self
            .selected_entry()
            .and_then(|entry| self.abs_path(&entry, cx))
        {
            cx.reveal_path(&abs_path);
        }
    }

    fn open_in_terminal(&mut self, _: &OpenInTerminal, cx: &mut ViewContext<Self>) {
        let selected_entry = self.selected_entry();
        let abs_path = selected_entry.and_then(|entry| self.abs_path(&entry, cx));
        let working_directory = if let (
            Some(abs_path),
            Some(PanelEntry::Fs(FsEntry::File(..) | FsEntry::ExternalFile(..))),
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
        if !self.active {
            return;
        }
        if !OutlinePanelSettings::get_global(cx).auto_reveal_entries {
            return;
        }
        let Some(entry_with_selection) = self.location_for_editor_selection(editor, cx) else {
            self.selected_entry = SelectedEntry::None;
            cx.notify();
            return;
        };

        let project = self.project.clone();
        self.reveal_selection_task = cx.spawn(|outline_panel, mut cx| async move {
            cx.background_executor().timer(UPDATE_DEBOUNCE).await;
            let related_buffer_entry = match &entry_with_selection {
                PanelEntry::Fs(FsEntry::File(worktree_id, _, buffer_id, _)) => {
                    project.update(&mut cx, |project, cx| {
                        let entry_id = project
                            .buffer_for_id(*buffer_id, cx)
                            .and_then(|buffer| buffer.read(cx).entry_id(cx));
                        project
                            .worktree_for_id(*worktree_id, cx)
                            .zip(entry_id)
                            .and_then(|(worktree, entry_id)| {
                                let entry = worktree.read(cx).entry_for_id(entry_id)?.clone();
                                Some((worktree, entry))
                            })
                    })?
                }
                PanelEntry::Outline(outline_entry) => {
                    let &(OutlineEntry::Outline(buffer_id, excerpt_id, _)
                    | OutlineEntry::Excerpt(buffer_id, excerpt_id, _)) = outline_entry;
                    outline_panel.update(&mut cx, |outline_panel, cx| {
                        outline_panel
                            .collapsed_entries
                            .remove(&CollapsedEntry::ExternalFile(buffer_id));
                        outline_panel
                            .collapsed_entries
                            .remove(&CollapsedEntry::Excerpt(buffer_id, excerpt_id));
                        let project = outline_panel.project.read(cx);
                        let entry_id = project
                            .buffer_for_id(buffer_id, cx)
                            .and_then(|buffer| buffer.read(cx).entry_id(cx));

                        entry_id.and_then(|entry_id| {
                            project
                                .worktree_for_entry(entry_id, cx)
                                .and_then(|worktree| {
                                    let worktree_id = worktree.read(cx).id();
                                    outline_panel
                                        .collapsed_entries
                                        .remove(&CollapsedEntry::File(worktree_id, buffer_id));
                                    let entry = worktree.read(cx).entry_for_id(entry_id)?.clone();
                                    Some((worktree, entry))
                                })
                        })
                    })?
                }
                PanelEntry::Fs(FsEntry::ExternalFile(..)) => None,
                PanelEntry::Search(SearchEntry { match_range, .. }) => match_range
                    .start
                    .buffer_id
                    .or(match_range.end.buffer_id)
                    .map(|buffer_id| {
                        outline_panel.update(&mut cx, |outline_panel, cx| {
                            outline_panel
                                .collapsed_entries
                                .remove(&CollapsedEntry::ExternalFile(buffer_id));
                            let project = project.read(cx);
                            let entry_id = project
                                .buffer_for_id(buffer_id, cx)
                                .and_then(|buffer| buffer.read(cx).entry_id(cx));

                            entry_id.and_then(|entry_id| {
                                project
                                    .worktree_for_entry(entry_id, cx)
                                    .and_then(|worktree| {
                                        let worktree_id = worktree.read(cx).id();
                                        outline_panel
                                            .collapsed_entries
                                            .remove(&CollapsedEntry::File(worktree_id, buffer_id));
                                        let entry =
                                            worktree.read(cx).entry_for_id(entry_id)?.clone();
                                        Some((worktree, entry))
                                    })
                            })
                        })
                    })
                    .transpose()?
                    .flatten(),
                _ => return anyhow::Ok(()),
            };
            if let Some((worktree, buffer_entry)) = related_buffer_entry {
                outline_panel.update(&mut cx, |outline_panel, cx| {
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
                                if outline_panel
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
                        project
                            .update(cx, |project, cx| {
                                project.expand_entry(worktree_id, dir_to_expand, cx)
                            })
                            .unwrap_or_else(|| Task::ready(Ok(())))
                            .detach_and_log_err(cx)
                    }
                })?
            }

            outline_panel.update(&mut cx, |outline_panel, cx| {
                outline_panel.select_entry(entry_with_selection, false, cx);
                outline_panel.update_cached_entries(None, cx);
            })?;

            anyhow::Ok(())
        });
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
        let is_active = match self.selected_entry() {
            Some(PanelEntry::Outline(OutlineEntry::Excerpt(
                selected_buffer_id,
                selected_excerpt_id,
                _,
            ))) => selected_buffer_id == &buffer_id && selected_excerpt_id == &excerpt_id,
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
            "Lines {}- {}",
            excerpt_range.start.row + 1,
            excerpt_range.end.row + 1,
        ))
        .single_line()
        .color(color)
        .into_any_element();

        Some(self.entry_element(
            PanelEntry::Outline(OutlineEntry::Excerpt(buffer_id, excerpt_id, range.clone())),
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
        string_match: Option<&StringMatch>,
        cx: &mut ViewContext<Self>,
    ) -> Stateful<Div> {
        let (item_id, label_element) = (
            ElementId::from(SharedString::from(format!(
                "{buffer_id:?}|{excerpt_id:?}{:?}|{:?}",
                rendered_outline.range, &rendered_outline.text,
            ))),
            language::render_item(
                &rendered_outline,
                string_match
                    .map(|string_match| string_match.ranges().collect::<Vec<_>>())
                    .unwrap_or_default(),
                cx,
            )
            .into_any_element(),
        );
        let is_active = match self.selected_entry() {
            Some(PanelEntry::Outline(OutlineEntry::Outline(
                selected_buffer_id,
                selected_excerpt_id,
                selected_entry,
            ))) => {
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
            PanelEntry::Outline(OutlineEntry::Outline(
                buffer_id,
                excerpt_id,
                rendered_outline.clone(),
            )),
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
        string_match: Option<&StringMatch>,
        cx: &mut ViewContext<Self>,
    ) -> Stateful<Div> {
        let settings = OutlinePanelSettings::get_global(cx);
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

    fn render_folded_dirs(
        &self,
        worktree_id: WorktreeId,
        dir_entries: &[Entry],
        depth: usize,
        string_match: Option<&StringMatch>,
        cx: &mut ViewContext<OutlinePanel>,
    ) -> Stateful<Div> {
        let settings = OutlinePanelSettings::get_global(cx);
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

    fn render_search_match(
        &self,
        match_range: &Range<editor::Anchor>,
        search_data: &SearchData,
        kind: SearchKind,
        depth: usize,
        string_match: Option<&StringMatch>,
        cx: &mut ViewContext<Self>,
    ) -> Stateful<Div> {
        let search_matches = string_match
            .iter()
            .flat_map(|string_match| string_match.ranges())
            .collect::<Vec<_>>();
        let match_ranges = if search_matches.is_empty() {
            &search_data.search_match_indices
        } else {
            &search_matches
        };
        let label_element = language::render_item(
            &OutlineItem {
                depth,
                annotation_range: None,
                range: search_data.context_range.clone(),
                text: search_data.context_text.clone(),
                highlight_ranges: search_data.highlight_ranges.clone(),
                name_ranges: search_data.search_match_indices.clone(),
                body_range: Some(search_data.context_range.clone()),
            },
            match_ranges.into_iter().cloned(),
            cx,
        )
        .into_any_element();

        let is_active = match self.selected_entry() {
            Some(PanelEntry::Search(SearchEntry {
                match_range: selected_match_range,
                ..
            })) => match_range == selected_match_range,
            _ => false,
        };
        self.entry_element(
            PanelEntry::Search(SearchEntry {
                kind,
                match_range: match_range.clone(),
                same_line_matches: Vec::new(),
                render_data: Some(OnceCell::new()),
            }),
            ElementId::from(SharedString::from(format!("search-{match_range:?}"))),
            depth,
            None,
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
        cx: &mut ViewContext<OutlinePanel>,
    ) -> Stateful<Div> {
        let settings = OutlinePanelSettings::get_global(cx);
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
                    })
                    .on_secondary_mouse_down(cx.listener(
                        move |outline_panel, event: &MouseDownEvent, cx| {
                            // Stop propagation to prevent the catch-all context menu for the project
                            // panel from being deployed.
                            cx.stop_propagation();
                            outline_panel.deploy_context_menu(
                                event.position,
                                rendered_entry.clone(),
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

    fn update_fs_entries(
        &mut self,
        active_editor: &View<Editor>,
        new_entries: HashSet<ExcerptId>,
        debounce: Option<Duration>,
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
                    None => ExcerptOutlines::NotFetched,
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

        self.updating_fs_entries = true;
        self.fs_entries_update_task = cx.spawn(|outline_panel, mut cx| async move {
            if let Some(debounce) = debounce {
                cx.background_executor().timer(debounce).await;
            }
            let Some((
                new_collapsed_entries,
                new_unfolded_dirs,
                new_fs_entries,
                new_depth_map,
                new_children_count,
            )) = cx
                .background_executor()
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
                                        .remove(&CollapsedEntry::File(worktree.id(), buffer_id));
                                }
                                None => {
                                    new_collapsed_entries
                                        .remove(&CollapsedEntry::ExternalFile(buffer_id));
                                }
                            }
                        }

                        if let Some(worktree) = worktree {
                            let worktree_id = worktree.id();
                            let unfolded_dirs = new_unfolded_dirs.entry(worktree_id).or_default();

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
                                                new_collapsed_entries.remove(&CollapsedEntry::Dir(
                                                    worktree_id,
                                                    current_entry.id,
                                                ));
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
                                        .entry(worktree_id)
                                        .or_insert_with(|| (worktree.clone(), HashSet::default()))
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

                    let mut new_children_count =
                        HashMap::<WorktreeId, HashMap<Arc<Path>, FsChildren>>::default();

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
                                                let children = new_children_count
                                                    .entry(worktree_id)
                                                    .or_default()
                                                    .entry(Arc::from(parent))
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

                                    let depth = if root_entries.contains(&dir_entry.id) {
                                        0
                                    } else {
                                        if auto_fold_dirs {
                                            let children = new_children_count
                                                .get(&worktree_id)
                                                .and_then(|children_count| {
                                                    children_count.get(&dir_entry.path)
                                                })
                                                .copied()
                                                .unwrap_or_default();

                                            if !children.may_be_fold_part()
                                                || (children.dirs == 0
                                                    && visited_dirs
                                                        .last()
                                                        .map(|(parent_dir_id, _)| {
                                                            new_unfolded_dirs
                                                                .get(&worktree_id)
                                                                .map_or(true, |unfolded_dirs| {
                                                                    unfolded_dirs
                                                                        .contains(&parent_dir_id)
                                                                })
                                                        })
                                                        .unwrap_or(true))
                                            {
                                                new_unfolded_dirs
                                                    .entry(*worktree_id)
                                                    .or_default()
                                                    .insert(dir_entry.id);
                                            }
                                        }

                                        parent_id
                                            .and_then(|(worktree_id, id)| {
                                                new_depth_map.get(&(worktree_id, id)).copied()
                                            })
                                            .unwrap_or(0)
                                            + 1
                                    };
                                    visited_dirs.push((dir_entry.id, dir_entry.path.clone()));
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
                        new_children_count,
                    ))
                })
                .await
                .log_err()
            else {
                return;
            };

            outline_panel
                .update(&mut cx, |outline_panel, cx| {
                    outline_panel.updating_fs_entries = false;
                    outline_panel.excerpts = new_excerpts;
                    outline_panel.collapsed_entries = new_collapsed_entries;
                    outline_panel.unfolded_dirs = new_unfolded_dirs;
                    outline_panel.fs_entries = new_fs_entries;
                    outline_panel.fs_entries_depth = new_depth_map;
                    outline_panel.fs_children_count = new_children_count;
                    outline_panel.update_cached_entries(Some(UPDATE_DEBOUNCE), cx);
                    outline_panel.update_non_fs_items(cx);

                    cx.notify();
                })
                .ok();
        });
    }

    fn replace_active_editor(
        &mut self,
        new_active_editor: View<Editor>,
        cx: &mut ViewContext<Self>,
    ) {
        self.clear_previous(cx);
        let buffer_search_subscription = cx.subscribe(
            &new_active_editor,
            |outline_panel: &mut Self, _, _: &SearchEvent, cx: &mut ViewContext<'_, Self>| {
                outline_panel.update_search_matches(cx);
                outline_panel.autoscroll(cx);
            },
        );
        self.active_item = Some(ActiveItem {
            _buffer_search_subscription: buffer_search_subscription,
            _editor_subscrpiption: subscribe_for_editor_events(&new_active_editor, cx),
            active_editor: new_active_editor.downgrade(),
        });
        let new_entries =
            HashSet::from_iter(new_active_editor.read(cx).buffer().read(cx).excerpt_ids());
        self.selected_entry.invalidate();
        self.update_fs_entries(&new_active_editor, new_entries, None, cx);
    }

    fn clear_previous(&mut self, cx: &mut WindowContext<'_>) {
        self.filter_editor.update(cx, |editor, cx| editor.clear(cx));
        self.collapsed_entries.clear();
        self.unfolded_dirs.clear();
        self.selected_entry = SelectedEntry::None;
        self.fs_entries_update_task = Task::ready(());
        self.cached_entries_update_task = Task::ready(());
        self.active_item = None;
        self.fs_entries.clear();
        self.fs_entries_depth.clear();
        self.fs_children_count.clear();
        self.outline_fetch_tasks.clear();
        self.excerpts.clear();
        self.cached_entries = Vec::new();
        self.search_matches.clear();
        self.search = None;
        self.pinned = false;
    }

    fn location_for_editor_selection(
        &mut self,
        editor: &View<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> Option<PanelEntry> {
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

        match self.mode {
            ItemsDisplayMode::Search => self
                .search_matches
                .iter()
                .rev()
                .min_by_key(|&match_range| {
                    let match_display_range =
                        match_range.clone().to_display_points(&editor_snapshot);
                    let start_distance = if selection_display_point < match_display_range.start {
                        match_display_range.start - selection_display_point
                    } else {
                        selection_display_point - match_display_range.start
                    };
                    let end_distance = if selection_display_point < match_display_range.end {
                        match_display_range.end - selection_display_point
                    } else {
                        selection_display_point - match_display_range.end
                    };
                    start_distance + end_distance
                })
                .and_then(|closest_range| {
                    self.cached_entries.iter().find_map(|cached_entry| {
                        if let PanelEntry::Search(SearchEntry {
                            match_range,
                            same_line_matches,
                            ..
                        }) = &cached_entry.entry
                        {
                            if match_range == closest_range
                                || same_line_matches.contains(&closest_range)
                            {
                                Some(cached_entry.entry.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                }),
            ItemsDisplayMode::Outline => self.outline_location(
                buffer_id,
                excerpt_id,
                multi_buffer_snapshot,
                editor_snapshot,
                selection_display_point,
            ),
        }
    }

    fn outline_location(
        &mut self,
        buffer_id: BufferId,
        excerpt_id: ExcerptId,
        multi_buffer_snapshot: editor::MultiBufferSnapshot,
        editor_snapshot: editor::EditorSnapshot,
        selection_display_point: DisplayPoint,
    ) -> Option<PanelEntry> {
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
            Some(outline) => {
                PanelEntry::Outline(OutlineEntry::Outline(buffer_id, excerpt_id, outline))
            }
            None => {
                self.cached_entries.iter().rev().find_map(|cached_entry| {
                    match &cached_entry.entry {
                        PanelEntry::Outline(OutlineEntry::Excerpt(
                            entry_buffer_id,
                            entry_excerpt_id,
                            _,
                        )) => {
                            if entry_buffer_id == &buffer_id && entry_excerpt_id == &excerpt_id {
                                Some(cached_entry.entry.clone())
                            } else {
                                None
                            }
                        }
                        PanelEntry::Fs(
                            FsEntry::ExternalFile(file_buffer_id, file_excerpts)
                            | FsEntry::File(_, _, file_buffer_id, file_excerpts),
                        ) => {
                            if file_buffer_id == &buffer_id && file_excerpts.contains(&excerpt_id) {
                                Some(cached_entry.entry.clone())
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                })?
            }
        };
        Some(closest_container)
    }

    fn fetch_outdated_outlines(&mut self, cx: &mut ViewContext<Self>) {
        let excerpt_fetch_ranges = self.excerpt_fetch_ranges(cx);
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
                                outline_panel.update_cached_entries(Some(UPDATE_DEBOUNCE), cx);
                            })
                            .ok();
                    }),
                );
            }
        }
    }

    fn is_singleton_active(&self, cx: &AppContext) -> bool {
        self.active_editor().map_or(false, |active_editor| {
            active_editor.read(cx).buffer().read(cx).is_singleton()
        })
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
        cx: &AppContext,
    ) -> HashMap<
        BufferId,
        (
            BufferSnapshot,
            HashMap<ExcerptId, ExcerptRange<language::Anchor>>,
        ),
    > {
        self.fs_entries
            .iter()
            .fold(HashMap::default(), |mut excerpts_to_fetch, fs_entry| {
                match fs_entry {
                    FsEntry::File(_, _, buffer_id, file_excerpts)
                    | FsEntry::ExternalFile(buffer_id, file_excerpts) => {
                        let excerpts = self.excerpts.get(&buffer_id);
                        for &file_excerpt in file_excerpts {
                            if let Some(excerpt) = excerpts
                                .and_then(|excerpts| excerpts.get(&file_excerpt))
                                .filter(|excerpt| excerpt.should_fetch_outlines())
                            {
                                match excerpts_to_fetch.entry(*buffer_id) {
                                    hash_map::Entry::Occupied(mut o) => {
                                        o.get_mut().1.insert(file_excerpt, excerpt.range.clone());
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
                    FsEntry::Directory(..) => {}
                }
                excerpts_to_fetch
            })
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

    fn abs_path(&self, entry: &PanelEntry, cx: &AppContext) -> Option<PathBuf> {
        match entry {
            PanelEntry::Fs(
                FsEntry::File(_, _, buffer_id, _) | FsEntry::ExternalFile(buffer_id, _),
            ) => self
                .buffer_snapshot_for_id(*buffer_id, cx)
                .and_then(|buffer_snapshot| {
                    let file = File::from_dyn(buffer_snapshot.file())?;
                    file.worktree.read(cx).absolutize(&file.path).ok()
                }),
            PanelEntry::Fs(FsEntry::Directory(worktree_id, entry)) => self
                .project
                .read(cx)
                .worktree_for_id(*worktree_id, cx)?
                .read(cx)
                .absolutize(&entry.path)
                .ok(),
            PanelEntry::FoldedDirs(worktree_id, dirs) => dirs.last().and_then(|entry| {
                self.project
                    .read(cx)
                    .worktree_for_id(*worktree_id, cx)
                    .and_then(|worktree| worktree.read(cx).absolutize(&entry.path).ok())
            }),
            PanelEntry::Search(_) | PanelEntry::Outline(..) => None,
        }
    }

    fn relative_path(&self, entry: &FsEntry, cx: &AppContext) -> Option<Arc<Path>> {
        match entry {
            FsEntry::ExternalFile(buffer_id, _) => {
                let buffer_snapshot = self.buffer_snapshot_for_id(*buffer_id, cx)?;
                Some(buffer_snapshot.file()?.path().clone())
            }
            FsEntry::Directory(_, entry) => Some(entry.path.clone()),
            FsEntry::File(_, entry, ..) => Some(entry.path.clone()),
        }
    }

    fn update_cached_entries(
        &mut self,
        debounce: Option<Duration>,
        cx: &mut ViewContext<OutlinePanel>,
    ) {
        if !self.active {
            return;
        }

        let is_singleton = self.is_singleton_active(cx);
        let query = self.query(cx);
        self.cached_entries_update_task = cx.spawn(|outline_panel, mut cx| async move {
            if let Some(debounce) = debounce {
                cx.background_executor().timer(debounce).await;
            }
            let Some(new_cached_entries) = outline_panel
                .update(&mut cx, |outline_panel, cx| {
                    outline_panel.generate_cached_entries(is_singleton, query, cx)
                })
                .ok()
            else {
                return;
            };
            let new_cached_entries = new_cached_entries.await;
            outline_panel
                .update(&mut cx, |outline_panel, cx| {
                    outline_panel.cached_entries = new_cached_entries;
                    if outline_panel.selected_entry.is_invalidated() {
                        if let Some(new_selected_entry) =
                            outline_panel.active_editor().and_then(|active_editor| {
                                outline_panel.location_for_editor_selection(&active_editor, cx)
                            })
                        {
                            outline_panel.select_entry(new_selected_entry, false, cx);
                        }
                    }

                    outline_panel.autoscroll(cx);
                    cx.notify();
                })
                .ok();
        });
    }

    fn generate_cached_entries(
        &self,
        is_singleton: bool,
        query: Option<String>,
        cx: &mut ViewContext<'_, Self>,
    ) -> Task<Vec<CachedEntry>> {
        let project = self.project.clone();
        cx.spawn(|outline_panel, mut cx| async move {
            let mut entries = Vec::new();
            let mut match_candidates = Vec::new();

            let Ok(()) = outline_panel.update(&mut cx, |outline_panel, cx| {
                let auto_fold_dirs = OutlinePanelSettings::get_global(cx).auto_fold_dirs;
                let mut folded_dirs_entry = None::<(usize, WorktreeId, Vec<Entry>)>;
                let track_matches = query.is_some();
                let mut parent_dirs = Vec::<(&Path, bool, bool, usize)>::new();

                for entry in &outline_panel.fs_entries {
                    let is_expanded = outline_panel.is_expanded(entry);
                    let (depth, should_add) = match entry {
                        FsEntry::Directory(worktree_id, dir_entry) => {
                            let is_root = project
                                .read(cx)
                                .worktree_for_id(*worktree_id, cx)
                                .map_or(false, |worktree| {
                                    worktree.read(cx).root_entry() == Some(dir_entry)
                                });
                            let folded = auto_fold_dirs
                                && !is_root
                                && outline_panel
                                    .unfolded_dirs
                                    .get(worktree_id)
                                    .map_or(true, |unfolded_dirs| {
                                        !unfolded_dirs.contains(&dir_entry.id)
                                    });
                            let fs_depth = outline_panel
                                .fs_entries_depth
                                .get(&(*worktree_id, dir_entry.id))
                                .copied()
                                .unwrap_or(0);
                            while let Some(&(previous_path, ..)) = parent_dirs.last() {
                                if dir_entry.path.starts_with(previous_path) {
                                    break;
                                }
                                parent_dirs.pop();
                            }
                            let auto_fold = match parent_dirs.last() {
                                Some((parent_path, parent_folded, _, _)) => {
                                    *parent_folded
                                        && Some(*parent_path) == dir_entry.path.parent()
                                        && outline_panel
                                            .fs_children_count
                                            .get(worktree_id)
                                            .and_then(|entries| entries.get(&dir_entry.path))
                                            .copied()
                                            .unwrap_or_default()
                                            .may_be_fold_part()
                                }
                                None => false,
                            };
                            let folded = folded || auto_fold;
                            let (depth, parent_expanded) = match parent_dirs.last() {
                                Some(&(_, previous_folded, previous_expanded, previous_depth)) => {
                                    let new_depth = if folded && previous_folded {
                                        previous_depth
                                    } else {
                                        previous_depth + 1
                                    };
                                    parent_dirs.push((
                                        &dir_entry.path,
                                        folded,
                                        previous_expanded && is_expanded,
                                        new_depth,
                                    ));
                                    (new_depth, previous_expanded)
                                }
                                None => {
                                    parent_dirs.push((
                                        &dir_entry.path,
                                        folded,
                                        is_expanded,
                                        fs_depth,
                                    ));
                                    (fs_depth, true)
                                }
                            };

                            if let Some((folded_depth, folded_worktree_id, mut folded_dirs)) =
                                folded_dirs_entry.take()
                            {
                                if folded
                                    && worktree_id == &folded_worktree_id
                                    && dir_entry.path.parent()
                                        == folded_dirs.last().map(|entry| entry.path.as_ref())
                                {
                                    folded_dirs.push(dir_entry.clone());
                                    folded_dirs_entry =
                                        Some((folded_depth, folded_worktree_id, folded_dirs))
                                } else {
                                    if !is_singleton && (parent_expanded || query.is_some()) {
                                        let new_folded_dirs =
                                            PanelEntry::FoldedDirs(folded_worktree_id, folded_dirs);
                                        outline_panel.push_entry(
                                            &mut entries,
                                            &mut match_candidates,
                                            track_matches,
                                            new_folded_dirs,
                                            folded_depth,
                                            cx,
                                        );
                                    }
                                    folded_dirs_entry =
                                        Some((depth, *worktree_id, vec![dir_entry.clone()]))
                                }
                            } else if folded {
                                folded_dirs_entry =
                                    Some((depth, *worktree_id, vec![dir_entry.clone()]));
                            }

                            let should_add = parent_expanded && folded_dirs_entry.is_none();
                            (depth, should_add)
                        }
                        FsEntry::ExternalFile(..) => {
                            if let Some((folded_depth, worktree_id, folded_dirs)) =
                                folded_dirs_entry.take()
                            {
                                let parent_expanded = parent_dirs
                                    .iter()
                                    .rev()
                                    .find(|(parent_path, ..)| {
                                        folded_dirs
                                            .iter()
                                            .all(|entry| entry.path.as_ref() != *parent_path)
                                    })
                                    .map_or(true, |&(_, _, parent_expanded, _)| parent_expanded);
                                if !is_singleton && (parent_expanded || query.is_some()) {
                                    outline_panel.push_entry(
                                        &mut entries,
                                        &mut match_candidates,
                                        track_matches,
                                        PanelEntry::FoldedDirs(worktree_id, folded_dirs),
                                        folded_depth,
                                        cx,
                                    );
                                }
                            }
                            parent_dirs.clear();
                            (0, true)
                        }
                        FsEntry::File(worktree_id, file_entry, ..) => {
                            if let Some((folded_depth, worktree_id, folded_dirs)) =
                                folded_dirs_entry.take()
                            {
                                let parent_expanded = parent_dirs
                                    .iter()
                                    .rev()
                                    .find(|(parent_path, ..)| {
                                        folded_dirs
                                            .iter()
                                            .all(|entry| entry.path.as_ref() != *parent_path)
                                    })
                                    .map_or(true, |&(_, _, parent_expanded, _)| parent_expanded);
                                if !is_singleton && (parent_expanded || query.is_some()) {
                                    outline_panel.push_entry(
                                        &mut entries,
                                        &mut match_candidates,
                                        track_matches,
                                        PanelEntry::FoldedDirs(worktree_id, folded_dirs),
                                        folded_depth,
                                        cx,
                                    );
                                }
                            }

                            let fs_depth = outline_panel
                                .fs_entries_depth
                                .get(&(*worktree_id, file_entry.id))
                                .copied()
                                .unwrap_or(0);
                            while let Some(&(previous_path, ..)) = parent_dirs.last() {
                                if file_entry.path.starts_with(previous_path) {
                                    break;
                                }
                                parent_dirs.pop();
                            }
                            let (depth, should_add) = match parent_dirs.last() {
                                Some(&(_, _, previous_expanded, previous_depth)) => {
                                    let new_depth = previous_depth + 1;
                                    (new_depth, previous_expanded)
                                }
                                None => (fs_depth, true),
                            };
                            (depth, should_add)
                        }
                    };

                    if !is_singleton
                        && (should_add || (query.is_some() && folded_dirs_entry.is_none()))
                    {
                        outline_panel.push_entry(
                            &mut entries,
                            &mut match_candidates,
                            track_matches,
                            PanelEntry::Fs(entry.clone()),
                            depth,
                            cx,
                        );
                    }

                    match outline_panel.mode {
                        ItemsDisplayMode::Search => {
                            if is_singleton || query.is_some() || (should_add && is_expanded) {
                                outline_panel.add_search_entries(
                                    entry,
                                    depth,
                                    track_matches,
                                    is_singleton,
                                    &mut entries,
                                    &mut match_candidates,
                                    cx,
                                );
                            }
                        }
                        ItemsDisplayMode::Outline => {
                            let excerpts_to_consider =
                                if is_singleton || query.is_some() || (should_add && is_expanded) {
                                    match entry {
                                        FsEntry::File(_, _, buffer_id, entry_excerpts) => {
                                            Some((*buffer_id, entry_excerpts))
                                        }
                                        FsEntry::ExternalFile(buffer_id, entry_excerpts) => {
                                            Some((*buffer_id, entry_excerpts))
                                        }
                                        _ => None,
                                    }
                                } else {
                                    None
                                };
                            if let Some((buffer_id, entry_excerpts)) = excerpts_to_consider {
                                outline_panel.add_excerpt_entries(
                                    buffer_id,
                                    entry_excerpts,
                                    depth,
                                    track_matches,
                                    is_singleton,
                                    query.as_deref(),
                                    &mut entries,
                                    &mut match_candidates,
                                    cx,
                                );
                            }
                        }
                    }

                    if is_singleton
                        && matches!(entry, FsEntry::File(..) | FsEntry::ExternalFile(..))
                        && !entries.iter().any(|item| {
                            matches!(item.entry, PanelEntry::Outline(..) | PanelEntry::Search(_))
                        })
                    {
                        outline_panel.push_entry(
                            &mut entries,
                            &mut match_candidates,
                            track_matches,
                            PanelEntry::Fs(entry.clone()),
                            0,
                            cx,
                        );
                    }
                }

                if let Some((folded_depth, worktree_id, folded_dirs)) = folded_dirs_entry.take() {
                    let parent_expanded = parent_dirs
                        .iter()
                        .rev()
                        .find(|(parent_path, ..)| {
                            folded_dirs
                                .iter()
                                .all(|entry| entry.path.as_ref() != *parent_path)
                        })
                        .map_or(true, |&(_, _, parent_expanded, _)| parent_expanded);
                    if parent_expanded || query.is_some() {
                        outline_panel.push_entry(
                            &mut entries,
                            &mut match_candidates,
                            track_matches,
                            PanelEntry::FoldedDirs(worktree_id, folded_dirs),
                            folded_depth,
                            cx,
                        );
                    }
                }
            }) else {
                return Vec::new();
            };

            let Some(query) = query else {
                return entries;
            };
            let mut matched_ids = match_strings(
                &match_candidates,
                &query,
                true,
                usize::MAX,
                &AtomicBool::default(),
                cx.background_executor().clone(),
            )
            .await
            .into_iter()
            .map(|string_match| (string_match.candidate_id, string_match))
            .collect::<HashMap<_, _>>();

            let mut id = 0;
            entries.retain_mut(|cached_entry| {
                let retain = match matched_ids.remove(&id) {
                    Some(string_match) => {
                        cached_entry.string_match = Some(string_match);
                        true
                    }
                    None => false,
                };
                id += 1;
                retain
            });

            entries
        })
    }

    fn push_entry(
        &self,
        entries: &mut Vec<CachedEntry>,
        match_candidates: &mut Vec<StringMatchCandidate>,
        track_matches: bool,
        entry: PanelEntry,
        depth: usize,
        cx: &mut WindowContext,
    ) {
        if track_matches {
            let id = entries.len();
            match &entry {
                PanelEntry::Fs(fs_entry) => {
                    if let Some(file_name) =
                        self.relative_path(fs_entry, cx).as_deref().map(file_name)
                    {
                        match_candidates.push(StringMatchCandidate {
                            id,
                            string: file_name.to_string(),
                            char_bag: file_name.chars().collect(),
                        });
                    }
                }
                PanelEntry::FoldedDirs(worktree_id, entries) => {
                    let dir_names = self.dir_names_string(entries, *worktree_id, cx);
                    {
                        match_candidates.push(StringMatchCandidate {
                            id,
                            string: dir_names.to_string(),
                            char_bag: dir_names.chars().collect(),
                        });
                    }
                }
                PanelEntry::Outline(outline_entry) => match outline_entry {
                    OutlineEntry::Outline(_, _, outline) => {
                        match_candidates.push(StringMatchCandidate {
                            id,
                            string: outline.text.clone(),
                            char_bag: outline.text.chars().collect(),
                        });
                    }
                    OutlineEntry::Excerpt(..) => {}
                },
                PanelEntry::Search(new_search_entry) => {
                    if let Some(search_data) = new_search_entry
                        .render_data
                        .as_ref()
                        .and_then(|data| data.get())
                    {
                        match_candidates.push(StringMatchCandidate {
                            id,
                            char_bag: search_data.context_text.chars().collect(),
                            string: search_data.context_text.clone(),
                        });
                    }
                }
            }
        }
        entries.push(CachedEntry {
            depth,
            entry,
            string_match: None,
        });
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

    fn query(&self, cx: &AppContext) -> Option<String> {
        let query = self.filter_editor.read(cx).text(cx);
        if query.trim().is_empty() {
            None
        } else {
            Some(query)
        }
    }

    fn is_expanded(&self, entry: &FsEntry) -> bool {
        let entry_to_check = match entry {
            FsEntry::ExternalFile(buffer_id, _) => CollapsedEntry::ExternalFile(*buffer_id),
            FsEntry::File(worktree_id, _, buffer_id, _) => {
                CollapsedEntry::File(*worktree_id, *buffer_id)
            }
            FsEntry::Directory(worktree_id, entry) => CollapsedEntry::Dir(*worktree_id, entry.id),
        };
        !self.collapsed_entries.contains(&entry_to_check)
    }

    fn update_non_fs_items(&mut self, cx: &mut ViewContext<OutlinePanel>) {
        if !self.active {
            return;
        }

        self.update_search_matches(cx);
        self.fetch_outdated_outlines(cx);
        self.autoscroll(cx);
    }

    fn update_search_matches(&mut self, cx: &mut ViewContext<OutlinePanel>) {
        if !self.active {
            return;
        }

        let active_editor = self.active_editor();
        let project_search = self.active_project_search(active_editor.as_ref(), cx);
        let project_search_matches = project_search
            .as_ref()
            .map(|project_search| project_search.read(cx).get_matches(cx))
            .unwrap_or_default();

        let buffer_search = active_editor
            .as_ref()
            .and_then(|active_editor| self.workspace.read(cx).pane_for(active_editor))
            .and_then(|pane| {
                pane.read(cx)
                    .toolbar()
                    .read(cx)
                    .item_of_type::<BufferSearchBar>()
            });
        let buffer_search_matches = active_editor
            .map(|active_editor| active_editor.update(cx, |editor, cx| editor.get_matches(cx)))
            .unwrap_or_default();

        let mut update_cached_entries = false;
        if buffer_search_matches.is_empty() && project_search_matches.is_empty() {
            self.search_matches.clear();
            self.search = None;
            if self.mode == ItemsDisplayMode::Search {
                self.mode = ItemsDisplayMode::Outline;
                update_cached_entries = true;
            }
        } else {
            let new_search_matches = if buffer_search_matches.is_empty() {
                self.search = project_search.map(|project_search| {
                    (
                        SearchKind::Project,
                        project_search.read(cx).search_query_text(cx),
                    )
                });
                project_search_matches
            } else {
                self.search = buffer_search
                    .map(|buffer_search| (SearchKind::Buffer, buffer_search.read(cx).query(cx)));
                buffer_search_matches
            };
            update_cached_entries = self.mode != ItemsDisplayMode::Search
                || self.search_matches.is_empty()
                || self.search_matches != new_search_matches;
            self.search_matches = new_search_matches;
            self.mode = ItemsDisplayMode::Search;
        }
        if update_cached_entries {
            self.selected_entry.invalidate();
            self.update_cached_entries(Some(UPDATE_DEBOUNCE), cx);
        }
    }

    fn active_project_search(
        &mut self,
        for_editor: Option<&View<Editor>>,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<ProjectSearchView>> {
        let for_editor = for_editor?;
        self.workspace
            .read(cx)
            .active_pane()
            .read(cx)
            .items()
            .filter_map(|item| item.downcast::<ProjectSearchView>())
            .find(|project_search| {
                let project_search_editor = project_search.boxed_clone().act_as::<Editor>(cx);
                Some(for_editor) == project_search_editor.as_ref()
            })
    }

    #[allow(clippy::too_many_arguments)]
    fn add_excerpt_entries(
        &self,
        buffer_id: BufferId,
        entries_to_add: &[ExcerptId],
        parent_depth: usize,
        track_matches: bool,
        is_singleton: bool,
        query: Option<&str>,
        entries: &mut Vec<CachedEntry>,
        match_candidates: &mut Vec<StringMatchCandidate>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(excerpts) = self.excerpts.get(&buffer_id) {
            for &excerpt_id in entries_to_add {
                let Some(excerpt) = excerpts.get(&excerpt_id) else {
                    continue;
                };
                let excerpt_depth = parent_depth + 1;
                self.push_entry(
                    entries,
                    match_candidates,
                    track_matches,
                    PanelEntry::Outline(OutlineEntry::Excerpt(
                        buffer_id,
                        excerpt_id,
                        excerpt.range.clone(),
                    )),
                    excerpt_depth,
                    cx,
                );

                let mut outline_base_depth = excerpt_depth + 1;
                if is_singleton {
                    outline_base_depth = 0;
                    entries.clear();
                    match_candidates.clear();
                } else if query.is_none()
                    && self
                        .collapsed_entries
                        .contains(&CollapsedEntry::Excerpt(buffer_id, excerpt_id))
                {
                    continue;
                }

                for outline in excerpt.iter_outlines() {
                    self.push_entry(
                        entries,
                        match_candidates,
                        track_matches,
                        PanelEntry::Outline(OutlineEntry::Outline(
                            buffer_id,
                            excerpt_id,
                            outline.clone(),
                        )),
                        outline_base_depth + outline.depth,
                        cx,
                    );
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn add_search_entries(
        &self,
        entry: &FsEntry,
        parent_depth: usize,
        track_matches: bool,
        is_singleton: bool,
        entries: &mut Vec<CachedEntry>,
        match_candidates: &mut Vec<StringMatchCandidate>,
        cx: &mut ViewContext<Self>,
    ) {
        let related_excerpts = match entry {
            FsEntry::Directory(_, _) => return,
            FsEntry::ExternalFile(_, excerpts) => excerpts,
            FsEntry::File(_, _, _, excerpts) => excerpts,
        }
        .iter()
        .copied()
        .collect::<HashSet<_>>();
        if related_excerpts.is_empty() || self.search_matches.is_empty() {
            return;
        }
        let Some(kind) = self.search.as_ref().map(|&(kind, _)| kind) else {
            return;
        };

        for match_range in &self.search_matches {
            if related_excerpts.contains(&match_range.start.excerpt_id)
                || related_excerpts.contains(&match_range.end.excerpt_id)
            {
                let depth = if is_singleton { 0 } else { parent_depth + 1 };
                let previous_search_entry = entries.last_mut().and_then(|entry| {
                    if let PanelEntry::Search(previous_search_entry) = &mut entry.entry {
                        Some(previous_search_entry)
                    } else {
                        None
                    }
                });
                let mut new_search_entry = SearchEntry {
                    kind,
                    match_range: match_range.clone(),
                    same_line_matches: Vec::new(),
                    render_data: Some(OnceCell::new()),
                };
                if self.init_search_data(previous_search_entry, &mut new_search_entry, cx) {
                    self.push_entry(
                        entries,
                        match_candidates,
                        track_matches,
                        PanelEntry::Search(new_search_entry),
                        depth,
                        cx,
                    );
                }
            }
        }
    }

    fn active_editor(&self) -> Option<View<Editor>> {
        self.active_item.as_ref()?.active_editor.upgrade()
    }

    fn should_replace_active_editor(&self, new_active_editor: &View<Editor>) -> bool {
        self.active_editor().map_or(true, |active_editor| {
            !self.pinned && active_editor.item_id() != new_active_editor.item_id()
        })
    }

    pub fn toggle_active_editor_pin(
        &mut self,
        _: &ToggleActiveEditorPin,
        cx: &mut ViewContext<Self>,
    ) {
        self.pinned = !self.pinned;
        if !self.pinned {
            if let Some(active_editor) = workspace_active_editor(self.workspace.read(cx), cx) {
                if self.should_replace_active_editor(&active_editor) {
                    self.replace_active_editor(active_editor, cx);
                }
            }
        }

        cx.notify();
    }

    fn selected_entry(&self) -> Option<&PanelEntry> {
        match &self.selected_entry {
            SelectedEntry::Invalidated(entry) => entry.as_ref(),
            SelectedEntry::Valid(entry) => Some(entry),
            SelectedEntry::None => None,
        }
    }

    fn init_search_data(
        &self,
        previous_search_entry: Option<&mut SearchEntry>,
        new_search_entry: &mut SearchEntry,
        cx: &WindowContext,
    ) -> bool {
        let Some(active_editor) = self.active_editor() else {
            return false;
        };
        let multi_buffer_snapshot = active_editor.read(cx).buffer().read(cx).snapshot(cx);
        let theme = cx.theme().syntax().clone();
        let previous_search_data = previous_search_entry.and_then(|previous_search_entry| {
            let previous_search_data = previous_search_entry.render_data.as_mut()?;
            previous_search_data.get_or_init(|| {
                SearchData::new(
                    new_search_entry.kind,
                    &previous_search_entry.match_range,
                    &multi_buffer_snapshot,
                    &theme,
                )
            });
            previous_search_data.get_mut()
        });
        let new_search_data = new_search_entry.render_data.as_mut().and_then(|data| {
            data.get_or_init(|| {
                SearchData::new(
                    new_search_entry.kind,
                    &new_search_entry.match_range,
                    &multi_buffer_snapshot,
                    &theme,
                )
            });
            data.get_mut()
        });
        match (previous_search_data, new_search_data) {
            (_, None) => false,
            (None, Some(_)) => true,
            (Some(previous_search_data), Some(new_search_data)) => {
                if previous_search_data.context_range == new_search_data.context_range {
                    previous_search_data
                        .highlight_ranges
                        .append(&mut new_search_data.highlight_ranges);
                    previous_search_data
                        .search_match_indices
                        .append(&mut new_search_data.search_match_indices);
                    false
                } else {
                    true
                }
            }
        }
    }

    fn select_entry(&mut self, entry: PanelEntry, focus: bool, cx: &mut ViewContext<Self>) {
        if focus {
            self.focus_handle.focus(cx);
        }
        self.selected_entry = SelectedEntry::Valid(entry);
        self.autoscroll(cx);
        cx.notify();
    }
}

fn workspace_active_editor(workspace: &Workspace, cx: &AppContext) -> Option<View<Editor>> {
    workspace
        .active_item(cx)?
        .act_as::<Editor>(cx)
        .filter(|editor| editor.read(cx).mode() == EditorMode::Full)
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
            move |settings, _| {
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
        cx.spawn(|outline_panel, mut cx| async move {
            outline_panel
                .update(&mut cx, |outline_panel, cx| {
                    let old_active = outline_panel.active;
                    outline_panel.active = active;
                    if active && old_active != active {
                        if let Some(active_editor) =
                            workspace_active_editor(outline_panel.workspace.read(cx), cx)
                        {
                            if outline_panel.should_replace_active_editor(&active_editor) {
                                outline_panel.replace_active_editor(active_editor, cx);
                            } else {
                                outline_panel.update_fs_entries(
                                    &active_editor,
                                    HashSet::default(),
                                    None,
                                    cx,
                                )
                            }
                        } else if !outline_panel.pinned {
                            outline_panel.clear_previous(cx);
                        }
                    }
                    outline_panel.serialize(cx);
                })
                .ok();
        })
        .detach()
    }
}

impl FocusableView for OutlinePanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.filter_editor.focus_handle(cx).clone()
    }
}

impl EventEmitter<Event> for OutlinePanel {}

impl EventEmitter<PanelEvent> for OutlinePanel {}

impl Render for OutlinePanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let project = self.project.read(cx);
        let query = self.query(cx);
        let pinned = self.pinned;

        let outline_panel = v_flex()
            .id("outline-panel")
            .size_full()
            .relative()
            .key_context(self.dispatch_context(cx))
            .on_action(cx.listener(Self::open))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::select_parent))
            .on_action(cx.listener(Self::expand_selected_entry))
            .on_action(cx.listener(Self::collapse_selected_entry))
            .on_action(cx.listener(Self::expand_all_entries))
            .on_action(cx.listener(Self::collapse_all_entries))
            .on_action(cx.listener(Self::copy_path))
            .on_action(cx.listener(Self::copy_relative_path))
            .on_action(cx.listener(Self::toggle_active_editor_pin))
            .on_action(cx.listener(Self::unfold_directory))
            .on_action(cx.listener(Self::fold_directory))
            .when(project.is_local_or_ssh(), |el| {
                el.on_action(cx.listener(Self::reveal_in_finder))
                    .on_action(cx.listener(Self::open_in_terminal))
            })
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |outline_panel, event: &MouseDownEvent, cx| {
                    if let Some(entry) = outline_panel.selected_entry().cloned() {
                        outline_panel.deploy_context_menu(event.position, entry, cx)
                    } else if let Some(entry) = outline_panel.fs_entries.first().cloned() {
                        outline_panel.deploy_context_menu(event.position, PanelEntry::Fs(entry), cx)
                    }
                }),
            )
            .track_focus(&self.focus_handle);

        if self.cached_entries.is_empty() {
            let header = if self.updating_fs_entries {
                "Loading outlines"
            } else if query.is_some() {
                "No matches for query"
            } else {
                "No outlines available"
            };

            outline_panel.child(
                v_flex()
                    .justify_center()
                    .size_full()
                    .child(h_flex().justify_center().child(Label::new(header)))
                    .when_some(query.clone(), |panel, query| {
                        panel.child(h_flex().justify_center().child(Label::new(query)))
                    })
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
                                Label::new(format!("Toggle this panel with {keystroke}"))
                            }),
                    ),
            )
        } else {
            outline_panel
                .when_some(self.search.as_ref(), |outline_panel, (_, search_query)| {
                    outline_panel.child(
                        div()
                            .mx_2()
                            .child(
                                Label::new(format!("Searching: '{search_query}'"))
                                    .color(Color::Muted),
                            )
                            .child(horizontal_separator(cx)),
                    )
                })
                .child({
                    let items_len = self.cached_entries.len();
                    uniform_list(cx.view().clone(), "entries", items_len, {
                        move |outline_panel, range, cx| {
                            let entries = outline_panel.cached_entries.get(range);
                            entries
                                .map(|entries| entries.to_vec())
                                .unwrap_or_default()
                                .into_iter()
                                .filter_map(|cached_entry| match cached_entry.entry {
                                    PanelEntry::Fs(entry) => Some(outline_panel.render_entry(
                                        &entry,
                                        cached_entry.depth,
                                        cached_entry.string_match.as_ref(),
                                        cx,
                                    )),
                                    PanelEntry::FoldedDirs(worktree_id, entries) => {
                                        Some(outline_panel.render_folded_dirs(
                                            worktree_id,
                                            &entries,
                                            cached_entry.depth,
                                            cached_entry.string_match.as_ref(),
                                            cx,
                                        ))
                                    }
                                    PanelEntry::Outline(OutlineEntry::Excerpt(
                                        buffer_id,
                                        excerpt_id,
                                        excerpt,
                                    )) => outline_panel.render_excerpt(
                                        buffer_id,
                                        excerpt_id,
                                        &excerpt,
                                        cached_entry.depth,
                                        cx,
                                    ),
                                    PanelEntry::Outline(OutlineEntry::Outline(
                                        buffer_id,
                                        excerpt_id,
                                        outline,
                                    )) => Some(outline_panel.render_outline(
                                        buffer_id,
                                        excerpt_id,
                                        &outline,
                                        cached_entry.depth,
                                        cached_entry.string_match.as_ref(),
                                        cx,
                                    )),
                                    PanelEntry::Search(SearchEntry {
                                        match_range,
                                        render_data,
                                        kind,
                                        same_line_matches: _,
                                    }) => render_data.as_ref().and_then(|search_data| {
                                        let search_data = search_data.get()?;
                                        Some(outline_panel.render_search_match(
                                            &match_range,
                                            search_data,
                                            kind,
                                            cached_entry.depth,
                                            cached_entry.string_match.as_ref(),
                                            cx,
                                        ))
                                    }),
                                })
                                .collect()
                        }
                    })
                    .size_full()
                    .track_scroll(self.scroll_handle.clone())
                })
        }
        .children(self.context_menu.as_ref().map(|(menu, position, _)| {
            deferred(
                anchored()
                    .position(*position)
                    .anchor(gpui::AnchorCorner::TopLeft)
                    .child(menu.clone()),
            )
            .with_priority(1)
        }))
        .child(
            v_flex().child(horizontal_separator(cx)).child(
                h_flex().p_2().child(self.filter_editor.clone()).child(
                    div().border_1().child(
                        IconButton::new(
                            "outline-panel-menu",
                            if pinned {
                                IconName::Unpin
                            } else {
                                IconName::Pin
                            },
                        )
                        .tooltip(move |cx| {
                            Tooltip::text(if pinned { "Unpin" } else { "Pin active editor" }, cx)
                        })
                        .shape(IconButtonShape::Square)
                        .on_click(cx.listener(|outline_panel, _, cx| {
                            outline_panel.toggle_active_editor_pin(&ToggleActiveEditorPin, cx);
                        })),
                    ),
                ),
            ),
        )
    }
}

fn subscribe_for_editor_events(
    editor: &View<Editor>,
    cx: &mut ViewContext<OutlinePanel>,
) -> Subscription {
    let debounce = Some(UPDATE_DEBOUNCE);
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
                    debounce,
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
                outline_panel.update_fs_entries(&editor, HashSet::default(), debounce, cx);
            }
            EditorEvent::ExcerptsExpanded { ids } => {
                outline_panel.invalidate_outlines(ids);
                outline_panel.update_non_fs_items(cx);
            }
            EditorEvent::ExcerptsEdited { ids } => {
                outline_panel.invalidate_outlines(ids);
                outline_panel.update_non_fs_items(cx);
            }
            EditorEvent::Reparsed(buffer_id) => {
                if let Some(excerpts) = outline_panel.excerpts.get_mut(buffer_id) {
                    for (_, excerpt) in excerpts {
                        excerpt.invalidate_outlines();
                    }
                }
                outline_panel.update_non_fs_items(cx);
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

fn horizontal_separator(cx: &mut WindowContext) -> Div {
    div().mx_2().border_primary(cx).border_t_1()
}
