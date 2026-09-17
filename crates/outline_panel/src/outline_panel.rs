mod outline_panel_settings;

use anyhow::Context as _;
use collections::{BTreeMap, BTreeSet, HashMap, HashSet, IndexMap};
use db::kvp::KeyValueStore;
use editor::{
    AnchorRangeExt, Bias, DisplayPoint, Editor, EditorEvent, ExcerptRange, MultiBufferSnapshot,
    RangeToAnchorExt, SelectionEffects,
    display_map::ToDisplayPoint,
    items::{entry_git_aware_label_color, entry_label_color},
    scroll::{Autoscroll, ScrollAnchor},
};
use file_icons::FileIcons;

use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use git::status::{FileStatus, GitSummary};
use gpui::{
    Action, AnyElement, App, AppContext as _, AsyncWindowContext, Bounds, ClipboardItem, Context,
    DismissEvent, Div, ElementId, Entity, EventEmitter, FocusHandle, Focusable, HighlightStyle,
    InteractiveElement, IntoElement, KeyContext, ListHorizontalSizingBehavior, ListSizingBehavior,
    MouseButton, MouseDownEvent, ParentElement, Pixels, Point, Render, ScrollStrategy,
    SharedString, Stateful, StatefulInteractiveElement as _, Styled, Subscription, Task,
    UniformListScrollHandle, WeakEntity, Window, actions, anchored, deferred, div, point, px, size,
    uniform_list,
};
use language::{Anchor, BufferId, BufferSnapshot, DiskState, OffsetRangeExt, OutlineItem};
use language::{LanguageAwareStyling, language_settings::LanguageSettings};

use menu::{Cancel, SelectFirst, SelectLast, SelectNext, SelectPrevious};
use std::{
    cmp,
    hash::Hash,
    ops::Range,
    path::{Path, PathBuf},
    sync::{
        Arc, OnceLock,
        atomic::{self, AtomicBool},
    },
    time::Duration,
    u32,
};

use outline_panel_settings::{DockSide, FolderIndicator, OutlinePanelSettings, ShowIndentGuides};
use project::{File, Fs, Project, ProjectPath};
use search::{BufferSearchBar, ProjectSearchView};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use theme::{GlobalTheme, SyntaxTheme};
use theme_settings::ThemeSettings;
use ui::{
    ContextMenu, FluentBuilder, HighlightedLabel, IconButton, IconButtonShape, IndentGuideColors,
    IndentGuideLayout, KeyBinding, ListItem, ScrollAxes, Scrollbars, Tab, Tooltip, WithScrollbar,
    prelude::*,
};
use util::{RangeExt, ResultExt, TryFutureExt, debug_panic, rel_path::RelPath};
use workspace::{
    OpenInTerminal, WeakItemHandle, Workspace,
    dock::{DockPosition, Panel, PanelEvent},
    item::ItemHandle,
    searchable::{SearchEvent, SearchableItem},
};
use worktree::{PathProgress, PathTarget, WorktreeId};

use crate::outline_panel_settings::OutlinePanelSettingsScrollbarProxy;

actions!(
    outline_panel,
    [
        /// Collapses all entries in the outline tree.
        CollapseAllEntries,
        /// Collapses the currently selected entry.
        CollapseSelectedEntry,
        /// Expands all entries in the outline tree.
        ExpandAllEntries,
        /// Expands the currently selected entry.
        ExpandSelectedEntry,
        /// Folds the selected directory.
        FoldDirectory,
        /// Opens the selected entry in the editor.
        OpenSelectedEntry,
        /// Reveals the selected item in the system file manager.
        RevealInFileManager,
        /// Scroll half a page upwards
        ScrollUp,
        /// Scroll half a page downwards
        ScrollDown,
        /// Scroll until the cursor displays at the center
        ScrollCursorCenter,
        /// Scroll until the cursor displays at the top
        ScrollCursorTop,
        /// Scroll until the cursor displays at the bottom
        ScrollCursorBottom,
        /// Selects the parent of the current entry.
        SelectParent,
        /// Toggles the pin status of the active editor.
        ToggleActiveEditorPin,
        /// Toggles showing symbols, excerpts and search matches for multi-buffer views.
        ToggleSymbols,
        /// Unfolds the selected directory.
        UnfoldDirectory,
        /// Toggles the outline panel.
        Toggle,
        /// Toggles focus on the outline panel.
        ToggleFocus,
    ]
);

const OUTLINE_PANEL_KEY: &str = "OutlinePanel";
const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);

type Outline = OutlineItem<language::Anchor>;
type OutlineChildren = HashMap<(Range<Anchor>, usize), bool>;
type HighlightStyleData = Arc<OnceLock<Vec<(Range<usize>, HighlightStyle)>>>;

pub struct OutlinePanel {
    fs: Arc<dyn Fs>,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    active: bool,
    pinned: bool,
    scroll_handle: UniformListScrollHandle,
    rendered_entries_len: usize,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    focus_handle: FocusHandle,
    pending_serialization: Task<Option<()>>,

    fs_entries: Vec<FsEntry>,
    fs_children_count: HashMap<WorktreeId, HashMap<Arc<RelPath>, FsChildren>>,
    collapsed_entries: HashSet<CollapsedEntry>,
    unfolded_dirs: HashMap<WorktreeId, BTreeSet<Arc<RelPath>>>,
    selected_entry: SelectedEntry,
    active_item: Option<ActiveItem>,
    _subscriptions: Vec<Subscription>,
    new_entries_for_fs_update: HashSet<BufferId>,
    fs_entries_update_task: Task<()>,
    fs_entries_update_pending: bool,
    cached_entries_update_task: Task<()>,
    cached_entries_update_pending: Option<CachedEntriesUpdate>,
    reveal_selection_task: Task<anyhow::Result<()>>,
    lsp_outline_refresh_task: Task<()>,
    outline_fetch_tasks: HashMap<BufferId, Task<()>>,
    buffers: HashMap<BufferId, BufferOutlines>,
    cached_entries: Vec<CachedEntry>,
    filter_editor: Entity<Editor>,
    mode: ItemsDisplayMode,
    max_width_item_index: Option<usize>,
    preserve_selection_on_buffer_fold_toggles: HashSet<BufferId>,
    pending_default_expansion_depth: Option<usize>,
    default_depth_applied_buffers: HashSet<BufferId>,
    outline_children_cache: HashMap<BufferId, OutlineChildren>,
    hide_symbols_override: Option<bool>,
}

#[derive(Debug)]
enum ItemsDisplayMode {
    Search(SearchState),
    Outline,
}

#[derive(Debug)]
struct SearchState {
    kind: SearchKind,
    query: String,
    matches: Vec<(Range<editor::Anchor>, Arc<OnceLock<SearchData>>)>,
    highlight_search_match_tx: async_channel::Sender<HighlightArguments>,
    _search_match_highlighter: Task<()>,
    _search_match_notify: Task<()>,
}

struct HighlightArguments {
    multi_buffer_snapshot: MultiBufferSnapshot,
    match_range: Range<editor::Anchor>,
    search_data: Arc<OnceLock<SearchData>>,
}

impl SearchState {
    fn new(
        kind: SearchKind,
        query: String,
        previous_matches: HashMap<Range<editor::Anchor>, Arc<OnceLock<SearchData>>>,
        new_matches: Vec<Range<editor::Anchor>>,
        theme: Arc<SyntaxTheme>,
        window: &mut Window,
        cx: &mut Context<OutlinePanel>,
    ) -> Self {
        let (highlight_search_match_tx, highlight_search_match_rx) = async_channel::unbounded();
        let (notify_tx, notify_rx) = async_channel::unbounded::<()>();
        Self {
            kind,
            query,
            matches: new_matches
                .into_iter()
                .map(|range| {
                    let search_data = previous_matches
                        .get(&range)
                        .map(Arc::clone)
                        .unwrap_or_default();
                    (range, search_data)
                })
                .collect(),
            highlight_search_match_tx,
            _search_match_highlighter: cx.background_spawn(async move {
                while let Ok(highlight_arguments) = highlight_search_match_rx.recv().await {
                    let needs_init = highlight_arguments.search_data.get().is_none();
                    let search_data = highlight_arguments.search_data.get_or_init(|| {
                        SearchData::new(
                            &highlight_arguments.match_range,
                            &highlight_arguments.multi_buffer_snapshot,
                        )
                    });
                    if needs_init {
                        notify_tx.try_send(()).ok();
                    }

                    let highlight_data = &search_data.highlights_data;
                    if highlight_data.get().is_some() {
                        continue;
                    }
                    let mut left_whitespaces_count = 0;
                    let mut non_whitespace_symbol_occurred = false;
                    let context_offset_range = search_data
                        .context_range
                        .to_offset(&highlight_arguments.multi_buffer_snapshot);
                    let mut offset = context_offset_range.start;
                    let mut context_text = String::new();
                    let mut highlight_ranges = Vec::new();
                    for mut chunk in highlight_arguments.multi_buffer_snapshot.chunks(
                        context_offset_range.start..context_offset_range.end,
                        LanguageAwareStyling {
                            tree_sitter: true,
                            diagnostics: true,
                        },
                    ) {
                        if !non_whitespace_symbol_occurred {
                            for c in chunk.text.chars() {
                                if c.is_whitespace() {
                                    left_whitespaces_count += c.len_utf8();
                                } else {
                                    non_whitespace_symbol_occurred = true;
                                    break;
                                }
                            }
                        }

                        if chunk.text.len() > context_offset_range.end - offset {
                            chunk.text = &chunk.text[0..(context_offset_range.end - offset)];
                            offset = context_offset_range.end;
                        } else {
                            offset += chunk.text.len();
                        }
                        let style = chunk
                            .syntax_highlight_id
                            .and_then(|highlight| theme.get(highlight).cloned());

                        if let Some(style) = style {
                            let start = context_text.len();
                            let end = start + chunk.text.len();
                            highlight_ranges.push((start..end, style));
                        }
                        context_text.push_str(chunk.text);
                        if offset >= context_offset_range.end {
                            break;
                        }
                    }

                    highlight_ranges.iter_mut().for_each(|(range, _)| {
                        range.start = range.start.saturating_sub(left_whitespaces_count);
                        range.end = range.end.saturating_sub(left_whitespaces_count);
                    });
                    if highlight_data.set(highlight_ranges).ok().is_some() {
                        notify_tx.try_send(()).ok();
                    }

                    let trimmed_text = context_text[left_whitespaces_count..].to_owned();
                    debug_assert_eq!(
                        trimmed_text, search_data.context_text,
                        "Highlighted text that does not match the buffer text"
                    );
                }
            }),
            _search_match_notify: cx.spawn_in(window, async move |outline_panel, cx| {
                loop {
                    match notify_rx.recv().await {
                        Ok(()) => {}
                        Err(_) => break,
                    };
                    while let Ok(()) = notify_rx.try_recv() {
                        //
                    }
                    let update_result = outline_panel.update(cx, |_, cx| {
                        cx.notify();
                    });
                    if update_result.is_err() {
                        break;
                    }
                }
            }),
        }
    }
}

#[derive(Debug)]
enum SelectedEntry {
    Invalidated(Option<PanelEntry>),
    Valid(PanelEntry, usize),
    None,
}

impl SelectedEntry {
    fn invalidate(&mut self) {
        match std::mem::replace(self, SelectedEntry::None) {
            Self::Valid(entry, _) => *self = Self::Invalidated(Some(entry)),
            Self::None => *self = Self::Invalidated(None),
            other => *self = other,
        }
    }

    fn is_invalidated(&self) -> bool {
        matches!(self, Self::Invalidated(_))
    }
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

#[derive(PartialEq, Eq)]
enum CachedEntriesUpdate {
    Entries,
    Contents,
}

#[derive(Clone, Debug)]
struct CachedEntry {
    depth: usize,
    string_match: Option<StringMatch>,
    entry: PanelEntry,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum CollapsedEntry {
    Dir(WorktreeId, Arc<RelPath>),
    File(WorktreeId, BufferId),
    ExternalFile(BufferId),
    Excerpt(ExcerptRange<Anchor>),
    Outline(Range<Anchor>),
}

struct BufferOutlines {
    excerpts: Vec<ExcerptRange<Anchor>>,
    outlines: OutlineState,
}

struct IndexedOutline<'a> {
    outline: &'a Outline,
    ranked_range: Range<usize>,
    collapsed: bool,
}

struct ExcerptOutlines<'a> {
    excerpt: &'a ExcerptRange<Anchor>,
    ranked_range: Range<usize>,
    expanded: bool,
    visible: Vec<&'a Outline>,
    children_to_cache: Vec<usize>,
}

impl IndexedOutline<'_> {
    fn is_hidden_by(&self, ancestor: &Self) -> bool {
        self.outline.depth > ancestor.outline.depth
            && ancestor.ranked_range.contains(&self.ranked_range.start)
    }

    fn cache_children(&self, next: Option<&Self>, children: &mut OutlineChildren) {
        children.insert(
            (self.outline.range.clone(), self.outline.depth),
            next.is_some_and(|next| next.outline.depth > self.outline.depth),
        );
    }
}

impl BufferOutlines {
    fn invalidate_outlines(&mut self) {
        if let OutlineState::Outlines(valid_outlines) = &mut self.outlines {
            self.outlines = OutlineState::Invalidated(std::mem::take(valid_outlines));
        }
    }

    fn iter_outlines(&self) -> impl Iterator<Item = &Outline> {
        match &self.outlines {
            OutlineState::Outlines(outlines) => outlines.iter(),
            OutlineState::Invalidated(outlines) => outlines.iter(),
            OutlineState::NotFetched => [].iter(),
        }
    }

    fn should_fetch_outlines(&self) -> bool {
        match &self.outlines {
            OutlineState::Outlines(_) => false,
            OutlineState::Invalidated(_) => true,
            OutlineState::NotFetched => true,
        }
    }

    fn assign_outline_cache_owners(
        outlines: &[IndexedOutline<'_>],
        excerpts: &mut [ExcerptOutlines<'_>],
    ) {
        let mut outline_order = (0..outlines.len()).collect::<Vec<_>>();
        let mut excerpt_order = excerpts
            .iter()
            .enumerate()
            .filter(|(_, excerpt)| excerpt.expanded)
            .collect::<Vec<_>>();
        outline_order
            .sort_unstable_by_key(|&index| cmp::Reverse(outlines[index].ranked_range.start));
        excerpt_order.sort_unstable_by_key(|(_, excerpt)| cmp::Reverse(excerpt.ranked_range.end));
        let mut owners = vec![None; outlines.len()];
        let mut frontier = BTreeMap::new();
        let mut cursor = 0;
        for index in outline_order {
            let outline = &outlines[index];
            while let Some(&(excerpt_index, excerpt)) = excerpt_order.get(cursor)
                && excerpt.ranked_range.end >= outline.ranked_range.start
            {
                Self::insert_outline_prefix_minimum(
                    &mut frontier,
                    excerpt.ranked_range.start,
                    cmp::Reverse(excerpt_index),
                );
                cursor += 1;
            }
            let last = frontier.range(..=outline.ranked_range.end).next_back();
            owners[index] = last.map(|(_, &cmp::Reverse(owner))| owner);
        }
        for (index, owner) in owners.into_iter().enumerate() {
            if let Some(owner) = owner {
                excerpts[owner].children_to_cache.push(index);
            }
        }
    }

    fn outlines_are_well_formed(outlines: &[IndexedOutline<'_>]) -> bool {
        if outlines
            .windows(2)
            .any(|pair| pair[0].ranked_range.start > pair[1].ranked_range.start)
        {
            return false;
        }
        let mut ancestors: Vec<&IndexedOutline<'_>> = Vec::new();
        for outline in outlines {
            if outline.ranked_range.start > outline.ranked_range.end {
                return false;
            }
            while let Some(&parent) = ancestors.last()
                && parent.outline.depth >= outline.outline.depth
            {
                ancestors.pop();
                if parent.ranked_range.end > outline.ranked_range.start {
                    return false;
                }
            }
            if ancestors
                .last()
                .is_some_and(|parent| outline.ranked_range.end > parent.ranked_range.end)
            {
                return false;
            }
            ancestors.push(outline);
        }
        true
    }

    fn collect_nested_outlines<'a>(
        outlines: &[IndexedOutline<'a>],
        excerpts: &mut [ExcerptOutlines<'a>],
        children: &mut OutlineChildren,
    ) {
        let mut collapsed_ancestor = None;
        let unfolded = outlines
            .iter()
            .enumerate()
            .filter_map(|(index, outline)| {
                if collapsed_ancestor.is_some_and(|ancestor| outline.is_hidden_by(ancestor)) {
                    return None;
                }
                collapsed_ancestor = outline.collapsed.then_some(outline);
                Some(index)
            })
            .collect::<BTreeSet<_>>();
        let mut active = BTreeSet::new();
        let mut unfolded_active = BTreeSet::new();
        let mut outline_order = (0..outlines.len()).collect::<Vec<_>>();
        let mut excerpt_order = excerpts
            .iter_mut()
            .filter(|excerpt| excerpt.expanded)
            .collect::<Vec<_>>();
        outline_order.sort_unstable_by_key(|&index| cmp::Reverse(outlines[index].ranked_range.end));
        excerpt_order.sort_unstable_by_key(|excerpt| cmp::Reverse(excerpt.ranked_range.start));
        let mut cursor = 0;
        for excerpt in excerpt_order {
            while let Some(&index) = outline_order.get(cursor)
                && outlines[index].ranked_range.end >= excerpt.ranked_range.start
            {
                active.insert(index);
                if unfolded.contains(&index) {
                    unfolded_active.insert(index);
                }
                cursor += 1;
            }
            let limit = outlines
                .partition_point(|outline| outline.ranked_range.start <= excerpt.ranked_range.end);
            excerpt.visible.extend(
                unfolded_active
                    .range(..limit)
                    .map(|&index| outlines[index].outline),
            );
            for &index in &excerpt.children_to_cache {
                let next = active
                    .range(index + 1..limit)
                    .next()
                    .map(|&next| &outlines[next]);
                outlines[index].cache_children(next, children);
            }
        }
    }

    fn collect_overlapping_outlines<'a>(
        outlines: &[IndexedOutline<'a>],
        excerpts: &mut [ExcerptOutlines<'a>],
        children: &mut OutlineChildren,
    ) {
        let leaf_count = outlines.len().next_power_of_two();
        let mut starts = vec![BTreeSet::new(); 2 * leaf_count];
        let mut depths = vec![BTreeMap::new(); 2 * leaf_count];
        let mut outline_order = (0..outlines.len()).collect::<Vec<_>>();
        let mut excerpt_order = excerpts
            .iter_mut()
            .filter(|excerpt| excerpt.expanded)
            .collect::<Vec<_>>();
        outline_order.sort_unstable_by_key(|&index| cmp::Reverse(outlines[index].ranked_range.end));
        excerpt_order.sort_unstable_by_key(|excerpt| cmp::Reverse(excerpt.ranked_range.start));
        let mut cursor = 0;
        for excerpt in excerpt_order {
            while let Some(&index) = outline_order.get(cursor)
                && outlines[index].ranked_range.end >= excerpt.ranked_range.start
            {
                let outline = &outlines[index];
                let mut node = leaf_count + index;
                while node > 0 {
                    starts[node].insert(outline.ranked_range.start);
                    Self::insert_outline_prefix_minimum(
                        &mut depths[node],
                        outline.ranked_range.start,
                        outline.outline.depth,
                    );
                    node /= 2;
                }
                cursor += 1;
            }
            for &index in &excerpt.children_to_cache {
                let next = Self::next_outline(index + 1, leaf_count, |node| {
                    starts[node]
                        .first()
                        .is_some_and(|&start| start <= excerpt.ranked_range.end)
                });
                outlines[index].cache_children(next.map(|next| &outlines[next]), children);
            }
            let mut collapsed_ancestor: Option<&IndexedOutline<'_>> = None;
            let mut first = 0;
            while let Some(index) = Self::next_outline(first, leaf_count, |node| {
                let Some(&first_start) = starts[node]
                    .first()
                    .filter(|&&start| start <= excerpt.ranked_range.end)
                else {
                    return false;
                };
                let Some(ancestor) = collapsed_ancestor else {
                    return true;
                };
                first_start < ancestor.ranked_range.start
                    || starts[node]
                        .range(..=excerpt.ranked_range.end)
                        .next_back()
                        .is_some_and(|&start| start >= ancestor.ranked_range.end)
                    || depths[node]
                        .range(..=excerpt.ranked_range.end)
                        .next_back()
                        .is_some_and(|(_, &depth)| depth <= ancestor.outline.depth)
            }) {
                let outline = &outlines[index];
                excerpt.visible.push(outline.outline);
                collapsed_ancestor = outline.collapsed.then_some(outline);
                first = index + 1;
            }
        }
    }

    fn next_outline(
        first: usize,
        leaf_count: usize,
        has_match: impl Fn(usize) -> bool,
    ) -> Option<usize> {
        if first >= leaf_count {
            return None;
        }
        let mut node = leaf_count + first;
        loop {
            while node.is_multiple_of(2) {
                node /= 2;
            }
            if has_match(node) {
                while node < leaf_count {
                    node *= 2;
                    if !has_match(node) {
                        node += 1;
                    }
                }
                return Some(node - leaf_count);
            }
            node += 1;
            if node.is_power_of_two() {
                return None;
            }
        }
    }

    fn insert_outline_prefix_minimum<T: Copy + Ord>(
        minimums: &mut BTreeMap<usize, T>,
        position: usize,
        value: T,
    ) {
        let previous = minimums.range(..=position).next_back();
        if previous.is_some_and(|(_, &current)| current <= value) {
            return;
        }
        while let Some((&next_position, &next_value)) = minimums.range(position..).next() {
            if next_value < value {
                break;
            }
            minimums.remove(&next_position);
        }
        minimums.insert(position, value);
    }
}

#[derive(Debug)]
enum OutlineState {
    Outlines(Vec<Outline>),
    Invalidated(Vec<Outline>),
    NotFetched,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FoldedDirsEntry {
    worktree_id: WorktreeId,
    first_buffer_id: BufferId,
    entries: Vec<FsEntryPath>,
}

// TODO: collapse the inner enums into panel entry
#[derive(Clone, Debug)]
enum PanelEntry {
    Fs(FsEntry),
    FoldedDirs(FoldedDirsEntry),
    Outline(OutlineEntry),
    Search(SearchEntry),
}

#[derive(Clone, Debug)]
struct SearchEntry {
    match_range: Range<editor::Anchor>,
    kind: SearchKind,
    render_data: Arc<OnceLock<SearchData>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum SearchKind {
    Project,
    Buffer,
}

#[derive(Clone, Debug)]
struct SearchData {
    context_range: Range<editor::Anchor>,
    context_text: String,
    truncated_left: bool,
    truncated_right: bool,
    search_match_indices: Vec<Range<usize>>,
    highlights_data: HighlightStyleData,
}

struct SearchPrecomputed {
    multi_buffer_snapshot: MultiBufferSnapshot,
    matches_by_buffer: HashMap<BufferId, Vec<(Range<editor::Anchor>, Arc<OnceLock<SearchData>>)>>,
    folded_buffers: HashSet<BufferId>,
}

impl PartialEq for PanelEntry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Fs(a), Self::Fs(b)) => a == b,
            (Self::FoldedDirs(left), Self::FoldedDirs(right)) => left == right,
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

const SEARCH_MATCH_CONTEXT_SIZE: u32 = 40;
const TRUNCATED_CONTEXT_MARK: &str = "…";

impl SearchData {
    fn new(
        match_range: &Range<editor::Anchor>,
        multi_buffer_snapshot: &MultiBufferSnapshot,
    ) -> Self {
        let match_point_range = match_range.to_point(multi_buffer_snapshot);
        let context_left_border = multi_buffer_snapshot.clip_point(
            language::Point::new(
                match_point_range.start.row,
                match_point_range
                    .start
                    .column
                    .saturating_sub(SEARCH_MATCH_CONTEXT_SIZE),
            ),
            Bias::Left,
        );
        let context_right_border = multi_buffer_snapshot.clip_point(
            language::Point::new(
                match_point_range.end.row,
                match_point_range.end.column + SEARCH_MATCH_CONTEXT_SIZE,
            ),
            Bias::Right,
        );

        let context_anchor_range =
            (context_left_border..context_right_border).to_anchors(multi_buffer_snapshot);
        let context_offset_range = context_anchor_range.to_offset(multi_buffer_snapshot);
        let match_offset_range = match_range.to_offset(multi_buffer_snapshot);

        let mut search_match_indices = vec![
            match_offset_range.start - context_offset_range.start
                ..match_offset_range.end - context_offset_range.start,
        ];

        let entire_context_text = multi_buffer_snapshot
            .text_for_range(context_offset_range.clone())
            .collect::<String>();
        let left_whitespaces_offset = entire_context_text
            .chars()
            .take_while(|c| c.is_whitespace())
            .map(|c| c.len_utf8())
            .sum::<usize>();

        let mut extended_context_left_border = context_left_border;
        extended_context_left_border.column = extended_context_left_border.column.saturating_sub(1);
        let extended_context_left_border =
            multi_buffer_snapshot.clip_point(extended_context_left_border, Bias::Left);
        let mut extended_context_right_border = context_right_border;
        extended_context_right_border.column += 1;
        let extended_context_right_border =
            multi_buffer_snapshot.clip_point(extended_context_right_border, Bias::Right);

        let truncated_left = left_whitespaces_offset == 0
            && extended_context_left_border < context_left_border
            && multi_buffer_snapshot
                .chars_at(extended_context_left_border)
                .next()
                .is_some_and(|c| !c.is_whitespace());
        let truncated_right = entire_context_text
            .chars()
            .last()
            .is_none_or(|c| !c.is_whitespace())
            && extended_context_right_border > context_right_border
            && multi_buffer_snapshot
                .chars_at(extended_context_right_border)
                .next()
                .is_some_and(|c| !c.is_whitespace());
        search_match_indices.iter_mut().for_each(|range| {
            range.start = range.start.saturating_sub(left_whitespaces_offset);
            range.end = range.end.saturating_sub(left_whitespaces_offset);
        });

        let trimmed_row_offset_range =
            context_offset_range.start + left_whitespaces_offset..context_offset_range.end;
        let trimmed_text = entire_context_text[left_whitespaces_offset..].to_owned();
        Self {
            highlights_data: Arc::default(),
            search_match_indices,
            context_range: trimmed_row_offset_range.to_anchors(multi_buffer_snapshot),
            context_text: trimmed_text,
            truncated_left,
            truncated_right,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum OutlineEntry {
    Excerpt(ExcerptRange<Anchor>),
    Outline(Outline),
}

impl OutlineEntry {
    fn buffer_id(&self) -> BufferId {
        match self {
            OutlineEntry::Excerpt(excerpt) => excerpt.context.start.buffer_id,
            OutlineEntry::Outline(outline) => outline.range.start.buffer_id,
        }
    }

    fn range(&self) -> Range<Anchor> {
        match self {
            OutlineEntry::Excerpt(excerpt) => excerpt.context.clone(),
            OutlineEntry::Outline(outline) => outline.range.clone(),
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct FsEntryPath {
    path: Arc<RelPath>,
    git_summary: GitSummary,
    is_ignored: bool,
}

impl PartialEq for FsEntryPath {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

#[derive(Debug, Clone, Eq)]
struct FsEntryFile {
    worktree_id: WorktreeId,
    entry: FsEntryPath,
    buffer_id: BufferId,
    excerpts: Arc<[ExcerptRange<Anchor>]>,
    is_deleted: bool,
}

impl PartialEq for FsEntryFile {
    fn eq(&self, other: &Self) -> bool {
        self.buffer_id == other.buffer_id
    }
}

impl Hash for FsEntryFile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buffer_id.hash(state);
    }
}

#[derive(Debug, Clone, Eq)]
struct FsEntryDirectory {
    worktree_id: WorktreeId,
    first_buffer_id: BufferId,
    entry: FsEntryPath,
}

impl PartialEq for FsEntryDirectory {
    fn eq(&self, other: &Self) -> bool {
        self.worktree_id == other.worktree_id
            && self.first_buffer_id == other.first_buffer_id
            && self.entry.path == other.entry.path
    }
}

impl Hash for FsEntryDirectory {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.worktree_id, self.first_buffer_id, &self.entry.path).hash(state);
    }
}

#[derive(Debug, Clone, Eq)]
struct FsEntryExternalFile {
    buffer_id: BufferId,
    excerpts: Arc<[ExcerptRange<Anchor>]>,
}

impl PartialEq for FsEntryExternalFile {
    fn eq(&self, other: &Self) -> bool {
        self.buffer_id == other.buffer_id
    }
}

impl Hash for FsEntryExternalFile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buffer_id.hash(state);
    }
}

#[derive(Clone, Debug, Eq)]
enum FsEntry {
    ExternalFile(FsEntryExternalFile),
    Directory(FsEntryDirectory),
    File(FsEntryFile),
}

impl PartialEq for FsEntry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Directory(left), Self::Directory(right)) => left == right,
            _ => self
                .buffer_id()
                .is_some_and(|id| Some(id) == other.buffer_id()),
        }
    }
}

impl FsEntry {
    fn buffer_id(&self) -> Option<BufferId> {
        match self {
            Self::File(file) => Some(file.buffer_id),
            Self::ExternalFile(file) => Some(file.buffer_id),
            Self::Directory(_) => None,
        }
    }
}

fn directory_element_id(
    worktree_id: WorktreeId,
    first_buffer_id: BufferId,
    path: &RelPath,
) -> ElementId {
    ElementId::from((
        SharedString::from(format!("directory-{first_buffer_id}-{path}")),
        worktree_id.to_proto() as usize,
    ))
}

fn is_parent_row(entry: &PanelEntry, worktree_id: WorktreeId, parent_path: &RelPath) -> bool {
    match entry {
        PanelEntry::Fs(FsEntry::Directory(directory)) => {
            directory.worktree_id == worktree_id && directory.entry.path.as_ref() == parent_path
        }
        PanelEntry::FoldedDirs(folded_dirs) => {
            folded_dirs.worktree_id == worktree_id
                && folded_dirs
                    .entries
                    .last()
                    .is_some_and(|dir| dir.path.as_ref() == parent_path)
        }
        _ => false,
    }
}

fn is_deleted_file(file: &File, status: Option<FileStatus>, has_base_text: bool) -> bool {
    match file.disk_state {
        DiskState::Deleted => true,
        DiskState::Historic { was_deleted } => was_deleted,
        DiskState::Present { .. } => false,
        DiskState::New => status.is_some_and(|status| status.is_deleted()) || has_base_text,
    }
}

struct BufferExcerpts {
    is_new: bool,
    is_folded: bool,
    excerpts: Vec<ExcerptRange<Anchor>>,
    file: Option<(worktree::Snapshot, FsEntryPath)>,
    is_deleted: bool,
}

struct ActiveItem {
    item_handle: Box<dyn WeakItemHandle>,
    active_editor: WeakEntity<Editor>,
    _buffer_search_subscription: Subscription,
    _editor_subscription: Subscription,
}

#[derive(Debug)]
pub enum Event {
    Focus,
}

#[derive(Serialize, Deserialize)]
struct SerializedOutlinePanel {
    active: Option<bool>,
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<OutlinePanel>(window, cx);
        });
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            if !workspace.toggle_panel_focus::<OutlinePanel>(window, cx) {
                workspace.close_panel::<OutlinePanel>(window, cx);
            }
        });
        workspace.register_action(|workspace, _: &ToggleSymbols, window, cx| {
            if let Some(outline_panel) = workspace.panel::<OutlinePanel>(cx) {
                // Defer to avoid updating the panel while the workspace is still being updated.
                window.defer(cx, move |window, cx| {
                    outline_panel.update(cx, |outline_panel, cx| {
                        outline_panel.toggle_symbols(&ToggleSymbols, window, cx);
                    });
                });
            }
        });
    })
    .detach();
}

impl OutlinePanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<Entity<Self>> {
        let serialized_panel = match workspace
            .read_with(&cx, |workspace, _| {
                OutlinePanel::serialization_key(workspace)
            })
            .ok()
            .flatten()
        {
            Some(serialization_key) => {
                let kvp = cx.update(|_, cx| KeyValueStore::global(cx))?;
                cx.background_spawn(async move { kvp.read_kvp(&serialization_key) })
                    .await
                    .context("loading outline panel")
                    .log_err()
                    .flatten()
                    .map(|panel| serde_json::from_str::<SerializedOutlinePanel>(&panel))
                    .transpose()
                    .log_err()
                    .flatten()
            }
            None => None,
        };

        workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = Self::new(workspace, serialized_panel.as_ref(), window, cx);
            panel.update(cx, |_, cx| cx.notify());
            panel
        })
    }

    fn new(
        workspace: &mut Workspace,
        serialized: Option<&SerializedOutlinePanel>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let project = workspace.project().clone();
        let workspace_handle = cx.entity().downgrade();

        cx.new(|cx| {
            let filter_editor = cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text("Search buffer symbols…", window, cx);
                editor
            });
            let filter_update_subscription = cx.subscribe_in(
                &filter_editor,
                window,
                |outline_panel: &mut Self, _, event, window, cx| {
                    if let editor::EditorEvent::BufferEdited = event {
                        outline_panel.update_cached_entries(Some(UPDATE_DEBOUNCE), window, cx);
                    }
                },
            );

            let focus_handle = cx.focus_handle();
            let focus_subscription = cx.on_focus(&focus_handle, window, Self::focus_in);
            let workspace_subscription = cx.subscribe_in(
                &workspace
                    .weak_handle()
                    .upgrade()
                    .expect("have a &mut Workspace"),
                window,
                move |outline_panel, workspace, event, window, cx| {
                    if let workspace::Event::ActiveItemChanged = event {
                        if let Some((new_active_item, new_active_editor)) =
                            workspace_active_editor(workspace.read(cx), cx)
                        {
                            if outline_panel.should_replace_active_item(new_active_item.as_ref()) {
                                outline_panel.replace_active_editor(
                                    new_active_item,
                                    new_active_editor,
                                    window,
                                    cx,
                                );
                            }
                        } else {
                            outline_panel.clear_previous(window, cx);
                            cx.notify();
                        }
                    }
                },
            );

            let icons_subscription = cx.observe_global::<FileIcons>(|_, cx| {
                cx.notify();
            });

            let theme_subscription =
                cx.observe_global_in::<GlobalTheme>(window, |panel, window, cx| {
                    let buffers = panel.buffers.keys().copied().collect::<Vec<_>>();
                    panel.invalidate_outlines(&buffers);
                    panel.update_contents(Some(UPDATE_DEBOUNCE), window, cx);
                });

            let mut outline_panel_settings = *OutlinePanelSettings::get_global(cx);
            let mut current_theme = ThemeSettings::get_global(cx).clone();
            let mut document_symbols_by_buffer = HashMap::default();
            let settings_subscription =
                cx.observe_global_in::<SettingsStore>(window, move |outline_panel, window, cx| {
                    let new_settings = OutlinePanelSettings::get_global(cx);
                    let new_theme = ThemeSettings::get_global(cx);
                    let auto_fold_dirs_changed =
                        outline_panel_settings.auto_fold_dirs != new_settings.auto_fold_dirs;
                    let mut outlines_invalidated = false;
                    if &current_theme != new_theme {
                        outline_panel_settings = *new_settings;
                        current_theme = new_theme.clone();
                        outline_panel.outline_fetch_tasks.clear();
                        for buffer in outline_panel.buffers.values_mut() {
                            buffer.invalidate_outlines();
                        }
                        outlines_invalidated = true;
                        if matches!(outline_panel.selected_entry(), Some(PanelEntry::Outline(_))) {
                            outline_panel.selected_entry.invalidate();
                        }
                        outline_panel.update_contents(Some(UPDATE_DEBOUNCE), window, cx);
                    } else if &outline_panel_settings != new_settings {
                        let old_expansion_depth = outline_panel_settings.expand_outlines_with_depth;
                        let old_hide_symbols = outline_panel_settings.multi_buffer_hide_symbols;
                        outline_panel_settings = *new_settings;

                        let mut update_cached_entries = false;
                        if old_hide_symbols != outline_panel_settings.multi_buffer_hide_symbols {
                            outline_panel.hide_symbols_override = None;
                            outline_panel.update_non_fs_items(window, cx);
                            outline_panel.remap_selection_for_hidden_symbols(window, cx);
                            update_cached_entries = true;
                        }
                        if old_expansion_depth != outline_panel_settings.expand_outlines_with_depth
                        {
                            let old_collapsed_entries = outline_panel.collapsed_entries.clone();
                            outline_panel
                                .collapsed_entries
                                .retain(|entry| !matches!(entry, CollapsedEntry::Outline(..)));

                            let new_depth = outline_panel_settings.expand_outlines_with_depth;
                            outline_panel.pending_default_expansion_depth = Some(new_depth);
                            outline_panel.default_depth_applied_buffers = outline_panel
                                .buffers
                                .iter()
                                .filter(|(_, buffer)| {
                                    matches!(buffer.outlines, OutlineState::Outlines(_))
                                })
                                .map(|(buffer_id, _)| *buffer_id)
                                .collect();

                            for (buffer_id, buffer) in &outline_panel.buffers {
                                if let OutlineState::Outlines(outlines) = &buffer.outlines {
                                    for outline in outlines {
                                        if outline_panel
                                            .outline_children_cache
                                            .get(buffer_id)
                                            .and_then(|children_map| {
                                                let key = (outline.range.clone(), outline.depth);
                                                children_map.get(&key)
                                            })
                                            .copied()
                                            .unwrap_or(false)
                                            && (new_depth == 0 || outline.depth >= new_depth)
                                        {
                                            outline_panel.collapsed_entries.insert(
                                                CollapsedEntry::Outline(outline.range.clone()),
                                            );
                                        }
                                    }
                                }
                            }

                            if old_collapsed_entries != outline_panel.collapsed_entries {
                                update_cached_entries = true;
                            }
                        }
                        if update_cached_entries {
                            outline_panel.update_cached_entries(Some(UPDATE_DEBOUNCE), window, cx);
                        } else {
                            cx.notify();
                        }
                    }

                    if auto_fold_dirs_changed && let Some(editor) = outline_panel.active_editor() {
                        outline_panel.update_fs_entries(editor, Some(UPDATE_DEBOUNCE), window, cx);
                    }

                    if !outlines_invalidated {
                        let new_document_symbols = outline_panel
                            .buffers
                            .keys()
                            .filter_map(|buffer_id| {
                                let buffer = outline_panel
                                    .project
                                    .read(cx)
                                    .buffer_for_id(*buffer_id, cx)?;
                                let buffer = buffer.read(cx);
                                let doc_symbols =
                                    LanguageSettings::for_buffer(buffer, cx).document_symbols;
                                Some((*buffer_id, doc_symbols))
                            })
                            .collect();
                        if new_document_symbols != document_symbols_by_buffer {
                            document_symbols_by_buffer = new_document_symbols;
                            outline_panel.outline_fetch_tasks.clear();
                            for buffer in outline_panel.buffers.values_mut() {
                                buffer.invalidate_outlines();
                            }
                            if matches!(
                                outline_panel.selected_entry(),
                                Some(PanelEntry::Outline(_))
                            ) {
                                outline_panel.selected_entry.invalidate();
                            }
                            outline_panel.update_contents(Some(UPDATE_DEBOUNCE), window, cx);
                        }
                    }
                });

            let scroll_handle = UniformListScrollHandle::new();

            let mut outline_panel = Self {
                mode: ItemsDisplayMode::Outline,
                active: serialized.and_then(|s| s.active).unwrap_or(false),
                pinned: false,
                workspace: workspace_handle,
                project,
                fs: workspace.app_state().fs.clone(),
                max_width_item_index: None,
                scroll_handle,
                rendered_entries_len: 0,
                focus_handle,
                filter_editor,
                fs_entries: Vec::new(),
                fs_children_count: HashMap::default(),
                collapsed_entries: HashSet::default(),
                unfolded_dirs: HashMap::default(),
                selected_entry: SelectedEntry::None,
                context_menu: None,
                active_item: None,
                pending_serialization: Task::ready(None),
                new_entries_for_fs_update: HashSet::default(),
                preserve_selection_on_buffer_fold_toggles: HashSet::default(),
                pending_default_expansion_depth: None,
                default_depth_applied_buffers: HashSet::default(),
                fs_entries_update_task: Task::ready(()),
                fs_entries_update_pending: false,
                cached_entries_update_task: Task::ready(()),
                cached_entries_update_pending: None,
                reveal_selection_task: Task::ready(Ok(())),
                lsp_outline_refresh_task: Task::ready(()),
                outline_fetch_tasks: HashMap::default(),
                buffers: HashMap::default(),
                cached_entries: Vec::new(),
                _subscriptions: vec![
                    theme_subscription,
                    settings_subscription,
                    icons_subscription,
                    focus_subscription,
                    workspace_subscription,
                    filter_update_subscription,
                ],
                outline_children_cache: HashMap::default(),
                hide_symbols_override: None,
            };
            if let Some((item, editor)) = workspace_active_editor(workspace, cx) {
                outline_panel.replace_active_editor(item, editor, window, cx);
            }
            outline_panel
        })
    }

    fn serialization_key(workspace: &Workspace) -> Option<String> {
        workspace
            .database_id()
            .map(|id| i64::from(id).to_string())
            .or(workspace.session_id())
            .map(|id| format!("{}-{:?}", OUTLINE_PANEL_KEY, id))
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        let Some(serialization_key) = self
            .workspace
            .read_with(cx, |workspace, _| {
                OutlinePanel::serialization_key(workspace)
            })
            .ok()
            .flatten()
        else {
            return;
        };
        let active = self.active.then_some(true);
        let kvp = KeyValueStore::global(cx);
        self.pending_serialization = cx.background_spawn(
            async move {
                kvp.write_kvp(
                    serialization_key,
                    serde_json::to_string(&SerializedOutlinePanel { active })?,
                )
                .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    fn dispatch_context(&self, window: &mut Window, cx: &mut Context<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("OutlinePanel");
        dispatch_context.add("menu");
        let identifier = if self.filter_editor.focus_handle(cx).is_focused(window) {
            "editing"
        } else {
            "not_editing"
        };
        dispatch_context.add(identifier);
        dispatch_context
    }

    fn unfold_directory(
        &mut self,
        _: &UnfoldDirectory,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(PanelEntry::FoldedDirs(FoldedDirsEntry {
            worktree_id,
            entries,
            ..
        })) = self.selected_entry().cloned()
        {
            self.unfolded_dirs
                .entry(worktree_id)
                .or_default()
                .extend(entries.iter().map(|entry| entry.path.clone()));
            self.update_cached_entries(None, window, cx);
        }
    }

    fn fold_directory(&mut self, _: &FoldDirectory, window: &mut Window, cx: &mut Context<Self>) {
        let (worktree_id, entry) = match self.selected_entry().cloned() {
            Some(PanelEntry::Fs(FsEntry::Directory(directory))) => {
                (directory.worktree_id, Some(directory.entry))
            }
            Some(PanelEntry::FoldedDirs(folded_dirs)) => {
                (folded_dirs.worktree_id, folded_dirs.entries.last().cloned())
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

        unfolded_dirs.remove(&entry.path);
        self.update_cached_entries(None, window, cx);
    }

    fn open_selected_entry(
        &mut self,
        _: &OpenSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.filter_editor.focus_handle(cx).is_focused(window) {
            cx.propagate()
        } else if let Some(selected_entry) = self.selected_entry().cloned() {
            self.scroll_editor_to_entry(&selected_entry, true, true, window, cx);
        }
    }

    fn cancel(&mut self, _: &Cancel, window: &mut Window, cx: &mut Context<Self>) {
        if self.filter_editor.focus_handle(cx).is_focused(window) {
            self.focus_handle.focus(window, cx);
        } else {
            self.filter_editor.focus_handle(cx).focus(window, cx);
        }

        if self.context_menu.is_some() {
            self.context_menu.take();
            cx.notify();
        }
    }

    fn open_excerpts(
        &mut self,
        action: &editor::actions::OpenExcerpts,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.filter_editor.focus_handle(cx).is_focused(window) {
            cx.propagate()
        } else if let Some((active_editor, selected_entry)) =
            self.active_editor().zip(self.selected_entry().cloned())
        {
            self.scroll_editor_to_entry(&selected_entry, true, true, window, cx);
            active_editor.update(cx, |editor, cx| editor.open_excerpts(action, window, cx));
        }
    }

    fn open_excerpts_split(
        &mut self,
        action: &editor::actions::OpenExcerptsSplit,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.filter_editor.focus_handle(cx).is_focused(window) {
            cx.propagate()
        } else if let Some((active_editor, selected_entry)) =
            self.active_editor().zip(self.selected_entry().cloned())
        {
            self.scroll_editor_to_entry(&selected_entry, true, true, window, cx);
            active_editor.update(cx, |editor, cx| {
                editor.open_excerpts_in_split(action, window, cx)
            });
        }
    }

    fn scroll_editor_to_entry(
        &mut self,
        entry: &PanelEntry,
        prefer_selection_change: bool,
        prefer_focus_change: bool,
        window: &mut Window,
        cx: &mut Context<OutlinePanel>,
    ) {
        let Some(active_editor) = self.active_editor() else {
            return;
        };
        let active_multi_buffer = active_editor.read(cx).buffer().clone();
        let multi_buffer_snapshot = active_multi_buffer.read(cx).snapshot(cx);
        let mut change_selection = prefer_selection_change;
        let mut change_focus = prefer_focus_change;
        let mut scroll_to_buffer = None;
        let scroll_target = match entry {
            PanelEntry::FoldedDirs(..) | PanelEntry::Fs(FsEntry::Directory(..)) => {
                change_focus = false;
                None
            }
            PanelEntry::Fs(FsEntry::ExternalFile(file)) => {
                change_selection = false;
                scroll_to_buffer = Some(file.buffer_id);
                multi_buffer_snapshot.excerpts().find_map(|excerpt_range| {
                    if excerpt_range.context.start.buffer_id == file.buffer_id {
                        multi_buffer_snapshot.anchor_in_excerpt(excerpt_range.context.start)
                    } else {
                        None
                    }
                })
            }

            PanelEntry::Fs(FsEntry::File(file)) => {
                change_selection = false;
                scroll_to_buffer = Some(file.buffer_id);
                multi_buffer_snapshot
                    .excerpts_for_buffer(file.buffer_id)
                    .next()
                    .and_then(|excerpt_range| {
                        multi_buffer_snapshot.anchor_in_excerpt(excerpt_range.context.start)
                    })
            }
            PanelEntry::Outline(OutlineEntry::Outline(outline)) => multi_buffer_snapshot
                .anchor_in_excerpt(outline.range.start)
                .or_else(|| multi_buffer_snapshot.anchor_in_excerpt(outline.range.end)),
            PanelEntry::Outline(OutlineEntry::Excerpt(excerpt)) => {
                change_selection = false;
                change_focus = false;
                multi_buffer_snapshot.anchor_in_excerpt(excerpt.context.start)
            }
            PanelEntry::Search(search_entry) => Some(search_entry.match_range.start),
        };

        if let Some(anchor) = scroll_target {
            let activate = self
                .workspace
                .update(cx, |workspace, cx| match self.active_item() {
                    Some(active_item) => workspace.activate_item(
                        active_item.as_ref(),
                        true,
                        change_focus,
                        window,
                        cx,
                    ),
                    None => workspace.activate_item(&active_editor, true, change_focus, window, cx),
                });

            if activate.is_ok() {
                self.select_entry(entry.clone(), true, window, cx);
                if change_selection {
                    active_editor.update(cx, |editor, cx| {
                        editor.change_selections(
                            SelectionEffects::scroll(Autoscroll::center()),
                            window,
                            cx,
                            |s| s.select_ranges(Some(anchor..anchor)),
                        );
                    });
                } else {
                    let mut offset = Point::default();
                    if let Some(buffer_id) = scroll_to_buffer
                        && multi_buffer_snapshot.as_singleton().is_none()
                        && !active_editor.read(cx).is_buffer_folded(buffer_id, cx)
                    {
                        offset.y = -(active_editor.read(cx).file_header_size() as f64);
                    }

                    active_editor.update(cx, |editor, cx| {
                        editor.set_scroll_anchor(ScrollAnchor { offset, anchor }, window, cx);
                    });
                }

                if change_focus {
                    active_editor.focus_handle(cx).focus(window, cx);
                } else {
                    self.focus_handle.focus(window, cx);
                }
            }
        }
    }

    fn scroll_up(&mut self, _: &ScrollUp, window: &mut Window, cx: &mut Context<Self>) {
        for _ in 0..self.rendered_entries_len / 2 {
            window.dispatch_action(SelectPrevious.boxed_clone(), cx);
        }
    }

    fn scroll_down(&mut self, _: &ScrollDown, window: &mut Window, cx: &mut Context<Self>) {
        for _ in 0..self.rendered_entries_len / 2 {
            window.dispatch_action(SelectNext.boxed_clone(), cx);
        }
    }

    fn scroll_cursor_center(
        &mut self,
        _: &ScrollCursorCenter,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selected_entry) = self.selected_entry() {
            let index = self
                .cached_entries
                .iter()
                .position(|cached_entry| &cached_entry.entry == selected_entry);
            if let Some(index) = index {
                self.scroll_handle
                    .scroll_to_item_strict(index, ScrollStrategy::Center);
                cx.notify();
            }
        }
    }

    fn scroll_cursor_top(&mut self, _: &ScrollCursorTop, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(selected_entry) = self.selected_entry() {
            let index = self
                .cached_entries
                .iter()
                .position(|cached_entry| &cached_entry.entry == selected_entry);
            if let Some(index) = index {
                self.scroll_handle
                    .scroll_to_item_strict(index, ScrollStrategy::Top);
                cx.notify();
            }
        }
    }

    fn scroll_cursor_bottom(
        &mut self,
        _: &ScrollCursorBottom,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selected_entry) = self.selected_entry() {
            let index = self
                .cached_entries
                .iter()
                .position(|cached_entry| &cached_entry.entry == selected_entry);
            if let Some(index) = index {
                self.scroll_handle
                    .scroll_to_item_strict(index, ScrollStrategy::Bottom);
                cx.notify();
            }
        }
    }

    fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(entry_to_select) = self.selected_entry().and_then(|selected_entry| {
            self.cached_entries
                .iter()
                .map(|cached_entry| &cached_entry.entry)
                .skip_while(|entry| entry != &selected_entry)
                .nth(1)
                .cloned()
        }) {
            self.select_entry(entry_to_select, true, window, cx);
        } else {
            self.select_first(&SelectFirst {}, window, cx)
        }
        if let Some(selected_entry) = self.selected_entry().cloned() {
            self.scroll_editor_to_entry(&selected_entry, true, false, window, cx);
        }
    }

    fn select_previous(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(entry_to_select) = self.selected_entry().and_then(|selected_entry| {
            self.cached_entries
                .iter()
                .rev()
                .map(|cached_entry| &cached_entry.entry)
                .skip_while(|entry| entry != &selected_entry)
                .nth(1)
                .cloned()
        }) {
            self.select_entry(entry_to_select, true, window, cx);
        } else {
            self.select_last(&SelectLast, window, cx)
        }
        if let Some(selected_entry) = self.selected_entry().cloned() {
            self.scroll_editor_to_entry(&selected_entry, true, false, window, cx);
        }
    }

    fn select_parent(&mut self, _: &SelectParent, window: &mut Window, cx: &mut Context<Self>) {
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
                    FsEntry::File(FsEntryFile {
                        worktree_id, entry, ..
                    })
                    | FsEntry::Directory(FsEntryDirectory {
                        worktree_id, entry, ..
                    }) => entry.path.parent().and_then(|parent_path| {
                        previous_entries
                            .find(|entry| is_parent_row(entry, *worktree_id, parent_path))
                    }),
                },
                PanelEntry::FoldedDirs(folded_dirs) => folded_dirs
                    .entries
                    .first()
                    .and_then(|entry| entry.path.parent())
                    .and_then(|parent_path| {
                        previous_entries.find(|entry| {
                            is_parent_row(entry, folded_dirs.worktree_id, parent_path)
                        })
                    }),
                PanelEntry::Outline(OutlineEntry::Excerpt(excerpt)) => {
                    previous_entries.find(|entry| match entry {
                        PanelEntry::Fs(FsEntry::File(file)) => {
                            file.buffer_id == excerpt.context.start.buffer_id
                                && file.excerpts.contains(&excerpt)
                        }
                        PanelEntry::Fs(FsEntry::ExternalFile(external_file)) => {
                            external_file.buffer_id == excerpt.context.start.buffer_id
                                && external_file.excerpts.contains(&excerpt)
                        }
                        _ => false,
                    })
                }
                PanelEntry::Outline(OutlineEntry::Outline(outline)) => {
                    previous_entries.find(|entry| {
                        if let PanelEntry::Outline(OutlineEntry::Excerpt(excerpt)) = entry {
                            if outline.range.start.buffer_id != excerpt.context.start.buffer_id {
                                return false;
                            }
                            let Some(buffer_snapshot) =
                                self.buffer_snapshot_for_id(outline.range.start.buffer_id, cx)
                            else {
                                return false;
                            };
                            excerpt.contains(&outline.range.start, &buffer_snapshot)
                                || excerpt.contains(&outline.range.end, &buffer_snapshot)
                        } else {
                            false
                        }
                    })
                }
                PanelEntry::Search(_) => {
                    previous_entries.find(|entry| !matches!(entry, PanelEntry::Search(_)))
                }
            }
        }) {
            self.select_entry(entry_to_select.clone(), true, window, cx);
        } else {
            self.select_first(&SelectFirst {}, window, cx);
        }
    }

    fn select_first(&mut self, _: &SelectFirst, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(first_entry) = self.cached_entries.first() {
            self.select_entry(first_entry.entry.clone(), true, window, cx);
        }
    }

    fn select_last(&mut self, _: &SelectLast, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(new_selection) = self
            .cached_entries
            .iter()
            .rev()
            .map(|cached_entry| &cached_entry.entry)
            .next()
        {
            self.select_entry(new_selection.clone(), true, window, cx);
        }
    }

    fn autoscroll(&mut self, cx: &mut Context<Self>) {
        if let Some(selected_entry) = self.selected_entry() {
            let index = self
                .cached_entries
                .iter()
                .position(|cached_entry| &cached_entry.entry == selected_entry);
            if let Some(index) = index {
                self.scroll_handle
                    .scroll_to_item(index, ScrollStrategy::Center);
                cx.notify();
            }
        }
    }

    fn focus_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.focus_handle.contains_focused(window, cx) {
            cx.emit(Event::Focus);
        }
    }

    fn deploy_context_menu(
        &mut self,
        position: Point<Pixels>,
        entry: PanelEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_entry(entry.clone(), true, window, cx);
        let is_root = match &entry {
            PanelEntry::Fs(FsEntry::File(FsEntryFile { entry, .. }))
            | PanelEntry::Fs(FsEntry::Directory(FsEntryDirectory { entry, .. })) => {
                entry.path.is_empty()
            }
            PanelEntry::FoldedDirs(folded_dirs) => folded_dirs
                .entries
                .first()
                .is_some_and(|entry| entry.path.is_empty()),
            PanelEntry::Fs(FsEntry::ExternalFile(_)) => false,
            PanelEntry::Outline(_) | PanelEntry::Search(_) => {
                cx.notify();
                return;
            }
        };
        let auto_fold_dirs = OutlinePanelSettings::get_global(cx).auto_fold_dirs;
        let is_foldable = auto_fold_dirs && !is_root && self.is_foldable(&entry);
        let is_unfoldable = auto_fold_dirs && !is_root && self.is_unfoldable(&entry);

        let context_menu = ContextMenu::build(window, cx, |menu, _, _| {
            menu.context(self.focus_handle.clone())
                .action(
                    ui::utils::reveal_in_file_manager_label(false),
                    Box::new(RevealInFileManager),
                )
                .action("Open in Terminal", Box::new(OpenInTerminal))
                .when(is_unfoldable, |menu| {
                    menu.action("Unfold Directory", Box::new(UnfoldDirectory))
                })
                .when(is_foldable, |menu| {
                    menu.action("Fold Directory", Box::new(FoldDirectory))
                })
                .separator()
                .action("Copy Path", Box::new(zed_actions::workspace::CopyPath))
                .action(
                    "Copy Relative Path",
                    Box::new(zed_actions::workspace::CopyRelativePath),
                )
        });
        window.focus(&context_menu.focus_handle(cx), cx);
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
            PanelEntry::Fs(FsEntry::Directory(FsEntryDirectory {
                worktree_id,
                entry: directory_entry,
                ..
            })) => (*worktree_id, Some(directory_entry)),
            _ => return false,
        };
        let Some(directory_entry) = directory_entry else {
            return false;
        };

        if self
            .unfolded_dirs
            .get(&directory_worktree)
            .is_none_or(|unfolded_dirs| !unfolded_dirs.contains(&directory_entry.path))
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

    fn expand_selected_entry(
        &mut self,
        _: &ExpandSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_editor) = self.active_editor() else {
            return;
        };
        let Some(selected_entry) = self.selected_entry().cloned() else {
            return;
        };
        let mut buffers_to_unfold = HashSet::default();
        let entry_to_expand = match &selected_entry {
            PanelEntry::FoldedDirs(FoldedDirsEntry {
                entries: dir_entries,
                worktree_id,
                ..
            }) => dir_entries.last().map(|entry| {
                buffers_to_unfold.extend(self.buffers_inside_directory(*worktree_id, entry));
                CollapsedEntry::Dir(*worktree_id, entry.path.clone())
            }),
            PanelEntry::Fs(FsEntry::Directory(FsEntryDirectory {
                worktree_id, entry, ..
            })) => {
                buffers_to_unfold.extend(self.buffers_inside_directory(*worktree_id, entry));
                Some(CollapsedEntry::Dir(*worktree_id, entry.path.clone()))
            }
            PanelEntry::Fs(FsEntry::File(FsEntryFile {
                worktree_id,
                buffer_id,
                ..
            })) => {
                buffers_to_unfold.insert(*buffer_id);
                Some(CollapsedEntry::File(*worktree_id, *buffer_id))
            }
            PanelEntry::Fs(FsEntry::ExternalFile(external_file)) => {
                buffers_to_unfold.insert(external_file.buffer_id);
                Some(CollapsedEntry::ExternalFile(external_file.buffer_id))
            }
            PanelEntry::Outline(OutlineEntry::Excerpt(excerpt)) => {
                Some(CollapsedEntry::Excerpt(excerpt.clone()))
            }
            PanelEntry::Outline(OutlineEntry::Outline(outline)) => {
                Some(CollapsedEntry::Outline(outline.range.clone()))
            }
            PanelEntry::Search(_) => return,
        };
        let Some(collapsed_entry) = entry_to_expand else {
            return;
        };
        let expanded = self.collapsed_entries.remove(&collapsed_entry);
        if expanded {
            active_editor.update(cx, |editor, cx| {
                buffers_to_unfold.retain(|buffer_id| editor.is_buffer_folded(*buffer_id, cx));
            });
            self.select_entry(selected_entry, true, window, cx);
            if buffers_to_unfold.is_empty() {
                self.update_cached_entries(None, window, cx);
            } else {
                self.toggle_buffers_fold(buffers_to_unfold, false, window, cx)
                    .detach();
            }
        } else {
            self.select_next(&SelectNext, window, cx)
        }
    }

    fn collapse_selected_entry(
        &mut self,
        _: &CollapseSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_editor) = self.active_editor() else {
            return;
        };
        let Some(selected_entry) = self.selected_entry().cloned() else {
            return;
        };

        let mut buffers_to_fold = HashSet::default();
        let collapsed = match &selected_entry {
            PanelEntry::Fs(FsEntry::Directory(FsEntryDirectory {
                worktree_id, entry, ..
            })) => {
                if self
                    .collapsed_entries
                    .insert(CollapsedEntry::Dir(*worktree_id, entry.path.clone()))
                {
                    buffers_to_fold.extend(self.buffers_inside_directory(*worktree_id, entry));
                    true
                } else {
                    false
                }
            }
            PanelEntry::Fs(FsEntry::File(FsEntryFile {
                worktree_id,
                buffer_id,
                ..
            })) => {
                if self
                    .collapsed_entries
                    .insert(CollapsedEntry::File(*worktree_id, *buffer_id))
                {
                    buffers_to_fold.insert(*buffer_id);
                    true
                } else {
                    false
                }
            }
            PanelEntry::Fs(FsEntry::ExternalFile(external_file)) => {
                if self
                    .collapsed_entries
                    .insert(CollapsedEntry::ExternalFile(external_file.buffer_id))
                {
                    buffers_to_fold.insert(external_file.buffer_id);
                    true
                } else {
                    false
                }
            }
            PanelEntry::FoldedDirs(folded_dirs) => {
                let mut folded = false;
                if let Some(dir_entry) = folded_dirs.entries.last()
                    && self.collapsed_entries.insert(CollapsedEntry::Dir(
                        folded_dirs.worktree_id,
                        dir_entry.path.clone(),
                    ))
                {
                    folded = true;
                    buffers_to_fold
                        .extend(self.buffers_inside_directory(folded_dirs.worktree_id, dir_entry));
                }
                folded
            }
            PanelEntry::Outline(OutlineEntry::Excerpt(excerpt)) => self
                .collapsed_entries
                .insert(CollapsedEntry::Excerpt(excerpt.clone())),
            PanelEntry::Outline(OutlineEntry::Outline(outline)) => self
                .collapsed_entries
                .insert(CollapsedEntry::Outline(outline.range.clone())),
            PanelEntry::Search(_) => false,
        };

        if collapsed {
            active_editor.update(cx, |editor, cx| {
                buffers_to_fold.retain(|buffer_id| !editor.is_buffer_folded(*buffer_id, cx));
            });
            self.select_entry(selected_entry, true, window, cx);
            if buffers_to_fold.is_empty() {
                self.update_cached_entries(None, window, cx);
            } else {
                self.toggle_buffers_fold(buffers_to_fold, true, window, cx)
                    .detach();
            }
        } else {
            self.select_parent(&SelectParent, window, cx);
        }
    }

    pub fn expand_all_entries(
        &mut self,
        _: &ExpandAllEntries,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_editor) = self.active_editor() else {
            return;
        };

        let mut to_uncollapse: HashSet<CollapsedEntry> = HashSet::default();
        let mut buffers_to_unfold: HashSet<BufferId> = HashSet::default();

        for fs_entry in &self.fs_entries {
            match fs_entry {
                FsEntry::File(FsEntryFile {
                    worktree_id,
                    buffer_id,
                    ..
                }) => {
                    to_uncollapse.insert(CollapsedEntry::File(*worktree_id, *buffer_id));
                    buffers_to_unfold.insert(*buffer_id);
                }
                FsEntry::ExternalFile(FsEntryExternalFile { buffer_id, .. }) => {
                    to_uncollapse.insert(CollapsedEntry::ExternalFile(*buffer_id));
                    buffers_to_unfold.insert(*buffer_id);
                }
                FsEntry::Directory(FsEntryDirectory {
                    worktree_id, entry, ..
                }) => {
                    to_uncollapse.insert(CollapsedEntry::Dir(*worktree_id, entry.path.clone()));
                }
            }
        }

        for (_buffer_id, buffer) in &self.buffers {
            match &buffer.outlines {
                OutlineState::Outlines(outlines) => {
                    for outline in outlines {
                        to_uncollapse.insert(CollapsedEntry::Outline(outline.range.clone()));
                    }
                }
                OutlineState::Invalidated(outlines) => {
                    for outline in outlines {
                        to_uncollapse.insert(CollapsedEntry::Outline(outline.range.clone()));
                    }
                }
                OutlineState::NotFetched => {}
            }
            to_uncollapse.extend(
                buffer
                    .excerpts
                    .iter()
                    .map(|excerpt| CollapsedEntry::Excerpt(excerpt.clone())),
            );
        }

        for cached in &self.cached_entries {
            if let PanelEntry::FoldedDirs(FoldedDirsEntry {
                worktree_id,
                entries,
                ..
            }) = &cached.entry
            {
                if let Some(last) = entries.last() {
                    to_uncollapse.insert(CollapsedEntry::Dir(*worktree_id, last.path.clone()));
                }
            }
        }

        self.collapsed_entries
            .retain(|entry| !to_uncollapse.contains(entry));

        active_editor.update(cx, |editor, cx| {
            buffers_to_unfold.retain(|buffer_id| editor.is_buffer_folded(*buffer_id, cx));
        });

        if buffers_to_unfold.is_empty() {
            self.update_cached_entries(None, window, cx);
        } else {
            self.toggle_buffers_fold(buffers_to_unfold, false, window, cx)
                .detach();
        }
    }

    pub fn collapse_all_entries(
        &mut self,
        _: &CollapseAllEntries,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_editor) = self.active_editor() else {
            return;
        };
        let mut buffers_to_fold = HashSet::default();
        self.collapsed_entries
            .extend(self.cached_entries.iter().filter_map(
                |cached_entry| match &cached_entry.entry {
                    PanelEntry::Fs(FsEntry::Directory(FsEntryDirectory {
                        worktree_id,
                        entry,
                        ..
                    })) => Some(CollapsedEntry::Dir(*worktree_id, entry.path.clone())),
                    PanelEntry::Fs(FsEntry::File(FsEntryFile {
                        worktree_id,
                        buffer_id,
                        ..
                    })) => {
                        buffers_to_fold.insert(*buffer_id);
                        Some(CollapsedEntry::File(*worktree_id, *buffer_id))
                    }
                    PanelEntry::Fs(FsEntry::ExternalFile(external_file)) => {
                        buffers_to_fold.insert(external_file.buffer_id);
                        Some(CollapsedEntry::ExternalFile(external_file.buffer_id))
                    }
                    PanelEntry::FoldedDirs(FoldedDirsEntry {
                        worktree_id,
                        entries,
                        ..
                    }) => Some(CollapsedEntry::Dir(
                        *worktree_id,
                        entries.last()?.path.clone(),
                    )),
                    PanelEntry::Outline(OutlineEntry::Excerpt(excerpt)) => {
                        Some(CollapsedEntry::Excerpt(excerpt.clone()))
                    }
                    PanelEntry::Outline(OutlineEntry::Outline(outline)) => {
                        Some(CollapsedEntry::Outline(outline.range.clone()))
                    }
                    PanelEntry::Search(_) => None,
                },
            ));

        active_editor.update(cx, |editor, cx| {
            buffers_to_fold.retain(|buffer_id| !editor.is_buffer_folded(*buffer_id, cx));
        });
        if buffers_to_fold.is_empty() {
            self.update_cached_entries(None, window, cx);
        } else {
            self.toggle_buffers_fold(buffers_to_fold, true, window, cx)
                .detach();
        }
    }

    fn toggle_expanded(&mut self, entry: &PanelEntry, window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_editor) = self.active_editor() else {
            return;
        };
        let mut fold = false;
        let mut buffers_to_toggle = HashSet::default();
        match entry {
            PanelEntry::Fs(FsEntry::Directory(FsEntryDirectory {
                worktree_id,
                entry: dir_entry,
                ..
            })) => {
                let directory_path = &dir_entry.path;
                let collapsed_entry = CollapsedEntry::Dir(*worktree_id, directory_path.clone());
                buffers_to_toggle.extend(self.buffers_inside_directory(*worktree_id, dir_entry));
                if !self.collapsed_entries.remove(&collapsed_entry) {
                    self.collapsed_entries.insert(collapsed_entry);
                    fold = true;
                }
            }
            PanelEntry::Fs(FsEntry::File(FsEntryFile {
                worktree_id,
                buffer_id,
                ..
            })) => {
                let collapsed_entry = CollapsedEntry::File(*worktree_id, *buffer_id);
                buffers_to_toggle.insert(*buffer_id);
                if !self.collapsed_entries.remove(&collapsed_entry) {
                    self.collapsed_entries.insert(collapsed_entry);
                    fold = true;
                }
            }
            PanelEntry::Fs(FsEntry::ExternalFile(external_file)) => {
                let collapsed_entry = CollapsedEntry::ExternalFile(external_file.buffer_id);
                buffers_to_toggle.insert(external_file.buffer_id);
                if !self.collapsed_entries.remove(&collapsed_entry) {
                    self.collapsed_entries.insert(collapsed_entry);
                    fold = true;
                }
            }
            PanelEntry::FoldedDirs(FoldedDirsEntry {
                worktree_id,
                entries: dir_entries,
                ..
            }) => {
                if let Some(dir_entry) = dir_entries.first() {
                    let directory_path = &dir_entry.path;
                    let collapsed_entry = CollapsedEntry::Dir(*worktree_id, directory_path.clone());
                    buffers_to_toggle
                        .extend(self.buffers_inside_directory(*worktree_id, dir_entry));
                    if !self.collapsed_entries.remove(&collapsed_entry) {
                        self.collapsed_entries.insert(collapsed_entry);
                        fold = true;
                    }
                }
            }
            PanelEntry::Outline(OutlineEntry::Excerpt(excerpt)) => {
                let collapsed_entry = CollapsedEntry::Excerpt(excerpt.clone());
                if !self.collapsed_entries.remove(&collapsed_entry) {
                    self.collapsed_entries.insert(collapsed_entry);
                }
            }
            PanelEntry::Outline(OutlineEntry::Outline(outline)) => {
                let collapsed_entry = CollapsedEntry::Outline(outline.range.clone());
                if !self.collapsed_entries.remove(&collapsed_entry) {
                    self.collapsed_entries.insert(collapsed_entry);
                }
            }
            PanelEntry::Search(_) => return,
        }

        active_editor.update(cx, |editor, cx| {
            buffers_to_toggle.retain(|buffer_id| {
                let folded = editor.is_buffer_folded(*buffer_id, cx);
                if fold { !folded } else { folded }
            });
        });

        self.select_entry(entry.clone(), true, window, cx);
        if buffers_to_toggle.is_empty() {
            self.update_cached_entries(None, window, cx);
        } else {
            self.toggle_buffers_fold(buffers_to_toggle, fold, window, cx)
                .detach();
        }
    }

    fn toggle_buffers_fold(
        &self,
        buffers: HashSet<BufferId>,
        fold: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let Some(active_editor) = self.active_editor() else {
            return Task::ready(());
        };
        cx.spawn_in(window, async move |outline_panel, cx| {
            outline_panel
                .update_in(cx, |outline_panel, window, cx| {
                    active_editor.update(cx, |editor, cx| {
                        for buffer_id in buffers {
                            outline_panel
                                .preserve_selection_on_buffer_fold_toggles
                                .insert(buffer_id);
                            if fold {
                                editor.fold_buffer(buffer_id, cx);
                            } else {
                                editor.unfold_buffer(buffer_id, cx);
                            }
                        }
                    });
                    if let Some(selection) = outline_panel.selected_entry().cloned() {
                        outline_panel.scroll_editor_to_entry(&selection, false, false, window, cx);
                    }
                })
                .ok();
        })
    }

    fn copy_path(
        &mut self,
        _: &zed_actions::workspace::CopyPath,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(clipboard_text) = self
            .selected_entry()
            .and_then(|entry| self.abs_path(entry, cx))
            .map(|p| p.to_string_lossy().into_owned())
        {
            cx.write_to_clipboard(ClipboardItem::new_string(clipboard_text));
        }
    }

    fn copy_relative_path(
        &mut self,
        _: &zed_actions::workspace::CopyRelativePath,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let path_style = self.project.read(cx).path_style(cx);
        if let Some(clipboard_text) = self
            .selected_entry()
            .and_then(|entry| match entry {
                PanelEntry::Fs(entry) => self.relative_path(entry, cx),
                PanelEntry::FoldedDirs(folded_dirs) => {
                    folded_dirs.entries.last().map(|entry| entry.path.clone())
                }
                PanelEntry::Search(_) | PanelEntry::Outline(..) => None,
            })
            .map(|p| p.display(path_style).to_string())
        {
            cx.write_to_clipboard(ClipboardItem::new_string(clipboard_text));
        }
    }

    fn reveal_in_finder(
        &mut self,
        _: &RevealInFileManager,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(abs_path) = self
            .selected_entry()
            .and_then(|entry| self.abs_path(entry, cx))
        {
            self.project
                .update(cx, |project, cx| project.reveal_path(&abs_path, cx));
        }
    }

    fn open_in_terminal(
        &mut self,
        _: &OpenInTerminal,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selected_entry = self.selected_entry();
        let abs_path = selected_entry.and_then(|entry| self.abs_path(entry, cx));
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
            window.dispatch_action(
                workspace::OpenTerminal {
                    working_directory,
                    local: false,
                }
                .boxed_clone(),
                cx,
            )
        }
    }

    fn reveal_entry_for_selection(
        &mut self,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.active
            || !OutlinePanelSettings::get_global(cx).auto_reveal_entries
            || self.focus_handle.contains_focused(window, cx)
        {
            return;
        }
        self.reveal_selection_task = cx.spawn_in(window, async move |outline_panel, cx| {
            cx.background_executor().timer(UPDATE_DEBOUNCE).await;
            outline_panel.update_in(cx, |outline_panel, window, cx| {
                let Some(entry) = outline_panel.location_for_editor_selection(&editor, window, cx)
                else {
                    outline_panel.selected_entry = SelectedEntry::None;
                    cx.notify();
                    return;
                };
                let buffer_id = match &entry {
                    PanelEntry::Fs(entry) => entry.buffer_id(),
                    PanelEntry::Outline(outline) => Some(outline.buffer_id()),
                    PanelEntry::Search(search) => editor
                        .read(cx)
                        .buffer()
                        .read(cx)
                        .snapshot(cx)
                        .anchor_to_buffer_anchor(search.match_range.start)
                        .map(|(anchor, _)| anchor.buffer_id),
                    PanelEntry::FoldedDirs(_) => None,
                };
                let Some(buffer_id) = buffer_id else { return };
                let collapsed_count = outline_panel.collapsed_entries.len();
                if let PanelEntry::Outline(outline) = &entry {
                    let range = outline.range();
                    if let Some(snapshot) = outline_panel.buffer_snapshot_for_id(buffer_id, cx) {
                        outline_panel.collapsed_entries.retain(|entry| match entry {
                            CollapsedEntry::Excerpt(excerpt) => {
                                excerpt.context.start.buffer_id != buffer_id
                                    || !(excerpt.contains(&range.start, &snapshot)
                                        || excerpt.contains(&range.end, &snapshot))
                            }
                            _ => true,
                        });
                    }
                }
                let file_path = outline_panel
                    .fs_entries
                    .iter()
                    .find_map(|entry| match entry {
                        FsEntry::File(file) if file.buffer_id == buffer_id => Some(ProjectPath {
                            worktree_id: file.worktree_id,
                            path: file.entry.path.clone(),
                        }),
                        _ => None,
                    });
                let reveal_buffer_contents =
                    matches!(entry, PanelEntry::Outline(_) | PanelEntry::Search(_));
                if reveal_buffer_contents {
                    outline_panel
                        .collapsed_entries
                        .remove(&CollapsedEntry::ExternalFile(buffer_id));
                }
                if let Some(path) = file_path {
                    if reveal_buffer_contents {
                        outline_panel
                            .collapsed_entries
                            .remove(&CollapsedEntry::File(path.worktree_id, buffer_id));
                    }
                    for ancestor in path.path.ancestors().skip(1) {
                        outline_panel
                            .collapsed_entries
                            .remove(&CollapsedEntry::Dir(path.worktree_id, Arc::from(ancestor)));
                    }
                }
                outline_panel.select_entry(entry, false, window, cx);
                if outline_panel.collapsed_entries.len() != collapsed_count {
                    outline_panel.update_cached_entries(None, window, cx);
                }
            })?;
            anyhow::Ok(())
        });
    }

    fn render_excerpt(
        &self,
        excerpt: &ExcerptRange<Anchor>,
        depth: usize,
        window: &mut Window,
        cx: &mut Context<OutlinePanel>,
    ) -> Option<Stateful<Div>> {
        let item_id = ElementId::from(format!("{excerpt:?}"));
        let is_active = match self.selected_entry() {
            Some(PanelEntry::Outline(OutlineEntry::Excerpt(selected_excerpt))) => {
                selected_excerpt == excerpt
            }
            _ => false,
        };
        let has_outlines = self
            .buffers
            .get(&excerpt.context.start.buffer_id)
            .and_then(|buffer| match &buffer.outlines {
                OutlineState::Outlines(outlines) => Some(outlines),
                OutlineState::Invalidated(outlines) => Some(outlines),
                OutlineState::NotFetched => None,
            })
            .is_some_and(|outlines| !outlines.is_empty());
        let is_expanded = !self
            .collapsed_entries
            .contains(&CollapsedEntry::Excerpt(excerpt.clone()));
        let color = entry_label_color(is_active);
        let icon = if has_outlines {
            FileIcons::get_chevron_icon(is_expanded, cx)
                .map(|icon_path| Icon::from_path(icon_path).color(color).into_any_element())
        } else {
            None
        }
        .unwrap_or_else(empty_icon);

        let label = self.excerpt_label(&excerpt, cx)?;
        let label_element = Label::new(label)
            .single_line()
            .color(color)
            .into_any_element();

        Some(self.entry_element(
            PanelEntry::Outline(OutlineEntry::Excerpt(excerpt.clone())),
            item_id,
            depth,
            icon,
            is_active,
            label_element,
            window,
            cx,
        ))
    }

    fn excerpt_label(&self, range: &ExcerptRange<language::Anchor>, cx: &App) -> Option<String> {
        let buffer_snapshot = self.buffer_snapshot_for_id(range.context.start.buffer_id, cx)?;
        let excerpt_range = range.context.to_point(&buffer_snapshot);
        Some(format!(
            "Lines {}- {}",
            excerpt_range.start.row + 1,
            excerpt_range.end.row + 1,
        ))
    }

    fn render_outline(
        &self,
        outline: &Outline,
        depth: usize,
        string_match: Option<&StringMatch>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let item_id = ElementId::from(SharedString::from(format!(
            "{:?}|{:?}",
            outline.range, outline.text,
        )));

        let label_element = outline::render_item(
            &outline,
            string_match
                .map(|string_match| string_match.ranges().collect::<Vec<_>>())
                .unwrap_or_default(),
            cx,
        )
        .into_any_element();

        let is_active = match self.selected_entry() {
            Some(PanelEntry::Outline(OutlineEntry::Outline(selected))) => outline == selected,
            _ => false,
        };

        let has_children = self
            .outline_children_cache
            .get(&outline.range.start.buffer_id)
            .and_then(|children_map| {
                let key = (outline.range.clone(), outline.depth);
                children_map.get(&key)
            })
            .copied()
            .unwrap_or(false);
        let is_expanded = !self
            .collapsed_entries
            .contains(&CollapsedEntry::Outline(outline.range.clone()));

        let icon = if has_children {
            FileIcons::get_chevron_icon(is_expanded, cx)
                .map(|icon_path| {
                    Icon::from_path(icon_path)
                        .color(entry_label_color(is_active))
                        .into_any_element()
                })
                .unwrap_or_else(empty_icon)
        } else {
            empty_icon()
        };

        self.entry_element(
            PanelEntry::Outline(OutlineEntry::Outline(outline.clone())),
            item_id,
            depth,
            icon,
            is_active,
            label_element,
            window,
            cx,
        )
    }

    fn render_entry(
        &self,
        rendered_entry: &FsEntry,
        depth: usize,
        string_match: Option<&StringMatch>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let settings = OutlinePanelSettings::get_global(cx);
        let is_active = match self.selected_entry() {
            Some(PanelEntry::Fs(selected_entry)) => selected_entry == rendered_entry,
            _ => false,
        };
        let (item_id, label_element, icon) = match rendered_entry {
            FsEntry::File(FsEntryFile {
                worktree_id,
                entry,
                buffer_id,
                ..
            }) => {
                let name = self.entry_name(worktree_id, entry, cx);
                let color =
                    entry_git_aware_label_color(entry.git_summary, entry.is_ignored, is_active);
                let icon = if settings.file_icons {
                    FileIcons::get_icon(Path::new(&name), cx)
                        .map(|icon_path| Icon::from_path(icon_path).color(color).into_any_element())
                } else {
                    None
                };
                (
                    ElementId::from(("buffer", buffer_id.to_proto())),
                    HighlightedLabel::new(
                        name,
                        string_match
                            .map(|string_match| string_match.positions.clone())
                            .unwrap_or_default(),
                    )
                    .color(color)
                    .into_any_element(),
                    reserve_chevron_slot(
                        settings.folder_indicator,
                        icon.unwrap_or_else(empty_icon),
                    ),
                )
            }
            FsEntry::Directory(directory) => {
                let name = self.entry_name(&directory.worktree_id, &directory.entry, cx);

                let is_expanded = !self.collapsed_entries.contains(&CollapsedEntry::Dir(
                    directory.worktree_id,
                    directory.entry.path.clone(),
                ));
                let color = entry_git_aware_label_color(
                    directory.entry.git_summary,
                    directory.entry.is_ignored,
                    is_active,
                );
                let icon = folder_indicator_element(
                    settings.folder_indicator,
                    is_expanded,
                    directory.entry.path.as_std_path(),
                    color,
                    cx,
                );
                (
                    directory_element_id(
                        directory.worktree_id,
                        directory.first_buffer_id,
                        &directory.entry.path,
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
            }
            FsEntry::ExternalFile(external_file) => {
                let color = entry_label_color(is_active);
                let (icon, name) = match self.buffer_snapshot_for_id(external_file.buffer_id, cx) {
                    Some(buffer_snapshot) => match buffer_snapshot.file() {
                        Some(file) => {
                            let name = file.file_name(cx).to_string();
                            let icon = if settings.file_icons {
                                FileIcons::get_icon(Path::new(&name), cx)
                            } else {
                                None
                            }
                            .map(Icon::from_path)
                            .map(|icon| icon.color(color).into_any_element());
                            (icon, name)
                        }
                        None => (None, "Untitled".to_string()),
                    },
                    None => (None, "Unknown buffer".to_string()),
                };
                (
                    ElementId::from(("buffer", external_file.buffer_id.to_proto())),
                    HighlightedLabel::new(
                        name,
                        string_match
                            .map(|string_match| string_match.positions.clone())
                            .unwrap_or_default(),
                    )
                    .color(color)
                    .into_any_element(),
                    reserve_chevron_slot(
                        settings.folder_indicator,
                        icon.unwrap_or_else(empty_icon),
                    ),
                )
            }
        };

        self.entry_element(
            PanelEntry::Fs(rendered_entry.clone()),
            item_id,
            depth,
            icon,
            is_active,
            label_element,
            window,
            cx,
        )
    }

    fn render_folded_dirs(
        &self,
        folded_dir: &FoldedDirsEntry,
        depth: usize,
        string_match: Option<&StringMatch>,
        window: &mut Window,
        cx: &mut Context<OutlinePanel>,
    ) -> Stateful<Div> {
        let settings = OutlinePanelSettings::get_global(cx);
        let is_active = match self.selected_entry() {
            Some(PanelEntry::FoldedDirs(selected_dirs)) => selected_dirs == folded_dir,
            _ => false,
        };
        let (item_id, label_element, icon) = {
            let name = self.dir_names_string(&folded_dir.entries, folded_dir.worktree_id, cx);

            let is_expanded = folded_dir.entries.iter().all(|dir| {
                !self.collapsed_entries.contains(&CollapsedEntry::Dir(
                    folded_dir.worktree_id,
                    dir.path.clone(),
                ))
            });
            let is_ignored = folded_dir.entries.iter().any(|entry| entry.is_ignored);
            let git_status = folded_dir
                .entries
                .first()
                .map(|entry| entry.git_summary)
                .unwrap_or_default();
            let color = entry_git_aware_label_color(git_status, is_ignored, is_active);
            let icon = folder_indicator_element(
                settings.folder_indicator,
                is_expanded,
                Path::new(&name),
                color,
                cx,
            );
            (
                folded_dir
                    .entries
                    .last()
                    .map(|entry| {
                        directory_element_id(
                            folded_dir.worktree_id,
                            folded_dir.first_buffer_id,
                            &entry.path,
                        )
                    })
                    .unwrap_or_else(|| {
                        ElementId::from(("empty-folded-dirs", folded_dir.worktree_id.to_proto()))
                    }),
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
            PanelEntry::FoldedDirs(folded_dir.clone()),
            item_id,
            depth,
            icon,
            is_active,
            label_element,
            window,
            cx,
        )
    }

    fn render_search_match(
        &mut self,
        multi_buffer_snapshot: Option<&MultiBufferSnapshot>,
        match_range: &Range<editor::Anchor>,
        render_data: &Arc<OnceLock<SearchData>>,
        kind: SearchKind,
        depth: usize,
        string_match: Option<&StringMatch>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Stateful<Div>> {
        let search_data = match render_data.get() {
            Some(search_data) => search_data,
            None => {
                if let ItemsDisplayMode::Search(search_state) = &mut self.mode
                    && let Some(multi_buffer_snapshot) = multi_buffer_snapshot
                {
                    search_state
                        .highlight_search_match_tx
                        .try_send(HighlightArguments {
                            multi_buffer_snapshot: multi_buffer_snapshot.clone(),
                            match_range: match_range.clone(),
                            search_data: Arc::clone(render_data),
                        })
                        .ok();
                }
                return None;
            }
        };
        let search_matches = string_match
            .iter()
            .flat_map(|string_match| string_match.ranges())
            .collect::<Vec<_>>();
        let match_ranges = if search_matches.is_empty() {
            &search_data.search_match_indices
        } else {
            &search_matches
        };
        let label_element = outline::render_item(
            &OutlineItem {
                depth,
                annotation_range: None,
                range: search_data.context_range.clone(),
                selection_range: search_data.context_range.clone(),
                text: search_data.context_text.clone().into(),
                source_range_for_text: search_data.context_range.clone(),
                highlight_ranges: search_data
                    .highlights_data
                    .get()
                    .cloned()
                    .unwrap_or_default(),
                name_ranges: search_data.search_match_indices.clone(),
                body_range: Some(search_data.context_range.clone()),
            },
            match_ranges.iter().cloned(),
            cx,
        );
        let truncated_contents_label = || Label::new(TRUNCATED_CONTEXT_MARK);
        let entire_label = h_flex()
            .justify_center()
            .p_0()
            .when(search_data.truncated_left, |parent| {
                parent.child(truncated_contents_label())
            })
            .child(label_element)
            .when(search_data.truncated_right, |parent| {
                parent.child(truncated_contents_label())
            })
            .into_any_element();

        let is_active = match self.selected_entry() {
            Some(PanelEntry::Search(SearchEntry {
                match_range: selected_match_range,
                ..
            })) => match_range == selected_match_range,
            _ => false,
        };
        Some(self.entry_element(
            PanelEntry::Search(SearchEntry {
                kind,
                match_range: match_range.clone(),
                render_data: render_data.clone(),
            }),
            ElementId::from(SharedString::from(format!("search-{match_range:?}"))),
            depth,
            empty_icon(),
            is_active,
            entire_label,
            window,
            cx,
        ))
    }

    fn entry_element(
        &self,
        rendered_entry: PanelEntry,
        item_id: ElementId,
        depth: usize,
        icon_element: AnyElement,
        is_active: bool,
        label_element: gpui::AnyElement,
        window: &mut Window,
        cx: &mut Context<OutlinePanel>,
    ) -> Stateful<Div> {
        let settings = OutlinePanelSettings::get_global(cx);
        let is_deleted = match &rendered_entry {
            PanelEntry::Fs(FsEntry::File(file)) => file.is_deleted,
            _ => false,
        };
        div()
            .text_ui(cx)
            .id(item_id.clone())
            .on_click({
                let clicked_entry = rendered_entry.clone();
                cx.listener(move |outline_panel, event: &gpui::ClickEvent, window, cx| {
                    if event.is_right_click() || event.first_focus() {
                        return;
                    }

                    let change_focus = event.click_count() > 1;
                    outline_panel.toggle_expanded(&clicked_entry, window, cx);

                    outline_panel.scroll_editor_to_entry(
                        &clicked_entry,
                        true,
                        change_focus,
                        window,
                        cx,
                    );
                })
            })
            .cursor_pointer()
            .child(
                ListItem::new(item_id)
                    .indent_level(depth)
                    .indent_step_size(px(settings.indent_size))
                    .toggle_state(is_active)
                    .child(
                        h_flex()
                            .child(h_flex().min_w(px(16.)).justify_center().child(icon_element))
                            .child(
                                h_flex()
                                    .h_6()
                                    .child(label_element)
                                    .ml_1()
                                    .when(is_deleted, |this| this.line_through()),
                            ),
                    )
                    .on_secondary_mouse_down(cx.listener(
                        move |outline_panel, event: &MouseDownEvent, window, cx| {
                            // Stop propagation to prevent the catch-all context menu for the project
                            // panel from being deployed.
                            cx.stop_propagation();
                            outline_panel.deploy_context_menu(
                                event.position,
                                rendered_entry.clone(),
                                window,
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
            .when(
                is_active && self.focus_handle.contains_focused(window, cx),
                |div| div.border_color(cx.theme().colors().panel_focused_border),
            )
    }

    fn entry_name(&self, worktree_id: &WorktreeId, entry: &FsEntryPath, cx: &App) -> String {
        match self.project.read(cx).worktree_for_id(*worktree_id, cx) {
            Some(worktree) => file_name(&worktree.read(cx).absolutize(&entry.path)),
            None => file_name(entry.path.as_std_path()),
        }
    }

    fn fs_entry_label(&self, fs_entry: &FsEntry, cx: &App) -> Option<String> {
        match fs_entry {
            FsEntry::ExternalFile(external) => Some(
                self.buffer_snapshot_for_id(external.buffer_id, cx)?
                    .file()?
                    .file_name(cx)
                    .to_string(),
            ),
            FsEntry::Directory(FsEntryDirectory {
                worktree_id, entry, ..
            })
            | FsEntry::File(FsEntryFile {
                worktree_id, entry, ..
            }) => match entry.path.file_name() {
                Some(name) => Some(name.to_string()),
                None => Some(self.entry_name(worktree_id, entry, cx)),
            },
        }
    }

    fn update_fs_entries(
        &mut self,
        active_editor: Entity<Editor>,
        debounce: Option<Duration>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.active || (debounce.is_some() && self.fs_entries_update_pending) {
            return;
        }
        self.fs_entries_update_pending = true;

        self.fs_entries_update_task = cx.spawn_in(window, async move |outline_panel, cx| {
            if let Some(debounce) = debounce {
                cx.background_executor().timer(debounce).await;
            }

            let mut new_buffers = HashMap::<BufferId, BufferOutlines>::default();
            let Ok((
                buffer_excerpts,
                auto_fold_dirs,
                repo_snapshots,
                mut collapsed_entries,
                mut unfolded_dirs,
            )) = outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.fs_entries_update_pending = false;
                let auto_fold_dirs = OutlinePanelSettings::get_global(cx).auto_fold_dirs;
                let git_store = outline_panel.project.read(cx).git_store().clone();
                let repo_snapshots = git_store.read(cx).display_repo_snapshots(cx);
                let editor = active_editor.read(cx);
                let snapshot = editor.buffer().read(cx).snapshot(cx);
                let mut buffer_excerpts = IndexMap::<BufferId, BufferExcerpts>::default();

                for excerpt in snapshot.excerpts() {
                    let buffer_id = excerpt.context.start.buffer_id;
                    let Some(buffer) = snapshot.buffer_for_id(buffer_id) else {
                        continue;
                    };
                    buffer_excerpts
                        .entry(buffer_id)
                        .or_insert_with(|| {
                            let status = git_store
                                .read(cx)
                                .display_status_for_buffer_id(buffer_id, cx);
                            let file = File::from_dyn(buffer.file());
                            let is_deleted = file.is_some_and(|file| {
                                is_deleted_file(
                                    file,
                                    status,
                                    snapshot
                                        .diff_for_buffer_id(buffer_id)
                                        .is_some_and(|diff| diff.base_text_exists()),
                                )
                            });
                            let file = file.and_then(|file| {
                                let worktree = file.worktree.read(cx).snapshot();
                                let entry = file
                                    .project_entry_id()
                                    .and_then(|id| worktree.entry_for_id(id));
                                if entry.is_none() && !is_deleted {
                                    return None;
                                }
                                let path_entry = FsEntryPath {
                                    path: file.path.clone(),
                                    git_summary: status.map(GitSummary::from).unwrap_or_default(),
                                    is_ignored: entry.is_some_and(|entry| entry.is_ignored),
                                };
                                Some((worktree, path_entry))
                            });
                            BufferExcerpts {
                                is_new: outline_panel
                                    .new_entries_for_fs_update
                                    .contains(&buffer_id)
                                    || !outline_panel.buffers.contains_key(&buffer_id),
                                is_folded: editor.is_buffer_folded(buffer_id, cx),
                                excerpts: Vec::new(),
                                file,
                                is_deleted,
                            }
                        })
                        .excerpts
                        .push(excerpt.clone());

                    new_buffers
                        .entry(buffer_id)
                        .or_insert_with(|| {
                            let outlines = match outline_panel.buffers.get(&buffer_id) {
                                Some(BufferOutlines {
                                    outlines: OutlineState::Outlines(outlines),
                                    ..
                                }) => OutlineState::Outlines(outlines.clone()),
                                _ => OutlineState::NotFetched,
                            };
                            BufferOutlines {
                                outlines,
                                excerpts: Vec::new(),
                            }
                        })
                        .excerpts
                        .push(excerpt);
                }
                (
                    buffer_excerpts,
                    auto_fold_dirs,
                    repo_snapshots,
                    outline_panel.collapsed_entries.clone(),
                    outline_panel.unfolded_dirs.clone(),
                )
            })
            else {
                return;
            };

            let (fs_entries, children_count, collapsed_entries, unfolded_dirs) = cx
                .background_spawn(async move {
                    let mut fs_entries = Vec::new();
                    let mut directories = HashMap::<ProjectPath, FsEntryPath>::default();
                    let mut parent_paths = Vec::<ProjectPath>::new();
                    let repositories_by_root = repo_snapshots
                        .values()
                        .map(|snapshot| (snapshot.work_directory_abs_path.as_ref(), snapshot))
                        .collect::<BTreeMap<_, _>>();
                    for (buffer_id, buffer) in buffer_excerpts {
                        let collapsed_entry = match &buffer.file {
                            Some((worktree, _)) => CollapsedEntry::File(worktree.id(), buffer_id),
                            None => CollapsedEntry::ExternalFile(buffer_id),
                        };
                        if buffer.is_folded {
                            collapsed_entries.insert(collapsed_entry);
                        } else if buffer.is_new {
                            collapsed_entries.remove(&collapsed_entry);
                        }
                        let Some((worktree, entry)) = buffer.file else {
                            parent_paths.clear();
                            fs_entries.push(FsEntry::ExternalFile(FsEntryExternalFile {
                                buffer_id,
                                excerpts: Arc::from(buffer.excerpts),
                            }));
                            continue;
                        };
                        let worktree_id = worktree.id();
                        let mut ancestors = entry.path.ancestors().skip(1).collect::<Vec<_>>();
                        ancestors.reverse();
                        let shared_depth = parent_paths
                            .last()
                            .filter(|parent| parent.worktree_id == worktree_id)
                            .zip(ancestors.last())
                            .map(|(parent, ancestor)| {
                                parent
                                    .path
                                    .components()
                                    .zip(ancestor.components())
                                    .take_while(|(left, right)| left == right)
                                    .count()
                                    + 1
                            })
                            .unwrap_or(0);
                        parent_paths.truncate(shared_depth);
                        for (depth, path) in ancestors.into_iter().enumerate() {
                            let ancestor =
                                parent_paths
                                    .get(depth)
                                    .cloned()
                                    .unwrap_or_else(|| ProjectPath {
                                        worktree_id,
                                        path: Arc::from(path),
                                    });
                            if buffer.is_new {
                                collapsed_entries.remove(&CollapsedEntry::Dir(
                                    worktree_id,
                                    ancestor.path.clone(),
                                ));
                            }
                            if depth < shared_depth {
                                continue;
                            }
                            let directory =
                                directories.entry(ancestor.clone()).or_insert_with(|| {
                                    let indexed_entry = worktree
                                        .entry_for_path(&ancestor.path)
                                        .filter(|entry| entry.is_dir());
                                    let git_summary = indexed_entry
                                        .and_then(|_| {
                                            let abs_path = worktree.absolutize(&ancestor.path);
                                            for query in abs_path.ancestors() {
                                                let (_, repository) = repositories_by_root
                                                    .range(Path::new("")..=query)
                                                    .next_back()?;
                                                let Some(path) =
                                                    repository.abs_path_to_repo_path(&abs_path)
                                                else {
                                                    continue;
                                                };
                                                let mut statuses = repository
                                                    .statuses_by_path
                                                    .cursor::<PathProgress>(());
                                                statuses.seek_forward(
                                                    &PathTarget::Path(&path),
                                                    Bias::Left,
                                                );
                                                return Some(statuses.summary(
                                                    &PathTarget::Successor(&path),
                                                    Bias::Left,
                                                ));
                                            }
                                            None
                                        })
                                        .unwrap_or_default();
                                    FsEntryPath {
                                        path: ancestor.path.clone(),
                                        git_summary,
                                        is_ignored: indexed_entry
                                            .is_some_and(|entry| entry.is_ignored),
                                    }
                                });
                            fs_entries.push(FsEntry::Directory(FsEntryDirectory {
                                worktree_id,
                                first_buffer_id: buffer_id,
                                entry: directory.clone(),
                            }));
                            parent_paths.push(ancestor);
                        }
                        fs_entries.push(FsEntry::File(FsEntryFile {
                            worktree_id,
                            entry,
                            buffer_id,
                            excerpts: Arc::from(buffer.excerpts),
                            is_deleted: buffer.is_deleted,
                        }));
                    }
                    collapsed_entries.retain(|entry| match entry {
                        CollapsedEntry::Dir(worktree_id, path) => {
                            directories.contains_key(&ProjectPath {
                                worktree_id: *worktree_id,
                                path: path.clone(),
                            })
                        }
                        _ => true,
                    });
                    unfolded_dirs.retain(|worktree_id, paths| {
                        paths.retain(|path| {
                            directories.contains_key(&ProjectPath {
                                worktree_id: *worktree_id,
                                path: path.clone(),
                            })
                        });
                        !paths.is_empty()
                    });
                    let mut children_count =
                        HashMap::<WorktreeId, HashMap<Arc<RelPath>, FsChildren>>::default();
                    for directory in directories.keys() {
                        if let Some(parent) = directory.path.parent() {
                            children_count
                                .entry(directory.worktree_id)
                                .or_default()
                                .entry(Arc::from(parent))
                                .or_default()
                                .dirs += 1;
                        }
                    }
                    for entry in &fs_entries {
                        if let FsEntry::File(file) = entry
                            && let Some(parent) = file.entry.path.parent()
                        {
                            children_count
                                .entry(file.worktree_id)
                                .or_default()
                                .entry(Arc::from(parent))
                                .or_default()
                                .files += 1;
                        }
                    }
                    if auto_fold_dirs {
                        for entry in &fs_entries {
                            let FsEntry::Directory(directory) = entry else {
                                continue;
                            };
                            let unfolded = unfolded_dirs.entry(directory.worktree_id).or_default();
                            let path = &directory.entry.path;
                            let children = children_count
                                .get(&directory.worktree_id)
                                .and_then(|children| children.get(path))
                                .copied()
                                .unwrap_or_default();
                            let parent_unfolded =
                                path.parent().is_none_or(|parent| unfolded.contains(parent));
                            if path.is_empty()
                                || !children.may_be_fold_part()
                                || (children.dirs == 0 && parent_unfolded)
                            {
                                unfolded.insert(path.clone());
                            }
                        }
                    }
                    (fs_entries, children_count, collapsed_entries, unfolded_dirs)
                })
                .await;

            outline_panel
                .update_in(cx, |outline_panel, window, cx| {
                    outline_panel.new_entries_for_fs_update.clear();
                    outline_panel.buffers = new_buffers;
                    outline_panel.collapsed_entries = collapsed_entries;
                    outline_panel.unfolded_dirs = unfolded_dirs;
                    outline_panel.fs_entries = fs_entries;
                    outline_panel.fs_children_count = children_count;
                    outline_panel.update_non_fs_items(window, cx);
                    if outline_panel.buffers_to_fetch(cx).is_empty() {
                        outline_panel.update_cached_entries(debounce, window, cx);
                    }
                    cx.notify();
                })
                .ok();
        });
    }

    fn replace_active_editor(
        &mut self,
        new_active_item: Box<dyn ItemHandle>,
        new_active_editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.clear_previous(window, cx);

        let default_expansion_depth =
            OutlinePanelSettings::get_global(cx).expand_outlines_with_depth;
        // We'll apply the expansion depth after outlines are loaded
        self.pending_default_expansion_depth = Some(default_expansion_depth);

        let buffer_search_subscription = cx.subscribe_in(
            &new_active_editor,
            window,
            |outline_panel: &mut Self,
             _,
             e: &SearchEvent,
             window: &mut Window,
             cx: &mut Context<Self>| {
                if matches!(e, SearchEvent::MatchesInvalidated) {
                    outline_panel.update_contents(Some(UPDATE_DEBOUNCE), window, cx);
                }
            },
        );
        self.active_item = Some(ActiveItem {
            _buffer_search_subscription: buffer_search_subscription,
            _editor_subscription: subscribe_for_editor_events(&new_active_editor, window, cx),
            item_handle: new_active_item.downgrade_item(),
            active_editor: new_active_editor.downgrade(),
        });
        self.new_entries_for_fs_update.extend(
            new_active_editor
                .read(cx)
                .buffer()
                .read(cx)
                .snapshot(cx)
                .excerpts()
                .map(|excerpt| excerpt.context.start.buffer_id),
        );
        self.selected_entry.invalidate();
        self.update_fs_entries(new_active_editor, None, window, cx);
    }

    fn clear_previous(&mut self, window: &mut Window, cx: &mut App) {
        self.fs_entries_update_task = Task::ready(());
        self.fs_entries_update_pending = false;
        self.lsp_outline_refresh_task = Task::ready(());
        self.outline_fetch_tasks.clear();
        self.cached_entries_update_task = Task::ready(());
        self.cached_entries_update_pending = None;
        self.reveal_selection_task = Task::ready(Ok(()));
        self.filter_editor
            .update(cx, |editor, cx| editor.clear(window, cx));
        self.collapsed_entries.clear();
        self.unfolded_dirs.clear();
        self.active_item = None;
        self.fs_entries.clear();
        self.fs_children_count.clear();
        self.buffers.clear();
        self.cached_entries = Vec::new();
        self.selected_entry = SelectedEntry::None;
        self.pinned = false;
        self.mode = ItemsDisplayMode::Outline;
        self.pending_default_expansion_depth = None;
        self.default_depth_applied_buffers.clear();
    }

    fn location_for_editor_selection(
        &self,
        editor: &Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<PanelEntry> {
        let editor_snapshot = editor.update(cx, |editor, cx| editor.snapshot(window, cx));
        let multi_buffer = editor.read(cx).buffer();
        let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);
        let anchor = editor.update(cx, |editor, _| editor.selections.newest_anchor().head());
        let selection_display_point = anchor.to_display_point(&editor_snapshot);
        let (anchor, _) = multi_buffer_snapshot.anchor_to_buffer_anchor(anchor)?;

        if editor.read(cx).is_buffer_folded(anchor.buffer_id, cx) {
            return self
                .fs_entries
                .iter()
                .find(|fs_entry| match fs_entry {
                    FsEntry::Directory(..) => false,
                    FsEntry::File(FsEntryFile {
                        buffer_id: other_buffer_id,
                        ..
                    })
                    | FsEntry::ExternalFile(FsEntryExternalFile {
                        buffer_id: other_buffer_id,
                        ..
                    }) => anchor.buffer_id == *other_buffer_id,
                })
                .cloned()
                .map(PanelEntry::Fs);
        }

        match &self.mode {
            ItemsDisplayMode::Search(search_state) => search_state
                .matches
                .iter()
                .rev()
                .min_by_key(|&(match_range, _)| {
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
                .and_then(|(closest_range, _)| {
                    self.cached_entries.iter().find_map(|cached_entry| {
                        if let PanelEntry::Search(SearchEntry { match_range, .. }) =
                            &cached_entry.entry
                        {
                            if match_range == closest_range {
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
                anchor,
                multi_buffer_snapshot,
                editor_snapshot,
                selection_display_point,
                cx,
            ),
        }
    }

    fn outline_location(
        &self,
        selection_anchor: Anchor,
        multi_buffer_snapshot: editor::MultiBufferSnapshot,
        editor_snapshot: editor::EditorSnapshot,
        selection_display_point: DisplayPoint,
        cx: &App,
    ) -> Option<PanelEntry> {
        let buffer_snapshot = multi_buffer_snapshot.buffer_for_id(selection_anchor.buffer_id);
        let mut excerpt_ranges = multi_buffer_snapshot
            .excerpts_for_buffer(selection_anchor.buffer_id)
            .map(|excerpt| excerpt.context)
            .collect::<Vec<_>>();
        if let Some(snapshot) = buffer_snapshot {
            excerpt_ranges.sort_unstable_by(|left, right| left.start.cmp(&right.start, snapshot));
            let mut maximum_end = None;
            for range in &mut excerpt_ranges {
                if let Some(end) = maximum_end
                    && range.end.cmp(&end, snapshot).is_lt()
                {
                    range.end = end;
                }
                maximum_end = Some(range.end);
            }
        }

        let excerpt_outlines = self
            .buffers
            .get(&selection_anchor.buffer_id)
            .into_iter()
            .flat_map(|buffer| buffer.iter_outlines())
            .flat_map(|outline| {
                let snapshot = buffer_snapshot?;
                let (start, end) = if outline
                    .range
                    .start
                    .cmp(&outline.range.end, snapshot)
                    .is_gt()
                {
                    (outline.range.end, outline.range.start)
                } else {
                    (outline.range.start, outline.range.end)
                };
                let index = excerpt_ranges
                    .partition_point(|excerpt| excerpt.start.cmp(&start, snapshot).is_le())
                    .checked_sub(1)?;
                if excerpt_ranges.get(index)?.end.cmp(&end, snapshot).is_lt() {
                    return None;
                }
                let range = multi_buffer_snapshot.anchor_range_in_buffer(outline.range.clone())?;
                Some((
                    range.start.to_display_point(&editor_snapshot)
                        ..range.end.to_display_point(&editor_snapshot),
                    outline,
                ))
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

                // An outline item's range can extend to the same row the next
                // item starts on, so when the cursor is at the start of that
                // row, prefer the item that starts there over any item whose
                // range merely overlaps that row.
                let cursor_not_at_outline_start = outline_range.start != selection_display_point;
                (
                    cursor_not_at_outline_start,
                    cmp::Reverse(outline.depth),
                    distance_from_start,
                    distance_from_end,
                )
            })
            .map(|(_, (_, outline))| *outline)
            .cloned();

        let closest_container = match outline_item {
            Some(outline) => PanelEntry::Outline(OutlineEntry::Outline(outline)),
            None => {
                self.cached_entries.iter().rev().find_map(|cached_entry| {
                    match &cached_entry.entry {
                        PanelEntry::Outline(OutlineEntry::Excerpt(excerpt)) => {
                            if excerpt.context.start.buffer_id == selection_anchor.buffer_id
                                && let Some(buffer_snapshot) =
                                    self.buffer_snapshot_for_id(excerpt.context.start.buffer_id, cx)
                                && excerpt.contains(&selection_anchor, &buffer_snapshot)
                            {
                                Some(cached_entry.entry.clone())
                            } else {
                                None
                            }
                        }
                        PanelEntry::Fs(
                            FsEntry::ExternalFile(FsEntryExternalFile {
                                buffer_id: file_buffer_id,
                                excerpts: file_excerpts,
                                ..
                            })
                            | FsEntry::File(FsEntryFile {
                                buffer_id: file_buffer_id,
                                excerpts: file_excerpts,
                                ..
                            }),
                        ) => {
                            if *file_buffer_id == selection_anchor.buffer_id
                                && let Some(buffer_snapshot) =
                                    self.buffer_snapshot_for_id(*file_buffer_id, cx)
                                && file_excerpts.iter().any(|excerpt| {
                                    excerpt.contains(&selection_anchor, &buffer_snapshot)
                                })
                            {
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

    fn fetch_outdated_outlines(
        &mut self,
        debounce: Option<Duration>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let buffers_to_fetch = self.buffers_to_fetch(cx);
        if buffers_to_fetch.is_empty() {
            return;
        }

        let first_update = Arc::new(AtomicBool::new(true));
        for buffer_id in buffers_to_fetch {
            if self
                .outline_fetch_tasks
                .get(&buffer_id)
                .is_some_and(|task| !task.is_ready())
            {
                continue;
            }
            let first_update = first_update.clone();

            self.outline_fetch_tasks.insert(
                buffer_id,
                cx.spawn_in(window, async move |outline_panel, cx| {
                    if let Some(debounce) = debounce {
                        cx.background_executor().timer(debounce).await;
                    }
                    let Some(outline_task) = outline_panel
                        .update(cx, |panel, cx| {
                            let editor = panel.active_editor()?;
                            Some(editor.update(cx, |editor, cx| {
                                editor.buffer_outline_items(buffer_id, cx)
                            }))
                        })
                        .ok()
                        .flatten()
                    else {
                        return;
                    };
                    let fetched_outlines = outline_task.await;

                    outline_panel
                        .update_in(cx, |outline_panel, window, cx| {
                            let pending_default_depth =
                                outline_panel.pending_default_expansion_depth.filter(|_| {
                                    outline_panel
                                        .default_depth_applied_buffers
                                        .insert(buffer_id)
                                });

                            let mut entries_changed = false;
                            if let Some(buffer) = outline_panel.buffers.get_mut(&buffer_id) {
                                entries_changed = pending_default_depth.is_some()
                                    || match &buffer.outlines {
                                        OutlineState::Outlines(outlines)
                                        | OutlineState::Invalidated(outlines) => {
                                            outlines != &fetched_outlines
                                        }
                                        OutlineState::NotFetched => true,
                                    };
                                buffer.outlines = OutlineState::Outlines(fetched_outlines);

                                if let Some(default_depth) = pending_default_depth
                                    && let OutlineState::Outlines(outlines) = &buffer.outlines
                                {
                                    let outlines_with_children = outlines
                                        .array_windows::<2>()
                                        .filter_map(|[current, next]| {
                                            if next.depth > current.depth {
                                                Some((current.range.clone(), current.depth))
                                            } else {
                                                None
                                            }
                                        })
                                        .collect::<HashSet<_>>();
                                    outlines
                                        .iter()
                                        .filter(|outline| {
                                            (default_depth == 0 || outline.depth >= default_depth)
                                                && outlines_with_children.contains(&(
                                                    outline.range.clone(),
                                                    outline.depth,
                                                ))
                                        })
                                        .for_each(|outline| {
                                            outline_panel.collapsed_entries.insert(
                                                CollapsedEntry::Outline(outline.range.clone()),
                                            );
                                        });
                                }
                            }

                            if entries_changed
                                || matches!(
                                    &outline_panel.selected_entry,
                                    SelectedEntry::Invalidated(Some(PanelEntry::Outline(outline)))
                                        if outline.buffer_id() == buffer_id
                                )
                            {
                                let debounce =
                                    if first_update.fetch_and(false, atomic::Ordering::AcqRel) {
                                        None
                                    } else {
                                        Some(UPDATE_DEBOUNCE)
                                    };
                                if let Some(PanelEntry::Outline(outline)) =
                                    outline_panel.selected_entry()
                                    && outline.buffer_id() == buffer_id
                                {
                                    outline_panel.selected_entry.invalidate();
                                }
                                outline_panel.update_cached_entries(debounce, window, cx);
                            }
                        })
                        .ok();
                }),
            );
        }
    }

    fn is_singleton_active(&self, cx: &App) -> bool {
        self.active_editor()
            .is_some_and(|active_editor| active_editor.read(cx).buffer().read(cx).is_singleton())
    }

    fn multi_buffer_active(&self, cx: &App) -> bool {
        self.active_editor()
            .is_some_and(|active_editor| !active_editor.read(cx).buffer().read(cx).is_singleton())
    }

    fn hide_symbols_active(&self, cx: &App) -> bool {
        self.hide_symbols_override
            .unwrap_or(OutlinePanelSettings::get_global(cx).multi_buffer_hide_symbols)
    }

    fn remap_selection_for_hidden_symbols(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.multi_buffer_active(cx) || !self.hide_symbols_active(cx) {
            return;
        }
        let buffer_id = match self.selected_entry() {
            Some(PanelEntry::Outline(OutlineEntry::Excerpt(excerpt))) => {
                Some(excerpt.context.start.buffer_id)
            }
            Some(PanelEntry::Outline(OutlineEntry::Outline(outline))) => {
                Some(outline.range.start.buffer_id)
            }
            Some(PanelEntry::Search(search_entry)) => {
                let match_start = search_entry.match_range.start;
                self.active_editor().and_then(|editor| {
                    let multi_buffer_snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
                    multi_buffer_snapshot
                        .anchor_to_buffer_anchor(match_start)
                        .map(|(anchor, _)| anchor.buffer_id)
                })
            }
            Some(PanelEntry::Fs(_) | PanelEntry::FoldedDirs(_)) | None => None,
        };
        let Some(buffer_id) = buffer_id else {
            return;
        };
        let file_entry = self
            .fs_entries
            .iter()
            .find(|fs_entry| match fs_entry {
                FsEntry::File(FsEntryFile {
                    buffer_id: file_buffer_id,
                    ..
                })
                | FsEntry::ExternalFile(FsEntryExternalFile {
                    buffer_id: file_buffer_id,
                    ..
                }) => *file_buffer_id == buffer_id,
                FsEntry::Directory(..) => false,
            })
            .cloned();
        if let Some(file_entry) = file_entry {
            self.select_entry(PanelEntry::Fs(file_entry), false, window, cx);
        }
    }

    fn invalidate_outlines(&mut self, ids: &[BufferId]) {
        if let Some(PanelEntry::Outline(outline)) = self.selected_entry()
            && ids.contains(&outline.buffer_id())
        {
            self.selected_entry.invalidate();
        }
        for buffer_id in ids {
            self.outline_fetch_tasks.remove(buffer_id);
            if let Some(buffer) = self.buffers.get_mut(buffer_id) {
                buffer.invalidate_outlines();
            }
        }
    }

    fn buffers_to_fetch(&self, cx: &App) -> HashSet<BufferId> {
        // Hide-symbols mode never renders outline rows, so fetching outlines would be wasted
        // work; `update_fs_entries`'s completion only refreshes cached entries once this is
        // empty, so the gate has to live here rather than in `fetch_outdated_outlines` alone.
        if self.multi_buffer_active(cx) && self.hide_symbols_active(cx) {
            return HashSet::default();
        }
        self.fs_entries
            .iter()
            .fold(HashSet::default(), |mut buffers_to_fetch, fs_entry| {
                match fs_entry {
                    FsEntry::File(FsEntryFile { buffer_id, .. })
                    | FsEntry::ExternalFile(FsEntryExternalFile { buffer_id, .. }) => {
                        if let Some(buffer) = self.buffers.get(buffer_id)
                            && buffer.should_fetch_outlines()
                        {
                            buffers_to_fetch.insert(*buffer_id);
                        }
                    }
                    FsEntry::Directory(..) => {}
                }
                buffers_to_fetch
            })
    }

    fn buffer_snapshot_for_id(&self, buffer_id: BufferId, cx: &App) -> Option<BufferSnapshot> {
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

    fn abs_path(&self, entry: &PanelEntry, cx: &App) -> Option<PathBuf> {
        match entry {
            PanelEntry::Fs(
                FsEntry::File(FsEntryFile { buffer_id, .. })
                | FsEntry::ExternalFile(FsEntryExternalFile { buffer_id, .. }),
            ) => self
                .buffer_snapshot_for_id(*buffer_id, cx)
                .and_then(|buffer_snapshot| {
                    let file = File::from_dyn(buffer_snapshot.file())?;
                    Some(file.worktree.read(cx).absolutize(&file.path))
                }),
            PanelEntry::Fs(FsEntry::Directory(FsEntryDirectory {
                worktree_id, entry, ..
            })) => Some(
                self.project
                    .read(cx)
                    .worktree_for_id(*worktree_id, cx)?
                    .read(cx)
                    .absolutize(&entry.path),
            ),
            PanelEntry::FoldedDirs(FoldedDirsEntry {
                worktree_id,
                entries: dirs,
                ..
            }) => dirs.last().and_then(|entry| {
                self.project
                    .read(cx)
                    .worktree_for_id(*worktree_id, cx)
                    .map(|worktree| worktree.read(cx).absolutize(&entry.path))
            }),
            PanelEntry::Search(_) | PanelEntry::Outline(..) => None,
        }
    }

    fn relative_path(&self, entry: &FsEntry, cx: &App) -> Option<Arc<RelPath>> {
        match entry {
            FsEntry::ExternalFile(FsEntryExternalFile { buffer_id, .. }) => {
                let buffer_snapshot = self.buffer_snapshot_for_id(*buffer_id, cx)?;
                Some(buffer_snapshot.file()?.path().clone())
            }
            FsEntry::Directory(FsEntryDirectory { entry, .. }) => Some(entry.path.clone()),
            FsEntry::File(FsEntryFile { entry, .. }) => Some(entry.path.clone()),
        }
    }

    fn update_contents(
        &mut self,
        debounce: Option<Duration>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.active {
            return;
        }
        self.cached_entries_update_pending = Some(CachedEntriesUpdate::Contents);
        self.update_cached_entries(debounce, window, cx);
    }

    fn update_cached_entries(
        &mut self,
        debounce: Option<Duration>,
        window: &mut Window,
        cx: &mut Context<OutlinePanel>,
    ) {
        if !self.active {
            return;
        }

        self.cached_entries_update_pending
            .get_or_insert(CachedEntriesUpdate::Entries);
        if !self.cached_entries_update_task.is_ready() {
            return;
        }

        self.cached_entries_update_task = cx.spawn_in(window, async move |outline_panel, cx| {
            let mut debounce = debounce;
            loop {
                if let Some(debounce) = debounce {
                    cx.background_executor().timer(debounce).await;
                }
                let Some(new_cached_entries) = outline_panel
                    .update_in(cx, |outline_panel, window, cx| {
                        if outline_panel.cached_entries_update_pending.take()
                            == Some(CachedEntriesUpdate::Contents)
                        {
                            outline_panel.update_non_fs_items(window, cx);
                        }
                        let is_singleton = outline_panel.is_singleton_active(cx);
                        let query = outline_panel.query(cx);
                        outline_panel.generate_cached_entries(is_singleton, query, window, cx)
                    })
                    .ok()
                else {
                    return;
                };
                let (new_cached_entries, max_width_item_index) = new_cached_entries.await;
                outline_panel
                    .update_in(cx, |outline_panel, window, cx| {
                        outline_panel.cached_entries = new_cached_entries;
                        outline_panel.max_width_item_index = max_width_item_index;
                        if let SelectedEntry::Valid(selected, _) = &outline_panel.selected_entry
                            && let Some((index, cached)) =
                                outline_panel.cached_entry_for_selection(selected)
                        {
                            outline_panel.selected_entry =
                                SelectedEntry::Valid(cached.entry.clone(), index);
                        }
                        if (outline_panel.selected_entry.is_invalidated()
                            || matches!(outline_panel.selected_entry, SelectedEntry::None))
                            && let Some(new_selected_entry) =
                                outline_panel.active_editor().and_then(|active_editor| {
                                    outline_panel.location_for_editor_selection(
                                        &active_editor,
                                        window,
                                        cx,
                                    )
                                })
                        {
                            let awaiting_outlines = match &new_selected_entry {
                                PanelEntry::Outline(outline) => outline_panel
                                    .buffers
                                    .get(&outline.buffer_id())
                                    .is_some_and(BufferOutlines::should_fetch_outlines),
                                _ => false,
                            };
                            outline_panel.select_entry(new_selected_entry, false, window, cx);
                            if awaiting_outlines {
                                outline_panel.selected_entry.invalidate();
                            }
                        }

                        cx.notify();
                    })
                    .ok();
                if !outline_panel
                    .read_with(cx, |panel, _| panel.cached_entries_update_pending.is_some())
                    .unwrap_or(false)
                {
                    break;
                }
                debounce = Some(UPDATE_DEBOUNCE);
            }
        });
    }

    fn cached_entry_for_selection(&self, selected: &PanelEntry) -> Option<(usize, &CachedEntry)> {
        if let Some(entry) = self
            .cached_entries
            .iter()
            .enumerate()
            .find(|(_, cached)| &cached.entry == selected)
        {
            return Some(entry);
        }
        let (worktree_id, path, first_buffer_id) = match selected {
            PanelEntry::Fs(FsEntry::Directory(directory)) => (
                directory.worktree_id,
                &directory.entry.path,
                directory.first_buffer_id,
            ),
            PanelEntry::FoldedDirs(group) => (
                group.worktree_id,
                &group.entries.last()?.path,
                group.first_buffer_id,
            ),
            _ => return None,
        };
        let mut first_section = None;
        let mut current_section = None;
        let mut selected_section = None;
        for entry in &self.fs_entries {
            match entry {
                FsEntry::Directory(directory)
                    if directory.worktree_id == worktree_id && directory.entry.path == *path =>
                {
                    first_section.get_or_insert(directory.first_buffer_id);
                    current_section = Some(directory.first_buffer_id);
                }
                FsEntry::File(file) if file.buffer_id == first_buffer_id => {
                    if file.worktree_id == worktree_id
                        && file
                            .entry
                            .path
                            .parent()
                            .is_some_and(|parent| parent.starts_with(path))
                    {
                        selected_section = current_section;
                        break;
                    }
                }
                _ => {}
            }
        }
        let selected_section = selected_section.or(first_section)?;
        self.cached_entries
            .iter()
            .enumerate()
            .find(|(_, cached)| match &cached.entry {
                PanelEntry::Fs(FsEntry::Directory(directory)) => {
                    directory.worktree_id == worktree_id
                        && directory.first_buffer_id == selected_section
                        && directory.entry.path == *path
                }
                PanelEntry::FoldedDirs(group) => {
                    group.worktree_id == worktree_id
                        && group.first_buffer_id == selected_section
                        && group.entries.iter().any(|entry| entry.path == *path)
                }
                _ => false,
            })
    }

    fn generate_cached_entries(
        &self,
        is_singleton: bool,
        query: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<(Vec<CachedEntry>, Option<usize>)> {
        let Some(active_editor) = self.active_editor() else {
            return Task::ready((Vec::new(), None));
        };
        cx.spawn_in(window, async move |outline_panel, cx| {
            let mut generation_state = GenerationState::default();

            let Ok(()) = outline_panel.update(cx, |outline_panel, cx| {
                let auto_fold_dirs = OutlinePanelSettings::get_global(cx).auto_fold_dirs;
                let hide_symbols = !is_singleton && outline_panel.hide_symbols_active(cx);
                let mut folded_dirs_entry = None::<(usize, FoldedDirsEntry)>;
                let track_matches = query.is_some();

                #[derive(Debug)]
                struct ParentStats {
                    path: Arc<RelPath>,
                    folded: bool,
                    expanded: bool,
                    depth: usize,
                    worktree_id: WorktreeId,
                }

                let search_precomputed = if hide_symbols {
                    None
                } else if let ItemsDisplayMode::Search(search_state) = &outline_panel.mode {
                    let multi_buffer_snapshot =
                        active_editor.read(cx).buffer().read(cx).snapshot(cx);
                    let mut folded_buffers = HashSet::default();
                    let mut not_folded_buffers = HashSet::default();
                    let mut matches_by_buffer = HashMap::default();

                    for (match_range, search_data) in &search_state.matches {
                        let Some((start_anchor, _)) =
                            multi_buffer_snapshot.anchor_to_buffer_anchor(match_range.start)
                        else {
                            continue;
                        };
                        let start_buffer_id = start_anchor.buffer_id;
                        let end_buffer_id = multi_buffer_snapshot
                            .anchor_to_buffer_anchor(match_range.end)
                            .map(|(anchor, _)| anchor.buffer_id);

                        let mut any_folded = false;
                        for buffer_id in
                            [Some(start_buffer_id), end_buffer_id].into_iter().flatten()
                        {
                            if folded_buffers.contains(&buffer_id) {
                                any_folded = true;
                            } else if !not_folded_buffers.contains(&buffer_id) {
                                if active_editor.read(cx).is_buffer_folded(buffer_id, cx) {
                                    folded_buffers.insert(buffer_id);
                                    any_folded = true;
                                } else {
                                    not_folded_buffers.insert(buffer_id);
                                }
                            }
                        }
                        if any_folded {
                            continue;
                        }

                        matches_by_buffer
                            .entry(start_buffer_id)
                            .or_insert_with(Vec::new)
                            .push((match_range.clone(), Arc::clone(search_data)));
                    }

                    Some(SearchPrecomputed {
                        multi_buffer_snapshot,
                        matches_by_buffer,
                        folded_buffers,
                    })
                } else {
                    None
                };

                let mut parent_dirs = Vec::<ParentStats>::new();
                for entry in outline_panel.fs_entries.clone() {
                    let is_expanded = outline_panel.is_expanded(&entry);
                    let (depth, should_add) = match &entry {
                        FsEntry::Directory(directory_entry) => {
                            let mut should_add = true;
                            let is_root = directory_entry.entry.path.is_empty();
                            let folded = auto_fold_dirs
                                && !is_root
                                && outline_panel
                                    .unfolded_dirs
                                    .get(&directory_entry.worktree_id)
                                    .is_none_or(|unfolded_dirs| {
                                        !unfolded_dirs.contains(&directory_entry.entry.path)
                                    });
                            let fs_depth = directory_entry.entry.path.components().count();
                            while let Some(parent) = parent_dirs.last() {
                                if !is_root
                                    && parent.worktree_id == directory_entry.worktree_id
                                    && directory_entry.entry.path.starts_with(&parent.path)
                                {
                                    break;
                                }
                                parent_dirs.pop();
                            }
                            let auto_fold = match parent_dirs.last() {
                                Some(parent) => {
                                    parent.folded
                                        && Some(parent.path.as_ref())
                                            == directory_entry.entry.path.parent()
                                        && outline_panel
                                            .fs_children_count
                                            .get(&directory_entry.worktree_id)
                                            .and_then(|entries| {
                                                entries.get(&directory_entry.entry.path)
                                            })
                                            .copied()
                                            .unwrap_or_default()
                                            .may_be_fold_part()
                                }
                                None => false,
                            };
                            let folded = folded || auto_fold;
                            let (depth, parent_expanded, parent_folded) = match parent_dirs.last() {
                                Some(parent) => {
                                    let parent_folded = parent.folded;
                                    let parent_expanded = parent.expanded;
                                    let new_depth = if parent_folded {
                                        parent.depth
                                    } else {
                                        parent.depth + 1
                                    };
                                    parent_dirs.push(ParentStats {
                                        worktree_id: directory_entry.worktree_id,
                                        path: directory_entry.entry.path.clone(),
                                        folded,
                                        expanded: parent_expanded && is_expanded,
                                        depth: new_depth,
                                    });
                                    (new_depth, parent_expanded, parent_folded)
                                }
                                None => {
                                    parent_dirs.push(ParentStats {
                                        worktree_id: directory_entry.worktree_id,
                                        path: directory_entry.entry.path.clone(),
                                        folded,
                                        expanded: is_expanded,
                                        depth: fs_depth,
                                    });
                                    (fs_depth, true, false)
                                }
                            };

                            if let Some((folded_depth, mut folded_dirs)) = folded_dirs_entry.take()
                            {
                                if folded
                                    && directory_entry.worktree_id == folded_dirs.worktree_id
                                    && directory_entry.entry.path.parent()
                                        == folded_dirs
                                            .entries
                                            .last()
                                            .map(|entry| entry.path.as_ref())
                                {
                                    folded_dirs.entries.push(directory_entry.entry.clone());
                                    folded_dirs_entry = Some((folded_depth, folded_dirs))
                                } else {
                                    if !is_singleton {
                                        let start_of_collapsed_dir_sequence = !parent_expanded
                                            && parent_dirs
                                                .iter()
                                                .rev()
                                                .nth(folded_dirs.entries.len() + 1)
                                                .is_none_or(|parent| parent.expanded);
                                        if start_of_collapsed_dir_sequence
                                            || parent_expanded
                                            || query.is_some()
                                        {
                                            if parent_folded {
                                                folded_dirs
                                                    .entries
                                                    .push(directory_entry.entry.clone());
                                                should_add = false;
                                            }
                                            let new_folded_dirs =
                                                PanelEntry::FoldedDirs(folded_dirs.clone());
                                            outline_panel.push_entry(
                                                &mut generation_state,
                                                track_matches,
                                                new_folded_dirs,
                                                folded_depth,
                                                cx,
                                            );
                                        }
                                    }

                                    folded_dirs_entry = if parent_folded {
                                        None
                                    } else {
                                        Some((
                                            depth,
                                            FoldedDirsEntry {
                                                worktree_id: directory_entry.worktree_id,
                                                first_buffer_id: directory_entry.first_buffer_id,
                                                entries: vec![directory_entry.entry.clone()],
                                            },
                                        ))
                                    };
                                }
                            } else if folded {
                                folded_dirs_entry = Some((
                                    depth,
                                    FoldedDirsEntry {
                                        worktree_id: directory_entry.worktree_id,
                                        first_buffer_id: directory_entry.first_buffer_id,
                                        entries: vec![directory_entry.entry.clone()],
                                    },
                                ));
                            }

                            let should_add =
                                should_add && parent_expanded && folded_dirs_entry.is_none();
                            (depth, should_add)
                        }
                        FsEntry::ExternalFile(..) => {
                            if let Some((folded_depth, folded_dir)) = folded_dirs_entry.take() {
                                let folded_paths = folded_dir
                                    .entries
                                    .iter()
                                    .map(|entry| &entry.path)
                                    .collect::<HashSet<_>>();
                                let parent_expanded = parent_dirs
                                    .iter()
                                    .rev()
                                    .find(|parent| !folded_paths.contains(&parent.path))
                                    .is_none_or(|parent| parent.expanded);
                                if !is_singleton && (parent_expanded || query.is_some()) {
                                    outline_panel.push_entry(
                                        &mut generation_state,
                                        track_matches,
                                        PanelEntry::FoldedDirs(folded_dir),
                                        folded_depth,
                                        cx,
                                    );
                                }
                            }
                            parent_dirs.clear();
                            (0, true)
                        }
                        FsEntry::File(file) => {
                            if let Some((folded_depth, folded_dirs)) = folded_dirs_entry.take() {
                                let folded_paths = folded_dirs
                                    .entries
                                    .iter()
                                    .map(|entry| &entry.path)
                                    .collect::<HashSet<_>>();
                                let parent_expanded = parent_dirs
                                    .iter()
                                    .rev()
                                    .find(|parent| !folded_paths.contains(&parent.path))
                                    .is_none_or(|parent| parent.expanded);
                                if !is_singleton && (parent_expanded || query.is_some()) {
                                    outline_panel.push_entry(
                                        &mut generation_state,
                                        track_matches,
                                        PanelEntry::FoldedDirs(folded_dirs),
                                        folded_depth,
                                        cx,
                                    );
                                }
                            }

                            let fs_depth = file.entry.path.components().count();
                            while let Some(parent) = parent_dirs.last() {
                                if parent.worktree_id == file.worktree_id
                                    && file
                                        .entry
                                        .path
                                        .parent()
                                        .is_some_and(|path| path.starts_with(&parent.path))
                                {
                                    break;
                                }
                                parent_dirs.pop();
                            }
                            match parent_dirs.last() {
                                Some(parent) => {
                                    let new_depth = parent.depth + 1;
                                    (new_depth, parent.expanded)
                                }
                                None => (fs_depth, true),
                            }
                        }
                    };

                    if !is_singleton
                        && (should_add || (query.is_some() && folded_dirs_entry.is_none()))
                    {
                        outline_panel.push_entry(
                            &mut generation_state,
                            track_matches,
                            PanelEntry::Fs(entry.clone()),
                            depth,
                            cx,
                        );
                    }

                    if !hide_symbols {
                        match outline_panel.mode {
                            ItemsDisplayMode::Search(_) => {
                                if (is_singleton || query.is_some() || (should_add && is_expanded))
                                    && let Some(search) = &search_precomputed
                                {
                                    outline_panel.add_search_entries(
                                        &mut generation_state,
                                        search,
                                        &entry,
                                        depth,
                                        query.is_some(),
                                        is_singleton,
                                        cx,
                                    );
                                }
                            }
                            ItemsDisplayMode::Outline => {
                                let excerpts_to_consider = if is_singleton
                                    || query.is_some()
                                    || (should_add && is_expanded)
                                {
                                    match &entry {
                                        FsEntry::File(FsEntryFile {
                                            buffer_id,
                                            excerpts,
                                            ..
                                        })
                                        | FsEntry::ExternalFile(FsEntryExternalFile {
                                            buffer_id,
                                            excerpts,
                                            ..
                                        }) => Some((*buffer_id, excerpts)),
                                        _ => None,
                                    }
                                } else {
                                    None
                                };
                                if let Some((buffer_id, _entry_excerpts)) = excerpts_to_consider
                                    && !active_editor.read(cx).is_buffer_folded(buffer_id, cx)
                                {
                                    outline_panel.add_buffer_entries(
                                        &mut generation_state,
                                        buffer_id,
                                        depth,
                                        track_matches,
                                        is_singleton,
                                        query.as_deref(),
                                        cx,
                                    );
                                }
                            }
                        }
                    }

                    if is_singleton
                        && matches!(entry, FsEntry::File(..) | FsEntry::ExternalFile(..))
                        && !generation_state.entries.iter().any(|item| {
                            matches!(item.entry, PanelEntry::Outline(..) | PanelEntry::Search(_))
                        })
                    {
                        outline_panel.push_entry(
                            &mut generation_state,
                            track_matches,
                            PanelEntry::Fs(entry.clone()),
                            0,
                            cx,
                        );
                    }
                }

                if let Some((folded_depth, folded_dirs)) = folded_dirs_entry.take() {
                    let folded_paths = folded_dirs
                        .entries
                        .iter()
                        .map(|entry| &entry.path)
                        .collect::<HashSet<_>>();
                    let parent_expanded = parent_dirs
                        .iter()
                        .rev()
                        .find(|parent| !folded_paths.contains(&parent.path))
                        .is_none_or(|parent| parent.expanded);
                    if parent_expanded || query.is_some() {
                        outline_panel.push_entry(
                            &mut generation_state,
                            track_matches,
                            PanelEntry::FoldedDirs(folded_dirs),
                            folded_depth,
                            cx,
                        );
                    }
                }
            }) else {
                return (Vec::new(), None);
            };

            let Some(query) = query else {
                return (
                    generation_state.entries,
                    generation_state
                        .max_width_estimate_and_index
                        .map(|(_, index)| index),
                );
            };

            let mut matched_ids = match_strings(
                &generation_state.match_candidates,
                &query,
                true,
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
            generation_state.entries.retain_mut(|cached_entry| {
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

            (
                generation_state.entries,
                generation_state
                    .max_width_estimate_and_index
                    .map(|(_, index)| index),
            )
        })
    }

    fn push_entry(
        &self,
        state: &mut GenerationState,
        track_matches: bool,
        entry: PanelEntry,
        depth: usize,
        cx: &mut App,
    ) {
        let entry = if let PanelEntry::FoldedDirs(folded_dirs_entry) = &entry {
            match folded_dirs_entry.entries.len() {
                0 => {
                    debug_panic!("Empty folded dirs receiver");
                    return;
                }
                1 => PanelEntry::Fs(FsEntry::Directory(FsEntryDirectory {
                    worktree_id: folded_dirs_entry.worktree_id,
                    first_buffer_id: folded_dirs_entry.first_buffer_id,
                    entry: folded_dirs_entry.entries[0].clone(),
                })),
                _ => entry,
            }
        } else {
            entry
        };

        if track_matches {
            let id = state.entries.len();
            match &entry {
                PanelEntry::Fs(fs_entry) => {
                    let candidate_label = match fs_entry {
                        FsEntry::Directory(directory) => {
                            directory.entry.path.file_name().map(str::to_string)
                        }
                        _ => self.fs_entry_label(fs_entry, cx),
                    };
                    if let Some(file_name) = candidate_label {
                        state
                            .match_candidates
                            .push(StringMatchCandidate::new(id, &file_name));
                    }
                }
                PanelEntry::FoldedDirs(folded_dir_entry) => {
                    let dir_names = self.dir_names_string(
                        &folded_dir_entry.entries,
                        folded_dir_entry.worktree_id,
                        cx,
                    );
                    {
                        state
                            .match_candidates
                            .push(StringMatchCandidate::new(id, &dir_names));
                    }
                }
                PanelEntry::Outline(OutlineEntry::Outline(outline_entry)) => state
                    .match_candidates
                    .push(StringMatchCandidate::new(id, &outline_entry.text)),
                PanelEntry::Outline(OutlineEntry::Excerpt(_)) => {}
                PanelEntry::Search(new_search_entry) => {
                    if let Some(search_data) = new_search_entry.render_data.get() {
                        state
                            .match_candidates
                            .push(StringMatchCandidate::new(id, &search_data.context_text));
                    }
                }
            }
        }

        let width_estimate = self.width_estimate(depth, &entry, cx);
        if Some(width_estimate)
            > state
                .max_width_estimate_and_index
                .map(|(estimate, _)| estimate)
        {
            state.max_width_estimate_and_index = Some((width_estimate, state.entries.len()));
        }
        state.entries.push(CachedEntry {
            depth,
            entry,
            string_match: None,
        });
    }

    fn dir_names_string(
        &self,
        entries: &[FsEntryPath],
        worktree_id: WorktreeId,
        cx: &App,
    ) -> String {
        let dir_names_segment = entries
            .iter()
            .map(|entry| self.entry_name(&worktree_id, entry, cx))
            .collect::<PathBuf>();
        dir_names_segment.to_string_lossy().into_owned()
    }

    fn query(&self, cx: &App) -> Option<String> {
        let query = self.filter_editor.read(cx).text(cx);
        if query.trim().is_empty() {
            None
        } else {
            Some(query)
        }
    }

    fn is_expanded(&self, entry: &FsEntry) -> bool {
        let entry_to_check = match entry {
            FsEntry::ExternalFile(FsEntryExternalFile { buffer_id, .. }) => {
                CollapsedEntry::ExternalFile(*buffer_id)
            }
            FsEntry::File(FsEntryFile {
                worktree_id,
                buffer_id,
                ..
            }) => CollapsedEntry::File(*worktree_id, *buffer_id),
            FsEntry::Directory(FsEntryDirectory {
                worktree_id, entry, ..
            }) => CollapsedEntry::Dir(*worktree_id, entry.path.clone()),
        };
        !self.collapsed_entries.contains(&entry_to_check)
    }

    fn update_non_fs_items(&mut self, window: &mut Window, cx: &mut Context<OutlinePanel>) {
        if !self.active {
            return;
        }

        if self.cached_entries_update_pending == Some(CachedEntriesUpdate::Contents) {
            self.cached_entries_update_pending = Some(CachedEntriesUpdate::Entries);
        }
        if self.update_search_matches(window, cx) {
            self.selected_entry.invalidate();
        }
        self.fetch_outdated_outlines(None, window, cx);
    }

    fn update_search_matches(
        &mut self,
        window: &mut Window,
        cx: &mut Context<OutlinePanel>,
    ) -> bool {
        if !self.active {
            return false;
        }

        let project_search = self
            .active_item()
            .and_then(|item| item.downcast::<ProjectSearchView>());

        let buffer_search_matches = self
            .active_editor()
            .map(|active_editor| {
                active_editor.update(cx, |editor, cx| editor.get_matches(window, cx).0)
            })
            .unwrap_or_default();
        let project_search_matches = if buffer_search_matches.is_empty() {
            project_search
                .as_ref()
                .map(|project_search| project_search.read(cx).get_matches(cx))
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        let mut update_cached_entries = false;
        if buffer_search_matches.is_empty() && project_search_matches.is_empty() {
            if matches!(self.mode, ItemsDisplayMode::Search(_)) {
                self.mode = ItemsDisplayMode::Outline;
                update_cached_entries = true;
            }
        } else {
            let (kind, new_search_matches, new_search_query) = if buffer_search_matches.is_empty() {
                (
                    SearchKind::Project,
                    project_search_matches,
                    project_search
                        .map(|project_search| project_search.read(cx).search_query_text(cx))
                        .unwrap_or_default(),
                )
            } else {
                let buffer_search = self
                    .active_item()
                    .as_deref()
                    .and_then(|active_item| {
                        self.workspace
                            .upgrade()
                            .and_then(|workspace| workspace.read(cx).pane_for(active_item))
                    })
                    .and_then(|pane| {
                        pane.read(cx)
                            .toolbar()
                            .read(cx)
                            .item_of_type::<BufferSearchBar>()
                    });
                (
                    SearchKind::Buffer,
                    buffer_search_matches,
                    buffer_search
                        .map(|buffer_search| buffer_search.read(cx).query(cx))
                        .unwrap_or_default(),
                )
            };

            let changed = match &self.mode {
                ItemsDisplayMode::Search(current) => {
                    current.query != new_search_query
                        || current.kind != kind
                        || current.matches.len() != new_search_matches.len()
                        || current
                            .matches
                            .iter()
                            .zip(&new_search_matches)
                            .any(|((existing, _), incoming)| existing != incoming)
                }
                ItemsDisplayMode::Outline => true,
            };
            if changed {
                let previous_matches = match &mut self.mode {
                    ItemsDisplayMode::Search(current) if current.kind == kind => {
                        current.matches.drain(..).collect()
                    }
                    _ => HashMap::default(),
                };
                self.mode = ItemsDisplayMode::Search(SearchState::new(
                    kind,
                    new_search_query,
                    previous_matches,
                    new_search_matches,
                    cx.theme().syntax().clone(),
                    window,
                    cx,
                ));
                update_cached_entries = true;
            }
        }
        update_cached_entries
    }

    fn add_buffer_entries(
        &mut self,
        state: &mut GenerationState,
        buffer_id: BufferId,
        parent_depth: usize,
        track_matches: bool,
        is_singleton: bool,
        query: Option<&str>,
        cx: &mut Context<Self>,
    ) {
        let Some(buffer) = self.buffers.get(&buffer_id) else {
            return;
        };
        let buffer_snapshot = self.buffer_snapshot_for_id(buffer_id, cx);
        let snapshot = buffer_snapshot.as_ref();
        let mut excerpts = buffer
            .excerpts
            .iter()
            .map(|excerpt| ExcerptOutlines {
                excerpt,
                ranked_range: 0..0,
                expanded: is_singleton
                    || query.is_some()
                    || !self
                        .collapsed_entries
                        .contains(&CollapsedEntry::Excerpt(excerpt.clone())),
                visible: Vec::new(),
                children_to_cache: Vec::new(),
            })
            .collect::<Vec<_>>();
        let expanded_excerpts = excerpts.iter().filter(|excerpt| excerpt.expanded).count();
        if expanded_excerpts > 0 {
            let children = self.outline_children_cache.entry(buffer_id).or_default();
            let outlines = buffer.iter_outlines().collect::<Vec<_>>();
            if let Some(snapshot) = snapshot
                && expanded_excerpts > 1
                && !outlines.is_empty()
            {
                let mut coordinates = Vec::new();
                for outline in &outlines {
                    coordinates.extend([outline.range.start, outline.range.end]);
                }
                for excerpt in excerpts.iter().filter(|excerpt| excerpt.expanded) {
                    coordinates
                        .extend([excerpt.excerpt.context.start, excerpt.excerpt.context.end]);
                }
                coordinates.sort_unstable_by(|left, right| left.cmp(right, snapshot));
                coordinates.dedup_by(|left, right| left.cmp(right, snapshot).is_eq());
                let rank = |anchor: Anchor| {
                    coordinates
                        .partition_point(|coordinate| coordinate.cmp(&anchor, snapshot).is_lt())
                };
                let outlines = outlines
                    .iter()
                    .map(|outline| IndexedOutline {
                        outline,
                        ranked_range: rank(outline.range.start)..rank(outline.range.end),
                        collapsed: self
                            .collapsed_entries
                            .contains(&CollapsedEntry::Outline(outline.range.clone())),
                    })
                    .collect::<Vec<_>>();
                for excerpt in excerpts.iter_mut().filter(|excerpt| excerpt.expanded) {
                    excerpt.ranked_range =
                        rank(excerpt.excerpt.context.start)..rank(excerpt.excerpt.context.end);
                }
                BufferOutlines::assign_outline_cache_owners(&outlines, &mut excerpts);
                if BufferOutlines::outlines_are_well_formed(&outlines) {
                    BufferOutlines::collect_nested_outlines(&outlines, &mut excerpts, children);
                } else {
                    BufferOutlines::collect_overlapping_outlines(
                        &outlines,
                        &mut excerpts,
                        children,
                    );
                }
            } else {
                for excerpt in excerpts.iter_mut().filter(|excerpt| excerpt.expanded) {
                    let mut matches = outlines
                        .iter()
                        .copied()
                        .filter(|outline| {
                            snapshot.is_none_or(|snapshot| {
                                outline
                                    .range
                                    .start
                                    .cmp(&excerpt.excerpt.context.end, snapshot)
                                    .is_le()
                                    && outline
                                        .range
                                        .end
                                        .cmp(&excerpt.excerpt.context.start, snapshot)
                                        .is_ge()
                            })
                        })
                        .peekable();
                    let mut collapsed_ancestor: Option<&Outline> = None;
                    while let Some(outline) = matches.next() {
                        children.insert(
                            (outline.range.clone(), outline.depth),
                            matches
                                .peek()
                                .is_some_and(|next| next.depth > outline.depth),
                        );
                        if let Some(ancestor) = collapsed_ancestor
                            && let Some(snapshot) = snapshot
                            && outline.depth > ancestor.depth
                            && outline
                                .range
                                .start
                                .cmp(&ancestor.range.start, snapshot)
                                .is_ge()
                            && outline
                                .range
                                .start
                                .cmp(&ancestor.range.end, snapshot)
                                .is_lt()
                        {
                            continue;
                        }
                        excerpt.visible.push(outline);
                        collapsed_ancestor = self
                            .collapsed_entries
                            .contains(&CollapsedEntry::Outline(outline.range.clone()))
                            .then_some(outline);
                    }
                }
            }
        }
        for excerpt in excerpts {
            let excerpt_depth = parent_depth + 1;
            self.push_entry(
                state,
                track_matches,
                PanelEntry::Outline(OutlineEntry::Excerpt(excerpt.excerpt.clone())),
                excerpt_depth,
                cx,
            );
            let outline_base_depth = if is_singleton {
                state.clear();
                0
            } else {
                excerpt_depth + 1
            };
            for outline in excerpt.visible {
                self.push_entry(
                    state,
                    track_matches,
                    PanelEntry::Outline(OutlineEntry::Outline(outline.clone())),
                    outline_base_depth + outline.depth,
                    cx,
                );
            }
        }
    }

    fn add_search_entries(
        &mut self,
        state: &mut GenerationState,
        search: &SearchPrecomputed,
        parent_entry: &FsEntry,
        parent_depth: usize,
        track_matches: bool,
        is_singleton: bool,
        cx: &mut Context<Self>,
    ) {
        let ItemsDisplayMode::Search(search_state) = &self.mode else {
            return;
        };
        let kind = search_state.kind;

        let (buffer_id, excerpts) = match parent_entry {
            FsEntry::Directory(_) => return,
            FsEntry::ExternalFile(external) => (external.buffer_id, &external.excerpts),
            FsEntry::File(file) => (file.buffer_id, &file.excerpts),
        };

        if search.folded_buffers.contains(&buffer_id) {
            return;
        }
        let Some(buffer_matches) = search.matches_by_buffer.get(&buffer_id) else {
            return;
        };

        let mut excerpt_ranges = excerpts
            .iter()
            .filter_map(|excerpt| {
                let start = search
                    .multi_buffer_snapshot
                    .anchor_in_buffer(excerpt.context.start)?;
                let end = search
                    .multi_buffer_snapshot
                    .anchor_in_buffer(excerpt.context.end)?;
                Some(start..end)
            })
            .collect::<Vec<_>>();

        let snapshot = &search.multi_buffer_snapshot;
        excerpt_ranges.sort_unstable_by(|left, right| left.start.cmp(&right.start, snapshot));
        let mut maximum_end = None;
        for range in &mut excerpt_ranges {
            if let Some(end) = maximum_end
                && range.end.cmp(&end, snapshot).is_lt()
            {
                range.end = end;
            }
            maximum_end = Some(range.end);
        }
        let depth = if is_singleton { 0 } else { parent_depth + 1 };
        for (match_range, search_data) in buffer_matches.iter().filter(|(match_range, _)| {
            let index = excerpt_ranges
                .partition_point(|excerpt| excerpt.start.cmp(&match_range.end, snapshot).is_le());
            index
                .checked_sub(1)
                .and_then(|index| excerpt_ranges.get(index))
                .is_some_and(|excerpt| excerpt.end.cmp(&match_range.start, snapshot).is_ge())
        }) {
            self.push_entry(
                state,
                track_matches,
                PanelEntry::Search(SearchEntry {
                    match_range: match_range.clone(),
                    kind,
                    render_data: Arc::clone(search_data),
                }),
                depth,
                cx,
            );
        }
    }

    fn active_editor(&self) -> Option<Entity<Editor>> {
        self.active_item.as_ref()?.active_editor.upgrade()
    }

    fn active_item(&self) -> Option<Box<dyn ItemHandle>> {
        self.active_item.as_ref()?.item_handle.upgrade()
    }

    fn should_replace_active_item(&self, new_active_item: &dyn ItemHandle) -> bool {
        self.active_item().is_none_or(|active_item| {
            !self.pinned && active_item.item_id() != new_active_item.item_id()
        })
    }

    pub fn toggle_active_editor_pin(
        &mut self,
        _: &ToggleActiveEditorPin,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.pinned = !self.pinned;
        if !self.pinned
            && let Some((active_item, active_editor)) = self
                .workspace
                .upgrade()
                .and_then(|workspace| workspace_active_editor(workspace.read(cx), cx))
            && self.should_replace_active_item(active_item.as_ref())
        {
            self.replace_active_editor(active_item, active_editor, window, cx);
        }

        cx.notify();
    }

    pub fn toggle_symbols(
        &mut self,
        _: &ToggleSymbols,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.multi_buffer_active(cx) {
            return;
        }
        self.hide_symbols_override = Some(!self.hide_symbols_active(cx));
        self.update_non_fs_items(window, cx);
        self.remap_selection_for_hidden_symbols(window, cx);
        self.update_cached_entries(None, window, cx);
        cx.notify();
    }

    fn selected_entry(&self) -> Option<&PanelEntry> {
        match &self.selected_entry {
            SelectedEntry::Invalidated(entry) => entry.as_ref(),
            SelectedEntry::Valid(entry, _) => Some(entry),
            SelectedEntry::None => None,
        }
    }

    fn select_entry(
        &mut self,
        entry: PanelEntry,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if focus {
            self.focus_handle.focus(window, cx);
        }
        let ix = self
            .cached_entries
            .iter()
            .enumerate()
            .find(|(_, cached_entry)| &cached_entry.entry == &entry)
            .map(|(i, _)| i)
            .unwrap_or_default();

        self.selected_entry = SelectedEntry::Valid(entry, ix);

        self.autoscroll(cx);
        cx.notify();
    }

    fn width_estimate(&self, depth: usize, entry: &PanelEntry, cx: &App) -> u64 {
        let item_text_chars = match entry {
            PanelEntry::Fs(fs_entry) => self
                .fs_entry_label(fs_entry, cx)
                .map(|label| label.len())
                .unwrap_or_default(),
            PanelEntry::FoldedDirs(folded_dirs) => {
                folded_dirs
                    .entries
                    .iter()
                    .map(|dir| {
                        dir.path
                            .file_name()
                            .map(|name| name.len())
                            .unwrap_or_default()
                    })
                    .sum::<usize>()
                    + folded_dirs.entries.len().saturating_sub(1) * "/".len()
            }
            PanelEntry::Outline(OutlineEntry::Excerpt(excerpt)) => self
                .excerpt_label(&excerpt, cx)
                .map(|label| label.len())
                .unwrap_or_default(),
            PanelEntry::Outline(OutlineEntry::Outline(entry)) => entry.text.len(),
            PanelEntry::Search(search) => search
                .render_data
                .get()
                .map(|data| data.context_text.len())
                .unwrap_or_default(),
        };

        (item_text_chars + depth) as u64
    }

    fn render_main_contents(
        &mut self,
        query: Option<String>,
        show_indent_guides: bool,
        indent_size: f32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let contents = if self.cached_entries.is_empty() {
            let header = if query.is_some() {
                "No matches for query"
            } else {
                "No outlines available"
            };

            v_flex()
                .id("empty-outline-state")
                .gap_0p5()
                .flex_1()
                .justify_center()
                .size_full()
                .child(h_flex().justify_center().child(Label::new(header)))
                .when_some(query, |panel, query| {
                    panel.child(
                        h_flex()
                            .px_0p5()
                            .justify_center()
                            .bg(cx.theme().colors().element_selected.opacity(0.2))
                            .child(Label::new(query)),
                    )
                })
                .child(
                    h_flex()
                        .gap_1()
                        .justify_center()
                        .child(Label::new("Toggle Panel With").color(Color::Muted))
                        .child({
                            let key_binding = match self.position(window, cx) {
                                DockPosition::Left => {
                                    KeyBinding::for_action(&workspace::ToggleLeftDock, cx)
                                        .into_any_element()
                                }
                                DockPosition::Bottom => {
                                    KeyBinding::for_action(&workspace::ToggleBottomDock, cx)
                                        .into_any_element()
                                }
                                DockPosition::Right => {
                                    KeyBinding::for_action(&workspace::ToggleRightDock, cx)
                                        .into_any_element()
                                }
                            };

                            key_binding
                        }),
                )
        } else {
            let list_contents = {
                let items_len = self.cached_entries.len();
                let multi_buffer_snapshot = match self.mode {
                    ItemsDisplayMode::Search(_) => self
                        .active_editor()
                        .map(|editor| editor.read(cx).buffer().read(cx).snapshot(cx)),
                    ItemsDisplayMode::Outline => None,
                };
                uniform_list(
                    "entries",
                    items_len,
                    cx.processor(move |outline_panel, range: Range<usize>, window, cx| {
                        outline_panel.rendered_entries_len = range.end - range.start;
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
                                    window,
                                    cx,
                                )),
                                PanelEntry::FoldedDirs(folded_dirs_entry) => {
                                    Some(outline_panel.render_folded_dirs(
                                        &folded_dirs_entry,
                                        cached_entry.depth,
                                        cached_entry.string_match.as_ref(),
                                        window,
                                        cx,
                                    ))
                                }
                                PanelEntry::Outline(OutlineEntry::Excerpt(excerpt)) => {
                                    outline_panel.render_excerpt(
                                        &excerpt,
                                        cached_entry.depth,
                                        window,
                                        cx,
                                    )
                                }
                                PanelEntry::Outline(OutlineEntry::Outline(entry)) => {
                                    Some(outline_panel.render_outline(
                                        &entry,
                                        cached_entry.depth,
                                        cached_entry.string_match.as_ref(),
                                        window,
                                        cx,
                                    ))
                                }
                                PanelEntry::Search(SearchEntry {
                                    match_range,
                                    render_data,
                                    kind,
                                    ..
                                }) => outline_panel.render_search_match(
                                    multi_buffer_snapshot.as_ref(),
                                    &match_range,
                                    &render_data,
                                    kind,
                                    cached_entry.depth,
                                    cached_entry.string_match.as_ref(),
                                    window,
                                    cx,
                                ),
                            })
                            .collect()
                    }),
                )
                .with_sizing_behavior(ListSizingBehavior::Infer)
                .with_horizontal_sizing_behavior(ListHorizontalSizingBehavior::Unconstrained)
                .with_width_from_item(self.max_width_item_index)
                .track_scroll(&self.scroll_handle)
                .when(show_indent_guides, |list| {
                    list.with_decoration(
                        ui::indent_guides(px(indent_size), IndentGuideColors::panel(cx))
                            .with_compute_indents_fn(cx.entity(), |outline_panel, range, _, _| {
                                let entries = outline_panel.cached_entries.get(range);
                                if let Some(entries) = entries {
                                    entries.iter().map(|item| item.depth).collect()
                                } else {
                                    smallvec::SmallVec::new()
                                }
                            })
                            .with_render_fn(cx.entity(), move |outline_panel, params, _, _| {
                                const LEFT_OFFSET: Pixels = ui::LIST_ITEM_INDENT_GUIDE_LEFT_OFFSET;

                                let indent_size = params.indent_size;
                                let item_height = params.item_height;
                                let active_indent_guide_ix = find_active_indent_guide_ix(
                                    outline_panel,
                                    &params.indent_guides,
                                );

                                params
                                    .indent_guides
                                    .into_iter()
                                    .enumerate()
                                    .map(|(ix, layout)| {
                                        let bounds = Bounds::new(
                                            point(
                                                layout.offset.x * indent_size + LEFT_OFFSET,
                                                layout.offset.y * item_height,
                                            ),
                                            size(px(1.), layout.length * item_height),
                                        );
                                        ui::RenderedIndentGuide {
                                            bounds,
                                            layout,
                                            is_active: active_indent_guide_ix == Some(ix),
                                            hitbox: None,
                                        }
                                    })
                                    .collect()
                            }),
                    )
                })
            };

            v_flex()
                .flex_shrink_1()
                .size_full()
                .child(list_contents.size_full().flex_shrink_1())
                .custom_scrollbars(
                    Scrollbars::for_settings::<OutlinePanelSettingsScrollbarProxy>()
                        .tracked_scroll_handle(&self.scroll_handle.clone())
                        .with_track_along(
                            ScrollAxes::Horizontal,
                            cx.theme().colors().panel_background,
                        )
                        .tracked_entity(cx.entity_id()),
                    window,
                    cx,
                )
        }
        .children(self.context_menu.as_ref().map(|(menu, position, _)| {
            deferred(
                anchored()
                    .position(*position)
                    .anchor(gpui::Anchor::TopLeft)
                    .child(menu.clone()),
            )
            .with_priority(1)
        }));

        v_flex().w_full().flex_1().overflow_hidden().child(contents)
    }

    fn render_filter_footer(&mut self, pinned: bool, cx: &mut Context<Self>) -> Div {
        let (pin_button_id, icon, icon_tooltip) = if pinned {
            ("unpin_button", IconName::Unpin, "Unpin Outline")
        } else {
            ("pin_button", IconName::Pin, "Pin Active Outline")
        };

        let has_query = self.query(cx).is_some();
        let show_symbols_toggle = self.multi_buffer_active(cx);
        let hide_symbols = self.hide_symbols_active(cx);
        let (hide_symbols_icon, hide_symbols_tooltip) = if hide_symbols {
            (IconName::FileCodeOff, "Show Symbols")
        } else {
            (IconName::FileCode, "Hide Symbols")
        };

        h_flex()
            .p_2()
            .h(Tab::container_height(cx))
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .w_full()
                    .gap_1p5()
                    .child(
                        Icon::new(IconName::MagnifyingGlass)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(self.filter_editor.clone()),
            )
            .child(
                h_flex()
                    .when(has_query, |this| {
                        this.child(
                            IconButton::new("clear_filter", IconName::Close)
                                .shape(IconButtonShape::Square)
                                .tooltip(Tooltip::text("Clear Filter"))
                                .on_click(cx.listener(|outline_panel, _, window, cx| {
                                    outline_panel.filter_editor.update(cx, |editor, cx| {
                                        editor.set_text("", window, cx);
                                    });
                                    cx.notify();
                                })),
                        )
                    })
                    .when(show_symbols_toggle, |this| {
                        this.child(
                            IconButton::new("toggle_symbols", hide_symbols_icon)
                                .shape(IconButtonShape::Square)
                                .tooltip(Tooltip::for_action_title_in(
                                    hide_symbols_tooltip,
                                    &ToggleSymbols,
                                    &self.focus_handle,
                                ))
                                .on_click(cx.listener(|outline_panel, _, window, cx| {
                                    outline_panel.toggle_symbols(&ToggleSymbols, window, cx);
                                })),
                        )
                    })
                    .child(
                        IconButton::new(pin_button_id, icon)
                            .tooltip(Tooltip::for_action_title_in(
                                icon_tooltip,
                                &ToggleActiveEditorPin,
                                &self.focus_handle,
                            ))
                            .shape(IconButtonShape::Square)
                            .on_click(cx.listener(|outline_panel, _, window, cx| {
                                outline_panel.toggle_active_editor_pin(
                                    &ToggleActiveEditorPin,
                                    window,
                                    cx,
                                );
                            })),
                    ),
            )
    }

    fn buffers_inside_directory(
        &self,
        worktree_id: WorktreeId,
        directory: &FsEntryPath,
    ) -> HashSet<BufferId> {
        self.fs_entries
            .iter()
            .filter_map(|entry| match entry {
                FsEntry::File(file)
                    if file.worktree_id == worktree_id
                        && file.entry.path != directory.path
                        && file.entry.path.starts_with(&directory.path) =>
                {
                    Some(file.buffer_id)
                }
                _ => None,
            })
            .collect()
    }
}

fn workspace_active_editor(
    workspace: &Workspace,
    cx: &App,
) -> Option<(Box<dyn ItemHandle>, Entity<Editor>)> {
    let active_item = workspace.active_item(cx)?;
    let active_editor = active_item
        .act_as::<Editor>(cx)
        .filter(|editor| editor.read(cx).mode().is_full())?;
    Some((active_item, active_editor))
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
    fn activation_focus_handle(&self, cx: &App) -> FocusHandle {
        self.filter_editor.focus_handle(cx)
    }

    fn persistent_name() -> &'static str {
        "Outline Panel"
    }

    fn panel_key() -> &'static str {
        OUTLINE_PANEL_KEY
    }

    fn position(&self, _: &Window, cx: &App) -> DockPosition {
        match OutlinePanelSettings::get_global(cx).dock {
            DockSide::Left => DockPosition::Left,
            DockSide::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            let dock = match position {
                DockPosition::Left | DockPosition::Bottom => DockSide::Left,
                DockPosition::Right => DockSide::Right,
            };
            settings.outline_panel.get_or_insert_default().dock = Some(dock);
        });
    }

    fn default_size(&self, _: &Window, cx: &App) -> Pixels {
        OutlinePanelSettings::get_global(cx).default_width
    }

    fn icon(&self, _: &Window, cx: &App) -> Option<IconName> {
        OutlinePanelSettings::get_global(cx)
            .button
            .then_some(IconName::ListTree)
    }

    fn icon_tooltip(&self, _window: &Window, _: &App) -> Option<&'static str> {
        Some("Outline Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn starts_open(&self, _window: &Window, _: &App) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool, window: &mut Window, cx: &mut Context<Self>) {
        cx.spawn_in(window, async move |outline_panel, cx| {
            outline_panel
                .update_in(cx, |outline_panel, window, cx| {
                    let old_active = outline_panel.active;
                    outline_panel.active = active;
                    if old_active != active {
                        outline_panel.lsp_outline_refresh_task = Task::ready(());
                        outline_panel.outline_fetch_tasks.clear();
                        if active
                            && let Some((active_item, active_editor)) =
                                outline_panel.workspace.upgrade().and_then(|workspace| {
                                    workspace_active_editor(workspace.read(cx), cx)
                                })
                        {
                            if outline_panel.should_replace_active_item(active_item.as_ref()) {
                                outline_panel.replace_active_editor(
                                    active_item,
                                    active_editor,
                                    window,
                                    cx,
                                );
                            } else {
                                outline_panel.update_fs_entries(active_editor, None, window, cx)
                            }
                            return;
                        }

                        if !outline_panel.pinned {
                            outline_panel.clear_previous(window, cx);
                        }
                    }
                    outline_panel.serialize(cx);
                })
                .ok();
        })
        .detach()
    }

    fn activation_priority(&self) -> u32 {
        6
    }

    fn hide_button_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        Some(workspace::HideStatusItem::new(|settings| {
            settings.outline_panel.get_or_insert_default().button = Some(false);
        }))
    }
}

impl Focusable for OutlinePanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<Event> for OutlinePanel {}

impl EventEmitter<PanelEvent> for OutlinePanel {}

impl Render for OutlinePanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (is_local, is_via_ssh) = self.project.read_with(cx, |project, _| {
            (project.is_local(), project.is_via_remote_server())
        });
        let query = self.query(cx);
        let pinned = self.pinned;
        let settings = OutlinePanelSettings::get_global(cx);
        let indent_size = settings.indent_size;
        let show_indent_guides = settings.indent_guides.show == ShowIndentGuides::Always;

        let search_query = match &self.mode {
            ItemsDisplayMode::Search(search_query) => Some(search_query),
            _ => None,
        };

        let search_query_text = search_query.map(|sq| sq.query.to_string());

        v_flex()
            .id("outline-panel")
            .size_full()
            .overflow_hidden()
            .relative()
            .key_context(self.dispatch_context(window, cx))
            .on_action(cx.listener(Self::open_selected_entry))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::scroll_up))
            .on_action(cx.listener(Self::scroll_down))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::scroll_cursor_center))
            .on_action(cx.listener(Self::scroll_cursor_top))
            .on_action(cx.listener(Self::scroll_cursor_bottom))
            .on_action(cx.listener(Self::select_previous))
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
            .on_action(cx.listener(Self::toggle_symbols))
            .on_action(cx.listener(Self::unfold_directory))
            .on_action(cx.listener(Self::fold_directory))
            .on_action(cx.listener(Self::open_excerpts))
            .on_action(cx.listener(Self::open_excerpts_split))
            .when(is_local, |el| {
                el.on_action(cx.listener(Self::reveal_in_finder))
            })
            .when(is_local || is_via_ssh, |el| {
                el.on_action(cx.listener(Self::open_in_terminal))
            })
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |outline_panel, event: &MouseDownEvent, window, cx| {
                    if let Some(entry) = outline_panel.selected_entry().cloned() {
                        outline_panel.deploy_context_menu(event.position, entry, window, cx)
                    } else if let Some(entry) = outline_panel.fs_entries.first().cloned() {
                        outline_panel.deploy_context_menu(
                            event.position,
                            PanelEntry::Fs(entry),
                            window,
                            cx,
                        )
                    }
                }),
            )
            .track_focus(&self.focus_handle)
            .child(self.render_filter_footer(pinned, cx))
            .when_some(search_query_text, |outline_panel, query_text| {
                outline_panel.child(
                    h_flex()
                        .py_1p5()
                        .px_2()
                        .h(Tab::container_height(cx))
                        .gap_0p5()
                        .border_b_1()
                        .border_color(cx.theme().colors().border_variant)
                        .child(Label::new("Searching:").color(Color::Muted))
                        .child(Label::new(query_text)),
                )
            })
            .child(self.render_main_contents(query, show_indent_guides, indent_size, window, cx))
    }
}

fn find_active_indent_guide_ix(
    outline_panel: &OutlinePanel,
    candidates: &[IndentGuideLayout],
) -> Option<usize> {
    let SelectedEntry::Valid(_, target_ix) = &outline_panel.selected_entry else {
        return None;
    };
    let target_depth = outline_panel
        .cached_entries
        .get(*target_ix)
        .map(|cached_entry| cached_entry.depth)?;

    let (target_ix, target_depth) = if let Some(target_depth) = outline_panel
        .cached_entries
        .get(target_ix + 1)
        .filter(|cached_entry| cached_entry.depth > target_depth)
        .map(|entry| entry.depth)
    {
        (target_ix + 1, target_depth.saturating_sub(1))
    } else {
        (*target_ix, target_depth.saturating_sub(1))
    };

    candidates
        .iter()
        .enumerate()
        .find(|(_, guide)| {
            guide.offset.y <= target_ix
                && target_ix < guide.offset.y + guide.length
                && guide.offset.x == target_depth
        })
        .map(|(ix, _)| ix)
}

fn subscribe_for_editor_events(
    editor: &Entity<Editor>,
    window: &mut Window,
    cx: &mut Context<OutlinePanel>,
) -> Subscription {
    let debounce = Some(UPDATE_DEBOUNCE);
    cx.subscribe_in(
        editor,
        window,
        move |outline_panel, editor, e: &EditorEvent, window, cx| {
            if !outline_panel.active {
                return;
            }
            match e {
                EditorEvent::SelectionsChanged { local: true } => {
                    outline_panel.reveal_entry_for_selection(editor.clone(), window, cx);
                }
                EditorEvent::BuffersRemoved { removed_buffer_ids } => {
                    for buffer_id in removed_buffer_ids {
                        outline_panel.buffers.remove(buffer_id);
                        outline_panel.outline_fetch_tasks.remove(buffer_id);
                    }
                    outline_panel.update_fs_entries(editor.clone(), debounce, window, cx);
                }
                EditorEvent::BufferRangesUpdated { buffer, .. } => {
                    outline_panel
                        .new_entries_for_fs_update
                        .insert(buffer.read(cx).remote_id());
                    outline_panel.invalidate_outlines(&[buffer.read(cx).remote_id()]);
                    outline_panel.update_fs_entries(editor.clone(), debounce, window, cx);
                }
                EditorEvent::BuffersEdited { buffer_ids } => {
                    outline_panel.invalidate_outlines(buffer_ids);
                    outline_panel.update_contents(debounce, window, cx);
                }
                EditorEvent::BufferFoldToggled { ids, .. } => {
                    outline_panel.invalidate_outlines(ids);
                    let mut latest_unfolded_buffer_id = None;
                    let mut latest_folded_buffer_id = None;
                    let mut ignore_selections_change = false;
                    outline_panel.new_entries_for_fs_update.extend(
                        ids.iter()
                            .filter(|id| {
                                if outline_panel.buffers.contains_key(&id) {
                                    ignore_selections_change |= outline_panel
                                        .preserve_selection_on_buffer_fold_toggles
                                        .remove(&id);
                                    if editor.read(cx).is_buffer_folded(**id, cx) {
                                        latest_folded_buffer_id = Some(**id);
                                        false
                                    } else {
                                        latest_unfolded_buffer_id = Some(**id);
                                        true
                                    }
                                } else {
                                    false
                                }
                            })
                            .copied(),
                    );
                    if !ignore_selections_change
                        && let Some(entry_to_select) = latest_unfolded_buffer_id
                            .or(latest_folded_buffer_id)
                            .and_then(|toggled_buffer_id| {
                                outline_panel.fs_entries.iter().find_map(
                                    |fs_entry| match fs_entry {
                                        FsEntry::ExternalFile(external) => {
                                            if external.buffer_id == toggled_buffer_id {
                                                Some(fs_entry.clone())
                                            } else {
                                                None
                                            }
                                        }
                                        FsEntry::File(FsEntryFile { buffer_id, .. }) => {
                                            if *buffer_id == toggled_buffer_id {
                                                Some(fs_entry.clone())
                                            } else {
                                                None
                                            }
                                        }
                                        FsEntry::Directory(..) => None,
                                    },
                                )
                            })
                            .map(PanelEntry::Fs)
                    {
                        outline_panel.select_entry(entry_to_select, true, window, cx);
                    }

                    outline_panel.update_fs_entries(editor.clone(), debounce, window, cx);
                }
                EditorEvent::Reparsed(buffer_id) => {
                    outline_panel.invalidate_outlines(&[*buffer_id]);
                    outline_panel.update_contents(debounce, window, cx);
                }
                EditorEvent::OutlineSymbolsChanged => {
                    if outline_panel.lsp_outline_refresh_task.is_ready() {
                        outline_panel.lsp_outline_refresh_task =
                            cx.spawn_in(window, async move |outline_panel, cx| {
                                cx.background_executor().timer(UPDATE_DEBOUNCE).await;
                                outline_panel
                                    .update_in(cx, |outline_panel, window, cx| {
                                        let Some(editor) = outline_panel.active_editor() else {
                                            return;
                                        };
                                        let editor_buffer = editor.read(cx).buffer().read(cx);
                                        let mut invalidated = false;
                                        for (buffer_id, outlines) in &mut outline_panel.buffers {
                                            if editor_buffer.buffer(*buffer_id).is_some_and(
                                                |buffer| {
                                                    LanguageSettings::for_buffer(
                                                        buffer.read(cx),
                                                        cx,
                                                    )
                                                    .document_symbols
                                                    .lsp_enabled()
                                                },
                                            ) {
                                                outlines.invalidate_outlines();
                                                invalidated = true;
                                            }
                                        }
                                        if invalidated {
                                            outline_panel.fetch_outdated_outlines(None, window, cx);
                                        }
                                    })
                                    .ok();
                            });
                    }
                }
                EditorEvent::TitleChanged => {
                    outline_panel.update_fs_entries(editor.clone(), debounce, window, cx);
                }
                _ => {}
            }
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

fn folder_indicator_element(
    indicator: FolderIndicator,
    expanded: bool,
    path: &Path,
    color: Color,
    cx: &App,
) -> Option<AnyElement> {
    let indicators = FileIcons::get_folder_indicators(indicator, expanded, path, cx);
    let render_indicator = |icon_path| Icon::from_path(icon_path).color(color);

    match (indicators.chevron, indicators.icon) {
        (Some(chevron), Some(icon)) => Some(
            h_flex()
                .flex_none()
                .gap_0p5()
                .child(render_indicator(chevron))
                .child(render_indicator(icon))
                .into_any_element(),
        ),
        (Some(only), None) | (None, Some(only)) => Some(render_indicator(only).into_any_element()),
        (None, None) => None,
    }
}

/// Adds a blank as wide as a chevron in front of `icon` in `both` mode. Leaves `icon`
/// alone in the other modes.
///
/// Directories show a chevron and an icon in `both` mode. Without the blank, a file's
/// icon would line up under the folder chevrons instead of the folder icons.
fn reserve_chevron_slot(indicator: FolderIndicator, icon: AnyElement) -> AnyElement {
    if indicator.shows_chevron() && indicator.shows_icon() {
        h_flex()
            .flex_none()
            .gap_0p5()
            .child(empty_icon())
            .child(icon)
            .into_any_element()
    } else {
        icon
    }
}

#[derive(Debug, Default)]
struct GenerationState {
    entries: Vec<CachedEntry>,
    match_candidates: Vec<StringMatchCandidate>,
    max_width_estimate_and_index: Option<(u64, usize)>,
}

impl GenerationState {
    fn clear(&mut self) {
        self.entries.clear();
        self.match_candidates.clear();
        self.max_width_estimate_and_index = None;
    }
}

#[cfg(test)]
mod tests {
    use buffer_diff::BufferDiff;
    use db::indoc;
    use editor::{HiddenUnstagedDiffHunkRenderer, PathKey};
    use futures::{FutureExt as _, StreamExt as _, future::poll_fn, task::Poll};
    use gpui::{TestAppContext, UpdateGlobal, VisualTestContext, WindowHandle};
    use language::{self, FakeLspAdapter, markdown_lang, rust_lang};
    use pretty_assertions::assert_eq;
    use project::FakeFs;
    use search::{
        buffer_search,
        project_search::{self, perform_project_search},
    };
    use serde_json::json;
    use util::{path, rel_path::rel_path};
    use workspace::{MultiWorkspace, OpenOptions, OpenVisible, ToolbarItemView};

    use super::*;

    const SELECTED_MARKER: &str = "  <==== selected";

    #[gpui::test]
    async fn test_outline_fetches_survive_unrelated_updates_but_not_deactivation(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        let project = Project::test(fs, [], cx).await;
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let panel = outline_panel(&workspace, cx);
        let [requested_id, retained_id, unknown_id] =
            [1, 2, 3].map(|id| BufferId::new(id).expect("nonzero buffer ID"));
        let (requested_sender, requested_receiver) = futures::channel::oneshot::channel::<()>();
        let (retained_sender, retained_receiver) = futures::channel::oneshot::channel::<()>();
        let (refresh_sender, refresh_receiver) = futures::channel::oneshot::channel::<()>();
        panel.update(cx, |panel, cx| {
            panel.lsp_outline_refresh_task = cx.background_spawn(async move {
                refresh_receiver
                    .await
                    .expect("LSP refresh should be canceled before completion");
            });
            for buffer_id in [requested_id, retained_id] {
                panel.buffers.insert(
                    buffer_id,
                    BufferOutlines {
                        excerpts: Vec::new(),
                        outlines: OutlineState::Outlines(Vec::new()),
                    },
                );
            }
            for (buffer_id, receiver) in [
                (requested_id, requested_receiver),
                (retained_id, retained_receiver),
            ] {
                panel.outline_fetch_tasks.insert(
                    buffer_id,
                    cx.background_spawn(async move {
                        receiver
                            .await
                            .expect("outline fetch should be canceled before completion");
                    }),
                );
            }
            panel.invalidate_outlines(&[]);
            panel.invalidate_outlines(&[unknown_id]);
            assert_eq!(panel.outline_fetch_tasks.len(), 2);
            panel.invalidate_outlines(&[requested_id, requested_id, unknown_id]);
            assert_eq!(
                panel
                    .outline_fetch_tasks
                    .keys()
                    .copied()
                    .collect::<Vec<_>>(),
                vec![retained_id]
            );
            assert!(matches!(
                panel.buffers[&requested_id].outlines,
                OutlineState::Invalidated(_)
            ));
            assert!(matches!(
                panel.buffers[&retained_id].outlines,
                OutlineState::Outlines(_)
            ));
        });
        cx.run_until_parked();
        assert!(requested_sender.is_canceled());
        assert!(!retained_sender.is_canceled());
        assert!(!refresh_sender.is_canceled());

        panel.update_in(cx, |panel, window, cx| {
            panel.active = true;
            panel.pinned = true;
            panel.set_active(false, window, cx);
        });
        cx.run_until_parked();
        assert!(retained_sender.is_canceled());
        assert!(refresh_sender.is_canceled());
        panel.read_with(cx, |panel, _| {
            assert!(!panel.active);
            assert!(panel.outline_fetch_tasks.is_empty());
            assert_eq!(panel.buffers.len(), 2);
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_project_search_results_toggling(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        let root = path!("/rust-analyzer");
        populate_with_test_ra_project(&fs, root).await;
        let project = Project::test(fs.clone(), [Path::new(root)], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        workspace.update_in(cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(
                workspace,
                &workspace::DeploySearch::default(),
                window,
                cx,
            )
        });
        let search_view = workspace.update_in(cx, |workspace, _window, cx| {
            workspace
                .active_pane()
                .read(cx)
                .items()
                .find_map(|item| item.downcast::<ProjectSearchView>())
                .expect("Project search view expected to appear after new search event trigger")
        });

        let query = "param_names_for_lifetime_elision_hints";
        perform_project_search(&search_view, query, cx);
        search_view.update(cx, |search_view, cx| {
            search_view
                .results_editor()
                .update(cx, |results_editor, cx| {
                    assert_eq!(
                        results_editor.display_text(cx).match_indices(query).count(),
                        9
                    );
                });
        });

        let all_matches = r#"rust-analyzer/
  crates/
    ide/src/
      inlay_hints/
        fn_lifetime_fn.rs
          search: match config.«param_names_for_lifetime_elision_hints» {
          search: allocated_lifetimes.push(if config.«param_names_for_lifetime_elision_hints» {
          search: Some(it) if config.«param_names_for_lifetime_elision_hints» => {
          search: InlayHintsConfig { «param_names_for_lifetime_elision_hints»: true, ..TEST_CONFIG },
      inlay_hints.rs
        search: pub «param_names_for_lifetime_elision_hints»: bool,
        search: «param_names_for_lifetime_elision_hints»: self
      static_index.rs
        search: «param_names_for_lifetime_elision_hints»: false,
    rust-analyzer/src/
      cli/
        analysis_stats.rs
          search: «param_names_for_lifetime_elision_hints»: true,
      config.rs
        search: «param_names_for_lifetime_elision_hints»: self"#
            .to_string();

        let select_first_in_all_matches = |line_to_select: &str| {
            assert!(
                all_matches.contains(line_to_select),
                "`{line_to_select}` was not found in all matches `{all_matches}`"
            );
            all_matches.replacen(
                line_to_select,
                &format!("{line_to_select}{SELECTED_MARKER}"),
                1,
            )
        };

        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                select_first_in_all_matches(
                    "search: match config.«param_names_for_lifetime_elision_hints» {"
                )
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.select_parent(&SelectParent, window, cx);
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                select_first_in_all_matches("fn_lifetime_fn.rs")
            );
        });
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                format!(
                    r#"rust-analyzer/
  crates/
    ide/src/
      inlay_hints/
        fn_lifetime_fn.rs{SELECTED_MARKER}
      inlay_hints.rs
        search: pub «param_names_for_lifetime_elision_hints»: bool,
        search: «param_names_for_lifetime_elision_hints»: self
      static_index.rs
        search: «param_names_for_lifetime_elision_hints»: false,
    rust-analyzer/src/
      cli/
        analysis_stats.rs
          search: «param_names_for_lifetime_elision_hints»: true,
      config.rs
        search: «param_names_for_lifetime_elision_hints»: self"#,
                )
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.expand_all_entries(&ExpandAllEntries, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.select_parent(&SelectParent, window, cx);
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                select_first_in_all_matches("inlay_hints/")
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.select_parent(&SelectParent, window, cx);
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                select_first_in_all_matches("ide/src/")
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                format!(
                    r#"rust-analyzer/
  crates/
    ide/src/{SELECTED_MARKER}
    rust-analyzer/src/
      cli/
        analysis_stats.rs
          search: «param_names_for_lifetime_elision_hints»: true,
      config.rs
        search: «param_names_for_lifetime_elision_hints»: self"#,
                )
            );
        });
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                select_first_in_all_matches("ide/src/")
            );
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_item_filtering(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        let root = path!("/rust-analyzer");
        populate_with_test_ra_project(&fs, root).await;
        let project = Project::test(fs.clone(), [Path::new(root)], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        workspace.update_in(cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(
                workspace,
                &workspace::DeploySearch::default(),
                window,
                cx,
            )
        });
        let search_view = workspace.update_in(cx, |workspace, _window, cx| {
            workspace
                .active_pane()
                .read(cx)
                .items()
                .find_map(|item| item.downcast::<ProjectSearchView>())
                .expect("Project search view expected to appear after new search event trigger")
        });

        let query = "param_names_for_lifetime_elision_hints";
        perform_project_search(&search_view, query, cx);
        search_view.update(cx, |search_view, cx| {
            search_view
                .results_editor()
                .update(cx, |results_editor, cx| {
                    assert_eq!(
                        results_editor.display_text(cx).match_indices(query).count(),
                        9
                    );
                });
        });
        let all_matches = r#"rust-analyzer/
  crates/
    ide/src/
      inlay_hints/
        fn_lifetime_fn.rs
          search: match config.«param_names_for_lifetime_elision_hints» {
          search: allocated_lifetimes.push(if config.«param_names_for_lifetime_elision_hints» {
          search: Some(it) if config.«param_names_for_lifetime_elision_hints» => {
          search: InlayHintsConfig { «param_names_for_lifetime_elision_hints»: true, ..TEST_CONFIG },
      inlay_hints.rs
        search: pub «param_names_for_lifetime_elision_hints»: bool,
        search: «param_names_for_lifetime_elision_hints»: self
      static_index.rs
        search: «param_names_for_lifetime_elision_hints»: false,
    rust-analyzer/src/
      cli/
        analysis_stats.rs
          search: «param_names_for_lifetime_elision_hints»: true,
      config.rs
        search: «param_names_for_lifetime_elision_hints»: self"#
            .to_string();

        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    None,
                    cx,
                ),
                all_matches,
            );
        });

        let filter_text = "a";
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.filter_editor.update(cx, |filter_editor, cx| {
                filter_editor.set_text(filter_text, window, cx);
            });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    None,
                    cx,
                ),
                all_matches
                    .lines()
                    .skip(1) // `/rust-analyzer/` is a root entry with path `` and it will be filtered out
                    .filter(|item| item.contains(filter_text))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.filter_editor.update(cx, |filter_editor, cx| {
                filter_editor.set_text("", window, cx);
            });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    None,
                    cx,
                ),
                all_matches,
            );
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_item_opening(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        let root = path!("/rust-analyzer");
        populate_with_test_ra_project(&fs, root).await;
        let project = Project::test(fs.clone(), [Path::new(root)], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        workspace.update_in(cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(
                workspace,
                &workspace::DeploySearch::default(),
                window,
                cx,
            )
        });
        let search_view = workspace.update_in(cx, |workspace, _window, cx| {
            workspace
                .active_pane()
                .read(cx)
                .items()
                .find_map(|item| item.downcast::<ProjectSearchView>())
                .expect("Project search view expected to appear after new search event trigger")
        });

        let query = "param_names_for_lifetime_elision_hints";
        perform_project_search(&search_view, query, cx);
        search_view.update(cx, |search_view, cx| {
            search_view
                .results_editor()
                .update(cx, |results_editor, cx| {
                    assert_eq!(
                        results_editor.display_text(cx).match_indices(query).count(),
                        9
                    );
                });
        });
        let all_matches = r#"rust-analyzer/
  crates/
    ide/src/
      inlay_hints/
        fn_lifetime_fn.rs
          search: match config.«param_names_for_lifetime_elision_hints» {
          search: allocated_lifetimes.push(if config.«param_names_for_lifetime_elision_hints» {
          search: Some(it) if config.«param_names_for_lifetime_elision_hints» => {
          search: InlayHintsConfig { «param_names_for_lifetime_elision_hints»: true, ..TEST_CONFIG },
      inlay_hints.rs
        search: pub «param_names_for_lifetime_elision_hints»: bool,
        search: «param_names_for_lifetime_elision_hints»: self
      static_index.rs
        search: «param_names_for_lifetime_elision_hints»: false,
    rust-analyzer/src/
      cli/
        analysis_stats.rs
          search: «param_names_for_lifetime_elision_hints»: true,
      config.rs
        search: «param_names_for_lifetime_elision_hints»: self"#
            .to_string();
        let select_first_in_all_matches = |line_to_select: &str| {
            assert!(
                all_matches.contains(line_to_select),
                "`{line_to_select}` was not found in all matches `{all_matches}`"
            );
            all_matches.replacen(
                line_to_select,
                &format!("{line_to_select}{SELECTED_MARKER}"),
                1,
            )
        };
        let clear_outline_metadata = |input: &str| {
            input
                .replace("search: ", "")
                .replace("«", "")
                .replace("»", "")
        };

        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        let active_editor = outline_panel.read_with(cx, |outline_panel, _| {
            outline_panel
                .active_editor()
                .expect("should have an active editor open")
        });
        let initial_outline_selection =
            "search: match config.«param_names_for_lifetime_elision_hints» {";
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                select_first_in_all_matches(initial_outline_selection)
            );
            assert_eq!(
                selected_row_text(&active_editor, cx),
                clear_outline_metadata(initial_outline_selection),
                "Should place the initial editor selection on the corresponding search result"
            );

            outline_panel.select_next(&SelectNext, window, cx);
            outline_panel.select_next(&SelectNext, window, cx);
        });

        let navigated_outline_selection =
            "search: Some(it) if config.«param_names_for_lifetime_elision_hints» => {";
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                select_first_in_all_matches(navigated_outline_selection)
            );
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        outline_panel.update(cx, |_, cx| {
            assert_eq!(
                selected_row_text(&active_editor, cx),
                clear_outline_metadata(navigated_outline_selection),
                "Should still have the initial caret position after SelectNext calls"
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.open_selected_entry(&OpenSelectedEntry, window, cx);
        });
        outline_panel.update(cx, |_outline_panel, cx| {
            assert_eq!(
                selected_row_text(&active_editor, cx),
                clear_outline_metadata(navigated_outline_selection),
                "After opening, should move the caret to the opened outline entry's position"
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.select_next(&SelectNext, window, cx);
        });
        let next_navigated_outline_selection = "search: InlayHintsConfig { «param_names_for_lifetime_elision_hints»: true, ..TEST_CONFIG },";
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                select_first_in_all_matches(next_navigated_outline_selection)
            );
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        outline_panel.update(cx, |_outline_panel, cx| {
            assert_eq!(
                selected_row_text(&active_editor, cx),
                clear_outline_metadata(next_navigated_outline_selection),
                "Should again preserve the selection after another SelectNext call"
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.open_excerpts(&editor::actions::OpenExcerpts, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        let new_active_editor = outline_panel.read_with(cx, |outline_panel, _| {
            outline_panel
                .active_editor()
                .expect("should have an active editor open")
        });
        outline_panel.update(cx, |outline_panel, cx| {
            assert_ne!(
                active_editor, new_active_editor,
                "After opening an excerpt, new editor should be open"
            );
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                "outline: pub(super) fn hints
outline: fn hints_lifetimes_named  <==== selected"
            );
            assert_eq!(
                selected_row_text(&new_active_editor, cx),
                clear_outline_metadata(next_navigated_outline_selection),
                "When opening the excerpt, should navigate to the place corresponding the outline entry"
            );
        });
    }

    #[gpui::test]
    async fn test_multiple_worktrees(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                "one": {
                    "a.txt": "aaa aaa"
                },
                "two": {
                    "b.txt": "a aaa"
                }

            }),
        )
        .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/root/one"))], cx).await;
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        let items = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_paths(
                    vec![PathBuf::from(path!("/root/two"))],
                    OpenOptions {
                        visible: Some(OpenVisible::OnlyDirectories),
                        ..Default::default()
                    },
                    None,
                    window,
                    cx,
                )
            })
            .await;
        assert_eq!(items.len(), 1, "Were opening another worktree directory");
        assert!(
            items[0].is_none(),
            "Directory should be opened successfully"
        );

        workspace.update_in(cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(
                workspace,
                &workspace::DeploySearch::default(),
                window,
                cx,
            )
        });
        let search_view = workspace.update_in(cx, |workspace, _window, cx| {
            workspace
                .active_pane()
                .read(cx)
                .items()
                .find_map(|item| item.downcast::<ProjectSearchView>())
                .expect("Project search view expected to appear after new search event trigger")
        });

        let query = "aaa";
        perform_project_search(&search_view, query, cx);
        search_view.update(cx, |search_view, cx| {
            search_view
                .results_editor()
                .update(cx, |results_editor, cx| {
                    assert_eq!(
                        results_editor.display_text(cx).match_indices(query).count(),
                        3
                    );
                });
        });

        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                r#"one/
  a.txt
    search: «aaa» aaa  <==== selected
    search: aaa «aaa»
two/
  b.txt
    search: a «aaa»"#
                    .to_string(),
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.select_previous(&SelectPrevious, window, cx);
            outline_panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                r#"one/
  a.txt  <==== selected
two/
  b.txt
    search: a «aaa»"#
                    .to_string(),
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.select_next(&SelectNext, window, cx);
            outline_panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                r#"one/
  a.txt
two/  <==== selected"#
                    .to_string(),
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                r#"one/
  a.txt
two/  <==== selected
  b.txt
    search: a «aaa»"#
                    .to_string()
            );
        });
    }

    #[gpui::test]
    async fn test_file_order_matches_diff_multibuffer(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                "crates": { "ide": {
                    "asdsads": "new file",
                    "src": {
                        "interpret.rs": "fn interpret() {}",
                        "join_lines.rs": "fn join_lines() {}"
                    }
                } }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;
        update_outline_panel_settings(cx, |settings| {
            settings.auto_fold_dirs = Some(true);
            settings.multi_buffer_hide_symbols = Some(true);
        });
        let added = open_buffer(&project, path!("/root/crates/ide/asdsads"), cx).await;
        let deleted = open_buffer(&project, path!("/root/crates/ide/src/join_lines.rs"), cx).await;
        let modified = open_buffer(&project, path!("/root/crates/ide/src/interpret.rs"), cx).await;
        fs.remove_file(
            Path::new(path!("/root/crates/ide/src/join_lines.rs")),
            project::RemoveOptions::default(),
        )
        .await
        .expect("file deletion should succeed");
        deleted.update(cx, |buffer, cx| {
            buffer.edit(
                [(language::Point::default()..buffer.max_point(), "")],
                None,
                cx,
            );
        });
        let editor = add_diff_editor(
            &workspace,
            &project,
            &[
                (&modified, "old interpret"),
                (&deleted, "fn join_lines() {}"),
                (&added, ""),
            ],
            cx,
        );
        set_buffer_order(&editor, &[&modified, &deleted, &added], cx);
        workspace.update_in(cx, |workspace, window, cx| {
            workspace
                .focus_panel::<OutlinePanel>(window, cx)
                .expect("outline panel should be installed");
        });
        select_file(
            &outline_panel,
            modified.read_with(cx, |buffer, _| buffer.remote_id()),
            cx,
        );
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  crates/ide/
                    src/
                      interpret.rs  <==== selected
                      ~~join_lines.rs~~
                    asdsads"
            ),
            cx,
        );
        set_buffer_order(&editor, &[&added, &deleted, &modified], cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  crates/ide/
                    asdsads
                    src/
                      ~~join_lines.rs~~
                      interpret.rs  <==== selected"
            ),
            cx,
        );
    }

    #[gpui::test]
    async fn test_file_order_preserves_interleaved_directory_sections(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/one"),
            json!({ "é": { "子": { "first.txt": "first", "second.txt": "second" } } }),
        )
        .await;
        fs.insert_tree(path!("/two"), json!({ "other.txt": "other" }))
            .await;
        let project = Project::test(
            fs.clone(),
            [Path::new(path!("/one")), Path::new(path!("/two"))],
            cx,
        )
        .await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;
        update_outline_panel_settings(cx, |settings| {
            settings.auto_fold_dirs = Some(true);
            settings.multi_buffer_hide_symbols = Some(true);
        });
        let first = open_buffer(&project, path!("/one/é/子/first.txt"), cx).await;
        let second = open_buffer(&project, path!("/one/é/子/second.txt"), cx).await;
        let other = open_buffer(&project, path!("/two/other.txt"), cx).await;
        let untitled = project.update(cx, |project, cx| {
            project.create_local_buffer("untitled", None, true, cx)
        });
        fs.remove_dir(
            Path::new(path!("/one/é")),
            project::RemoveOptions {
                recursive: true,
                ignore_if_not_exists: false,
            },
        )
        .await
        .expect("ancestor deletion should succeed");
        fs.insert_file(path!("/one/é"), b"replacement".to_vec())
            .await;
        let replacement = open_buffer(&project, path!("/one/é"), cx).await;
        let editor = add_multi_buffer_editor(
            &workspace,
            &project,
            &[
                (&first, Vec::new()),
                (&second, Vec::new()),
                (&other, Vec::new()),
                (&untitled, Vec::new()),
                (&replacement, Vec::new()),
            ],
            cx,
        );
        set_buffer_order(
            &editor,
            &[&second, &other, &untitled, &first, &replacement],
            cx,
        );
        select_file(
            &outline_panel,
            first.read_with(cx, |buffer, _| buffer.remote_id()),
            cx,
        );
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.select_parent(&SelectParent, window, cx);
        });
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                one/
                  é/子/
                    ~~second.txt~~
                two/
                  other.txt
                external: untitled
                one/
                  é/子/  <==== selected
                    ~~first.txt~~
                  é"
            ),
            cx,
        );
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                one/
                  é/子/
                two/
                  other.txt
                external: untitled
                one/
                  é/子/  <==== selected
                  é"
            ),
            cx,
        );
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
        });
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                one/
                  é/子/
                    ~~second.txt~~
                two/
                  other.txt
                external: untitled
                one/
                  é/子/  <==== selected
                    ~~first.txt~~
                  é"
            ),
            cx,
        );
        update_outline_panel_settings(cx, |settings| {
            settings.auto_fold_dirs = Some(false);
        });
        wait_for_outline_tasks(&outline_panel, cx).await;
        select_file(
            &outline_panel,
            first.read_with(cx, |buffer, _| buffer.remote_id()),
            cx,
        );
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.select_parent(&SelectParent, window, cx);
        });
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                one/
                  é/
                    子/
                      ~~second.txt~~
                two/
                  other.txt
                external: untitled
                one/
                  é/
                    子/  <==== selected
                      ~~first.txt~~
                  é"
            ),
            cx,
        );
    }

    #[gpui::test]
    async fn test_directory_selection_survives_buffer_reordering(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({ "dir": { "sub": { "a.txt": "a", "b.txt": "b" } }, "z.txt": "z" }),
        )
        .await;
        let project = Project::test(fs, [Path::new(path!("/root"))], cx).await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;
        update_outline_panel_settings(cx, |settings| {
            settings.auto_fold_dirs = Some(true);
            settings.multi_buffer_hide_symbols = Some(true);
        });
        let buffer_a = open_buffer(&project, path!("/root/dir/sub/a.txt"), cx).await;
        let buffer_b = open_buffer(&project, path!("/root/dir/sub/b.txt"), cx).await;
        let buffer_z = open_buffer(&project, path!("/root/z.txt"), cx).await;
        let worktree_id = buffer_a.read_with(cx, |buffer, cx| {
            File::from_dyn(buffer.file())
                .expect("buffer should have a file")
                .worktree
                .read(cx)
                .id()
        });
        let editor = add_multi_buffer_editor(
            &workspace,
            &project,
            &[
                (&buffer_a, Vec::new()),
                (&buffer_b, Vec::new()),
                (&buffer_z, Vec::new()),
            ],
            cx,
        );
        set_buffer_order(&editor, &[&buffer_b, &buffer_a, &buffer_z], cx);
        workspace.update_in(cx, |workspace, window, cx| {
            workspace
                .focus_panel::<OutlinePanel>(window, cx)
                .expect("outline panel should be installed");
        });
        select_directory(&outline_panel, worktree_id, rel_path("dir/sub"), cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/sub/  <==== selected
                    b.txt
                    a.txt
                  z.txt"
            ),
            cx,
        );
        set_buffer_order(&editor, &[&buffer_a, &buffer_b, &buffer_z], cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/sub/  <==== selected
                    a.txt
                    b.txt
                  z.txt"
            ),
            cx,
        );
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.select_next(&SelectNext, window, cx);
        });
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/sub/
                    a.txt  <==== selected
                    b.txt
                  z.txt"
            ),
            cx,
        );
        select_directory(&outline_panel, worktree_id, rel_path("dir/sub"), cx);
        rename_buffer(&project, &buffer_a, rel_path("a.txt"), cx).await;
        set_buffer_order(&editor, &[&buffer_a, &buffer_b, &buffer_z], cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  a.txt
                  dir/sub/  <==== selected
                    b.txt
                  z.txt"
            ),
            cx,
        );
        rename_buffer(&project, &buffer_a, rel_path("dir/sub/a.txt"), cx).await;
        set_buffer_order(&editor, &[&buffer_a, &buffer_b, &buffer_z], cx);
        let buffer_a_id = buffer_a.read_with(cx, |buffer, _| buffer.remote_id());
        editor.update(cx, |editor, cx| {
            editor.buffer().update(cx, |multibuffer, cx| {
                multibuffer.remove_excerpts_for_buffer(buffer_a_id, cx);
            });
        });
        set_buffer_order(&editor, &[&buffer_b, &buffer_z], cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/sub/  <==== selected
                    b.txt
                  z.txt"
            ),
            cx,
        );
    }

    #[gpui::test]
    async fn test_selected_file_survives_rename_and_deletion(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/root"), json!({ "a.txt": "a", "z.txt": "z" }))
            .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;
        update_outline_panel_settings(cx, |settings| {
            settings.multi_buffer_hide_symbols = Some(true);
        });
        let buffer_a = open_buffer(&project, path!("/root/a.txt"), cx).await;
        let buffer_z = open_buffer(&project, path!("/root/z.txt"), cx).await;
        let buffer_id = buffer_a.read_with(cx, |buffer, _| buffer.remote_id());
        add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer_a, Vec::new()), (&buffer_z, Vec::new())],
            cx,
        );
        flush_outline_tasks(cx);
        workspace.update_in(cx, |workspace, window, cx| {
            workspace
                .focus_panel::<OutlinePanel>(window, cx)
                .expect("outline panel should be installed");
        });
        select_file(&outline_panel, buffer_id, cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  a.txt  <==== selected
                  z.txt"
            ),
            cx,
        );

        fs.rename(
            Path::new(path!("/root/a.txt")),
            Path::new(path!("/root/b.txt")),
            project::RenameOptions::default(),
        )
        .await
        .expect("file rename should succeed");
        flush_outline_tasks(cx);
        buffer_a.read_with(cx, |buffer, _| {
            assert_eq!(
                File::from_dyn(buffer.file())
                    .expect("buffer should retain its file")
                    .path
                    .as_ref(),
                rel_path("b.txt"),
            );
        });
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  b.txt  <==== selected
                  z.txt"
            ),
            cx,
        );
        outline_panel.update_in(cx, |panel, window, cx| {
            assert!(panel.focus_handle.is_focused(window));
            match &panel.selected_entry {
                SelectedEntry::Valid(PanelEntry::Fs(FsEntry::File(file)), _) => {
                    assert_eq!(
                        (file.buffer_id, file.entry.path.as_ref()),
                        (buffer_id, rel_path("b.txt"))
                    );
                }
                selected => {
                    panic!("expected the renamed file to remain selected, got {selected:?}")
                }
            }
            panel.select_next(&SelectNext, window, cx);
        });
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  b.txt
                  z.txt  <==== selected"
            ),
            cx,
        );

        outline_panel.update_in(cx, |panel, window, cx| {
            panel.select_previous(&SelectPrevious, window, cx);
        });
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  b.txt  <==== selected
                  z.txt"
            ),
            cx,
        );
        fs.remove_file(
            Path::new(path!("/root/b.txt")),
            project::RemoveOptions::default(),
        )
        .await
        .expect("renamed file deletion should succeed");
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  ~~b.txt~~  <==== selected
                  z.txt"
            ),
            cx,
        );
        outline_panel.read_with(cx, |panel, _| match &panel.selected_entry {
            SelectedEntry::Valid(PanelEntry::Fs(FsEntry::File(file)), _) => {
                assert_eq!(
                    (file.buffer_id, file.entry.path.as_ref()),
                    (buffer_id, rel_path("b.txt"))
                );
                assert!(file.is_deleted);
            }
            selected => panic!("expected the deleted file to remain selected, got {selected:?}"),
        });
    }

    #[gpui::test]
    async fn test_directory_state_prunes_paths_after_repeated_renames(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({ "d0": { "file.txt": "retained buffer" }, "z.txt": "z" }),
        )
        .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;
        update_outline_panel_settings(cx, |settings| {
            settings.auto_fold_dirs = Some(true);
            settings.multi_buffer_hide_symbols = Some(true);
        });
        let buffer = open_buffer(&project, path!("/root/d0/file.txt"), cx).await;
        let buffer_z = open_buffer(&project, path!("/root/z.txt"), cx).await;
        let (buffer_id, worktree_id) = buffer.read_with(cx, |buffer, cx| {
            let file = File::from_dyn(buffer.file()).expect("buffer should have a file");
            (buffer.remote_id(), file.worktree.read(cx).id())
        });
        let buffer_z_id = buffer_z.read_with(cx, |buffer, _| buffer.remote_id());
        add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer, Vec::new()), (&buffer_z, Vec::new())],
            cx,
        );
        flush_outline_tasks(cx);
        select_file(&outline_panel, buffer_z_id, cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  d0/
                    file.txt
                  z.txt  <==== selected"
            ),
            cx,
        );
        assert_directory_paths(&outline_panel, worktree_id, &["", "d0"], &[], cx);

        for (previous, next) in [("d0", "d1"), ("d1", "d2"), ("d2", "d3"), ("d3", "d4")] {
            select_directory(&outline_panel, worktree_id, rel_path(previous), cx);
            outline_panel.update_in(cx, |panel, window, cx| {
                panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
            });
            flush_outline_tasks(cx);
            select_file(&outline_panel, buffer_z_id, cx);
            assert_tree(
                &outline_panel,
                &project,
                &format!(
                    indoc!(
                        "
                        root/
                          {previous}/
                          z.txt  <==== selected"
                    ),
                    previous = previous,
                ),
                cx,
            );
            assert_directory_paths(
                &outline_panel,
                worktree_id,
                &["", previous],
                &[previous],
                cx,
            );

            let next_directory = Path::new(path!("/root")).join(next);
            fs.create_dir(&next_directory)
                .await
                .expect("destination directory should be created");
            let entry_id = buffer.read_with(cx, |buffer, _| {
                File::from_dyn(buffer.file())
                    .and_then(|file| file.project_entry_id())
                    .expect("file should have a worktree entry")
            });
            project
                .update(cx, |project, cx| {
                    project.rename_entry(
                        entry_id,
                        ProjectPath {
                            worktree_id,
                            path: Arc::from(rel_path(&format!("{next}/file.txt"))),
                        },
                        cx,
                    )
                })
                .await
                .expect("file rename should succeed");
            cx.run_until_parked();
            project
                .update(cx, |project, cx| project.git_scans_complete(cx))
                .await;
            flush_outline_tasks(cx);
            buffer.read_with(cx, |buffer, _| {
                let file = File::from_dyn(buffer.file()).expect("buffer should retain its file");
                assert_eq!(buffer.remote_id(), buffer_id);
                assert_eq!(file.path.as_ref(), rel_path(&format!("{next}/file.txt")));
            });
            assert_tree(
                &outline_panel,
                &project,
                &format!(
                    indoc!(
                        "
                        root/
                          {next}/
                            file.txt
                          z.txt  <==== selected"
                    ),
                    next = next,
                ),
                cx,
            );
            assert_directory_paths(&outline_panel, worktree_id, &["", next], &[], cx);
        }
    }

    #[gpui::test]
    async fn test_ignored_directory_expansion_does_not_scan_worktree(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                ".gitignore": "ignored\n",
                "ignored": {
                    "a": { "deep": { "open.txt": "first\n" } },
                    "b": { "open.txt": "second\n" },
                    "c": { "unopened.txt": "unopened\n" }
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;
        update_outline_panel_settings(cx, |settings| {
            settings.multi_buffer_hide_symbols = Some(true);
        });
        let buffer_a = open_buffer(&project, path!("/root/ignored/a/deep/open.txt"), cx).await;
        let buffer_b = open_buffer(&project, path!("/root/ignored/b/open.txt"), cx).await;
        let worktree = buffer_a.read_with(cx, |buffer, _| {
            File::from_dyn(buffer.file()).unwrap().worktree.clone()
        });
        let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());
        let buffer_a_id = buffer_a.read_with(cx, |buffer, _| buffer.remote_id());
        let editor = add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer_a, Vec::new()), (&buffer_b, Vec::new())],
            cx,
        );
        wait_for_outline_tasks(&outline_panel, cx).await;
        let scan_id = worktree.read_with(cx, |worktree, _| worktree.scan_id());
        let read_dir_calls = fs.read_dir_call_count();
        let metadata_calls = fs.metadata_call_count();

        for auto_fold_dirs in [false, true] {
            update_outline_panel_settings(cx, |settings| {
                settings.auto_fold_dirs = Some(auto_fold_dirs);
            });
            wait_for_outline_tasks(&outline_panel, cx).await;
            let expanded_tree = if auto_fold_dirs {
                indoc!(
                    "
                    root/
                      ignored/
                        a/deep/  <==== selected
                          open.txt
                        b/
                          open.txt"
                )
            } else {
                indoc!(
                    "
                    root/
                      ignored/
                        a/
                          deep/  <==== selected
                            open.txt
                        b/
                          open.txt"
                )
            };
            select_directory(&outline_panel, worktree_id, rel_path("ignored/a/deep"), cx);
            for keyboard in [true, false] {
                outline_panel.update_in(cx, |panel, window, cx| {
                    if keyboard {
                        panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
                    } else {
                        let entry = panel.selected_entry().unwrap().clone();
                        panel.toggle_expanded(&entry, window, cx);
                    }
                });
                wait_for_outline_tasks(&outline_panel, cx).await;
                assert!(
                    editor.read_with(cx, |editor, cx| editor.is_buffer_folded(buffer_a_id, cx))
                );
                outline_panel.update_in(cx, |panel, window, cx| {
                    if keyboard {
                        panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
                    } else {
                        let entry = panel.selected_entry().unwrap().clone();
                        panel.toggle_expanded(&entry, window, cx);
                    }
                });
                wait_for_outline_tasks(&outline_panel, cx).await;
                assert!(
                    !editor.read_with(cx, |editor, cx| editor.is_buffer_folded(buffer_a_id, cx))
                );
                assert_tree(&outline_panel, &project, expanded_tree, cx);
                assert_eq!(
                    worktree.read_with(cx, |worktree, _| worktree.scan_id()),
                    scan_id
                );
                assert_eq!(fs.read_dir_call_count(), read_dir_calls);
                assert_eq!(fs.metadata_call_count(), metadata_calls);
            }

            outline_panel.update_in(cx, |panel, window, cx| {
                panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
            });
            wait_for_outline_tasks(&outline_panel, cx).await;
            select_in_buffer(&editor, buffer_a_id, cx);
            wait_for_outline_tasks(&outline_panel, cx).await;
            let revealed_tree = if auto_fold_dirs {
                indoc!(
                    "
                    root/
                      ignored/
                        a/deep/
                          open.txt  <==== selected
                        b/
                          open.txt"
                )
            } else {
                indoc!(
                    "
                    root/
                      ignored/
                        a/
                          deep/
                            open.txt  <==== selected
                        b/
                          open.txt"
                )
            };
            assert_tree(&outline_panel, &project, revealed_tree, cx);
            assert_eq!(
                worktree.read_with(cx, |worktree, _| worktree.scan_id()),
                scan_id
            );
            assert_eq!(fs.read_dir_call_count(), read_dir_calls);
            assert_eq!(fs.metadata_call_count(), metadata_calls);
            worktree.read_with(cx, |worktree, _| {
                assert_eq!(
                    worktree
                        .entry_for_path(rel_path("ignored/c"))
                        .map(|entry| entry.kind),
                    Some(worktree::EntryKind::UnloadedDir),
                );
                assert_eq!(
                    worktree.entry_for_path(rel_path("ignored/c/unopened.txt")),
                    None
                );
            });
            outline_panel.read_with(cx, |panel, _| {
                assert_eq!(
                    panel
                        .fs_entries
                        .iter()
                        .map(|entry| match entry {
                            FsEntry::Directory(directory) =>
                                (directory.entry.path.as_ref(), directory.entry.is_ignored),
                            FsEntry::File(file) =>
                                (file.entry.path.as_ref(), file.entry.is_ignored),
                            FsEntry::ExternalFile(_) =>
                                panic!("indexed ignored files should not be external"),
                        })
                        .collect::<Vec<_>>(),
                    vec![
                        (rel_path(""), false),
                        (rel_path("ignored"), true),
                        (rel_path("ignored/a"), true),
                        (rel_path("ignored/a/deep"), true),
                        (rel_path("ignored/a/deep/open.txt"), true),
                        (rel_path("ignored/b"), true),
                        (rel_path("ignored/b/open.txt"), true),
                    ],
                );
            });
        }
    }

    #[gpui::test]
    async fn test_selected_folded_directory_survives_git_metadata_changes(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                ".git": {},
                "a": { "b": { "file.txt": "modified\n" } },
                "z.txt": "unchanged\n"
            }),
        )
        .await;
        let base_contents = [
            ("a/b/file.txt", "base\n".to_string()),
            ("z.txt", "unchanged\n".to_string()),
        ];
        let dot_git = Path::new(path!("/root/.git"));
        fs.set_head_for_repo(dot_git, &base_contents, "deadbeef");
        fs.set_index_for_repo(dot_git, &base_contents);
        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;
        update_outline_panel_settings(cx, |settings| {
            settings.auto_fold_dirs = Some(true);
            settings.multi_buffer_hide_symbols = Some(true);
        });
        let buffer = open_buffer(&project, path!("/root/a/b/file.txt"), cx).await;
        let buffer_z = open_buffer(&project, path!("/root/z.txt"), cx).await;
        let worktree_id = buffer.read_with(cx, |buffer, cx| {
            File::from_dyn(buffer.file())
                .expect("buffer should have a file")
                .worktree
                .read(cx)
                .id()
        });
        add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer, Vec::new()), (&buffer_z, Vec::new())],
            cx,
        );
        flush_outline_tasks(cx);
        select_directory(&outline_panel, worktree_id, rel_path("a/b"), cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  a/b/  <==== selected
                    file.txt
                  z.txt"
            ),
            cx,
        );
        let metadata_before = outline_panel.read_with(cx, |panel, _| {
            let Some(PanelEntry::FoldedDirs(group)) = panel.selected_entry() else {
                panic!("folded directory should be selected");
            };
            assert_eq!(
                group
                    .entries
                    .iter()
                    .map(|entry| (
                        entry.path.as_ref(),
                        entry.git_summary.worktree.modified,
                        entry.is_ignored
                    ))
                    .collect::<Vec<_>>(),
                vec![(rel_path("a"), 1, false), (rel_path("a/b"), 1, false)],
            );
            group
                .entries
                .iter()
                .map(|entry| entry.git_summary)
                .collect::<Vec<_>>()
        });

        fs.remove_dir(
            Path::new(path!("/root/a")),
            project::RemoveOptions {
                recursive: true,
                ignore_if_not_exists: false,
            },
        )
        .await
        .expect("ancestor deletion should succeed");
        cx.run_until_parked();
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
        flush_outline_tasks(cx);
        buffer.read_with(cx, |buffer, cx| {
            let file = File::from_dyn(buffer.file()).expect("buffer should retain its file");
            assert_eq!(file.disk_state, DiskState::Deleted);
            assert_eq!(
                file.worktree
                    .read(cx)
                    .entry_for_path(rel_path("a"))
                    .map(|entry| entry.kind),
                None
            );
        });
        let metadata_after = outline_panel.read_with(cx, |panel, _| {
            let group = panel
                .cached_entries
                .iter()
                .find_map(|cached| match &cached.entry {
                    PanelEntry::FoldedDirs(group)
                        if group.worktree_id == worktree_id
                            && group
                                .entries
                                .last()
                                .is_some_and(|entry| entry.path.as_ref() == rel_path("a/b")) =>
                    {
                        Some(group)
                    }
                    _ => None,
                })
                .expect("folded directory should retain its row");
            assert_eq!(
                group
                    .entries
                    .iter()
                    .map(|entry| entry.path.as_ref())
                    .collect::<Vec<_>>(),
                vec![rel_path("a"), rel_path("a/b")],
            );
            group
                .entries
                .iter()
                .map(|entry| entry.git_summary)
                .collect::<Vec<_>>()
        });
        assert_ne!(metadata_before, metadata_after);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  a/b/  <==== selected
                    ~~file.txt~~
                  z.txt"
            ),
            cx,
        );
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.select_next(&SelectNext, window, cx);
        });
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  a/b/
                    ~~file.txt~~  <==== selected
                  z.txt"
            ),
            cx,
        );
    }

    #[gpui::test]
    async fn test_deleted_file_nests_under_existing_parent(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                "dir": {
                    "a.txt": "hello world",
                    "b.txt": "hello there",
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;

        let buffer_a = open_buffer(&project, path!("/root/dir/a.txt"), cx).await;
        let buffer_b = open_buffer(&project, path!("/root/dir/b.txt"), cx).await;
        add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer_a, Vec::new()), (&buffer_b, Vec::new())],
            cx,
        );
        flush_outline_tasks(cx);

        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/
                    a.txt
                    b.txt"
            ),
            cx,
        );

        fs.remove_file(Path::new(path!("/root/dir/a.txt")), Default::default())
            .await
            .unwrap();
        flush_outline_tasks(cx);

        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/
                    ~~a.txt~~
                    b.txt"
            ),
            cx,
        );
    }

    #[gpui::test]
    async fn test_directory_collapse_survives_ancestor_changes(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                "keep.txt": "hello there",
                "dir": { "sub": { "a.txt": "hello world" } }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;
        update_outline_panel_settings(cx, |settings| {
            settings.auto_fold_dirs = Some(false);
        });

        let buffer_keep = open_buffer(&project, path!("/root/keep.txt"), cx).await;
        let buffer_a = open_buffer(&project, path!("/root/dir/sub/a.txt"), cx).await;
        let worktree_id = buffer_a.read_with(cx, |buffer, cx| {
            File::from_dyn(buffer.file())
                .expect("buffer should have a file")
                .worktree
                .read(cx)
                .id()
        });
        add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer_keep, Vec::new()), (&buffer_a, Vec::new())],
            cx,
        );
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/
                    sub/
                      a.txt
                  keep.txt"
            ),
            cx,
        );

        fs.remove_file(
            Path::new(path!("/root/dir/sub/a.txt")),
            project::RemoveOptions::default(),
        )
        .await
        .expect("file deletion should succeed");
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/
                    sub/
                      ~~a.txt~~
                  keep.txt"
            ),
            cx,
        );

        select_directory(&outline_panel, worktree_id, rel_path("dir"), cx);
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/  <==== selected
                  keep.txt"
            ),
            cx,
        );

        fs.remove_dir(
            Path::new(path!("/root/dir")),
            project::RemoveOptions {
                recursive: true,
                ignore_if_not_exists: false,
            },
        )
        .await
        .expect("ancestor deletion should succeed");
        flush_outline_tasks(cx);
        buffer_a.read_with(cx, |buffer, cx| {
            let file = File::from_dyn(buffer.file()).expect("buffer should retain its file");
            assert_eq!(file.disk_state, DiskState::Deleted);
            assert_eq!(
                file.worktree
                    .read(cx)
                    .entry_for_path(rel_path("dir"))
                    .map(|entry| entry.kind),
                None,
            );
        });
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/  <==== selected
                  keep.txt"
            ),
            cx,
        );

        fs.create_dir(Path::new(path!("/root/dir/sub")))
            .await
            .expect("ancestor recreation should succeed");
        flush_outline_tasks(cx);
        buffer_a.read_with(cx, |buffer, cx| {
            let file = File::from_dyn(buffer.file()).expect("buffer should retain its file");
            assert_eq!(
                file.worktree
                    .read(cx)
                    .entry_for_path(rel_path("dir/sub"))
                    .map(|entry| entry.is_dir()),
                Some(true),
            );
        });
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/  <==== selected
                  keep.txt"
            ),
            cx,
        );

        fs.insert_file(path!("/root/dir/sub/a.txt"), b"hello world".to_vec())
            .await;
        flush_outline_tasks(cx);
        buffer_a.read_with(cx, |buffer, _| {
            let file = File::from_dyn(buffer.file()).expect("buffer should retain its file");
            assert!(matches!(file.disk_state, DiskState::Present { .. }));
        });
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/  <==== selected
                  keep.txt"
            ),
            cx,
        );

        outline_panel.update_in(cx, |panel, window, cx| {
            panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
        });
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/  <==== selected
                    sub/
                      a.txt
                  keep.txt"
            ),
            cx,
        );
    }

    #[gpui::test]
    async fn test_only_deleted_files_in_multiple_worktrees(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                "one": { "é": { "子": { "a.txt": "one" } } },
                "two": { "é": { "子": { "a.txt": "two" } } }
            }),
        )
        .await;
        let project = Project::test(
            fs.clone(),
            [Path::new(path!("/root/one")), Path::new(path!("/root/two"))],
            cx,
        )
        .await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;
        update_outline_panel_settings(cx, |settings| {
            settings.auto_fold_dirs = Some(false);
        });

        let buffer_one = open_buffer(&project, path!("/root/one/é/子/a.txt"), cx).await;
        let buffer_two = open_buffer(&project, path!("/root/two/é/子/a.txt"), cx).await;
        let worktree_one = buffer_one.read_with(cx, |buffer, cx| {
            File::from_dyn(buffer.file())
                .expect("buffer should have a file")
                .worktree
                .read(cx)
                .id()
        });
        add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer_two, Vec::new()), (&buffer_one, Vec::new())],
            cx,
        );
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                one/
                  é/
                    子/
                      a.txt
                two/
                  é/
                    子/
                      a.txt"
            ),
            cx,
        );

        for path in [path!("/root/one/é"), path!("/root/two/é")] {
            fs.remove_dir(
                Path::new(path),
                project::RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: false,
                },
            )
            .await
            .expect("directory deletion should succeed");
        }
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                one/
                  é/
                    子/
                      ~~a.txt~~
                two/
                  é/
                    子/
                      ~~a.txt~~"
            ),
            cx,
        );

        select_directory(&outline_panel, worktree_one, rel_path("é"), cx);
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                one/
                  é/  <==== selected
                two/
                  é/
                    子/
                      ~~a.txt~~"
            ),
            cx,
        );
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
        });
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                one/
                  é/  <==== selected
                    子/
                      ~~a.txt~~
                two/
                  é/
                    子/
                      ~~a.txt~~"
            ),
            cx,
        );
    }

    #[gpui::test]
    async fn test_untitled_buffer_still_external(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/root"), json!({ "existing.txt": "hello" }))
            .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;

        let untitled_buffer = project.update(cx, |project, cx| {
            project.create_local_buffer("untitled content", None, true, cx)
        });
        add_multi_buffer_editor(&workspace, &project, &[(&untitled_buffer, Vec::new())], cx);
        flush_outline_tasks(cx);

        assert_tree(&outline_panel, &project, "external: untitled", cx);
    }

    #[gpui::test]
    async fn test_auto_reveal_expands_ancestor_of_deleted_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                "dir": {
                    "a.rs": indoc!(
                        "
                        pub fn foo() {
                            let x = 1;
                        }
                        "
                    ),
                    "b.rs": "pub fn bar() {}\n",
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;

        let buffer_a = open_buffer(&project, path!("/root/dir/a.rs"), cx).await;
        let buffer_b = open_buffer(&project, path!("/root/dir/b.rs"), cx).await;
        let editor = add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer_a, Vec::new()), (&buffer_b, Vec::new())],
            cx,
        );
        flush_outline_tasks(cx);

        fs.remove_file(
            Path::new(path!("/root/dir/a.rs")),
            project::RemoveOptions::default(),
        )
        .await
        .unwrap();
        flush_outline_tasks(cx);

        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/
                    ~~a.rs~~
                        outline: pub fn foo  <==== selected
                    b.rs
                        outline: pub fn bar"
            ),
            cx,
        );

        let buffer_a_id = buffer_a.read_with(cx, |buffer, _cx| buffer.remote_id());
        let buffer_b_id = buffer_b.read_with(cx, |buffer, _cx| buffer.remote_id());
        select_in_buffer(&editor, buffer_b_id, cx);

        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/
                    ~~a.rs~~
                        outline: pub fn foo
                    b.rs
                        outline: pub fn bar  <==== selected"
            ),
            cx,
        );

        let worktree_id = buffer_a.read_with(cx, |buffer, cx| {
            File::from_dyn(buffer.file())
                .expect("buffer should have a file")
                .worktree
                .read(cx)
                .id()
        });
        select_directory(&outline_panel, worktree_id, rel_path("dir"), cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        flush_outline_tasks(cx);

        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/  <==== selected"
            ),
            cx,
        );

        select_in_buffer(&editor, buffer_a_id, cx);

        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/
                    ~~a.rs~~  <==== selected
                    b.rs"
            ),
            cx,
        );
    }

    #[gpui::test]
    async fn test_open_deleted_file_entry_navigates_editor(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                "dir": {
                    "a.txt": "hello world",
                    "b.txt": "hello there",
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;

        let buffer_a = open_buffer(&project, path!("/root/dir/a.txt"), cx).await;
        let buffer_b = open_buffer(&project, path!("/root/dir/b.txt"), cx).await;
        let editor = add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer_a, Vec::new()), (&buffer_b, Vec::new())],
            cx,
        );
        flush_outline_tasks(cx);

        let buffer_b_id = buffer_b.read_with(cx, |buffer, _cx| buffer.remote_id());
        select_in_buffer(&editor, buffer_b_id, cx);

        fs.remove_file(Path::new(path!("/root/dir/a.txt")), Default::default())
            .await
            .unwrap();
        flush_outline_tasks(cx);

        let deleted_row = outline_panel.read_with(cx, |outline_panel, _cx| {
            outline_panel
                .cached_entries
                .iter()
                .find_map(|cached_entry| match &cached_entry.entry {
                    PanelEntry::Fs(FsEntry::File(file))
                        if file.entry.path.as_ref() == rel_path("dir/a.txt") =>
                    {
                        Some(cached_entry.entry.clone())
                    }
                    _ => None,
                })
                .expect("the deleted file should have a row")
        });
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.select_entry(deleted_row, true, window, cx);
        });
        editor.update_in(cx, |editor, window, cx| {
            assert!(
                !editor.focus_handle(cx).is_focused(window),
                "sanity check: selecting a row focuses the panel, not the editor"
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.open_selected_entry(&OpenSelectedEntry, window, cx);
        });
        cx.run_until_parked();

        let buffer_a_id = buffer_a.read_with(cx, |buffer, _cx| buffer.remote_id());
        editor.update_in(cx, |editor, window, cx| {
            assert_eq!(
                editor
                    .scroll_manager
                    .shared_scroll_anchor(cx)
                    .scroll_anchor
                    .anchor
                    .buffer_id(),
                Some(buffer_a_id),
                "opening the deleted file's row should scroll the editor to its excerpt"
            );
            assert!(editor.focus_handle(cx).is_focused(window));
        });

        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  dir/
                    ~~a.txt~~  <==== selected
                    b.txt"
            ),
            cx,
        );
    }

    #[gpui::test]
    async fn test_buffer_for_nonexistent_path_not_treated_as_deleted(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/root"), json!({ "existing.txt": "hello" }))
            .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;

        let new_buffer = project
            .update(cx, |project, cx| {
                let path = project
                    .find_project_path(path!("/root/missing_dir/new.txt"), cx)
                    .expect("path inside the worktree should resolve");
                project.open_buffer(path, cx)
            })
            .await
            .unwrap();
        new_buffer.read_with(cx, |buffer, _cx| {
            assert_eq!(
                buffer.file().map(|file| file.disk_state()),
                Some(language::DiskState::New),
                "sanity check: a buffer opened on a nonexistent path should be DiskState::New"
            );
        });
        add_multi_buffer_editor(&workspace, &project, &[(&new_buffer, Vec::new())], cx);
        flush_outline_tasks(cx);

        assert_tree(&outline_panel, &project, "external: new.txt", cx);
    }

    #[gpui::test]
    async fn test_deleted_file_ancestor_replaced_by_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                "éx": "hello there",
                "é": {
                    "a.rs": indoc!(
                        "
                        pub fn foo() {
                            let x = 1;
                        }
                        "
                    )
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;
        update_outline_panel_settings(cx, |settings| {
            settings.auto_fold_dirs = Some(false);
        });

        let buffer_keep = open_buffer(&project, path!("/root/éx"), cx).await;
        let buffer_a = open_buffer(&project, path!("/root/é/a.rs"), cx).await;
        add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer_keep, Vec::new()), (&buffer_a, Vec::new())],
            cx,
        );
        flush_outline_tasks(cx);

        fs.remove_dir(
            Path::new(path!("/root/é")),
            project::RemoveOptions {
                recursive: true,
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();
        fs.insert_file(path!("/root/é"), b"not a directory anymore".to_vec())
            .await;
        flush_outline_tasks(cx);

        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  é/
                    ~~a.rs~~
                        outline: pub fn foo  <==== selected
                  éx"
            ),
            cx,
        );

        let buffer_replacement = open_buffer(&project, path!("/root/é"), cx).await;
        let editor = add_multi_buffer_editor(
            &workspace,
            &project,
            &[
                (&buffer_keep, Vec::new()),
                (&buffer_a, Vec::new()),
                (&buffer_replacement, Vec::new()),
            ],
            cx,
        );
        flush_outline_tasks(cx);

        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  é
                  é/
                    ~~a.rs~~
                        outline: pub fn foo
                  éx"
            ),
            cx,
        );

        let worktree_id = buffer_a.read_with(cx, |buffer, cx| {
            File::from_dyn(buffer.file())
                .expect("buffer should have a file")
                .worktree
                .read(cx)
                .id()
        });
        select_directory(&outline_panel, worktree_id, rel_path("é"), cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        flush_outline_tasks(cx);

        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  é
                  é/  <==== selected
                  éx"
            ),
            cx,
        );

        let buffer_a_id = buffer_a.read_with(cx, |buffer, _cx| buffer.remote_id());
        select_in_buffer(&editor, buffer_a_id, cx);

        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  é
                  é/
                    ~~a.rs~~  <==== selected
                  éx"
            ),
            cx,
        );
    }

    #[gpui::test]
    async fn test_folded_directories_survive_ancestor_changes(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                "keep.txt": "hello there",
                "a": {
                    "b": {
                        "c": {
                            "d": {
                                "gone.txt": "hello world"
                            }
                        }
                    }
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;
        update_outline_panel_settings(cx, |settings| {
            settings.auto_fold_dirs = Some(true);
        });

        let buffer_keep = open_buffer(&project, path!("/root/keep.txt"), cx).await;
        let buffer_gone = open_buffer(&project, path!("/root/a/b/c/d/gone.txt"), cx).await;
        add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer_keep, Vec::new()), (&buffer_gone, Vec::new())],
            cx,
        );
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  a/b/c/d/
                    gone.txt
                  keep.txt"
            ),
            cx,
        );

        fs.remove_dir(
            Path::new(path!("/root/a/b/c")),
            project::RemoveOptions {
                recursive: true,
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();
        flush_outline_tasks(cx);

        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  a/b/c/d/
                    ~~gone.txt~~
                  keep.txt"
            ),
            cx,
        );
        fs.create_dir(Path::new(path!("/root/a/b/c/d")))
            .await
            .expect("ancestor recreation should succeed");
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  a/b/c/d/
                    ~~gone.txt~~
                  keep.txt"
            ),
            cx,
        );

        let deleted_file_row = outline_panel.read_with(cx, |outline_panel, _cx| {
            outline_panel
                .cached_entries
                .iter()
                .find_map(|cached_entry| match &cached_entry.entry {
                    PanelEntry::Fs(FsEntry::File(file))
                        if file.entry.path.as_ref() == rel_path("a/b/c/d/gone.txt") =>
                    {
                        Some(cached_entry.entry.clone())
                    }
                    _ => None,
                })
                .expect("the deleted file should have a row")
        });
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.select_entry(deleted_file_row, true, window, cx);
        });

        for expected in [
            indoc!(
                "
                root/
                  a/b/c/d/  <==== selected
                    ~~gone.txt~~
                  keep.txt"
            ),
            indoc!(
                "
                root/  <==== selected
                  a/b/c/d/
                    ~~gone.txt~~
                  keep.txt"
            ),
        ] {
            outline_panel.update_in(cx, |outline_panel, window, cx| {
                outline_panel.select_parent(&SelectParent, window, cx);
            });
            assert_tree(&outline_panel, &project, expected, cx);
        }

        let worktree_id = buffer_gone.read_with(cx, |buffer, cx| {
            File::from_dyn(buffer.file())
                .expect("buffer should have a file")
                .worktree
                .read(cx)
                .id()
        });
        select_directory(&outline_panel, worktree_id, rel_path("a/b/c/d"), cx);
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  a/b/c/d/  <==== selected
                  keep.txt"
            ),
            cx,
        );
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
        });
        flush_outline_tasks(cx);
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  a/b/c/d/  <==== selected
                    ~~gone.txt~~
                  keep.txt"
            ),
            cx,
        );
        for expected in [
            indoc!(
                "
                root/
                  a/b/c/d/  <==== selected
                  keep.txt"
            ),
            indoc!(
                "
                root/
                  a/b/c/d/  <==== selected
                    ~~gone.txt~~
                  keep.txt"
            ),
        ] {
            outline_panel.update_in(cx, |panel, window, cx| {
                let entry = panel
                    .selected_entry()
                    .expect("folded directory should be selected")
                    .clone();
                panel.toggle_expanded(&entry, window, cx);
            });
            flush_outline_tasks(cx);
            assert_tree(&outline_panel, &project, expected, cx);
        }
    }

    #[gpui::test]
    async fn test_scan_excluded_modified_file_stays_external(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.worktree.file_scan_exclusions =
                        Some(settings::SplicingVec::from(vec![
                            "**/excluded.txt".to_string(),
                        ]));
                });
            });
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                ".git": {},
                "keep.txt": "hello there",
                "excluded.txt": "new",
            }),
        )
        .await;
        let dot_git = Path::new(path!("/root/.git"));
        fs.set_head_for_repo(
            dot_git,
            &[
                ("keep.txt", "hello there".into()),
                ("excluded.txt", "old".into()),
            ],
            "deadbeef",
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;

        let buffer_keep = open_buffer(&project, path!("/root/keep.txt"), cx).await;
        let buffer_excluded = open_buffer(&project, path!("/root/excluded.txt"), cx).await;
        buffer_excluded.read_with(cx, |buffer, _cx| {
            let file = File::from_dyn(buffer.file()).expect("excluded buffer should have a file");
            assert_eq!(
                file.project_entry_id(),
                None,
                "sanity check: a scan-excluded file has no worktree entry"
            );
            assert!(
                matches!(file.disk_state, language::DiskState::Present { .. }),
                "sanity check: a scan-excluded file is present on disk, got {:?}",
                file.disk_state
            );
        });

        add_diff_editor(
            &workspace,
            &project,
            &[
                (&buffer_keep, "previous contents\n"),
                (&buffer_excluded, "old\n"),
            ],
            cx,
        );
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                external: excluded.txt
                root/
                  keep.txt"
            ),
            cx,
        );
    }

    #[gpui::test]
    async fn test_diff_deletions_group_under_scanned_and_excluded_parents(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.worktree.file_scan_exclusions =
                        Some(settings::SplicingVec::from(vec![
                            "**/excluded".to_string(),
                            "**/missing".to_string(),
                        ]));
                });
            });
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                ".git": {},
                "scanned": { "keep.txt": "still here" },
                "excluded": { "keep.txt": "still here" }
            }),
        )
        .await;
        let dot_git = Path::new(path!("/root/.git"));
        fs.set_branch_name(dot_git, Some("feature"));
        fs.insert_branches(dot_git, &["main"]);
        fs.set_head_for_repo(
            dot_git,
            &[
                ("scanned/keep.txt", "still here".to_string()),
                ("excluded/keep.txt", "still here".to_string()),
            ],
            "deadbeef",
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;
        update_outline_panel_settings(cx, |settings| {
            settings.auto_fold_dirs = Some(false);
        });

        let buffer_scanned = open_buffer(&project, path!("/root/scanned/gone.txt"), cx).await;
        let buffer_excluded = open_buffer(&project, path!("/root/excluded/gone.txt"), cx).await;
        let buffer_missing = open_buffer(&project, path!("/root/missing/gone.txt"), cx).await;
        for buffer in [&buffer_scanned, &buffer_excluded, &buffer_missing] {
            buffer.read_with(cx, |buffer, cx| {
                let file = File::from_dyn(buffer.file()).expect("buffer should have a file");
                assert_eq!(file.disk_state, DiskState::New);
                assert_eq!(file.project_entry_id(), None);
                assert_eq!(
                    project
                        .read(cx)
                        .git_store()
                        .read(cx)
                        .display_status_for_buffer_id(buffer.remote_id(), cx),
                    None,
                );
            });
        }
        buffer_excluded.read_with(cx, |buffer, cx| {
            let file = File::from_dyn(buffer.file()).expect("buffer should have a file");
            let worktree = file.worktree.read(cx);
            assert_eq!(
                worktree
                    .entry_for_path(rel_path("excluded"))
                    .map(|entry| entry.kind),
                None
            );
            assert_eq!(
                worktree
                    .entry_for_path(rel_path("missing"))
                    .map(|entry| entry.kind),
                None
            );
            assert_eq!(
                worktree
                    .entry_for_path(rel_path("scanned"))
                    .map(|entry| entry.is_dir()),
                Some(true),
            );
        });
        assert_eq!(
            fs.metadata(Path::new(path!("/root/excluded")))
                .await
                .expect("metadata should be readable")
                .map(|metadata| metadata.is_dir),
            Some(true),
        );
        assert_eq!(
            fs.metadata(Path::new(path!("/root/missing")))
                .await
                .expect("metadata should be readable")
                .map(|metadata| metadata.is_dir),
            None,
        );
        add_diff_editor(
            &workspace,
            &project,
            &[
                (&buffer_scanned, "scanned base\n"),
                (&buffer_excluded, "excluded base\n"),
                (&buffer_missing, "missing base\n"),
            ],
            cx,
        );
        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                root/
                  excluded/
                    ~~gone.txt~~
                  missing/
                    ~~gone.txt~~
                  scanned/
                    ~~gone.txt~~"
            ),
            cx,
        );
    }

    #[gpui::test]
    async fn test_auto_reveal_ignores_external_file_on_nonexistent_path(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                "dir": {
                    "real.rs": "pub fn real() {}\n",
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/root"))], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (workspace, outline_panel, mut cx) = active_outline_panel(&project, cx).await;
        let cx = &mut cx;

        let buffer_real = open_buffer(&project, path!("/root/dir/real.rs"), cx).await;
        let buffer_new = open_buffer(&project, path!("/root/dir/never_existed.rs"), cx).await;
        buffer_new.update(cx, |buffer, cx| {
            buffer.edit(
                [(
                    0..0,
                    indoc!(
                        "
                    pub fn phantom() {
                        let x = 1;
                    }
                    "
                    ),
                )],
                None,
                cx,
            );
        });
        let editor = add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer_real, Vec::new()), (&buffer_new, Vec::new())],
            cx,
        );
        flush_outline_tasks(cx);

        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                external: never_existed.rs
                    outline: pub fn phantom  <==== selected
                root/
                  dir/
                    real.rs
                        outline: pub fn real"
            ),
            cx,
        );

        let worktree_id = buffer_real.read_with(cx, |buffer, cx| {
            File::from_dyn(buffer.file())
                .expect("buffer should have a file")
                .worktree
                .read(cx)
                .id()
        });
        select_directory(&outline_panel, worktree_id, rel_path("dir"), cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        flush_outline_tasks(cx);

        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                external: never_existed.rs
                    outline: pub fn phantom
                root/
                  dir/  <==== selected"
            ),
            cx,
        );

        let buffer_new_id = buffer_new.read_with(cx, |buffer, _cx| buffer.remote_id());
        select_in_buffer(&editor, buffer_new_id, cx);

        assert_tree(
            &outline_panel,
            &project,
            indoc!(
                "
                external: never_existed.rs
                    outline: pub fn phantom  <==== selected
                root/
                  dir/"
            ),
            cx,
        );
    }

    #[gpui::test]
    async fn test_navigating_in_singleton(cx: &mut TestAppContext) {
        init_test(cx);

        let root = path!("/root");
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            root,
            json!({
                "src": {
                    "lib.rs": indoc!("
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct OutlineEntryExcerpt {
    id: ExcerptId,
    buffer_id: BufferId,
    range: ExcerptRange<language::Anchor>,
}"),
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [Path::new(root)], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);
        cx.update(|window, cx| {
            outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.set_active(true, window, cx)
            });
        });

        let _editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/root/src/lib.rs")),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .await
            .expect("Failed to open Rust source file")
            .downcast::<Editor>()
            .expect("Should open an editor for Rust source file");

        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct OutlineEntryExcerpt
  outline: id
  outline: buffer_id
  outline: range"
                )
            );
        });

        cx.update(|window, cx| {
            outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.select_next(&SelectNext, window, cx);
            });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct OutlineEntryExcerpt  <==== selected
  outline: id
  outline: buffer_id
  outline: range"
                )
            );
        });

        cx.update(|window, cx| {
            outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.select_next(&SelectNext, window, cx);
            });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct OutlineEntryExcerpt
  outline: id  <==== selected
  outline: buffer_id
  outline: range"
                )
            );
        });

        cx.update(|window, cx| {
            outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.select_next(&SelectNext, window, cx);
            });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct OutlineEntryExcerpt
  outline: id
  outline: buffer_id  <==== selected
  outline: range"
                )
            );
        });

        cx.update(|window, cx| {
            outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.select_next(&SelectNext, window, cx);
            });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct OutlineEntryExcerpt
  outline: id
  outline: buffer_id
  outline: range  <==== selected"
                )
            );
        });

        cx.update(|window, cx| {
            outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.select_next(&SelectNext, window, cx);
            });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct OutlineEntryExcerpt  <==== selected
  outline: id
  outline: buffer_id
  outline: range"
                )
            );
        });

        cx.update(|window, cx| {
            outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.select_previous(&SelectPrevious, window, cx);
            });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct OutlineEntryExcerpt
  outline: id
  outline: buffer_id
  outline: range  <==== selected"
                )
            );
        });

        cx.update(|window, cx| {
            outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.select_previous(&SelectPrevious, window, cx);
            });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct OutlineEntryExcerpt
  outline: id
  outline: buffer_id  <==== selected
  outline: range"
                )
            );
        });

        cx.update(|window, cx| {
            outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.select_previous(&SelectPrevious, window, cx);
            });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct OutlineEntryExcerpt
  outline: id  <==== selected
  outline: buffer_id
  outline: range"
                )
            );
        });

        cx.update(|window, cx| {
            outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.select_previous(&SelectPrevious, window, cx);
            });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct OutlineEntryExcerpt  <==== selected
  outline: id
  outline: buffer_id
  outline: range"
                )
            );
        });

        cx.update(|window, cx| {
            outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.select_previous(&SelectPrevious, window, cx);
            });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct OutlineEntryExcerpt
  outline: id
  outline: buffer_id
  outline: range  <==== selected"
                )
            );
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_frontend_repo_structure(cx: &mut TestAppContext) {
        init_test(cx);

        let root = path!("/frontend-project");
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            root,
            json!({
                "public": {
                    "lottie": {
                        "syntax-tree.json": r#"{ "something": "static" }"#
                    }
                },
                "src": {
                    "app": {
                        "(site)": {
                            "(about)": {
                                "jobs": {
                                    "[slug]": {
                                        "page.tsx": r#"static"#
                                    }
                                }
                            },
                            "(blog)": {
                                "post": {
                                    "[slug]": {
                                        "page.tsx": r#"static"#
                                    }
                                }
                            },
                        }
                    },
                    "components": {
                        "ErrorBoundary.tsx": r#"static"#,
                    }
                }

            }),
        )
        .await;
        let project = Project::test(fs.clone(), [Path::new(root)], cx).await;
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        workspace.update_in(cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(
                workspace,
                &workspace::DeploySearch::default(),
                window,
                cx,
            )
        });
        let search_view = workspace.update_in(cx, |workspace, _window, cx| {
            workspace
                .active_pane()
                .read(cx)
                .items()
                .find_map(|item| item.downcast::<ProjectSearchView>())
                .expect("Project search view expected to appear after new search event trigger")
        });

        let query = "static";
        perform_project_search(&search_view, query, cx);
        search_view.update(cx, |search_view, cx| {
            search_view
                .results_editor()
                .update(cx, |results_editor, cx| {
                    assert_eq!(
                        results_editor.display_text(cx).match_indices(query).count(),
                        4
                    );
                });
        });

        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                r#"frontend-project/
  public/lottie/
    syntax-tree.json
      search: { "something": "«static»" }  <==== selected
  src/
    app/(site)/
      (about)/jobs/[slug]/
        page.tsx
          search: «static»
      (blog)/post/[slug]/
        page.tsx
          search: «static»
    components/
      ErrorBoundary.tsx
        search: «static»"#
                    .to_string()
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            // Move to 5th element in the list, 3 items down.
            for _ in 0..2 {
                outline_panel.select_next(&SelectNext, window, cx);
            }
            outline_panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                r#"frontend-project/
  public/lottie/
    syntax-tree.json
      search: { "something": "«static»" }
  src/
    app/(site)/  <==== selected
    components/
      ErrorBoundary.tsx
        search: «static»"#
                    .to_string()
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            // Move to the next visible non-FS entry
            for _ in 0..3 {
                outline_panel.select_next(&SelectNext, window, cx);
            }
        });
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                r#"frontend-project/
  public/lottie/
    syntax-tree.json
      search: { "something": "«static»" }
  src/
    app/(site)/
    components/
      ErrorBoundary.tsx
        search: «static»  <==== selected"#
                    .to_string()
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel
                .active_editor()
                .expect("Should have an active editor")
                .update(cx, |editor, cx| {
                    editor.toggle_fold(&editor::actions::ToggleFold, window, cx)
                });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                r#"frontend-project/
  public/lottie/
    syntax-tree.json
      search: { "something": "«static»" }
  src/
    app/(site)/
    components/
      ErrorBoundary.tsx  <==== selected"#
                    .to_string()
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel
                .active_editor()
                .expect("Should have an active editor")
                .update(cx, |editor, cx| {
                    editor.toggle_fold(&editor::actions::ToggleFold, window, cx)
                });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                r#"frontend-project/
  public/lottie/
    syntax-tree.json
      search: { "something": "«static»" }
  src/
    app/(site)/
    components/
      ErrorBoundary.tsx  <==== selected
        search: «static»"#
                    .to_string()
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.collapse_all_entries(&CollapseAllEntries, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                r#"frontend-project/"#.to_string()
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.expand_all_entries(&ExpandAllEntries, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                r#"frontend-project/
  public/lottie/
    syntax-tree.json
      search: { "something": "«static»" }
  src/
    app/(site)/
      (about)/jobs/[slug]/
        page.tsx
          search: «static»
      (blog)/post/[slug]/
        page.tsx
          search: «static»
    components/
      ErrorBoundary.tsx  <==== selected
        search: «static»"#
                    .to_string()
            );
        });
    }

    async fn add_outline_panel(
        project: &Entity<Project>,
        cx: &mut TestAppContext,
    ) -> (WindowHandle<MultiWorkspace>, Entity<Workspace>) {
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        let workspace_weak = workspace.downgrade();
        let outline_panel = window
            .update(cx, |_, window, cx| {
                cx.spawn_in(window, async move |_this, cx| {
                    OutlinePanel::load(workspace_weak, cx.clone()).await
                })
            })
            .unwrap()
            .await
            .expect("Failed to load outline panel");

        window
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    workspace.add_panel(outline_panel, window, cx);
                });
            })
            .unwrap();
        (window, workspace)
    }

    fn outline_panel(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<OutlinePanel> {
        workspace.update_in(cx, |workspace, _window, cx| {
            workspace
                .panel::<OutlinePanel>(cx)
                .expect("no outline panel")
        })
    }

    async fn open_buffer(
        project: &Entity<Project>,
        abs_path: &str,
        cx: &mut VisualTestContext,
    ) -> Entity<language::Buffer> {
        project
            .update(cx, |project, cx| {
                let path = project.find_project_path(abs_path, cx).unwrap();
                project.open_buffer(path, cx)
            })
            .await
            .unwrap()
    }

    async fn rename_buffer(
        project: &Entity<Project>,
        buffer: &Entity<language::Buffer>,
        path: &RelPath,
        cx: &mut VisualTestContext,
    ) {
        let (entry_id, worktree_id) = buffer.read_with(cx, |buffer, cx| {
            let file = File::from_dyn(buffer.file()).expect("buffer should have a file");
            (
                file.project_entry_id()
                    .expect("file should have a worktree entry"),
                file.worktree.read(cx).id(),
            )
        });
        project
            .update(cx, |project, cx| {
                project.rename_entry(
                    entry_id,
                    ProjectPath {
                        worktree_id,
                        path: Arc::from(path),
                    },
                    cx,
                )
            })
            .await
            .expect("file rename should succeed");
        cx.run_until_parked();
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
    }

    fn add_multi_buffer_editor(
        workspace: &Entity<Workspace>,
        project: &Entity<Project>,
        excerpts: &[(&Entity<language::Buffer>, Vec<Range<language::Point>>)],
        cx: &mut VisualTestContext,
    ) -> Entity<Editor> {
        workspace.update_in(cx, |workspace, window, cx| {
            let multibuffer = cx.new(|cx| {
                let mut multibuffer = editor::MultiBuffer::new(language::Capability::ReadWrite);
                for (buffer, ranges) in excerpts {
                    let ranges = if ranges.is_empty() {
                        vec![language::Point::default()..buffer.read(cx).max_point()]
                    } else {
                        ranges.clone()
                    };
                    multibuffer.set_excerpts_for_buffer((*buffer).clone(), ranges, 0, cx);
                }
                multibuffer
            });
            let editor = cx
                .new(|cx| Editor::for_multibuffer(multibuffer, Some(project.clone()), window, cx));
            workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, window, cx);
            editor.update(cx, |editor, cx| {
                window.focus(&editor.focus_handle(cx), cx);
            });
            editor
        })
    }

    fn set_buffer_order(
        editor: &Entity<Editor>,
        buffers: &[&Entity<language::Buffer>],
        cx: &mut VisualTestContext,
    ) {
        editor.update_in(cx, |editor, window, cx| {
            editor.buffer().update(cx, |multibuffer, cx| {
                for (index, buffer) in buffers.iter().enumerate() {
                    let mut path = PathKey::for_buffer(buffer, cx);
                    path.sort_prefix = Some(index as u64);
                    let range = language::Point::default()..buffer.read(cx).max_point();
                    multibuffer.set_excerpts_for_path(path, (*buffer).clone(), [range], 0, cx);
                }
                assert_eq!(
                    multibuffer
                        .snapshot(cx)
                        .excerpts()
                        .map(|excerpt| excerpt.context.start.buffer_id)
                        .collect::<Vec<_>>(),
                    buffers
                        .iter()
                        .map(|buffer| buffer.read(cx).remote_id())
                        .collect::<Vec<_>>(),
                );
            });
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([language::Point::default()..language::Point::default()]);
            });
        });
        flush_outline_tasks(cx);
    }

    fn select_in_buffer(editor: &Entity<Editor>, buffer_id: BufferId, cx: &mut VisualTestContext) {
        let anchor = editor.read_with(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let excerpt = snapshot
                .excerpts_for_buffer(buffer_id)
                .next()
                .expect("buffer should have an excerpt in the multi-buffer");
            snapshot
                .anchor_in_excerpt(excerpt.context.start)
                .expect("the excerpt's start should resolve")
        });
        cx.update(|window, cx| {
            window.focus(&editor.focus_handle(cx), cx);
            editor.update(cx, |editor, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges(Some(anchor..anchor))
                });
            });
        });
        flush_outline_tasks(cx);
    }

    async fn wait_for_outline_tasks(
        outline_panel: &Entity<OutlinePanel>,
        cx: &mut VisualTestContext,
    ) {
        cx.run_until_parked();
        poll_fn(|task_context| {
            outline_panel.update(cx, |panel, _| {
                for task in [
                    &mut panel.fs_entries_update_task,
                    &mut panel.lsp_outline_refresh_task,
                    &mut panel.cached_entries_update_task,
                ]
                .into_iter()
                .chain(panel.outline_fetch_tasks.values_mut())
                {
                    if !task.is_ready() {
                        futures::ready!(task.poll_unpin(task_context));
                    }
                }
                if !panel.reveal_selection_task.is_ready() {
                    futures::ready!(panel.reveal_selection_task.poll_unpin(task_context))
                        .expect("selection reveal should complete");
                }
                Poll::Ready(())
            })
        })
        .await;
    }

    fn update_outline_panel_settings(
        cx: &mut VisualTestContext,
        update: impl FnOnce(&mut settings::OutlinePanelSettingsContent),
    ) {
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    update(settings.outline_panel.get_or_insert_default());
                });
            });
        });
    }

    fn assert_tree(
        outline_panel: &Entity<OutlinePanel>,
        project: &Entity<Project>,
        expected: &str,
        cx: &mut VisualTestContext,
    ) {
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                expected
            );
        });
    }

    fn mark_deleted(name: String, is_deleted: bool) -> String {
        if is_deleted {
            format!("~~{name}~~")
        } else {
            name
        }
    }

    fn display_entries(
        project: &Entity<Project>,
        multi_buffer_snapshot: &MultiBufferSnapshot,
        cached_entries: &[CachedEntry],
        selected_entry: Option<&PanelEntry>,
        cx: &mut App,
    ) -> String {
        let project = project.read(cx);
        let mut display_string = String::new();
        for entry in cached_entries {
            if matches!(entry.entry, PanelEntry::Outline(OutlineEntry::Excerpt(_))) {
                continue;
            }
            if !display_string.is_empty() {
                display_string += "\n";
            }
            for _ in 0..entry.depth {
                display_string += "  ";
            }
            display_string += &match &entry.entry {
                PanelEntry::Fs(entry) => match entry {
                    FsEntry::ExternalFile(external_file) => {
                        let name = multi_buffer_snapshot
                            .buffer_for_id(external_file.buffer_id)
                            .and_then(|buffer| buffer.file())
                            .map(|file| file.file_name(cx).to_string())
                            .unwrap_or_else(|| "untitled".to_string());
                        format!("external: {name}")
                    }
                    FsEntry::Directory(directory) => {
                        let path = if let Some(worktree) = project
                            .worktree_for_id(directory.worktree_id, cx)
                            .filter(|_| directory.entry.path.is_empty())
                        {
                            worktree
                                .read(cx)
                                .root_name()
                                .join(&directory.entry.path)
                                .as_unix_str()
                                .to_string()
                        } else {
                            directory
                                .entry
                                .path
                                .file_name()
                                .unwrap_or_default()
                                .to_string()
                        };
                        format!("{path}/")
                    }
                    FsEntry::File(file) => mark_deleted(
                        file.entry
                            .path
                            .file_name()
                            .map(|name| name.to_string())
                            .unwrap_or_default(),
                        file.is_deleted,
                    ),
                },
                PanelEntry::FoldedDirs(folded_dirs) => folded_dirs
                    .entries
                    .iter()
                    .filter_map(|dir| dir.path.file_name())
                    .map(|name| name.to_string() + "/")
                    .collect(),
                PanelEntry::Outline(outline_entry) => match outline_entry {
                    OutlineEntry::Excerpt(_) => continue,
                    OutlineEntry::Outline(outline_entry) => {
                        format!("outline: {}", outline_entry.text)
                    }
                },
                PanelEntry::Search(search_entry) => {
                    let search_data = search_entry.render_data.get_or_init(|| {
                        SearchData::new(&search_entry.match_range, multi_buffer_snapshot)
                    });
                    let mut search_result = String::new();
                    let mut last_end = 0;
                    for range in &search_data.search_match_indices {
                        search_result.push_str(&search_data.context_text[last_end..range.start]);
                        search_result.push('«');
                        search_result.push_str(&search_data.context_text[range.start..range.end]);
                        search_result.push('»');
                        last_end = range.end;
                    }
                    search_result.push_str(&search_data.context_text[last_end..]);

                    format!("search: {search_result}")
                }
            };

            if Some(&entry.entry) == selected_entry {
                display_string += SELECTED_MARKER;
            }
        }
        display_string
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);

            theme_settings::init(theme::LoadThemes::JustBase, cx);

            editor::init(cx);
            project_search::init(cx);
            buffer_search::init(cx);
            super::init(cx);

            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.worktree.file_scan_depth = Some(0);
                });
            });
        });
    }

    // Based on https://github.com/rust-lang/rust-analyzer/
    async fn populate_with_test_ra_project(fs: &FakeFs, root: &str) {
        fs.insert_tree(
            root,
            json!({
                    "crates": {
                        "ide": {
                            "src": {
                                "inlay_hints": {
                                    "fn_lifetime_fn.rs": r##"
        pub(super) fn hints(
            acc: &mut Vec<InlayHint>,
            config: &InlayHintsConfig,
            func: ast::Fn,
        ) -> Option<()> {
            // ... snip

            let mut used_names: FxHashMap<SmolStr, usize> =
                match config.param_names_for_lifetime_elision_hints {
                    true => generic_param_list
                        .iter()
                        .flat_map(|gpl| gpl.lifetime_params())
                        .filter_map(|param| param.lifetime())
                        .filter_map(|lt| Some((SmolStr::from(lt.text().as_str().get(1..)?), 0)))
                        .collect(),
                    false => Default::default(),
                };
            {
                let mut potential_lt_refs = potential_lt_refs.iter().filter(|&&(.., is_elided)| is_elided);
                if self_param.is_some() && potential_lt_refs.next().is_some() {
                    allocated_lifetimes.push(if config.param_names_for_lifetime_elision_hints {
                        // self can't be used as a lifetime, so no need to check for collisions
                        "'self".into()
                    } else {
                        gen_idx_name()
                    });
                }
                potential_lt_refs.for_each(|(name, ..)| {
                    let name = match name {
                        Some(it) if config.param_names_for_lifetime_elision_hints => {
                            if let Some(c) = used_names.get_mut(it.text().as_str()) {
                                *c += 1;
                                SmolStr::from(format!("'{text}{c}", text = it.text().as_str()))
                            } else {
                                used_names.insert(it.text().as_str().into(), 0);
                                SmolStr::from_iter(["\'", it.text().as_str()])
                            }
                        }
                        _ => gen_idx_name(),
                    };
                    allocated_lifetimes.push(name);
                });
            }

            // ... snip
        }

        // ... snip

            #[test]
            fn hints_lifetimes_named() {
                check_with_config(
                    InlayHintsConfig { param_names_for_lifetime_elision_hints: true, ..TEST_CONFIG },
                    r#"
        fn nested_in<'named>(named: &        &X<      &()>) {}
        //          ^'named1, 'named2, 'named3, $
                                  //^'named1 ^'named2 ^'named3
        "#,
                );
            }

        // ... snip
        "##,
                                },
                        "inlay_hints.rs": r#"
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct InlayHintsConfig {
        // ... snip
        pub param_names_for_lifetime_elision_hints: bool,
        pub max_length: Option<usize>,
        // ... snip
    }

    impl Config {
        pub fn inlay_hints(&self) -> InlayHintsConfig {
            InlayHintsConfig {
                // ... snip
                param_names_for_lifetime_elision_hints: self
                    .inlayHints_lifetimeElisionHints_useParameterNames()
                    .to_owned(),
                max_length: self.inlayHints_maxLength().to_owned(),
                // ... snip
            }
        }
    }
    "#,
                        "static_index.rs": r#"
// ... snip
        fn add_file(&mut self, file_id: FileId) {
            let current_crate = crates_for(self.db, file_id).pop().map(Into::into);
            let folds = self.analysis.folding_ranges(file_id).unwrap();
            let inlay_hints = self
                .analysis
                .inlay_hints(
                    &InlayHintsConfig {
                        // ... snip
                        closure_style: hir::ClosureStyle::ImplFn,
                        param_names_for_lifetime_elision_hints: false,
                        binding_mode_hints: false,
                        max_length: Some(25),
                        closure_capture_hints: false,
                        // ... snip
                    },
                    file_id,
                    None,
                )
                .unwrap();
            // ... snip
    }
// ... snip
    "#
                            }
                        },
                        "rust-analyzer": {
                            "src": {
                                "cli": {
                                    "analysis_stats.rs": r#"
        // ... snip
                for &file_id in &file_ids {
                    _ = analysis.inlay_hints(
                        &InlayHintsConfig {
                            // ... snip
                            implicit_drop_hints: true,
                            lifetime_elision_hints: ide::LifetimeElisionHints::Always,
                            param_names_for_lifetime_elision_hints: true,
                            hide_named_constructor_hints: false,
                            hide_closure_initialization_hints: false,
                            closure_style: hir::ClosureStyle::ImplFn,
                            max_length: Some(25),
                            closing_brace_hints_min_lines: Some(20),
                            fields_to_resolve: InlayFieldsToResolve::empty(),
                            range_exclusive_hints: true,
                        },
                        file_id.into(),
                        None,
                    );
                }
        // ... snip
                                    "#,
                                },
                                "config.rs": r#"
                config_data! {
                    /// Configs that only make sense when they are set by a client. As such they can only be defined
                    /// by setting them using client's settings (e.g `settings.json` on VS Code).
                    client: struct ClientDefaultConfigData <- ClientConfigInput -> {
                        // ... snip
                        /// Maximum length for inlay hints. Set to null to have an unlimited length.
                        inlayHints_maxLength: Option<usize>                        = Some(25),
                        // ... snip
                        /// Whether to prefer using parameter names as the name for elided lifetime hints if possible.
                        inlayHints_lifetimeElisionHints_useParameterNames: bool    = false,
                        // ... snip
                    }
                }

                impl Config {
                    // ... snip
                    pub fn inlay_hints(&self) -> InlayHintsConfig {
                        InlayHintsConfig {
                            // ... snip
                            param_names_for_lifetime_elision_hints: self
                                .inlayHints_lifetimeElisionHints_useParameterNames()
                                .to_owned(),
                            max_length: self.inlayHints_maxLength().to_owned(),
                            // ... snip
                        }
                    }
                    // ... snip
                }
                "#
                                }
                        }
                    }
            }),
        )
        .await;
    }

    fn snapshot(outline_panel: &OutlinePanel, cx: &App) -> MultiBufferSnapshot {
        outline_panel
            .active_editor()
            .unwrap()
            .read(cx)
            .buffer()
            .read(cx)
            .snapshot(cx)
    }

    fn selected_row_text(editor: &Entity<Editor>, cx: &mut App) -> String {
        editor.update(cx, |editor, cx| {
            let selections = editor.selections.all::<language::Point>(&editor.display_snapshot(cx));
            assert_eq!(selections.len(), 1, "Active editor should have exactly one selection after any outline panel interactions");
            let selection = selections.first().unwrap();
            let multi_buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
            let line_start = language::Point::new(selection.start.row, 0);
            let line_end = multi_buffer_snapshot.clip_point(language::Point::new(selection.end.row, u32::MAX), language::Bias::Right);
            multi_buffer_snapshot.text_for_range(line_start..line_end).collect::<String>().trim().to_owned()
        })
    }

    async fn active_outline_panel(
        project: &Entity<Project>,
        cx: &mut TestAppContext,
    ) -> (Entity<Workspace>, Entity<OutlinePanel>, VisualTestContext) {
        let (window, workspace) = add_outline_panel(project, cx).await;
        let mut cx = VisualTestContext::from_window(window.into(), cx);
        let panel = outline_panel(&workspace, &mut cx);
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.set_active(true, window, cx);
        });
        (workspace, panel, cx)
    }

    fn add_diff_editor(
        workspace: &Entity<Workspace>,
        project: &Entity<Project>,
        entries: &[(&Entity<language::Buffer>, &str)],
        cx: &mut VisualTestContext,
    ) -> Entity<Editor> {
        let (editor, diffs) = workspace.update_in(cx, |workspace, window, cx| {
            let mut diffs = Vec::new();
            let multibuffer = cx.new(|cx| {
                let mut multibuffer = editor::MultiBuffer::new(language::Capability::ReadWrite);
                for (buffer, base_text) in entries {
                    let diff = cx.new(|cx| {
                        BufferDiff::new_with_base_text(
                            base_text,
                            &buffer.read(cx).text_snapshot(),
                            cx,
                        )
                    });
                    let buffer_snapshot = buffer.read(cx).snapshot();
                    let ranges = diff
                        .read(cx)
                        .snapshot(cx)
                        .hunks(&buffer_snapshot)
                        .map(|hunk| hunk.buffer_range.to_point(&buffer_snapshot))
                        .collect::<Vec<_>>();
                    assert!(
                        !ranges.is_empty(),
                        "diff fixture must have at least one hunk"
                    );
                    multibuffer.set_excerpts_for_buffer((*buffer).clone(), ranges, 0, cx);
                    multibuffer.add_diff(diff.clone(), cx);
                    diffs.push(diff);
                }
                multibuffer
            });
            let editor = cx.new(|cx| {
                let mut editor =
                    Editor::for_multibuffer(multibuffer, Some(project.clone()), window, cx);
                editor.set_diff_hunk_renderer(Some(Arc::new(HiddenUnstagedDiffHunkRenderer)), cx);
                editor.set_expand_all_diff_hunks(cx);
                editor
            });
            workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, window, cx);
            (editor, diffs)
        });
        flush_outline_tasks(cx);
        editor.read_with(cx, |editor, cx| {
            let multibuffer = editor.buffer().read(cx);
            let snapshot = multibuffer.snapshot(cx);
            for ((buffer, base_text), diff) in entries.iter().zip(&diffs) {
                let buffer_id = buffer.read(cx).remote_id();
                assert_eq!(
                    multibuffer.diff_for(buffer_id).map(|diff| diff.entity_id()),
                    Some(diff.entity_id()),
                );
                let diff = snapshot
                    .diff_for_buffer_id(buffer_id)
                    .expect("fixture diff should remain attached");
                assert!(diff.base_text_exists());
                assert_eq!(diff.base_text().text(), *base_text);
            }
        });
        editor
    }

    fn flush_outline_tasks(cx: &mut VisualTestContext) {
        cx.run_until_parked();
        cx.executor().advance_clock(UPDATE_DEBOUNCE * 3);
        cx.run_until_parked();
    }

    fn select_directory(
        panel: &Entity<OutlinePanel>,
        worktree_id: WorktreeId,
        path: &RelPath,
        cx: &mut VisualTestContext,
    ) {
        panel.update_in(cx, |panel, window, cx| {
            let mut entries = panel
                .cached_entries
                .iter()
                .filter(|cached| match &cached.entry {
                    PanelEntry::Fs(FsEntry::Directory(directory)) => {
                        directory.worktree_id == worktree_id
                            && directory.entry.path.as_ref() == path
                    }
                    PanelEntry::FoldedDirs(directories) => {
                        directories.worktree_id == worktree_id
                            && directories
                                .entries
                                .last()
                                .is_some_and(|entry| entry.path.as_ref() == path)
                    }
                    _ => false,
                });
            let entry = entries
                .next()
                .expect("directory should have a row")
                .entry
                .clone();
            assert!(
                entries.next().is_none(),
                "directory should have exactly one row"
            );
            panel.select_entry(entry, true, window, cx);
        });
    }

    fn select_file(panel: &Entity<OutlinePanel>, buffer_id: BufferId, cx: &mut VisualTestContext) {
        panel.update_in(cx, |panel, window, cx| {
            let mut entries = panel.cached_entries.iter().filter(|cached| {
                matches!(&cached.entry, PanelEntry::Fs(FsEntry::File(file)) if file.buffer_id == buffer_id)
            });
            let entry = entries.next().expect("file should have a row").entry.clone();
            assert_eq!(entries.count(), 0, "file should have exactly one row");
            panel.select_entry(entry, true, window, cx);
        });
    }

    fn assert_directory_paths(
        panel: &Entity<OutlinePanel>,
        worktree_id: WorktreeId,
        expected_unfolded: &[&str],
        expected_collapsed: &[&str],
        cx: &mut VisualTestContext,
    ) {
        panel.read_with(cx, |panel, _| {
            assert_eq!(
                panel.unfolded_dirs,
                HashMap::from_iter([(
                    worktree_id,
                    expected_unfolded
                        .iter()
                        .map(|path| Arc::<RelPath>::from(rel_path(path)))
                        .collect::<BTreeSet<_>>(),
                )]),
            );
            assert_eq!(
                panel
                    .collapsed_entries
                    .iter()
                    .filter_map(|entry| match entry {
                        CollapsedEntry::Dir(worktree_id, path) =>
                            Some((*worktree_id, path.clone())),
                        _ => None,
                    })
                    .collect::<HashSet<_>>(),
                expected_collapsed
                    .iter()
                    .map(|path| (worktree_id, Arc::<RelPath>::from(rel_path(path))))
                    .collect::<HashSet<_>>(),
            );
        });
    }

    #[gpui::test]
    async fn test_outline_keyboard_expand_collapse(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "lib.rs": indoc!("
                            mod outer {
                                pub struct OuterStruct {
                                    field: String,
                                }
                                impl OuterStruct {
                                    pub fn new() -> Self {
                                        Self { field: String::new() }
                                    }
                                    pub fn method(&self) {
                                        println!(\"{}\", self.field);
                                    }
                                }
                                mod inner {
                                    pub fn inner_function() {
                                        let x = 42;
                                        println!(\"{}\", x);
                                    }
                                    pub struct InnerStruct {
                                        value: i32,
                                    }
                                }
                            }
                            fn main() {
                                let s = outer::OuterStruct::new();
                                s.method();
                            }
                        "),
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from("/test/src/lib.rs"),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .await
            .unwrap();

        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(500));
        cx.run_until_parked();

        // Force another update cycle to ensure outlines are fetched
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.update_non_fs_items(window, cx);
            panel.update_cached_entries(Some(UPDATE_DEBOUNCE), window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(500));
        cx.run_until_parked();

        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: mod outer  <==== selected
  outline: pub struct OuterStruct
    outline: field
  outline: impl OuterStruct
    outline: pub fn new
    outline: pub fn method
  outline: mod inner
    outline: pub fn inner_function
    outline: pub struct InnerStruct
      outline: value
outline: fn main"
                )
            );
        });

        let parent_outline = outline_panel
            .read_with(cx, |panel, _cx| {
                panel
                    .cached_entries
                    .iter()
                    .find_map(|entry| match &entry.entry {
                        PanelEntry::Outline(OutlineEntry::Outline(outline))
                            if panel
                                .outline_children_cache
                                .get(&outline.range.start.buffer_id)
                                .and_then(|children_map| {
                                    let key = (outline.range.clone(), outline.depth);
                                    children_map.get(&key)
                                })
                                .copied()
                                .unwrap_or(false) =>
                        {
                            Some(entry.entry.clone())
                        }
                        _ => None,
                    })
            })
            .expect("Should find an outline with children");

        outline_panel.update_in(cx, |panel, window, cx| {
            panel.select_entry(parent_outline.clone(), true, window, cx);
            panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: mod outer  <==== selected
outline: fn main"
                )
            );
        });

        outline_panel.update_in(cx, |panel, window, cx| {
            panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: mod outer  <==== selected
  outline: pub struct OuterStruct
    outline: field
  outline: impl OuterStruct
    outline: pub fn new
    outline: pub fn method
  outline: mod inner
    outline: pub fn inner_function
    outline: pub struct InnerStruct
      outline: value
outline: fn main"
                )
            );
        });

        outline_panel.update_in(cx, |panel, window, cx| {
            panel.collapsed_entries.clear();
            panel.update_cached_entries(None, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        outline_panel.update_in(cx, |panel, window, cx| {
            let outlines_with_children: Vec<_> = panel
                .cached_entries
                .iter()
                .filter_map(|entry| match &entry.entry {
                    PanelEntry::Outline(OutlineEntry::Outline(outline))
                        if panel
                            .outline_children_cache
                            .get(&outline.range.start.buffer_id)
                            .and_then(|children_map| {
                                let key = (outline.range.clone(), outline.depth);
                                children_map.get(&key)
                            })
                            .copied()
                            .unwrap_or(false) =>
                    {
                        Some(entry.entry.clone())
                    }
                    _ => None,
                })
                .collect();

            for outline in outlines_with_children {
                panel.select_entry(outline, false, window, cx);
                panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
            }
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: mod outer
outline: fn main"
                )
            );
        });

        let collapsed_entries_count =
            outline_panel.read_with(cx, |panel, _| panel.collapsed_entries.len());
        assert!(
            collapsed_entries_count > 0,
            "Should have collapsed entries tracked"
        );
    }

    #[gpui::test]
    async fn test_outline_click_toggle_behavior(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "main.rs": indoc!("
                            struct Config {
                                name: String,
                                value: i32,
                            }
                            impl Config {
                                fn new(name: String) -> Self {
                                    Self { name, value: 0 }
                                }
                                fn get_value(&self) -> i32 {
                                    self.value
                                }
                            }
                            enum Status {
                                Active,
                                Inactive,
                            }
                            fn process_config(config: Config) -> Status {
                                if config.get_value() > 0 {
                                    Status::Active
                                } else {
                                    Status::Inactive
                                }
                            }
                            fn main() {
                                let config = Config::new(\"test\".to_string());
                                let status = process_config(config);
                            }
                        "),
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));

        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        let _editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from("/test/src/main.rs"),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .await
            .unwrap();

        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        outline_panel.update(cx, |outline_panel, _cx| {
            outline_panel.selected_entry = SelectedEntry::None;
        });

        // Check initial state - all entries should be expanded by default
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct Config
  outline: name
  outline: value
outline: impl Config
  outline: fn new
  outline: fn get_value
outline: enum Status
  outline: Active
  outline: Inactive
outline: fn process_config
outline: fn main"
                )
            );
        });

        outline_panel.update(cx, |outline_panel, _cx| {
            outline_panel.selected_entry = SelectedEntry::None;
        });

        cx.update(|window, cx| {
            outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.select_first(&SelectFirst, window, cx);
            });
        });

        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct Config  <==== selected
  outline: name
  outline: value
outline: impl Config
  outline: fn new
  outline: fn get_value
outline: enum Status
  outline: Active
  outline: Inactive
outline: fn process_config
outline: fn main"
                )
            );
        });

        cx.update(|window, cx| {
            outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
            });
        });

        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct Config  <==== selected
outline: impl Config
  outline: fn new
  outline: fn get_value
outline: enum Status
  outline: Active
  outline: Inactive
outline: fn process_config
outline: fn main"
                )
            );
        });

        cx.update(|window, cx| {
            outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
            });
        });

        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct Config  <==== selected
  outline: name
  outline: value
outline: impl Config
  outline: fn new
  outline: fn get_value
outline: enum Status
  outline: Active
  outline: Inactive
outline: fn process_config
outline: fn main"
                )
            );
        });
    }

    #[gpui::test]
    async fn test_outline_expand_collapse_all(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "lib.rs": indoc!("
                            mod outer {
                                pub struct OuterStruct {
                                    field: String,
                                }
                                impl OuterStruct {
                                    pub fn new() -> Self {
                                        Self { field: String::new() }
                                    }
                                    pub fn method(&self) {
                                        println!(\"{}\", self.field);
                                    }
                                }
                                mod inner {
                                    pub fn inner_function() {
                                        let x = 42;
                                        println!(\"{}\", x);
                                    }
                                    pub struct InnerStruct {
                                        value: i32,
                                    }
                                }
                            }
                            fn main() {
                                let s = outer::OuterStruct::new();
                                s.method();
                            }
                        "),
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from("/test/src/lib.rs"),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .await
            .unwrap();

        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(500));
        cx.run_until_parked();

        // Force another update cycle to ensure outlines are fetched
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.update_non_fs_items(window, cx);
            panel.update_cached_entries(Some(UPDATE_DEBOUNCE), window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(500));
        cx.run_until_parked();

        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: mod outer  <==== selected
  outline: pub struct OuterStruct
    outline: field
  outline: impl OuterStruct
    outline: pub fn new
    outline: pub fn method
  outline: mod inner
    outline: pub fn inner_function
    outline: pub struct InnerStruct
      outline: value
outline: fn main"
                )
            );
        });

        let _parent_outline = outline_panel
            .read_with(cx, |panel, _cx| {
                panel
                    .cached_entries
                    .iter()
                    .find_map(|entry| match &entry.entry {
                        PanelEntry::Outline(OutlineEntry::Outline(outline))
                            if panel
                                .outline_children_cache
                                .get(&outline.range.start.buffer_id)
                                .and_then(|children_map| {
                                    let key = (outline.range.clone(), outline.depth);
                                    children_map.get(&key)
                                })
                                .copied()
                                .unwrap_or(false) =>
                        {
                            Some(entry.entry.clone())
                        }
                        _ => None,
                    })
            })
            .expect("Should find an outline with children");

        // Collapse all entries
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.collapse_all_entries(&CollapseAllEntries, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        let expected_collapsed_output = indoc!(
            "
        outline: mod outer  <==== selected
        outline: fn main"
        );

        outline_panel.update(cx, |panel, cx| {
            assert_eq! {
                display_entries(
                    &project,
                    &snapshot(panel, cx),
                    &panel.cached_entries,
                    panel.selected_entry(),
                    cx,
                ),
                expected_collapsed_output
            };
        });

        // Expand all entries
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.expand_all_entries(&ExpandAllEntries, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        let expected_expanded_output = indoc!(
            "
        outline: mod outer  <==== selected
          outline: pub struct OuterStruct
            outline: field
          outline: impl OuterStruct
            outline: pub fn new
            outline: pub fn method
          outline: mod inner
            outline: pub fn inner_function
            outline: pub struct InnerStruct
              outline: value
        outline: fn main"
        );

        outline_panel.update(cx, |panel, cx| {
            assert_eq! {
                display_entries(
                    &project,
                    &snapshot(panel, cx),
                    &panel.cached_entries,
                    panel.selected_entry(),
                    cx,
                ),
                expected_expanded_output
            };
        });
    }

    #[gpui::test]
    async fn test_buffer_search(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "foo.txt": r#"<_constitution>

</_constitution>



## 📊 Output

| Field          | Meaning                |
"#
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);

        let editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from("/test/foo.txt"),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..OpenOptions::default()
                    },
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        let search_bar = workspace.update_in(cx, |_, window, cx| {
            cx.new(|cx| {
                let mut search_bar = BufferSearchBar::new(None, window, cx);
                search_bar.set_active_pane_item(Some(&editor), window, cx);
                search_bar.show(window, cx);
                search_bar
            })
        });

        let outline_panel = outline_panel(&workspace, cx);

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search("  ", None, true, window, cx)
            })
            .await
            .unwrap();

        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(500));
        cx.run_until_parked();

        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                "search: | Field«  »        | Meaning                |  <==== selected
search: | Field  «  »      | Meaning                |
search: | Field    «  »    | Meaning                |
search: | Field      «  »  | Meaning                |
search: | Field        «  »| Meaning                |
search: | Field          | Meaning«  »              |
search: | Field          | Meaning  «  »            |
search: | Field          | Meaning    «  »          |
search: | Field          | Meaning      «  »        |
search: | Field          | Meaning        «  »      |
search: | Field          | Meaning          «  »    |
search: | Field          | Meaning            «  »  |
search: | Field          | Meaning              «  »|"
            );
        });

        let entries_capacity = outline_panel.update_in(cx, |panel, window, cx| {
            panel
                .cached_entries
                .reserve(panel.cached_entries.capacity() + 1);
            let entry = panel
                .cached_entries
                .get(1)
                .expect("second search result")
                .entry
                .clone();
            panel.toggle_expanded(&entry, window, cx);
            panel.scroll_editor_to_entry(&entry, true, false, window, cx);
            panel.cached_entries.capacity()
        });
        wait_for_outline_tasks(&outline_panel, cx).await;
        outline_panel.read_with(cx, |panel, _| {
            assert_eq!(panel.cached_entries.capacity(), entries_capacity);
            assert_eq!(
                panel.selected_entry(),
                panel.cached_entries.get(1).map(|cached| &cached.entry)
            );
        });

        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search("Field", None, true, window, cx)
            })
            .await
            .expect("search should complete");
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.update_cached_entries(None, window, cx);
        });
        wait_for_outline_tasks(&outline_panel, cx).await;
        outline_panel.update(cx, |panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(panel, cx),
                    &panel.cached_entries,
                    panel.selected_entry(),
                    cx
                ),
                "search: | «Field»          | Meaning                |  <==== selected"
            );
        });
    }

    #[gpui::test]
    async fn test_outline_panel_lsp_document_symbols(cx: &mut TestAppContext) {
        init_test(cx);

        let root = path!("/root");
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            root,
            json!({
                "src": {
                    "lib.rs": indoc!(
                        "
                        struct Foo {
                            bar: u32,
                            baz: String,
                        }
                        "
                    ),
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [Path::new(root)], cx).await;
        let language_registry = project.read_with(cx, |project, _| {
            project.languages().add(rust_lang());
            project.languages().clone()
        });

        let mut fake_language_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    document_symbol_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new(|fake_language_server| {
                    fake_language_server
                        .set_request_handler::<lsp::request::DocumentSymbolRequest, _, _>(
                            move |_, _| async move {
                                #[allow(deprecated)]
                                Ok(Some(lsp::DocumentSymbolResponse::Nested(vec![
                                    lsp::DocumentSymbol {
                                        name: "Foo".to_string(),
                                        detail: None,
                                        kind: lsp::SymbolKind::STRUCT,
                                        tags: None,
                                        deprecated: None,
                                        range: lsp::Range::new(
                                            lsp::Position::new(0, 0),
                                            lsp::Position::new(3, 1),
                                        ),
                                        selection_range: lsp::Range::new(
                                            lsp::Position::new(0, 7),
                                            lsp::Position::new(0, 10),
                                        ),
                                        children: Some(vec![
                                            lsp::DocumentSymbol {
                                                name: "bar".to_string(),
                                                detail: None,
                                                kind: lsp::SymbolKind::FIELD,
                                                tags: None,
                                                deprecated: None,
                                                range: lsp::Range::new(
                                                    lsp::Position::new(1, 4),
                                                    lsp::Position::new(1, 13),
                                                ),
                                                selection_range: lsp::Range::new(
                                                    lsp::Position::new(1, 4),
                                                    lsp::Position::new(1, 7),
                                                ),
                                                children: None,
                                            },
                                            lsp::DocumentSymbol {
                                                name: "lsp_only_field".to_string(),
                                                detail: None,
                                                kind: lsp::SymbolKind::FIELD,
                                                tags: None,
                                                deprecated: None,
                                                range: lsp::Range::new(
                                                    lsp::Position::new(2, 4),
                                                    lsp::Position::new(2, 15),
                                                ),
                                                selection_range: lsp::Range::new(
                                                    lsp::Position::new(2, 4),
                                                    lsp::Position::new(2, 7),
                                                ),
                                                children: None,
                                            },
                                        ]),
                                    },
                                ])))
                            },
                        );
                })),
                ..FakeLspAdapter::default()
            },
        );

        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);
        cx.update(|window, cx| {
            outline_panel.update(cx, |outline_panel, cx| {
                outline_panel.set_active(true, window, cx)
            });
        });

        let editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/root/src/lib.rs")),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..OpenOptions::default()
                    },
                    window,
                    cx,
                )
            })
            .await
            .expect("Failed to open Rust source file")
            .downcast::<Editor>()
            .expect("Should open an editor for Rust source file");
        let _fake_language_server = fake_language_servers.next().await.unwrap();
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        // Step 1: tree-sitter outlines by default
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct Foo  <==== selected
  outline: bar
  outline: baz"
                ),
                "Step 1: tree-sitter outlines should be displayed by default"
            );
        });

        // Step 2: Switch to LSP document symbols
        cx.update(|_, cx| {
            settings::SettingsStore::update_global(
                cx,
                |store: &mut settings::SettingsStore, cx| {
                    store.update_user_settings(cx, |settings| {
                        settings.project.all_languages.defaults.document_symbols =
                            Some(settings::DocumentSymbols::On);
                    });
                },
            );
        });
        wait_for_outline_tasks(&outline_panel, cx).await;

        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct Foo  <==== selected
  outline: bar
  outline: lsp_only_field"
                ),
                "Step 2: After switching to LSP, should see LSP-provided symbols"
            );
        });

        for _ in 0..2 {
            outline_panel.update(cx, |panel, _| panel.outline_fetch_tasks.clear());
            editor.update(cx, |_, cx| cx.emit(EditorEvent::OutlineSymbolsChanged));
            cx.run_until_parked();
            cx.executor()
                .advance_clock(UPDATE_DEBOUNCE - Duration::from_millis(1));
            editor.update(cx, |_, cx| {
                for _ in 0..8 {
                    cx.emit(EditorEvent::OutlineSymbolsChanged);
                }
            });
            cx.run_until_parked();
            outline_panel.read_with(cx, |panel, _| {
                assert!(panel.outline_fetch_tasks.is_empty());
                assert!(
                    panel
                        .buffers
                        .values()
                        .all(|buffer| { matches!(buffer.outlines, OutlineState::Outlines(_)) })
                );
            });
            cx.executor().advance_clock(Duration::from_millis(1));
            cx.run_until_parked();
            outline_panel.read_with(cx, |panel, _| {
                assert_eq!(panel.outline_fetch_tasks.len(), 1);
                assert!(panel.outline_fetch_tasks.values().all(Task::is_ready));
                assert!(
                    panel
                        .buffers
                        .values()
                        .all(|buffer| { matches!(buffer.outlines, OutlineState::Outlines(_)) })
                );
            });
        }

        let entries_capacity = outline_panel.update(cx, |panel, _| {
            panel
                .cached_entries
                .reserve(panel.cached_entries.capacity() + 1);
            panel.cached_entries.capacity()
        });
        for (row, expected_symbol) in [(1, "bar"), (2, "lsp_only_field"), (0, "struct Foo")] {
            editor.update_in(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    let point = language::Point::new(row, 4);
                    selections.select_ranges([point..point]);
                });
            });
            wait_for_outline_tasks(&outline_panel, cx).await;
            outline_panel.read_with(cx, |panel, _| {
                assert_eq!(panel.cached_entries.capacity(), entries_capacity);
                let selected_symbol = panel.selected_entry().and_then(|entry| match entry {
                    PanelEntry::Outline(OutlineEntry::Outline(outline)) => {
                        Some(outline.text.as_ref())
                    }
                    _ => None,
                });
                assert_eq!(selected_symbol, Some(expected_symbol));
            });
        }

        // Step 3: Switch back to tree-sitter
        cx.update(|_, cx| {
            settings::SettingsStore::update_global(
                cx,
                |store: &mut settings::SettingsStore, cx| {
                    store.update_user_settings(cx, |settings| {
                        settings.project.all_languages.defaults.document_symbols =
                            Some(settings::DocumentSymbols::Off);
                    });
                },
            );
        });
        wait_for_outline_tasks(&outline_panel, cx).await;

        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: struct Foo  <==== selected
  outline: bar
  outline: baz"
                ),
                "Step 3: tree-sitter outlines should be restored"
            );
        });

        editor.update(cx, |_, cx| cx.emit(EditorEvent::OutlineSymbolsChanged));
        cx.run_until_parked();
        outline_panel.update_in(cx, |panel, window, cx| {
            assert!(!panel.lsp_outline_refresh_task.is_ready());
            panel.clear_previous(window, cx);
            assert!(panel.lsp_outline_refresh_task.is_ready());
        });
    }

    #[gpui::test]
    async fn test_markdown_outline_selection_at_heading_boundaries(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "doc.md": indoc!("
                    # Section A

                    ## Sub Section A

                    ## Sub Section B

                    # Section B

                ")
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [Path::new("/test")], cx).await;
        project.read_with(cx, |project, _| project.languages().add(markdown_lang()));
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        let editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from("/test/doc.md"),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        wait_for_outline_tasks(&outline_panel, cx).await;

        let entries_capacity = outline_panel.update(cx, |panel, _| {
            panel
                .cached_entries
                .reserve(panel.cached_entries.capacity() + 1);
            panel.cached_entries.capacity()
        });

        // Helper function to move the cursor to the first column of a given row
        // and return the selected outline entry's text.
        let move_cursor_and_get_selection =
            async |row: u32, cx: &mut VisualTestContext| -> Option<SharedString> {
                cx.update(|window, cx| {
                    editor.update(cx, |editor, cx| {
                        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                            s.select_ranges(Some(
                                language::Point::new(row, 0)..language::Point::new(row, 0),
                            ))
                        });
                    });
                });

                wait_for_outline_tasks(&outline_panel, cx).await;

                outline_panel.read_with(cx, |panel, _cx| {
                    assert_eq!(panel.cached_entries.capacity(), entries_capacity);
                    panel.selected_entry().and_then(|entry| match entry {
                        PanelEntry::Outline(OutlineEntry::Outline(outline)) => {
                            Some(outline.text.clone())
                        }
                        _ => None,
                    })
                })
            };

        assert_eq!(
            move_cursor_and_get_selection(0, cx).await.as_deref(),
            Some("# Section A"),
            "Cursor at row 0 should select '# Section A'"
        );

        assert_eq!(
            move_cursor_and_get_selection(2, cx).await.as_deref(),
            Some("## Sub Section A"),
            "Cursor at row 2 should select '## Sub Section A'"
        );

        assert_eq!(
            move_cursor_and_get_selection(4, cx).await.as_deref(),
            Some("## Sub Section B"),
            "Cursor at row 4 should select '## Sub Section B'"
        );

        assert_eq!(
            move_cursor_and_get_selection(6, cx).await.as_deref(),
            Some("# Section B"),
            "Cursor at row 6 should select '# Section B'"
        );

        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .outline_panel
                        .get_or_insert_default()
                        .auto_reveal_entries = Some(false);
                });
            });
        });
        wait_for_outline_tasks(&outline_panel, cx).await;
        let mut notifications = cx.notifications(&outline_panel);
        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([language::Point::new(0, 0)..language::Point::new(0, 0)]);
            });
        });
        wait_for_outline_tasks(&outline_panel, cx).await;
        assert_eq!(notifications.next().now_or_never(), None);
    }

    #[gpui::test]
    async fn test_hide_symbols_in_multibuffer(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/test"),
            json!({
                "src": {
                    "one.rs": indoc!("
                        pub fn one() {
                            let x = 1;
                        }
                    "),
                    "two.rs": indoc!("
                        pub struct Two {
                            field: i32,
                        }
                    "),
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        let buffer_one = open_buffer(&project, path!("/test/src/one.rs"), cx).await;
        let buffer_two = open_buffer(&project, path!("/test/src/two.rs"), cx).await;
        add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer_one, Vec::new()), (&buffer_two, Vec::new())],
            cx,
        );
        wait_for_outline_tasks(&outline_panel, cx).await;

        outline_panel.update(cx, |panel, _cx| {
            assert!(
                panel.cached_entries.iter().any(|entry| matches!(
                    entry.entry,
                    PanelEntry::Outline(OutlineEntry::Outline(_))
                )),
                "expected outline rows in the multibuffer before enabling hide-symbols mode"
            );
        });

        let collapsed_entries_snapshot =
            outline_panel.read_with(cx, |panel, _cx| panel.collapsed_entries.clone());

        let struct_two_outline = outline_panel.read_with(cx, |panel, _cx| {
            panel
                .cached_entries
                .iter()
                .find_map(|entry| match &entry.entry {
                    PanelEntry::Outline(OutlineEntry::Outline(outline))
                        if outline.text == "pub struct Two" =>
                    {
                        Some(entry.entry.clone())
                    }
                    _ => None,
                })
                .expect("expected an outline row for `pub struct Two`")
        });
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.select_entry(struct_two_outline, true, window, cx);
        });

        update_outline_panel_settings(cx, |settings| {
            settings.multi_buffer_hide_symbols = Some(true);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(500));
        cx.run_until_parked();

        outline_panel.update(cx, |panel, _cx| {
            assert!(
                panel.cached_entries.iter().all(|entry| matches!(
                    entry.entry,
                    PanelEntry::Fs(_) | PanelEntry::FoldedDirs(_)
                )),
                "hide-symbols mode should only show fs entries, got {:#?}",
                panel.cached_entries
            );
            assert_eq!(
                panel.collapsed_entries, collapsed_entries_snapshot,
                "hide-symbols mode should not mutate collapsed_entries"
            );
        });

        outline_panel.update(cx, |panel, cx| {
            let buffer_two_id = buffer_two.read(cx).remote_id();
            match panel.selected_entry() {
                Some(PanelEntry::Fs(FsEntry::File(file))) => assert_eq!(
                    file.buffer_id, buffer_two_id,
                    "hiding symbols should remap the selected outline to its containing file"
                ),
                other => panic!("expected the file row of two.rs to be selected, got {other:?}"),
            }
        });

        outline_panel.update_in(cx, |panel, window, cx| {
            panel.toggle_symbols(&ToggleSymbols, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(500));
        cx.run_until_parked();

        outline_panel.update(cx, |panel, cx| {
            assert!(
                panel.cached_entries.iter().any(|entry| matches!(
                    entry.entry,
                    PanelEntry::Outline(OutlineEntry::Outline(_))
                )),
                "toggling hide-symbols off should re-fetch and show outline rows again"
            );
            assert_eq!(
                panel.collapsed_entries, collapsed_entries_snapshot,
                "toggling hide-symbols should not mutate collapsed_entries"
            );
            let buffer_two_id = buffer_two.read(cx).remote_id();
            match panel.selected_entry() {
                Some(PanelEntry::Fs(FsEntry::File(file))) => assert_eq!(
                    file.buffer_id, buffer_two_id,
                    "re-showing symbols should keep the remapped file row selected"
                ),
                other => panic!("expected the file row of two.rs to stay selected, got {other:?}"),
            }
        });

        outline_panel.update_in(cx, |panel, window, cx| {
            panel.toggle_symbols(&ToggleSymbols, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(500));
        cx.run_until_parked();

        outline_panel.update(cx, |panel, _cx| {
            assert!(
                panel.cached_entries.iter().all(|entry| matches!(
                    entry.entry,
                    PanelEntry::Fs(_) | PanelEntry::FoldedDirs(_)
                )),
                "toggling hide-symbols back on should hide outline rows again"
            );
            assert_eq!(
                panel.collapsed_entries, collapsed_entries_snapshot,
                "hide-symbols mode should not mutate collapsed_entries"
            );
        });

        // `hide_symbols_override` is `Some(true)` here while the setting is still `true` too,
        // so flipping the setting to `false` makes the override disagree with the incoming
        // settings change, exercising the settings-observer's override reset.
        update_outline_panel_settings(cx, |settings| {
            settings.multi_buffer_hide_symbols = Some(false);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(500));
        cx.run_until_parked();

        outline_panel.update(cx, |panel, _cx| {
            assert!(
                panel.cached_entries.iter().any(|entry| matches!(
                    entry.entry,
                    PanelEntry::Outline(OutlineEntry::Outline(_))
                )),
                "settings change should clear a stale hide-symbols override and re-show outline rows"
            );
            assert_eq!(
                panel.collapsed_entries, collapsed_entries_snapshot,
                "settings-driven override reset should not mutate collapsed_entries"
            );
        });

        // Regression test: dispatch through the workspace-registered handler (not by calling
        // `toggle_symbols` directly) while the editor, not the panel, is focused. This is the
        // path that used to double-lease the `Workspace` entity and panic.
        outline_panel.update_in(cx, |panel, window, cx| {
            assert!(
                !panel.focus_handle(cx).contains_focused(window, cx),
                "the editor, not the outline panel, should be focused before dispatching"
            );
        });

        cx.dispatch_action(ToggleSymbols);
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(500));
        cx.run_until_parked();

        outline_panel.update(cx, |panel, _cx| {
            assert!(
                panel.cached_entries.iter().all(|entry| matches!(
                    entry.entry,
                    PanelEntry::Fs(_) | PanelEntry::FoldedDirs(_)
                )),
                "dispatching ToggleSymbols via the workspace handler while unfocused should \
                 enable hide-symbols mode without panicking, got {:#?}",
                panel.cached_entries
            );
        });
    }

    #[gpui::test]
    async fn test_hide_symbols_exempts_singleton(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .outline_panel
                        .get_or_insert_default()
                        .multi_buffer_hide_symbols = Some(true);
                });
            });
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "lib.rs": indoc!("
                            mod outer {
                                pub struct OuterStruct {
                                    field: String,
                                }
                                impl OuterStruct {
                                    pub fn new() -> Self {
                                        Self { field: String::new() }
                                    }
                                    pub fn method(&self) {
                                        println!(\"{}\", self.field);
                                    }
                                }
                                mod inner {
                                    pub fn inner_function() {
                                        let x = 42;
                                        println!(\"{}\", x);
                                    }
                                    pub struct InnerStruct {
                                        value: i32,
                                    }
                                }
                            }
                            fn main() {
                                let s = outer::OuterStruct::new();
                                s.method();
                            }
                        "),
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from("/test/src/lib.rs"),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .await
            .unwrap();

        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(500));
        cx.run_until_parked();

        // Force another update cycle to ensure outlines are fetched
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.update_non_fs_items(window, cx);
            panel.update_cached_entries(Some(UPDATE_DEBOUNCE), window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(500));
        cx.run_until_parked();

        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
outline: mod outer  <==== selected
  outline: pub struct OuterStruct
    outline: field
  outline: impl OuterStruct
    outline: pub fn new
    outline: pub fn method
  outline: mod inner
    outline: pub fn inner_function
    outline: pub struct InnerStruct
      outline: value
outline: fn main"
                ),
                "singleton editors should be exempt from hide-symbols mode"
            );
        });

        outline_panel.update_in(cx, |panel, window, cx| {
            panel.toggle_symbols(&ToggleSymbols, window, cx);
            assert_eq!(
                panel.hide_symbols_override, None,
                "toggling symbols in a singleton view should be a no-op and not arm the override"
            );
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_hide_symbols_hides_search_matches(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        let root = path!("/rust-analyzer");
        populate_with_test_ra_project(&fs, root).await;
        let project = Project::test(fs.clone(), [Path::new(root)], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        workspace.update_in(cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(
                workspace,
                &workspace::DeploySearch::default(),
                window,
                cx,
            )
        });
        let search_view = workspace.update_in(cx, |workspace, _window, cx| {
            workspace
                .active_pane()
                .read(cx)
                .items()
                .find_map(|item| item.downcast::<ProjectSearchView>())
                .expect("Project search view expected to appear after new search event trigger")
        });

        let query = "param_names_for_lifetime_elision_hints";
        perform_project_search(&search_view, query, cx);
        search_view.update(cx, |search_view, cx| {
            search_view
                .results_editor()
                .update(cx, |results_editor, cx| {
                    assert_eq!(
                        results_editor.display_text(cx).match_indices(query).count(),
                        9
                    );
                });
        });

        let all_matches = r#"rust-analyzer/
  crates/
    ide/src/
      inlay_hints/
        fn_lifetime_fn.rs
          search: match config.«param_names_for_lifetime_elision_hints» {
          search: allocated_lifetimes.push(if config.«param_names_for_lifetime_elision_hints» {
          search: Some(it) if config.«param_names_for_lifetime_elision_hints» => {
          search: InlayHintsConfig { «param_names_for_lifetime_elision_hints»: true, ..TEST_CONFIG },
      inlay_hints.rs
        search: pub «param_names_for_lifetime_elision_hints»: bool,
        search: «param_names_for_lifetime_elision_hints»: self
      static_index.rs
        search: «param_names_for_lifetime_elision_hints»: false,
    rust-analyzer/src/
      cli/
        analysis_stats.rs
          search: «param_names_for_lifetime_elision_hints»: true,
      config.rs
        search: «param_names_for_lifetime_elision_hints»: self"#
            .to_string();

        let select_first_in_all_matches = |line_to_select: &str| {
            assert!(
                all_matches.contains(line_to_select),
                "`{line_to_select}` was not found in all matches `{all_matches}`"
            );
            all_matches.replacen(
                line_to_select,
                &format!("{line_to_select}{SELECTED_MARKER}"),
                1,
            )
        };

        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();
        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                select_first_in_all_matches(
                    "search: match config.«param_names_for_lifetime_elision_hints» {"
                )
            );
        });

        update_outline_panel_settings(cx, |settings| {
            settings.multi_buffer_hide_symbols = Some(true);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        outline_panel.update(cx, |outline_panel, cx| {
            assert!(
                outline_panel.cached_entries.iter().all(|entry| matches!(
                    entry.entry,
                    PanelEntry::Fs(_) | PanelEntry::FoldedDirs(_)
                )),
                "hide-symbols mode should hide search match rows, got {:#?}",
                outline_panel.cached_entries
            );
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
rust-analyzer/
  crates/
    ide/src/
      inlay_hints/
        fn_lifetime_fn.rs  <==== selected
      inlay_hints.rs
      static_index.rs
    rust-analyzer/src/
      cli/
        analysis_stats.rs
      config.rs"
                ),
                "directories and files should still render in hide-symbols mode, \
                 with the previously selected search match remapped to its file"
            );
        });

        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.toggle_symbols(&ToggleSymbols, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        outline_panel.update(cx, |outline_panel, cx| {
            assert!(
                outline_panel
                    .cached_entries
                    .iter()
                    .any(|entry| matches!(entry.entry, PanelEntry::Search(_))),
                "toggling hide-symbols off should bring search match rows back"
            );
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    outline_panel.selected_entry(),
                    cx,
                ),
                all_matches.replacen(
                    "fn_lifetime_fn.rs",
                    &format!("fn_lifetime_fn.rs{SELECTED_MARKER}"),
                    1,
                ),
                "re-showing search matches should keep the remapped file row selected"
            );
        });
    }

    #[gpui::test]
    async fn test_excerpt_outlines_scoped_to_excerpt_range(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/test"),
            json!({
                "src": {
                    "one.rs": indoc!("
                        fn alpha() {
                            let one = 1;
                        }

                        fn beta() {
                            let two = 2;
                        }
                    "),
                    "two.rs": indoc!("
                        struct Gamma {
                            field: i32,
                        }

                        fn delta() {}
                    "),
                    "three.rs": indoc!("
                        mod epsilon {
                            fn zeta() {
                                let three = 3;
                            }
                        }
                    "),
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        let buffer_one = open_buffer(&project, path!("/test/src/one.rs"), cx).await;
        let buffer_two = open_buffer(&project, path!("/test/src/two.rs"), cx).await;
        let buffer_three = open_buffer(&project, path!("/test/src/three.rs"), cx).await;

        add_multi_buffer_editor(
            &workspace,
            &project,
            &[
                (
                    &buffer_one,
                    vec![
                        language::Point::new(0, 0)..language::Point::new(2, 1),
                        language::Point::new(4, 0)..language::Point::new(6, 1),
                    ],
                ),
                (
                    &buffer_two,
                    vec![language::Point::new(0, 0)..language::Point::new(2, 1)],
                ),
                (
                    &buffer_three,
                    vec![language::Point::new(2, 0)..language::Point::new(2, 22)],
                ),
            ],
            cx,
        );
        wait_for_outline_tasks(&outline_panel, cx).await;

        outline_panel.update(cx, |outline_panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(outline_panel, cx),
                    &outline_panel.cached_entries,
                    None,
                    cx,
                ),
                indoc!(
                    "
test/
  src/
    one.rs
        outline: fn alpha
        outline: fn beta
    three.rs
        outline: mod epsilon
          outline: fn zeta
    two.rs
        outline: struct Gamma
          outline: field"
                ),
                "each excerpt should only show outlines intersecting its range, \
                 without duplicating outlines from other excerpts of the same buffer \
                 and without outlines that lie outside every excerpt"
            );
        });

        let count = 256_u32;
        let text = (0..count)
            .map(|index| format!("fn needle_{index:04}() {{}}\n\n\n\n\n\n\n\n"))
            .collect::<String>();
        buffer_one.update(cx, |buffer, cx| {
            buffer.edit([(0..buffer.len(), text)], None, cx);
        });
        let excerpts = (0..count)
            .step_by(2)
            .map(|index| language::Point::new(index * 8, 0)..language::Point::new(index * 8 + 1, 0))
            .collect::<Vec<_>>();
        let editor = add_multi_buffer_editor(&workspace, &project, &[(&buffer_one, excerpts)], cx);
        wait_for_outline_tasks(&outline_panel, cx).await;
        outline_panel.read_with(cx, |panel, _| {
            assert_eq!(
                panel
                    .cached_entries
                    .iter()
                    .filter_map(|cached| match &cached.entry {
                        PanelEntry::Outline(OutlineEntry::Outline(outline)) =>
                            Some(outline.text.to_string()),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
                (0..count)
                    .step_by(2)
                    .map(|index| format!("fn needle_{index:04}"))
                    .collect::<Vec<_>>(),
            );
        });
        let search_bar = cx.update(|window, cx| {
            cx.new(|cx| {
                let mut search_bar = BufferSearchBar::new(None, window, cx);
                search_bar.set_active_pane_item(Some(&editor), window, cx);
                search_bar.show(window, cx);
                search_bar
            })
        });
        for dense in [false, true] {
            if dense {
                editor.update(cx, |editor, cx| {
                    editor.buffer().update(cx, |multibuffer, cx| {
                        multibuffer.set_excerpts_for_buffer(
                            buffer_one.clone(),
                            [language::Point::default()..buffer_one.read(cx).max_point()],
                            0,
                            cx,
                        );
                    });
                });
            }
            search_bar
                .update_in(cx, |search_bar, window, cx| {
                    search_bar.search("needle", None, true, window, cx)
                })
                .await
                .unwrap();
            wait_for_outline_tasks(&outline_panel, cx).await;
            outline_panel.update(cx, |panel, cx| {
                let snapshot = snapshot(panel, cx);
                assert_eq!(
                    snapshot.excerpts().count(),
                    if dense { 1 } else { count as usize / 2 }
                );
                let mut expected = indoc!(
                    "
                    test/
                      src/
                        one.rs"
                )
                .to_string();
                for index in (0..count).step_by(if dense { 1 } else { 2 }) {
                    expected.push_str(&format!("\n      search: fn «needle»_{index:04}() {{}}"));
                }
                assert_eq!(
                    display_entries(&project, &snapshot, &panel.cached_entries, None, cx),
                    expected
                );
            });
        }
    }

    #[gpui::test]
    async fn test_simultaneous_hide_symbols_and_depth_settings_change(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/test"),
            json!({
                "src": {
                    "one.rs": indoc!("
                        pub fn one() {
                            let x = 1;
                        }
                    "),
                    "two.rs": indoc!("
                        pub struct Two {
                            field: i32,
                        }
                    "),
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        let buffer_one = open_buffer(&project, path!("/test/src/one.rs"), cx).await;
        let buffer_two = open_buffer(&project, path!("/test/src/two.rs"), cx).await;
        add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer_one, Vec::new()), (&buffer_two, Vec::new())],
            cx,
        );
        wait_for_outline_tasks(&outline_panel, cx).await;

        outline_panel.update(cx, |panel, _cx| {
            assert!(
                panel.cached_entries.iter().any(|entry| matches!(
                    entry.entry,
                    PanelEntry::Outline(OutlineEntry::Outline(_))
                )),
                "expected outline rows before the settings change"
            );
            assert_eq!(
                panel
                    .collapsed_entries
                    .iter()
                    .filter(|entry| matches!(entry, CollapsedEntry::Outline(..)))
                    .count(),
                0,
                "no outline should be collapsed at the default expansion depth"
            );
        });

        update_outline_panel_settings(cx, |settings| {
            settings.multi_buffer_hide_symbols = Some(true);
            settings.expand_outlines_with_depth = Some(0);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(500));
        cx.run_until_parked();

        outline_panel.update(cx, |panel, _cx| {
            assert!(
                panel.cached_entries.iter().all(|entry| matches!(
                    entry.entry,
                    PanelEntry::Fs(_) | PanelEntry::FoldedDirs(_)
                )),
                "hide-symbols mode should only show fs entries, got {:#?}",
                panel.cached_entries
            );
            assert_eq!(
                panel
                    .collapsed_entries
                    .iter()
                    .filter(|entry| matches!(entry, CollapsedEntry::Outline(..)))
                    .count(),
                1,
                "a depth change arriving together with a hide-symbols change should still \
                 collapse the only outline with children (struct Two)"
            );
        });

        update_outline_panel_settings(cx, |settings| {
            settings.multi_buffer_hide_symbols = Some(false);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(500));
        cx.run_until_parked();

        outline_panel.update(cx, |panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(panel, cx),
                    &panel.cached_entries,
                    None,
                    cx
                ),
                indoc!(
                    "
test/
  src/
    one.rs
        outline: pub fn one
    two.rs
        outline: pub struct Two"
                ),
                "after re-showing symbols, the depth applied during the combined settings \
                 change should keep struct Two collapsed, hiding its field"
            );
        });
    }

    #[gpui::test]
    async fn test_filtered_symbol_selection_survives_hide_symbols(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/test"),
            json!({
                "src": {
                    "one.rs": indoc!("
                        fn alpha() {
                            let one = 1;
                        }
                    "),
                    "two.rs": indoc!("
                        pub struct Two {
                            field: i32,
                        }
                    "),
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        let buffer_one = open_buffer(&project, path!("/test/src/one.rs"), cx).await;
        let buffer_two = open_buffer(&project, path!("/test/src/two.rs"), cx).await;
        add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer_one, Vec::new()), (&buffer_two, Vec::new())],
            cx,
        );
        wait_for_outline_tasks(&outline_panel, cx).await;

        outline_panel.update_in(cx, |panel, window, cx| {
            panel.filter_editor.update(cx, |filter_editor, cx| {
                filter_editor.set_text("alpha", window, cx);
            });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        let alpha_outline = outline_panel.read_with(cx, |panel, _cx| {
            panel
                .cached_entries
                .iter()
                .find_map(|entry| match &entry.entry {
                    PanelEntry::Outline(OutlineEntry::Outline(outline))
                        if outline.text == "fn alpha" =>
                    {
                        Some(entry.entry.clone())
                    }
                    _ => None,
                })
                .expect("expected the filtered view to contain the `fn alpha` outline")
        });
        outline_panel.update_in(cx, |panel, window, cx| {
            panel.select_entry(alpha_outline, true, window, cx);
        });

        outline_panel.update_in(cx, |panel, window, cx| {
            panel.toggle_symbols(&ToggleSymbols, window, cx);
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        outline_panel.update_in(cx, |panel, window, cx| {
            assert_eq!(
                panel.cached_entries.len(),
                0,
                "hiding symbols with a symbol-only filter should show an empty list, got {:#?}",
                panel.cached_entries
            );
            let buffer_one_id = buffer_one.read(cx).remote_id();
            match panel.selected_entry() {
                Some(PanelEntry::Fs(FsEntry::File(file))) => assert_eq!(
                    file.buffer_id, buffer_one_id,
                    "the filtered outline selection should remap to its containing file"
                ),
                other => panic!("expected the file row of one.rs to be selected, got {other:?}"),
            }
            panel.select_next(&SelectNext, window, cx);
            panel.select_previous(&SelectPrevious, window, cx);
        });

        outline_panel.update_in(cx, |panel, window, cx| {
            panel.filter_editor.update(cx, |filter_editor, cx| {
                filter_editor.set_text("", window, cx);
            });
        });
        cx.executor()
            .advance_clock(UPDATE_DEBOUNCE + Duration::from_millis(100));
        cx.run_until_parked();

        outline_panel.update(cx, |panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(panel, cx),
                    &panel.cached_entries,
                    panel.selected_entry(),
                    cx,
                ),
                indoc!(
                    "
test/
  src/
    one.rs  <==== selected
    two.rs"
                ),
                "clearing the filter should reveal the remapped file row still selected"
            );
        });
    }

    #[gpui::test]
    async fn test_default_depth_applies_to_all_buffers(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .outline_panel
                        .get_or_insert_default()
                        .expand_outlines_with_depth = Some(0);
                });
            });
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/test"),
            json!({
                "src": {
                    "one.rs": indoc!("
                        mod alpha {
                            fn a() {}
                        }
                    "),
                    "two.rs": indoc!("
                        pub struct Two {
                            field: i32,
                        }
                    "),
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        let buffer_one = open_buffer(&project, path!("/test/src/one.rs"), cx).await;
        let buffer_two = open_buffer(&project, path!("/test/src/two.rs"), cx).await;
        add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer_one, Vec::new()), (&buffer_two, Vec::new())],
            cx,
        );
        wait_for_outline_tasks(&outline_panel, cx).await;

        outline_panel.update(cx, |panel, cx| {
            assert_eq!(
                panel
                    .collapsed_entries
                    .iter()
                    .filter(|entry| matches!(entry, CollapsedEntry::Outline(..)))
                    .count(),
                2,
                "the default expansion depth should collapse parent outlines in every \
                 buffer of the multi-buffer, not just the first fetched one"
            );
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(panel, cx),
                    &panel.cached_entries,
                    None,
                    cx
                ),
                indoc!(
                    "
test/
  src/
    one.rs
        outline: mod alpha
    two.rs
        outline: pub struct Two"
                ),
                "both files should render only their collapsed parent outlines"
            );
        });
    }

    #[gpui::test]
    async fn test_depth_change_while_symbols_hidden_applies_on_reveal(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .outline_panel
                        .get_or_insert_default()
                        .multi_buffer_hide_symbols = Some(true);
                });
            });
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/test"),
            json!({
                "src": {
                    "one.rs": indoc!("
                        mod alpha {
                            fn a() {}
                        }
                    "),
                    "two.rs": indoc!("
                        pub struct Two {
                            field: i32,
                        }
                    "),
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let outline_panel = outline_panel(&workspace, cx);
        outline_panel.update_in(cx, |outline_panel, window, cx| {
            outline_panel.set_active(true, window, cx)
        });

        let buffer_one = open_buffer(&project, path!("/test/src/one.rs"), cx).await;
        let buffer_two = open_buffer(&project, path!("/test/src/two.rs"), cx).await;
        add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&buffer_one, Vec::new()), (&buffer_two, Vec::new())],
            cx,
        );
        wait_for_outline_tasks(&outline_panel, cx).await;

        outline_panel.update(cx, |panel, _cx| {
            assert!(
                panel.cached_entries.iter().all(|entry| matches!(
                    entry.entry,
                    PanelEntry::Fs(_) | PanelEntry::FoldedDirs(_)
                )),
                "symbols should be hidden and outlines unfetched before the settings change"
            );
        });

        update_outline_panel_settings(cx, |settings| {
            settings.multi_buffer_hide_symbols = Some(false);
            settings.expand_outlines_with_depth = Some(0);
        });
        wait_for_outline_tasks(&outline_panel, cx).await;

        outline_panel.update(cx, |panel, cx| {
            assert_eq!(
                display_entries(
                    &project,
                    &snapshot(panel, cx),
                    &panel.cached_entries,
                    None,
                    cx
                ),
                indoc!(
                    "
test/
  src/
    one.rs
        outline: mod alpha
    two.rs
        outline: pub struct Two"
                ),
                "a depth change while symbols were hidden should apply to outlines \
                 fetched after the symbols are shown again"
            );
        });
    }
}
