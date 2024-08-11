#![allow(rustdoc::private_intra_doc_links)]
//! This is the place where everything editor-related is stored (data-wise) and displayed (ui-wise).
//! The main point of interest in this crate is [`Editor`] type, which is used in every other Zed part as a user input element.
//! It comes in different flavors: single line, multiline and a fixed height one.
//!
//! Editor contains of multiple large submodules:
//! * [`element`] — the place where all rendering happens
//! * [`display_map`] - chunks up text in the editor into the logical blocks, establishes coordinates and mapping between each of them.
//!   Contains all metadata related to text transformations (folds, fake inlay text insertions, soft wraps, tab markup, etc.).
//! * [`inlay_hint_cache`] - is a storage of inlay hints out of LSP requests, responsible for querying LSP and updating `display_map`'s state accordingly.
//!
//! All other submodules and structs are mostly concerned with holding editor data about the way it displays current buffer region(s).
//!
//! If you're looking to improve Vim mode, you should check out Vim crate that wraps Editor and overrides its behavior.
pub mod actions;
mod blame_entry_tooltip;
mod blink_manager;
mod debounced_delay;
pub mod display_map;
mod editor_settings;
mod editor_settings_controls;
mod element;
mod git;
mod highlight_matching_bracket;
mod hover_links;
mod hover_popover;
mod hunk_diff;
mod indent_guides;
mod inlay_hint_cache;
mod inline_completion_provider;
pub mod items;
mod linked_editing_ranges;
mod mouse_context_menu;
pub mod movement;
mod persistence;
mod rust_analyzer_ext;
pub mod scroll;
mod selections_collection;
pub mod tasks;

#[cfg(test)]
mod editor_tests;
mod signature_help;
#[cfg(any(test, feature = "test-support"))]
pub mod test;

use ::git::diff::{DiffHunk, DiffHunkStatus};
use ::git::{parse_git_remote_url, BuildPermalinkParams, GitHostingProviderRegistry};
pub(crate) use actions::*;
use aho_corasick::AhoCorasick;
use anyhow::{anyhow, Context as _, Result};
use blink_manager::BlinkManager;
use client::{Collaborator, ParticipantIndex};
use clock::ReplicaId;
use collections::{BTreeMap, Bound, HashMap, HashSet, VecDeque};
use convert_case::{Case, Casing};
use debounced_delay::DebouncedDelay;
use display_map::*;
pub use display_map::{DisplayPoint, FoldPlaceholder};
pub use editor_settings::{CurrentLineHighlight, EditorSettings};
pub use editor_settings_controls::*;
use element::LineWithInvisibles;
pub use element::{
    CursorLayout, EditorElement, HighlightedRange, HighlightedRangeLine, PointForPosition,
};
use futures::FutureExt;
use fuzzy::{StringMatch, StringMatchCandidate};
use git::blame::GitBlame;
use git::diff_hunk_to_display;
use gpui::{
    div, impl_actions, point, prelude::*, px, relative, size, uniform_list, Action, AnyElement,
    AppContext, AsyncWindowContext, AvailableSpace, BackgroundExecutor, Bounds, ClipboardItem,
    Context, DispatchPhase, ElementId, EntityId, EventEmitter, FocusHandle, FocusOutEvent,
    FocusableView, FontId, FontWeight, HighlightStyle, Hsla, InteractiveText, KeyContext,
    ListSizingBehavior, Model, MouseButton, PaintQuad, ParentElement, Pixels, Render, SharedString,
    Size, StrikethroughStyle, Styled, StyledText, Subscription, Task, TextStyle, UnderlineStyle,
    UniformListScrollHandle, View, ViewContext, ViewInputHandler, VisualContext, WeakFocusHandle,
    WeakView, WindowContext,
};
use highlight_matching_bracket::refresh_matching_bracket_highlights;
use hover_popover::{hide_hover, HoverState};
use hunk_diff::ExpandedHunks;
pub(crate) use hunk_diff::HoveredHunk;
use indent_guides::ActiveIndentGuidesState;
use inlay_hint_cache::{InlayHintCache, InlaySplice, InvalidationStrategy};
pub use inline_completion_provider::*;
pub use items::MAX_TAB_TITLE_LEN;
use itertools::Itertools;
use language::{
    char_kind,
    language_settings::{self, all_language_settings, InlayHintSettings},
    markdown, point_from_lsp, AutoindentMode, BracketPair, Buffer, Capability, CharKind, CodeLabel,
    CursorShape, Diagnostic, Documentation, IndentKind, IndentSize, Language, OffsetRangeExt,
    Point, Selection, SelectionGoal, TransactionId,
};
use language::{point_to_lsp, BufferRow, Runnable, RunnableRange};
use linked_editing_ranges::refresh_linked_ranges;
use task::{ResolvedTask, TaskTemplate, TaskVariables};

use hover_links::{HoverLink, HoveredLinkState, InlayHighlight};
pub use lsp::CompletionContext;
use lsp::{
    CompletionItemKind, CompletionTriggerKind, DiagnosticSeverity, InsertTextFormat,
    LanguageServerId,
};
use mouse_context_menu::MouseContextMenu;
use movement::TextLayoutDetails;
pub use multi_buffer::{
    Anchor, AnchorRangeExt, ExcerptId, ExcerptRange, MultiBuffer, MultiBufferSnapshot, ToOffset,
    ToPoint,
};
use multi_buffer::{ExpandExcerptDirection, MultiBufferPoint, MultiBufferRow, ToOffsetUtf16};
use ordered_float::OrderedFloat;
use parking_lot::{Mutex, RwLock};
use project::project_settings::{GitGutterSetting, ProjectSettings};
use project::{
    CodeAction, Completion, FormatTrigger, Item, Location, Project, ProjectPath,
    ProjectTransaction, TaskSourceKind, WorktreeId,
};
use rand::prelude::*;
use rpc::{proto::*, ErrorExt};
use scroll::{Autoscroll, OngoingScroll, ScrollAnchor, ScrollManager, ScrollbarAutoHide};
use selections_collection::{resolve_multiple, MutableSelectionsCollection, SelectionsCollection};
use serde::{Deserialize, Serialize};
use settings::{update_settings_file, Settings, SettingsStore};
use smallvec::SmallVec;
use snippet::Snippet;
use std::{
    any::TypeId,
    borrow::Cow,
    cell::RefCell,
    cmp::{self, Ordering, Reverse},
    mem,
    num::NonZeroU32,
    ops::{ControlFlow, Deref, DerefMut, Not as _, Range, RangeInclusive},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant},
};
pub use sum_tree::Bias;
use sum_tree::TreeMap;
use text::{BufferId, OffsetUtf16, Rope};
use theme::{
    observe_buffer_font_size_adjustment, ActiveTheme, PlayerColor, StatusColors, SyntaxTheme,
    ThemeColors, ThemeSettings,
};
use ui::{
    h_flex, prelude::*, ButtonSize, ButtonStyle, Disclosure, IconButton, IconName, IconSize,
    ListItem, Popover, Tooltip,
};
use util::{defer, maybe, post_inc, RangeExt, ResultExt, TryFutureExt};
use workspace::item::{ItemHandle, PreviewTabsSettings};
use workspace::notifications::{DetachAndPromptErr, NotificationId};
use workspace::{
    searchable::SearchEvent, ItemNavHistory, SplitDirection, ViewId, Workspace, WorkspaceId,
};
use workspace::{OpenInTerminal, OpenTerminal, TabBarSettings, Toast};

use crate::hover_links::find_url;
use crate::signature_help::{SignatureHelpHiddenBy, SignatureHelpState};

pub const FILE_HEADER_HEIGHT: u32 = 1;
pub const MULTI_BUFFER_EXCERPT_HEADER_HEIGHT: u32 = 1;
pub const MULTI_BUFFER_EXCERPT_FOOTER_HEIGHT: u32 = 1;
pub const DEFAULT_MULTIBUFFER_CONTEXT: u32 = 2;
const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);
const MAX_LINE_LEN: usize = 1024;
const MIN_NAVIGATION_HISTORY_ROW_DELTA: i64 = 10;
const MAX_SELECTION_HISTORY_LEN: usize = 1024;
pub(crate) const CURSORS_VISIBLE_FOR: Duration = Duration::from_millis(2000);
#[doc(hidden)]
pub const CODE_ACTIONS_DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(250);
#[doc(hidden)]
pub const DOCUMENT_HIGHLIGHTS_DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(75);

pub(crate) const FORMAT_TIMEOUT: Duration = Duration::from_secs(2);

pub fn render_parsed_markdown(
    element_id: impl Into<ElementId>,
    parsed: &language::ParsedMarkdown,
    editor_style: &EditorStyle,
    workspace: Option<WeakView<Workspace>>,
    cx: &mut WindowContext,
) -> InteractiveText {
    let code_span_background_color = cx
        .theme()
        .colors()
        .editor_document_highlight_read_background;

    let highlights = gpui::combine_highlights(
        parsed.highlights.iter().filter_map(|(range, highlight)| {
            let highlight = highlight.to_highlight_style(&editor_style.syntax)?;
            Some((range.clone(), highlight))
        }),
        parsed
            .regions
            .iter()
            .zip(&parsed.region_ranges)
            .filter_map(|(region, range)| {
                if region.code {
                    Some((
                        range.clone(),
                        HighlightStyle {
                            background_color: Some(code_span_background_color),
                            ..Default::default()
                        },
                    ))
                } else {
                    None
                }
            }),
    );

    let mut links = Vec::new();
    let mut link_ranges = Vec::new();
    for (range, region) in parsed.region_ranges.iter().zip(&parsed.regions) {
        if let Some(link) = region.link.clone() {
            links.push(link);
            link_ranges.push(range.clone());
        }
    }

    InteractiveText::new(
        element_id,
        StyledText::new(parsed.text.clone()).with_highlights(&editor_style.text, highlights),
    )
    .on_click(link_ranges, move |clicked_range_ix, cx| {
        match &links[clicked_range_ix] {
            markdown::Link::Web { url } => cx.open_url(url),
            markdown::Link::Path { path } => {
                if let Some(workspace) = &workspace {
                    _ = workspace.update(cx, |workspace, cx| {
                        workspace.open_abs_path(path.clone(), false, cx).detach();
                    });
                }
            }
        }
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum InlayId {
    Suggestion(usize),
    Hint(usize),
}

impl InlayId {
    fn id(&self) -> usize {
        match self {
            Self::Suggestion(id) => *id,
            Self::Hint(id) => *id,
        }
    }
}

enum DiffRowHighlight {}
enum DocumentHighlightRead {}
enum DocumentHighlightWrite {}
enum InputComposition {}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Prev,
    Next,
}

pub fn init_settings(cx: &mut AppContext) {
    EditorSettings::register(cx);
}

pub fn init(cx: &mut AppContext) {
    init_settings(cx);

    workspace::register_project_item::<Editor>(cx);
    workspace::FollowableViewRegistry::register::<Editor>(cx);
    workspace::register_serializable_item::<Editor>(cx);

    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace.register_action(Editor::new_file);
            workspace.register_action(Editor::new_file_in_direction);
        },
    )
    .detach();

    cx.on_action(move |_: &workspace::NewFile, cx| {
        let app_state = workspace::AppState::global(cx);
        if let Some(app_state) = app_state.upgrade() {
            workspace::open_new(app_state, cx, |workspace, cx| {
                Editor::new_file(workspace, &Default::default(), cx)
            })
            .detach();
        }
    });
    cx.on_action(move |_: &workspace::NewWindow, cx| {
        let app_state = workspace::AppState::global(cx);
        if let Some(app_state) = app_state.upgrade() {
            workspace::open_new(app_state, cx, |workspace, cx| {
                Editor::new_file(workspace, &Default::default(), cx)
            })
            .detach();
        }
    });
}

pub struct SearchWithinRange;

trait InvalidationRegion {
    fn ranges(&self) -> &[Range<Anchor>];
}

#[derive(Clone, Debug, PartialEq)]
pub enum SelectPhase {
    Begin {
        position: DisplayPoint,
        add: bool,
        click_count: usize,
    },
    BeginColumnar {
        position: DisplayPoint,
        reset: bool,
        goal_column: u32,
    },
    Extend {
        position: DisplayPoint,
        click_count: usize,
    },
    Update {
        position: DisplayPoint,
        goal_column: u32,
        scroll_delta: gpui::Point<f32>,
    },
    End,
}

#[derive(Clone, Debug)]
pub enum SelectMode {
    Character,
    Word(Range<Anchor>),
    Line(Range<Anchor>),
    All,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum EditorMode {
    SingleLine { auto_width: bool },
    AutoHeight { max_lines: usize },
    Full,
}

#[derive(Clone, Debug)]
pub enum SoftWrap {
    None,
    PreferLine,
    EditorWidth,
    Column(u32),
}

#[derive(Clone)]
pub struct EditorStyle {
    pub background: Hsla,
    pub local_player: PlayerColor,
    pub text: TextStyle,
    pub scrollbar_width: Pixels,
    pub syntax: Arc<SyntaxTheme>,
    pub status: StatusColors,
    pub inlay_hints_style: HighlightStyle,
    pub suggestions_style: HighlightStyle,
}

impl Default for EditorStyle {
    fn default() -> Self {
        Self {
            background: Hsla::default(),
            local_player: PlayerColor::default(),
            text: TextStyle::default(),
            scrollbar_width: Pixels::default(),
            syntax: Default::default(),
            // HACK: Status colors don't have a real default.
            // We should look into removing the status colors from the editor
            // style and retrieve them directly from the theme.
            status: StatusColors::dark(),
            inlay_hints_style: HighlightStyle::default(),
            suggestions_style: HighlightStyle::default(),
        }
    }
}

type CompletionId = usize;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Default)]
struct EditorActionId(usize);

impl EditorActionId {
    pub fn post_inc(&mut self) -> Self {
        let answer = self.0;

        *self = Self(answer + 1);

        Self(answer)
    }
}

// type GetFieldEditorTheme = dyn Fn(&theme::Theme) -> theme::FieldEditor;
// type OverrideTextStyle = dyn Fn(&EditorStyle) -> Option<HighlightStyle>;

type BackgroundHighlight = (fn(&ThemeColors) -> Hsla, Arc<[Range<Anchor>]>);
type GutterHighlight = (fn(&AppContext) -> Hsla, Arc<[Range<Anchor>]>);

#[derive(Default)]
struct ScrollbarMarkerState {
    scrollbar_size: Size<Pixels>,
    dirty: bool,
    markers: Arc<[PaintQuad]>,
    pending_refresh: Option<Task<Result<()>>>,
}

impl ScrollbarMarkerState {
    fn should_refresh(&self, scrollbar_size: Size<Pixels>) -> bool {
        self.pending_refresh.is_none() && (self.scrollbar_size != scrollbar_size || self.dirty)
    }
}

#[derive(Clone, Debug)]
struct RunnableTasks {
    templates: Vec<(TaskSourceKind, TaskTemplate)>,
    offset: MultiBufferOffset,
    // We need the column at which the task context evaluation should take place (when we're spawning it via gutter).
    column: u32,
    // Values of all named captures, including those starting with '_'
    extra_variables: HashMap<String, String>,
    // Full range of the tagged region. We use it to determine which `extra_variables` to grab for context resolution in e.g. a modal.
    context_range: Range<BufferOffset>,
}

#[derive(Clone)]
struct ResolvedTasks {
    templates: SmallVec<[(TaskSourceKind, ResolvedTask); 1]>,
    position: Anchor,
}
#[derive(Copy, Clone, Debug)]
struct MultiBufferOffset(usize);
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct BufferOffset(usize);
/// Zed's primary text input `View`, allowing users to edit a [`MultiBuffer`]
///
/// See the [module level documentation](self) for more information.
pub struct Editor {
    focus_handle: FocusHandle,
    last_focused_descendant: Option<WeakFocusHandle>,
    /// The text buffer being edited
    buffer: Model<MultiBuffer>,
    /// Map of how text in the buffer should be displayed.
    /// Handles soft wraps, folds, fake inlay text insertions, etc.
    pub display_map: Model<DisplayMap>,
    pub selections: SelectionsCollection,
    pub scroll_manager: ScrollManager,
    /// When inline assist editors are linked, they all render cursors because
    /// typing enters text into each of them, even the ones that aren't focused.
    pub(crate) show_cursor_when_unfocused: bool,
    columnar_selection_tail: Option<Anchor>,
    add_selections_state: Option<AddSelectionsState>,
    select_next_state: Option<SelectNextState>,
    select_prev_state: Option<SelectNextState>,
    selection_history: SelectionHistory,
    autoclose_regions: Vec<AutocloseRegion>,
    snippet_stack: InvalidationStack<SnippetState>,
    select_larger_syntax_node_stack: Vec<Box<[Selection<usize>]>>,
    ime_transaction: Option<TransactionId>,
    active_diagnostics: Option<ActiveDiagnosticGroup>,
    soft_wrap_mode_override: Option<language_settings::SoftWrap>,
    project: Option<Model<Project>>,
    completion_provider: Option<Box<dyn CompletionProvider>>,
    collaboration_hub: Option<Box<dyn CollaborationHub>>,
    blink_manager: Model<BlinkManager>,
    show_cursor_names: bool,
    hovered_cursors: HashMap<HoveredCursor, Task<()>>,
    pub show_local_selections: bool,
    mode: EditorMode,
    show_breadcrumbs: bool,
    show_gutter: bool,
    show_line_numbers: Option<bool>,
    show_git_diff_gutter: Option<bool>,
    show_code_actions: Option<bool>,
    show_runnables: Option<bool>,
    show_wrap_guides: Option<bool>,
    show_indent_guides: Option<bool>,
    placeholder_text: Option<Arc<str>>,
    highlight_order: usize,
    highlighted_rows: HashMap<TypeId, Vec<RowHighlight>>,
    background_highlights: TreeMap<TypeId, BackgroundHighlight>,
    gutter_highlights: TreeMap<TypeId, GutterHighlight>,
    scrollbar_marker_state: ScrollbarMarkerState,
    active_indent_guides_state: ActiveIndentGuidesState,
    nav_history: Option<ItemNavHistory>,
    context_menu: RwLock<Option<ContextMenu>>,
    mouse_context_menu: Option<MouseContextMenu>,
    completion_tasks: Vec<(CompletionId, Task<Option<()>>)>,
    signature_help_state: SignatureHelpState,
    auto_signature_help: Option<bool>,
    find_all_references_task_sources: Vec<Anchor>,
    next_completion_id: CompletionId,
    completion_documentation_pre_resolve_debounce: DebouncedDelay,
    available_code_actions: Option<(Location, Arc<[CodeAction]>)>,
    code_actions_task: Option<Task<()>>,
    document_highlights_task: Option<Task<()>>,
    linked_editing_range_task: Option<Task<Option<()>>>,
    linked_edit_ranges: linked_editing_ranges::LinkedEditingRanges,
    pending_rename: Option<RenameState>,
    searchable: bool,
    cursor_shape: CursorShape,
    current_line_highlight: Option<CurrentLineHighlight>,
    collapse_matches: bool,
    autoindent_mode: Option<AutoindentMode>,
    workspace: Option<(WeakView<Workspace>, Option<WorkspaceId>)>,
    keymap_context_layers: BTreeMap<TypeId, KeyContext>,
    input_enabled: bool,
    use_modal_editing: bool,
    read_only: bool,
    leader_peer_id: Option<PeerId>,
    remote_id: Option<ViewId>,
    hover_state: HoverState,
    gutter_hovered: bool,
    hovered_link_state: Option<HoveredLinkState>,
    inline_completion_provider: Option<RegisteredInlineCompletionProvider>,
    active_inline_completion: Option<(Inlay, Option<Range<Anchor>>)>,
    show_inline_completions: bool,
    inlay_hint_cache: InlayHintCache,
    expanded_hunks: ExpandedHunks,
    next_inlay_id: usize,
    _subscriptions: Vec<Subscription>,
    pixel_position_of_newest_cursor: Option<gpui::Point<Pixels>>,
    gutter_dimensions: GutterDimensions,
    pub vim_replace_map: HashMap<Range<usize>, String>,
    style: Option<EditorStyle>,
    next_editor_action_id: EditorActionId,
    editor_actions: Rc<RefCell<BTreeMap<EditorActionId, Box<dyn Fn(&mut ViewContext<Self>)>>>>,
    use_autoclose: bool,
    use_auto_surround: bool,
    auto_replace_emoji_shortcode: bool,
    show_git_blame_gutter: bool,
    show_git_blame_inline: bool,
    show_git_blame_inline_delay_task: Option<Task<()>>,
    git_blame_inline_enabled: bool,
    serialize_dirty_buffers: bool,
    show_selection_menu: Option<bool>,
    blame: Option<Model<GitBlame>>,
    blame_subscription: Option<Subscription>,
    custom_context_menu: Option<
        Box<
            dyn 'static
                + Fn(&mut Self, DisplayPoint, &mut ViewContext<Self>) -> Option<View<ui::ContextMenu>>,
        >,
    >,
    last_bounds: Option<Bounds<Pixels>>,
    expect_bounds_change: Option<Bounds<Pixels>>,
    tasks: BTreeMap<(BufferId, BufferRow), RunnableTasks>,
    tasks_update_task: Option<Task<()>>,
    previous_search_ranges: Option<Arc<[Range<Anchor>]>>,
    file_header_size: u32,
    breadcrumb_header: Option<String>,
    focused_block: Option<FocusedBlock>,
    // TODO kb recheck names
    next_scroll_direction: NextScollCursorCenterTopBottom, 
    _scroll_cursor_center_top_bottom_task: Task<()>,
}

#[derive(Copy, Clone, Debug,  PartialEq, Eq, Default)]
enum NextScollCursorCenterTopBottom {
    #[default]
    Center,
    Top,
    Bottom,
}

impl NextScollCursorCenterTopBottom {
    fn next(&self) -> Self {
        match self {
            Self::Center => Self::Top,
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Center,
        }
    }
}

#[derive(Clone)]
pub struct EditorSnapshot {
    pub mode: EditorMode,
    show_gutter: bool,
    show_line_numbers: Option<bool>,
    show_git_diff_gutter: Option<bool>,
    show_code_actions: Option<bool>,
    show_runnables: Option<bool>,
    render_git_blame_gutter: bool,
    pub display_snapshot: DisplaySnapshot,
    pub placeholder_text: Option<Arc<str>>,
    is_focused: bool,
    scroll_anchor: ScrollAnchor,
    ongoing_scroll: OngoingScroll,
    current_line_highlight: CurrentLineHighlight,
    gutter_hovered: bool,
}

const GIT_BLAME_GUTTER_WIDTH_CHARS: f32 = 53.;

#[derive(Default, Debug, Clone, Copy)]
pub struct GutterDimensions {
    pub left_padding: Pixels,
    pub right_padding: Pixels,
    pub width: Pixels,
    pub margin: Pixels,
    pub git_blame_entries_width: Option<Pixels>,
}

impl GutterDimensions {
    /// The full width of the space taken up by the gutter.
    pub fn full_width(&self) -> Pixels {
        self.margin + self.width
    }

    /// The width of the space reserved for the fold indicators,
    /// use alongside 'justify_end' and `gutter_width` to
    /// right align content with the line numbers
    pub fn fold_area_width(&self) -> Pixels {
        self.margin + self.right_padding
    }
}

#[derive(Debug)]
pub struct RemoteSelection {
    pub replica_id: ReplicaId,
    pub selection: Selection<Anchor>,
    pub cursor_shape: CursorShape,
    pub peer_id: PeerId,
    pub line_mode: bool,
    pub participant_index: Option<ParticipantIndex>,
    pub user_name: Option<SharedString>,
}

#[derive(Clone, Debug)]
struct SelectionHistoryEntry {
    selections: Arc<[Selection<Anchor>]>,
    select_next_state: Option<SelectNextState>,
    select_prev_state: Option<SelectNextState>,
    add_selections_state: Option<AddSelectionsState>,
}

enum SelectionHistoryMode {
    Normal,
    Undoing,
    Redoing,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct HoveredCursor {
    replica_id: u16,
    selection_id: usize,
}

impl Default for SelectionHistoryMode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Default)]
struct SelectionHistory {
    #[allow(clippy::type_complexity)]
    selections_by_transaction:
        HashMap<TransactionId, (Arc<[Selection<Anchor>]>, Option<Arc<[Selection<Anchor>]>>)>,
    mode: SelectionHistoryMode,
    undo_stack: VecDeque<SelectionHistoryEntry>,
    redo_stack: VecDeque<SelectionHistoryEntry>,
}

impl SelectionHistory {
    fn insert_transaction(
        &mut self,
        transaction_id: TransactionId,
        selections: Arc<[Selection<Anchor>]>,
    ) {
        self.selections_by_transaction
            .insert(transaction_id, (selections, None));
    }

    #[allow(clippy::type_complexity)]
    fn transaction(
        &self,
        transaction_id: TransactionId,
    ) -> Option<&(Arc<[Selection<Anchor>]>, Option<Arc<[Selection<Anchor>]>>)> {
        self.selections_by_transaction.get(&transaction_id)
    }

    #[allow(clippy::type_complexity)]
    fn transaction_mut(
        &mut self,
        transaction_id: TransactionId,
    ) -> Option<&mut (Arc<[Selection<Anchor>]>, Option<Arc<[Selection<Anchor>]>>)> {
        self.selections_by_transaction.get_mut(&transaction_id)
    }

    fn push(&mut self, entry: SelectionHistoryEntry) {
        if !entry.selections.is_empty() {
            match self.mode {
                SelectionHistoryMode::Normal => {
                    self.push_undo(entry);
                    self.redo_stack.clear();
                }
                SelectionHistoryMode::Undoing => self.push_redo(entry),
                SelectionHistoryMode::Redoing => self.push_undo(entry),
            }
        }
    }

    fn push_undo(&mut self, entry: SelectionHistoryEntry) {
        if self
            .undo_stack
            .back()
            .map_or(true, |e| e.selections != entry.selections)
        {
            self.undo_stack.push_back(entry);
            if self.undo_stack.len() > MAX_SELECTION_HISTORY_LEN {
                self.undo_stack.pop_front();
            }
        }
    }

    fn push_redo(&mut self, entry: SelectionHistoryEntry) {
        if self
            .redo_stack
            .back()
            .map_or(true, |e| e.selections != entry.selections)
        {
            self.redo_stack.push_back(entry);
            if self.redo_stack.len() > MAX_SELECTION_HISTORY_LEN {
                self.redo_stack.pop_front();
            }
        }
    }
}

struct RowHighlight {
    index: usize,
    range: RangeInclusive<Anchor>,
    color: Option<Hsla>,
    should_autoscroll: bool,
}

#[derive(Clone, Debug)]
struct AddSelectionsState {
    above: bool,
    stack: Vec<usize>,
}

#[derive(Clone)]
struct SelectNextState {
    query: AhoCorasick,
    wordwise: bool,
    done: bool,
}

impl std::fmt::Debug for SelectNextState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("wordwise", &self.wordwise)
            .field("done", &self.done)
            .finish()
    }
}

#[derive(Debug)]
struct AutocloseRegion {
    selection_id: usize,
    range: Range<Anchor>,
    pair: BracketPair,
}

#[derive(Debug)]
struct SnippetState {
    ranges: Vec<Vec<Range<Anchor>>>,
    active_index: usize,
}

#[doc(hidden)]
pub struct RenameState {
    pub range: Range<Anchor>,
    pub old_name: Arc<str>,
    pub editor: View<Editor>,
    block_id: CustomBlockId,
}

struct InvalidationStack<T>(Vec<T>);

struct RegisteredInlineCompletionProvider {
    provider: Arc<dyn InlineCompletionProviderHandle>,
    _subscription: Subscription,
}

enum ContextMenu {
    Completions(CompletionsMenu),
    CodeActions(CodeActionsMenu),
}

impl ContextMenu {
    fn select_first(
        &mut self,
        project: Option<&Model<Project>>,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                ContextMenu::Completions(menu) => menu.select_first(project, cx),
                ContextMenu::CodeActions(menu) => menu.select_first(cx),
            }
            true
        } else {
            false
        }
    }

    fn select_prev(
        &mut self,
        project: Option<&Model<Project>>,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                ContextMenu::Completions(menu) => menu.select_prev(project, cx),
                ContextMenu::CodeActions(menu) => menu.select_prev(cx),
            }
            true
        } else {
            false
        }
    }

    fn select_next(
        &mut self,
        project: Option<&Model<Project>>,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                ContextMenu::Completions(menu) => menu.select_next(project, cx),
                ContextMenu::CodeActions(menu) => menu.select_next(cx),
            }
            true
        } else {
            false
        }
    }

    fn select_last(
        &mut self,
        project: Option<&Model<Project>>,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                ContextMenu::Completions(menu) => menu.select_last(project, cx),
                ContextMenu::CodeActions(menu) => menu.select_last(cx),
            }
            true
        } else {
            false
        }
    }

    fn visible(&self) -> bool {
        match self {
            ContextMenu::Completions(menu) => menu.visible(),
            ContextMenu::CodeActions(menu) => menu.visible(),
        }
    }

    fn render(
        &self,
        cursor_position: DisplayPoint,
        style: &EditorStyle,
        max_height: Pixels,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut ViewContext<Editor>,
    ) -> (ContextMenuOrigin, AnyElement) {
        match self {
            ContextMenu::Completions(menu) => (
                ContextMenuOrigin::EditorPoint(cursor_position),
                menu.render(style, max_height, workspace, cx),
            ),
            ContextMenu::CodeActions(menu) => menu.render(cursor_position, style, max_height, cx),
        }
    }
}

enum ContextMenuOrigin {
    EditorPoint(DisplayPoint),
    GutterIndicator(DisplayRow),
}

#[derive(Clone)]
struct CompletionsMenu {
    id: CompletionId,
    initial_position: Anchor,
    buffer: Model<Buffer>,
    completions: Arc<RwLock<Box<[Completion]>>>,
    match_candidates: Arc<[StringMatchCandidate]>,
    matches: Arc<[StringMatch]>,
    selected_item: usize,
    scroll_handle: UniformListScrollHandle,
    selected_completion_documentation_resolve_debounce: Arc<Mutex<DebouncedDelay>>,
}

impl CompletionsMenu {
    fn select_first(&mut self, project: Option<&Model<Project>>, cx: &mut ViewContext<Editor>) {
        self.selected_item = 0;
        self.scroll_handle.scroll_to_item(self.selected_item);
        self.attempt_resolve_selected_completion_documentation(project, cx);
        cx.notify();
    }

    fn select_prev(&mut self, project: Option<&Model<Project>>, cx: &mut ViewContext<Editor>) {
        if self.selected_item > 0 {
            self.selected_item -= 1;
        } else {
            self.selected_item = self.matches.len() - 1;
        }
        self.scroll_handle.scroll_to_item(self.selected_item);
        self.attempt_resolve_selected_completion_documentation(project, cx);
        cx.notify();
    }

    fn select_next(&mut self, project: Option<&Model<Project>>, cx: &mut ViewContext<Editor>) {
        if self.selected_item + 1 < self.matches.len() {
            self.selected_item += 1;
        } else {
            self.selected_item = 0;
        }
        self.scroll_handle.scroll_to_item(self.selected_item);
        self.attempt_resolve_selected_completion_documentation(project, cx);
        cx.notify();
    }

    fn select_last(&mut self, project: Option<&Model<Project>>, cx: &mut ViewContext<Editor>) {
        self.selected_item = self.matches.len() - 1;
        self.scroll_handle.scroll_to_item(self.selected_item);
        self.attempt_resolve_selected_completion_documentation(project, cx);
        cx.notify();
    }

    fn pre_resolve_completion_documentation(
        buffer: Model<Buffer>,
        completions: Arc<RwLock<Box<[Completion]>>>,
        matches: Arc<[StringMatch]>,
        editor: &Editor,
        cx: &mut ViewContext<Editor>,
    ) -> Task<()> {
        let settings = EditorSettings::get_global(cx);
        if !settings.show_completion_documentation {
            return Task::ready(());
        }

        let Some(provider) = editor.completion_provider.as_ref() else {
            return Task::ready(());
        };

        let resolve_task = provider.resolve_completions(
            buffer,
            matches.iter().map(|m| m.candidate_id).collect(),
            completions.clone(),
            cx,
        );

        return cx.spawn(move |this, mut cx| async move {
            if let Some(true) = resolve_task.await.log_err() {
                this.update(&mut cx, |_, cx| cx.notify()).ok();
            }
        });
    }

    fn attempt_resolve_selected_completion_documentation(
        &mut self,
        project: Option<&Model<Project>>,
        cx: &mut ViewContext<Editor>,
    ) {
        let settings = EditorSettings::get_global(cx);
        if !settings.show_completion_documentation {
            return;
        }

        let completion_index = self.matches[self.selected_item].candidate_id;
        let Some(project) = project else {
            return;
        };

        let resolve_task = project.update(cx, |project, cx| {
            project.resolve_completions(
                self.buffer.clone(),
                vec![completion_index],
                self.completions.clone(),
                cx,
            )
        });

        let delay_ms =
            EditorSettings::get_global(cx).completion_documentation_secondary_query_debounce;
        let delay = Duration::from_millis(delay_ms);

        self.selected_completion_documentation_resolve_debounce
            .lock()
            .fire_new(delay, cx, |_, cx| {
                cx.spawn(move |this, mut cx| async move {
                    if let Some(true) = resolve_task.await.log_err() {
                        this.update(&mut cx, |_, cx| cx.notify()).ok();
                    }
                })
            });
    }

    fn visible(&self) -> bool {
        !self.matches.is_empty()
    }

    fn render(
        &self,
        style: &EditorStyle,
        max_height: Pixels,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut ViewContext<Editor>,
    ) -> AnyElement {
        let settings = EditorSettings::get_global(cx);
        let show_completion_documentation = settings.show_completion_documentation;

        let widest_completion_ix = self
            .matches
            .iter()
            .enumerate()
            .max_by_key(|(_, mat)| {
                let completions = self.completions.read();
                let completion = &completions[mat.candidate_id];
                let documentation = &completion.documentation;

                let mut len = completion.label.text.chars().count();
                if let Some(Documentation::SingleLine(text)) = documentation {
                    if show_completion_documentation {
                        len += text.chars().count();
                    }
                }

                len
            })
            .map(|(ix, _)| ix);

        let completions = self.completions.clone();
        let matches = self.matches.clone();
        let selected_item = self.selected_item;
        let style = style.clone();

        let multiline_docs = if show_completion_documentation {
            let mat = &self.matches[selected_item];
            let multiline_docs = match &self.completions.read()[mat.candidate_id].documentation {
                Some(Documentation::MultiLinePlainText(text)) => {
                    Some(div().child(SharedString::from(text.clone())))
                }
                Some(Documentation::MultiLineMarkdown(parsed)) if !parsed.text.is_empty() => {
                    Some(div().child(render_parsed_markdown(
                        "completions_markdown",
                        parsed,
                        &style,
                        workspace,
                        cx,
                    )))
                }
                _ => None,
            };
            multiline_docs.map(|div| {
                div.id("multiline_docs")
                    .max_h(max_height)
                    .flex_1()
                    .px_1p5()
                    .py_1()
                    .min_w(px(260.))
                    .max_w(px(640.))
                    .w(px(500.))
                    .overflow_y_scroll()
                    .occlude()
            })
        } else {
            None
        };

        let list = uniform_list(
            cx.view().clone(),
            "completions",
            matches.len(),
            move |_editor, range, cx| {
                let start_ix = range.start;
                let completions_guard = completions.read();

                matches[range]
                    .iter()
                    .enumerate()
                    .map(|(ix, mat)| {
                        let item_ix = start_ix + ix;
                        let candidate_id = mat.candidate_id;
                        let completion = &completions_guard[candidate_id];

                        let documentation = if show_completion_documentation {
                            &completion.documentation
                        } else {
                            &None
                        };

                        let highlights = gpui::combine_highlights(
                            mat.ranges().map(|range| (range, FontWeight::BOLD.into())),
                            styled_runs_for_code_label(&completion.label, &style.syntax).map(
                                |(range, mut highlight)| {
                                    // Ignore font weight for syntax highlighting, as we'll use it
                                    // for fuzzy matches.
                                    highlight.font_weight = None;

                                    if completion.lsp_completion.deprecated.unwrap_or(false) {
                                        highlight.strikethrough = Some(StrikethroughStyle {
                                            thickness: 1.0.into(),
                                            ..Default::default()
                                        });
                                        highlight.color = Some(cx.theme().colors().text_muted);
                                    }

                                    (range, highlight)
                                },
                            ),
                        );
                        let completion_label = StyledText::new(completion.label.text.clone())
                            .with_highlights(&style.text, highlights);
                        let documentation_label =
                            if let Some(Documentation::SingleLine(text)) = documentation {
                                if text.trim().is_empty() {
                                    None
                                } else {
                                    Some(
                                        Label::new(text.clone())
                                            .ml_4()
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                }
                            } else {
                                None
                            };

                        div().min_w(px(220.)).max_w(px(540.)).child(
                            ListItem::new(mat.candidate_id)
                                .inset(true)
                                .selected(item_ix == selected_item)
                                .on_click(cx.listener(move |editor, _event, cx| {
                                    cx.stop_propagation();
                                    if let Some(task) = editor.confirm_completion(
                                        &ConfirmCompletion {
                                            item_ix: Some(item_ix),
                                        },
                                        cx,
                                    ) {
                                        task.detach_and_log_err(cx)
                                    }
                                }))
                                .child(h_flex().overflow_hidden().child(completion_label))
                                .end_slot::<Label>(documentation_label),
                        )
                    })
                    .collect()
            },
        )
        .occlude()
        .max_h(max_height)
        .track_scroll(self.scroll_handle.clone())
        .with_width_from_item(widest_completion_ix)
        .with_sizing_behavior(ListSizingBehavior::Infer);

        Popover::new()
            .child(list)
            .when_some(multiline_docs, |popover, multiline_docs| {
                popover.aside(multiline_docs)
            })
            .into_any_element()
    }

    pub async fn filter(&mut self, query: Option<&str>, executor: BackgroundExecutor) {
        let mut matches = if let Some(query) = query {
            fuzzy::match_strings(
                &self.match_candidates,
                query,
                query.chars().any(|c| c.is_uppercase()),
                100,
                &Default::default(),
                executor,
            )
            .await
        } else {
            self.match_candidates
                .iter()
                .enumerate()
                .map(|(candidate_id, candidate)| StringMatch {
                    candidate_id,
                    score: Default::default(),
                    positions: Default::default(),
                    string: candidate.string.clone(),
                })
                .collect()
        };

        // Remove all candidates where the query's start does not match the start of any word in the candidate
        if let Some(query) = query {
            if let Some(query_start) = query.chars().next() {
                matches.retain(|string_match| {
                    split_words(&string_match.string).any(|word| {
                        // Check that the first codepoint of the word as lowercase matches the first
                        // codepoint of the query as lowercase
                        word.chars()
                            .flat_map(|codepoint| codepoint.to_lowercase())
                            .zip(query_start.to_lowercase())
                            .all(|(word_cp, query_cp)| word_cp == query_cp)
                    })
                });
            }
        }

        let completions = self.completions.read();
        matches.sort_unstable_by_key(|mat| {
            // We do want to strike a balance here between what the language server tells us
            // to sort by (the sort_text) and what are "obvious" good matches (i.e. when you type
            // `Creat` and there is a local variable called `CreateComponent`).
            // So what we do is: we bucket all matches into two buckets
            // - Strong matches
            // - Weak matches
            // Strong matches are the ones with a high fuzzy-matcher score (the "obvious" matches)
            // and the Weak matches are the rest.
            //
            // For the strong matches, we sort by the language-servers score first and for the weak
            // matches, we prefer our fuzzy finder first.
            //
            // The thinking behind that: it's useless to take the sort_text the language-server gives
            // us into account when it's obviously a bad match.

            #[derive(PartialEq, Eq, PartialOrd, Ord)]
            enum MatchScore<'a> {
                Strong {
                    sort_text: Option<&'a str>,
                    score: Reverse<OrderedFloat<f64>>,
                    sort_key: (usize, &'a str),
                },
                Weak {
                    score: Reverse<OrderedFloat<f64>>,
                    sort_text: Option<&'a str>,
                    sort_key: (usize, &'a str),
                },
            }

            let completion = &completions[mat.candidate_id];
            let sort_key = completion.sort_key();
            let sort_text = completion.lsp_completion.sort_text.as_deref();
            let score = Reverse(OrderedFloat(mat.score));

            if mat.score >= 0.2 {
                MatchScore::Strong {
                    sort_text,
                    score,
                    sort_key,
                }
            } else {
                MatchScore::Weak {
                    score,
                    sort_text,
                    sort_key,
                }
            }
        });

        for mat in &mut matches {
            let completion = &completions[mat.candidate_id];
            mat.string.clone_from(&completion.label.text);
            for position in &mut mat.positions {
                *position += completion.label.filter_range.start;
            }
        }
        drop(completions);

        self.matches = matches.into();
        self.selected_item = 0;
    }
}

#[derive(Clone)]
struct CodeActionContents {
    tasks: Option<Arc<ResolvedTasks>>,
    actions: Option<Arc<[CodeAction]>>,
}

impl CodeActionContents {
    fn len(&self) -> usize {
        match (&self.tasks, &self.actions) {
            (Some(tasks), Some(actions)) => actions.len() + tasks.templates.len(),
            (Some(tasks), None) => tasks.templates.len(),
            (None, Some(actions)) => actions.len(),
            (None, None) => 0,
        }
    }

    fn is_empty(&self) -> bool {
        match (&self.tasks, &self.actions) {
            (Some(tasks), Some(actions)) => actions.is_empty() && tasks.templates.is_empty(),
            (Some(tasks), None) => tasks.templates.is_empty(),
            (None, Some(actions)) => actions.is_empty(),
            (None, None) => true,
        }
    }

    fn iter(&self) -> impl Iterator<Item = CodeActionsItem> + '_ {
        self.tasks
            .iter()
            .flat_map(|tasks| {
                tasks
                    .templates
                    .iter()
                    .map(|(kind, task)| CodeActionsItem::Task(kind.clone(), task.clone()))
            })
            .chain(self.actions.iter().flat_map(|actions| {
                actions
                    .iter()
                    .map(|action| CodeActionsItem::CodeAction(action.clone()))
            }))
    }
    fn get(&self, index: usize) -> Option<CodeActionsItem> {
        match (&self.tasks, &self.actions) {
            (Some(tasks), Some(actions)) => {
                if index < tasks.templates.len() {
                    tasks
                        .templates
                        .get(index)
                        .cloned()
                        .map(|(kind, task)| CodeActionsItem::Task(kind, task))
                } else {
                    actions
                        .get(index - tasks.templates.len())
                        .cloned()
                        .map(CodeActionsItem::CodeAction)
                }
            }
            (Some(tasks), None) => tasks
                .templates
                .get(index)
                .cloned()
                .map(|(kind, task)| CodeActionsItem::Task(kind, task)),
            (None, Some(actions)) => actions.get(index).cloned().map(CodeActionsItem::CodeAction),
            (None, None) => None,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
enum CodeActionsItem {
    Task(TaskSourceKind, ResolvedTask),
    CodeAction(CodeAction),
}

impl CodeActionsItem {
    fn as_task(&self) -> Option<&ResolvedTask> {
        let Self::Task(_, task) = self else {
            return None;
        };
        Some(task)
    }
    fn as_code_action(&self) -> Option<&CodeAction> {
        let Self::CodeAction(action) = self else {
            return None;
        };
        Some(action)
    }
    fn label(&self) -> String {
        match self {
            Self::CodeAction(action) => action.lsp_action.title.clone(),
            Self::Task(_, task) => task.resolved_label.clone(),
        }
    }
}

struct CodeActionsMenu {
    actions: CodeActionContents,
    buffer: Model<Buffer>,
    selected_item: usize,
    scroll_handle: UniformListScrollHandle,
    deployed_from_indicator: Option<DisplayRow>,
}

impl CodeActionsMenu {
    fn select_first(&mut self, cx: &mut ViewContext<Editor>) {
        self.selected_item = 0;
        self.scroll_handle.scroll_to_item(self.selected_item);
        cx.notify()
    }

    fn select_prev(&mut self, cx: &mut ViewContext<Editor>) {
        if self.selected_item > 0 {
            self.selected_item -= 1;
        } else {
            self.selected_item = self.actions.len() - 1;
        }
        self.scroll_handle.scroll_to_item(self.selected_item);
        cx.notify();
    }

    fn select_next(&mut self, cx: &mut ViewContext<Editor>) {
        if self.selected_item + 1 < self.actions.len() {
            self.selected_item += 1;
        } else {
            self.selected_item = 0;
        }
        self.scroll_handle.scroll_to_item(self.selected_item);
        cx.notify();
    }

    fn select_last(&mut self, cx: &mut ViewContext<Editor>) {
        self.selected_item = self.actions.len() - 1;
        self.scroll_handle.scroll_to_item(self.selected_item);
        cx.notify()
    }

    fn visible(&self) -> bool {
        !self.actions.is_empty()
    }

    fn render(
        &self,
        cursor_position: DisplayPoint,
        _style: &EditorStyle,
        max_height: Pixels,
        cx: &mut ViewContext<Editor>,
    ) -> (ContextMenuOrigin, AnyElement) {
        let actions = self.actions.clone();
        let selected_item = self.selected_item;
        let element = uniform_list(
            cx.view().clone(),
            "code_actions_menu",
            self.actions.len(),
            move |_this, range, cx| {
                actions
                    .iter()
                    .skip(range.start)
                    .take(range.end - range.start)
                    .enumerate()
                    .map(|(ix, action)| {
                        let item_ix = range.start + ix;
                        let selected = selected_item == item_ix;
                        let colors = cx.theme().colors();
                        div()
                            .px_2()
                            .text_color(colors.text)
                            .when(selected, |style| {
                                style
                                    .bg(colors.element_active)
                                    .text_color(colors.text_accent)
                            })
                            .hover(|style| {
                                style
                                    .bg(colors.element_hover)
                                    .text_color(colors.text_accent)
                            })
                            .whitespace_nowrap()
                            .when_some(action.as_code_action(), |this, action| {
                                this.on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |editor, _, cx| {
                                        cx.stop_propagation();
                                        if let Some(task) = editor.confirm_code_action(
                                            &ConfirmCodeAction {
                                                item_ix: Some(item_ix),
                                            },
                                            cx,
                                        ) {
                                            task.detach_and_log_err(cx)
                                        }
                                    }),
                                )
                                // TASK: It would be good to make lsp_action.title a SharedString to avoid allocating here.
                                .child(SharedString::from(action.lsp_action.title.clone()))
                            })
                            .when_some(action.as_task(), |this, task| {
                                this.on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |editor, _, cx| {
                                        cx.stop_propagation();
                                        if let Some(task) = editor.confirm_code_action(
                                            &ConfirmCodeAction {
                                                item_ix: Some(item_ix),
                                            },
                                            cx,
                                        ) {
                                            task.detach_and_log_err(cx)
                                        }
                                    }),
                                )
                                .child(SharedString::from(task.resolved_label.clone()))
                            })
                    })
                    .collect()
            },
        )
        .elevation_1(cx)
        .px_2()
        .py_1()
        .max_h(max_height)
        .occlude()
        .track_scroll(self.scroll_handle.clone())
        .with_width_from_item(
            self.actions
                .iter()
                .enumerate()
                .max_by_key(|(_, action)| match action {
                    CodeActionsItem::Task(_, task) => task.resolved_label.chars().count(),
                    CodeActionsItem::CodeAction(action) => action.lsp_action.title.chars().count(),
                })
                .map(|(ix, _)| ix),
        )
        .with_sizing_behavior(ListSizingBehavior::Infer)
        .into_any_element();

        let cursor_position = if let Some(row) = self.deployed_from_indicator {
            ContextMenuOrigin::GutterIndicator(row)
        } else {
            ContextMenuOrigin::EditorPoint(cursor_position)
        };

        (cursor_position, element)
    }
}

#[derive(Debug)]
struct ActiveDiagnosticGroup {
    primary_range: Range<Anchor>,
    primary_message: String,
    group_id: usize,
    blocks: HashMap<CustomBlockId, Diagnostic>,
    is_valid: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClipboardSelection {
    pub len: usize,
    pub is_entire_line: bool,
    pub first_line_indent: u32,
}

#[derive(Debug)]
pub(crate) struct NavigationData {
    cursor_anchor: Anchor,
    cursor_position: Point,
    scroll_anchor: ScrollAnchor,
    scroll_top_row: u32,
}

enum GotoDefinitionKind {
    Symbol,
    Declaration,
    Type,
    Implementation,
}

#[derive(Debug, Clone)]
enum InlayHintRefreshReason {
    Toggle(bool),
    SettingsChange(InlayHintSettings),
    NewLinesShown,
    BufferEdited(HashSet<Arc<Language>>),
    RefreshRequested,
    ExcerptsRemoved(Vec<ExcerptId>),
}

impl InlayHintRefreshReason {
    fn description(&self) -> &'static str {
        match self {
            Self::Toggle(_) => "toggle",
            Self::SettingsChange(_) => "settings change",
            Self::NewLinesShown => "new lines shown",
            Self::BufferEdited(_) => "buffer edited",
            Self::RefreshRequested => "refresh requested",
            Self::ExcerptsRemoved(_) => "excerpts removed",
        }
    }
}

pub(crate) struct FocusedBlock {
    id: BlockId,
    focus_handle: WeakFocusHandle,
}

impl Editor {
    pub fn single_line(cx: &mut ViewContext<Self>) -> Self {
        let buffer = cx.new_model(|cx| Buffer::local("", cx));
        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
        Self::new(
            EditorMode::SingleLine { auto_width: false },
            buffer,
            None,
            false,
            cx,
        )
    }

    pub fn multi_line(cx: &mut ViewContext<Self>) -> Self {
        let buffer = cx.new_model(|cx| Buffer::local("", cx));
        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
        Self::new(EditorMode::Full, buffer, None, false, cx)
    }

    pub fn auto_width(cx: &mut ViewContext<Self>) -> Self {
        let buffer = cx.new_model(|cx| Buffer::local("", cx));
        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
        Self::new(
            EditorMode::SingleLine { auto_width: true },
            buffer,
            None,
            false,
            cx,
        )
    }

    pub fn auto_height(max_lines: usize, cx: &mut ViewContext<Self>) -> Self {
        let buffer = cx.new_model(|cx| Buffer::local("", cx));
        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
        Self::new(
            EditorMode::AutoHeight { max_lines },
            buffer,
            None,
            false,
            cx,
        )
    }

    pub fn for_buffer(
        buffer: Model<Buffer>,
        project: Option<Model<Project>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
        Self::new(EditorMode::Full, buffer, project, false, cx)
    }

    pub fn for_multibuffer(
        buffer: Model<MultiBuffer>,
        project: Option<Model<Project>>,
        show_excerpt_controls: bool,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self::new(EditorMode::Full, buffer, project, show_excerpt_controls, cx)
    }

    pub fn clone(&self, cx: &mut ViewContext<Self>) -> Self {
        let show_excerpt_controls = self.display_map.read(cx).show_excerpt_controls();
        let mut clone = Self::new(
            self.mode,
            self.buffer.clone(),
            self.project.clone(),
            show_excerpt_controls,
            cx,
        );
        self.display_map.update(cx, |display_map, cx| {
            let snapshot = display_map.snapshot(cx);
            clone.display_map.update(cx, |display_map, cx| {
                display_map.set_state(&snapshot, cx);
            });
        });
        clone.selections.clone_state(&self.selections);
        clone.scroll_manager.clone_state(&self.scroll_manager);
        clone.searchable = self.searchable;
        clone
    }

    pub fn new(
        mode: EditorMode,
        buffer: Model<MultiBuffer>,
        project: Option<Model<Project>>,
        show_excerpt_controls: bool,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let style = cx.text_style();
        let font_size = style.font_size.to_pixels(cx.rem_size());
        let editor = cx.view().downgrade();
        let fold_placeholder = FoldPlaceholder {
            constrain_width: true,
            render: Arc::new(move |fold_id, fold_range, cx| {
                let editor = editor.clone();
                div()
                    .id(fold_id)
                    .bg(cx.theme().colors().ghost_element_background)
                    .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                    .active(|style| style.bg(cx.theme().colors().ghost_element_active))
                    .rounded_sm()
                    .size_full()
                    .cursor_pointer()
                    .child("⋯")
                    .on_mouse_down(MouseButton::Left, |_, cx| cx.stop_propagation())
                    .on_click(move |_, cx| {
                        editor
                            .update(cx, |editor, cx| {
                                editor.unfold_ranges(
                                    [fold_range.start..fold_range.end],
                                    true,
                                    false,
                                    cx,
                                );
                                cx.stop_propagation();
                            })
                            .ok();
                    })
                    .into_any()
            }),
            merge_adjacent: true,
        };
        let file_header_size = if show_excerpt_controls { 3 } else { 2 };
        let display_map = cx.new_model(|cx| {
            DisplayMap::new(
                buffer.clone(),
                style.font(),
                font_size,
                None,
                show_excerpt_controls,
                file_header_size,
                MULTI_BUFFER_EXCERPT_HEADER_HEIGHT,
                MULTI_BUFFER_EXCERPT_FOOTER_HEIGHT,
                fold_placeholder,
                cx,
            )
        });

        let selections = SelectionsCollection::new(display_map.clone(), buffer.clone());

        let blink_manager = cx.new_model(|cx| BlinkManager::new(CURSOR_BLINK_INTERVAL, cx));

        let soft_wrap_mode_override = matches!(mode, EditorMode::SingleLine { .. })
            .then(|| language_settings::SoftWrap::PreferLine);

        let mut project_subscriptions = Vec::new();
        if mode == EditorMode::Full {
            if let Some(project) = project.as_ref() {
                if buffer.read(cx).is_singleton() {
                    project_subscriptions.push(cx.observe(project, |_, _, cx| {
                        cx.emit(EditorEvent::TitleChanged);
                    }));
                }
                project_subscriptions.push(cx.subscribe(project, |editor, _, event, cx| {
                    if let project::Event::RefreshInlayHints = event {
                        editor.refresh_inlay_hints(InlayHintRefreshReason::RefreshRequested, cx);
                    } else if let project::Event::SnippetEdit(id, snippet_edits) = event {
                        if let Some(buffer) = editor.buffer.read(cx).buffer(*id) {
                            let focus_handle = editor.focus_handle(cx);
                            if focus_handle.is_focused(cx) {
                                let snapshot = buffer.read(cx).snapshot();
                                for (range, snippet) in snippet_edits {
                                    let editor_range =
                                        language::range_from_lsp(*range).to_offset(&snapshot);
                                    editor
                                        .insert_snippet(&[editor_range], snippet.clone(), cx)
                                        .ok();
                                }
                            }
                        }
                    }
                }));
                let task_inventory = project.read(cx).task_inventory().clone();
                project_subscriptions.push(cx.observe(&task_inventory, |editor, _, cx| {
                    editor.tasks_update_task = Some(editor.refresh_runnables(cx));
                }));
            }
        }

        let inlay_hint_settings = inlay_hint_settings(
            selections.newest_anchor().head(),
            &buffer.read(cx).snapshot(cx),
            cx,
        );
        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, Self::handle_focus).detach();
        cx.on_focus_in(&focus_handle, Self::handle_focus_in)
            .detach();
        cx.on_focus_out(&focus_handle, Self::handle_focus_out)
            .detach();
        cx.on_blur(&focus_handle, Self::handle_blur).detach();

        let show_indent_guides = if matches!(mode, EditorMode::SingleLine { .. }) {
            Some(false)
        } else {
            None
        };

        let mut this = Self {
            focus_handle,
            show_cursor_when_unfocused: false,
            last_focused_descendant: None,
            buffer: buffer.clone(),
            display_map: display_map.clone(),
            selections,
            scroll_manager: ScrollManager::new(cx),
            columnar_selection_tail: None,
            add_selections_state: None,
            select_next_state: None,
            select_prev_state: None,
            selection_history: Default::default(),
            autoclose_regions: Default::default(),
            snippet_stack: Default::default(),
            select_larger_syntax_node_stack: Vec::new(),
            ime_transaction: Default::default(),
            active_diagnostics: None,
            soft_wrap_mode_override,
            completion_provider: project.clone().map(|project| Box::new(project) as _),
            collaboration_hub: project.clone().map(|project| Box::new(project) as _),
            project,
            blink_manager: blink_manager.clone(),
            show_local_selections: true,
            mode,
            show_breadcrumbs: EditorSettings::get_global(cx).toolbar.breadcrumbs,
            show_gutter: mode == EditorMode::Full,
            show_line_numbers: None,
            show_git_diff_gutter: None,
            show_code_actions: None,
            show_runnables: None,
            show_wrap_guides: None,
            show_indent_guides,
            placeholder_text: None,
            highlight_order: 0,
            highlighted_rows: HashMap::default(),
            background_highlights: Default::default(),
            gutter_highlights: TreeMap::default(),
            scrollbar_marker_state: ScrollbarMarkerState::default(),
            active_indent_guides_state: ActiveIndentGuidesState::default(),
            nav_history: None,
            context_menu: RwLock::new(None),
            mouse_context_menu: None,
            completion_tasks: Default::default(),
            signature_help_state: SignatureHelpState::default(),
            auto_signature_help: None,
            find_all_references_task_sources: Vec::new(),
            next_completion_id: 0,
            completion_documentation_pre_resolve_debounce: DebouncedDelay::new(),
            next_inlay_id: 0,
            available_code_actions: Default::default(),
            code_actions_task: Default::default(),
            document_highlights_task: Default::default(),
            linked_editing_range_task: Default::default(),
            pending_rename: Default::default(),
            searchable: true,
            cursor_shape: Default::default(),
            current_line_highlight: None,
            autoindent_mode: Some(AutoindentMode::EachLine),
            collapse_matches: false,
            workspace: None,
            keymap_context_layers: Default::default(),
            input_enabled: true,
            use_modal_editing: mode == EditorMode::Full,
            read_only: false,
            use_autoclose: true,
            use_auto_surround: true,
            auto_replace_emoji_shortcode: false,
            leader_peer_id: None,
            remote_id: None,
            hover_state: Default::default(),
            hovered_link_state: Default::default(),
            inline_completion_provider: None,
            active_inline_completion: None,
            inlay_hint_cache: InlayHintCache::new(inlay_hint_settings),
            expanded_hunks: ExpandedHunks::default(),
            gutter_hovered: false,
            pixel_position_of_newest_cursor: None,
            last_bounds: None,
            expect_bounds_change: None,
            gutter_dimensions: GutterDimensions::default(),
            style: None,
            show_cursor_names: false,
            hovered_cursors: Default::default(),
            next_editor_action_id: EditorActionId::default(),
            editor_actions: Rc::default(),
            vim_replace_map: Default::default(),
            show_inline_completions: mode == EditorMode::Full,
            custom_context_menu: None,
            show_git_blame_gutter: false,
            show_git_blame_inline: false,
            show_selection_menu: None,
            show_git_blame_inline_delay_task: None,
            git_blame_inline_enabled: ProjectSettings::get_global(cx).git.inline_blame_enabled(),
            serialize_dirty_buffers: ProjectSettings::get_global(cx)
                .session
                .restore_unsaved_buffers,
            blame: None,
            blame_subscription: None,
            file_header_size,
            tasks: Default::default(),
            _subscriptions: vec![
                cx.observe(&buffer, Self::on_buffer_changed),
                cx.subscribe(&buffer, Self::on_buffer_event),
                cx.observe(&display_map, Self::on_display_map_changed),
                cx.observe(&blink_manager, |_, _, cx| cx.notify()),
                cx.observe_global::<SettingsStore>(Self::settings_changed),
                observe_buffer_font_size_adjustment(cx, |_, cx| cx.notify()),
                cx.observe_window_activation(|editor, cx| {
                    let active = cx.is_window_active();
                    editor.blink_manager.update(cx, |blink_manager, cx| {
                        if active {
                            blink_manager.enable(cx);
                        } else {
                            blink_manager.disable(cx);
                        }
                    });
                }),
            ],
            tasks_update_task: None,
            linked_edit_ranges: Default::default(),
            previous_search_ranges: None,
            breadcrumb_header: None,
            focused_block: None,
            next_scroll_direction: Default::default(),
            _scroll_cursor_center_top_bottom_task: Task::ready(()),
        };
        this.tasks_update_task = Some(this.refresh_runnables(cx));
        this._subscriptions.extend(project_subscriptions);

        this.end_selection(cx);
        this.scroll_manager.show_scrollbar(cx);

        if mode == EditorMode::Full {
            let should_auto_hide_scrollbars = cx.should_auto_hide_scrollbars();
            cx.set_global(ScrollbarAutoHide(should_auto_hide_scrollbars));

            if this.git_blame_inline_enabled {
                this.git_blame_inline_enabled = true;
                this.start_git_blame_inline(false, cx);
            }
        }

        this.report_editor_event("open", None, cx);
        this
    }

    pub fn mouse_menu_is_focused(&self, cx: &mut WindowContext) -> bool {
        self.mouse_context_menu
            .as_ref()
            .is_some_and(|menu| menu.context_menu.focus_handle(cx).is_focused(cx))
    }

    fn key_context(&self, cx: &AppContext) -> KeyContext {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("Editor");
        let mode = match self.mode {
            EditorMode::SingleLine { .. } => "single_line",
            EditorMode::AutoHeight { .. } => "auto_height",
            EditorMode::Full => "full",
        };

        if EditorSettings::jupyter_enabled(cx) {
            key_context.add("jupyter");
        }

        key_context.set("mode", mode);
        if self.pending_rename.is_some() {
            key_context.add("renaming");
        }
        if self.context_menu_visible() {
            match self.context_menu.read().as_ref() {
                Some(ContextMenu::Completions(_)) => {
                    key_context.add("menu");
                    key_context.add("showing_completions")
                }
                Some(ContextMenu::CodeActions(_)) => {
                    key_context.add("menu");
                    key_context.add("showing_code_actions")
                }
                None => {}
            }
        }

        for layer in self.keymap_context_layers.values() {
            key_context.extend(layer);
        }

        if let Some(extension) = self
            .buffer
            .read(cx)
            .as_singleton()
            .and_then(|buffer| buffer.read(cx).file()?.path().extension()?.to_str())
        {
            key_context.set("extension", extension.to_string());
        }

        if self.has_active_inline_completion(cx) {
            key_context.add("copilot_suggestion");
            key_context.add("inline_completion");
        }

        key_context
    }

    pub fn new_file(
        workspace: &mut Workspace,
        _: &workspace::NewFile,
        cx: &mut ViewContext<Workspace>,
    ) {
        Self::new_in_workspace(workspace, cx).detach_and_prompt_err(
            "Failed to create buffer",
            cx,
            |e, _| match e.error_code() {
                ErrorCode::RemoteUpgradeRequired => Some(format!(
                "The remote instance of Zed does not support this yet. It must be upgraded to {}",
                e.error_tag("required").unwrap_or("the latest version")
            )),
                _ => None,
            },
        );
    }

    pub fn new_in_workspace(
        workspace: &mut Workspace,
        cx: &mut ViewContext<Workspace>,
    ) -> Task<Result<View<Editor>>> {
        let project = workspace.project().clone();
        let create = project.update(cx, |project, cx| project.create_buffer(cx));

        cx.spawn(|workspace, mut cx| async move {
            let buffer = create.await?;
            workspace.update(&mut cx, |workspace, cx| {
                let editor =
                    cx.new_view(|cx| Editor::for_buffer(buffer, Some(project.clone()), cx));
                workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, cx);
                editor
            })
        })
    }

    pub fn new_file_in_direction(
        workspace: &mut Workspace,
        action: &workspace::NewFileInDirection,
        cx: &mut ViewContext<Workspace>,
    ) {
        let project = workspace.project().clone();
        let create = project.update(cx, |project, cx| project.create_buffer(cx));
        let direction = action.0;

        cx.spawn(|workspace, mut cx| async move {
            let buffer = create.await?;
            workspace.update(&mut cx, move |workspace, cx| {
                workspace.split_item(
                    direction,
                    Box::new(
                        cx.new_view(|cx| Editor::for_buffer(buffer, Some(project.clone()), cx)),
                    ),
                    cx,
                )
            })?;
            anyhow::Ok(())
        })
        .detach_and_prompt_err("Failed to create buffer", cx, |e, _| match e.error_code() {
            ErrorCode::RemoteUpgradeRequired => Some(format!(
                "The remote instance of Zed does not support this yet. It must be upgraded to {}",
                e.error_tag("required").unwrap_or("the latest version")
            )),
            _ => None,
        });
    }

    pub fn replica_id(&self, cx: &AppContext) -> ReplicaId {
        self.buffer.read(cx).replica_id()
    }

    pub fn leader_peer_id(&self) -> Option<PeerId> {
        self.leader_peer_id
    }

    pub fn buffer(&self) -> &Model<MultiBuffer> {
        &self.buffer
    }

    pub fn workspace(&self) -> Option<View<Workspace>> {
        self.workspace.as_ref()?.0.upgrade()
    }

    pub fn title<'a>(&self, cx: &'a AppContext) -> Cow<'a, str> {
        self.buffer().read(cx).title(cx)
    }

    pub fn snapshot(&mut self, cx: &mut WindowContext) -> EditorSnapshot {
        EditorSnapshot {
            mode: self.mode,
            show_gutter: self.show_gutter,
            show_line_numbers: self.show_line_numbers,
            show_git_diff_gutter: self.show_git_diff_gutter,
            show_code_actions: self.show_code_actions,
            show_runnables: self.show_runnables,
            render_git_blame_gutter: self.render_git_blame_gutter(cx),
            display_snapshot: self.display_map.update(cx, |map, cx| map.snapshot(cx)),
            scroll_anchor: self.scroll_manager.anchor(),
            ongoing_scroll: self.scroll_manager.ongoing_scroll(),
            placeholder_text: self.placeholder_text.clone(),
            is_focused: self.focus_handle.is_focused(cx),
            current_line_highlight: self
                .current_line_highlight
                .unwrap_or_else(|| EditorSettings::get_global(cx).current_line_highlight),
            gutter_hovered: self.gutter_hovered,
        }
    }

    pub fn language_at<T: ToOffset>(&self, point: T, cx: &AppContext) -> Option<Arc<Language>> {
        self.buffer.read(cx).language_at(point, cx)
    }

    pub fn file_at<T: ToOffset>(
        &self,
        point: T,
        cx: &AppContext,
    ) -> Option<Arc<dyn language::File>> {
        self.buffer.read(cx).read(cx).file_at(point).cloned()
    }

    pub fn active_excerpt(
        &self,
        cx: &AppContext,
    ) -> Option<(ExcerptId, Model<Buffer>, Range<text::Anchor>)> {
        self.buffer
            .read(cx)
            .excerpt_containing(self.selections.newest_anchor().head(), cx)
    }

    pub fn mode(&self) -> EditorMode {
        self.mode
    }

    pub fn collaboration_hub(&self) -> Option<&dyn CollaborationHub> {
        self.collaboration_hub.as_deref()
    }

    pub fn set_collaboration_hub(&mut self, hub: Box<dyn CollaborationHub>) {
        self.collaboration_hub = Some(hub);
    }

    pub fn set_custom_context_menu(
        &mut self,
        f: impl 'static
            + Fn(&mut Self, DisplayPoint, &mut ViewContext<Self>) -> Option<View<ui::ContextMenu>>,
    ) {
        self.custom_context_menu = Some(Box::new(f))
    }

    pub fn set_completion_provider(&mut self, provider: Box<dyn CompletionProvider>) {
        self.completion_provider = Some(provider);
    }

    pub fn set_inline_completion_provider<T>(
        &mut self,
        provider: Option<Model<T>>,
        cx: &mut ViewContext<Self>,
    ) where
        T: InlineCompletionProvider,
    {
        self.inline_completion_provider =
            provider.map(|provider| RegisteredInlineCompletionProvider {
                _subscription: cx.observe(&provider, |this, _, cx| {
                    if this.focus_handle.is_focused(cx) {
                        this.update_visible_inline_completion(cx);
                    }
                }),
                provider: Arc::new(provider),
            });
        self.refresh_inline_completion(false, cx);
    }

    pub fn placeholder_text(&self, _cx: &WindowContext) -> Option<&str> {
        self.placeholder_text.as_deref()
    }

    pub fn set_placeholder_text(
        &mut self,
        placeholder_text: impl Into<Arc<str>>,
        cx: &mut ViewContext<Self>,
    ) {
        let placeholder_text = Some(placeholder_text.into());
        if self.placeholder_text != placeholder_text {
            self.placeholder_text = placeholder_text;
            cx.notify();
        }
    }

    pub fn set_cursor_shape(&mut self, cursor_shape: CursorShape, cx: &mut ViewContext<Self>) {
        self.cursor_shape = cursor_shape;

        // Disrupt blink for immediate user feedback that the cursor shape has changed
        self.blink_manager.update(cx, BlinkManager::show_cursor);

        cx.notify();
    }

    pub fn set_current_line_highlight(
        &mut self,
        current_line_highlight: Option<CurrentLineHighlight>,
    ) {
        self.current_line_highlight = current_line_highlight;
    }

    pub fn set_collapse_matches(&mut self, collapse_matches: bool) {
        self.collapse_matches = collapse_matches;
    }

    pub fn range_for_match<T: std::marker::Copy>(&self, range: &Range<T>) -> Range<T> {
        if self.collapse_matches {
            return range.start..range.start;
        }
        range.clone()
    }

    pub fn set_clip_at_line_ends(&mut self, clip: bool, cx: &mut ViewContext<Self>) {
        if self.display_map.read(cx).clip_at_line_ends != clip {
            self.display_map
                .update(cx, |map, _| map.clip_at_line_ends = clip);
        }
    }

    pub fn set_keymap_context_layer<Tag: 'static>(
        &mut self,
        context: KeyContext,
        cx: &mut ViewContext<Self>,
    ) {
        self.keymap_context_layers
            .insert(TypeId::of::<Tag>(), context);
        cx.notify();
    }

    pub fn remove_keymap_context_layer<Tag: 'static>(&mut self, cx: &mut ViewContext<Self>) {
        self.keymap_context_layers.remove(&TypeId::of::<Tag>());
        cx.notify();
    }

    pub fn set_input_enabled(&mut self, input_enabled: bool) {
        self.input_enabled = input_enabled;
    }

    pub fn set_autoindent(&mut self, autoindent: bool) {
        if autoindent {
            self.autoindent_mode = Some(AutoindentMode::EachLine);
        } else {
            self.autoindent_mode = None;
        }
    }

    pub fn read_only(&self, cx: &AppContext) -> bool {
        self.read_only || self.buffer.read(cx).read_only()
    }

    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    pub fn set_use_autoclose(&mut self, autoclose: bool) {
        self.use_autoclose = autoclose;
    }

    pub fn set_use_auto_surround(&mut self, auto_surround: bool) {
        self.use_auto_surround = auto_surround;
    }

    pub fn set_auto_replace_emoji_shortcode(&mut self, auto_replace: bool) {
        self.auto_replace_emoji_shortcode = auto_replace;
    }

    pub fn set_show_inline_completions(&mut self, show_inline_completions: bool) {
        self.show_inline_completions = show_inline_completions;
    }

    pub fn set_use_modal_editing(&mut self, to: bool) {
        self.use_modal_editing = to;
    }

    pub fn use_modal_editing(&self) -> bool {
        self.use_modal_editing
    }

    fn selections_did_change(
        &mut self,
        local: bool,
        old_cursor_position: &Anchor,
        show_completions: bool,
        cx: &mut ViewContext<Self>,
    ) {
        // Copy selections to primary selection buffer
        #[cfg(target_os = "linux")]
        if local {
            let selections = self.selections.all::<usize>(cx);
            let buffer_handle = self.buffer.read(cx).read(cx);

            let mut text = String::new();
            for (index, selection) in selections.iter().enumerate() {
                let text_for_selection = buffer_handle
                    .text_for_range(selection.start..selection.end)
                    .collect::<String>();

                text.push_str(&text_for_selection);
                if index != selections.len() - 1 {
                    text.push('\n');
                }
            }

            if !text.is_empty() {
                cx.write_to_primary(ClipboardItem::new(text));
            }
        }

        if self.focus_handle.is_focused(cx) && self.leader_peer_id.is_none() {
            self.buffer.update(cx, |buffer, cx| {
                buffer.set_active_selections(
                    &self.selections.disjoint_anchors(),
                    self.selections.line_mode,
                    self.cursor_shape,
                    cx,
                )
            });
        }
        let display_map = self
            .display_map
            .update(cx, |display_map, cx| display_map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;
        self.add_selections_state = None;
        self.select_next_state = None;
        self.select_prev_state = None;
        self.select_larger_syntax_node_stack.clear();
        self.invalidate_autoclose_regions(&self.selections.disjoint_anchors(), buffer);
        self.snippet_stack
            .invalidate(&self.selections.disjoint_anchors(), buffer);
        self.take_rename(false, cx);

        let new_cursor_position = self.selections.newest_anchor().head();

        self.push_to_nav_history(
            *old_cursor_position,
            Some(new_cursor_position.to_point(buffer)),
            cx,
        );

        if local {
            let new_cursor_position = self.selections.newest_anchor().head();
            let mut context_menu = self.context_menu.write();
            let completion_menu = match context_menu.as_ref() {
                Some(ContextMenu::Completions(menu)) => Some(menu),

                _ => {
                    *context_menu = None;
                    None
                }
            };

            if let Some(completion_menu) = completion_menu {
                let cursor_position = new_cursor_position.to_offset(buffer);
                let (word_range, kind) = buffer.surrounding_word(completion_menu.initial_position);
                if kind == Some(CharKind::Word)
                    && word_range.to_inclusive().contains(&cursor_position)
                {
                    let mut completion_menu = completion_menu.clone();
                    drop(context_menu);

                    let query = Self::completion_query(buffer, cursor_position);
                    cx.spawn(move |this, mut cx| async move {
                        completion_menu
                            .filter(query.as_deref(), cx.background_executor().clone())
                            .await;

                        this.update(&mut cx, |this, cx| {
                            let mut context_menu = this.context_menu.write();
                            let Some(ContextMenu::Completions(menu)) = context_menu.as_ref() else {
                                return;
                            };

                            if menu.id > completion_menu.id {
                                return;
                            }

                            *context_menu = Some(ContextMenu::Completions(completion_menu));
                            drop(context_menu);
                            cx.notify();
                        })
                    })
                    .detach();

                    if show_completions {
                        self.show_completions(&ShowCompletions { trigger: None }, cx);
                    }
                } else {
                    drop(context_menu);
                    self.hide_context_menu(cx);
                }
            } else {
                drop(context_menu);
            }

            hide_hover(self, cx);

            if old_cursor_position.to_display_point(&display_map).row()
                != new_cursor_position.to_display_point(&display_map).row()
            {
                self.available_code_actions.take();
            }
            self.refresh_code_actions(cx);
            self.refresh_document_highlights(cx);
            refresh_matching_bracket_highlights(self, cx);
            self.discard_inline_completion(false, cx);
            linked_editing_ranges::refresh_linked_ranges(self, cx);
            if self.git_blame_inline_enabled {
                self.start_inline_blame_timer(cx);
            }
        }

        self.blink_manager.update(cx, BlinkManager::pause_blinking);
        cx.emit(EditorEvent::SelectionsChanged { local });

        if self.selections.disjoint_anchors().len() == 1 {
            cx.emit(SearchEvent::ActiveMatchChanged)
        }
        cx.notify();
    }

    pub fn change_selections<R>(
        &mut self,
        autoscroll: Option<Autoscroll>,
        cx: &mut ViewContext<Self>,
        change: impl FnOnce(&mut MutableSelectionsCollection<'_>) -> R,
    ) -> R {
        self.change_selections_inner(autoscroll, true, cx, change)
    }

    pub fn change_selections_inner<R>(
        &mut self,
        autoscroll: Option<Autoscroll>,
        request_completions: bool,
        cx: &mut ViewContext<Self>,
        change: impl FnOnce(&mut MutableSelectionsCollection<'_>) -> R,
    ) -> R {
        let old_cursor_position = self.selections.newest_anchor().head();
        self.push_to_selection_history();

        let (changed, result) = self.selections.change_with(cx, change);

        if changed {
            if let Some(autoscroll) = autoscroll {
                self.request_autoscroll(autoscroll, cx);
            }
            self.selections_did_change(true, &old_cursor_position, request_completions, cx);

            if self.should_open_signature_help_automatically(
                &old_cursor_position,
                self.signature_help_state.backspace_pressed(),
                cx,
            ) {
                self.show_signature_help(&ShowSignatureHelp, cx);
            }
            self.signature_help_state.set_backspace_pressed(false);
        }

        result
    }

    pub fn edit<I, S, T>(&mut self, edits: I, cx: &mut ViewContext<Self>)
    where
        I: IntoIterator<Item = (Range<S>, T)>,
        S: ToOffset,
        T: Into<Arc<str>>,
    {
        if self.read_only(cx) {
            return;
        }

        self.buffer
            .update(cx, |buffer, cx| buffer.edit(edits, None, cx));
    }

    pub fn edit_with_autoindent<I, S, T>(&mut self, edits: I, cx: &mut ViewContext<Self>)
    where
        I: IntoIterator<Item = (Range<S>, T)>,
        S: ToOffset,
        T: Into<Arc<str>>,
    {
        if self.read_only(cx) {
            return;
        }

        self.buffer.update(cx, |buffer, cx| {
            buffer.edit(edits, self.autoindent_mode.clone(), cx)
        });
    }

    pub fn edit_with_block_indent<I, S, T>(
        &mut self,
        edits: I,
        original_indent_columns: Vec<u32>,
        cx: &mut ViewContext<Self>,
    ) where
        I: IntoIterator<Item = (Range<S>, T)>,
        S: ToOffset,
        T: Into<Arc<str>>,
    {
        if self.read_only(cx) {
            return;
        }

        self.buffer.update(cx, |buffer, cx| {
            buffer.edit(
                edits,
                Some(AutoindentMode::Block {
                    original_indent_columns,
                }),
                cx,
            )
        });
    }

    fn select(&mut self, phase: SelectPhase, cx: &mut ViewContext<Self>) {
        self.hide_context_menu(cx);

        match phase {
            SelectPhase::Begin {
                position,
                add,
                click_count,
            } => self.begin_selection(position, add, click_count, cx),
            SelectPhase::BeginColumnar {
                position,
                goal_column,
                reset,
            } => self.begin_columnar_selection(position, goal_column, reset, cx),
            SelectPhase::Extend {
                position,
                click_count,
            } => self.extend_selection(position, click_count, cx),
            SelectPhase::Update {
                position,
                goal_column,
                scroll_delta,
            } => self.update_selection(position, goal_column, scroll_delta, cx),
            SelectPhase::End => self.end_selection(cx),
        }
    }

    fn extend_selection(
        &mut self,
        position: DisplayPoint,
        click_count: usize,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let tail = self.selections.newest::<usize>(cx).tail();
        self.begin_selection(position, false, click_count, cx);

        let position = position.to_offset(&display_map, Bias::Left);
        let tail_anchor = display_map.buffer_snapshot.anchor_before(tail);

        let mut pending_selection = self
            .selections
            .pending_anchor()
            .expect("extend_selection not called with pending selection");
        if position >= tail {
            pending_selection.start = tail_anchor;
        } else {
            pending_selection.end = tail_anchor;
            pending_selection.reversed = true;
        }

        let mut pending_mode = self.selections.pending_mode().unwrap();
        match &mut pending_mode {
            SelectMode::Word(range) | SelectMode::Line(range) => *range = tail_anchor..tail_anchor,
            _ => {}
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.set_pending(pending_selection, pending_mode)
        });
    }

    fn begin_selection(
        &mut self,
        position: DisplayPoint,
        add: bool,
        click_count: usize,
        cx: &mut ViewContext<Self>,
    ) {
        if !self.focus_handle.is_focused(cx) {
            self.last_focused_descendant = None;
            cx.focus(&self.focus_handle);
        }

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;
        let newest_selection = self.selections.newest_anchor().clone();
        let position = display_map.clip_point(position, Bias::Left);

        let start;
        let end;
        let mode;
        let auto_scroll;
        match click_count {
            1 => {
                start = buffer.anchor_before(position.to_point(&display_map));
                end = start;
                mode = SelectMode::Character;
                auto_scroll = true;
            }
            2 => {
                let range = movement::surrounding_word(&display_map, position);
                start = buffer.anchor_before(range.start.to_point(&display_map));
                end = buffer.anchor_before(range.end.to_point(&display_map));
                mode = SelectMode::Word(start..end);
                auto_scroll = true;
            }
            3 => {
                let position = display_map
                    .clip_point(position, Bias::Left)
                    .to_point(&display_map);
                let line_start = display_map.prev_line_boundary(position).0;
                let next_line_start = buffer.clip_point(
                    display_map.next_line_boundary(position).0 + Point::new(1, 0),
                    Bias::Left,
                );
                start = buffer.anchor_before(line_start);
                end = buffer.anchor_before(next_line_start);
                mode = SelectMode::Line(start..end);
                auto_scroll = true;
            }
            _ => {
                start = buffer.anchor_before(0);
                end = buffer.anchor_before(buffer.len());
                mode = SelectMode::All;
                auto_scroll = false;
            }
        }

        let point_to_delete: Option<usize> = {
            let selected_points: Vec<Selection<Point>> =
                self.selections.disjoint_in_range(start..end, cx);

            if !add || click_count > 1 {
                None
            } else if selected_points.len() > 0 {
                Some(selected_points[0].id)
            } else {
                let clicked_point_already_selected =
                    self.selections.disjoint.iter().find(|selection| {
                        selection.start.to_point(buffer) == start.to_point(buffer)
                            || selection.end.to_point(buffer) == end.to_point(buffer)
                    });

                if let Some(selection) = clicked_point_already_selected {
                    Some(selection.id)
                } else {
                    None
                }
            }
        };

        let selections_count = self.selections.count();

        self.change_selections(auto_scroll.then(|| Autoscroll::newest()), cx, |s| {
            if let Some(point_to_delete) = point_to_delete {
                s.delete(point_to_delete);

                if selections_count == 1 {
                    s.set_pending_anchor_range(start..end, mode);
                }
            } else {
                if !add {
                    s.clear_disjoint();
                } else if click_count > 1 {
                    s.delete(newest_selection.id)
                }

                s.set_pending_anchor_range(start..end, mode);
            }
        });
    }

    fn begin_columnar_selection(
        &mut self,
        position: DisplayPoint,
        goal_column: u32,
        reset: bool,
        cx: &mut ViewContext<Self>,
    ) {
        if !self.focus_handle.is_focused(cx) {
            self.last_focused_descendant = None;
            cx.focus(&self.focus_handle);
        }

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        if reset {
            let pointer_position = display_map
                .buffer_snapshot
                .anchor_before(position.to_point(&display_map));

            self.change_selections(Some(Autoscroll::newest()), cx, |s| {
                s.clear_disjoint();
                s.set_pending_anchor_range(
                    pointer_position..pointer_position,
                    SelectMode::Character,
                );
            });
        }

        let tail = self.selections.newest::<Point>(cx).tail();
        self.columnar_selection_tail = Some(display_map.buffer_snapshot.anchor_before(tail));

        if !reset {
            self.select_columns(
                tail.to_display_point(&display_map),
                position,
                goal_column,
                &display_map,
                cx,
            );
        }
    }

    fn update_selection(
        &mut self,
        position: DisplayPoint,
        goal_column: u32,
        scroll_delta: gpui::Point<f32>,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        if let Some(tail) = self.columnar_selection_tail.as_ref() {
            let tail = tail.to_display_point(&display_map);
            self.select_columns(tail, position, goal_column, &display_map, cx);
        } else if let Some(mut pending) = self.selections.pending_anchor() {
            let buffer = self.buffer.read(cx).snapshot(cx);
            let head;
            let tail;
            let mode = self.selections.pending_mode().unwrap();
            match &mode {
                SelectMode::Character => {
                    head = position.to_point(&display_map);
                    tail = pending.tail().to_point(&buffer);
                }
                SelectMode::Word(original_range) => {
                    let original_display_range = original_range.start.to_display_point(&display_map)
                        ..original_range.end.to_display_point(&display_map);
                    let original_buffer_range = original_display_range.start.to_point(&display_map)
                        ..original_display_range.end.to_point(&display_map);
                    if movement::is_inside_word(&display_map, position)
                        || original_display_range.contains(&position)
                    {
                        let word_range = movement::surrounding_word(&display_map, position);
                        if word_range.start < original_display_range.start {
                            head = word_range.start.to_point(&display_map);
                        } else {
                            head = word_range.end.to_point(&display_map);
                        }
                    } else {
                        head = position.to_point(&display_map);
                    }

                    if head <= original_buffer_range.start {
                        tail = original_buffer_range.end;
                    } else {
                        tail = original_buffer_range.start;
                    }
                }
                SelectMode::Line(original_range) => {
                    let original_range = original_range.to_point(&display_map.buffer_snapshot);

                    let position = display_map
                        .clip_point(position, Bias::Left)
                        .to_point(&display_map);
                    let line_start = display_map.prev_line_boundary(position).0;
                    let next_line_start = buffer.clip_point(
                        display_map.next_line_boundary(position).0 + Point::new(1, 0),
                        Bias::Left,
                    );

                    if line_start < original_range.start {
                        head = line_start
                    } else {
                        head = next_line_start
                    }

                    if head <= original_range.start {
                        tail = original_range.end;
                    } else {
                        tail = original_range.start;
                    }
                }
                SelectMode::All => {
                    return;
                }
            };

            if head < tail {
                pending.start = buffer.anchor_before(head);
                pending.end = buffer.anchor_before(tail);
                pending.reversed = true;
            } else {
                pending.start = buffer.anchor_before(tail);
                pending.end = buffer.anchor_before(head);
                pending.reversed = false;
            }

            self.change_selections(None, cx, |s| {
                s.set_pending(pending, mode);
            });
        } else {
            log::error!("update_selection dispatched with no pending selection");
            return;
        }

        self.apply_scroll_delta(scroll_delta, cx);
        cx.notify();
    }

    fn end_selection(&mut self, cx: &mut ViewContext<Self>) {
        self.columnar_selection_tail.take();
        if self.selections.pending_anchor().is_some() {
            let selections = self.selections.all::<usize>(cx);
            self.change_selections(None, cx, |s| {
                s.select(selections);
                s.clear_pending();
            });
        }
    }

    fn select_columns(
        &mut self,
        tail: DisplayPoint,
        head: DisplayPoint,
        goal_column: u32,
        display_map: &DisplaySnapshot,
        cx: &mut ViewContext<Self>,
    ) {
        let start_row = cmp::min(tail.row(), head.row());
        let end_row = cmp::max(tail.row(), head.row());
        let start_column = cmp::min(tail.column(), goal_column);
        let end_column = cmp::max(tail.column(), goal_column);
        let reversed = start_column < tail.column();

        let selection_ranges = (start_row.0..=end_row.0)
            .map(DisplayRow)
            .filter_map(|row| {
                if start_column <= display_map.line_len(row) && !display_map.is_block_line(row) {
                    let start = display_map
                        .clip_point(DisplayPoint::new(row, start_column), Bias::Left)
                        .to_point(display_map);
                    let end = display_map
                        .clip_point(DisplayPoint::new(row, end_column), Bias::Right)
                        .to_point(display_map);
                    if reversed {
                        Some(end..start)
                    } else {
                        Some(start..end)
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        self.change_selections(None, cx, |s| {
            s.select_ranges(selection_ranges);
        });
        cx.notify();
    }

    pub fn has_pending_nonempty_selection(&self) -> bool {
        let pending_nonempty_selection = match self.selections.pending_anchor() {
            Some(Selection { start, end, .. }) => start != end,
            None => false,
        };

        pending_nonempty_selection
            || (self.columnar_selection_tail.is_some() && self.selections.disjoint.len() > 1)
    }

    pub fn has_pending_selection(&self) -> bool {
        self.selections.pending_anchor().is_some() || self.columnar_selection_tail.is_some()
    }

    pub fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        if self.clear_clicked_diff_hunks(cx) {
            cx.notify();
            return;
        }
        if self.dismiss_menus_and_popups(true, cx) {
            return;
        }

        if self.mode == EditorMode::Full {
            if self.change_selections(Some(Autoscroll::fit()), cx, |s| s.try_cancel()) {
                return;
            }
        }

        cx.propagate();
    }

    pub fn dismiss_menus_and_popups(
        &mut self,
        should_report_inline_completion_event: bool,
        cx: &mut ViewContext<Self>,
    ) -> bool {
        if self.take_rename(false, cx).is_some() {
            return true;
        }

        if hide_hover(self, cx) {
            return true;
        }

        if self.hide_signature_help(cx, SignatureHelpHiddenBy::Escape) {
            return true;
        }

        if self.hide_context_menu(cx).is_some() {
            return true;
        }

        if self.mouse_context_menu.take().is_some() {
            return true;
        }

        if self.discard_inline_completion(should_report_inline_completion_event, cx) {
            return true;
        }

        if self.snippet_stack.pop().is_some() {
            return true;
        }

        if self.mode == EditorMode::Full {
            if self.active_diagnostics.is_some() {
                self.dismiss_diagnostics(cx);
                return true;
            }
        }

        false
    }

    fn linked_editing_ranges_for(
        &self,
        selection: Range<text::Anchor>,
        cx: &AppContext,
    ) -> Option<HashMap<Model<Buffer>, Vec<Range<text::Anchor>>>> {
        if self.linked_edit_ranges.is_empty() {
            return None;
        }
        let ((base_range, linked_ranges), buffer_snapshot, buffer) =
            selection.end.buffer_id.and_then(|end_buffer_id| {
                if selection.start.buffer_id != Some(end_buffer_id) {
                    return None;
                }
                let buffer = self.buffer.read(cx).buffer(end_buffer_id)?;
                let snapshot = buffer.read(cx).snapshot();
                self.linked_edit_ranges
                    .get(end_buffer_id, selection.start..selection.end, &snapshot)
                    .map(|ranges| (ranges, snapshot, buffer))
            })?;
        use text::ToOffset as TO;
        // find offset from the start of current range to current cursor position
        let start_byte_offset = TO::to_offset(&base_range.start, &buffer_snapshot);

        let start_offset = TO::to_offset(&selection.start, &buffer_snapshot);
        let start_difference = start_offset - start_byte_offset;
        let end_offset = TO::to_offset(&selection.end, &buffer_snapshot);
        let end_difference = end_offset - start_byte_offset;
        // Current range has associated linked ranges.
        let mut linked_edits = HashMap::<_, Vec<_>>::default();
        for range in linked_ranges.iter() {
            let start_offset = TO::to_offset(&range.start, &buffer_snapshot);
            let end_offset = start_offset + end_difference;
            let start_offset = start_offset + start_difference;
            if start_offset > buffer_snapshot.len() || end_offset > buffer_snapshot.len() {
                continue;
            }
            let start = buffer_snapshot.anchor_after(start_offset);
            let end = buffer_snapshot.anchor_after(end_offset);
            linked_edits
                .entry(buffer.clone())
                .or_default()
                .push(start..end);
        }
        Some(linked_edits)
    }

    pub fn handle_input(&mut self, text: &str, cx: &mut ViewContext<Self>) {
        let text: Arc<str> = text.into();

        if self.read_only(cx) {
            return;
        }

        let selections = self.selections.all_adjusted(cx);
        let mut bracket_inserted = false;
        let mut edits = Vec::new();
        let mut linked_edits = HashMap::<_, Vec<_>>::default();
        let mut new_selections = Vec::with_capacity(selections.len());
        let mut new_autoclose_regions = Vec::new();
        let snapshot = self.buffer.read(cx).read(cx);

        for (selection, autoclose_region) in
            self.selections_with_autoclose_regions(selections, &snapshot)
        {
            if let Some(scope) = snapshot.language_scope_at(selection.head()) {
                // Determine if the inserted text matches the opening or closing
                // bracket of any of this language's bracket pairs.
                let mut bracket_pair = None;
                let mut is_bracket_pair_start = false;
                let mut is_bracket_pair_end = false;
                if !text.is_empty() {
                    // `text` can be empty when a user is using IME (e.g. Chinese Wubi Simplified)
                    //  and they are removing the character that triggered IME popup.
                    for (pair, enabled) in scope.brackets() {
                        if !pair.close && !pair.surround {
                            continue;
                        }

                        if enabled && pair.start.ends_with(text.as_ref()) {
                            bracket_pair = Some(pair.clone());
                            is_bracket_pair_start = true;
                            break;
                        }
                        if pair.end.as_str() == text.as_ref() {
                            bracket_pair = Some(pair.clone());
                            is_bracket_pair_end = true;
                            break;
                        }
                    }
                }

                if let Some(bracket_pair) = bracket_pair {
                    let snapshot_settings = snapshot.settings_at(selection.start, cx);
                    let autoclose = self.use_autoclose && snapshot_settings.use_autoclose;
                    let auto_surround =
                        self.use_auto_surround && snapshot_settings.use_auto_surround;
                    if selection.is_empty() {
                        if is_bracket_pair_start {
                            let prefix_len = bracket_pair.start.len() - text.len();

                            // If the inserted text is a suffix of an opening bracket and the
                            // selection is preceded by the rest of the opening bracket, then
                            // insert the closing bracket.
                            let following_text_allows_autoclose = snapshot
                                .chars_at(selection.start)
                                .next()
                                .map_or(true, |c| scope.should_autoclose_before(c));
                            let preceding_text_matches_prefix = prefix_len == 0
                                || (selection.start.column >= (prefix_len as u32)
                                    && snapshot.contains_str_at(
                                        Point::new(
                                            selection.start.row,
                                            selection.start.column - (prefix_len as u32),
                                        ),
                                        &bracket_pair.start[..prefix_len],
                                    ));

                            if autoclose
                                && bracket_pair.close
                                && following_text_allows_autoclose
                                && preceding_text_matches_prefix
                            {
                                let anchor = snapshot.anchor_before(selection.end);
                                new_selections.push((selection.map(|_| anchor), text.len()));
                                new_autoclose_regions.push((
                                    anchor,
                                    text.len(),
                                    selection.id,
                                    bracket_pair.clone(),
                                ));
                                edits.push((
                                    selection.range(),
                                    format!("{}{}", text, bracket_pair.end).into(),
                                ));
                                bracket_inserted = true;
                                continue;
                            }
                        }

                        if let Some(region) = autoclose_region {
                            // If the selection is followed by an auto-inserted closing bracket,
                            // then don't insert that closing bracket again; just move the selection
                            // past the closing bracket.
                            let should_skip = selection.end == region.range.end.to_point(&snapshot)
                                && text.as_ref() == region.pair.end.as_str();
                            if should_skip {
                                let anchor = snapshot.anchor_after(selection.end);
                                new_selections
                                    .push((selection.map(|_| anchor), region.pair.end.len()));
                                continue;
                            }
                        }

                        let always_treat_brackets_as_autoclosed = snapshot
                            .settings_at(selection.start, cx)
                            .always_treat_brackets_as_autoclosed;
                        if always_treat_brackets_as_autoclosed
                            && is_bracket_pair_end
                            && snapshot.contains_str_at(selection.end, text.as_ref())
                        {
                            // Otherwise, when `always_treat_brackets_as_autoclosed` is set to `true
                            // and the inserted text is a closing bracket and the selection is followed
                            // by the closing bracket then move the selection past the closing bracket.
                            let anchor = snapshot.anchor_after(selection.end);
                            new_selections.push((selection.map(|_| anchor), text.len()));
                            continue;
                        }
                    }
                    // If an opening bracket is 1 character long and is typed while
                    // text is selected, then surround that text with the bracket pair.
                    else if auto_surround
                        && bracket_pair.surround
                        && is_bracket_pair_start
                        && bracket_pair.start.chars().count() == 1
                    {
                        edits.push((selection.start..selection.start, text.clone()));
                        edits.push((
                            selection.end..selection.end,
                            bracket_pair.end.as_str().into(),
                        ));
                        bracket_inserted = true;
                        new_selections.push((
                            Selection {
                                id: selection.id,
                                start: snapshot.anchor_after(selection.start),
                                end: snapshot.anchor_before(selection.end),
                                reversed: selection.reversed,
                                goal: selection.goal,
                            },
                            0,
                        ));
                        continue;
                    }
                }
            }

            if self.auto_replace_emoji_shortcode
                && selection.is_empty()
                && text.as_ref().ends_with(':')
            {
                if let Some(possible_emoji_short_code) =
                    Self::find_possible_emoji_shortcode_at_position(&snapshot, selection.start)
                {
                    if !possible_emoji_short_code.is_empty() {
                        if let Some(emoji) = emojis::get_by_shortcode(&possible_emoji_short_code) {
                            let emoji_shortcode_start = Point::new(
                                selection.start.row,
                                selection.start.column - possible_emoji_short_code.len() as u32 - 1,
                            );

                            // Remove shortcode from buffer
                            edits.push((
                                emoji_shortcode_start..selection.start,
                                "".to_string().into(),
                            ));
                            new_selections.push((
                                Selection {
                                    id: selection.id,
                                    start: snapshot.anchor_after(emoji_shortcode_start),
                                    end: snapshot.anchor_before(selection.start),
                                    reversed: selection.reversed,
                                    goal: selection.goal,
                                },
                                0,
                            ));

                            // Insert emoji
                            let selection_start_anchor = snapshot.anchor_after(selection.start);
                            new_selections.push((selection.map(|_| selection_start_anchor), 0));
                            edits.push((selection.start..selection.end, emoji.to_string().into()));

                            continue;
                        }
                    }
                }
            }

            // If not handling any auto-close operation, then just replace the selected
            // text with the given input and move the selection to the end of the
            // newly inserted text.
            let anchor = snapshot.anchor_after(selection.end);
            if !self.linked_edit_ranges.is_empty() {
                let start_anchor = snapshot.anchor_before(selection.start);

                let is_word_char = text.chars().next().map_or(true, |char| {
                    let scope = snapshot.language_scope_at(start_anchor.to_offset(&snapshot));
                    let kind = char_kind(&scope, char);

                    kind == CharKind::Word
                });

                if is_word_char {
                    if let Some(ranges) = self
                        .linked_editing_ranges_for(start_anchor.text_anchor..anchor.text_anchor, cx)
                    {
                        for (buffer, edits) in ranges {
                            linked_edits
                                .entry(buffer.clone())
                                .or_default()
                                .extend(edits.into_iter().map(|range| (range, text.clone())));
                        }
                    }
                }
            }

            new_selections.push((selection.map(|_| anchor), 0));
            edits.push((selection.start..selection.end, text.clone()));
        }

        drop(snapshot);

        self.transact(cx, |this, cx| {
            this.buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, this.autoindent_mode.clone(), cx);
            });
            for (buffer, edits) in linked_edits {
                buffer.update(cx, |buffer, cx| {
                    let snapshot = buffer.snapshot();
                    let edits = edits
                        .into_iter()
                        .map(|(range, text)| {
                            use text::ToPoint as TP;
                            let end_point = TP::to_point(&range.end, &snapshot);
                            let start_point = TP::to_point(&range.start, &snapshot);
                            (start_point..end_point, text)
                        })
                        .sorted_by_key(|(range, _)| range.start)
                        .collect::<Vec<_>>();
                    buffer.edit(edits, None, cx);
                })
            }
            let new_anchor_selections = new_selections.iter().map(|e| &e.0);
            let new_selection_deltas = new_selections.iter().map(|e| e.1);
            let snapshot = this.buffer.read(cx).read(cx);
            let new_selections = resolve_multiple::<usize, _>(new_anchor_selections, &snapshot)
                .zip(new_selection_deltas)
                .map(|(selection, delta)| Selection {
                    id: selection.id,
                    start: selection.start + delta,
                    end: selection.end + delta,
                    reversed: selection.reversed,
                    goal: SelectionGoal::None,
                })
                .collect::<Vec<_>>();

            let mut i = 0;
            for (position, delta, selection_id, pair) in new_autoclose_regions {
                let position = position.to_offset(&snapshot) + delta;
                let start = snapshot.anchor_before(position);
                let end = snapshot.anchor_after(position);
                while let Some(existing_state) = this.autoclose_regions.get(i) {
                    match existing_state.range.start.cmp(&start, &snapshot) {
                        Ordering::Less => i += 1,
                        Ordering::Greater => break,
                        Ordering::Equal => match end.cmp(&existing_state.range.end, &snapshot) {
                            Ordering::Less => i += 1,
                            Ordering::Equal => break,
                            Ordering::Greater => break,
                        },
                    }
                }
                this.autoclose_regions.insert(
                    i,
                    AutocloseRegion {
                        selection_id,
                        range: start..end,
                        pair,
                    },
                );
            }

            drop(snapshot);
            let had_active_inline_completion = this.has_active_inline_completion(cx);
            this.change_selections_inner(Some(Autoscroll::fit()), false, cx, |s| {
                s.select(new_selections)
            });

            if !bracket_inserted && EditorSettings::get_global(cx).use_on_type_format {
                if let Some(on_type_format_task) =
                    this.trigger_on_type_formatting(text.to_string(), cx)
                {
                    on_type_format_task.detach_and_log_err(cx);
                }
            }

            let editor_settings = EditorSettings::get_global(cx);
            if bracket_inserted
                && (editor_settings.auto_signature_help
                    || editor_settings.show_signature_help_after_edits)
            {
                this.show_signature_help(&ShowSignatureHelp, cx);
            }

            let trigger_in_words = !had_active_inline_completion;
            this.trigger_completion_on_input(&text, trigger_in_words, cx);
            linked_editing_ranges::refresh_linked_ranges(this, cx);
            this.refresh_inline_completion(true, cx);
        });
    }

    fn find_possible_emoji_shortcode_at_position(
        snapshot: &MultiBufferSnapshot,
        position: Point,
    ) -> Option<String> {
        let mut chars = Vec::new();
        let mut found_colon = false;
        for char in snapshot.reversed_chars_at(position).take(100) {
            // Found a possible emoji shortcode in the middle of the buffer
            if found_colon {
                if char.is_whitespace() {
                    chars.reverse();
                    return Some(chars.iter().collect());
                }
                // If the previous character is not a whitespace, we are in the middle of a word
                // and we only want to complete the shortcode if the word is made up of other emojis
                let mut containing_word = String::new();
                for ch in snapshot
                    .reversed_chars_at(position)
                    .skip(chars.len() + 1)
                    .take(100)
                {
                    if ch.is_whitespace() {
                        break;
                    }
                    containing_word.push(ch);
                }
                let containing_word = containing_word.chars().rev().collect::<String>();
                if util::word_consists_of_emojis(containing_word.as_str()) {
                    chars.reverse();
                    return Some(chars.iter().collect());
                }
            }

            if char.is_whitespace() || !char.is_ascii() {
                return None;
            }
            if char == ':' {
                found_colon = true;
            } else {
                chars.push(char);
            }
        }
        // Found a possible emoji shortcode at the beginning of the buffer
        chars.reverse();
        Some(chars.iter().collect())
    }

    pub fn newline(&mut self, _: &Newline, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            let (edits, selection_fixup_info): (Vec<_>, Vec<_>) = {
                let selections = this.selections.all::<usize>(cx);
                let multi_buffer = this.buffer.read(cx);
                let buffer = multi_buffer.snapshot(cx);
                selections
                    .iter()
                    .map(|selection| {
                        let start_point = selection.start.to_point(&buffer);
                        let mut indent =
                            buffer.indent_size_for_line(MultiBufferRow(start_point.row));
                        indent.len = cmp::min(indent.len, start_point.column);
                        let start = selection.start;
                        let end = selection.end;
                        let selection_is_empty = start == end;
                        let language_scope = buffer.language_scope_at(start);
                        let (comment_delimiter, insert_extra_newline) = if let Some(language) =
                            &language_scope
                        {
                            let leading_whitespace_len = buffer
                                .reversed_chars_at(start)
                                .take_while(|c| c.is_whitespace() && *c != '\n')
                                .map(|c| c.len_utf8())
                                .sum::<usize>();

                            let trailing_whitespace_len = buffer
                                .chars_at(end)
                                .take_while(|c| c.is_whitespace() && *c != '\n')
                                .map(|c| c.len_utf8())
                                .sum::<usize>();

                            let insert_extra_newline =
                                language.brackets().any(|(pair, enabled)| {
                                    let pair_start = pair.start.trim_end();
                                    let pair_end = pair.end.trim_start();

                                    enabled
                                        && pair.newline
                                        && buffer.contains_str_at(
                                            end + trailing_whitespace_len,
                                            pair_end,
                                        )
                                        && buffer.contains_str_at(
                                            (start - leading_whitespace_len)
                                                .saturating_sub(pair_start.len()),
                                            pair_start,
                                        )
                                });

                            // Comment extension on newline is allowed only for cursor selections
                            let comment_delimiter = maybe!({
                                if !selection_is_empty {
                                    return None;
                                }

                                if !multi_buffer.settings_at(0, cx).extend_comment_on_newline {
                                    return None;
                                }

                                let delimiters = language.line_comment_prefixes();
                                let max_len_of_delimiter =
                                    delimiters.iter().map(|delimiter| delimiter.len()).max()?;
                                let (snapshot, range) =
                                    buffer.buffer_line_for_row(MultiBufferRow(start_point.row))?;

                                let mut index_of_first_non_whitespace = 0;
                                let comment_candidate = snapshot
                                    .chars_for_range(range)
                                    .skip_while(|c| {
                                        let should_skip = c.is_whitespace();
                                        if should_skip {
                                            index_of_first_non_whitespace += 1;
                                        }
                                        should_skip
                                    })
                                    .take(max_len_of_delimiter)
                                    .collect::<String>();
                                let comment_prefix = delimiters.iter().find(|comment_prefix| {
                                    comment_candidate.starts_with(comment_prefix.as_ref())
                                })?;
                                let cursor_is_placed_after_comment_marker =
                                    index_of_first_non_whitespace + comment_prefix.len()
                                        <= start_point.column as usize;
                                if cursor_is_placed_after_comment_marker {
                                    Some(comment_prefix.clone())
                                } else {
                                    None
                                }
                            });
                            (comment_delimiter, insert_extra_newline)
                        } else {
                            (None, false)
                        };

                        let capacity_for_delimiter = comment_delimiter
                            .as_deref()
                            .map(str::len)
                            .unwrap_or_default();
                        let mut new_text =
                            String::with_capacity(1 + capacity_for_delimiter + indent.len as usize);
                        new_text.push_str("\n");
                        new_text.extend(indent.chars());
                        if let Some(delimiter) = &comment_delimiter {
                            new_text.push_str(&delimiter);
                        }
                        if insert_extra_newline {
                            new_text = new_text.repeat(2);
                        }

                        let anchor = buffer.anchor_after(end);
                        let new_selection = selection.map(|_| anchor);
                        (
                            (start..end, new_text),
                            (insert_extra_newline, new_selection),
                        )
                    })
                    .unzip()
            };

            this.edit_with_autoindent(edits, cx);
            let buffer = this.buffer.read(cx).snapshot(cx);
            let new_selections = selection_fixup_info
                .into_iter()
                .map(|(extra_newline_inserted, new_selection)| {
                    let mut cursor = new_selection.end.to_point(&buffer);
                    if extra_newline_inserted {
                        cursor.row -= 1;
                        cursor.column = buffer.line_len(MultiBufferRow(cursor.row));
                    }
                    new_selection.map(|_| cursor)
                })
                .collect();

            this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(new_selections));
            this.refresh_inline_completion(true, cx);
        });
    }

    pub fn newline_above(&mut self, _: &NewlineAbove, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx);
        let snapshot = buffer.snapshot(cx);

        let mut edits = Vec::new();
        let mut rows = Vec::new();

        for (rows_inserted, selection) in self.selections.all_adjusted(cx).into_iter().enumerate() {
            let cursor = selection.head();
            let row = cursor.row;

            let start_of_line = snapshot.clip_point(Point::new(row, 0), Bias::Left);

            let newline = "\n".to_string();
            edits.push((start_of_line..start_of_line, newline));

            rows.push(row + rows_inserted as u32);
        }

        self.transact(cx, |editor, cx| {
            editor.edit(edits, cx);

            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let mut index = 0;
                s.move_cursors_with(|map, _, _| {
                    let row = rows[index];
                    index += 1;

                    let point = Point::new(row, 0);
                    let boundary = map.next_line_boundary(point).1;
                    let clipped = map.clip_point(boundary, Bias::Left);

                    (clipped, SelectionGoal::None)
                });
            });

            let mut indent_edits = Vec::new();
            let multibuffer_snapshot = editor.buffer.read(cx).snapshot(cx);
            for row in rows {
                let indents = multibuffer_snapshot.suggested_indents(row..row + 1, cx);
                for (row, indent) in indents {
                    if indent.len == 0 {
                        continue;
                    }

                    let text = match indent.kind {
                        IndentKind::Space => " ".repeat(indent.len as usize),
                        IndentKind::Tab => "\t".repeat(indent.len as usize),
                    };
                    let point = Point::new(row.0, 0);
                    indent_edits.push((point..point, text));
                }
            }
            editor.edit(indent_edits, cx);
        });
    }

    pub fn newline_below(&mut self, _: &NewlineBelow, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx);
        let snapshot = buffer.snapshot(cx);

        let mut edits = Vec::new();
        let mut rows = Vec::new();
        let mut rows_inserted = 0;

        for selection in self.selections.all_adjusted(cx) {
            let cursor = selection.head();
            let row = cursor.row;

            let point = Point::new(row + 1, 0);
            let start_of_line = snapshot.clip_point(point, Bias::Left);

            let newline = "\n".to_string();
            edits.push((start_of_line..start_of_line, newline));

            rows_inserted += 1;
            rows.push(row + rows_inserted);
        }

        self.transact(cx, |editor, cx| {
            editor.edit(edits, cx);

            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let mut index = 0;
                s.move_cursors_with(|map, _, _| {
                    let row = rows[index];
                    index += 1;

                    let point = Point::new(row, 0);
                    let boundary = map.next_line_boundary(point).1;
                    let clipped = map.clip_point(boundary, Bias::Left);

                    (clipped, SelectionGoal::None)
                });
            });

            let mut indent_edits = Vec::new();
            let multibuffer_snapshot = editor.buffer.read(cx).snapshot(cx);
            for row in rows {
                let indents = multibuffer_snapshot.suggested_indents(row..row + 1, cx);
                for (row, indent) in indents {
                    if indent.len == 0 {
                        continue;
                    }

                    let text = match indent.kind {
                        IndentKind::Space => " ".repeat(indent.len as usize),
                        IndentKind::Tab => "\t".repeat(indent.len as usize),
                    };
                    let point = Point::new(row.0, 0);
                    indent_edits.push((point..point, text));
                }
            }
            editor.edit(indent_edits, cx);
        });
    }

    pub fn insert(&mut self, text: &str, cx: &mut ViewContext<Self>) {
        let autoindent = text.is_empty().not().then(|| AutoindentMode::Block {
            original_indent_columns: Vec::new(),
        });
        self.insert_with_autoindent_mode(text, autoindent, cx);
    }

    fn insert_with_autoindent_mode(
        &mut self,
        text: &str,
        autoindent_mode: Option<AutoindentMode>,
        cx: &mut ViewContext<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }

        let text: Arc<str> = text.into();
        self.transact(cx, |this, cx| {
            let old_selections = this.selections.all_adjusted(cx);
            let selection_anchors = this.buffer.update(cx, |buffer, cx| {
                let anchors = {
                    let snapshot = buffer.read(cx);
                    old_selections
                        .iter()
                        .map(|s| {
                            let anchor = snapshot.anchor_after(s.head());
                            s.map(|_| anchor)
                        })
                        .collect::<Vec<_>>()
                };
                buffer.edit(
                    old_selections
                        .iter()
                        .map(|s| (s.start..s.end, text.clone())),
                    autoindent_mode,
                    cx,
                );
                anchors
            });

            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select_anchors(selection_anchors);
            })
        });
    }

    fn trigger_completion_on_input(
        &mut self,
        text: &str,
        trigger_in_words: bool,
        cx: &mut ViewContext<Self>,
    ) {
        if self.is_completion_trigger(text, trigger_in_words, cx) {
            self.show_completions(
                &ShowCompletions {
                    trigger: Some(text.to_owned()).filter(|x| !x.is_empty()),
                },
                cx,
            );
        } else {
            self.hide_context_menu(cx);
        }
    }

    fn is_completion_trigger(
        &self,
        text: &str,
        trigger_in_words: bool,
        cx: &mut ViewContext<Self>,
    ) -> bool {
        let position = self.selections.newest_anchor().head();
        let multibuffer = self.buffer.read(cx);
        let Some(buffer) = position
            .buffer_id
            .and_then(|buffer_id| multibuffer.buffer(buffer_id).clone())
        else {
            return false;
        };

        if let Some(completion_provider) = &self.completion_provider {
            completion_provider.is_completion_trigger(
                &buffer,
                position.text_anchor,
                text,
                trigger_in_words,
                cx,
            )
        } else {
            false
        }
    }

    /// If any empty selections is touching the start of its innermost containing autoclose
    /// region, expand it to select the brackets.
    fn select_autoclose_pair(&mut self, cx: &mut ViewContext<Self>) {
        let selections = self.selections.all::<usize>(cx);
        let buffer = self.buffer.read(cx).read(cx);
        let new_selections = self
            .selections_with_autoclose_regions(selections, &buffer)
            .map(|(mut selection, region)| {
                if !selection.is_empty() {
                    return selection;
                }

                if let Some(region) = region {
                    let mut range = region.range.to_offset(&buffer);
                    if selection.start == range.start && range.start >= region.pair.start.len() {
                        range.start -= region.pair.start.len();
                        if buffer.contains_str_at(range.start, &region.pair.start)
                            && buffer.contains_str_at(range.end, &region.pair.end)
                        {
                            range.end += region.pair.end.len();
                            selection.start = range.start;
                            selection.end = range.end;

                            return selection;
                        }
                    }
                }

                let always_treat_brackets_as_autoclosed = buffer
                    .settings_at(selection.start, cx)
                    .always_treat_brackets_as_autoclosed;

                if !always_treat_brackets_as_autoclosed {
                    return selection;
                }

                if let Some(scope) = buffer.language_scope_at(selection.start) {
                    for (pair, enabled) in scope.brackets() {
                        if !enabled || !pair.close {
                            continue;
                        }

                        if buffer.contains_str_at(selection.start, &pair.end) {
                            let pair_start_len = pair.start.len();
                            if buffer.contains_str_at(selection.start - pair_start_len, &pair.start)
                            {
                                selection.start -= pair_start_len;
                                selection.end += pair.end.len();

                                return selection;
                            }
                        }
                    }
                }

                selection
            })
            .collect();

        drop(buffer);
        self.change_selections(None, cx, |selections| selections.select(new_selections));
    }

    /// Iterate the given selections, and for each one, find the smallest surrounding
    /// autoclose region. This uses the ordering of the selections and the autoclose
    /// regions to avoid repeated comparisons.
    fn selections_with_autoclose_regions<'a, D: ToOffset + Clone>(
        &'a self,
        selections: impl IntoIterator<Item = Selection<D>>,
        buffer: &'a MultiBufferSnapshot,
    ) -> impl Iterator<Item = (Selection<D>, Option<&'a AutocloseRegion>)> {
        let mut i = 0;
        let mut regions = self.autoclose_regions.as_slice();
        selections.into_iter().map(move |selection| {
            let range = selection.start.to_offset(buffer)..selection.end.to_offset(buffer);

            let mut enclosing = None;
            while let Some(pair_state) = regions.get(i) {
                if pair_state.range.end.to_offset(buffer) < range.start {
                    regions = &regions[i + 1..];
                    i = 0;
                } else if pair_state.range.start.to_offset(buffer) > range.end {
                    break;
                } else {
                    if pair_state.selection_id == selection.id {
                        enclosing = Some(pair_state);
                    }
                    i += 1;
                }
            }

            (selection.clone(), enclosing)
        })
    }

    /// Remove any autoclose regions that no longer contain their selection.
    fn invalidate_autoclose_regions(
        &mut self,
        mut selections: &[Selection<Anchor>],
        buffer: &MultiBufferSnapshot,
    ) {
        self.autoclose_regions.retain(|state| {
            let mut i = 0;
            while let Some(selection) = selections.get(i) {
                if selection.end.cmp(&state.range.start, buffer).is_lt() {
                    selections = &selections[1..];
                    continue;
                }
                if selection.start.cmp(&state.range.end, buffer).is_gt() {
                    break;
                }
                if selection.id == state.selection_id {
                    return true;
                } else {
                    i += 1;
                }
            }
            false
        });
    }

    fn completion_query(buffer: &MultiBufferSnapshot, position: impl ToOffset) -> Option<String> {
        let offset = position.to_offset(buffer);
        let (word_range, kind) = buffer.surrounding_word(offset);
        if offset > word_range.start && kind == Some(CharKind::Word) {
            Some(
                buffer
                    .text_for_range(word_range.start..offset)
                    .collect::<String>(),
            )
        } else {
            None
        }
    }

    pub fn toggle_inlay_hints(&mut self, _: &ToggleInlayHints, cx: &mut ViewContext<Self>) {
        self.refresh_inlay_hints(
            InlayHintRefreshReason::Toggle(!self.inlay_hint_cache.enabled),
            cx,
        );
    }

    pub fn inlay_hints_enabled(&self) -> bool {
        self.inlay_hint_cache.enabled
    }

    fn refresh_inlay_hints(&mut self, reason: InlayHintRefreshReason, cx: &mut ViewContext<Self>) {
        if self.project.is_none() || self.mode != EditorMode::Full {
            return;
        }

        let reason_description = reason.description();
        let ignore_debounce = matches!(
            reason,
            InlayHintRefreshReason::SettingsChange(_)
                | InlayHintRefreshReason::Toggle(_)
                | InlayHintRefreshReason::ExcerptsRemoved(_)
        );
        let (invalidate_cache, required_languages) = match reason {
            InlayHintRefreshReason::Toggle(enabled) => {
                self.inlay_hint_cache.enabled = enabled;
                if enabled {
                    (InvalidationStrategy::RefreshRequested, None)
                } else {
                    self.inlay_hint_cache.clear();
                    self.splice_inlays(
                        self.visible_inlay_hints(cx)
                            .iter()
                            .map(|inlay| inlay.id)
                            .collect(),
                        Vec::new(),
                        cx,
                    );
                    return;
                }
            }
            InlayHintRefreshReason::SettingsChange(new_settings) => {
                match self.inlay_hint_cache.update_settings(
                    &self.buffer,
                    new_settings,
                    self.visible_inlay_hints(cx),
                    cx,
                ) {
                    ControlFlow::Break(Some(InlaySplice {
                        to_remove,
                        to_insert,
                    })) => {
                        self.splice_inlays(to_remove, to_insert, cx);
                        return;
                    }
                    ControlFlow::Break(None) => return,
                    ControlFlow::Continue(()) => (InvalidationStrategy::RefreshRequested, None),
                }
            }
            InlayHintRefreshReason::ExcerptsRemoved(excerpts_removed) => {
                if let Some(InlaySplice {
                    to_remove,
                    to_insert,
                }) = self.inlay_hint_cache.remove_excerpts(excerpts_removed)
                {
                    self.splice_inlays(to_remove, to_insert, cx);
                }
                return;
            }
            InlayHintRefreshReason::NewLinesShown => (InvalidationStrategy::None, None),
            InlayHintRefreshReason::BufferEdited(buffer_languages) => {
                (InvalidationStrategy::BufferEdited, Some(buffer_languages))
            }
            InlayHintRefreshReason::RefreshRequested => {
                (InvalidationStrategy::RefreshRequested, None)
            }
        };

        if let Some(InlaySplice {
            to_remove,
            to_insert,
        }) = self.inlay_hint_cache.spawn_hint_refresh(
            reason_description,
            self.excerpts_for_inlay_hints_query(required_languages.as_ref(), cx),
            invalidate_cache,
            ignore_debounce,
            cx,
        ) {
            self.splice_inlays(to_remove, to_insert, cx);
        }
    }

    fn visible_inlay_hints(&self, cx: &ViewContext<'_, Editor>) -> Vec<Inlay> {
        self.display_map
            .read(cx)
            .current_inlays()
            .filter(move |inlay| matches!(inlay.id, InlayId::Hint(_)))
            .cloned()
            .collect()
    }

    pub fn excerpts_for_inlay_hints_query(
        &self,
        restrict_to_languages: Option<&HashSet<Arc<Language>>>,
        cx: &mut ViewContext<Editor>,
    ) -> HashMap<ExcerptId, (Model<Buffer>, clock::Global, Range<usize>)> {
        let Some(project) = self.project.as_ref() else {
            return HashMap::default();
        };
        let project = project.read(cx);
        let multi_buffer = self.buffer().read(cx);
        let multi_buffer_snapshot = multi_buffer.snapshot(cx);
        let multi_buffer_visible_start = self
            .scroll_manager
            .anchor()
            .anchor
            .to_point(&multi_buffer_snapshot);
        let multi_buffer_visible_end = multi_buffer_snapshot.clip_point(
            multi_buffer_visible_start
                + Point::new(self.visible_line_count().unwrap_or(0.).ceil() as u32, 0),
            Bias::Left,
        );
        let multi_buffer_visible_range = multi_buffer_visible_start..multi_buffer_visible_end;
        multi_buffer
            .range_to_buffer_ranges(multi_buffer_visible_range, cx)
            .into_iter()
            .filter(|(_, excerpt_visible_range, _)| !excerpt_visible_range.is_empty())
            .filter_map(|(buffer_handle, excerpt_visible_range, excerpt_id)| {
                let buffer = buffer_handle.read(cx);
                let buffer_file = project::File::from_dyn(buffer.file())?;
                let buffer_worktree = project.worktree_for_id(buffer_file.worktree_id(cx), cx)?;
                let worktree_entry = buffer_worktree
                    .read(cx)
                    .entry_for_id(buffer_file.project_entry_id(cx)?)?;
                if worktree_entry.is_ignored {
                    return None;
                }

                let language = buffer.language()?;
                if let Some(restrict_to_languages) = restrict_to_languages {
                    if !restrict_to_languages.contains(language) {
                        return None;
                    }
                }
                Some((
                    excerpt_id,
                    (
                        buffer_handle,
                        buffer.version().clone(),
                        excerpt_visible_range,
                    ),
                ))
            })
            .collect()
    }

    pub fn text_layout_details(&self, cx: &WindowContext) -> TextLayoutDetails {
        TextLayoutDetails {
            text_system: cx.text_system().clone(),
            editor_style: self.style.clone().unwrap(),
            rem_size: cx.rem_size(),
            scroll_anchor: self.scroll_manager.anchor(),
            visible_rows: self.visible_line_count(),
            vertical_scroll_margin: self.scroll_manager.vertical_scroll_margin,
        }
    }

    fn splice_inlays(
        &self,
        to_remove: Vec<InlayId>,
        to_insert: Vec<Inlay>,
        cx: &mut ViewContext<Self>,
    ) {
        self.display_map.update(cx, |display_map, cx| {
            display_map.splice_inlays(to_remove, to_insert, cx);
        });
        cx.notify();
    }

    fn trigger_on_type_formatting(
        &self,
        input: String,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        if input.len() != 1 {
            return None;
        }

        let project = self.project.as_ref()?;
        let position = self.selections.newest_anchor().head();
        let (buffer, buffer_position) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(position, cx)?;

        // OnTypeFormatting returns a list of edits, no need to pass them between Zed instances,
        // hence we do LSP request & edit on host side only — add formats to host's history.
        let push_to_lsp_host_history = true;
        // If this is not the host, append its history with new edits.
        let push_to_client_history = project.read(cx).is_remote();

        let on_type_formatting = project.update(cx, |project, cx| {
            project.on_type_format(
                buffer.clone(),
                buffer_position,
                input,
                push_to_lsp_host_history,
                cx,
            )
        });
        Some(cx.spawn(|editor, mut cx| async move {
            if let Some(transaction) = on_type_formatting.await? {
                if push_to_client_history {
                    buffer
                        .update(&mut cx, |buffer, _| {
                            buffer.push_transaction(transaction, Instant::now());
                        })
                        .ok();
                }
                editor.update(&mut cx, |editor, cx| {
                    editor.refresh_document_highlights(cx);
                })?;
            }
            Ok(())
        }))
    }

    pub fn show_completions(&mut self, options: &ShowCompletions, cx: &mut ViewContext<Self>) {
        if self.pending_rename.is_some() {
            return;
        }

        let Some(provider) = self.completion_provider.as_ref() else {
            return;
        };

        let position = self.selections.newest_anchor().head();
        let (buffer, buffer_position) =
            if let Some(output) = self.buffer.read(cx).text_anchor_for_position(position, cx) {
                output
            } else {
                return;
            };

        let query = Self::completion_query(&self.buffer.read(cx).read(cx), position);
        let is_followup_invoke = {
            let context_menu_state = self.context_menu.read();
            matches!(
                context_menu_state.deref(),
                Some(ContextMenu::Completions(_))
            )
        };
        let trigger_kind = match (&options.trigger, is_followup_invoke) {
            (_, true) => CompletionTriggerKind::TRIGGER_FOR_INCOMPLETE_COMPLETIONS,
            (Some(trigger), _) if buffer.read(cx).completion_triggers().contains(&trigger) => {
                CompletionTriggerKind::TRIGGER_CHARACTER
            }

            _ => CompletionTriggerKind::INVOKED,
        };
        let completion_context = CompletionContext {
            trigger_character: options.trigger.as_ref().and_then(|trigger| {
                if trigger_kind == CompletionTriggerKind::TRIGGER_CHARACTER {
                    Some(String::from(trigger))
                } else {
                    None
                }
            }),
            trigger_kind,
        };
        let completions = provider.completions(&buffer, buffer_position, completion_context, cx);

        let id = post_inc(&mut self.next_completion_id);
        let task = cx.spawn(|this, mut cx| {
            async move {
                this.update(&mut cx, |this, _| {
                    this.completion_tasks.retain(|(task_id, _)| *task_id >= id);
                })?;
                let completions = completions.await.log_err();
                let menu = if let Some(completions) = completions {
                    let mut menu = CompletionsMenu {
                        id,
                        initial_position: position,
                        match_candidates: completions
                            .iter()
                            .enumerate()
                            .map(|(id, completion)| {
                                StringMatchCandidate::new(
                                    id,
                                    completion.label.text[completion.label.filter_range.clone()]
                                        .into(),
                                )
                            })
                            .collect(),
                        buffer: buffer.clone(),
                        completions: Arc::new(RwLock::new(completions.into())),
                        matches: Vec::new().into(),
                        selected_item: 0,
                        scroll_handle: UniformListScrollHandle::new(),
                        selected_completion_documentation_resolve_debounce: Arc::new(Mutex::new(
                            DebouncedDelay::new(),
                        )),
                    };
                    menu.filter(query.as_deref(), cx.background_executor().clone())
                        .await;

                    if menu.matches.is_empty() {
                        None
                    } else {
                        this.update(&mut cx, |editor, cx| {
                            let completions = menu.completions.clone();
                            let matches = menu.matches.clone();

                            let delay_ms = EditorSettings::get_global(cx)
                                .completion_documentation_secondary_query_debounce;
                            let delay = Duration::from_millis(delay_ms);
                            editor
                                .completion_documentation_pre_resolve_debounce
                                .fire_new(delay, cx, |editor, cx| {
                                    CompletionsMenu::pre_resolve_completion_documentation(
                                        buffer,
                                        completions,
                                        matches,
                                        editor,
                                        cx,
                                    )
                                });
                        })
                        .ok();
                        Some(menu)
                    }
                } else {
                    None
                };

                this.update(&mut cx, |this, cx| {
                    let mut context_menu = this.context_menu.write();
                    match context_menu.as_ref() {
                        None => {}

                        Some(ContextMenu::Completions(prev_menu)) => {
                            if prev_menu.id > id {
                                return;
                            }
                        }

                        _ => return,
                    }

                    if this.focus_handle.is_focused(cx) && menu.is_some() {
                        let menu = menu.unwrap();
                        *context_menu = Some(ContextMenu::Completions(menu));
                        drop(context_menu);
                        this.discard_inline_completion(false, cx);
                        cx.notify();
                    } else if this.completion_tasks.len() <= 1 {
                        // If there are no more completion tasks and the last menu was
                        // empty, we should hide it. If it was already hidden, we should
                        // also show the copilot completion when available.
                        drop(context_menu);
                        if this.hide_context_menu(cx).is_none() {
                            this.update_visible_inline_completion(cx);
                        }
                    }
                })?;

                Ok::<_, anyhow::Error>(())
            }
            .log_err()
        });

        self.completion_tasks.push((id, task));
    }

    pub fn confirm_completion(
        &mut self,
        action: &ConfirmCompletion,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        use language::ToOffset as _;

        let completions_menu = if let ContextMenu::Completions(menu) = self.hide_context_menu(cx)? {
            menu
        } else {
            return None;
        };

        let mat = completions_menu
            .matches
            .get(action.item_ix.unwrap_or(completions_menu.selected_item))?;
        let buffer_handle = completions_menu.buffer;
        let completions = completions_menu.completions.read();
        let completion = completions.get(mat.candidate_id)?;
        cx.stop_propagation();

        let snippet;
        let text;

        if completion.is_snippet() {
            snippet = Some(Snippet::parse(&completion.new_text).log_err()?);
            text = snippet.as_ref().unwrap().text.clone();
        } else {
            snippet = None;
            text = completion.new_text.clone();
        };
        let selections = self.selections.all::<usize>(cx);
        let buffer = buffer_handle.read(cx);
        let old_range = completion.old_range.to_offset(buffer);
        let old_text = buffer.text_for_range(old_range.clone()).collect::<String>();

        let newest_selection = self.selections.newest_anchor();
        if newest_selection.start.buffer_id != Some(buffer_handle.read(cx).remote_id()) {
            return None;
        }

        let lookbehind = newest_selection
            .start
            .text_anchor
            .to_offset(buffer)
            .saturating_sub(old_range.start);
        let lookahead = old_range
            .end
            .saturating_sub(newest_selection.end.text_anchor.to_offset(buffer));
        let mut common_prefix_len = old_text
            .bytes()
            .zip(text.bytes())
            .take_while(|(a, b)| a == b)
            .count();

        let snapshot = self.buffer.read(cx).snapshot(cx);
        let mut range_to_replace: Option<Range<isize>> = None;
        let mut ranges = Vec::new();
        let mut linked_edits = HashMap::<_, Vec<_>>::default();
        for selection in &selections {
            if snapshot.contains_str_at(selection.start.saturating_sub(lookbehind), &old_text) {
                let start = selection.start.saturating_sub(lookbehind);
                let end = selection.end + lookahead;
                if selection.id == newest_selection.id {
                    range_to_replace = Some(
                        ((start + common_prefix_len) as isize - selection.start as isize)
                            ..(end as isize - selection.start as isize),
                    );
                }
                ranges.push(start + common_prefix_len..end);
            } else {
                common_prefix_len = 0;
                ranges.clear();
                ranges.extend(selections.iter().map(|s| {
                    if s.id == newest_selection.id {
                        range_to_replace = Some(
                            old_range.start.to_offset_utf16(&snapshot).0 as isize
                                - selection.start as isize
                                ..old_range.end.to_offset_utf16(&snapshot).0 as isize
                                    - selection.start as isize,
                        );
                        old_range.clone()
                    } else {
                        s.start..s.end
                    }
                }));
                break;
            }
            if !self.linked_edit_ranges.is_empty() {
                let start_anchor = snapshot.anchor_before(selection.head());
                let end_anchor = snapshot.anchor_after(selection.tail());
                if let Some(ranges) = self
                    .linked_editing_ranges_for(start_anchor.text_anchor..end_anchor.text_anchor, cx)
                {
                    for (buffer, edits) in ranges {
                        linked_edits.entry(buffer.clone()).or_default().extend(
                            edits
                                .into_iter()
                                .map(|range| (range, text[common_prefix_len..].to_owned())),
                        );
                    }
                }
            }
        }
        let text = &text[common_prefix_len..];

        cx.emit(EditorEvent::InputHandled {
            utf16_range_to_replace: range_to_replace,
            text: text.into(),
        });

        self.transact(cx, |this, cx| {
            if let Some(mut snippet) = snippet {
                snippet.text = text.to_string();
                for tabstop in snippet.tabstops.iter_mut().flatten() {
                    tabstop.start -= common_prefix_len as isize;
                    tabstop.end -= common_prefix_len as isize;
                }

                this.insert_snippet(&ranges, snippet, cx).log_err();
            } else {
                this.buffer.update(cx, |buffer, cx| {
                    buffer.edit(
                        ranges.iter().map(|range| (range.clone(), text)),
                        this.autoindent_mode.clone(),
                        cx,
                    );
                });
            }
            for (buffer, edits) in linked_edits {
                buffer.update(cx, |buffer, cx| {
                    let snapshot = buffer.snapshot();
                    let edits = edits
                        .into_iter()
                        .map(|(range, text)| {
                            use text::ToPoint as TP;
                            let end_point = TP::to_point(&range.end, &snapshot);
                            let start_point = TP::to_point(&range.start, &snapshot);
                            (start_point..end_point, text)
                        })
                        .sorted_by_key(|(range, _)| range.start)
                        .collect::<Vec<_>>();
                    buffer.edit(edits, None, cx);
                })
            }

            this.refresh_inline_completion(true, cx);
        });

        if let Some(confirm) = completion.confirm.as_ref() {
            (confirm)(cx);
        }

        if completion.show_new_completions_on_confirm {
            self.show_completions(&ShowCompletions { trigger: None }, cx);
        }

        let provider = self.completion_provider.as_ref()?;
        let apply_edits = provider.apply_additional_edits_for_completion(
            buffer_handle,
            completion.clone(),
            true,
            cx,
        );

        let editor_settings = EditorSettings::get_global(cx);
        if editor_settings.show_signature_help_after_edits || editor_settings.auto_signature_help {
            // After the code completion is finished, users often want to know what signatures are needed.
            // so we should automatically call signature_help
            self.show_signature_help(&ShowSignatureHelp, cx);
        }

        Some(cx.foreground_executor().spawn(async move {
            apply_edits.await?;
            Ok(())
        }))
    }

    pub fn toggle_code_actions(&mut self, action: &ToggleCodeActions, cx: &mut ViewContext<Self>) {
        let mut context_menu = self.context_menu.write();
        if let Some(ContextMenu::CodeActions(code_actions)) = context_menu.as_ref() {
            if code_actions.deployed_from_indicator == action.deployed_from_indicator {
                // Toggle if we're selecting the same one
                *context_menu = None;
                cx.notify();
                return;
            } else {
                // Otherwise, clear it and start a new one
                *context_menu = None;
                cx.notify();
            }
        }
        drop(context_menu);
        let snapshot = self.snapshot(cx);
        let deployed_from_indicator = action.deployed_from_indicator;
        let mut task = self.code_actions_task.take();
        let action = action.clone();
        cx.spawn(|editor, mut cx| async move {
            while let Some(prev_task) = task {
                prev_task.await;
                task = editor.update(&mut cx, |this, _| this.code_actions_task.take())?;
            }

            let spawned_test_task = editor.update(&mut cx, |editor, cx| {
                if editor.focus_handle.is_focused(cx) {
                    let multibuffer_point = action
                        .deployed_from_indicator
                        .map(|row| DisplayPoint::new(row, 0).to_point(&snapshot))
                        .unwrap_or_else(|| editor.selections.newest::<Point>(cx).head());
                    let (buffer, buffer_row) = snapshot
                        .buffer_snapshot
                        .buffer_line_for_row(MultiBufferRow(multibuffer_point.row))
                        .and_then(|(buffer_snapshot, range)| {
                            editor
                                .buffer
                                .read(cx)
                                .buffer(buffer_snapshot.remote_id())
                                .map(|buffer| (buffer, range.start.row))
                        })?;
                    let (_, code_actions) = editor
                        .available_code_actions
                        .clone()
                        .and_then(|(location, code_actions)| {
                            let snapshot = location.buffer.read(cx).snapshot();
                            let point_range = location.range.to_point(&snapshot);
                            let point_range = point_range.start.row..=point_range.end.row;
                            if point_range.contains(&buffer_row) {
                                Some((location, code_actions))
                            } else {
                                None
                            }
                        })
                        .unzip();
                    let buffer_id = buffer.read(cx).remote_id();
                    let tasks = editor
                        .tasks
                        .get(&(buffer_id, buffer_row))
                        .map(|t| Arc::new(t.to_owned()));
                    if tasks.is_none() && code_actions.is_none() {
                        return None;
                    }

                    editor.completion_tasks.clear();
                    editor.discard_inline_completion(false, cx);
                    let task_context =
                        tasks
                            .as_ref()
                            .zip(editor.project.clone())
                            .map(|(tasks, project)| {
                                let position = Point::new(buffer_row, tasks.column);
                                let range_start = buffer.read(cx).anchor_at(position, Bias::Right);
                                let location = Location {
                                    buffer: buffer.clone(),
                                    range: range_start..range_start,
                                };
                                // Fill in the environmental variables from the tree-sitter captures
                                let mut captured_task_variables = TaskVariables::default();
                                for (capture_name, value) in tasks.extra_variables.clone() {
                                    captured_task_variables.insert(
                                        task::VariableName::Custom(capture_name.into()),
                                        value.clone(),
                                    );
                                }
                                project.update(cx, |project, cx| {
                                    project.task_context_for_location(
                                        captured_task_variables,
                                        location,
                                        cx,
                                    )
                                })
                            });

                    Some(cx.spawn(|editor, mut cx| async move {
                        let task_context = match task_context {
                            Some(task_context) => task_context.await,
                            None => None,
                        };
                        let resolved_tasks =
                            tasks.zip(task_context).map(|(tasks, task_context)| {
                                Arc::new(ResolvedTasks {
                                    templates: tasks
                                        .templates
                                        .iter()
                                        .filter_map(|(kind, template)| {
                                            template
                                                .resolve_task(&kind.to_id_base(), &task_context)
                                                .map(|task| (kind.clone(), task))
                                        })
                                        .collect(),
                                    position: snapshot.buffer_snapshot.anchor_before(Point::new(
                                        multibuffer_point.row,
                                        tasks.column,
                                    )),
                                })
                            });
                        let spawn_straight_away = resolved_tasks
                            .as_ref()
                            .map_or(false, |tasks| tasks.templates.len() == 1)
                            && code_actions
                                .as_ref()
                                .map_or(true, |actions| actions.is_empty());
                        if let Some(task) = editor
                            .update(&mut cx, |editor, cx| {
                                *editor.context_menu.write() =
                                    Some(ContextMenu::CodeActions(CodeActionsMenu {
                                        buffer,
                                        actions: CodeActionContents {
                                            tasks: resolved_tasks,
                                            actions: code_actions,
                                        },
                                        selected_item: Default::default(),
                                        scroll_handle: UniformListScrollHandle::default(),
                                        deployed_from_indicator,
                                    }));
                                if spawn_straight_away {
                                    if let Some(task) = editor.confirm_code_action(
                                        &ConfirmCodeAction { item_ix: Some(0) },
                                        cx,
                                    ) {
                                        cx.notify();
                                        return task;
                                    }
                                }
                                cx.notify();
                                Task::ready(Ok(()))
                            })
                            .ok()
                        {
                            task.await
                        } else {
                            Ok(())
                        }
                    }))
                } else {
                    Some(Task::ready(Ok(())))
                }
            })?;
            if let Some(task) = spawned_test_task {
                task.await?;
            }

            Ok::<_, anyhow::Error>(())
        })
        .detach_and_log_err(cx);
    }

    pub fn confirm_code_action(
        &mut self,
        action: &ConfirmCodeAction,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let actions_menu = if let ContextMenu::CodeActions(menu) = self.hide_context_menu(cx)? {
            menu
        } else {
            return None;
        };
        let action_ix = action.item_ix.unwrap_or(actions_menu.selected_item);
        let action = actions_menu.actions.get(action_ix)?;
        let title = action.label();
        let buffer = actions_menu.buffer;
        let workspace = self.workspace()?;

        match action {
            CodeActionsItem::Task(task_source_kind, resolved_task) => {
                workspace.update(cx, |workspace, cx| {
                    workspace::tasks::schedule_resolved_task(
                        workspace,
                        task_source_kind,
                        resolved_task,
                        false,
                        cx,
                    );

                    Some(Task::ready(Ok(())))
                })
            }
            CodeActionsItem::CodeAction(action) => {
                let apply_code_actions = workspace
                    .read(cx)
                    .project()
                    .clone()
                    .update(cx, |project, cx| {
                        project.apply_code_action(buffer, action, true, cx)
                    });
                let workspace = workspace.downgrade();
                Some(cx.spawn(|editor, cx| async move {
                    let project_transaction = apply_code_actions.await?;
                    Self::open_project_transaction(
                        &editor,
                        workspace,
                        project_transaction,
                        title,
                        cx,
                    )
                    .await
                }))
            }
        }
    }

    pub async fn open_project_transaction(
        this: &WeakView<Editor>,
        workspace: WeakView<Workspace>,
        transaction: ProjectTransaction,
        title: String,
        mut cx: AsyncWindowContext,
    ) -> Result<()> {
        let replica_id = this.update(&mut cx, |this, cx| this.replica_id(cx))?;

        let mut entries = transaction.0.into_iter().collect::<Vec<_>>();
        cx.update(|cx| {
            entries.sort_unstable_by_key(|(buffer, _)| {
                buffer.read(cx).file().map(|f| f.path().clone())
            });
        })?;

        // If the project transaction's edits are all contained within this editor, then
        // avoid opening a new editor to display them.

        if let Some((buffer, transaction)) = entries.first() {
            if entries.len() == 1 {
                let excerpt = this.update(&mut cx, |editor, cx| {
                    editor
                        .buffer()
                        .read(cx)
                        .excerpt_containing(editor.selections.newest_anchor().head(), cx)
                })?;
                if let Some((_, excerpted_buffer, excerpt_range)) = excerpt {
                    if excerpted_buffer == *buffer {
                        let all_edits_within_excerpt = buffer.read_with(&cx, |buffer, _| {
                            let excerpt_range = excerpt_range.to_offset(buffer);
                            buffer
                                .edited_ranges_for_transaction::<usize>(transaction)
                                .all(|range| {
                                    excerpt_range.start <= range.start
                                        && excerpt_range.end >= range.end
                                })
                        })?;

                        if all_edits_within_excerpt {
                            return Ok(());
                        }
                    }
                }
            }
        } else {
            return Ok(());
        }

        let mut ranges_to_highlight = Vec::new();
        let excerpt_buffer = cx.new_model(|cx| {
            let mut multibuffer =
                MultiBuffer::new(replica_id, Capability::ReadWrite).with_title(title);
            for (buffer_handle, transaction) in &entries {
                let buffer = buffer_handle.read(cx);
                ranges_to_highlight.extend(
                    multibuffer.push_excerpts_with_context_lines(
                        buffer_handle.clone(),
                        buffer
                            .edited_ranges_for_transaction::<usize>(transaction)
                            .collect(),
                        DEFAULT_MULTIBUFFER_CONTEXT,
                        cx,
                    ),
                );
            }
            multibuffer.push_transaction(entries.iter().map(|(b, t)| (b, t)), cx);
            multibuffer
        })?;

        workspace.update(&mut cx, |workspace, cx| {
            let project = workspace.project().clone();
            let editor =
                cx.new_view(|cx| Editor::for_multibuffer(excerpt_buffer, Some(project), true, cx));
            workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, cx);
            editor.update(cx, |editor, cx| {
                editor.highlight_background::<Self>(
                    &ranges_to_highlight,
                    |theme| theme.editor_highlighted_line_background,
                    cx,
                );
            });
        })?;

        Ok(())
    }

    fn refresh_code_actions(&mut self, cx: &mut ViewContext<Self>) -> Option<()> {
        let project = self.project.clone()?;
        let buffer = self.buffer.read(cx);
        let newest_selection = self.selections.newest_anchor().clone();
        let (start_buffer, start) = buffer.text_anchor_for_position(newest_selection.start, cx)?;
        let (end_buffer, end) = buffer.text_anchor_for_position(newest_selection.end, cx)?;
        if start_buffer != end_buffer {
            return None;
        }

        self.code_actions_task = Some(cx.spawn(|this, mut cx| async move {
            cx.background_executor()
                .timer(CODE_ACTIONS_DEBOUNCE_TIMEOUT)
                .await;

            let actions = if let Ok(code_actions) = project.update(&mut cx, |project, cx| {
                project.code_actions(&start_buffer, start..end, cx)
            }) {
                code_actions.await
            } else {
                Vec::new()
            };

            this.update(&mut cx, |this, cx| {
                this.available_code_actions = if actions.is_empty() {
                    None
                } else {
                    Some((
                        Location {
                            buffer: start_buffer,
                            range: start..end,
                        },
                        actions.into(),
                    ))
                };
                cx.notify();
            })
            .log_err();
        }));
        None
    }

    fn start_inline_blame_timer(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(delay) = ProjectSettings::get_global(cx).git.inline_blame_delay() {
            self.show_git_blame_inline = false;

            self.show_git_blame_inline_delay_task = Some(cx.spawn(|this, mut cx| async move {
                cx.background_executor().timer(delay).await;

                this.update(&mut cx, |this, cx| {
                    this.show_git_blame_inline = true;
                    cx.notify();
                })
                .log_err();
            }));
        }
    }

    fn refresh_document_highlights(&mut self, cx: &mut ViewContext<Self>) -> Option<()> {
        if self.pending_rename.is_some() {
            return None;
        }

        let project = self.project.clone()?;
        let buffer = self.buffer.read(cx);
        let newest_selection = self.selections.newest_anchor().clone();
        let cursor_position = newest_selection.head();
        let (cursor_buffer, cursor_buffer_position) =
            buffer.text_anchor_for_position(cursor_position, cx)?;
        let (tail_buffer, _) = buffer.text_anchor_for_position(newest_selection.tail(), cx)?;
        if cursor_buffer != tail_buffer {
            return None;
        }

        self.document_highlights_task = Some(cx.spawn(|this, mut cx| async move {
            cx.background_executor()
                .timer(DOCUMENT_HIGHLIGHTS_DEBOUNCE_TIMEOUT)
                .await;

            let highlights = if let Some(highlights) = project
                .update(&mut cx, |project, cx| {
                    project.document_highlights(&cursor_buffer, cursor_buffer_position, cx)
                })
                .log_err()
            {
                highlights.await.log_err()
            } else {
                None
            };

            if let Some(highlights) = highlights {
                this.update(&mut cx, |this, cx| {
                    if this.pending_rename.is_some() {
                        return;
                    }

                    let buffer_id = cursor_position.buffer_id;
                    let buffer = this.buffer.read(cx);
                    if !buffer
                        .text_anchor_for_position(cursor_position, cx)
                        .map_or(false, |(buffer, _)| buffer == cursor_buffer)
                    {
                        return;
                    }

                    let cursor_buffer_snapshot = cursor_buffer.read(cx);
                    let mut write_ranges = Vec::new();
                    let mut read_ranges = Vec::new();
                    for highlight in highlights {
                        for (excerpt_id, excerpt_range) in
                            buffer.excerpts_for_buffer(&cursor_buffer, cx)
                        {
                            let start = highlight
                                .range
                                .start
                                .max(&excerpt_range.context.start, cursor_buffer_snapshot);
                            let end = highlight
                                .range
                                .end
                                .min(&excerpt_range.context.end, cursor_buffer_snapshot);
                            if start.cmp(&end, cursor_buffer_snapshot).is_ge() {
                                continue;
                            }

                            let range = Anchor {
                                buffer_id,
                                excerpt_id: excerpt_id,
                                text_anchor: start,
                            }..Anchor {
                                buffer_id,
                                excerpt_id,
                                text_anchor: end,
                            };
                            if highlight.kind == lsp::DocumentHighlightKind::WRITE {
                                write_ranges.push(range);
                            } else {
                                read_ranges.push(range);
                            }
                        }
                    }

                    this.highlight_background::<DocumentHighlightRead>(
                        &read_ranges,
                        |theme| theme.editor_document_highlight_read_background,
                        cx,
                    );
                    this.highlight_background::<DocumentHighlightWrite>(
                        &write_ranges,
                        |theme| theme.editor_document_highlight_write_background,
                        cx,
                    );
                    cx.notify();
                })
                .log_err();
            }
        }));
        None
    }

    fn refresh_inline_completion(
        &mut self,
        debounce: bool,
        cx: &mut ViewContext<Self>,
    ) -> Option<()> {
        let provider = self.inline_completion_provider()?;
        let cursor = self.selections.newest_anchor().head();
        let (buffer, cursor_buffer_position) =
            self.buffer.read(cx).text_anchor_for_position(cursor, cx)?;
        if !self.show_inline_completions
            || !provider.is_enabled(&buffer, cursor_buffer_position, cx)
        {
            self.discard_inline_completion(false, cx);
            return None;
        }

        self.update_visible_inline_completion(cx);
        provider.refresh(buffer, cursor_buffer_position, debounce, cx);
        Some(())
    }

    fn cycle_inline_completion(
        &mut self,
        direction: Direction,
        cx: &mut ViewContext<Self>,
    ) -> Option<()> {
        let provider = self.inline_completion_provider()?;
        let cursor = self.selections.newest_anchor().head();
        let (buffer, cursor_buffer_position) =
            self.buffer.read(cx).text_anchor_for_position(cursor, cx)?;
        if !self.show_inline_completions
            || !provider.is_enabled(&buffer, cursor_buffer_position, cx)
        {
            return None;
        }

        provider.cycle(buffer, cursor_buffer_position, direction, cx);
        self.update_visible_inline_completion(cx);

        Some(())
    }

    pub fn show_inline_completion(&mut self, _: &ShowInlineCompletion, cx: &mut ViewContext<Self>) {
        if !self.has_active_inline_completion(cx) {
            self.refresh_inline_completion(false, cx);
            return;
        }

        self.update_visible_inline_completion(cx);
    }

    pub fn display_cursor_names(&mut self, _: &DisplayCursorNames, cx: &mut ViewContext<Self>) {
        self.show_cursor_names(cx);
    }

    fn show_cursor_names(&mut self, cx: &mut ViewContext<Self>) {
        self.show_cursor_names = true;
        cx.notify();
        cx.spawn(|this, mut cx| async move {
            cx.background_executor().timer(CURSORS_VISIBLE_FOR).await;
            this.update(&mut cx, |this, cx| {
                this.show_cursor_names = false;
                cx.notify()
            })
            .ok()
        })
        .detach();
    }

    pub fn next_inline_completion(&mut self, _: &NextInlineCompletion, cx: &mut ViewContext<Self>) {
        if self.has_active_inline_completion(cx) {
            self.cycle_inline_completion(Direction::Next, cx);
        } else {
            let is_copilot_disabled = self.refresh_inline_completion(false, cx).is_none();
            if is_copilot_disabled {
                cx.propagate();
            }
        }
    }

    pub fn previous_inline_completion(
        &mut self,
        _: &PreviousInlineCompletion,
        cx: &mut ViewContext<Self>,
    ) {
        if self.has_active_inline_completion(cx) {
            self.cycle_inline_completion(Direction::Prev, cx);
        } else {
            let is_copilot_disabled = self.refresh_inline_completion(false, cx).is_none();
            if is_copilot_disabled {
                cx.propagate();
            }
        }
    }

    pub fn accept_inline_completion(
        &mut self,
        _: &AcceptInlineCompletion,
        cx: &mut ViewContext<Self>,
    ) {
        let Some((completion, delete_range)) = self.take_active_inline_completion(cx) else {
            return;
        };
        if let Some(provider) = self.inline_completion_provider() {
            provider.accept(cx);
        }

        cx.emit(EditorEvent::InputHandled {
            utf16_range_to_replace: None,
            text: completion.text.to_string().into(),
        });

        if let Some(range) = delete_range {
            self.change_selections(None, cx, |s| s.select_ranges([range]))
        }
        self.insert_with_autoindent_mode(&completion.text.to_string(), None, cx);
        self.refresh_inline_completion(true, cx);
        cx.notify();
    }

    pub fn accept_partial_inline_completion(
        &mut self,
        _: &AcceptPartialInlineCompletion,
        cx: &mut ViewContext<Self>,
    ) {
        if self.selections.count() == 1 && self.has_active_inline_completion(cx) {
            if let Some((completion, delete_range)) = self.take_active_inline_completion(cx) {
                let mut partial_completion = completion
                    .text
                    .chars()
                    .by_ref()
                    .take_while(|c| c.is_alphabetic())
                    .collect::<String>();
                if partial_completion.is_empty() {
                    partial_completion = completion
                        .text
                        .chars()
                        .by_ref()
                        .take_while(|c| c.is_whitespace() || !c.is_alphabetic())
                        .collect::<String>();
                }

                cx.emit(EditorEvent::InputHandled {
                    utf16_range_to_replace: None,
                    text: partial_completion.clone().into(),
                });

                if let Some(range) = delete_range {
                    self.change_selections(None, cx, |s| s.select_ranges([range]))
                }
                self.insert_with_autoindent_mode(&partial_completion, None, cx);

                self.refresh_inline_completion(true, cx);
                cx.notify();
            }
        }
    }

    fn discard_inline_completion(
        &mut self,
        should_report_inline_completion_event: bool,
        cx: &mut ViewContext<Self>,
    ) -> bool {
        if let Some(provider) = self.inline_completion_provider() {
            provider.discard(should_report_inline_completion_event, cx);
        }

        self.take_active_inline_completion(cx).is_some()
    }

    pub fn has_active_inline_completion(&self, cx: &AppContext) -> bool {
        if let Some(completion) = self.active_inline_completion.as_ref() {
            let buffer = self.buffer.read(cx).read(cx);
            completion.0.position.is_valid(&buffer)
        } else {
            false
        }
    }

    fn take_active_inline_completion(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> Option<(Inlay, Option<Range<Anchor>>)> {
        let completion = self.active_inline_completion.take()?;
        self.display_map.update(cx, |map, cx| {
            map.splice_inlays(vec![completion.0.id], Default::default(), cx);
        });
        let buffer = self.buffer.read(cx).read(cx);

        if completion.0.position.is_valid(&buffer) {
            Some(completion)
        } else {
            None
        }
    }

    fn update_visible_inline_completion(&mut self, cx: &mut ViewContext<Self>) {
        let selection = self.selections.newest_anchor();
        let cursor = selection.head();

        let excerpt_id = cursor.excerpt_id;

        if self.context_menu.read().is_none()
            && self.completion_tasks.is_empty()
            && selection.start == selection.end
        {
            if let Some(provider) = self.inline_completion_provider() {
                if let Some((buffer, cursor_buffer_position)) =
                    self.buffer.read(cx).text_anchor_for_position(cursor, cx)
                {
                    if let Some((text, text_anchor_range)) =
                        provider.active_completion_text(&buffer, cursor_buffer_position, cx)
                    {
                        let text = Rope::from(text);
                        let mut to_remove = Vec::new();
                        if let Some(completion) = self.active_inline_completion.take() {
                            to_remove.push(completion.0.id);
                        }

                        let completion_inlay =
                            Inlay::suggestion(post_inc(&mut self.next_inlay_id), cursor, text);

                        let multibuffer_anchor_range = text_anchor_range.and_then(|range| {
                            let snapshot = self.buffer.read(cx).snapshot(cx);
                            Some(
                                snapshot.anchor_in_excerpt(excerpt_id, range.start)?
                                    ..snapshot.anchor_in_excerpt(excerpt_id, range.end)?,
                            )
                        });
                        self.active_inline_completion =
                            Some((completion_inlay.clone(), multibuffer_anchor_range));

                        self.display_map.update(cx, move |map, cx| {
                            map.splice_inlays(to_remove, vec![completion_inlay], cx)
                        });
                        cx.notify();
                        return;
                    }
                }
            }
        }

        self.discard_inline_completion(false, cx);
    }

    fn inline_completion_provider(&self) -> Option<Arc<dyn InlineCompletionProviderHandle>> {
        Some(self.inline_completion_provider.as_ref()?.provider.clone())
    }

    fn render_code_actions_indicator(
        &self,
        _style: &EditorStyle,
        row: DisplayRow,
        is_active: bool,
        cx: &mut ViewContext<Self>,
    ) -> Option<IconButton> {
        if self.available_code_actions.is_some() {
            Some(
                IconButton::new("code_actions_indicator", ui::IconName::Bolt)
                    .shape(ui::IconButtonShape::Square)
                    .icon_size(IconSize::XSmall)
                    .icon_color(Color::Muted)
                    .selected(is_active)
                    .on_click(cx.listener(move |editor, _e, cx| {
                        editor.focus(cx);
                        editor.toggle_code_actions(
                            &ToggleCodeActions {
                                deployed_from_indicator: Some(row),
                            },
                            cx,
                        );
                    })),
            )
        } else {
            None
        }
    }

    fn clear_tasks(&mut self) {
        self.tasks.clear()
    }

    fn insert_tasks(&mut self, key: (BufferId, BufferRow), value: RunnableTasks) {
        if let Some(_) = self.tasks.insert(key, value) {
            // This case should hopefully be rare, but just in case...
            log::error!("multiple different run targets found on a single line, only the last target will be rendered")
        }
    }

    fn render_run_indicator(
        &self,
        _style: &EditorStyle,
        is_active: bool,
        row: DisplayRow,
        cx: &mut ViewContext<Self>,
    ) -> IconButton {
        IconButton::new(("run_indicator", row.0 as usize), ui::IconName::Play)
            .shape(ui::IconButtonShape::Square)
            .icon_size(IconSize::XSmall)
            .icon_color(Color::Muted)
            .selected(is_active)
            .on_click(cx.listener(move |editor, _e, cx| {
                editor.focus(cx);
                editor.toggle_code_actions(
                    &ToggleCodeActions {
                        deployed_from_indicator: Some(row),
                    },
                    cx,
                );
            }))
    }

    fn close_hunk_diff_button(
        &self,
        hunk: HoveredHunk,
        row: DisplayRow,
        cx: &mut ViewContext<Self>,
    ) -> IconButton {
        IconButton::new(
            ("close_hunk_diff_indicator", row.0 as usize),
            ui::IconName::Close,
        )
        .shape(ui::IconButtonShape::Square)
        .icon_size(IconSize::XSmall)
        .icon_color(Color::Muted)
        .tooltip(|cx| Tooltip::for_action("Close hunk diff", &ToggleHunkDiff, cx))
        .on_click(cx.listener(move |editor, _e, cx| editor.toggle_hovered_hunk(&hunk, cx)))
    }

    pub fn context_menu_visible(&self) -> bool {
        self.context_menu
            .read()
            .as_ref()
            .map_or(false, |menu| menu.visible())
    }

    fn render_context_menu(
        &self,
        cursor_position: DisplayPoint,
        style: &EditorStyle,
        max_height: Pixels,
        cx: &mut ViewContext<Editor>,
    ) -> Option<(ContextMenuOrigin, AnyElement)> {
        self.context_menu.read().as_ref().map(|menu| {
            menu.render(
                cursor_position,
                style,
                max_height,
                self.workspace.as_ref().map(|(w, _)| w.clone()),
                cx,
            )
        })
    }

    fn hide_context_menu(&mut self, cx: &mut ViewContext<Self>) -> Option<ContextMenu> {
        cx.notify();
        self.completion_tasks.clear();
        let context_menu = self.context_menu.write().take();
        if context_menu.is_some() {
            self.update_visible_inline_completion(cx);
        }
        context_menu
    }

    pub fn insert_snippet(
        &mut self,
        insertion_ranges: &[Range<usize>],
        snippet: Snippet,
        cx: &mut ViewContext<Self>,
    ) -> Result<()> {
        struct Tabstop<T> {
            is_end_tabstop: bool,
            ranges: Vec<Range<T>>,
        }

        let tabstops = self.buffer.update(cx, |buffer, cx| {
            let snippet_text: Arc<str> = snippet.text.clone().into();
            buffer.edit(
                insertion_ranges
                    .iter()
                    .cloned()
                    .map(|range| (range, snippet_text.clone())),
                Some(AutoindentMode::EachLine),
                cx,
            );

            let snapshot = &*buffer.read(cx);
            let snippet = &snippet;
            snippet
                .tabstops
                .iter()
                .map(|tabstop| {
                    let is_end_tabstop = tabstop.first().map_or(false, |tabstop| {
                        tabstop.is_empty() && tabstop.start == snippet.text.len() as isize
                    });
                    let mut tabstop_ranges = tabstop
                        .iter()
                        .flat_map(|tabstop_range| {
                            let mut delta = 0_isize;
                            insertion_ranges.iter().map(move |insertion_range| {
                                let insertion_start = insertion_range.start as isize + delta;
                                delta +=
                                    snippet.text.len() as isize - insertion_range.len() as isize;

                                let start = ((insertion_start + tabstop_range.start) as usize)
                                    .min(snapshot.len());
                                let end = ((insertion_start + tabstop_range.end) as usize)
                                    .min(snapshot.len());
                                snapshot.anchor_before(start)..snapshot.anchor_after(end)
                            })
                        })
                        .collect::<Vec<_>>();
                    tabstop_ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start, snapshot));

                    Tabstop {
                        is_end_tabstop,
                        ranges: tabstop_ranges,
                    }
                })
                .collect::<Vec<_>>()
        });
        if let Some(tabstop) = tabstops.first() {
            self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select_ranges(tabstop.ranges.iter().cloned());
            });

            // If we're already at the last tabstop and it's at the end of the snippet,
            // we're done, we don't need to keep the state around.
            if !tabstop.is_end_tabstop {
                let ranges = tabstops
                    .into_iter()
                    .map(|tabstop| tabstop.ranges)
                    .collect::<Vec<_>>();
                self.snippet_stack.push(SnippetState {
                    active_index: 0,
                    ranges,
                });
            }

            // Check whether the just-entered snippet ends with an auto-closable bracket.
            if self.autoclose_regions.is_empty() {
                let snapshot = self.buffer.read(cx).snapshot(cx);
                for selection in &mut self.selections.all::<Point>(cx) {
                    let selection_head = selection.head();
                    let Some(scope) = snapshot.language_scope_at(selection_head) else {
                        continue;
                    };

                    let mut bracket_pair = None;
                    let next_chars = snapshot.chars_at(selection_head).collect::<String>();
                    let prev_chars = snapshot
                        .reversed_chars_at(selection_head)
                        .collect::<String>();
                    for (pair, enabled) in scope.brackets() {
                        if enabled
                            && pair.close
                            && prev_chars.starts_with(pair.start.as_str())
                            && next_chars.starts_with(pair.end.as_str())
                        {
                            bracket_pair = Some(pair.clone());
                            break;
                        }
                    }
                    if let Some(pair) = bracket_pair {
                        let start = snapshot.anchor_after(selection_head);
                        let end = snapshot.anchor_after(selection_head);
                        self.autoclose_regions.push(AutocloseRegion {
                            selection_id: selection.id,
                            range: start..end,
                            pair,
                        });
                    }
                }
            }
        }
        Ok(())
    }

    pub fn move_to_next_snippet_tabstop(&mut self, cx: &mut ViewContext<Self>) -> bool {
        self.move_to_snippet_tabstop(Bias::Right, cx)
    }

    pub fn move_to_prev_snippet_tabstop(&mut self, cx: &mut ViewContext<Self>) -> bool {
        self.move_to_snippet_tabstop(Bias::Left, cx)
    }

    pub fn move_to_snippet_tabstop(&mut self, bias: Bias, cx: &mut ViewContext<Self>) -> bool {
        if let Some(mut snippet) = self.snippet_stack.pop() {
            match bias {
                Bias::Left => {
                    if snippet.active_index > 0 {
                        snippet.active_index -= 1;
                    } else {
                        self.snippet_stack.push(snippet);
                        return false;
                    }
                }
                Bias::Right => {
                    if snippet.active_index + 1 < snippet.ranges.len() {
                        snippet.active_index += 1;
                    } else {
                        self.snippet_stack.push(snippet);
                        return false;
                    }
                }
            }
            if let Some(current_ranges) = snippet.ranges.get(snippet.active_index) {
                self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.select_anchor_ranges(current_ranges.iter().cloned())
                });
                // If snippet state is not at the last tabstop, push it back on the stack
                if snippet.active_index + 1 < snippet.ranges.len() {
                    self.snippet_stack.push(snippet);
                }
                return true;
            }
        }

        false
    }

    pub fn clear(&mut self, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.select_all(&SelectAll, cx);
            this.insert("", cx);
        });
    }

    pub fn backspace(&mut self, _: &Backspace, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.select_autoclose_pair(cx);
            let mut linked_ranges = HashMap::<_, Vec<_>>::default();
            if !this.linked_edit_ranges.is_empty() {
                let selections = this.selections.all::<MultiBufferPoint>(cx);
                let snapshot = this.buffer.read(cx).snapshot(cx);

                for selection in selections.iter() {
                    let selection_start = snapshot.anchor_before(selection.start).text_anchor;
                    let selection_end = snapshot.anchor_after(selection.end).text_anchor;
                    if selection_start.buffer_id != selection_end.buffer_id {
                        continue;
                    }
                    if let Some(ranges) =
                        this.linked_editing_ranges_for(selection_start..selection_end, cx)
                    {
                        for (buffer, entries) in ranges {
                            linked_ranges.entry(buffer).or_default().extend(entries);
                        }
                    }
                }
            }

            let mut selections = this.selections.all::<MultiBufferPoint>(cx);
            if !this.selections.line_mode {
                let display_map = this.display_map.update(cx, |map, cx| map.snapshot(cx));
                for selection in &mut selections {
                    if selection.is_empty() {
                        let old_head = selection.head();
                        let mut new_head =
                            movement::left(&display_map, old_head.to_display_point(&display_map))
                                .to_point(&display_map);
                        if let Some((buffer, line_buffer_range)) = display_map
                            .buffer_snapshot
                            .buffer_line_for_row(MultiBufferRow(old_head.row))
                        {
                            let indent_size =
                                buffer.indent_size_for_line(line_buffer_range.start.row);
                            let indent_len = match indent_size.kind {
                                IndentKind::Space => {
                                    buffer.settings_at(line_buffer_range.start, cx).tab_size
                                }
                                IndentKind::Tab => NonZeroU32::new(1).unwrap(),
                            };
                            if old_head.column <= indent_size.len && old_head.column > 0 {
                                let indent_len = indent_len.get();
                                new_head = cmp::min(
                                    new_head,
                                    MultiBufferPoint::new(
                                        old_head.row,
                                        ((old_head.column - 1) / indent_len) * indent_len,
                                    ),
                                );
                            }
                        }

                        selection.set_head(new_head, SelectionGoal::None);
                    }
                }
            }

            this.signature_help_state.set_backspace_pressed(true);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(selections));
            this.insert("", cx);
            let empty_str: Arc<str> = Arc::from("");
            for (buffer, edits) in linked_ranges {
                let snapshot = buffer.read(cx).snapshot();
                use text::ToPoint as TP;

                let edits = edits
                    .into_iter()
                    .map(|range| {
                        let end_point = TP::to_point(&range.end, &snapshot);
                        let mut start_point = TP::to_point(&range.start, &snapshot);

                        if end_point == start_point {
                            let offset = text::ToOffset::to_offset(&range.start, &snapshot)
                                .saturating_sub(1);
                            start_point = TP::to_point(&offset, &snapshot);
                        };

                        (start_point..end_point, empty_str.clone())
                    })
                    .sorted_by_key(|(range, _)| range.start)
                    .collect::<Vec<_>>();
                buffer.update(cx, |this, cx| {
                    this.edit(edits, None, cx);
                })
            }
            this.refresh_inline_completion(true, cx);
            linked_editing_ranges::refresh_linked_ranges(this, cx);
        });
    }

    pub fn delete(&mut self, _: &Delete, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let line_mode = s.line_mode;
                s.move_with(|map, selection| {
                    if selection.is_empty() && !line_mode {
                        let cursor = movement::right(map, selection.head());
                        selection.end = cursor;
                        selection.reversed = true;
                        selection.goal = SelectionGoal::None;
                    }
                })
            });
            this.insert("", cx);
            this.refresh_inline_completion(true, cx);
        });
    }

    pub fn tab_prev(&mut self, _: &TabPrev, cx: &mut ViewContext<Self>) {
        if self.move_to_prev_snippet_tabstop(cx) {
            return;
        }

        self.outdent(&Outdent, cx);
    }

    pub fn tab(&mut self, _: &Tab, cx: &mut ViewContext<Self>) {
        if self.move_to_next_snippet_tabstop(cx) || self.read_only(cx) {
            return;
        }

        let mut selections = self.selections.all_adjusted(cx);
        let buffer = self.buffer.read(cx);
        let snapshot = buffer.snapshot(cx);
        let rows_iter = selections.iter().map(|s| s.head().row);
        let suggested_indents = snapshot.suggested_indents(rows_iter, cx);

        let mut edits = Vec::new();
        let mut prev_edited_row = 0;
        let mut row_delta = 0;
        for selection in &mut selections {
            if selection.start.row != prev_edited_row {
                row_delta = 0;
            }
            prev_edited_row = selection.end.row;

            // If the selection is non-empty, then increase the indentation of the selected lines.
            if !selection.is_empty() {
                row_delta =
                    Self::indent_selection(buffer, &snapshot, selection, &mut edits, row_delta, cx);
                continue;
            }

            // If the selection is empty and the cursor is in the leading whitespace before the
            // suggested indentation, then auto-indent the line.
            let cursor = selection.head();
            let current_indent = snapshot.indent_size_for_line(MultiBufferRow(cursor.row));
            if let Some(suggested_indent) =
                suggested_indents.get(&MultiBufferRow(cursor.row)).copied()
            {
                if cursor.column < suggested_indent.len
                    && cursor.column <= current_indent.len
                    && current_indent.len <= suggested_indent.len
                {
                    selection.start = Point::new(cursor.row, suggested_indent.len);
                    selection.end = selection.start;
                    if row_delta == 0 {
                        edits.extend(Buffer::edit_for_indent_size_adjustment(
                            cursor.row,
                            current_indent,
                            suggested_indent,
                        ));
                        row_delta = suggested_indent.len - current_indent.len;
                    }
                    continue;
                }
            }

            // Otherwise, insert a hard or soft tab.
            let settings = buffer.settings_at(cursor, cx);
            let tab_size = if settings.hard_tabs {
                IndentSize::tab()
            } else {
                let tab_size = settings.tab_size.get();
                let char_column = snapshot
                    .text_for_range(Point::new(cursor.row, 0)..cursor)
                    .flat_map(str::chars)
                    .count()
                    + row_delta as usize;
                let chars_to_next_tab_stop = tab_size - (char_column as u32 % tab_size);
                IndentSize::spaces(chars_to_next_tab_stop)
            };
            selection.start = Point::new(cursor.row, cursor.column + row_delta + tab_size.len);
            selection.end = selection.start;
            edits.push((cursor..cursor, tab_size.chars().collect::<String>()));
            row_delta += tab_size.len;
        }

        self.transact(cx, |this, cx| {
            this.buffer.update(cx, |b, cx| b.edit(edits, None, cx));
            this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(selections));
            this.refresh_inline_completion(true, cx);
        });
    }

    pub fn indent(&mut self, _: &Indent, cx: &mut ViewContext<Self>) {
        if self.read_only(cx) {
            return;
        }
        let mut selections = self.selections.all::<Point>(cx);
        let mut prev_edited_row = 0;
        let mut row_delta = 0;
        let mut edits = Vec::new();
        let buffer = self.buffer.read(cx);
        let snapshot = buffer.snapshot(cx);
        for selection in &mut selections {
            if selection.start.row != prev_edited_row {
                row_delta = 0;
            }
            prev_edited_row = selection.end.row;

            row_delta =
                Self::indent_selection(buffer, &snapshot, selection, &mut edits, row_delta, cx);
        }

        self.transact(cx, |this, cx| {
            this.buffer.update(cx, |b, cx| b.edit(edits, None, cx));
            this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(selections));
        });
    }

    fn indent_selection(
        buffer: &MultiBuffer,
        snapshot: &MultiBufferSnapshot,
        selection: &mut Selection<Point>,
        edits: &mut Vec<(Range<Point>, String)>,
        delta_for_start_row: u32,
        cx: &AppContext,
    ) -> u32 {
        let settings = buffer.settings_at(selection.start, cx);
        let tab_size = settings.tab_size.get();
        let indent_kind = if settings.hard_tabs {
            IndentKind::Tab
        } else {
            IndentKind::Space
        };
        let mut start_row = selection.start.row;
        let mut end_row = selection.end.row + 1;

        // If a selection ends at the beginning of a line, don't indent
        // that last line.
        if selection.end.column == 0 && selection.end.row > selection.start.row {
            end_row -= 1;
        }

        // Avoid re-indenting a row that has already been indented by a
        // previous selection, but still update this selection's column
        // to reflect that indentation.
        if delta_for_start_row > 0 {
            start_row += 1;
            selection.start.column += delta_for_start_row;
            if selection.end.row == selection.start.row {
                selection.end.column += delta_for_start_row;
            }
        }

        let mut delta_for_end_row = 0;
        let has_multiple_rows = start_row + 1 != end_row;
        for row in start_row..end_row {
            let current_indent = snapshot.indent_size_for_line(MultiBufferRow(row));
            let indent_delta = match (current_indent.kind, indent_kind) {
                (IndentKind::Space, IndentKind::Space) => {
                    let columns_to_next_tab_stop = tab_size - (current_indent.len % tab_size);
                    IndentSize::spaces(columns_to_next_tab_stop)
                }
                (IndentKind::Tab, IndentKind::Space) => IndentSize::spaces(tab_size),
                (_, IndentKind::Tab) => IndentSize::tab(),
            };

            let start = if has_multiple_rows || current_indent.len < selection.start.column {
                0
            } else {
                selection.start.column
            };
            let row_start = Point::new(row, start);
            edits.push((
                row_start..row_start,
                indent_delta.chars().collect::<String>(),
            ));

            // Update this selection's endpoints to reflect the indentation.
            if row == selection.start.row {
                selection.start.column += indent_delta.len;
            }
            if row == selection.end.row {
                selection.end.column += indent_delta.len;
                delta_for_end_row = indent_delta.len;
            }
        }

        if selection.start.row == selection.end.row {
            delta_for_start_row + delta_for_end_row
        } else {
            delta_for_end_row
        }
    }

    pub fn outdent(&mut self, _: &Outdent, cx: &mut ViewContext<Self>) {
        if self.read_only(cx) {
            return;
        }
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let selections = self.selections.all::<Point>(cx);
        let mut deletion_ranges = Vec::new();
        let mut last_outdent = None;
        {
            let buffer = self.buffer.read(cx);
            let snapshot = buffer.snapshot(cx);
            for selection in &selections {
                let settings = buffer.settings_at(selection.start, cx);
                let tab_size = settings.tab_size.get();
                let mut rows = selection.spanned_rows(false, &display_map);

                // Avoid re-outdenting a row that has already been outdented by a
                // previous selection.
                if let Some(last_row) = last_outdent {
                    if last_row == rows.start {
                        rows.start = rows.start.next_row();
                    }
                }
                let has_multiple_rows = rows.len() > 1;
                for row in rows.iter_rows() {
                    let indent_size = snapshot.indent_size_for_line(row);
                    if indent_size.len > 0 {
                        let deletion_len = match indent_size.kind {
                            IndentKind::Space => {
                                let columns_to_prev_tab_stop = indent_size.len % tab_size;
                                if columns_to_prev_tab_stop == 0 {
                                    tab_size
                                } else {
                                    columns_to_prev_tab_stop
                                }
                            }
                            IndentKind::Tab => 1,
                        };
                        let start = if has_multiple_rows
                            || deletion_len > selection.start.column
                            || indent_size.len < selection.start.column
                        {
                            0
                        } else {
                            selection.start.column - deletion_len
                        };
                        deletion_ranges.push(
                            Point::new(row.0, start)..Point::new(row.0, start + deletion_len),
                        );
                        last_outdent = Some(row);
                    }
                }
            }
        }

        self.transact(cx, |this, cx| {
            this.buffer.update(cx, |buffer, cx| {
                let empty_str: Arc<str> = Arc::default();
                buffer.edit(
                    deletion_ranges
                        .into_iter()
                        .map(|range| (range, empty_str.clone())),
                    None,
                    cx,
                );
            });
            let selections = this.selections.all::<usize>(cx);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(selections));
        });
    }

    pub fn delete_line(&mut self, _: &DeleteLine, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let selections = self.selections.all::<Point>(cx);

        let mut new_cursors = Vec::new();
        let mut edit_ranges = Vec::new();
        let mut selections = selections.iter().peekable();
        while let Some(selection) = selections.next() {
            let mut rows = selection.spanned_rows(false, &display_map);
            let goal_display_column = selection.head().to_display_point(&display_map).column();

            // Accumulate contiguous regions of rows that we want to delete.
            while let Some(next_selection) = selections.peek() {
                let next_rows = next_selection.spanned_rows(false, &display_map);
                if next_rows.start <= rows.end {
                    rows.end = next_rows.end;
                    selections.next().unwrap();
                } else {
                    break;
                }
            }

            let buffer = &display_map.buffer_snapshot;
            let mut edit_start = Point::new(rows.start.0, 0).to_offset(buffer);
            let edit_end;
            let cursor_buffer_row;
            if buffer.max_point().row >= rows.end.0 {
                // If there's a line after the range, delete the \n from the end of the row range
                // and position the cursor on the next line.
                edit_end = Point::new(rows.end.0, 0).to_offset(buffer);
                cursor_buffer_row = rows.end;
            } else {
                // If there isn't a line after the range, delete the \n from the line before the
                // start of the row range and position the cursor there.
                edit_start = edit_start.saturating_sub(1);
                edit_end = buffer.len();
                cursor_buffer_row = rows.start.previous_row();
            }

            let mut cursor = Point::new(cursor_buffer_row.0, 0).to_display_point(&display_map);
            *cursor.column_mut() =
                cmp::min(goal_display_column, display_map.line_len(cursor.row()));

            new_cursors.push((
                selection.id,
                buffer.anchor_after(cursor.to_point(&display_map)),
            ));
            edit_ranges.push(edit_start..edit_end);
        }

        self.transact(cx, |this, cx| {
            let buffer = this.buffer.update(cx, |buffer, cx| {
                let empty_str: Arc<str> = Arc::default();
                buffer.edit(
                    edit_ranges
                        .into_iter()
                        .map(|range| (range, empty_str.clone())),
                    None,
                    cx,
                );
                buffer.snapshot(cx)
            });
            let new_selections = new_cursors
                .into_iter()
                .map(|(id, cursor)| {
                    let cursor = cursor.to_point(&buffer);
                    Selection {
                        id,
                        start: cursor,
                        end: cursor,
                        reversed: false,
                        goal: SelectionGoal::None,
                    }
                })
                .collect();

            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(new_selections);
            });
        });
    }

    pub fn join_lines(&mut self, _: &JoinLines, cx: &mut ViewContext<Self>) {
        if self.read_only(cx) {
            return;
        }
        let mut row_ranges = Vec::<Range<MultiBufferRow>>::new();
        for selection in self.selections.all::<Point>(cx) {
            let start = MultiBufferRow(selection.start.row);
            let end = if selection.start.row == selection.end.row {
                MultiBufferRow(selection.start.row + 1)
            } else {
                MultiBufferRow(selection.end.row)
            };

            if let Some(last_row_range) = row_ranges.last_mut() {
                if start <= last_row_range.end {
                    last_row_range.end = end;
                    continue;
                }
            }
            row_ranges.push(start..end);
        }

        let snapshot = self.buffer.read(cx).snapshot(cx);
        let mut cursor_positions = Vec::new();
        for row_range in &row_ranges {
            let anchor = snapshot.anchor_before(Point::new(
                row_range.end.previous_row().0,
                snapshot.line_len(row_range.end.previous_row()),
            ));
            cursor_positions.push(anchor..anchor);
        }

        self.transact(cx, |this, cx| {
            for row_range in row_ranges.into_iter().rev() {
                for row in row_range.iter_rows().rev() {
                    let end_of_line = Point::new(row.0, snapshot.line_len(row));
                    let next_line_row = row.next_row();
                    let indent = snapshot.indent_size_for_line(next_line_row);
                    let start_of_next_line = Point::new(next_line_row.0, indent.len);

                    let replace = if snapshot.line_len(next_line_row) > indent.len {
                        " "
                    } else {
                        ""
                    };

                    this.buffer.update(cx, |buffer, cx| {
                        buffer.edit([(end_of_line..start_of_next_line, replace)], None, cx)
                    });
                }
            }

            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select_anchor_ranges(cursor_positions)
            });
        });
    }

    pub fn sort_lines_case_sensitive(
        &mut self,
        _: &SortLinesCaseSensitive,
        cx: &mut ViewContext<Self>,
    ) {
        self.manipulate_lines(cx, |lines| lines.sort())
    }

    pub fn sort_lines_case_insensitive(
        &mut self,
        _: &SortLinesCaseInsensitive,
        cx: &mut ViewContext<Self>,
    ) {
        self.manipulate_lines(cx, |lines| lines.sort_by_key(|line| line.to_lowercase()))
    }

    pub fn unique_lines_case_insensitive(
        &mut self,
        _: &UniqueLinesCaseInsensitive,
        cx: &mut ViewContext<Self>,
    ) {
        self.manipulate_lines(cx, |lines| {
            let mut seen = HashSet::default();
            lines.retain(|line| seen.insert(line.to_lowercase()));
        })
    }

    pub fn unique_lines_case_sensitive(
        &mut self,
        _: &UniqueLinesCaseSensitive,
        cx: &mut ViewContext<Self>,
    ) {
        self.manipulate_lines(cx, |lines| {
            let mut seen = HashSet::default();
            lines.retain(|line| seen.insert(*line));
        })
    }

    pub fn revert_selected_hunks(&mut self, _: &RevertSelectedHunks, cx: &mut ViewContext<Self>) {
        let revert_changes = self.gather_revert_changes(&self.selections.disjoint_anchors(), cx);
        if !revert_changes.is_empty() {
            self.transact(cx, |editor, cx| {
                editor.revert(revert_changes, cx);
            });
        }
    }

    pub fn open_active_item_in_terminal(&mut self, _: &OpenInTerminal, cx: &mut ViewContext<Self>) {
        if let Some(working_directory) = self.active_excerpt(cx).and_then(|(_, buffer, _)| {
            let project_path = buffer.read(cx).project_path(cx)?;
            let project = self.project.as_ref()?.read(cx);
            let entry = project.entry_for_path(&project_path, cx)?;
            let abs_path = project.absolute_path(&project_path, cx)?;
            let parent = if entry.is_symlink {
                abs_path.canonicalize().ok()?
            } else {
                abs_path
            }
            .parent()?
            .to_path_buf();
            Some(parent)
        }) {
            cx.dispatch_action(OpenTerminal { working_directory }.boxed_clone());
        }
    }

    fn gather_revert_changes(
        &mut self,
        selections: &[Selection<Anchor>],
        cx: &mut ViewContext<'_, Editor>,
    ) -> HashMap<BufferId, Vec<(Range<text::Anchor>, Rope)>> {
        let mut revert_changes = HashMap::default();
        let multi_buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        for hunk in hunks_for_selections(&multi_buffer_snapshot, selections) {
            Self::prepare_revert_change(&mut revert_changes, self.buffer(), &hunk, cx);
        }
        revert_changes
    }

    pub fn prepare_revert_change(
        revert_changes: &mut HashMap<BufferId, Vec<(Range<text::Anchor>, Rope)>>,
        multi_buffer: &Model<MultiBuffer>,
        hunk: &DiffHunk<MultiBufferRow>,
        cx: &AppContext,
    ) -> Option<()> {
        let buffer = multi_buffer.read(cx).buffer(hunk.buffer_id)?;
        let buffer = buffer.read(cx);
        let original_text = buffer.diff_base()?.slice(hunk.diff_base_byte_range.clone());
        let buffer_snapshot = buffer.snapshot();
        let buffer_revert_changes = revert_changes.entry(buffer.remote_id()).or_default();
        if let Err(i) = buffer_revert_changes.binary_search_by(|probe| {
            probe
                .0
                .start
                .cmp(&hunk.buffer_range.start, &buffer_snapshot)
                .then(probe.0.end.cmp(&hunk.buffer_range.end, &buffer_snapshot))
        }) {
            buffer_revert_changes.insert(i, (hunk.buffer_range.clone(), original_text));
            Some(())
        } else {
            None
        }
    }

    pub fn reverse_lines(&mut self, _: &ReverseLines, cx: &mut ViewContext<Self>) {
        self.manipulate_lines(cx, |lines| lines.reverse())
    }

    pub fn shuffle_lines(&mut self, _: &ShuffleLines, cx: &mut ViewContext<Self>) {
        self.manipulate_lines(cx, |lines| lines.shuffle(&mut thread_rng()))
    }

    fn manipulate_lines<Fn>(&mut self, cx: &mut ViewContext<Self>, mut callback: Fn)
    where
        Fn: FnMut(&mut Vec<&str>),
    {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);

        let mut edits = Vec::new();

        let selections = self.selections.all::<Point>(cx);
        let mut selections = selections.iter().peekable();
        let mut contiguous_row_selections = Vec::new();
        let mut new_selections = Vec::new();
        let mut added_lines = 0;
        let mut removed_lines = 0;

        while let Some(selection) = selections.next() {
            let (start_row, end_row) = consume_contiguous_rows(
                &mut contiguous_row_selections,
                selection,
                &display_map,
                &mut selections,
            );

            let start_point = Point::new(start_row.0, 0);
            let end_point = Point::new(
                end_row.previous_row().0,
                buffer.line_len(end_row.previous_row()),
            );
            let text = buffer
                .text_for_range(start_point..end_point)
                .collect::<String>();

            let mut lines = text.split('\n').collect_vec();

            let lines_before = lines.len();
            callback(&mut lines);
            let lines_after = lines.len();

            edits.push((start_point..end_point, lines.join("\n")));

            // Selections must change based on added and removed line count
            let start_row =
                MultiBufferRow(start_point.row + added_lines as u32 - removed_lines as u32);
            let end_row = MultiBufferRow(start_row.0 + lines_after.saturating_sub(1) as u32);
            new_selections.push(Selection {
                id: selection.id,
                start: start_row,
                end: end_row,
                goal: SelectionGoal::None,
                reversed: selection.reversed,
            });

            if lines_after > lines_before {
                added_lines += lines_after - lines_before;
            } else if lines_before > lines_after {
                removed_lines += lines_before - lines_after;
            }
        }

        self.transact(cx, |this, cx| {
            let buffer = this.buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
                buffer.snapshot(cx)
            });

            // Recalculate offsets on newly edited buffer
            let new_selections = new_selections
                .iter()
                .map(|s| {
                    let start_point = Point::new(s.start.0, 0);
                    let end_point = Point::new(s.end.0, buffer.line_len(s.end));
                    Selection {
                        id: s.id,
                        start: buffer.point_to_offset(start_point),
                        end: buffer.point_to_offset(end_point),
                        goal: s.goal,
                        reversed: s.reversed,
                    }
                })
                .collect();

            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(new_selections);
            });

            this.request_autoscroll(Autoscroll::fit(), cx);
        });
    }

    pub fn convert_to_upper_case(&mut self, _: &ConvertToUpperCase, cx: &mut ViewContext<Self>) {
        self.manipulate_text(cx, |text| text.to_uppercase())
    }

    pub fn convert_to_lower_case(&mut self, _: &ConvertToLowerCase, cx: &mut ViewContext<Self>) {
        self.manipulate_text(cx, |text| text.to_lowercase())
    }

    pub fn convert_to_title_case(&mut self, _: &ConvertToTitleCase, cx: &mut ViewContext<Self>) {
        self.manipulate_text(cx, |text| {
            // Hack to get around the fact that to_case crate doesn't support '\n' as a word boundary
            // https://github.com/rutrum/convert-case/issues/16
            text.split('\n')
                .map(|line| line.to_case(Case::Title))
                .join("\n")
        })
    }

    pub fn convert_to_snake_case(&mut self, _: &ConvertToSnakeCase, cx: &mut ViewContext<Self>) {
        self.manipulate_text(cx, |text| text.to_case(Case::Snake))
    }

    pub fn convert_to_kebab_case(&mut self, _: &ConvertToKebabCase, cx: &mut ViewContext<Self>) {
        self.manipulate_text(cx, |text| text.to_case(Case::Kebab))
    }

    pub fn convert_to_upper_camel_case(
        &mut self,
        _: &ConvertToUpperCamelCase,
        cx: &mut ViewContext<Self>,
    ) {
        self.manipulate_text(cx, |text| {
            // Hack to get around the fact that to_case crate doesn't support '\n' as a word boundary
            // https://github.com/rutrum/convert-case/issues/16
            text.split('\n')
                .map(|line| line.to_case(Case::UpperCamel))
                .join("\n")
        })
    }

    pub fn convert_to_lower_camel_case(
        &mut self,
        _: &ConvertToLowerCamelCase,
        cx: &mut ViewContext<Self>,
    ) {
        self.manipulate_text(cx, |text| text.to_case(Case::Camel))
    }

    pub fn convert_to_opposite_case(
        &mut self,
        _: &ConvertToOppositeCase,
        cx: &mut ViewContext<Self>,
    ) {
        self.manipulate_text(cx, |text| {
            text.chars()
                .fold(String::with_capacity(text.len()), |mut t, c| {
                    if c.is_uppercase() {
                        t.extend(c.to_lowercase());
                    } else {
                        t.extend(c.to_uppercase());
                    }
                    t
                })
        })
    }

    fn manipulate_text<Fn>(&mut self, cx: &mut ViewContext<Self>, mut callback: Fn)
    where
        Fn: FnMut(&str) -> String,
    {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);

        let mut new_selections = Vec::new();
        let mut edits = Vec::new();
        let mut selection_adjustment = 0i32;

        for selection in self.selections.all::<usize>(cx) {
            let selection_is_empty = selection.is_empty();

            let (start, end) = if selection_is_empty {
                let word_range = movement::surrounding_word(
                    &display_map,
                    selection.start.to_display_point(&display_map),
                );
                let start = word_range.start.to_offset(&display_map, Bias::Left);
                let end = word_range.end.to_offset(&display_map, Bias::Left);
                (start, end)
            } else {
                (selection.start, selection.end)
            };

            let text = buffer.text_for_range(start..end).collect::<String>();
            let old_length = text.len() as i32;
            let text = callback(&text);

            new_selections.push(Selection {
                start: (start as i32 - selection_adjustment) as usize,
                end: ((start + text.len()) as i32 - selection_adjustment) as usize,
                goal: SelectionGoal::None,
                ..selection
            });

            selection_adjustment += old_length - text.len() as i32;

            edits.push((start..end, text));
        }

        self.transact(cx, |this, cx| {
            this.buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            });

            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(new_selections);
            });

            this.request_autoscroll(Autoscroll::fit(), cx);
        });
    }

    pub fn duplicate_line(&mut self, upwards: bool, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;
        let selections = self.selections.all::<Point>(cx);

        let mut edits = Vec::new();
        let mut selections_iter = selections.iter().peekable();
        while let Some(selection) = selections_iter.next() {
            // Avoid duplicating the same lines twice.
            let mut rows = selection.spanned_rows(false, &display_map);

            while let Some(next_selection) = selections_iter.peek() {
                let next_rows = next_selection.spanned_rows(false, &display_map);
                if next_rows.start < rows.end {
                    rows.end = next_rows.end;
                    selections_iter.next().unwrap();
                } else {
                    break;
                }
            }

            // Copy the text from the selected row region and splice it either at the start
            // or end of the region.
            let start = Point::new(rows.start.0, 0);
            let end = Point::new(
                rows.end.previous_row().0,
                buffer.line_len(rows.end.previous_row()),
            );
            let text = buffer
                .text_for_range(start..end)
                .chain(Some("\n"))
                .collect::<String>();
            let insert_location = if upwards {
                Point::new(rows.end.0, 0)
            } else {
                start
            };
            edits.push((insert_location..insert_location, text));
        }

        self.transact(cx, |this, cx| {
            this.buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            });

            this.request_autoscroll(Autoscroll::fit(), cx);
        });
    }

    pub fn duplicate_line_up(&mut self, _: &DuplicateLineUp, cx: &mut ViewContext<Self>) {
        self.duplicate_line(true, cx);
    }

    pub fn duplicate_line_down(&mut self, _: &DuplicateLineDown, cx: &mut ViewContext<Self>) {
        self.duplicate_line(false, cx);
    }

    pub fn move_line_up(&mut self, _: &MoveLineUp, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);

        let mut edits = Vec::new();
        let mut unfold_ranges = Vec::new();
        let mut refold_ranges = Vec::new();

        let selections = self.selections.all::<Point>(cx);
        let mut selections = selections.iter().peekable();
        let mut contiguous_row_selections = Vec::new();
        let mut new_selections = Vec::new();

        while let Some(selection) = selections.next() {
            // Find all the selections that span a contiguous row range
            let (start_row, end_row) = consume_contiguous_rows(
                &mut contiguous_row_selections,
                selection,
                &display_map,
                &mut selections,
            );

            // Move the text spanned by the row range to be before the line preceding the row range
            if start_row.0 > 0 {
                let range_to_move = Point::new(
                    start_row.previous_row().0,
                    buffer.line_len(start_row.previous_row()),
                )
                    ..Point::new(
                        end_row.previous_row().0,
                        buffer.line_len(end_row.previous_row()),
                    );
                let insertion_point = display_map
                    .prev_line_boundary(Point::new(start_row.previous_row().0, 0))
                    .0;

                // Don't move lines across excerpts
                if buffer
                    .excerpt_boundaries_in_range((
                        Bound::Excluded(insertion_point),
                        Bound::Included(range_to_move.end),
                    ))
                    .next()
                    .is_none()
                {
                    let text = buffer
                        .text_for_range(range_to_move.clone())
                        .flat_map(|s| s.chars())
                        .skip(1)
                        .chain(['\n'])
                        .collect::<String>();

                    edits.push((
                        buffer.anchor_after(range_to_move.start)
                            ..buffer.anchor_before(range_to_move.end),
                        String::new(),
                    ));
                    let insertion_anchor = buffer.anchor_after(insertion_point);
                    edits.push((insertion_anchor..insertion_anchor, text));

                    let row_delta = range_to_move.start.row - insertion_point.row + 1;

                    // Move selections up
                    new_selections.extend(contiguous_row_selections.drain(..).map(
                        |mut selection| {
                            selection.start.row -= row_delta;
                            selection.end.row -= row_delta;
                            selection
                        },
                    ));

                    // Move folds up
                    unfold_ranges.push(range_to_move.clone());
                    for fold in display_map.folds_in_range(
                        buffer.anchor_before(range_to_move.start)
                            ..buffer.anchor_after(range_to_move.end),
                    ) {
                        let mut start = fold.range.start.to_point(&buffer);
                        let mut end = fold.range.end.to_point(&buffer);
                        start.row -= row_delta;
                        end.row -= row_delta;
                        refold_ranges.push((start..end, fold.placeholder.clone()));
                    }
                }
            }

            // If we didn't move line(s), preserve the existing selections
            new_selections.append(&mut contiguous_row_selections);
        }

        self.transact(cx, |this, cx| {
            this.unfold_ranges(unfold_ranges, true, true, cx);
            this.buffer.update(cx, |buffer, cx| {
                for (range, text) in edits {
                    buffer.edit([(range, text)], None, cx);
                }
            });
            this.fold_ranges(refold_ranges, true, cx);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(new_selections);
            })
        });
    }

    pub fn move_line_down(&mut self, _: &MoveLineDown, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);

        let mut edits = Vec::new();
        let mut unfold_ranges = Vec::new();
        let mut refold_ranges = Vec::new();

        let selections = self.selections.all::<Point>(cx);
        let mut selections = selections.iter().peekable();
        let mut contiguous_row_selections = Vec::new();
        let mut new_selections = Vec::new();

        while let Some(selection) = selections.next() {
            // Find all the selections that span a contiguous row range
            let (start_row, end_row) = consume_contiguous_rows(
                &mut contiguous_row_selections,
                selection,
                &display_map,
                &mut selections,
            );

            // Move the text spanned by the row range to be after the last line of the row range
            if end_row.0 <= buffer.max_point().row {
                let range_to_move =
                    MultiBufferPoint::new(start_row.0, 0)..MultiBufferPoint::new(end_row.0, 0);
                let insertion_point = display_map
                    .next_line_boundary(MultiBufferPoint::new(end_row.0, 0))
                    .0;

                // Don't move lines across excerpt boundaries
                if buffer
                    .excerpt_boundaries_in_range((
                        Bound::Excluded(range_to_move.start),
                        Bound::Included(insertion_point),
                    ))
                    .next()
                    .is_none()
                {
                    let mut text = String::from("\n");
                    text.extend(buffer.text_for_range(range_to_move.clone()));
                    text.pop(); // Drop trailing newline
                    edits.push((
                        buffer.anchor_after(range_to_move.start)
                            ..buffer.anchor_before(range_to_move.end),
                        String::new(),
                    ));
                    let insertion_anchor = buffer.anchor_after(insertion_point);
                    edits.push((insertion_anchor..insertion_anchor, text));

                    let row_delta = insertion_point.row - range_to_move.end.row + 1;

                    // Move selections down
                    new_selections.extend(contiguous_row_selections.drain(..).map(
                        |mut selection| {
                            selection.start.row += row_delta;
                            selection.end.row += row_delta;
                            selection
                        },
                    ));

                    // Move folds down
                    unfold_ranges.push(range_to_move.clone());
                    for fold in display_map.folds_in_range(
                        buffer.anchor_before(range_to_move.start)
                            ..buffer.anchor_after(range_to_move.end),
                    ) {
                        let mut start = fold.range.start.to_point(&buffer);
                        let mut end = fold.range.end.to_point(&buffer);
                        start.row += row_delta;
                        end.row += row_delta;
                        refold_ranges.push((start..end, fold.placeholder.clone()));
                    }
                }
            }

            // If we didn't move line(s), preserve the existing selections
            new_selections.append(&mut contiguous_row_selections);
        }

        self.transact(cx, |this, cx| {
            this.unfold_ranges(unfold_ranges, true, true, cx);
            this.buffer.update(cx, |buffer, cx| {
                for (range, text) in edits {
                    buffer.edit([(range, text)], None, cx);
                }
            });
            this.fold_ranges(refold_ranges, true, cx);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(new_selections));
        });
    }

    pub fn transpose(&mut self, _: &Transpose, cx: &mut ViewContext<Self>) {
        let text_layout_details = &self.text_layout_details(cx);
        self.transact(cx, |this, cx| {
            let edits = this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let mut edits: Vec<(Range<usize>, String)> = Default::default();
                let line_mode = s.line_mode;
                s.move_with(|display_map, selection| {
                    if !selection.is_empty() || line_mode {
                        return;
                    }

                    let mut head = selection.head();
                    let mut transpose_offset = head.to_offset(display_map, Bias::Right);
                    if head.column() == display_map.line_len(head.row()) {
                        transpose_offset = display_map
                            .buffer_snapshot
                            .clip_offset(transpose_offset.saturating_sub(1), Bias::Left);
                    }

                    if transpose_offset == 0 {
                        return;
                    }

                    *head.column_mut() += 1;
                    head = display_map.clip_point(head, Bias::Right);
                    let goal = SelectionGoal::HorizontalPosition(
                        display_map
                            .x_for_display_point(head, &text_layout_details)
                            .into(),
                    );
                    selection.collapse_to(head, goal);

                    let transpose_start = display_map
                        .buffer_snapshot
                        .clip_offset(transpose_offset.saturating_sub(1), Bias::Left);
                    if edits.last().map_or(true, |e| e.0.end <= transpose_start) {
                        let transpose_end = display_map
                            .buffer_snapshot
                            .clip_offset(transpose_offset + 1, Bias::Right);
                        if let Some(ch) =
                            display_map.buffer_snapshot.chars_at(transpose_start).next()
                        {
                            edits.push((transpose_start..transpose_offset, String::new()));
                            edits.push((transpose_end..transpose_end, ch.to_string()));
                        }
                    }
                });
                edits
            });
            this.buffer
                .update(cx, |buffer, cx| buffer.edit(edits, None, cx));
            let selections = this.selections.all::<usize>(cx);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(selections);
            });
        });
    }

    pub fn cut(&mut self, _: &Cut, cx: &mut ViewContext<Self>) {
        let mut text = String::new();
        let buffer = self.buffer.read(cx).snapshot(cx);
        let mut selections = self.selections.all::<Point>(cx);
        let mut clipboard_selections = Vec::with_capacity(selections.len());
        {
            let max_point = buffer.max_point();
            let mut is_first = true;
            for selection in &mut selections {
                let is_entire_line = selection.is_empty() || self.selections.line_mode;
                if is_entire_line {
                    selection.start = Point::new(selection.start.row, 0);
                    selection.end = cmp::min(max_point, Point::new(selection.end.row + 1, 0));
                    selection.goal = SelectionGoal::None;
                }
                if is_first {
                    is_first = false;
                } else {
                    text += "\n";
                }
                let mut len = 0;
                for chunk in buffer.text_for_range(selection.start..selection.end) {
                    text.push_str(chunk);
                    len += chunk.len();
                }
                clipboard_selections.push(ClipboardSelection {
                    len,
                    is_entire_line,
                    first_line_indent: buffer
                        .indent_size_for_line(MultiBufferRow(selection.start.row))
                        .len,
                });
            }
        }

        self.transact(cx, |this, cx| {
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(selections);
            });
            this.insert("", cx);
            cx.write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
        });
    }

    pub fn copy(&mut self, _: &Copy, cx: &mut ViewContext<Self>) {
        let selections = self.selections.all::<Point>(cx);
        let buffer = self.buffer.read(cx).read(cx);
        let mut text = String::new();

        let mut clipboard_selections = Vec::with_capacity(selections.len());
        {
            let max_point = buffer.max_point();
            let mut is_first = true;
            for selection in selections.iter() {
                let mut start = selection.start;
                let mut end = selection.end;
                let is_entire_line = selection.is_empty() || self.selections.line_mode;
                if is_entire_line {
                    start = Point::new(start.row, 0);
                    end = cmp::min(max_point, Point::new(end.row + 1, 0));
                }
                if is_first {
                    is_first = false;
                } else {
                    text += "\n";
                }
                let mut len = 0;
                for chunk in buffer.text_for_range(start..end) {
                    text.push_str(chunk);
                    len += chunk.len();
                }
                clipboard_selections.push(ClipboardSelection {
                    len,
                    is_entire_line,
                    first_line_indent: buffer.indent_size_for_line(MultiBufferRow(start.row)).len,
                });
            }
        }

        cx.write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
    }

    pub fn do_paste(
        &mut self,
        text: &String,
        clipboard_selections: Option<Vec<ClipboardSelection>>,
        handle_entire_lines: bool,
        cx: &mut ViewContext<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }

        let clipboard_text = Cow::Borrowed(text);

        self.transact(cx, |this, cx| {
            if let Some(mut clipboard_selections) = clipboard_selections {
                let old_selections = this.selections.all::<usize>(cx);
                let all_selections_were_entire_line =
                    clipboard_selections.iter().all(|s| s.is_entire_line);
                let first_selection_indent_column =
                    clipboard_selections.first().map(|s| s.first_line_indent);
                if clipboard_selections.len() != old_selections.len() {
                    clipboard_selections.drain(..);
                }

                this.buffer.update(cx, |buffer, cx| {
                    let snapshot = buffer.read(cx);
                    let mut start_offset = 0;
                    let mut edits = Vec::new();
                    let mut original_indent_columns = Vec::new();
                    for (ix, selection) in old_selections.iter().enumerate() {
                        let to_insert;
                        let entire_line;
                        let original_indent_column;
                        if let Some(clipboard_selection) = clipboard_selections.get(ix) {
                            let end_offset = start_offset + clipboard_selection.len;
                            to_insert = &clipboard_text[start_offset..end_offset];
                            entire_line = clipboard_selection.is_entire_line;
                            start_offset = end_offset + 1;
                            original_indent_column = Some(clipboard_selection.first_line_indent);
                        } else {
                            to_insert = clipboard_text.as_str();
                            entire_line = all_selections_were_entire_line;
                            original_indent_column = first_selection_indent_column
                        }

                        // If the corresponding selection was empty when this slice of the
                        // clipboard text was written, then the entire line containing the
                        // selection was copied. If this selection is also currently empty,
                        // then paste the line before the current line of the buffer.
                        let range = if selection.is_empty() && handle_entire_lines && entire_line {
                            let column = selection.start.to_point(&snapshot).column as usize;
                            let line_start = selection.start - column;
                            line_start..line_start
                        } else {
                            selection.range()
                        };

                        edits.push((range, to_insert));
                        original_indent_columns.extend(original_indent_column);
                    }
                    drop(snapshot);

                    buffer.edit(
                        edits,
                        Some(AutoindentMode::Block {
                            original_indent_columns,
                        }),
                        cx,
                    );
                });

                let selections = this.selections.all::<usize>(cx);
                this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(selections));
            } else {
                this.insert(&clipboard_text, cx);
            }
        });
    }

    pub fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
        if let Some(item) = cx.read_from_clipboard() {
            self.do_paste(
                item.text(),
                item.metadata::<Vec<ClipboardSelection>>(),
                true,
                cx,
            )
        };
    }

    pub fn undo(&mut self, _: &Undo, cx: &mut ViewContext<Self>) {
        if self.read_only(cx) {
            return;
        }

        if let Some(transaction_id) = self.buffer.update(cx, |buffer, cx| buffer.undo(cx)) {
            if let Some((selections, _)) =
                self.selection_history.transaction(transaction_id).cloned()
            {
                self.change_selections(None, cx, |s| {
                    s.select_anchors(selections.to_vec());
                });
            }
            self.request_autoscroll(Autoscroll::fit(), cx);
            self.unmark_text(cx);
            self.refresh_inline_completion(true, cx);
            cx.emit(EditorEvent::Edited { transaction_id });
            cx.emit(EditorEvent::TransactionUndone { transaction_id });
        }
    }

    pub fn redo(&mut self, _: &Redo, cx: &mut ViewContext<Self>) {
        if self.read_only(cx) {
            return;
        }

        if let Some(transaction_id) = self.buffer.update(cx, |buffer, cx| buffer.redo(cx)) {
            if let Some((_, Some(selections))) =
                self.selection_history.transaction(transaction_id).cloned()
            {
                self.change_selections(None, cx, |s| {
                    s.select_anchors(selections.to_vec());
                });
            }
            self.request_autoscroll(Autoscroll::fit(), cx);
            self.unmark_text(cx);
            self.refresh_inline_completion(true, cx);
            cx.emit(EditorEvent::Edited { transaction_id });
        }
    }

    pub fn finalize_last_transaction(&mut self, cx: &mut ViewContext<Self>) {
        self.buffer
            .update(cx, |buffer, cx| buffer.finalize_last_transaction(cx));
    }

    pub fn group_until_transaction(&mut self, tx_id: TransactionId, cx: &mut ViewContext<Self>) {
        self.buffer
            .update(cx, |buffer, cx| buffer.group_until_transaction(tx_id, cx));
    }

    pub fn move_left(&mut self, _: &MoveLeft, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                let cursor = if selection.is_empty() && !line_mode {
                    movement::left(map, selection.start)
                } else {
                    selection.start
                };
                selection.collapse_to(cursor, SelectionGoal::None);
            });
        })
    }

    pub fn select_left(&mut self, _: &SelectLeft, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| (movement::left(map, head), SelectionGoal::None));
        })
    }

    pub fn move_right(&mut self, _: &MoveRight, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                let cursor = if selection.is_empty() && !line_mode {
                    movement::right(map, selection.end)
                } else {
                    selection.end
                };
                selection.collapse_to(cursor, SelectionGoal::None)
            });
        })
    }

    pub fn select_right(&mut self, _: &SelectRight, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| (movement::right(map, head), SelectionGoal::None));
        })
    }

    pub fn move_up(&mut self, _: &MoveUp, cx: &mut ViewContext<Self>) {
        if self.take_rename(true, cx).is_some() {
            return;
        }

        if matches!(self.mode, EditorMode::SingleLine { .. }) {
            cx.propagate();
            return;
        }

        let text_layout_details = &self.text_layout_details(cx);
        let selection_count = self.selections.count();
        let first_selection = self.selections.first_anchor();

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                if !selection.is_empty() && !line_mode {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::up(
                    map,
                    selection.start,
                    selection.goal,
                    false,
                    &text_layout_details,
                );
                selection.collapse_to(cursor, goal);
            });
        });

        if selection_count == 1 && first_selection.range() == self.selections.first_anchor().range()
        {
            cx.propagate();
        }
    }

    pub fn move_up_by_lines(&mut self, action: &MoveUpByLines, cx: &mut ViewContext<Self>) {
        if self.take_rename(true, cx).is_some() {
            return;
        }

        if matches!(self.mode, EditorMode::SingleLine { .. }) {
            cx.propagate();
            return;
        }

        let text_layout_details = &self.text_layout_details(cx);

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                if !selection.is_empty() && !line_mode {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::up_by_rows(
                    map,
                    selection.start,
                    action.lines,
                    selection.goal,
                    false,
                    &text_layout_details,
                );
                selection.collapse_to(cursor, goal);
            });
        })
    }

    pub fn move_down_by_lines(&mut self, action: &MoveDownByLines, cx: &mut ViewContext<Self>) {
        if self.take_rename(true, cx).is_some() {
            return;
        }

        if matches!(self.mode, EditorMode::SingleLine { .. }) {
            cx.propagate();
            return;
        }

        let text_layout_details = &self.text_layout_details(cx);

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                if !selection.is_empty() && !line_mode {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::down_by_rows(
                    map,
                    selection.start,
                    action.lines,
                    selection.goal,
                    false,
                    &text_layout_details,
                );
                selection.collapse_to(cursor, goal);
            });
        })
    }

    pub fn select_down_by_lines(&mut self, action: &SelectDownByLines, cx: &mut ViewContext<Self>) {
        let text_layout_details = &self.text_layout_details(cx);
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, goal| {
                movement::down_by_rows(map, head, action.lines, goal, false, &text_layout_details)
            })
        })
    }

    pub fn select_up_by_lines(&mut self, action: &SelectUpByLines, cx: &mut ViewContext<Self>) {
        let text_layout_details = &self.text_layout_details(cx);
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, goal| {
                movement::up_by_rows(map, head, action.lines, goal, false, &text_layout_details)
            })
        })
    }

    pub fn select_page_up(&mut self, _: &SelectPageUp, cx: &mut ViewContext<Self>) {
        let Some(row_count) = self.visible_row_count() else {
            return;
        };

        let text_layout_details = &self.text_layout_details(cx);

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, goal| {
                movement::up_by_rows(map, head, row_count, goal, false, &text_layout_details)
            })
        })
    }

    pub fn move_page_up(&mut self, action: &MovePageUp, cx: &mut ViewContext<Self>) {
        if self.take_rename(true, cx).is_some() {
            return;
        }

        if self
            .context_menu
            .write()
            .as_mut()
            .map(|menu| menu.select_first(self.project.as_ref(), cx))
            .unwrap_or(false)
        {
            return;
        }

        if matches!(self.mode, EditorMode::SingleLine { .. }) {
            cx.propagate();
            return;
        }

        let Some(row_count) = self.visible_row_count() else {
            return;
        };

        let autoscroll = if action.center_cursor {
            Autoscroll::center()
        } else {
            Autoscroll::fit()
        };

        let text_layout_details = &self.text_layout_details(cx);

        self.change_selections(Some(autoscroll), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                if !selection.is_empty() && !line_mode {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::up_by_rows(
                    map,
                    selection.end,
                    row_count,
                    selection.goal,
                    false,
                    &text_layout_details,
                );
                selection.collapse_to(cursor, goal);
            });
        });
    }

    pub fn select_up(&mut self, _: &SelectUp, cx: &mut ViewContext<Self>) {
        let text_layout_details = &self.text_layout_details(cx);
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, goal| {
                movement::up(map, head, goal, false, &text_layout_details)
            })
        })
    }

    pub fn move_down(&mut self, _: &MoveDown, cx: &mut ViewContext<Self>) {
        self.take_rename(true, cx);

        if matches!(self.mode, EditorMode::SingleLine { .. }) {
            cx.propagate();
            return;
        }

        let text_layout_details = &self.text_layout_details(cx);
        let selection_count = self.selections.count();
        let first_selection = self.selections.first_anchor();

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                if !selection.is_empty() && !line_mode {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::down(
                    map,
                    selection.end,
                    selection.goal,
                    false,
                    &text_layout_details,
                );
                selection.collapse_to(cursor, goal);
            });
        });

        if selection_count == 1 && first_selection.range() == self.selections.first_anchor().range()
        {
            cx.propagate();
        }
    }

    pub fn select_page_down(&mut self, _: &SelectPageDown, cx: &mut ViewContext<Self>) {
        let Some(row_count) = self.visible_row_count() else {
            return;
        };

        let text_layout_details = &self.text_layout_details(cx);

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, goal| {
                movement::down_by_rows(map, head, row_count, goal, false, &text_layout_details)
            })
        })
    }

    pub fn move_page_down(&mut self, action: &MovePageDown, cx: &mut ViewContext<Self>) {
        if self.take_rename(true, cx).is_some() {
            return;
        }

        if self
            .context_menu
            .write()
            .as_mut()
            .map(|menu| menu.select_last(self.project.as_ref(), cx))
            .unwrap_or(false)
        {
            return;
        }

        if matches!(self.mode, EditorMode::SingleLine { .. }) {
            cx.propagate();
            return;
        }

        let Some(row_count) = self.visible_row_count() else {
            return;
        };

        let autoscroll = if action.center_cursor {
            Autoscroll::center()
        } else {
            Autoscroll::fit()
        };

        let text_layout_details = &self.text_layout_details(cx);
        self.change_selections(Some(autoscroll), cx, |s| {
            let line_mode = s.line_mode;
            s.move_with(|map, selection| {
                if !selection.is_empty() && !line_mode {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::down_by_rows(
                    map,
                    selection.end,
                    row_count,
                    selection.goal,
                    false,
                    &text_layout_details,
                );
                selection.collapse_to(cursor, goal);
            });
        });
    }

    pub fn select_down(&mut self, _: &SelectDown, cx: &mut ViewContext<Self>) {
        let text_layout_details = &self.text_layout_details(cx);
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, goal| {
                movement::down(map, head, goal, false, &text_layout_details)
            })
        });
    }

    pub fn context_menu_first(&mut self, _: &ContextMenuFirst, cx: &mut ViewContext<Self>) {
        if let Some(context_menu) = self.context_menu.write().as_mut() {
            context_menu.select_first(self.project.as_ref(), cx);
        }
    }

    pub fn context_menu_prev(&mut self, _: &ContextMenuPrev, cx: &mut ViewContext<Self>) {
        if let Some(context_menu) = self.context_menu.write().as_mut() {
            context_menu.select_prev(self.project.as_ref(), cx);
        }
    }

    pub fn context_menu_next(&mut self, _: &ContextMenuNext, cx: &mut ViewContext<Self>) {
        if let Some(context_menu) = self.context_menu.write().as_mut() {
            context_menu.select_next(self.project.as_ref(), cx);
        }
    }

    pub fn context_menu_last(&mut self, _: &ContextMenuLast, cx: &mut ViewContext<Self>) {
        if let Some(context_menu) = self.context_menu.write().as_mut() {
            context_menu.select_last(self.project.as_ref(), cx);
        }
    }

    pub fn move_to_previous_word_start(
        &mut self,
        _: &MoveToPreviousWordStart,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_cursors_with(|map, head, _| {
                (
                    movement::previous_word_start(map, head),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn move_to_previous_subword_start(
        &mut self,
        _: &MoveToPreviousSubwordStart,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_cursors_with(|map, head, _| {
                (
                    movement::previous_subword_start(map, head),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_previous_word_start(
        &mut self,
        _: &SelectToPreviousWordStart,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (
                    movement::previous_word_start(map, head),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_previous_subword_start(
        &mut self,
        _: &SelectToPreviousSubwordStart,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (
                    movement::previous_subword_start(map, head),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn delete_to_previous_word_start(
        &mut self,
        _: &DeleteToPreviousWordStart,
        cx: &mut ViewContext<Self>,
    ) {
        self.transact(cx, |this, cx| {
            this.select_autoclose_pair(cx);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let line_mode = s.line_mode;
                s.move_with(|map, selection| {
                    if selection.is_empty() && !line_mode {
                        let cursor = movement::previous_word_start(map, selection.head());
                        selection.set_head(cursor, SelectionGoal::None);
                    }
                });
            });
            this.insert("", cx);
        });
    }

    pub fn delete_to_previous_subword_start(
        &mut self,
        _: &DeleteToPreviousSubwordStart,
        cx: &mut ViewContext<Self>,
    ) {
        self.transact(cx, |this, cx| {
            this.select_autoclose_pair(cx);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let line_mode = s.line_mode;
                s.move_with(|map, selection| {
                    if selection.is_empty() && !line_mode {
                        let cursor = movement::previous_subword_start(map, selection.head());
                        selection.set_head(cursor, SelectionGoal::None);
                    }
                });
            });
            this.insert("", cx);
        });
    }

    pub fn move_to_next_word_end(&mut self, _: &MoveToNextWordEnd, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_cursors_with(|map, head, _| {
                (movement::next_word_end(map, head), SelectionGoal::None)
            });
        })
    }

    pub fn move_to_next_subword_end(
        &mut self,
        _: &MoveToNextSubwordEnd,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_cursors_with(|map, head, _| {
                (movement::next_subword_end(map, head), SelectionGoal::None)
            });
        })
    }

    pub fn select_to_next_word_end(&mut self, _: &SelectToNextWordEnd, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (movement::next_word_end(map, head), SelectionGoal::None)
            });
        })
    }

    pub fn select_to_next_subword_end(
        &mut self,
        _: &SelectToNextSubwordEnd,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (movement::next_subword_end(map, head), SelectionGoal::None)
            });
        })
    }

    pub fn delete_to_next_word_end(&mut self, _: &DeleteToNextWordEnd, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let line_mode = s.line_mode;
                s.move_with(|map, selection| {
                    if selection.is_empty() && !line_mode {
                        let cursor = movement::next_word_end(map, selection.head());
                        selection.set_head(cursor, SelectionGoal::None);
                    }
                });
            });
            this.insert("", cx);
        });
    }

    pub fn delete_to_next_subword_end(
        &mut self,
        _: &DeleteToNextSubwordEnd,
        cx: &mut ViewContext<Self>,
    ) {
        self.transact(cx, |this, cx| {
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    if selection.is_empty() {
                        let cursor = movement::next_subword_end(map, selection.head());
                        selection.set_head(cursor, SelectionGoal::None);
                    }
                });
            });
            this.insert("", cx);
        });
    }

    pub fn move_to_beginning_of_line(
        &mut self,
        action: &MoveToBeginningOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_cursors_with(|map, head, _| {
                (
                    movement::indented_line_beginning(map, head, action.stop_at_soft_wraps),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_beginning_of_line(
        &mut self,
        action: &SelectToBeginningOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (
                    movement::indented_line_beginning(map, head, action.stop_at_soft_wraps),
                    SelectionGoal::None,
                )
            });
        });
    }

    pub fn delete_to_beginning_of_line(
        &mut self,
        _: &DeleteToBeginningOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        self.transact(cx, |this, cx| {
            this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|_, selection| {
                    selection.reversed = true;
                });
            });

            this.select_to_beginning_of_line(
                &SelectToBeginningOfLine {
                    stop_at_soft_wraps: false,
                },
                cx,
            );
            this.backspace(&Backspace, cx);
        });
    }

    pub fn move_to_end_of_line(&mut self, action: &MoveToEndOfLine, cx: &mut ViewContext<Self>) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_cursors_with(|map, head, _| {
                (
                    movement::line_end(map, head, action.stop_at_soft_wraps),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_end_of_line(
        &mut self,
        action: &SelectToEndOfLine,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (
                    movement::line_end(map, head, action.stop_at_soft_wraps),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn delete_to_end_of_line(&mut self, _: &DeleteToEndOfLine, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.select_to_end_of_line(
                &SelectToEndOfLine {
                    stop_at_soft_wraps: false,
                },
                cx,
            );
            this.delete(&Delete, cx);
        });
    }

    pub fn cut_to_end_of_line(&mut self, _: &CutToEndOfLine, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.select_to_end_of_line(
                &SelectToEndOfLine {
                    stop_at_soft_wraps: false,
                },
                cx,
            );
            this.cut(&Cut, cx);
        });
    }

    pub fn move_to_start_of_paragraph(
        &mut self,
        _: &MoveToStartOfParagraph,
        cx: &mut ViewContext<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine { .. }) {
            cx.propagate();
            return;
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_with(|map, selection| {
                selection.collapse_to(
                    movement::start_of_paragraph(map, selection.head(), 1),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn move_to_end_of_paragraph(
        &mut self,
        _: &MoveToEndOfParagraph,
        cx: &mut ViewContext<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine { .. }) {
            cx.propagate();
            return;
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_with(|map, selection| {
                selection.collapse_to(
                    movement::end_of_paragraph(map, selection.head(), 1),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_start_of_paragraph(
        &mut self,
        _: &SelectToStartOfParagraph,
        cx: &mut ViewContext<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine { .. }) {
            cx.propagate();
            return;
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (
                    movement::start_of_paragraph(map, head, 1),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_end_of_paragraph(
        &mut self,
        _: &SelectToEndOfParagraph,
        cx: &mut ViewContext<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine { .. }) {
            cx.propagate();
            return;
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_heads_with(|map, head, _| {
                (
                    movement::end_of_paragraph(map, head, 1),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn move_to_beginning(&mut self, _: &MoveToBeginning, cx: &mut ViewContext<Self>) {
        if matches!(self.mode, EditorMode::SingleLine { .. }) {
            cx.propagate();
            return;
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select_ranges(vec![0..0]);
        });
    }

    pub fn select_to_beginning(&mut self, _: &SelectToBeginning, cx: &mut ViewContext<Self>) {
        let mut selection = self.selections.last::<Point>(cx);
        selection.set_head(Point::zero(), SelectionGoal::None);

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select(vec![selection]);
        });
    }

    pub fn move_to_end(&mut self, _: &MoveToEnd, cx: &mut ViewContext<Self>) {
        if matches!(self.mode, EditorMode::SingleLine { .. }) {
            cx.propagate();
            return;
        }

        let cursor = self.buffer.read(cx).read(cx).len();
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select_ranges(vec![cursor..cursor])
        });
    }

    pub fn set_nav_history(&mut self, nav_history: Option<ItemNavHistory>) {
        self.nav_history = nav_history;
    }

    pub fn nav_history(&self) -> Option<&ItemNavHistory> {
        self.nav_history.as_ref()
    }

    fn push_to_nav_history(
        &mut self,
        cursor_anchor: Anchor,
        new_position: Option<Point>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(nav_history) = self.nav_history.as_mut() {
            let buffer = self.buffer.read(cx).read(cx);
            let cursor_position = cursor_anchor.to_point(&buffer);
            let scroll_state = self.scroll_manager.anchor();
            let scroll_top_row = scroll_state.top_row(&buffer);
            drop(buffer);

            if let Some(new_position) = new_position {
                let row_delta = (new_position.row as i64 - cursor_position.row as i64).abs();
                if row_delta < MIN_NAVIGATION_HISTORY_ROW_DELTA {
                    return;
                }
            }

            nav_history.push(
                Some(NavigationData {
                    cursor_anchor,
                    cursor_position,
                    scroll_anchor: scroll_state,
                    scroll_top_row,
                }),
                cx,
            );
        }
    }

    pub fn select_to_end(&mut self, _: &SelectToEnd, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let mut selection = self.selections.first::<usize>(cx);
        selection.set_head(buffer.len(), SelectionGoal::None);
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select(vec![selection]);
        });
    }

    pub fn select_all(&mut self, _: &SelectAll, cx: &mut ViewContext<Self>) {
        let end = self.buffer.read(cx).read(cx).len();
        self.change_selections(None, cx, |s| {
            s.select_ranges(vec![0..end]);
        });
    }

    pub fn select_line(&mut self, _: &SelectLine, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections.all::<Point>(cx);
        let max_point = display_map.buffer_snapshot.max_point();
        for selection in &mut selections {
            let rows = selection.spanned_rows(true, &display_map);
            selection.start = Point::new(rows.start.0, 0);
            selection.end = cmp::min(max_point, Point::new(rows.end.0, 0));
            selection.reversed = false;
        }
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select(selections);
        });
    }

    pub fn split_selection_into_lines(
        &mut self,
        _: &SplitSelectionIntoLines,
        cx: &mut ViewContext<Self>,
    ) {
        let mut to_unfold = Vec::new();
        let mut new_selection_ranges = Vec::new();
        {
            let selections = self.selections.all::<Point>(cx);
            let buffer = self.buffer.read(cx).read(cx);
            for selection in selections {
                for row in selection.start.row..selection.end.row {
                    let cursor = Point::new(row, buffer.line_len(MultiBufferRow(row)));
                    new_selection_ranges.push(cursor..cursor);
                }
                new_selection_ranges.push(selection.end..selection.end);
                to_unfold.push(selection.start..selection.end);
            }
        }
        self.unfold_ranges(to_unfold, true, true, cx);
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select_ranges(new_selection_ranges);
        });
    }

    pub fn add_selection_above(&mut self, _: &AddSelectionAbove, cx: &mut ViewContext<Self>) {
        self.add_selection(true, cx);
    }

    pub fn add_selection_below(&mut self, _: &AddSelectionBelow, cx: &mut ViewContext<Self>) {
        self.add_selection(false, cx);
    }

    fn add_selection(&mut self, above: bool, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections.all::<Point>(cx);
        let text_layout_details = self.text_layout_details(cx);
        let mut state = self.add_selections_state.take().unwrap_or_else(|| {
            let oldest_selection = selections.iter().min_by_key(|s| s.id).unwrap().clone();
            let range = oldest_selection.display_range(&display_map).sorted();

            let start_x = display_map.x_for_display_point(range.start, &text_layout_details);
            let end_x = display_map.x_for_display_point(range.end, &text_layout_details);
            let positions = start_x.min(end_x)..start_x.max(end_x);

            selections.clear();
            let mut stack = Vec::new();
            for row in range.start.row().0..=range.end.row().0 {
                if let Some(selection) = self.selections.build_columnar_selection(
                    &display_map,
                    DisplayRow(row),
                    &positions,
                    oldest_selection.reversed,
                    &text_layout_details,
                ) {
                    stack.push(selection.id);
                    selections.push(selection);
                }
            }

            if above {
                stack.reverse();
            }

            AddSelectionsState { above, stack }
        });

        let last_added_selection = *state.stack.last().unwrap();
        let mut new_selections = Vec::new();
        if above == state.above {
            let end_row = if above {
                DisplayRow(0)
            } else {
                display_map.max_point().row()
            };

            'outer: for selection in selections {
                if selection.id == last_added_selection {
                    let range = selection.display_range(&display_map).sorted();
                    debug_assert_eq!(range.start.row(), range.end.row());
                    let mut row = range.start.row();
                    let positions =
                        if let SelectionGoal::HorizontalRange { start, end } = selection.goal {
                            px(start)..px(end)
                        } else {
                            let start_x =
                                display_map.x_for_display_point(range.start, &text_layout_details);
                            let end_x =
                                display_map.x_for_display_point(range.end, &text_layout_details);
                            start_x.min(end_x)..start_x.max(end_x)
                        };

                    while row != end_row {
                        if above {
                            row.0 -= 1;
                        } else {
                            row.0 += 1;
                        }

                        if let Some(new_selection) = self.selections.build_columnar_selection(
                            &display_map,
                            row,
                            &positions,
                            selection.reversed,
                            &text_layout_details,
                        ) {
                            state.stack.push(new_selection.id);
                            if above {
                                new_selections.push(new_selection);
                                new_selections.push(selection);
                            } else {
                                new_selections.push(selection);
                                new_selections.push(new_selection);
                            }

                            continue 'outer;
                        }
                    }
                }

                new_selections.push(selection);
            }
        } else {
            new_selections = selections;
            new_selections.retain(|s| s.id != last_added_selection);
            state.stack.pop();
        }

        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select(new_selections);
        });
        if state.stack.len() > 1 {
            self.add_selections_state = Some(state);
        }
    }

    pub fn select_next_match_internal(
        &mut self,
        display_map: &DisplaySnapshot,
        replace_newest: bool,
        autoscroll: Option<Autoscroll>,
        cx: &mut ViewContext<Self>,
    ) -> Result<()> {
        fn select_next_match_ranges(
            this: &mut Editor,
            range: Range<usize>,
            replace_newest: bool,
            auto_scroll: Option<Autoscroll>,
            cx: &mut ViewContext<Editor>,
        ) {
            this.unfold_ranges([range.clone()], false, true, cx);
            this.change_selections(auto_scroll, cx, |s| {
                if replace_newest {
                    s.delete(s.newest_anchor().id);
                }
                s.insert_range(range.clone());
            });
        }

        let buffer = &display_map.buffer_snapshot;
        let mut selections = self.selections.all::<usize>(cx);
        if let Some(mut select_next_state) = self.select_next_state.take() {
            let query = &select_next_state.query;
            if !select_next_state.done {
                let first_selection = selections.iter().min_by_key(|s| s.id).unwrap();
                let last_selection = selections.iter().max_by_key(|s| s.id).unwrap();
                let mut next_selected_range = None;

                let bytes_after_last_selection =
                    buffer.bytes_in_range(last_selection.end..buffer.len());
                let bytes_before_first_selection = buffer.bytes_in_range(0..first_selection.start);
                let query_matches = query
                    .stream_find_iter(bytes_after_last_selection)
                    .map(|result| (last_selection.end, result))
                    .chain(
                        query
                            .stream_find_iter(bytes_before_first_selection)
                            .map(|result| (0, result)),
                    );

                for (start_offset, query_match) in query_matches {
                    let query_match = query_match.unwrap(); // can only fail due to I/O
                    let offset_range =
                        start_offset + query_match.start()..start_offset + query_match.end();
                    let display_range = offset_range.start.to_display_point(&display_map)
                        ..offset_range.end.to_display_point(&display_map);

                    if !select_next_state.wordwise
                        || (!movement::is_inside_word(&display_map, display_range.start)
                            && !movement::is_inside_word(&display_map, display_range.end))
                    {
                        // TODO: This is n^2, because we might check all the selections
                        if !selections
                            .iter()
                            .any(|selection| selection.range().overlaps(&offset_range))
                        {
                            next_selected_range = Some(offset_range);
                            break;
                        }
                    }
                }

                if let Some(next_selected_range) = next_selected_range {
                    select_next_match_ranges(
                        self,
                        next_selected_range,
                        replace_newest,
                        autoscroll,
                        cx,
                    );
                } else {
                    select_next_state.done = true;
                }
            }

            self.select_next_state = Some(select_next_state);
        } else {
            let mut only_carets = true;
            let mut same_text_selected = true;
            let mut selected_text = None;

            let mut selections_iter = selections.iter().peekable();
            while let Some(selection) = selections_iter.next() {
                if selection.start != selection.end {
                    only_carets = false;
                }

                if same_text_selected {
                    if selected_text.is_none() {
                        selected_text =
                            Some(buffer.text_for_range(selection.range()).collect::<String>());
                    }

                    if let Some(next_selection) = selections_iter.peek() {
                        if next_selection.range().len() == selection.range().len() {
                            let next_selected_text = buffer
                                .text_for_range(next_selection.range())
                                .collect::<String>();
                            if Some(next_selected_text) != selected_text {
                                same_text_selected = false;
                                selected_text = None;
                            }
                        } else {
                            same_text_selected = false;
                            selected_text = None;
                        }
                    }
                }
            }

            if only_carets {
                for selection in &mut selections {
                    let word_range = movement::surrounding_word(
                        &display_map,
                        selection.start.to_display_point(&display_map),
                    );
                    selection.start = word_range.start.to_offset(&display_map, Bias::Left);
                    selection.end = word_range.end.to_offset(&display_map, Bias::Left);
                    selection.goal = SelectionGoal::None;
                    selection.reversed = false;
                    select_next_match_ranges(
                        self,
                        selection.start..selection.end,
                        replace_newest,
                        autoscroll,
                        cx,
                    );
                }

                if selections.len() == 1 {
                    let selection = selections
                        .last()
                        .expect("ensured that there's only one selection");
                    let query = buffer
                        .text_for_range(selection.start..selection.end)
                        .collect::<String>();
                    let is_empty = query.is_empty();
                    let select_state = SelectNextState {
                        query: AhoCorasick::new(&[query])?,
                        wordwise: true,
                        done: is_empty,
                    };
                    self.select_next_state = Some(select_state);
                } else {
                    self.select_next_state = None;
                }
            } else if let Some(selected_text) = selected_text {
                self.select_next_state = Some(SelectNextState {
                    query: AhoCorasick::new(&[selected_text])?,
                    wordwise: false,
                    done: false,
                });
                self.select_next_match_internal(display_map, replace_newest, autoscroll, cx)?;
            }
        }
        Ok(())
    }

    pub fn select_all_matches(
        &mut self,
        _action: &SelectAllMatches,
        cx: &mut ViewContext<Self>,
    ) -> Result<()> {
        self.push_to_selection_history();
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        self.select_next_match_internal(&display_map, false, None, cx)?;
        let Some(select_next_state) = self.select_next_state.as_mut() else {
            return Ok(());
        };
        if select_next_state.done {
            return Ok(());
        }

        let mut new_selections = self.selections.all::<usize>(cx);

        let buffer = &display_map.buffer_snapshot;
        let query_matches = select_next_state
            .query
            .stream_find_iter(buffer.bytes_in_range(0..buffer.len()));

        for query_match in query_matches {
            let query_match = query_match.unwrap(); // can only fail due to I/O
            let offset_range = query_match.start()..query_match.end();
            let display_range = offset_range.start.to_display_point(&display_map)
                ..offset_range.end.to_display_point(&display_map);

            if !select_next_state.wordwise
                || (!movement::is_inside_word(&display_map, display_range.start)
                    && !movement::is_inside_word(&display_map, display_range.end))
            {
                self.selections.change_with(cx, |selections| {
                    new_selections.push(Selection {
                        id: selections.new_selection_id(),
                        start: offset_range.start,
                        end: offset_range.end,
                        reversed: false,
                        goal: SelectionGoal::None,
                    });
                });
            }
        }

        new_selections.sort_by_key(|selection| selection.start);
        let mut ix = 0;
        while ix + 1 < new_selections.len() {
            let current_selection = &new_selections[ix];
            let next_selection = &new_selections[ix + 1];
            if current_selection.range().overlaps(&next_selection.range()) {
                if current_selection.id < next_selection.id {
                    new_selections.remove(ix + 1);
                } else {
                    new_selections.remove(ix);
                }
            } else {
                ix += 1;
            }
        }

        select_next_state.done = true;
        self.unfold_ranges(
            new_selections.iter().map(|selection| selection.range()),
            false,
            false,
            cx,
        );
        self.change_selections(Some(Autoscroll::fit()), cx, |selections| {
            selections.select(new_selections)
        });

        Ok(())
    }

    pub fn select_next(&mut self, action: &SelectNext, cx: &mut ViewContext<Self>) -> Result<()> {
        self.push_to_selection_history();
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        self.select_next_match_internal(
            &display_map,
            action.replace_newest,
            Some(Autoscroll::newest()),
            cx,
        )?;
        Ok(())
    }

    pub fn select_previous(
        &mut self,
        action: &SelectPrevious,
        cx: &mut ViewContext<Self>,
    ) -> Result<()> {
        self.push_to_selection_history();
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;
        let mut selections = self.selections.all::<usize>(cx);
        if let Some(mut select_prev_state) = self.select_prev_state.take() {
            let query = &select_prev_state.query;
            if !select_prev_state.done {
                let first_selection = selections.iter().min_by_key(|s| s.id).unwrap();
                let last_selection = selections.iter().max_by_key(|s| s.id).unwrap();
                let mut next_selected_range = None;
                // When we're iterating matches backwards, the oldest match will actually be the furthest one in the buffer.
                let bytes_before_last_selection =
                    buffer.reversed_bytes_in_range(0..last_selection.start);
                let bytes_after_first_selection =
                    buffer.reversed_bytes_in_range(first_selection.end..buffer.len());
                let query_matches = query
                    .stream_find_iter(bytes_before_last_selection)
                    .map(|result| (last_selection.start, result))
                    .chain(
                        query
                            .stream_find_iter(bytes_after_first_selection)
                            .map(|result| (buffer.len(), result)),
                    );
                for (end_offset, query_match) in query_matches {
                    let query_match = query_match.unwrap(); // can only fail due to I/O
                    let offset_range =
                        end_offset - query_match.end()..end_offset - query_match.start();
                    let display_range = offset_range.start.to_display_point(&display_map)
                        ..offset_range.end.to_display_point(&display_map);

                    if !select_prev_state.wordwise
                        || (!movement::is_inside_word(&display_map, display_range.start)
                            && !movement::is_inside_word(&display_map, display_range.end))
                    {
                        next_selected_range = Some(offset_range);
                        break;
                    }
                }

                if let Some(next_selected_range) = next_selected_range {
                    self.unfold_ranges([next_selected_range.clone()], false, true, cx);
                    self.change_selections(Some(Autoscroll::newest()), cx, |s| {
                        if action.replace_newest {
                            s.delete(s.newest_anchor().id);
                        }
                        s.insert_range(next_selected_range);
                    });
                } else {
                    select_prev_state.done = true;
                }
            }

            self.select_prev_state = Some(select_prev_state);
        } else {
            let mut only_carets = true;
            let mut same_text_selected = true;
            let mut selected_text = None;

            let mut selections_iter = selections.iter().peekable();
            while let Some(selection) = selections_iter.next() {
                if selection.start != selection.end {
                    only_carets = false;
                }

                if same_text_selected {
                    if selected_text.is_none() {
                        selected_text =
                            Some(buffer.text_for_range(selection.range()).collect::<String>());
                    }

                    if let Some(next_selection) = selections_iter.peek() {
                        if next_selection.range().len() == selection.range().len() {
                            let next_selected_text = buffer
                                .text_for_range(next_selection.range())
                                .collect::<String>();
                            if Some(next_selected_text) != selected_text {
                                same_text_selected = false;
                                selected_text = None;
                            }
                        } else {
                            same_text_selected = false;
                            selected_text = None;
                        }
                    }
                }
            }

            if only_carets {
                for selection in &mut selections {
                    let word_range = movement::surrounding_word(
                        &display_map,
                        selection.start.to_display_point(&display_map),
                    );
                    selection.start = word_range.start.to_offset(&display_map, Bias::Left);
                    selection.end = word_range.end.to_offset(&display_map, Bias::Left);
                    selection.goal = SelectionGoal::None;
                    selection.reversed = false;
                }
                if selections.len() == 1 {
                    let selection = selections
                        .last()
                        .expect("ensured that there's only one selection");
                    let query = buffer
                        .text_for_range(selection.start..selection.end)
                        .collect::<String>();
                    let is_empty = query.is_empty();
                    let select_state = SelectNextState {
                        query: AhoCorasick::new(&[query.chars().rev().collect::<String>()])?,
                        wordwise: true,
                        done: is_empty,
                    };
                    self.select_prev_state = Some(select_state);
                } else {
                    self.select_prev_state = None;
                }

                self.unfold_ranges(
                    selections.iter().map(|s| s.range()).collect::<Vec<_>>(),
                    false,
                    true,
                    cx,
                );
                self.change_selections(Some(Autoscroll::newest()), cx, |s| {
                    s.select(selections);
                });
            } else if let Some(selected_text) = selected_text {
                self.select_prev_state = Some(SelectNextState {
                    query: AhoCorasick::new(&[selected_text.chars().rev().collect::<String>()])?,
                    wordwise: false,
                    done: false,
                });
                self.select_previous(action, cx)?;
            }
        }
        Ok(())
    }

    pub fn toggle_comments(&mut self, action: &ToggleComments, cx: &mut ViewContext<Self>) {
        let text_layout_details = &self.text_layout_details(cx);
        self.transact(cx, |this, cx| {
            let mut selections = this.selections.all::<MultiBufferPoint>(cx);
            let mut edits = Vec::new();
            let mut selection_edit_ranges = Vec::new();
            let mut last_toggled_row = None;
            let snapshot = this.buffer.read(cx).read(cx);
            let empty_str: Arc<str> = Arc::default();
            let mut suffixes_inserted = Vec::new();

            fn comment_prefix_range(
                snapshot: &MultiBufferSnapshot,
                row: MultiBufferRow,
                comment_prefix: &str,
                comment_prefix_whitespace: &str,
            ) -> Range<Point> {
                let start = Point::new(row.0, snapshot.indent_size_for_line(row).len);

                let mut line_bytes = snapshot
                    .bytes_in_range(start..snapshot.max_point())
                    .flatten()
                    .copied();

                // If this line currently begins with the line comment prefix, then record
                // the range containing the prefix.
                if line_bytes
                    .by_ref()
                    .take(comment_prefix.len())
                    .eq(comment_prefix.bytes())
                {
                    // Include any whitespace that matches the comment prefix.
                    let matching_whitespace_len = line_bytes
                        .zip(comment_prefix_whitespace.bytes())
                        .take_while(|(a, b)| a == b)
                        .count() as u32;
                    let end = Point::new(
                        start.row,
                        start.column + comment_prefix.len() as u32 + matching_whitespace_len,
                    );
                    start..end
                } else {
                    start..start
                }
            }

            fn comment_suffix_range(
                snapshot: &MultiBufferSnapshot,
                row: MultiBufferRow,
                comment_suffix: &str,
                comment_suffix_has_leading_space: bool,
            ) -> Range<Point> {
                let end = Point::new(row.0, snapshot.line_len(row));
                let suffix_start_column = end.column.saturating_sub(comment_suffix.len() as u32);

                let mut line_end_bytes = snapshot
                    .bytes_in_range(Point::new(end.row, suffix_start_column.saturating_sub(1))..end)
                    .flatten()
                    .copied();

                let leading_space_len = if suffix_start_column > 0
                    && line_end_bytes.next() == Some(b' ')
                    && comment_suffix_has_leading_space
                {
                    1
                } else {
                    0
                };

                // If this line currently begins with the line comment prefix, then record
                // the range containing the prefix.
                if line_end_bytes.by_ref().eq(comment_suffix.bytes()) {
                    let start = Point::new(end.row, suffix_start_column - leading_space_len);
                    start..end
                } else {
                    end..end
                }
            }

            // TODO: Handle selections that cross excerpts
            for selection in &mut selections {
                let start_column = snapshot
                    .indent_size_for_line(MultiBufferRow(selection.start.row))
                    .len;
                let language = if let Some(language) =
                    snapshot.language_scope_at(Point::new(selection.start.row, start_column))
                {
                    language
                } else {
                    continue;
                };

                selection_edit_ranges.clear();

                // If multiple selections contain a given row, avoid processing that
                // row more than once.
                let mut start_row = MultiBufferRow(selection.start.row);
                if last_toggled_row == Some(start_row) {
                    start_row = start_row.next_row();
                }
                let end_row =
                    if selection.end.row > selection.start.row && selection.end.column == 0 {
                        MultiBufferRow(selection.end.row - 1)
                    } else {
                        MultiBufferRow(selection.end.row)
                    };
                last_toggled_row = Some(end_row);

                if start_row > end_row {
                    continue;
                }

                // If the language has line comments, toggle those.
                let full_comment_prefixes = language.line_comment_prefixes();
                if !full_comment_prefixes.is_empty() {
                    let first_prefix = full_comment_prefixes
                        .first()
                        .expect("prefixes is non-empty");
                    let prefix_trimmed_lengths = full_comment_prefixes
                        .iter()
                        .map(|p| p.trim_end_matches(' ').len())
                        .collect::<SmallVec<[usize; 4]>>();

                    let mut all_selection_lines_are_comments = true;

                    for row in start_row.0..=end_row.0 {
                        let row = MultiBufferRow(row);
                        if start_row < end_row && snapshot.is_line_blank(row) {
                            continue;
                        }

                        let prefix_range = full_comment_prefixes
                            .iter()
                            .zip(prefix_trimmed_lengths.iter().copied())
                            .map(|(prefix, trimmed_prefix_len)| {
                                comment_prefix_range(
                                    snapshot.deref(),
                                    row,
                                    &prefix[..trimmed_prefix_len],
                                    &prefix[trimmed_prefix_len..],
                                )
                            })
                            .max_by_key(|range| range.end.column - range.start.column)
                            .expect("prefixes is non-empty");

                        if prefix_range.is_empty() {
                            all_selection_lines_are_comments = false;
                        }

                        selection_edit_ranges.push(prefix_range);
                    }

                    if all_selection_lines_are_comments {
                        edits.extend(
                            selection_edit_ranges
                                .iter()
                                .cloned()
                                .map(|range| (range, empty_str.clone())),
                        );
                    } else {
                        let min_column = selection_edit_ranges
                            .iter()
                            .map(|range| range.start.column)
                            .min()
                            .unwrap_or(0);
                        edits.extend(selection_edit_ranges.iter().map(|range| {
                            let position = Point::new(range.start.row, min_column);
                            (position..position, first_prefix.clone())
                        }));
                    }
                } else if let Some((full_comment_prefix, comment_suffix)) =
                    language.block_comment_delimiters()
                {
                    let comment_prefix = full_comment_prefix.trim_end_matches(' ');
                    let comment_prefix_whitespace = &full_comment_prefix[comment_prefix.len()..];
                    let prefix_range = comment_prefix_range(
                        snapshot.deref(),
                        start_row,
                        comment_prefix,
                        comment_prefix_whitespace,
                    );
                    let suffix_range = comment_suffix_range(
                        snapshot.deref(),
                        end_row,
                        comment_suffix.trim_start_matches(' '),
                        comment_suffix.starts_with(' '),
                    );

                    if prefix_range.is_empty() || suffix_range.is_empty() {
                        edits.push((
                            prefix_range.start..prefix_range.start,
                            full_comment_prefix.clone(),
                        ));
                        edits.push((suffix_range.end..suffix_range.end, comment_suffix.clone()));
                        suffixes_inserted.push((end_row, comment_suffix.len()));
                    } else {
                        edits.push((prefix_range, empty_str.clone()));
                        edits.push((suffix_range, empty_str.clone()));
                    }
                } else {
                    continue;
                }
            }

            drop(snapshot);
            this.buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            });

            // Adjust selections so that they end before any comment suffixes that
            // were inserted.
            let mut suffixes_inserted = suffixes_inserted.into_iter().peekable();
            let mut selections = this.selections.all::<Point>(cx);
            let snapshot = this.buffer.read(cx).read(cx);
            for selection in &mut selections {
                while let Some((row, suffix_len)) = suffixes_inserted.peek().copied() {
                    match row.cmp(&MultiBufferRow(selection.end.row)) {
                        Ordering::Less => {
                            suffixes_inserted.next();
                            continue;
                        }
                        Ordering::Greater => break,
                        Ordering::Equal => {
                            if selection.end.column == snapshot.line_len(row) {
                                if selection.is_empty() {
                                    selection.start.column -= suffix_len as u32;
                                }
                                selection.end.column -= suffix_len as u32;
                            }
                            break;
                        }
                    }
                }
            }

            drop(snapshot);
            this.change_selections(Some(Autoscroll::fit()), cx, |s| s.select(selections));

            let selections = this.selections.all::<Point>(cx);
            let selections_on_single_row = selections.windows(2).all(|selections| {
                selections[0].start.row == selections[1].start.row
                    && selections[0].end.row == selections[1].end.row
                    && selections[0].start.row == selections[0].end.row
            });
            let selections_selecting = selections
                .iter()
                .any(|selection| selection.start != selection.end);
            let advance_downwards = action.advance_downwards
                && selections_on_single_row
                && !selections_selecting
                && !matches!(this.mode, EditorMode::SingleLine { .. });

            if advance_downwards {
                let snapshot = this.buffer.read(cx).snapshot(cx);

                this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_cursors_with(|display_snapshot, display_point, _| {
                        let mut point = display_point.to_point(display_snapshot);
                        point.row += 1;
                        point = snapshot.clip_point(point, Bias::Left);
                        let display_point = point.to_display_point(display_snapshot);
                        let goal = SelectionGoal::HorizontalPosition(
                            display_snapshot
                                .x_for_display_point(display_point, &text_layout_details)
                                .into(),
                        );
                        (display_point, goal)
                    })
                });
            }
        });
    }

    pub fn select_enclosing_symbol(
        &mut self,
        _: &SelectEnclosingSymbol,
        cx: &mut ViewContext<Self>,
    ) {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let old_selections = self.selections.all::<usize>(cx).into_boxed_slice();

        fn update_selection(
            selection: &Selection<usize>,
            buffer_snap: &MultiBufferSnapshot,
        ) -> Option<Selection<usize>> {
            let cursor = selection.head();
            let (_buffer_id, symbols) = buffer_snap.symbols_containing(cursor, None)?;
            for symbol in symbols.iter().rev() {
                let start = symbol.range.start.to_offset(&buffer_snap);
                let end = symbol.range.end.to_offset(&buffer_snap);
                let new_range = start..end;
                if start < selection.start || end > selection.end {
                    return Some(Selection {
                        id: selection.id,
                        start: new_range.start,
                        end: new_range.end,
                        goal: SelectionGoal::None,
                        reversed: selection.reversed,
                    });
                }
            }
            None
        }

        let mut selected_larger_symbol = false;
        let new_selections = old_selections
            .iter()
            .map(|selection| match update_selection(selection, &buffer) {
                Some(new_selection) => {
                    if new_selection.range() != selection.range() {
                        selected_larger_symbol = true;
                    }
                    new_selection
                }
                None => selection.clone(),
            })
            .collect::<Vec<_>>();

        if selected_larger_symbol {
            self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(new_selections);
            });
        }
    }

    pub fn select_larger_syntax_node(
        &mut self,
        _: &SelectLargerSyntaxNode,
        cx: &mut ViewContext<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);
        let old_selections = self.selections.all::<usize>(cx).into_boxed_slice();

        let mut stack = mem::take(&mut self.select_larger_syntax_node_stack);
        let mut selected_larger_node = false;
        let new_selections = old_selections
            .iter()
            .map(|selection| {
                let old_range = selection.start..selection.end;
                let mut new_range = old_range.clone();
                while let Some(containing_range) =
                    buffer.range_for_syntax_ancestor(new_range.clone())
                {
                    new_range = containing_range;
                    if !display_map.intersects_fold(new_range.start)
                        && !display_map.intersects_fold(new_range.end)
                    {
                        break;
                    }
                }

                selected_larger_node |= new_range != old_range;
                Selection {
                    id: selection.id,
                    start: new_range.start,
                    end: new_range.end,
                    goal: SelectionGoal::None,
                    reversed: selection.reversed,
                }
            })
            .collect::<Vec<_>>();

        if selected_larger_node {
            stack.push(old_selections);
            self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(new_selections);
            });
        }
        self.select_larger_syntax_node_stack = stack;
    }

    pub fn select_smaller_syntax_node(
        &mut self,
        _: &SelectSmallerSyntaxNode,
        cx: &mut ViewContext<Self>,
    ) {
        let mut stack = mem::take(&mut self.select_larger_syntax_node_stack);
        if let Some(selections) = stack.pop() {
            self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select(selections.to_vec());
            });
        }
        self.select_larger_syntax_node_stack = stack;
    }

    fn refresh_runnables(&mut self, cx: &mut ViewContext<Self>) -> Task<()> {
        if !EditorSettings::get_global(cx).gutter.runnables {
            self.clear_tasks();
            return Task::ready(());
        }
        let project = self.project.clone();
        cx.spawn(|this, mut cx| async move {
            let Ok(display_snapshot) = this.update(&mut cx, |this, cx| {
                this.display_map.update(cx, |map, cx| map.snapshot(cx))
            }) else {
                return;
            };

            let Some(project) = project else {
                return;
            };

            let hide_runnables = project
                .update(&mut cx, |project, cx| {
                    // Do not display any test indicators in non-dev server remote projects.
                    project.is_remote() && project.ssh_connection_string(cx).is_none()
                })
                .unwrap_or(true);
            if hide_runnables {
                return;
            }
            let new_rows =
                cx.background_executor()
                    .spawn({
                        let snapshot = display_snapshot.clone();
                        async move {
                            Self::fetch_runnable_ranges(&snapshot, Anchor::min()..Anchor::max())
                        }
                    })
                    .await;
            let rows = Self::runnable_rows(project, display_snapshot, new_rows, cx.clone());

            this.update(&mut cx, |this, _| {
                this.clear_tasks();
                for (key, value) in rows {
                    this.insert_tasks(key, value);
                }
            })
            .ok();
        })
    }
    fn fetch_runnable_ranges(
        snapshot: &DisplaySnapshot,
        range: Range<Anchor>,
    ) -> Vec<language::RunnableRange> {
        snapshot.buffer_snapshot.runnable_ranges(range).collect()
    }

    fn runnable_rows(
        project: Model<Project>,
        snapshot: DisplaySnapshot,
        runnable_ranges: Vec<RunnableRange>,
        mut cx: AsyncWindowContext,
    ) -> Vec<((BufferId, u32), RunnableTasks)> {
        runnable_ranges
            .into_iter()
            .filter_map(|mut runnable| {
                let tasks = cx
                    .update(|cx| Self::templates_with_tags(&project, &mut runnable.runnable, cx))
                    .ok()?;
                if tasks.is_empty() {
                    return None;
                }

                let point = runnable.run_range.start.to_point(&snapshot.buffer_snapshot);

                let row = snapshot
                    .buffer_snapshot
                    .buffer_line_for_row(MultiBufferRow(point.row))?
                    .1
                    .start
                    .row;

                let context_range =
                    BufferOffset(runnable.full_range.start)..BufferOffset(runnable.full_range.end);
                Some((
                    (runnable.buffer_id, row),
                    RunnableTasks {
                        templates: tasks,
                        offset: MultiBufferOffset(runnable.run_range.start),
                        context_range,
                        column: point.column,
                        extra_variables: runnable.extra_captures,
                    },
                ))
            })
            .collect()
    }

    fn templates_with_tags(
        project: &Model<Project>,
        runnable: &mut Runnable,
        cx: &WindowContext<'_>,
    ) -> Vec<(TaskSourceKind, TaskTemplate)> {
        let (inventory, worktree_id, file) = project.read_with(cx, |project, cx| {
            let (worktree_id, file) = project
                .buffer_for_id(runnable.buffer, cx)
                .and_then(|buffer| buffer.read(cx).file())
                .map(|file| (WorktreeId::from_usize(file.worktree_id()), file.clone()))
                .unzip();

            (project.task_inventory().clone(), worktree_id, file)
        });

        let inventory = inventory.read(cx);
        let tags = mem::take(&mut runnable.tags);
        let mut tags: Vec<_> = tags
            .into_iter()
            .flat_map(|tag| {
                let tag = tag.0.clone();
                inventory
                    .list_tasks(
                        file.clone(),
                        Some(runnable.language.clone()),
                        worktree_id,
                        cx,
                    )
                    .into_iter()
                    .filter(move |(_, template)| {
                        template.tags.iter().any(|source_tag| source_tag == &tag)
                    })
            })
            .sorted_by_key(|(kind, _)| kind.to_owned())
            .collect();
        if let Some((leading_tag_source, _)) = tags.first() {
            // Strongest source wins; if we have worktree tag binding, prefer that to
            // global and language bindings;
            // if we have a global binding, prefer that to language binding.
            let first_mismatch = tags
                .iter()
                .position(|(tag_source, _)| tag_source != leading_tag_source);
            if let Some(index) = first_mismatch {
                tags.truncate(index);
            }
        }

        tags
    }

    pub fn move_to_enclosing_bracket(
        &mut self,
        _: &MoveToEnclosingBracket,
        cx: &mut ViewContext<Self>,
    ) {
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_offsets_with(|snapshot, selection| {
                let Some(enclosing_bracket_ranges) =
                    snapshot.enclosing_bracket_ranges(selection.start..selection.end)
                else {
                    return;
                };

                let mut best_length = usize::MAX;
                let mut best_inside = false;
                let mut best_in_bracket_range = false;
                let mut best_destination = None;
                for (open, close) in enclosing_bracket_ranges {
                    let close = close.to_inclusive();
                    let length = close.end() - open.start;
                    let inside = selection.start >= open.end && selection.end <= *close.start();
                    let in_bracket_range = open.to_inclusive().contains(&selection.head())
                        || close.contains(&selection.head());

                    // If best is next to a bracket and current isn't, skip
                    if !in_bracket_range && best_in_bracket_range {
                        continue;
                    }

                    // Prefer smaller lengths unless best is inside and current isn't
                    if length > best_length && (best_inside || !inside) {
                        continue;
                    }

                    best_length = length;
                    best_inside = inside;
                    best_in_bracket_range = in_bracket_range;
                    best_destination = Some(
                        if close.contains(&selection.start) && close.contains(&selection.end) {
                            if inside {
                                open.end
                            } else {
                                open.start
                            }
                        } else {
                            if inside {
                                *close.start()
                            } else {
                                *close.end()
                            }
                        },
                    );
                }

                if let Some(destination) = best_destination {
                    selection.collapse_to(destination, SelectionGoal::None);
                }
            })
        });
    }

    pub fn undo_selection(&mut self, _: &UndoSelection, cx: &mut ViewContext<Self>) {
        self.end_selection(cx);
        self.selection_history.mode = SelectionHistoryMode::Undoing;
        if let Some(entry) = self.selection_history.undo_stack.pop_back() {
            self.change_selections(None, cx, |s| s.select_anchors(entry.selections.to_vec()));
            self.select_next_state = entry.select_next_state;
            self.select_prev_state = entry.select_prev_state;
            self.add_selections_state = entry.add_selections_state;
            self.request_autoscroll(Autoscroll::newest(), cx);
        }
        self.selection_history.mode = SelectionHistoryMode::Normal;
    }

    pub fn redo_selection(&mut self, _: &RedoSelection, cx: &mut ViewContext<Self>) {
        self.end_selection(cx);
        self.selection_history.mode = SelectionHistoryMode::Redoing;
        if let Some(entry) = self.selection_history.redo_stack.pop_back() {
            self.change_selections(None, cx, |s| s.select_anchors(entry.selections.to_vec()));
            self.select_next_state = entry.select_next_state;
            self.select_prev_state = entry.select_prev_state;
            self.add_selections_state = entry.add_selections_state;
            self.request_autoscroll(Autoscroll::newest(), cx);
        }
        self.selection_history.mode = SelectionHistoryMode::Normal;
    }

    pub fn expand_excerpts(&mut self, action: &ExpandExcerpts, cx: &mut ViewContext<Self>) {
        self.expand_excerpts_for_direction(action.lines, ExpandExcerptDirection::UpAndDown, cx)
    }

    pub fn expand_excerpts_down(
        &mut self,
        action: &ExpandExcerptsDown,
        cx: &mut ViewContext<Self>,
    ) {
        self.expand_excerpts_for_direction(action.lines, ExpandExcerptDirection::Down, cx)
    }

    pub fn expand_excerpts_up(&mut self, action: &ExpandExcerptsUp, cx: &mut ViewContext<Self>) {
        self.expand_excerpts_for_direction(action.lines, ExpandExcerptDirection::Up, cx)
    }

    pub fn expand_excerpts_for_direction(
        &mut self,
        lines: u32,
        direction: ExpandExcerptDirection,
        cx: &mut ViewContext<Self>,
    ) {
        let selections = self.selections.disjoint_anchors();

        let lines = if lines == 0 {
            EditorSettings::get_global(cx).expand_excerpt_lines
        } else {
            lines
        };

        self.buffer.update(cx, |buffer, cx| {
            buffer.expand_excerpts(
                selections
                    .into_iter()
                    .map(|selection| selection.head().excerpt_id)
                    .dedup(),
                lines,
                direction,
                cx,
            )
        })
    }

    pub fn expand_excerpt(
        &mut self,
        excerpt: ExcerptId,
        direction: ExpandExcerptDirection,
        cx: &mut ViewContext<Self>,
    ) {
        let lines = EditorSettings::get_global(cx).expand_excerpt_lines;
        self.buffer.update(cx, |buffer, cx| {
            buffer.expand_excerpts([excerpt], lines, direction, cx)
        })
    }

    fn go_to_diagnostic(&mut self, _: &GoToDiagnostic, cx: &mut ViewContext<Self>) {
        self.go_to_diagnostic_impl(Direction::Next, cx)
    }

    fn go_to_prev_diagnostic(&mut self, _: &GoToPrevDiagnostic, cx: &mut ViewContext<Self>) {
        self.go_to_diagnostic_impl(Direction::Prev, cx)
    }

    pub fn go_to_diagnostic_impl(&mut self, direction: Direction, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let selection = self.selections.newest::<usize>(cx);

        // If there is an active Diagnostic Popover jump to its diagnostic instead.
        if direction == Direction::Next {
            if let Some(popover) = self.hover_state.diagnostic_popover.as_ref() {
                let (group_id, jump_to) = popover.activation_info();
                if self.activate_diagnostics(group_id, cx) {
                    self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                        let mut new_selection = s.newest_anchor().clone();
                        new_selection.collapse_to(jump_to, SelectionGoal::None);
                        s.select_anchors(vec![new_selection.clone()]);
                    });
                }
                return;
            }
        }

        let mut active_primary_range = self.active_diagnostics.as_ref().map(|active_diagnostics| {
            active_diagnostics
                .primary_range
                .to_offset(&buffer)
                .to_inclusive()
        });
        let mut search_start = if let Some(active_primary_range) = active_primary_range.as_ref() {
            if active_primary_range.contains(&selection.head()) {
                *active_primary_range.start()
            } else {
                selection.head()
            }
        } else {
            selection.head()
        };
        let snapshot = self.snapshot(cx);
        loop {
            let diagnostics = if direction == Direction::Prev {
                buffer.diagnostics_in_range::<_, usize>(0..search_start, true)
            } else {
                buffer.diagnostics_in_range::<_, usize>(search_start..buffer.len(), false)
            }
            .filter(|diagnostic| !snapshot.intersects_fold(diagnostic.range.start));
            let group = diagnostics
                // relies on diagnostics_in_range to return diagnostics with the same starting range to
                // be sorted in a stable way
                // skip until we are at current active diagnostic, if it exists
                .skip_while(|entry| {
                    (match direction {
                        Direction::Prev => entry.range.start >= search_start,
                        Direction::Next => entry.range.start <= search_start,
                    }) && self
                        .active_diagnostics
                        .as_ref()
                        .is_some_and(|a| a.group_id != entry.diagnostic.group_id)
                })
                .find_map(|entry| {
                    if entry.diagnostic.is_primary
                        && entry.diagnostic.severity <= DiagnosticSeverity::WARNING
                        && !entry.range.is_empty()
                        // if we match with the active diagnostic, skip it
                        && Some(entry.diagnostic.group_id)
                            != self.active_diagnostics.as_ref().map(|d| d.group_id)
                    {
                        Some((entry.range, entry.diagnostic.group_id))
                    } else {
                        None
                    }
                });

            if let Some((primary_range, group_id)) = group {
                if self.activate_diagnostics(group_id, cx) {
                    self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                        s.select(vec![Selection {
                            id: selection.id,
                            start: primary_range.start,
                            end: primary_range.start,
                            reversed: false,
                            goal: SelectionGoal::None,
                        }]);
                    });
                }
                break;
            } else {
                // Cycle around to the start of the buffer, potentially moving back to the start of
                // the currently active diagnostic.
                active_primary_range.take();
                if direction == Direction::Prev {
                    if search_start == buffer.len() {
                        break;
                    } else {
                        search_start = buffer.len();
                    }
                } else if search_start == 0 {
                    break;
                } else {
                    search_start = 0;
                }
            }
        }
    }

    fn go_to_hunk(&mut self, _: &GoToHunk, cx: &mut ViewContext<Self>) {
        let snapshot = self
            .display_map
            .update(cx, |display_map, cx| display_map.snapshot(cx));
        let selection = self.selections.newest::<Point>(cx);

        if !self.seek_in_direction(
            &snapshot,
            selection.head(),
            false,
            snapshot.buffer_snapshot.git_diff_hunks_in_range(
                MultiBufferRow(selection.head().row + 1)..MultiBufferRow::MAX,
            ),
            cx,
        ) {
            let wrapped_point = Point::zero();
            self.seek_in_direction(
                &snapshot,
                wrapped_point,
                true,
                snapshot.buffer_snapshot.git_diff_hunks_in_range(
                    MultiBufferRow(wrapped_point.row + 1)..MultiBufferRow::MAX,
                ),
                cx,
            );
        }
    }

    fn go_to_prev_hunk(&mut self, _: &GoToPrevHunk, cx: &mut ViewContext<Self>) {
        let snapshot = self
            .display_map
            .update(cx, |display_map, cx| display_map.snapshot(cx));
        let selection = self.selections.newest::<Point>(cx);

        if !self.seek_in_direction(
            &snapshot,
            selection.head(),
            false,
            snapshot.buffer_snapshot.git_diff_hunks_in_range_rev(
                MultiBufferRow(0)..MultiBufferRow(selection.head().row),
            ),
            cx,
        ) {
            let wrapped_point = snapshot.buffer_snapshot.max_point();
            self.seek_in_direction(
                &snapshot,
                wrapped_point,
                true,
                snapshot.buffer_snapshot.git_diff_hunks_in_range_rev(
                    MultiBufferRow(0)..MultiBufferRow(wrapped_point.row),
                ),
                cx,
            );
        }
    }

    fn seek_in_direction(
        &mut self,
        snapshot: &DisplaySnapshot,
        initial_point: Point,
        is_wrapped: bool,
        hunks: impl Iterator<Item = DiffHunk<MultiBufferRow>>,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        let display_point = initial_point.to_display_point(snapshot);
        let mut hunks = hunks
            .map(|hunk| diff_hunk_to_display(&hunk, &snapshot))
            .filter(|hunk| is_wrapped || !hunk.contains_display_row(display_point.row()))
            .dedup();

        if let Some(hunk) = hunks.next() {
            self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let row = hunk.start_display_row();
                let point = DisplayPoint::new(row, 0);
                s.select_display_ranges([point..point]);
            });

            true
        } else {
            false
        }
    }

    pub fn go_to_definition(
        &mut self,
        _: &GoToDefinition,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<bool>> {
        self.go_to_definition_of_kind(GotoDefinitionKind::Symbol, false, cx)
    }

    pub fn go_to_declaration(
        &mut self,
        _: &GoToDeclaration,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<bool>> {
        self.go_to_definition_of_kind(GotoDefinitionKind::Declaration, false, cx)
    }

    pub fn go_to_declaration_split(
        &mut self,
        _: &GoToDeclaration,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<bool>> {
        self.go_to_definition_of_kind(GotoDefinitionKind::Declaration, true, cx)
    }

    pub fn go_to_implementation(
        &mut self,
        _: &GoToImplementation,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<bool>> {
        self.go_to_definition_of_kind(GotoDefinitionKind::Implementation, false, cx)
    }

    pub fn go_to_implementation_split(
        &mut self,
        _: &GoToImplementationSplit,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<bool>> {
        self.go_to_definition_of_kind(GotoDefinitionKind::Implementation, true, cx)
    }

    pub fn go_to_type_definition(
        &mut self,
        _: &GoToTypeDefinition,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<bool>> {
        self.go_to_definition_of_kind(GotoDefinitionKind::Type, false, cx)
    }

    pub fn go_to_definition_split(
        &mut self,
        _: &GoToDefinitionSplit,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<bool>> {
        self.go_to_definition_of_kind(GotoDefinitionKind::Symbol, true, cx)
    }

    pub fn go_to_type_definition_split(
        &mut self,
        _: &GoToTypeDefinitionSplit,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<bool>> {
        self.go_to_definition_of_kind(GotoDefinitionKind::Type, true, cx)
    }

    fn go_to_definition_of_kind(
        &mut self,
        kind: GotoDefinitionKind,
        split: bool,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<bool>> {
        let Some(workspace) = self.workspace() else {
            return Task::ready(Ok(false));
        };
        let buffer = self.buffer.read(cx);
        let head = self.selections.newest::<usize>(cx).head();
        let (buffer, head) = if let Some(text_anchor) = buffer.text_anchor_for_position(head, cx) {
            text_anchor
        } else {
            return Task::ready(Ok(false));
        };

        let project = workspace.read(cx).project().clone();
        let definitions = project.update(cx, |project, cx| match kind {
            GotoDefinitionKind::Symbol => project.definition(&buffer, head, cx),
            GotoDefinitionKind::Declaration => project.declaration(&buffer, head, cx),
            GotoDefinitionKind::Type => project.type_definition(&buffer, head, cx),
            GotoDefinitionKind::Implementation => project.implementation(&buffer, head, cx),
        });

        cx.spawn(|editor, mut cx| async move {
            let definitions = definitions.await?;
            let navigated = editor
                .update(&mut cx, |editor, cx| {
                    editor.navigate_to_hover_links(
                        Some(kind),
                        definitions
                            .into_iter()
                            .filter(|location| {
                                hover_links::exclude_link_to_position(&buffer, &head, location, cx)
                            })
                            .map(HoverLink::Text)
                            .collect::<Vec<_>>(),
                        split,
                        cx,
                    )
                })?
                .await?;
            anyhow::Ok(navigated)
        })
    }

    pub fn open_url(&mut self, _: &OpenUrl, cx: &mut ViewContext<Self>) {
        let position = self.selections.newest_anchor().head();
        let Some((buffer, buffer_position)) =
            self.buffer.read(cx).text_anchor_for_position(position, cx)
        else {
            return;
        };

        cx.spawn(|editor, mut cx| async move {
            if let Some((_, url)) = find_url(&buffer, buffer_position, cx.clone()) {
                editor.update(&mut cx, |_, cx| {
                    cx.open_url(&url);
                })
            } else {
                Ok(())
            }
        })
        .detach();
    }

    pub(crate) fn navigate_to_hover_links(
        &mut self,
        kind: Option<GotoDefinitionKind>,
        mut definitions: Vec<HoverLink>,
        split: bool,
        cx: &mut ViewContext<Editor>,
    ) -> Task<Result<bool>> {
        // If there is one definition, just open it directly
        if definitions.len() == 1 {
            let definition = definitions.pop().unwrap();
            let target_task = match definition {
                HoverLink::Text(link) => Task::Ready(Some(Ok(Some(link.target)))),
                HoverLink::InlayHint(lsp_location, server_id) => {
                    self.compute_target_location(lsp_location, server_id, cx)
                }
                HoverLink::Url(url) => {
                    cx.open_url(&url);
                    Task::ready(Ok(None))
                }
            };
            cx.spawn(|editor, mut cx| async move {
                let target = target_task.await.context("target resolution task")?;
                if let Some(target) = target {
                    editor.update(&mut cx, |editor, cx| {
                        let Some(workspace) = editor.workspace() else {
                            return false;
                        };
                        let pane = workspace.read(cx).active_pane().clone();

                        let range = target.range.to_offset(target.buffer.read(cx));
                        let range = editor.range_for_match(&range);

                        /// If select range has more than one line, we
                        /// just point the cursor to range.start.
                        fn check_multiline_range(
                            buffer: &Buffer,
                            range: Range<usize>,
                        ) -> Range<usize> {
                            if buffer.offset_to_point(range.start).row
                                == buffer.offset_to_point(range.end).row
                            {
                                range
                            } else {
                                range.start..range.start
                            }
                        }

                        if Some(&target.buffer) == editor.buffer.read(cx).as_singleton().as_ref() {
                            let buffer = target.buffer.read(cx);
                            let range = check_multiline_range(buffer, range);
                            editor.change_selections(Some(Autoscroll::focused()), cx, |s| {
                                s.select_ranges([range]);
                            });
                        } else {
                            cx.window_context().defer(move |cx| {
                                let target_editor: View<Self> =
                                    workspace.update(cx, |workspace, cx| {
                                        let pane = if split {
                                            workspace.adjacent_pane(cx)
                                        } else {
                                            workspace.active_pane().clone()
                                        };

                                        workspace.open_project_item(
                                            pane,
                                            target.buffer.clone(),
                                            true,
                                            true,
                                            cx,
                                        )
                                    });
                                target_editor.update(cx, |target_editor, cx| {
                                    // When selecting a definition in a different buffer, disable the nav history
                                    // to avoid creating a history entry at the previous cursor location.
                                    pane.update(cx, |pane, _| pane.disable_history());
                                    let buffer = target.buffer.read(cx);
                                    let range = check_multiline_range(buffer, range);
                                    target_editor.change_selections(
                                        Some(Autoscroll::focused()),
                                        cx,
                                        |s| {
                                            s.select_ranges([range]);
                                        },
                                    );
                                    pane.update(cx, |pane, _| pane.enable_history());
                                });
                            });
                        }
                        true
                    })
                } else {
                    Ok(false)
                }
            })
        } else if !definitions.is_empty() {
            let replica_id = self.replica_id(cx);
            cx.spawn(|editor, mut cx| async move {
                let (title, location_tasks, workspace) = editor
                    .update(&mut cx, |editor, cx| {
                        let tab_kind = match kind {
                            Some(GotoDefinitionKind::Implementation) => "Implementations",
                            _ => "Definitions",
                        };
                        let title = definitions
                            .iter()
                            .find_map(|definition| match definition {
                                HoverLink::Text(link) => link.origin.as_ref().map(|origin| {
                                    let buffer = origin.buffer.read(cx);
                                    format!(
                                        "{} for {}",
                                        tab_kind,
                                        buffer
                                            .text_for_range(origin.range.clone())
                                            .collect::<String>()
                                    )
                                }),
                                HoverLink::InlayHint(_, _) => None,
                                HoverLink::Url(_) => None,
                            })
                            .unwrap_or(tab_kind.to_string());
                        let location_tasks = definitions
                            .into_iter()
                            .map(|definition| match definition {
                                HoverLink::Text(link) => Task::Ready(Some(Ok(Some(link.target)))),
                                HoverLink::InlayHint(lsp_location, server_id) => {
                                    editor.compute_target_location(lsp_location, server_id, cx)
                                }
                                HoverLink::Url(_) => Task::ready(Ok(None)),
                            })
                            .collect::<Vec<_>>();
                        (title, location_tasks, editor.workspace().clone())
                    })
                    .context("location tasks preparation")?;

                let locations = futures::future::join_all(location_tasks)
                    .await
                    .into_iter()
                    .filter_map(|location| location.transpose())
                    .collect::<Result<_>>()
                    .context("location tasks")?;

                let Some(workspace) = workspace else {
                    return Ok(false);
                };
                let opened = workspace
                    .update(&mut cx, |workspace, cx| {
                        Self::open_locations_in_multibuffer(
                            workspace, locations, replica_id, title, split, cx,
                        )
                    })
                    .ok();

                anyhow::Ok(opened.is_some())
            })
        } else {
            Task::ready(Ok(false))
        }
    }

    fn compute_target_location(
        &self,
        lsp_location: lsp::Location,
        server_id: LanguageServerId,
        cx: &mut ViewContext<Editor>,
    ) -> Task<anyhow::Result<Option<Location>>> {
        let Some(project) = self.project.clone() else {
            return Task::Ready(Some(Ok(None)));
        };

        cx.spawn(move |editor, mut cx| async move {
            let location_task = editor.update(&mut cx, |editor, cx| {
                project.update(cx, |project, cx| {
                    let language_server_name =
                        editor.buffer.read(cx).as_singleton().and_then(|buffer| {
                            project
                                .language_server_for_buffer(buffer.read(cx), server_id, cx)
                                .map(|(lsp_adapter, _)| lsp_adapter.name.clone())
                        });
                    language_server_name.map(|language_server_name| {
                        project.open_local_buffer_via_lsp(
                            lsp_location.uri.clone(),
                            server_id,
                            language_server_name,
                            cx,
                        )
                    })
                })
            })?;
            let location = match location_task {
                Some(task) => Some({
                    let target_buffer_handle = task.await.context("open local buffer")?;
                    let range = target_buffer_handle.update(&mut cx, |target_buffer, _| {
                        let target_start = target_buffer
                            .clip_point_utf16(point_from_lsp(lsp_location.range.start), Bias::Left);
                        let target_end = target_buffer
                            .clip_point_utf16(point_from_lsp(lsp_location.range.end), Bias::Left);
                        target_buffer.anchor_after(target_start)
                            ..target_buffer.anchor_before(target_end)
                    })?;
                    Location {
                        buffer: target_buffer_handle,
                        range,
                    }
                }),
                None => None,
            };
            Ok(location)
        })
    }

    pub fn find_all_references(
        &mut self,
        _: &FindAllReferences,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let multi_buffer = self.buffer.read(cx);
        let selection = self.selections.newest::<usize>(cx);
        let head = selection.head();

        let multi_buffer_snapshot = multi_buffer.snapshot(cx);
        let head_anchor = multi_buffer_snapshot.anchor_at(
            head,
            if head < selection.tail() {
                Bias::Right
            } else {
                Bias::Left
            },
        );

        match self
            .find_all_references_task_sources
            .binary_search_by(|anchor| anchor.cmp(&head_anchor, &multi_buffer_snapshot))
        {
            Ok(_) => {
                log::info!(
                    "Ignoring repeated FindAllReferences invocation with the position of already running task"
                );
                return None;
            }
            Err(i) => {
                self.find_all_references_task_sources.insert(i, head_anchor);
            }
        }

        let (buffer, head) = multi_buffer.text_anchor_for_position(head, cx)?;
        let replica_id = self.replica_id(cx);
        let workspace = self.workspace()?;
        let project = workspace.read(cx).project().clone();
        let references = project.update(cx, |project, cx| project.references(&buffer, head, cx));
        Some(cx.spawn(|editor, mut cx| async move {
            let _cleanup = defer({
                let mut cx = cx.clone();
                move || {
                    let _ = editor.update(&mut cx, |editor, _| {
                        if let Ok(i) =
                            editor
                                .find_all_references_task_sources
                                .binary_search_by(|anchor| {
                                    anchor.cmp(&head_anchor, &multi_buffer_snapshot)
                                })
                        {
                            editor.find_all_references_task_sources.remove(i);
                        }
                    });
                }
            });

            let locations = references.await?;
            if locations.is_empty() {
                return anyhow::Ok(());
            }

            workspace.update(&mut cx, |workspace, cx| {
                let title = locations
                    .first()
                    .as_ref()
                    .map(|location| {
                        let buffer = location.buffer.read(cx);
                        format!(
                            "References to `{}`",
                            buffer
                                .text_for_range(location.range.clone())
                                .collect::<String>()
                        )
                    })
                    .unwrap();
                Self::open_locations_in_multibuffer(
                    workspace, locations, replica_id, title, false, cx,
                );
            })
        }))
    }

    /// Opens a multibuffer with the given project locations in it
    pub fn open_locations_in_multibuffer(
        workspace: &mut Workspace,
        mut locations: Vec<Location>,
        replica_id: ReplicaId,
        title: String,
        split: bool,
        cx: &mut ViewContext<Workspace>,
    ) {
        // If there are multiple definitions, open them in a multibuffer
        locations.sort_by_key(|location| location.buffer.read(cx).remote_id());
        let mut locations = locations.into_iter().peekable();
        let mut ranges_to_highlight = Vec::new();
        let capability = workspace.project().read(cx).capability();

        let excerpt_buffer = cx.new_model(|cx| {
            let mut multibuffer = MultiBuffer::new(replica_id, capability);
            while let Some(location) = locations.next() {
                let buffer = location.buffer.read(cx);
                let mut ranges_for_buffer = Vec::new();
                let range = location.range.to_offset(buffer);
                ranges_for_buffer.push(range.clone());

                while let Some(next_location) = locations.peek() {
                    if next_location.buffer == location.buffer {
                        ranges_for_buffer.push(next_location.range.to_offset(buffer));
                        locations.next();
                    } else {
                        break;
                    }
                }

                ranges_for_buffer.sort_by_key(|range| (range.start, Reverse(range.end)));
                ranges_to_highlight.extend(multibuffer.push_excerpts_with_context_lines(
                    location.buffer.clone(),
                    ranges_for_buffer,
                    DEFAULT_MULTIBUFFER_CONTEXT,
                    cx,
                ))
            }

            multibuffer.with_title(title)
        });

        let editor = cx.new_view(|cx| {
            Editor::for_multibuffer(excerpt_buffer, Some(workspace.project().clone()), true, cx)
        });
        editor.update(cx, |editor, cx| {
            if let Some(first_range) = ranges_to_highlight.first() {
                editor.change_selections(None, cx, |selections| {
                    selections.clear_disjoint();
                    selections.select_anchor_ranges(std::iter::once(first_range.clone()));
                });
            }
            editor.highlight_background::<Self>(
                &ranges_to_highlight,
                |theme| theme.editor_highlighted_line_background,
                cx,
            );
        });

        let item = Box::new(editor);
        let item_id = item.item_id();

        if split {
            workspace.split_item(SplitDirection::Right, item.clone(), cx);
        } else {
            let destination_index = workspace.active_pane().update(cx, |pane, cx| {
                if PreviewTabsSettings::get_global(cx).enable_preview_from_code_navigation {
                    pane.close_current_preview_item(cx)
                } else {
                    None
                }
            });
            workspace.add_item_to_active_pane(item.clone(), destination_index, true, cx);
        }
        workspace.active_pane().update(cx, |pane, cx| {
            pane.set_preview_item_id(Some(item_id), cx);
        });
    }

    pub fn rename(&mut self, _: &Rename, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        use language::ToOffset as _;

        let project = self.project.clone()?;
        let selection = self.selections.newest_anchor().clone();
        let (cursor_buffer, cursor_buffer_position) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(selection.head(), cx)?;
        let (tail_buffer, cursor_buffer_position_end) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(selection.tail(), cx)?;
        if tail_buffer != cursor_buffer {
            return None;
        }

        let snapshot = cursor_buffer.read(cx).snapshot();
        let cursor_buffer_offset = cursor_buffer_position.to_offset(&snapshot);
        let cursor_buffer_offset_end = cursor_buffer_position_end.to_offset(&snapshot);
        let prepare_rename = project.update(cx, |project, cx| {
            project.prepare_rename(cursor_buffer.clone(), cursor_buffer_offset, cx)
        });
        drop(snapshot);

        Some(cx.spawn(|this, mut cx| async move {
            let rename_range = if let Some(range) = prepare_rename.await? {
                Some(range)
            } else {
                this.update(&mut cx, |this, cx| {
                    let buffer = this.buffer.read(cx).snapshot(cx);
                    let mut buffer_highlights = this
                        .document_highlights_for_position(selection.head(), &buffer)
                        .filter(|highlight| {
                            highlight.start.excerpt_id == selection.head().excerpt_id
                                && highlight.end.excerpt_id == selection.head().excerpt_id
                        });
                    buffer_highlights
                        .next()
                        .map(|highlight| highlight.start.text_anchor..highlight.end.text_anchor)
                })?
            };
            if let Some(rename_range) = rename_range {
                this.update(&mut cx, |this, cx| {
                    let snapshot = cursor_buffer.read(cx).snapshot();
                    let rename_buffer_range = rename_range.to_offset(&snapshot);
                    let cursor_offset_in_rename_range =
                        cursor_buffer_offset.saturating_sub(rename_buffer_range.start);
                    let cursor_offset_in_rename_range_end =
                        cursor_buffer_offset_end.saturating_sub(rename_buffer_range.start);

                    this.take_rename(false, cx);
                    let buffer = this.buffer.read(cx).read(cx);
                    let cursor_offset = selection.head().to_offset(&buffer);
                    let rename_start = cursor_offset.saturating_sub(cursor_offset_in_rename_range);
                    let rename_end = rename_start + rename_buffer_range.len();
                    let range = buffer.anchor_before(rename_start)..buffer.anchor_after(rename_end);
                    let mut old_highlight_id = None;
                    let old_name: Arc<str> = buffer
                        .chunks(rename_start..rename_end, true)
                        .map(|chunk| {
                            if old_highlight_id.is_none() {
                                old_highlight_id = chunk.syntax_highlight_id;
                            }
                            chunk.text
                        })
                        .collect::<String>()
                        .into();

                    drop(buffer);

                    // Position the selection in the rename editor so that it matches the current selection.
                    this.show_local_selections = false;
                    let rename_editor = cx.new_view(|cx| {
                        let mut editor = Editor::single_line(cx);
                        editor.buffer.update(cx, |buffer, cx| {
                            buffer.edit([(0..0, old_name.clone())], None, cx)
                        });
                        let rename_selection_range = match cursor_offset_in_rename_range
                            .cmp(&cursor_offset_in_rename_range_end)
                        {
                            Ordering::Equal => {
                                editor.select_all(&SelectAll, cx);
                                return editor;
                            }
                            Ordering::Less => {
                                cursor_offset_in_rename_range..cursor_offset_in_rename_range_end
                            }
                            Ordering::Greater => {
                                cursor_offset_in_rename_range_end..cursor_offset_in_rename_range
                            }
                        };
                        if rename_selection_range.end > old_name.len() {
                            editor.select_all(&SelectAll, cx);
                        } else {
                            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                                s.select_ranges([rename_selection_range]);
                            });
                        }
                        editor
                    });
                    cx.subscribe(&rename_editor, |_, _, e, cx| match e {
                        EditorEvent::Focused => cx.emit(EditorEvent::FocusedIn),
                        _ => {}
                    })
                    .detach();

                    let write_highlights =
                        this.clear_background_highlights::<DocumentHighlightWrite>(cx);
                    let read_highlights =
                        this.clear_background_highlights::<DocumentHighlightRead>(cx);
                    let ranges = write_highlights
                        .iter()
                        .flat_map(|(_, ranges)| ranges.iter())
                        .chain(read_highlights.iter().flat_map(|(_, ranges)| ranges.iter()))
                        .cloned()
                        .collect();

                    this.highlight_text::<Rename>(
                        ranges,
                        HighlightStyle {
                            fade_out: Some(0.6),
                            ..Default::default()
                        },
                        cx,
                    );
                    let rename_focus_handle = rename_editor.focus_handle(cx);
                    cx.focus(&rename_focus_handle);
                    let block_id = this.insert_blocks(
                        [BlockProperties {
                            style: BlockStyle::Flex,
                            position: range.start,
                            height: 1,
                            render: Box::new({
                                let rename_editor = rename_editor.clone();
                                move |cx: &mut BlockContext| {
                                    let mut text_style = cx.editor_style.text.clone();
                                    if let Some(highlight_style) = old_highlight_id
                                        .and_then(|h| h.style(&cx.editor_style.syntax))
                                    {
                                        text_style = text_style.highlight(highlight_style);
                                    }
                                    div()
                                        .pl(cx.anchor_x)
                                        .child(EditorElement::new(
                                            &rename_editor,
                                            EditorStyle {
                                                background: cx.theme().system().transparent,
                                                local_player: cx.editor_style.local_player,
                                                text: text_style,
                                                scrollbar_width: cx.editor_style.scrollbar_width,
                                                syntax: cx.editor_style.syntax.clone(),
                                                status: cx.editor_style.status.clone(),
                                                inlay_hints_style: HighlightStyle {
                                                    color: Some(cx.theme().status().hint),
                                                    font_weight: Some(FontWeight::BOLD),
                                                    ..HighlightStyle::default()
                                                },
                                                suggestions_style: HighlightStyle {
                                                    color: Some(cx.theme().status().predictive),
                                                    ..HighlightStyle::default()
                                                },
                                            },
                                        ))
                                        .into_any_element()
                                }
                            }),
                            disposition: BlockDisposition::Below,
                            priority: 0,
                        }],
                        Some(Autoscroll::fit()),
                        cx,
                    )[0];
                    this.pending_rename = Some(RenameState {
                        range,
                        old_name,
                        editor: rename_editor,
                        block_id,
                    });
                })?;
            }

            Ok(())
        }))
    }

    pub fn confirm_rename(
        &mut self,
        _: &ConfirmRename,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let rename = self.take_rename(false, cx)?;
        let workspace = self.workspace()?;
        let (start_buffer, start) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(rename.range.start, cx)?;
        let (end_buffer, end) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(rename.range.end, cx)?;
        if start_buffer != end_buffer {
            return None;
        }

        let buffer = start_buffer;
        let range = start..end;
        let old_name = rename.old_name;
        let new_name = rename.editor.read(cx).text(cx);

        let rename = workspace
            .read(cx)
            .project()
            .clone()
            .update(cx, |project, cx| {
                project.perform_rename(buffer.clone(), range.start, new_name.clone(), true, cx)
            });
        let workspace = workspace.downgrade();

        Some(cx.spawn(|editor, mut cx| async move {
            let project_transaction = rename.await?;
            Self::open_project_transaction(
                &editor,
                workspace,
                project_transaction,
                format!("Rename: {} → {}", old_name, new_name),
                cx.clone(),
            )
            .await?;

            editor.update(&mut cx, |editor, cx| {
                editor.refresh_document_highlights(cx);
            })?;
            Ok(())
        }))
    }

    fn take_rename(
        &mut self,
        moving_cursor: bool,
        cx: &mut ViewContext<Self>,
    ) -> Option<RenameState> {
        let rename = self.pending_rename.take()?;
        if rename.editor.focus_handle(cx).is_focused(cx) {
            cx.focus(&self.focus_handle);
        }

        self.remove_blocks(
            [rename.block_id].into_iter().collect(),
            Some(Autoscroll::fit()),
            cx,
        );
        self.clear_highlights::<Rename>(cx);
        self.show_local_selections = true;

        if moving_cursor {
            let rename_editor = rename.editor.read(cx);
            let cursor_in_rename_editor = rename_editor.selections.newest::<usize>(cx).head();

            // Update the selection to match the position of the selection inside
            // the rename editor.
            let snapshot = self.buffer.read(cx).read(cx);
            let rename_range = rename.range.to_offset(&snapshot);
            let cursor_in_editor = snapshot
                .clip_offset(rename_range.start + cursor_in_rename_editor, Bias::Left)
                .min(rename_range.end);
            drop(snapshot);

            self.change_selections(None, cx, |s| {
                s.select_ranges(vec![cursor_in_editor..cursor_in_editor])
            });
        } else {
            self.refresh_document_highlights(cx);
        }

        Some(rename)
    }

    pub fn pending_rename(&self) -> Option<&RenameState> {
        self.pending_rename.as_ref()
    }

    fn format(&mut self, _: &Format, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        let project = match &self.project {
            Some(project) => project.clone(),
            None => return None,
        };

        Some(self.perform_format(project, FormatTrigger::Manual, cx))
    }

    fn perform_format(
        &mut self,
        project: Model<Project>,
        trigger: FormatTrigger,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let buffer = self.buffer().clone();
        let mut buffers = buffer.read(cx).all_buffers();
        if trigger == FormatTrigger::Save {
            buffers.retain(|buffer| buffer.read(cx).is_dirty());
        }

        let mut timeout = cx.background_executor().timer(FORMAT_TIMEOUT).fuse();
        let format = project.update(cx, |project, cx| project.format(buffers, true, trigger, cx));

        cx.spawn(|_, mut cx| async move {
            let transaction = futures::select_biased! {
                () = timeout => {
                    log::warn!("timed out waiting for formatting");
                    None
                }
                transaction = format.log_err().fuse() => transaction,
            };

            buffer
                .update(&mut cx, |buffer, cx| {
                    if let Some(transaction) = transaction {
                        if !buffer.is_singleton() {
                            buffer.push_transaction(&transaction.0, cx);
                        }
                    }

                    cx.notify();
                })
                .ok();

            Ok(())
        })
    }

    fn restart_language_server(&mut self, _: &RestartLanguageServer, cx: &mut ViewContext<Self>) {
        if let Some(project) = self.project.clone() {
            self.buffer.update(cx, |multi_buffer, cx| {
                project.update(cx, |project, cx| {
                    project.restart_language_servers_for_buffers(multi_buffer.all_buffers(), cx);
                });
            })
        }
    }

    fn cancel_language_server_work(
        &mut self,
        _: &CancelLanguageServerWork,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(project) = self.project.clone() {
            self.buffer.update(cx, |multi_buffer, cx| {
                project.update(cx, |project, cx| {
                    project.cancel_language_server_work_for_buffers(multi_buffer.all_buffers(), cx);
                });
            })
        }
    }

    fn show_character_palette(&mut self, _: &ShowCharacterPalette, cx: &mut ViewContext<Self>) {
        cx.show_character_palette();
    }

    fn refresh_active_diagnostics(&mut self, cx: &mut ViewContext<Editor>) {
        if let Some(active_diagnostics) = self.active_diagnostics.as_mut() {
            let buffer = self.buffer.read(cx).snapshot(cx);
            let primary_range_start = active_diagnostics.primary_range.start.to_offset(&buffer);
            let is_valid = buffer
                .diagnostics_in_range::<_, usize>(active_diagnostics.primary_range.clone(), false)
                .any(|entry| {
                    entry.diagnostic.is_primary
                        && !entry.range.is_empty()
                        && entry.range.start == primary_range_start
                        && entry.diagnostic.message == active_diagnostics.primary_message
                });

            if is_valid != active_diagnostics.is_valid {
                active_diagnostics.is_valid = is_valid;
                let mut new_styles = HashMap::default();
                for (block_id, diagnostic) in &active_diagnostics.blocks {
                    new_styles.insert(
                        *block_id,
                        diagnostic_block_renderer(diagnostic.clone(), None, true, is_valid),
                    );
                }
                self.display_map.update(cx, |display_map, _cx| {
                    display_map.replace_blocks(new_styles)
                });
            }
        }
    }

    fn activate_diagnostics(&mut self, group_id: usize, cx: &mut ViewContext<Self>) -> bool {
        self.dismiss_diagnostics(cx);
        let snapshot = self.snapshot(cx);
        self.active_diagnostics = self.display_map.update(cx, |display_map, cx| {
            let buffer = self.buffer.read(cx).snapshot(cx);

            let mut primary_range = None;
            let mut primary_message = None;
            let mut group_end = Point::zero();
            let diagnostic_group = buffer
                .diagnostic_group::<MultiBufferPoint>(group_id)
                .filter_map(|entry| {
                    if snapshot.is_line_folded(MultiBufferRow(entry.range.start.row))
                        && (entry.range.start.row == entry.range.end.row
                            || snapshot.is_line_folded(MultiBufferRow(entry.range.end.row)))
                    {
                        return None;
                    }
                    if entry.range.end > group_end {
                        group_end = entry.range.end;
                    }
                    if entry.diagnostic.is_primary {
                        primary_range = Some(entry.range.clone());
                        primary_message = Some(entry.diagnostic.message.clone());
                    }
                    Some(entry)
                })
                .collect::<Vec<_>>();
            let primary_range = primary_range?;
            let primary_message = primary_message?;
            let primary_range =
                buffer.anchor_after(primary_range.start)..buffer.anchor_before(primary_range.end);

            let blocks = display_map
                .insert_blocks(
                    diagnostic_group.iter().map(|entry| {
                        let diagnostic = entry.diagnostic.clone();
                        let message_height = diagnostic.message.matches('\n').count() as u32 + 1;
                        BlockProperties {
                            style: BlockStyle::Fixed,
                            position: buffer.anchor_after(entry.range.start),
                            height: message_height,
                            render: diagnostic_block_renderer(diagnostic, None, true, true),
                            disposition: BlockDisposition::Below,
                            priority: 0,
                        }
                    }),
                    cx,
                )
                .into_iter()
                .zip(diagnostic_group.into_iter().map(|entry| entry.diagnostic))
                .collect();

            Some(ActiveDiagnosticGroup {
                primary_range,
                primary_message,
                group_id,
                blocks,
                is_valid: true,
            })
        });
        self.active_diagnostics.is_some()
    }

    fn dismiss_diagnostics(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_diagnostic_group) = self.active_diagnostics.take() {
            self.display_map.update(cx, |display_map, cx| {
                display_map.remove_blocks(active_diagnostic_group.blocks.into_keys().collect(), cx);
            });
            cx.notify();
        }
    }

    pub fn set_selections_from_remote(
        &mut self,
        selections: Vec<Selection<Anchor>>,
        pending_selection: Option<Selection<Anchor>>,
        cx: &mut ViewContext<Self>,
    ) {
        let old_cursor_position = self.selections.newest_anchor().head();
        self.selections.change_with(cx, |s| {
            s.select_anchors(selections);
            if let Some(pending_selection) = pending_selection {
                s.set_pending(pending_selection, SelectMode::Character);
            } else {
                s.clear_pending();
            }
        });
        self.selections_did_change(false, &old_cursor_position, true, cx);
    }

    fn push_to_selection_history(&mut self) {
        self.selection_history.push(SelectionHistoryEntry {
            selections: self.selections.disjoint_anchors(),
            select_next_state: self.select_next_state.clone(),
            select_prev_state: self.select_prev_state.clone(),
            add_selections_state: self.add_selections_state.clone(),
        });
    }

    pub fn transact(
        &mut self,
        cx: &mut ViewContext<Self>,
        update: impl FnOnce(&mut Self, &mut ViewContext<Self>),
    ) -> Option<TransactionId> {
        self.start_transaction_at(Instant::now(), cx);
        update(self, cx);
        self.end_transaction_at(Instant::now(), cx)
    }

    fn start_transaction_at(&mut self, now: Instant, cx: &mut ViewContext<Self>) {
        self.end_selection(cx);
        if let Some(tx_id) = self
            .buffer
            .update(cx, |buffer, cx| buffer.start_transaction_at(now, cx))
        {
            self.selection_history
                .insert_transaction(tx_id, self.selections.disjoint_anchors());
            cx.emit(EditorEvent::TransactionBegun {
                transaction_id: tx_id,
            })
        }
    }

    fn end_transaction_at(
        &mut self,
        now: Instant,
        cx: &mut ViewContext<Self>,
    ) -> Option<TransactionId> {
        if let Some(transaction_id) = self
            .buffer
            .update(cx, |buffer, cx| buffer.end_transaction_at(now, cx))
        {
            if let Some((_, end_selections)) =
                self.selection_history.transaction_mut(transaction_id)
            {
                *end_selections = Some(self.selections.disjoint_anchors());
            } else {
                log::error!("unexpectedly ended a transaction that wasn't started by this editor");
            }

            cx.emit(EditorEvent::Edited { transaction_id });
            Some(transaction_id)
        } else {
            None
        }
    }

    pub fn fold(&mut self, _: &actions::Fold, cx: &mut ViewContext<Self>) {
        let mut fold_ranges = Vec::new();

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        let selections = self.selections.all_adjusted(cx);
        for selection in selections {
            let range = selection.range().sorted();
            let buffer_start_row = range.start.row;

            for row in (0..=range.end.row).rev() {
                if let Some((foldable_range, fold_text)) =
                    display_map.foldable_range(MultiBufferRow(row))
                {
                    if foldable_range.end.row >= buffer_start_row {
                        fold_ranges.push((foldable_range, fold_text));
                        if row <= range.start.row {
                            break;
                        }
                    }
                }
            }
        }

        self.fold_ranges(fold_ranges, true, cx);
    }

    pub fn fold_at(&mut self, fold_at: &FoldAt, cx: &mut ViewContext<Self>) {
        let buffer_row = fold_at.buffer_row;
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        if let Some((fold_range, placeholder)) = display_map.foldable_range(buffer_row) {
            let autoscroll = self
                .selections
                .all::<Point>(cx)
                .iter()
                .any(|selection| fold_range.overlaps(&selection.range()));

            self.fold_ranges([(fold_range, placeholder)], autoscroll, cx);
        }
    }

    pub fn unfold_lines(&mut self, _: &UnfoldLines, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = &display_map.buffer_snapshot;
        let selections = self.selections.all::<Point>(cx);
        let ranges = selections
            .iter()
            .map(|s| {
                let range = s.display_range(&display_map).sorted();
                let mut start = range.start.to_point(&display_map);
                let mut end = range.end.to_point(&display_map);
                start.column = 0;
                end.column = buffer.line_len(MultiBufferRow(end.row));
                start..end
            })
            .collect::<Vec<_>>();

        self.unfold_ranges(ranges, true, true, cx);
    }

    pub fn unfold_at(&mut self, unfold_at: &UnfoldAt, cx: &mut ViewContext<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        let intersection_range = Point::new(unfold_at.buffer_row.0, 0)
            ..Point::new(
                unfold_at.buffer_row.0,
                display_map.buffer_snapshot.line_len(unfold_at.buffer_row),
            );

        let autoscroll = self
            .selections
            .all::<Point>(cx)
            .iter()
            .any(|selection| selection.range().overlaps(&intersection_range));

        self.unfold_ranges(std::iter::once(intersection_range), true, autoscroll, cx)
    }

    pub fn fold_selected_ranges(&mut self, _: &FoldSelectedRanges, cx: &mut ViewContext<Self>) {
        let selections = self.selections.all::<Point>(cx);
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let line_mode = self.selections.line_mode;
        let ranges = selections.into_iter().map(|s| {
            if line_mode {
                let start = Point::new(s.start.row, 0);
                let end = Point::new(
                    s.end.row,
                    display_map
                        .buffer_snapshot
                        .line_len(MultiBufferRow(s.end.row)),
                );
                (start..end, display_map.fold_placeholder.clone())
            } else {
                (s.start..s.end, display_map.fold_placeholder.clone())
            }
        });
        self.fold_ranges(ranges, true, cx);
    }

    pub fn fold_ranges<T: ToOffset + Clone>(
        &mut self,
        ranges: impl IntoIterator<Item = (Range<T>, FoldPlaceholder)>,
        auto_scroll: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let mut fold_ranges = Vec::new();
        let mut buffers_affected = HashMap::default();
        let multi_buffer = self.buffer().read(cx);
        for (fold_range, fold_text) in ranges {
            if let Some((_, buffer, _)) =
                multi_buffer.excerpt_containing(fold_range.start.clone(), cx)
            {
                buffers_affected.insert(buffer.read(cx).remote_id(), buffer);
            };
            fold_ranges.push((fold_range, fold_text));
        }

        let mut ranges = fold_ranges.into_iter().peekable();
        if ranges.peek().is_some() {
            self.display_map.update(cx, |map, cx| map.fold(ranges, cx));

            if auto_scroll {
                self.request_autoscroll(Autoscroll::fit(), cx);
            }

            for buffer in buffers_affected.into_values() {
                self.sync_expanded_diff_hunks(buffer, cx);
            }

            cx.notify();

            if let Some(active_diagnostics) = self.active_diagnostics.take() {
                // Clear diagnostics block when folding a range that contains it.
                let snapshot = self.snapshot(cx);
                if snapshot.intersects_fold(active_diagnostics.primary_range.start) {
                    drop(snapshot);
                    self.active_diagnostics = Some(active_diagnostics);
                    self.dismiss_diagnostics(cx);
                } else {
                    self.active_diagnostics = Some(active_diagnostics);
                }
            }

            self.scrollbar_marker_state.dirty = true;
        }
    }

    pub fn unfold_ranges<T: ToOffset + Clone>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        inclusive: bool,
        auto_scroll: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let mut unfold_ranges = Vec::new();
        let mut buffers_affected = HashMap::default();
        let multi_buffer = self.buffer().read(cx);
        for range in ranges {
            if let Some((_, buffer, _)) = multi_buffer.excerpt_containing(range.start.clone(), cx) {
                buffers_affected.insert(buffer.read(cx).remote_id(), buffer);
            };
            unfold_ranges.push(range);
        }

        let mut ranges = unfold_ranges.into_iter().peekable();
        if ranges.peek().is_some() {
            self.display_map
                .update(cx, |map, cx| map.unfold(ranges, inclusive, cx));
            if auto_scroll {
                self.request_autoscroll(Autoscroll::fit(), cx);
            }

            for buffer in buffers_affected.into_values() {
                self.sync_expanded_diff_hunks(buffer, cx);
            }

            cx.notify();
            self.scrollbar_marker_state.dirty = true;
            self.active_indent_guides_state.dirty = true;
        }
    }

    pub fn set_gutter_hovered(&mut self, hovered: bool, cx: &mut ViewContext<Self>) {
        if hovered != self.gutter_hovered {
            self.gutter_hovered = hovered;
            cx.notify();
        }
    }

    pub fn insert_blocks(
        &mut self,
        blocks: impl IntoIterator<Item = BlockProperties<Anchor>>,
        autoscroll: Option<Autoscroll>,
        cx: &mut ViewContext<Self>,
    ) -> Vec<CustomBlockId> {
        let blocks = self
            .display_map
            .update(cx, |display_map, cx| display_map.insert_blocks(blocks, cx));
        if let Some(autoscroll) = autoscroll {
            self.request_autoscroll(autoscroll, cx);
        }
        cx.notify();
        blocks
    }

    pub fn resize_blocks(
        &mut self,
        heights: HashMap<CustomBlockId, u32>,
        autoscroll: Option<Autoscroll>,
        cx: &mut ViewContext<Self>,
    ) {
        self.display_map
            .update(cx, |display_map, cx| display_map.resize_blocks(heights, cx));
        if let Some(autoscroll) = autoscroll {
            self.request_autoscroll(autoscroll, cx);
        }
        cx.notify();
    }

    pub fn replace_blocks(
        &mut self,
        renderers: HashMap<CustomBlockId, RenderBlock>,
        autoscroll: Option<Autoscroll>,
        cx: &mut ViewContext<Self>,
    ) {
        self.display_map
            .update(cx, |display_map, _cx| display_map.replace_blocks(renderers));
        if let Some(autoscroll) = autoscroll {
            self.request_autoscroll(autoscroll, cx);
        }
        cx.notify();
    }

    pub fn remove_blocks(
        &mut self,
        block_ids: HashSet<CustomBlockId>,
        autoscroll: Option<Autoscroll>,
        cx: &mut ViewContext<Self>,
    ) {
        self.display_map.update(cx, |display_map, cx| {
            display_map.remove_blocks(block_ids, cx)
        });
        if let Some(autoscroll) = autoscroll {
            self.request_autoscroll(autoscroll, cx);
        }
        cx.notify();
    }

    pub fn row_for_block(
        &self,
        block_id: CustomBlockId,
        cx: &mut ViewContext<Self>,
    ) -> Option<DisplayRow> {
        self.display_map
            .update(cx, |map, cx| map.row_for_block(block_id, cx))
    }

    pub(crate) fn set_focused_block(&mut self, focused_block: FocusedBlock) {
        self.focused_block = Some(focused_block);
    }

    pub(crate) fn take_focused_block(&mut self) -> Option<FocusedBlock> {
        self.focused_block.take()
    }

    pub fn insert_creases(
        &mut self,
        creases: impl IntoIterator<Item = Crease>,
        cx: &mut ViewContext<Self>,
    ) -> Vec<CreaseId> {
        self.display_map
            .update(cx, |map, cx| map.insert_creases(creases, cx))
    }

    pub fn remove_creases(
        &mut self,
        ids: impl IntoIterator<Item = CreaseId>,
        cx: &mut ViewContext<Self>,
    ) {
        self.display_map
            .update(cx, |map, cx| map.remove_creases(ids, cx));
    }

    pub fn longest_row(&self, cx: &mut AppContext) -> DisplayRow {
        self.display_map
            .update(cx, |map, cx| map.snapshot(cx))
            .longest_row()
    }

    pub fn max_point(&self, cx: &mut AppContext) -> DisplayPoint {
        self.display_map
            .update(cx, |map, cx| map.snapshot(cx))
            .max_point()
    }

    pub fn text(&self, cx: &AppContext) -> String {
        self.buffer.read(cx).read(cx).text()
    }

    pub fn text_option(&self, cx: &AppContext) -> Option<String> {
        let text = self.text(cx);
        let text = text.trim();

        if text.is_empty() {
            return None;
        }

        Some(text.to_string())
    }

    pub fn set_text(&mut self, text: impl Into<Arc<str>>, cx: &mut ViewContext<Self>) {
        self.transact(cx, |this, cx| {
            this.buffer
                .read(cx)
                .as_singleton()
                .expect("you can only call set_text on editors for singleton buffers")
                .update(cx, |buffer, cx| buffer.set_text(text, cx));
        });
    }

    pub fn display_text(&self, cx: &mut AppContext) -> String {
        self.display_map
            .update(cx, |map, cx| map.snapshot(cx))
            .text()
    }

    pub fn wrap_guides(&self, cx: &AppContext) -> SmallVec<[(usize, bool); 2]> {
        let mut wrap_guides = smallvec::smallvec![];

        if self.show_wrap_guides == Some(false) {
            return wrap_guides;
        }

        let settings = self.buffer.read(cx).settings_at(0, cx);
        if settings.show_wrap_guides {
            if let SoftWrap::Column(soft_wrap) = self.soft_wrap_mode(cx) {
                wrap_guides.push((soft_wrap as usize, true));
            }
            wrap_guides.extend(settings.wrap_guides.iter().map(|guide| (*guide, false)))
        }

        wrap_guides
    }

    pub fn soft_wrap_mode(&self, cx: &AppContext) -> SoftWrap {
        let settings = self.buffer.read(cx).settings_at(0, cx);
        let mode = self
            .soft_wrap_mode_override
            .unwrap_or_else(|| settings.soft_wrap);
        match mode {
            language_settings::SoftWrap::None => SoftWrap::None,
            language_settings::SoftWrap::PreferLine => SoftWrap::PreferLine,
            language_settings::SoftWrap::EditorWidth => SoftWrap::EditorWidth,
            language_settings::SoftWrap::PreferredLineLength => {
                SoftWrap::Column(settings.preferred_line_length)
            }
        }
    }

    pub fn set_soft_wrap_mode(
        &mut self,
        mode: language_settings::SoftWrap,
        cx: &mut ViewContext<Self>,
    ) {
        self.soft_wrap_mode_override = Some(mode);
        cx.notify();
    }

    pub fn set_style(&mut self, style: EditorStyle, cx: &mut ViewContext<Self>) {
        let rem_size = cx.rem_size();
        self.display_map.update(cx, |map, cx| {
            map.set_font(
                style.text.font(),
                style.text.font_size.to_pixels(rem_size),
                cx,
            )
        });
        self.style = Some(style);
    }

    pub fn style(&self) -> Option<&EditorStyle> {
        self.style.as_ref()
    }

    // Called by the element. This method is not designed to be called outside of the editor
    // element's layout code because it does not notify when rewrapping is computed synchronously.
    pub(crate) fn set_wrap_width(&self, width: Option<Pixels>, cx: &mut AppContext) -> bool {
        self.display_map
            .update(cx, |map, cx| map.set_wrap_width(width, cx))
    }

    pub fn toggle_soft_wrap(&mut self, _: &ToggleSoftWrap, cx: &mut ViewContext<Self>) {
        if self.soft_wrap_mode_override.is_some() {
            self.soft_wrap_mode_override.take();
        } else {
            let soft_wrap = match self.soft_wrap_mode(cx) {
                SoftWrap::None | SoftWrap::PreferLine => language_settings::SoftWrap::EditorWidth,
                SoftWrap::EditorWidth | SoftWrap::Column(_) => {
                    language_settings::SoftWrap::PreferLine
                }
            };
            self.soft_wrap_mode_override = Some(soft_wrap);
        }
        cx.notify();
    }

    pub fn toggle_tab_bar(&mut self, _: &ToggleTabBar, cx: &mut ViewContext<Self>) {
        let Some(workspace) = self.workspace() else {
            return;
        };
        let fs = workspace.read(cx).app_state().fs.clone();
        let current_show = TabBarSettings::get_global(cx).show;
        update_settings_file::<TabBarSettings>(fs, cx, move |setting, _| {
            setting.show = Some(!current_show);
        });
    }

    pub fn toggle_indent_guides(&mut self, _: &ToggleIndentGuides, cx: &mut ViewContext<Self>) {
        let currently_enabled = self.should_show_indent_guides().unwrap_or_else(|| {
            self.buffer
                .read(cx)
                .settings_at(0, cx)
                .indent_guides
                .enabled
        });
        self.show_indent_guides = Some(!currently_enabled);
        cx.notify();
    }

    fn should_show_indent_guides(&self) -> Option<bool> {
        self.show_indent_guides
    }

    pub fn toggle_line_numbers(&mut self, _: &ToggleLineNumbers, cx: &mut ViewContext<Self>) {
        let mut editor_settings = EditorSettings::get_global(cx).clone();
        editor_settings.gutter.line_numbers = !editor_settings.gutter.line_numbers;
        EditorSettings::override_global(editor_settings, cx);
    }

    pub fn set_show_gutter(&mut self, show_gutter: bool, cx: &mut ViewContext<Self>) {
        self.show_gutter = show_gutter;
        cx.notify();
    }

    pub fn set_show_line_numbers(&mut self, show_line_numbers: bool, cx: &mut ViewContext<Self>) {
        self.show_line_numbers = Some(show_line_numbers);
        cx.notify();
    }

    pub fn set_show_git_diff_gutter(
        &mut self,
        show_git_diff_gutter: bool,
        cx: &mut ViewContext<Self>,
    ) {
        self.show_git_diff_gutter = Some(show_git_diff_gutter);
        cx.notify();
    }

    pub fn set_show_code_actions(&mut self, show_code_actions: bool, cx: &mut ViewContext<Self>) {
        self.show_code_actions = Some(show_code_actions);
        cx.notify();
    }

    pub fn set_show_runnables(&mut self, show_runnables: bool, cx: &mut ViewContext<Self>) {
        self.show_runnables = Some(show_runnables);
        cx.notify();
    }

    pub fn set_masked(&mut self, masked: bool, cx: &mut ViewContext<Self>) {
        if self.display_map.read(cx).masked != masked {
            self.display_map.update(cx, |map, _| map.masked = masked);
        }
        cx.notify()
    }

    pub fn set_show_wrap_guides(&mut self, show_wrap_guides: bool, cx: &mut ViewContext<Self>) {
        self.show_wrap_guides = Some(show_wrap_guides);
        cx.notify();
    }

    pub fn set_show_indent_guides(&mut self, show_indent_guides: bool, cx: &mut ViewContext<Self>) {
        self.show_indent_guides = Some(show_indent_guides);
        cx.notify();
    }

    pub fn working_directory(&self, cx: &WindowContext) -> Option<PathBuf> {
        if let Some(buffer) = self.buffer().read(cx).as_singleton() {
            if let Some(file) = buffer.read(cx).file().and_then(|f| f.as_local()) {
                if let Some(dir) = file.abs_path(cx).parent() {
                    return Some(dir.to_owned());
                }
            }

            if let Some(project_path) = buffer.read(cx).project_path(cx) {
                return Some(project_path.path.to_path_buf());
            }
        }

        None
    }

    pub fn reveal_in_finder(&mut self, _: &RevealInFileManager, cx: &mut ViewContext<Self>) {
        if let Some(buffer) = self.buffer().read(cx).as_singleton() {
            if let Some(file) = buffer.read(cx).file().and_then(|f| f.as_local()) {
                cx.reveal_path(&file.abs_path(cx));
            }
        }
    }

    pub fn copy_path(&mut self, _: &CopyPath, cx: &mut ViewContext<Self>) {
        if let Some(buffer) = self.buffer().read(cx).as_singleton() {
            if let Some(file) = buffer.read(cx).file().and_then(|f| f.as_local()) {
                if let Some(path) = file.abs_path(cx).to_str() {
                    cx.write_to_clipboard(ClipboardItem::new(path.to_string()));
                }
            }
        }
    }

    pub fn copy_relative_path(&mut self, _: &CopyRelativePath, cx: &mut ViewContext<Self>) {
        if let Some(buffer) = self.buffer().read(cx).as_singleton() {
            if let Some(file) = buffer.read(cx).file().and_then(|f| f.as_local()) {
                if let Some(path) = file.path().to_str() {
                    cx.write_to_clipboard(ClipboardItem::new(path.to_string()));
                }
            }
        }
    }

    pub fn toggle_git_blame(&mut self, _: &ToggleGitBlame, cx: &mut ViewContext<Self>) {
        self.show_git_blame_gutter = !self.show_git_blame_gutter;

        if self.show_git_blame_gutter && !self.has_blame_entries(cx) {
            self.start_git_blame(true, cx);
        }

        cx.notify();
    }

    pub fn toggle_git_blame_inline(
        &mut self,
        _: &ToggleGitBlameInline,
        cx: &mut ViewContext<Self>,
    ) {
        self.toggle_git_blame_inline_internal(true, cx);
        cx.notify();
    }

    pub fn git_blame_inline_enabled(&self) -> bool {
        self.git_blame_inline_enabled
    }

    pub fn toggle_selection_menu(&mut self, _: &ToggleSelectionMenu, cx: &mut ViewContext<Self>) {
        self.show_selection_menu = self
            .show_selection_menu
            .map(|show_selections_menu| !show_selections_menu)
            .or_else(|| Some(!EditorSettings::get_global(cx).toolbar.selections_menu));

        cx.notify();
    }

    pub fn selection_menu_enabled(&self, cx: &AppContext) -> bool {
        self.show_selection_menu
            .unwrap_or_else(|| EditorSettings::get_global(cx).toolbar.selections_menu)
    }

    fn start_git_blame(&mut self, user_triggered: bool, cx: &mut ViewContext<Self>) {
        if let Some(project) = self.project.as_ref() {
            let Some(buffer) = self.buffer().read(cx).as_singleton() else {
                return;
            };

            if buffer.read(cx).file().is_none() {
                return;
            }

            let focused = self.focus_handle(cx).contains_focused(cx);

            let project = project.clone();
            let blame =
                cx.new_model(|cx| GitBlame::new(buffer, project, user_triggered, focused, cx));
            self.blame_subscription = Some(cx.observe(&blame, |_, _, cx| cx.notify()));
            self.blame = Some(blame);
        }
    }

    fn toggle_git_blame_inline_internal(
        &mut self,
        user_triggered: bool,
        cx: &mut ViewContext<Self>,
    ) {
        if self.git_blame_inline_enabled {
            self.git_blame_inline_enabled = false;
            self.show_git_blame_inline = false;
            self.show_git_blame_inline_delay_task.take();
        } else {
            self.git_blame_inline_enabled = true;
            self.start_git_blame_inline(user_triggered, cx);
        }

        cx.notify();
    }

    fn start_git_blame_inline(&mut self, user_triggered: bool, cx: &mut ViewContext<Self>) {
        self.start_git_blame(user_triggered, cx);

        if ProjectSettings::get_global(cx)
            .git
            .inline_blame_delay()
            .is_some()
        {
            self.start_inline_blame_timer(cx);
        } else {
            self.show_git_blame_inline = true
        }
    }

    pub fn blame(&self) -> Option<&Model<GitBlame>> {
        self.blame.as_ref()
    }

    pub fn render_git_blame_gutter(&mut self, cx: &mut WindowContext) -> bool {
        self.show_git_blame_gutter && self.has_blame_entries(cx)
    }

    pub fn render_git_blame_inline(&mut self, cx: &mut WindowContext) -> bool {
        self.show_git_blame_inline
            && self.focus_handle.is_focused(cx)
            && !self.newest_selection_head_on_empty_line(cx)
            && self.has_blame_entries(cx)
    }

    fn has_blame_entries(&self, cx: &mut WindowContext) -> bool {
        self.blame()
            .map_or(false, |blame| blame.read(cx).has_generated_entries())
    }

    fn newest_selection_head_on_empty_line(&mut self, cx: &mut WindowContext) -> bool {
        let cursor_anchor = self.selections.newest_anchor().head();

        let snapshot = self.buffer.read(cx).snapshot(cx);
        let buffer_row = MultiBufferRow(cursor_anchor.to_point(&snapshot).row);

        snapshot.line_len(buffer_row) == 0
    }

    fn get_permalink_to_line(&mut self, cx: &mut ViewContext<Self>) -> Result<url::Url> {
        let (path, selection, repo) = maybe!({
            let project_handle = self.project.as_ref()?.clone();
            let project = project_handle.read(cx);

            let selection = self.selections.newest::<Point>(cx);
            let selection_range = selection.range();

            let (buffer, selection) = if let Some(buffer) = self.buffer().read(cx).as_singleton() {
                (buffer, selection_range.start.row..selection_range.end.row)
            } else {
                let buffer_ranges = self
                    .buffer()
                    .read(cx)
                    .range_to_buffer_ranges(selection_range, cx);

                let (buffer, range, _) = if selection.reversed {
                    buffer_ranges.first()
                } else {
                    buffer_ranges.last()
                }?;

                let snapshot = buffer.read(cx).snapshot();
                let selection = text::ToPoint::to_point(&range.start, &snapshot).row
                    ..text::ToPoint::to_point(&range.end, &snapshot).row;
                (buffer.clone(), selection)
            };

            let path = buffer
                .read(cx)
                .file()?
                .as_local()?
                .path()
                .to_str()?
                .to_string();
            let repo = project.get_repo(&buffer.read(cx).project_path(cx)?, cx)?;
            Some((path, selection, repo))
        })
        .ok_or_else(|| anyhow!("unable to open git repository"))?;

        const REMOTE_NAME: &str = "origin";
        let origin_url = repo
            .remote_url(REMOTE_NAME)
            .ok_or_else(|| anyhow!("remote \"{REMOTE_NAME}\" not found"))?;
        let sha = repo
            .head_sha()
            .ok_or_else(|| anyhow!("failed to read HEAD SHA"))?;

        let (provider, remote) =
            parse_git_remote_url(GitHostingProviderRegistry::default_global(cx), &origin_url)
                .ok_or_else(|| anyhow!("failed to parse Git remote URL"))?;

        Ok(provider.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: &sha,
                path: &path,
                selection: Some(selection),
            },
        ))
    }

    pub fn copy_permalink_to_line(&mut self, _: &CopyPermalinkToLine, cx: &mut ViewContext<Self>) {
        let permalink = self.get_permalink_to_line(cx);

        match permalink {
            Ok(permalink) => {
                cx.write_to_clipboard(ClipboardItem::new(permalink.to_string()));
            }
            Err(err) => {
                let message = format!("Failed to copy permalink: {err}");

                Err::<(), anyhow::Error>(err).log_err();

                if let Some(workspace) = self.workspace() {
                    workspace.update(cx, |workspace, cx| {
                        struct CopyPermalinkToLine;

                        workspace.show_toast(
                            Toast::new(NotificationId::unique::<CopyPermalinkToLine>(), message),
                            cx,
                        )
                    })
                }
            }
        }
    }

    pub fn open_permalink_to_line(&mut self, _: &OpenPermalinkToLine, cx: &mut ViewContext<Self>) {
        let permalink = self.get_permalink_to_line(cx);

        match permalink {
            Ok(permalink) => {
                cx.open_url(permalink.as_ref());
            }
            Err(err) => {
                let message = format!("Failed to open permalink: {err}");

                Err::<(), anyhow::Error>(err).log_err();

                if let Some(workspace) = self.workspace() {
                    workspace.update(cx, |workspace, cx| {
                        struct OpenPermalinkToLine;

                        workspace.show_toast(
                            Toast::new(NotificationId::unique::<OpenPermalinkToLine>(), message),
                            cx,
                        )
                    })
                }
            }
        }
    }

    /// Adds or removes (on `None` color) a highlight for the rows corresponding to the anchor range given.
    /// On matching anchor range, replaces the old highlight; does not clear the other existing highlights.
    /// If multiple anchor ranges will produce highlights for the same row, the last range added will be used.
    pub fn highlight_rows<T: 'static>(
        &mut self,
        rows: RangeInclusive<Anchor>,
        color: Option<Hsla>,
        should_autoscroll: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let snapshot = self.buffer().read(cx).snapshot(cx);
        let row_highlights = self.highlighted_rows.entry(TypeId::of::<T>()).or_default();
        let existing_highlight_index = row_highlights.binary_search_by(|highlight| {
            highlight
                .range
                .start()
                .cmp(&rows.start(), &snapshot)
                .then(highlight.range.end().cmp(&rows.end(), &snapshot))
        });
        match (color, existing_highlight_index) {
            (Some(_), Ok(ix)) | (_, Err(ix)) => row_highlights.insert(
                ix,
                RowHighlight {
                    index: post_inc(&mut self.highlight_order),
                    range: rows,
                    should_autoscroll,
                    color,
                },
            ),
            (None, Ok(i)) => {
                row_highlights.remove(i);
            }
        }
    }

    /// Clear all anchor ranges for a certain highlight context type, so no corresponding rows will be highlighted.
    pub fn clear_row_highlights<T: 'static>(&mut self) {
        self.highlighted_rows.remove(&TypeId::of::<T>());
    }

    /// For a highlight given context type, gets all anchor ranges that will be used for row highlighting.
    pub fn highlighted_rows<T: 'static>(
        &self,
    ) -> Option<impl Iterator<Item = (&RangeInclusive<Anchor>, Option<&Hsla>)>> {
        Some(
            self.highlighted_rows
                .get(&TypeId::of::<T>())?
                .iter()
                .map(|highlight| (&highlight.range, highlight.color.as_ref())),
        )
    }

    /// Merges all anchor ranges for all context types ever set, picking the last highlight added in case of a row conflict.
    /// Rerturns a map of display rows that are highlighted and their corresponding highlight color.
    /// Allows to ignore certain kinds of highlights.
    pub fn highlighted_display_rows(
        &mut self,
        cx: &mut WindowContext,
    ) -> BTreeMap<DisplayRow, Hsla> {
        let snapshot = self.snapshot(cx);
        let mut used_highlight_orders = HashMap::default();
        self.highlighted_rows
            .iter()
            .flat_map(|(_, highlighted_rows)| highlighted_rows.iter())
            .fold(
                BTreeMap::<DisplayRow, Hsla>::new(),
                |mut unique_rows, highlight| {
                    let start_row = highlight.range.start().to_display_point(&snapshot).row();
                    let end_row = highlight.range.end().to_display_point(&snapshot).row();
                    for row in start_row.0..=end_row.0 {
                        let used_index =
                            used_highlight_orders.entry(row).or_insert(highlight.index);
                        if highlight.index >= *used_index {
                            *used_index = highlight.index;
                            match highlight.color {
                                Some(hsla) => unique_rows.insert(DisplayRow(row), hsla),
                                None => unique_rows.remove(&DisplayRow(row)),
                            };
                        }
                    }
                    unique_rows
                },
            )
    }

    pub fn highlighted_display_row_for_autoscroll(
        &self,
        snapshot: &DisplaySnapshot,
    ) -> Option<DisplayRow> {
        self.highlighted_rows
            .values()
            .flat_map(|highlighted_rows| highlighted_rows.iter())
            .filter_map(|highlight| {
                if highlight.color.is_none() || !highlight.should_autoscroll {
                    return None;
                }
                Some(highlight.range.start().to_display_point(&snapshot).row())
            })
            .min()
    }

    pub fn set_search_within_ranges(
        &mut self,
        ranges: &[Range<Anchor>],
        cx: &mut ViewContext<Self>,
    ) {
        self.highlight_background::<SearchWithinRange>(
            ranges,
            |colors| colors.editor_document_highlight_read_background,
            cx,
        )
    }

    pub fn set_breadcrumb_header(&mut self, new_header: String) {
        self.breadcrumb_header = Some(new_header);
    }

    pub fn clear_search_within_ranges(&mut self, cx: &mut ViewContext<Self>) {
        self.clear_background_highlights::<SearchWithinRange>(cx);
    }

    pub fn highlight_background<T: 'static>(
        &mut self,
        ranges: &[Range<Anchor>],
        color_fetcher: fn(&ThemeColors) -> Hsla,
        cx: &mut ViewContext<Self>,
    ) {
        self.background_highlights
            .insert(TypeId::of::<T>(), (color_fetcher, Arc::from(ranges)));
        self.scrollbar_marker_state.dirty = true;
        cx.notify();
    }

    pub fn clear_background_highlights<T: 'static>(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> Option<BackgroundHighlight> {
        let text_highlights = self.background_highlights.remove(&TypeId::of::<T>())?;
        if !text_highlights.1.is_empty() {
            self.scrollbar_marker_state.dirty = true;
            cx.notify();
        }
        Some(text_highlights)
    }

    pub fn highlight_gutter<T: 'static>(
        &mut self,
        ranges: &[Range<Anchor>],
        color_fetcher: fn(&AppContext) -> Hsla,
        cx: &mut ViewContext<Self>,
    ) {
        self.gutter_highlights
            .insert(TypeId::of::<T>(), (color_fetcher, Arc::from(ranges)));
        cx.notify();
    }

    pub fn clear_gutter_highlights<T: 'static>(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> Option<GutterHighlight> {
        cx.notify();
        self.gutter_highlights.remove(&TypeId::of::<T>())
    }

    #[cfg(feature = "test-support")]
    pub fn all_text_background_highlights(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> Vec<(Range<DisplayPoint>, Hsla)> {
        let snapshot = self.snapshot(cx);
        let buffer = &snapshot.buffer_snapshot;
        let start = buffer.anchor_before(0);
        let end = buffer.anchor_after(buffer.len());
        let theme = cx.theme().colors();
        self.background_highlights_in_range(start..end, &snapshot, theme)
    }

    #[cfg(feature = "test-support")]
    pub fn search_background_highlights(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> Vec<Range<Point>> {
        let snapshot = self.buffer().read(cx).snapshot(cx);

        let highlights = self
            .background_highlights
            .get(&TypeId::of::<items::BufferSearchHighlights>());

        if let Some((_color, ranges)) = highlights {
            ranges
                .iter()
                .map(|range| range.start.to_point(&snapshot)..range.end.to_point(&snapshot))
                .collect_vec()
        } else {
            vec![]
        }
    }

    fn document_highlights_for_position<'a>(
        &'a self,
        position: Anchor,
        buffer: &'a MultiBufferSnapshot,
    ) -> impl 'a + Iterator<Item = &Range<Anchor>> {
        let read_highlights = self
            .background_highlights
            .get(&TypeId::of::<DocumentHighlightRead>())
            .map(|h| &h.1);
        let write_highlights = self
            .background_highlights
            .get(&TypeId::of::<DocumentHighlightWrite>())
            .map(|h| &h.1);
        let left_position = position.bias_left(buffer);
        let right_position = position.bias_right(buffer);
        read_highlights
            .into_iter()
            .chain(write_highlights)
            .flat_map(move |ranges| {
                let start_ix = match ranges.binary_search_by(|probe| {
                    let cmp = probe.end.cmp(&left_position, buffer);
                    if cmp.is_ge() {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }) {
                    Ok(i) | Err(i) => i,
                };

                ranges[start_ix..]
                    .iter()
                    .take_while(move |range| range.start.cmp(&right_position, buffer).is_le())
            })
    }

    pub fn has_background_highlights<T: 'static>(&self) -> bool {
        self.background_highlights
            .get(&TypeId::of::<T>())
            .map_or(false, |(_, highlights)| !highlights.is_empty())
    }

    pub fn background_highlights_in_range(
        &self,
        search_range: Range<Anchor>,
        display_snapshot: &DisplaySnapshot,
        theme: &ThemeColors,
    ) -> Vec<(Range<DisplayPoint>, Hsla)> {
        let mut results = Vec::new();
        for (color_fetcher, ranges) in self.background_highlights.values() {
            let color = color_fetcher(theme);
            let start_ix = match ranges.binary_search_by(|probe| {
                let cmp = probe
                    .end
                    .cmp(&search_range.start, &display_snapshot.buffer_snapshot);
                if cmp.is_gt() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }) {
                Ok(i) | Err(i) => i,
            };
            for range in &ranges[start_ix..] {
                if range
                    .start
                    .cmp(&search_range.end, &display_snapshot.buffer_snapshot)
                    .is_ge()
                {
                    break;
                }

                let start = range.start.to_display_point(&display_snapshot);
                let end = range.end.to_display_point(&display_snapshot);
                results.push((start..end, color))
            }
        }
        results
    }

    pub fn background_highlight_row_ranges<T: 'static>(
        &self,
        search_range: Range<Anchor>,
        display_snapshot: &DisplaySnapshot,
        count: usize,
    ) -> Vec<RangeInclusive<DisplayPoint>> {
        let mut results = Vec::new();
        let Some((_, ranges)) = self.background_highlights.get(&TypeId::of::<T>()) else {
            return vec![];
        };

        let start_ix = match ranges.binary_search_by(|probe| {
            let cmp = probe
                .end
                .cmp(&search_range.start, &display_snapshot.buffer_snapshot);
            if cmp.is_gt() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }) {
            Ok(i) | Err(i) => i,
        };
        let mut push_region = |start: Option<Point>, end: Option<Point>| {
            if let (Some(start_display), Some(end_display)) = (start, end) {
                results.push(
                    start_display.to_display_point(display_snapshot)
                        ..=end_display.to_display_point(display_snapshot),
                );
            }
        };
        let mut start_row: Option<Point> = None;
        let mut end_row: Option<Point> = None;
        if ranges.len() > count {
            return Vec::new();
        }
        for range in &ranges[start_ix..] {
            if range
                .start
                .cmp(&search_range.end, &display_snapshot.buffer_snapshot)
                .is_ge()
            {
                break;
            }
            let end = range.end.to_point(&display_snapshot.buffer_snapshot);
            if let Some(current_row) = &end_row {
                if end.row == current_row.row {
                    continue;
                }
            }
            let start = range.start.to_point(&display_snapshot.buffer_snapshot);
            if start_row.is_none() {
                assert_eq!(end_row, None);
                start_row = Some(start);
                end_row = Some(end);
                continue;
            }
            if let Some(current_end) = end_row.as_mut() {
                if start.row > current_end.row + 1 {
                    push_region(start_row, end_row);
                    start_row = Some(start);
                    end_row = Some(end);
                } else {
                    // Merge two hunks.
                    *current_end = end;
                }
            } else {
                unreachable!();
            }
        }
        // We might still have a hunk that was not rendered (if there was a search hit on the last line)
        push_region(start_row, end_row);
        results
    }

    pub fn gutter_highlights_in_range(
        &self,
        search_range: Range<Anchor>,
        display_snapshot: &DisplaySnapshot,
        cx: &AppContext,
    ) -> Vec<(Range<DisplayPoint>, Hsla)> {
        let mut results = Vec::new();
        for (color_fetcher, ranges) in self.gutter_highlights.values() {
            let color = color_fetcher(cx);
            let start_ix = match ranges.binary_search_by(|probe| {
                let cmp = probe
                    .end
                    .cmp(&search_range.start, &display_snapshot.buffer_snapshot);
                if cmp.is_gt() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }) {
                Ok(i) | Err(i) => i,
            };
            for range in &ranges[start_ix..] {
                if range
                    .start
                    .cmp(&search_range.end, &display_snapshot.buffer_snapshot)
                    .is_ge()
                {
                    break;
                }

                let start = range.start.to_display_point(&display_snapshot);
                let end = range.end.to_display_point(&display_snapshot);
                results.push((start..end, color))
            }
        }
        results
    }

    /// Get the text ranges corresponding to the redaction query
    pub fn redacted_ranges(
        &self,
        search_range: Range<Anchor>,
        display_snapshot: &DisplaySnapshot,
        cx: &WindowContext,
    ) -> Vec<Range<DisplayPoint>> {
        display_snapshot
            .buffer_snapshot
            .redacted_ranges(search_range, |file| {
                if let Some(file) = file {
                    file.is_private()
                        && EditorSettings::get(Some(file.as_ref().into()), cx).redact_private_values
                } else {
                    false
                }
            })
            .map(|range| {
                range.start.to_display_point(display_snapshot)
                    ..range.end.to_display_point(display_snapshot)
            })
            .collect()
    }

    pub fn highlight_text<T: 'static>(
        &mut self,
        ranges: Vec<Range<Anchor>>,
        style: HighlightStyle,
        cx: &mut ViewContext<Self>,
    ) {
        self.display_map.update(cx, |map, _| {
            map.highlight_text(TypeId::of::<T>(), ranges, style)
        });
        cx.notify();
    }

    pub(crate) fn highlight_inlays<T: 'static>(
        &mut self,
        highlights: Vec<InlayHighlight>,
        style: HighlightStyle,
        cx: &mut ViewContext<Self>,
    ) {
        self.display_map.update(cx, |map, _| {
            map.highlight_inlays(TypeId::of::<T>(), highlights, style)
        });
        cx.notify();
    }

    pub fn text_highlights<'a, T: 'static>(
        &'a self,
        cx: &'a AppContext,
    ) -> Option<(HighlightStyle, &'a [Range<Anchor>])> {
        self.display_map.read(cx).text_highlights(TypeId::of::<T>())
    }

    pub fn clear_highlights<T: 'static>(&mut self, cx: &mut ViewContext<Self>) {
        let cleared = self
            .display_map
            .update(cx, |map, _| map.clear_highlights(TypeId::of::<T>()));
        if cleared {
            cx.notify();
        }
    }

    pub fn show_local_cursors(&self, cx: &WindowContext) -> bool {
        (self.read_only(cx) || self.blink_manager.read(cx).visible())
            && self.focus_handle.is_focused(cx)
    }

    pub fn set_show_cursor_when_unfocused(&mut self, is_enabled: bool, cx: &mut ViewContext<Self>) {
        self.show_cursor_when_unfocused = is_enabled;
        cx.notify();
    }

    fn on_buffer_changed(&mut self, _: Model<MultiBuffer>, cx: &mut ViewContext<Self>) {
        cx.notify();
    }

    fn on_buffer_event(
        &mut self,
        multibuffer: Model<MultiBuffer>,
        event: &multi_buffer::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            multi_buffer::Event::Edited {
                singleton_buffer_edited,
            } => {
                self.scrollbar_marker_state.dirty = true;
                self.active_indent_guides_state.dirty = true;
                self.refresh_active_diagnostics(cx);
                self.refresh_code_actions(cx);
                if self.has_active_inline_completion(cx) {
                    self.update_visible_inline_completion(cx);
                }
                cx.emit(EditorEvent::BufferEdited);
                cx.emit(SearchEvent::MatchesInvalidated);
                if *singleton_buffer_edited {
                    if let Some(project) = &self.project {
                        let project = project.read(cx);
                        #[allow(clippy::mutable_key_type)]
                        let languages_affected = multibuffer
                            .read(cx)
                            .all_buffers()
                            .into_iter()
                            .filter_map(|buffer| {
                                let buffer = buffer.read(cx);
                                let language = buffer.language()?;
                                if project.is_local()
                                    && project.language_servers_for_buffer(buffer, cx).count() == 0
                                {
                                    None
                                } else {
                                    Some(language)
                                }
                            })
                            .cloned()
                            .collect::<HashSet<_>>();
                        if !languages_affected.is_empty() {
                            self.refresh_inlay_hints(
                                InlayHintRefreshReason::BufferEdited(languages_affected),
                                cx,
                            );
                        }
                    }
                }

                let Some(project) = &self.project else { return };
                let telemetry = project.read(cx).client().telemetry().clone();
                refresh_linked_ranges(self, cx);
                telemetry.log_edit_event("editor");
            }
            multi_buffer::Event::ExcerptsAdded {
                buffer,
                predecessor,
                excerpts,
            } => {
                self.tasks_update_task = Some(self.refresh_runnables(cx));
                cx.emit(EditorEvent::ExcerptsAdded {
                    buffer: buffer.clone(),
                    predecessor: *predecessor,
                    excerpts: excerpts.clone(),
                });
                self.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
            }
            multi_buffer::Event::ExcerptsRemoved { ids } => {
                self.refresh_inlay_hints(InlayHintRefreshReason::ExcerptsRemoved(ids.clone()), cx);
                cx.emit(EditorEvent::ExcerptsRemoved { ids: ids.clone() })
            }
            multi_buffer::Event::ExcerptsEdited { ids } => {
                cx.emit(EditorEvent::ExcerptsEdited { ids: ids.clone() })
            }
            multi_buffer::Event::ExcerptsExpanded { ids } => {
                cx.emit(EditorEvent::ExcerptsExpanded { ids: ids.clone() })
            }
            multi_buffer::Event::Reparsed(buffer_id) => {
                self.tasks_update_task = Some(self.refresh_runnables(cx));

                cx.emit(EditorEvent::Reparsed(*buffer_id));
            }
            multi_buffer::Event::LanguageChanged(buffer_id) => {
                linked_editing_ranges::refresh_linked_ranges(self, cx);
                cx.emit(EditorEvent::Reparsed(*buffer_id));
                cx.notify();
            }
            multi_buffer::Event::DirtyChanged => cx.emit(EditorEvent::DirtyChanged),
            multi_buffer::Event::Saved => cx.emit(EditorEvent::Saved),
            multi_buffer::Event::FileHandleChanged | multi_buffer::Event::Reloaded => {
                cx.emit(EditorEvent::TitleChanged)
            }
            multi_buffer::Event::DiffBaseChanged => {
                self.scrollbar_marker_state.dirty = true;
                cx.emit(EditorEvent::DiffBaseChanged);
                cx.notify();
            }
            multi_buffer::Event::DiffUpdated { buffer } => {
                self.sync_expanded_diff_hunks(buffer.clone(), cx);
                cx.notify();
            }
            multi_buffer::Event::Closed => cx.emit(EditorEvent::Closed),
            multi_buffer::Event::DiagnosticsUpdated => {
                self.refresh_active_diagnostics(cx);
                self.scrollbar_marker_state.dirty = true;
                cx.notify();
            }
            _ => {}
        };
    }

    fn on_display_map_changed(&mut self, _: Model<DisplayMap>, cx: &mut ViewContext<Self>) {
        cx.notify();
    }

    fn settings_changed(&mut self, cx: &mut ViewContext<Self>) {
        self.tasks_update_task = Some(self.refresh_runnables(cx));
        self.refresh_inline_completion(true, cx);
        self.refresh_inlay_hints(
            InlayHintRefreshReason::SettingsChange(inlay_hint_settings(
                self.selections.newest_anchor().head(),
                &self.buffer.read(cx).snapshot(cx),
                cx,
            )),
            cx,
        );
        let editor_settings = EditorSettings::get_global(cx);
        self.scroll_manager.vertical_scroll_margin = editor_settings.vertical_scroll_margin;
        self.show_breadcrumbs = editor_settings.toolbar.breadcrumbs;

        let project_settings = ProjectSettings::get_global(cx);
        self.serialize_dirty_buffers = project_settings.session.restore_unsaved_buffers;

        if self.mode == EditorMode::Full {
            let inline_blame_enabled = project_settings.git.inline_blame_enabled();
            if self.git_blame_inline_enabled != inline_blame_enabled {
                self.toggle_git_blame_inline_internal(false, cx);
            }
        }

        cx.notify();
    }

    pub fn set_searchable(&mut self, searchable: bool) {
        self.searchable = searchable;
    }

    pub fn searchable(&self) -> bool {
        self.searchable
    }

    fn open_excerpts_in_split(&mut self, _: &OpenExcerptsSplit, cx: &mut ViewContext<Self>) {
        self.open_excerpts_common(true, cx)
    }

    fn open_excerpts(&mut self, _: &OpenExcerpts, cx: &mut ViewContext<Self>) {
        self.open_excerpts_common(false, cx)
    }

    fn open_excerpts_common(&mut self, split: bool, cx: &mut ViewContext<Self>) {
        let buffer = self.buffer.read(cx);
        if buffer.is_singleton() {
            cx.propagate();
            return;
        }

        let Some(workspace) = self.workspace() else {
            cx.propagate();
            return;
        };

        let mut new_selections_by_buffer = HashMap::default();
        for selection in self.selections.all::<usize>(cx) {
            for (buffer, mut range, _) in
                buffer.range_to_buffer_ranges(selection.start..selection.end, cx)
            {
                if selection.reversed {
                    mem::swap(&mut range.start, &mut range.end);
                }
                new_selections_by_buffer
                    .entry(buffer)
                    .or_insert(Vec::new())
                    .push(range)
            }
        }

        // We defer the pane interaction because we ourselves are a workspace item
        // and activating a new item causes the pane to call a method on us reentrantly,
        // which panics if we're on the stack.
        cx.window_context().defer(move |cx| {
            workspace.update(cx, |workspace, cx| {
                let pane = if split {
                    workspace.adjacent_pane(cx)
                } else {
                    workspace.active_pane().clone()
                };

                for (buffer, ranges) in new_selections_by_buffer {
                    let editor =
                        workspace.open_project_item::<Self>(pane.clone(), buffer, true, true, cx);
                    editor.update(cx, |editor, cx| {
                        editor.change_selections(Some(Autoscroll::newest()), cx, |s| {
                            s.select_ranges(ranges);
                        });
                    });
                }
            })
        });
    }

    fn jump(
        &mut self,
        path: ProjectPath,
        position: Point,
        anchor: language::Anchor,
        offset_from_top: u32,
        cx: &mut ViewContext<Self>,
    ) {
        let workspace = self.workspace();
        cx.spawn(|_, mut cx| async move {
            let workspace = workspace.ok_or_else(|| anyhow!("cannot jump without workspace"))?;
            let editor = workspace.update(&mut cx, |workspace, cx| {
                // Reset the preview item id before opening the new item
                workspace.active_pane().update(cx, |pane, cx| {
                    pane.set_preview_item_id(None, cx);
                });
                workspace.open_path_preview(path, None, true, true, cx)
            })?;
            let editor = editor
                .await?
                .downcast::<Editor>()
                .ok_or_else(|| anyhow!("opened item was not an editor"))?
                .downgrade();
            editor.update(&mut cx, |editor, cx| {
                let buffer = editor
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .ok_or_else(|| anyhow!("cannot jump in a multi-buffer"))?;
                let buffer = buffer.read(cx);
                let cursor = if buffer.can_resolve(&anchor) {
                    language::ToPoint::to_point(&anchor, buffer)
                } else {
                    buffer.clip_point(position, Bias::Left)
                };

                let nav_history = editor.nav_history.take();
                editor.change_selections(
                    Some(Autoscroll::top_relative(offset_from_top as usize)),
                    cx,
                    |s| {
                        s.select_ranges([cursor..cursor]);
                    },
                );
                editor.nav_history = nav_history;

                anyhow::Ok(())
            })??;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn marked_text_ranges(&self, cx: &AppContext) -> Option<Vec<Range<OffsetUtf16>>> {
        let snapshot = self.buffer.read(cx).read(cx);
        let (_, ranges) = self.text_highlights::<InputComposition>(cx)?;
        Some(
            ranges
                .iter()
                .map(move |range| {
                    range.start.to_offset_utf16(&snapshot)..range.end.to_offset_utf16(&snapshot)
                })
                .collect(),
        )
    }

    fn selection_replacement_ranges(
        &self,
        range: Range<OffsetUtf16>,
        cx: &AppContext,
    ) -> Vec<Range<OffsetUtf16>> {
        let selections = self.selections.all::<OffsetUtf16>(cx);
        let newest_selection = selections
            .iter()
            .max_by_key(|selection| selection.id)
            .unwrap();
        let start_delta = range.start.0 as isize - newest_selection.start.0 as isize;
        let end_delta = range.end.0 as isize - newest_selection.end.0 as isize;
        let snapshot = self.buffer.read(cx).read(cx);
        selections
            .into_iter()
            .map(|mut selection| {
                selection.start.0 =
                    (selection.start.0 as isize).saturating_add(start_delta) as usize;
                selection.end.0 = (selection.end.0 as isize).saturating_add(end_delta) as usize;
                snapshot.clip_offset_utf16(selection.start, Bias::Left)
                    ..snapshot.clip_offset_utf16(selection.end, Bias::Right)
            })
            .collect()
    }

    fn report_editor_event(
        &self,
        operation: &'static str,
        file_extension: Option<String>,
        cx: &AppContext,
    ) {
        if cfg!(any(test, feature = "test-support")) {
            return;
        }

        let Some(project) = &self.project else { return };

        // If None, we are in a file without an extension
        let file = self
            .buffer
            .read(cx)
            .as_singleton()
            .and_then(|b| b.read(cx).file());
        let file_extension = file_extension.or(file
            .as_ref()
            .and_then(|file| Path::new(file.file_name(cx)).extension())
            .and_then(|e| e.to_str())
            .map(|a| a.to_string()));

        let vim_mode = cx
            .global::<SettingsStore>()
            .raw_user_settings()
            .get("vim_mode")
            == Some(&serde_json::Value::Bool(true));

        let copilot_enabled = all_language_settings(file, cx).inline_completions.provider
            == language::language_settings::InlineCompletionProvider::Copilot;
        let copilot_enabled_for_language = self
            .buffer
            .read(cx)
            .settings_at(0, cx)
            .show_inline_completions;

        let telemetry = project.read(cx).client().telemetry().clone();
        telemetry.report_editor_event(
            file_extension,
            vim_mode,
            operation,
            copilot_enabled,
            copilot_enabled_for_language,
        )
    }

    /// Copy the highlighted chunks to the clipboard as JSON. The format is an array of lines,
    /// with each line being an array of {text, highlight} objects.
    fn copy_highlight_json(&mut self, _: &CopyHighlightJson, cx: &mut ViewContext<Self>) {
        let Some(buffer) = self.buffer.read(cx).as_singleton() else {
            return;
        };

        #[derive(Serialize)]
        struct Chunk<'a> {
            text: String,
            highlight: Option<&'a str>,
        }

        let snapshot = buffer.read(cx).snapshot();
        let range = self
            .selected_text_range(cx)
            .and_then(|selected_range| {
                if selected_range.is_empty() {
                    None
                } else {
                    Some(selected_range)
                }
            })
            .unwrap_or_else(|| 0..snapshot.len());

        let chunks = snapshot.chunks(range, true);
        let mut lines = Vec::new();
        let mut line: VecDeque<Chunk> = VecDeque::new();

        let Some(style) = self.style.as_ref() else {
            return;
        };

        for chunk in chunks {
            let highlight = chunk
                .syntax_highlight_id
                .and_then(|id| id.name(&style.syntax));
            let mut chunk_lines = chunk.text.split('\n').peekable();
            while let Some(text) = chunk_lines.next() {
                let mut merged_with_last_token = false;
                if let Some(last_token) = line.back_mut() {
                    if last_token.highlight == highlight {
                        last_token.text.push_str(text);
                        merged_with_last_token = true;
                    }
                }

                if !merged_with_last_token {
                    line.push_back(Chunk {
                        text: text.into(),
                        highlight,
                    });
                }

                if chunk_lines.peek().is_some() {
                    if line.len() > 1 && line.front().unwrap().text.is_empty() {
                        line.pop_front();
                    }
                    if line.len() > 1 && line.back().unwrap().text.is_empty() {
                        line.pop_back();
                    }

                    lines.push(mem::take(&mut line));
                }
            }
        }

        let Some(lines) = serde_json::to_string_pretty(&lines).log_err() else {
            return;
        };
        cx.write_to_clipboard(ClipboardItem::new(lines));
    }

    pub fn inlay_hint_cache(&self) -> &InlayHintCache {
        &self.inlay_hint_cache
    }

    pub fn replay_insert_event(
        &mut self,
        text: &str,
        relative_utf16_range: Option<Range<isize>>,
        cx: &mut ViewContext<Self>,
    ) {
        if !self.input_enabled {
            cx.emit(EditorEvent::InputIgnored { text: text.into() });
            return;
        }
        if let Some(relative_utf16_range) = relative_utf16_range {
            let selections = self.selections.all::<OffsetUtf16>(cx);
            self.change_selections(None, cx, |s| {
                let new_ranges = selections.into_iter().map(|range| {
                    let start = OffsetUtf16(
                        range
                            .head()
                            .0
                            .saturating_add_signed(relative_utf16_range.start),
                    );
                    let end = OffsetUtf16(
                        range
                            .head()
                            .0
                            .saturating_add_signed(relative_utf16_range.end),
                    );
                    start..end
                });
                s.select_ranges(new_ranges);
            });
        }

        self.handle_input(text, cx);
    }

    pub fn supports_inlay_hints(&self, cx: &AppContext) -> bool {
        let Some(project) = self.project.as_ref() else {
            return false;
        };
        let project = project.read(cx);

        let mut supports = false;
        self.buffer().read(cx).for_each_buffer(|buffer| {
            if !supports {
                supports = project
                    .language_servers_for_buffer(buffer.read(cx), cx)
                    .any(
                        |(_, server)| match server.capabilities().inlay_hint_provider {
                            Some(lsp::OneOf::Left(enabled)) => enabled,
                            Some(lsp::OneOf::Right(_)) => true,
                            None => false,
                        },
                    )
            }
        });
        supports
    }

    pub fn focus(&self, cx: &mut WindowContext) {
        cx.focus(&self.focus_handle)
    }

    pub fn is_focused(&self, cx: &WindowContext) -> bool {
        self.focus_handle.is_focused(cx)
    }

    fn handle_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(EditorEvent::Focused);

        if let Some(descendant) = self
            .last_focused_descendant
            .take()
            .and_then(|descendant| descendant.upgrade())
        {
            cx.focus(&descendant);
        } else {
            if let Some(blame) = self.blame.as_ref() {
                blame.update(cx, GitBlame::focus)
            }

            self.blink_manager.update(cx, BlinkManager::enable);
            self.show_cursor_names(cx);
            self.buffer.update(cx, |buffer, cx| {
                buffer.finalize_last_transaction(cx);
                if self.leader_peer_id.is_none() {
                    buffer.set_active_selections(
                        &self.selections.disjoint_anchors(),
                        self.selections.line_mode,
                        self.cursor_shape,
                        cx,
                    );
                }
            });
        }
    }

    fn handle_focus_in(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(EditorEvent::FocusedIn)
    }

    fn handle_focus_out(&mut self, event: FocusOutEvent, _cx: &mut ViewContext<Self>) {
        if event.blurred != self.focus_handle {
            self.last_focused_descendant = Some(event.blurred);
        }
    }

    pub fn handle_blur(&mut self, cx: &mut ViewContext<Self>) {
        self.blink_manager.update(cx, BlinkManager::disable);
        self.buffer
            .update(cx, |buffer, cx| buffer.remove_active_selections(cx));

        if let Some(blame) = self.blame.as_ref() {
            blame.update(cx, GitBlame::blur)
        }
        if !self.hover_state.focused(cx) {
            hide_hover(self, cx);
        }

        self.hide_context_menu(cx);
        cx.emit(EditorEvent::Blurred);
        cx.notify();
    }

    pub fn register_action<A: Action>(
        &mut self,
        listener: impl Fn(&A, &mut WindowContext) + 'static,
    ) -> Subscription {
        let id = self.next_editor_action_id.post_inc();
        let listener = Arc::new(listener);
        self.editor_actions.borrow_mut().insert(
            id,
            Box::new(move |cx| {
                let _view = cx.view().clone();
                let cx = cx.window_context();
                let listener = listener.clone();
                cx.on_action(TypeId::of::<A>(), move |action, phase, cx| {
                    let action = action.downcast_ref().unwrap();
                    if phase == DispatchPhase::Bubble {
                        listener(action, cx)
                    }
                })
            }),
        );

        let editor_actions = self.editor_actions.clone();
        Subscription::new(move || {
            editor_actions.borrow_mut().remove(&id);
        })
    }

    pub fn file_header_size(&self) -> u32 {
        self.file_header_size
    }

    pub fn revert(
        &mut self,
        revert_changes: HashMap<BufferId, Vec<(Range<text::Anchor>, Rope)>>,
        cx: &mut ViewContext<Self>,
    ) {
        self.buffer().update(cx, |multi_buffer, cx| {
            for (buffer_id, changes) in revert_changes {
                if let Some(buffer) = multi_buffer.buffer(buffer_id) {
                    buffer.update(cx, |buffer, cx| {
                        buffer.edit(
                            changes.into_iter().map(|(range, text)| {
                                (range, text.to_string().map(Arc::<str>::from))
                            }),
                            None,
                            cx,
                        );
                    });
                }
            }
        });
        self.change_selections(None, cx, |selections| selections.refresh());
    }

    pub fn to_pixel_point(
        &mut self,
        source: multi_buffer::Anchor,
        editor_snapshot: &EditorSnapshot,
        cx: &mut ViewContext<Self>,
    ) -> Option<gpui::Point<Pixels>> {
        let source_point = source.to_display_point(editor_snapshot);
        self.display_to_pixel_point(source_point, editor_snapshot, cx)
    }

    pub fn display_to_pixel_point(
        &mut self,
        source: DisplayPoint,
        editor_snapshot: &EditorSnapshot,
        cx: &mut ViewContext<Self>,
    ) -> Option<gpui::Point<Pixels>> {
        let line_height = self.style()?.text.line_height_in_pixels(cx.rem_size());
        let text_layout_details = self.text_layout_details(cx);
        let scroll_top = text_layout_details
            .scroll_anchor
            .scroll_position(editor_snapshot)
            .y;

        if source.row().as_f32() < scroll_top.floor() {
            return None;
        }
        let source_x = editor_snapshot.x_for_display_point(source, &text_layout_details);
        let source_y = line_height * (source.row().as_f32() - scroll_top);
        Some(gpui::Point::new(source_x, source_y))
    }

    fn gutter_bounds(&self) -> Option<Bounds<Pixels>> {
        let bounds = self.last_bounds?;
        Some(element::gutter_bounds(bounds, self.gutter_dimensions))
    }
}

fn hunks_for_selections(
    multi_buffer_snapshot: &MultiBufferSnapshot,
    selections: &[Selection<Anchor>],
) -> Vec<DiffHunk<MultiBufferRow>> {
    let buffer_rows_for_selections = selections.iter().map(|selection| {
        let head = selection.head();
        let tail = selection.tail();
        let start = MultiBufferRow(tail.to_point(&multi_buffer_snapshot).row);
        let end = MultiBufferRow(head.to_point(&multi_buffer_snapshot).row);
        if start > end {
            end..start
        } else {
            start..end
        }
    });

    hunks_for_rows(buffer_rows_for_selections, multi_buffer_snapshot)
}

pub fn hunks_for_rows(
    rows: impl Iterator<Item = Range<MultiBufferRow>>,
    multi_buffer_snapshot: &MultiBufferSnapshot,
) -> Vec<DiffHunk<MultiBufferRow>> {
    let mut hunks = Vec::new();
    let mut processed_buffer_rows: HashMap<BufferId, HashSet<Range<text::Anchor>>> =
        HashMap::default();
    for selected_multi_buffer_rows in rows {
        let query_rows =
            selected_multi_buffer_rows.start..selected_multi_buffer_rows.end.next_row();
        for hunk in multi_buffer_snapshot.git_diff_hunks_in_range(query_rows.clone()) {
            // Deleted hunk is an empty row range, no caret can be placed there and Zed allows to revert it
            // when the caret is just above or just below the deleted hunk.
            let allow_adjacent = hunk_status(&hunk) == DiffHunkStatus::Removed;
            let related_to_selection = if allow_adjacent {
                hunk.associated_range.overlaps(&query_rows)
                    || hunk.associated_range.start == query_rows.end
                    || hunk.associated_range.end == query_rows.start
            } else {
                // `selected_multi_buffer_rows` are inclusive (e.g. [2..2] means 2nd row is selected)
                // `hunk.associated_range` is exclusive (e.g. [2..3] means 2nd row is selected)
                hunk.associated_range.overlaps(&selected_multi_buffer_rows)
                    || selected_multi_buffer_rows.end == hunk.associated_range.start
            };
            if related_to_selection {
                if !processed_buffer_rows
                    .entry(hunk.buffer_id)
                    .or_default()
                    .insert(hunk.buffer_range.start..hunk.buffer_range.end)
                {
                    continue;
                }
                hunks.push(hunk);
            }
        }
    }

    hunks
}

pub trait CollaborationHub {
    fn collaborators<'a>(&self, cx: &'a AppContext) -> &'a HashMap<PeerId, Collaborator>;
    fn user_participant_indices<'a>(
        &self,
        cx: &'a AppContext,
    ) -> &'a HashMap<u64, ParticipantIndex>;
    fn user_names(&self, cx: &AppContext) -> HashMap<u64, SharedString>;
}

impl CollaborationHub for Model<Project> {
    fn collaborators<'a>(&self, cx: &'a AppContext) -> &'a HashMap<PeerId, Collaborator> {
        self.read(cx).collaborators()
    }

    fn user_participant_indices<'a>(
        &self,
        cx: &'a AppContext,
    ) -> &'a HashMap<u64, ParticipantIndex> {
        self.read(cx).user_store().read(cx).participant_indices()
    }

    fn user_names(&self, cx: &AppContext) -> HashMap<u64, SharedString> {
        let this = self.read(cx);
        let user_ids = this.collaborators().values().map(|c| c.user_id);
        this.user_store().read_with(cx, |user_store, cx| {
            user_store.participant_names(user_ids, cx)
        })
    }
}

pub trait CompletionProvider {
    fn completions(
        &self,
        buffer: &Model<Buffer>,
        buffer_position: text::Anchor,
        trigger: CompletionContext,
        cx: &mut ViewContext<Editor>,
    ) -> Task<Result<Vec<Completion>>>;

    fn resolve_completions(
        &self,
        buffer: Model<Buffer>,
        completion_indices: Vec<usize>,
        completions: Arc<RwLock<Box<[Completion]>>>,
        cx: &mut ViewContext<Editor>,
    ) -> Task<Result<bool>>;

    fn apply_additional_edits_for_completion(
        &self,
        buffer: Model<Buffer>,
        completion: Completion,
        push_to_history: bool,
        cx: &mut ViewContext<Editor>,
    ) -> Task<Result<Option<language::Transaction>>>;

    fn is_completion_trigger(
        &self,
        buffer: &Model<Buffer>,
        position: language::Anchor,
        text: &str,
        trigger_in_words: bool,
        cx: &mut ViewContext<Editor>,
    ) -> bool;
}

fn snippet_completions(
    project: &Project,
    buffer: &Model<Buffer>,
    buffer_position: text::Anchor,
    cx: &mut AppContext,
) -> Vec<Completion> {
    let language = buffer.read(cx).language_at(buffer_position);
    let language_name = language.as_ref().map(|language| language.lsp_id());
    let snippet_store = project.snippets().read(cx);
    let snippets = snippet_store.snippets_for(language_name, cx);

    if snippets.is_empty() {
        return vec![];
    }
    let snapshot = buffer.read(cx).text_snapshot();
    let chunks = snapshot.reversed_chunks_in_range(text::Anchor::MIN..buffer_position);

    let mut lines = chunks.lines();
    let Some(line_at) = lines.next().filter(|line| !line.is_empty()) else {
        return vec![];
    };

    let scope = language.map(|language| language.default_scope());
    let mut last_word = line_at
        .chars()
        .rev()
        .take_while(|c| char_kind(&scope, *c) == CharKind::Word)
        .collect::<String>();
    last_word = last_word.chars().rev().collect();
    let as_offset = text::ToOffset::to_offset(&buffer_position, &snapshot);
    let to_lsp = |point: &text::Anchor| {
        let end = text::ToPointUtf16::to_point_utf16(point, &snapshot);
        point_to_lsp(end)
    };
    let lsp_end = to_lsp(&buffer_position);
    snippets
        .into_iter()
        .filter_map(|snippet| {
            let matching_prefix = snippet
                .prefix
                .iter()
                .find(|prefix| prefix.starts_with(&last_word))?;
            let start = as_offset - last_word.len();
            let start = snapshot.anchor_before(start);
            let range = start..buffer_position;
            let lsp_start = to_lsp(&start);
            let lsp_range = lsp::Range {
                start: lsp_start,
                end: lsp_end,
            };
            Some(Completion {
                old_range: range,
                new_text: snippet.body.clone(),
                label: CodeLabel {
                    text: matching_prefix.clone(),
                    runs: vec![],
                    filter_range: 0..matching_prefix.len(),
                },
                server_id: LanguageServerId(usize::MAX),
                documentation: snippet
                    .description
                    .clone()
                    .map(|description| Documentation::SingleLine(description)),
                lsp_completion: lsp::CompletionItem {
                    label: snippet.prefix.first().unwrap().clone(),
                    kind: Some(CompletionItemKind::SNIPPET),
                    label_details: snippet.description.as_ref().map(|description| {
                        lsp::CompletionItemLabelDetails {
                            detail: Some(description.clone()),
                            description: None,
                        }
                    }),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    text_edit: Some(lsp::CompletionTextEdit::InsertAndReplace(
                        lsp::InsertReplaceEdit {
                            new_text: snippet.body.clone(),
                            insert: lsp_range,
                            replace: lsp_range,
                        },
                    )),
                    filter_text: Some(snippet.body.clone()),
                    sort_text: Some(char::MAX.to_string()),
                    ..Default::default()
                },
                confirm: None,
                show_new_completions_on_confirm: false,
            })
        })
        .collect()
}

impl CompletionProvider for Model<Project> {
    fn completions(
        &self,
        buffer: &Model<Buffer>,
        buffer_position: text::Anchor,
        options: CompletionContext,
        cx: &mut ViewContext<Editor>,
    ) -> Task<Result<Vec<Completion>>> {
        self.update(cx, |project, cx| {
            let snippets = snippet_completions(project, buffer, buffer_position, cx);
            let project_completions = project.completions(&buffer, buffer_position, options, cx);
            cx.background_executor().spawn(async move {
                let mut completions = project_completions.await?;
                //let snippets = snippets.into_iter().;
                completions.extend(snippets);
                Ok(completions)
            })
        })
    }

    fn resolve_completions(
        &self,
        buffer: Model<Buffer>,
        completion_indices: Vec<usize>,
        completions: Arc<RwLock<Box<[Completion]>>>,
        cx: &mut ViewContext<Editor>,
    ) -> Task<Result<bool>> {
        self.update(cx, |project, cx| {
            project.resolve_completions(buffer, completion_indices, completions, cx)
        })
    }

    fn apply_additional_edits_for_completion(
        &self,
        buffer: Model<Buffer>,
        completion: Completion,
        push_to_history: bool,
        cx: &mut ViewContext<Editor>,
    ) -> Task<Result<Option<language::Transaction>>> {
        self.update(cx, |project, cx| {
            project.apply_additional_edits_for_completion(buffer, completion, push_to_history, cx)
        })
    }

    fn is_completion_trigger(
        &self,
        buffer: &Model<Buffer>,
        position: language::Anchor,
        text: &str,
        trigger_in_words: bool,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        if !EditorSettings::get_global(cx).show_completions_on_input {
            return false;
        }

        let mut chars = text.chars();
        let char = if let Some(char) = chars.next() {
            char
        } else {
            return false;
        };
        if chars.next().is_some() {
            return false;
        }

        let buffer = buffer.read(cx);
        let scope = buffer.snapshot().language_scope_at(position);
        if trigger_in_words && char_kind(&scope, char) == CharKind::Word {
            return true;
        }

        buffer
            .completion_triggers()
            .iter()
            .any(|string| string == text)
    }
}

fn inlay_hint_settings(
    location: Anchor,
    snapshot: &MultiBufferSnapshot,
    cx: &mut ViewContext<'_, Editor>,
) -> InlayHintSettings {
    let file = snapshot.file_at(location);
    let language = snapshot.language_at(location);
    let settings = all_language_settings(file, cx);
    settings
        .language(language.map(|l| l.name()).as_deref())
        .inlay_hints
}

fn consume_contiguous_rows(
    contiguous_row_selections: &mut Vec<Selection<Point>>,
    selection: &Selection<Point>,
    display_map: &DisplaySnapshot,
    selections: &mut std::iter::Peekable<std::slice::Iter<Selection<Point>>>,
) -> (MultiBufferRow, MultiBufferRow) {
    contiguous_row_selections.push(selection.clone());
    let start_row = MultiBufferRow(selection.start.row);
    let mut end_row = ending_row(selection, display_map);

    while let Some(next_selection) = selections.peek() {
        if next_selection.start.row <= end_row.0 {
            end_row = ending_row(next_selection, display_map);
            contiguous_row_selections.push(selections.next().unwrap().clone());
        } else {
            break;
        }
    }
    (start_row, end_row)
}

fn ending_row(next_selection: &Selection<Point>, display_map: &DisplaySnapshot) -> MultiBufferRow {
    if next_selection.end.column > 0 || next_selection.is_empty() {
        MultiBufferRow(display_map.next_line_boundary(next_selection.end).0.row + 1)
    } else {
        MultiBufferRow(next_selection.end.row)
    }
}

impl EditorSnapshot {
    pub fn remote_selections_in_range<'a>(
        &'a self,
        range: &'a Range<Anchor>,
        collaboration_hub: &dyn CollaborationHub,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = RemoteSelection> {
        let participant_names = collaboration_hub.user_names(cx);
        let participant_indices = collaboration_hub.user_participant_indices(cx);
        let collaborators_by_peer_id = collaboration_hub.collaborators(cx);
        let collaborators_by_replica_id = collaborators_by_peer_id
            .iter()
            .map(|(_, collaborator)| (collaborator.replica_id, collaborator))
            .collect::<HashMap<_, _>>();
        self.buffer_snapshot
            .selections_in_range(range, false)
            .filter_map(move |(replica_id, line_mode, cursor_shape, selection)| {
                let collaborator = collaborators_by_replica_id.get(&replica_id)?;
                let participant_index = participant_indices.get(&collaborator.user_id).copied();
                let user_name = participant_names.get(&collaborator.user_id).cloned();
                Some(RemoteSelection {
                    replica_id,
                    selection,
                    cursor_shape,
                    line_mode,
                    participant_index,
                    peer_id: collaborator.peer_id,
                    user_name,
                })
            })
    }

    pub fn language_at<T: ToOffset>(&self, position: T) -> Option<&Arc<Language>> {
        self.display_snapshot.buffer_snapshot.language_at(position)
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    pub fn placeholder_text(&self) -> Option<&Arc<str>> {
        self.placeholder_text.as_ref()
    }

    pub fn scroll_position(&self) -> gpui::Point<f32> {
        self.scroll_anchor.scroll_position(&self.display_snapshot)
    }

    fn gutter_dimensions(
        &self,
        font_id: FontId,
        font_size: Pixels,
        em_width: Pixels,
        max_line_number_width: Pixels,
        cx: &AppContext,
    ) -> GutterDimensions {
        if !self.show_gutter {
            return GutterDimensions::default();
        }
        let descent = cx.text_system().descent(font_id, font_size);

        let show_git_gutter = self.show_git_diff_gutter.unwrap_or_else(|| {
            matches!(
                ProjectSettings::get_global(cx).git.git_gutter,
                Some(GitGutterSetting::TrackedFiles)
            )
        });
        let gutter_settings = EditorSettings::get_global(cx).gutter;
        let show_line_numbers = self
            .show_line_numbers
            .unwrap_or(gutter_settings.line_numbers);
        let line_gutter_width = if show_line_numbers {
            // Avoid flicker-like gutter resizes when the line number gains another digit and only resize the gutter on files with N*10^5 lines.
            let min_width_for_number_on_gutter = em_width * 4.0;
            max_line_number_width.max(min_width_for_number_on_gutter)
        } else {
            0.0.into()
        };

        let show_code_actions = self
            .show_code_actions
            .unwrap_or(gutter_settings.code_actions);

        let show_runnables = self.show_runnables.unwrap_or(gutter_settings.runnables);

        let git_blame_entries_width = self
            .render_git_blame_gutter
            .then_some(em_width * GIT_BLAME_GUTTER_WIDTH_CHARS);

        let mut left_padding = git_blame_entries_width.unwrap_or(Pixels::ZERO);
        left_padding += if show_code_actions || show_runnables {
            em_width * 3.0
        } else if show_git_gutter && show_line_numbers {
            em_width * 2.0
        } else if show_git_gutter || show_line_numbers {
            em_width
        } else {
            px(0.)
        };

        let right_padding = if gutter_settings.folds && show_line_numbers {
            em_width * 4.0
        } else if gutter_settings.folds {
            em_width * 3.0
        } else if show_line_numbers {
            em_width
        } else {
            px(0.)
        };

        GutterDimensions {
            left_padding,
            right_padding,
            width: line_gutter_width + left_padding + right_padding,
            margin: -descent,
            git_blame_entries_width,
        }
    }

    pub fn render_fold_toggle(
        &self,
        buffer_row: MultiBufferRow,
        row_contains_cursor: bool,
        editor: View<Editor>,
        cx: &mut WindowContext,
    ) -> Option<AnyElement> {
        let folded = self.is_line_folded(buffer_row);

        if let Some(crease) = self
            .crease_snapshot
            .query_row(buffer_row, &self.buffer_snapshot)
        {
            let toggle_callback = Arc::new(move |folded, cx: &mut WindowContext| {
                if folded {
                    editor.update(cx, |editor, cx| {
                        editor.fold_at(&crate::FoldAt { buffer_row }, cx)
                    });
                } else {
                    editor.update(cx, |editor, cx| {
                        editor.unfold_at(&crate::UnfoldAt { buffer_row }, cx)
                    });
                }
            });

            Some((crease.render_toggle)(
                buffer_row,
                folded,
                toggle_callback,
                cx,
            ))
        } else if folded
            || (self.starts_indent(buffer_row) && (row_contains_cursor || self.gutter_hovered))
        {
            Some(
                Disclosure::new(("indent-fold-indicator", buffer_row.0), !folded)
                    .selected(folded)
                    .on_click(cx.listener_for(&editor, move |this, _e, cx| {
                        if folded {
                            this.unfold_at(&UnfoldAt { buffer_row }, cx);
                        } else {
                            this.fold_at(&FoldAt { buffer_row }, cx);
                        }
                    }))
                    .into_any_element(),
            )
        } else {
            None
        }
    }

    pub fn render_crease_trailer(
        &self,
        buffer_row: MultiBufferRow,
        cx: &mut WindowContext,
    ) -> Option<AnyElement> {
        let folded = self.is_line_folded(buffer_row);
        let crease = self
            .crease_snapshot
            .query_row(buffer_row, &self.buffer_snapshot)?;
        Some((crease.render_trailer)(buffer_row, folded, cx))
    }
}

impl Deref for EditorSnapshot {
    type Target = DisplaySnapshot;

    fn deref(&self) -> &Self::Target {
        &self.display_snapshot
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditorEvent {
    InputIgnored {
        text: Arc<str>,
    },
    InputHandled {
        utf16_range_to_replace: Option<Range<isize>>,
        text: Arc<str>,
    },
    ExcerptsAdded {
        buffer: Model<Buffer>,
        predecessor: ExcerptId,
        excerpts: Vec<(ExcerptId, ExcerptRange<language::Anchor>)>,
    },
    ExcerptsRemoved {
        ids: Vec<ExcerptId>,
    },
    ExcerptsEdited {
        ids: Vec<ExcerptId>,
    },
    ExcerptsExpanded {
        ids: Vec<ExcerptId>,
    },
    BufferEdited,
    Edited {
        transaction_id: clock::Lamport,
    },
    Reparsed(BufferId),
    Focused,
    FocusedIn,
    Blurred,
    DirtyChanged,
    Saved,
    TitleChanged,
    DiffBaseChanged,
    SelectionsChanged {
        local: bool,
    },
    ScrollPositionChanged {
        local: bool,
        autoscroll: bool,
    },
    Closed,
    TransactionUndone {
        transaction_id: clock::Lamport,
    },
    TransactionBegun {
        transaction_id: clock::Lamport,
    },
}

impl EventEmitter<EditorEvent> for Editor {}

impl FocusableView for Editor {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Editor {
    fn render<'a>(&mut self, cx: &mut ViewContext<'a, Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);

        let text_style = match self.mode {
            EditorMode::SingleLine { .. } | EditorMode::AutoHeight { .. } => TextStyle {
                color: cx.theme().colors().editor_foreground,
                font_family: settings.ui_font.family.clone(),
                font_features: settings.ui_font.features.clone(),
                font_fallbacks: settings.ui_font.fallbacks.clone(),
                font_size: rems(0.875).into(),
                font_weight: settings.ui_font.weight,
                line_height: relative(settings.buffer_line_height.value()),
                ..Default::default()
            },
            EditorMode::Full => TextStyle {
                color: cx.theme().colors().editor_foreground,
                font_family: settings.buffer_font.family.clone(),
                font_features: settings.buffer_font.features.clone(),
                font_fallbacks: settings.buffer_font.fallbacks.clone(),
                font_size: settings.buffer_font_size(cx).into(),
                font_weight: settings.buffer_font.weight,
                line_height: relative(settings.buffer_line_height.value()),
                ..Default::default()
            },
        };

        let background = match self.mode {
            EditorMode::SingleLine { .. } => cx.theme().system().transparent,
            EditorMode::AutoHeight { max_lines: _ } => cx.theme().system().transparent,
            EditorMode::Full => cx.theme().colors().editor_background,
        };

        EditorElement::new(
            cx.view(),
            EditorStyle {
                background,
                local_player: cx.theme().players().local(),
                text: text_style,
                scrollbar_width: EditorElement::SCROLLBAR_WIDTH,
                syntax: cx.theme().syntax().clone(),
                status: cx.theme().status().clone(),
                inlay_hints_style: HighlightStyle {
                    color: Some(cx.theme().status().hint),
                    ..HighlightStyle::default()
                },
                suggestions_style: HighlightStyle {
                    color: Some(cx.theme().status().predictive),
                    ..HighlightStyle::default()
                },
            },
        )
    }
}

impl ViewInputHandler for Editor {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        cx: &mut ViewContext<Self>,
    ) -> Option<String> {
        Some(
            self.buffer
                .read(cx)
                .read(cx)
                .text_for_range(OffsetUtf16(range_utf16.start)..OffsetUtf16(range_utf16.end))
                .collect(),
        )
    }

    fn selected_text_range(&mut self, cx: &mut ViewContext<Self>) -> Option<Range<usize>> {
        // Prevent the IME menu from appearing when holding down an alphabetic key
        // while input is disabled.
        if !self.input_enabled {
            return None;
        }

        let range = self.selections.newest::<OffsetUtf16>(cx).range();
        Some(range.start.0..range.end.0)
    }

    fn marked_text_range(&self, cx: &mut ViewContext<Self>) -> Option<Range<usize>> {
        let snapshot = self.buffer.read(cx).read(cx);
        let range = self.text_highlights::<InputComposition>(cx)?.1.get(0)?;
        Some(range.start.to_offset_utf16(&snapshot).0..range.end.to_offset_utf16(&snapshot).0)
    }

    fn unmark_text(&mut self, cx: &mut ViewContext<Self>) {
        self.clear_highlights::<InputComposition>(cx);
        self.ime_transaction.take();
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        text: &str,
        cx: &mut ViewContext<Self>,
    ) {
        if !self.input_enabled {
            cx.emit(EditorEvent::InputIgnored { text: text.into() });
            return;
        }

        self.transact(cx, |this, cx| {
            let new_selected_ranges = if let Some(range_utf16) = range_utf16 {
                let range_utf16 = OffsetUtf16(range_utf16.start)..OffsetUtf16(range_utf16.end);
                Some(this.selection_replacement_ranges(range_utf16, cx))
            } else {
                this.marked_text_ranges(cx)
            };

            let range_to_replace = new_selected_ranges.as_ref().and_then(|ranges_to_replace| {
                let newest_selection_id = this.selections.newest_anchor().id;
                this.selections
                    .all::<OffsetUtf16>(cx)
                    .iter()
                    .zip(ranges_to_replace.iter())
                    .find_map(|(selection, range)| {
                        if selection.id == newest_selection_id {
                            Some(
                                (range.start.0 as isize - selection.head().0 as isize)
                                    ..(range.end.0 as isize - selection.head().0 as isize),
                            )
                        } else {
                            None
                        }
                    })
            });

            cx.emit(EditorEvent::InputHandled {
                utf16_range_to_replace: range_to_replace,
                text: text.into(),
            });

            if let Some(new_selected_ranges) = new_selected_ranges {
                this.change_selections(None, cx, |selections| {
                    selections.select_ranges(new_selected_ranges)
                });
                this.backspace(&Default::default(), cx);
            }

            this.handle_input(text, cx);
        });

        if let Some(transaction) = self.ime_transaction {
            self.buffer.update(cx, |buffer, cx| {
                buffer.group_until_transaction(transaction, cx);
            });
        }

        self.unmark_text(cx);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        cx: &mut ViewContext<Self>,
    ) {
        if !self.input_enabled {
            return;
        }

        let transaction = self.transact(cx, |this, cx| {
            let ranges_to_replace = if let Some(mut marked_ranges) = this.marked_text_ranges(cx) {
                let snapshot = this.buffer.read(cx).read(cx);
                if let Some(relative_range_utf16) = range_utf16.as_ref() {
                    for marked_range in &mut marked_ranges {
                        marked_range.end.0 = marked_range.start.0 + relative_range_utf16.end;
                        marked_range.start.0 += relative_range_utf16.start;
                        marked_range.start =
                            snapshot.clip_offset_utf16(marked_range.start, Bias::Left);
                        marked_range.end =
                            snapshot.clip_offset_utf16(marked_range.end, Bias::Right);
                    }
                }
                Some(marked_ranges)
            } else if let Some(range_utf16) = range_utf16 {
                let range_utf16 = OffsetUtf16(range_utf16.start)..OffsetUtf16(range_utf16.end);
                Some(this.selection_replacement_ranges(range_utf16, cx))
            } else {
                None
            };

            let range_to_replace = ranges_to_replace.as_ref().and_then(|ranges_to_replace| {
                let newest_selection_id = this.selections.newest_anchor().id;
                this.selections
                    .all::<OffsetUtf16>(cx)
                    .iter()
                    .zip(ranges_to_replace.iter())
                    .find_map(|(selection, range)| {
                        if selection.id == newest_selection_id {
                            Some(
                                (range.start.0 as isize - selection.head().0 as isize)
                                    ..(range.end.0 as isize - selection.head().0 as isize),
                            )
                        } else {
                            None
                        }
                    })
            });

            cx.emit(EditorEvent::InputHandled {
                utf16_range_to_replace: range_to_replace,
                text: text.into(),
            });

            if let Some(ranges) = ranges_to_replace {
                this.change_selections(None, cx, |s| s.select_ranges(ranges));
            }

            let marked_ranges = {
                let snapshot = this.buffer.read(cx).read(cx);
                this.selections
                    .disjoint_anchors()
                    .iter()
                    .map(|selection| {
                        selection.start.bias_left(&snapshot)..selection.end.bias_right(&snapshot)
                    })
                    .collect::<Vec<_>>()
            };

            if text.is_empty() {
                this.unmark_text(cx);
            } else {
                this.highlight_text::<InputComposition>(
                    marked_ranges.clone(),
                    HighlightStyle {
                        underline: Some(UnderlineStyle {
                            thickness: px(1.),
                            color: None,
                            wavy: false,
                        }),
                        ..Default::default()
                    },
                    cx,
                );
            }

            // Disable auto-closing when composing text (i.e. typing a `"` on a Brazilian keyboard)
            let use_autoclose = this.use_autoclose;
            let use_auto_surround = this.use_auto_surround;
            this.set_use_autoclose(false);
            this.set_use_auto_surround(false);
            this.handle_input(text, cx);
            this.set_use_autoclose(use_autoclose);
            this.set_use_auto_surround(use_auto_surround);

            if let Some(new_selected_range) = new_selected_range_utf16 {
                let snapshot = this.buffer.read(cx).read(cx);
                let new_selected_ranges = marked_ranges
                    .into_iter()
                    .map(|marked_range| {
                        let insertion_start = marked_range.start.to_offset_utf16(&snapshot).0;
                        let new_start = OffsetUtf16(new_selected_range.start + insertion_start);
                        let new_end = OffsetUtf16(new_selected_range.end + insertion_start);
                        snapshot.clip_offset_utf16(new_start, Bias::Left)
                            ..snapshot.clip_offset_utf16(new_end, Bias::Right)
                    })
                    .collect::<Vec<_>>();

                drop(snapshot);
                this.change_selections(None, cx, |selections| {
                    selections.select_ranges(new_selected_ranges)
                });
            }
        });

        self.ime_transaction = self.ime_transaction.or(transaction);
        if let Some(transaction) = self.ime_transaction {
            self.buffer.update(cx, |buffer, cx| {
                buffer.group_until_transaction(transaction, cx);
            });
        }

        if self.text_highlights::<InputComposition>(cx).is_none() {
            self.ime_transaction.take();
        }
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        element_bounds: gpui::Bounds<Pixels>,
        cx: &mut ViewContext<Self>,
    ) -> Option<gpui::Bounds<Pixels>> {
        let text_layout_details = self.text_layout_details(cx);
        let style = &text_layout_details.editor_style;
        let font_id = cx.text_system().resolve_font(&style.text.font());
        let font_size = style.text.font_size.to_pixels(cx.rem_size());
        let line_height = style.text.line_height_in_pixels(cx.rem_size());

        let em_width = cx
            .text_system()
            .typographic_bounds(font_id, font_size, 'm')
            .unwrap()
            .size
            .width;

        let snapshot = self.snapshot(cx);
        let scroll_position = snapshot.scroll_position();
        let scroll_left = scroll_position.x * em_width;

        let start = OffsetUtf16(range_utf16.start).to_display_point(&snapshot);
        let x = snapshot.x_for_display_point(start, &text_layout_details) - scroll_left
            + self.gutter_dimensions.width;
        let y = line_height * (start.row().as_f32() - scroll_position.y);

        Some(Bounds {
            origin: element_bounds.origin + point(x, y),
            size: size(em_width, line_height),
        })
    }
}

trait SelectionExt {
    fn display_range(&self, map: &DisplaySnapshot) -> Range<DisplayPoint>;
    fn spanned_rows(
        &self,
        include_end_if_at_line_start: bool,
        map: &DisplaySnapshot,
    ) -> Range<MultiBufferRow>;
}

impl<T: ToPoint + ToOffset> SelectionExt for Selection<T> {
    fn display_range(&self, map: &DisplaySnapshot) -> Range<DisplayPoint> {
        let start = self
            .start
            .to_point(&map.buffer_snapshot)
            .to_display_point(map);
        let end = self
            .end
            .to_point(&map.buffer_snapshot)
            .to_display_point(map);
        if self.reversed {
            end..start
        } else {
            start..end
        }
    }

    fn spanned_rows(
        &self,
        include_end_if_at_line_start: bool,
        map: &DisplaySnapshot,
    ) -> Range<MultiBufferRow> {
        let start = self.start.to_point(&map.buffer_snapshot);
        let mut end = self.end.to_point(&map.buffer_snapshot);
        if !include_end_if_at_line_start && start.row != end.row && end.column == 0 {
            end.row -= 1;
        }

        let buffer_start = map.prev_line_boundary(start).0;
        let buffer_end = map.next_line_boundary(end).0;
        MultiBufferRow(buffer_start.row)..MultiBufferRow(buffer_end.row + 1)
    }
}

impl<T: InvalidationRegion> InvalidationStack<T> {
    fn invalidate<S>(&mut self, selections: &[Selection<S>], buffer: &MultiBufferSnapshot)
    where
        S: Clone + ToOffset,
    {
        while let Some(region) = self.last() {
            let all_selections_inside_invalidation_ranges =
                if selections.len() == region.ranges().len() {
                    selections
                        .iter()
                        .zip(region.ranges().iter().map(|r| r.to_offset(buffer)))
                        .all(|(selection, invalidation_range)| {
                            let head = selection.head().to_offset(buffer);
                            invalidation_range.start <= head && invalidation_range.end >= head
                        })
                } else {
                    false
                };

            if all_selections_inside_invalidation_ranges {
                break;
            } else {
                self.pop();
            }
        }
    }
}

impl<T> Default for InvalidationStack<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Deref for InvalidationStack<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for InvalidationStack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl InvalidationRegion for SnippetState {
    fn ranges(&self) -> &[Range<Anchor>] {
        &self.ranges[self.active_index]
    }
}

pub fn diagnostic_block_renderer(
    diagnostic: Diagnostic,
    max_message_rows: Option<u8>,
    allow_closing: bool,
    _is_valid: bool,
) -> RenderBlock {
    let (text_without_backticks, code_ranges) =
        highlight_diagnostic_message(&diagnostic, max_message_rows);

    Box::new(move |cx: &mut BlockContext| {
        let group_id: SharedString = cx.block_id.to_string().into();

        let mut text_style = cx.text_style().clone();
        text_style.color = diagnostic_style(diagnostic.severity, cx.theme().status());
        let theme_settings = ThemeSettings::get_global(cx);
        text_style.font_family = theme_settings.buffer_font.family.clone();
        text_style.font_style = theme_settings.buffer_font.style;
        text_style.font_features = theme_settings.buffer_font.features.clone();
        text_style.font_weight = theme_settings.buffer_font.weight;

        let multi_line_diagnostic = diagnostic.message.contains('\n');

        let buttons = |diagnostic: &Diagnostic, block_id: BlockId| {
            if multi_line_diagnostic {
                v_flex()
            } else {
                h_flex()
            }
            .when(allow_closing, |div| {
                div.children(diagnostic.is_primary.then(|| {
                    IconButton::new(("close-block", EntityId::from(block_id)), IconName::XCircle)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::Compact)
                        .style(ButtonStyle::Transparent)
                        .visible_on_hover(group_id.clone())
                        .on_click(move |_click, cx| cx.dispatch_action(Box::new(Cancel)))
                        .tooltip(|cx| Tooltip::for_action("Close Diagnostics", &Cancel, cx))
                }))
            })
            .child(
                IconButton::new(("copy-block", EntityId::from(block_id)), IconName::Copy)
                    .icon_color(Color::Muted)
                    .size(ButtonSize::Compact)
                    .style(ButtonStyle::Transparent)
                    .visible_on_hover(group_id.clone())
                    .on_click({
                        let message = diagnostic.message.clone();
                        move |_click, cx| cx.write_to_clipboard(ClipboardItem::new(message.clone()))
                    })
                    .tooltip(|cx| Tooltip::text("Copy diagnostic message", cx)),
            )
        };

        let icon_size = buttons(&diagnostic, cx.block_id)
            .into_any_element()
            .layout_as_root(AvailableSpace::min_size(), cx);

        h_flex()
            .id(cx.block_id)
            .group(group_id.clone())
            .relative()
            .size_full()
            .pl(cx.gutter_dimensions.width)
            .w(cx.max_width + cx.gutter_dimensions.width)
            .child(
                div()
                    .flex()
                    .w(cx.anchor_x - cx.gutter_dimensions.width - icon_size.width)
                    .flex_shrink(),
            )
            .child(buttons(&diagnostic, cx.block_id))
            .child(div().flex().flex_shrink_0().child(
                StyledText::new(text_without_backticks.clone()).with_highlights(
                    &text_style,
                    code_ranges.iter().map(|range| {
                        (
                            range.clone(),
                            HighlightStyle {
                                font_weight: Some(FontWeight::BOLD),
                                ..Default::default()
                            },
                        )
                    }),
                ),
            ))
            .into_any_element()
    })
}

pub fn highlight_diagnostic_message(
    diagnostic: &Diagnostic,
    mut max_message_rows: Option<u8>,
) -> (SharedString, Vec<Range<usize>>) {
    let mut text_without_backticks = String::new();
    let mut code_ranges = Vec::new();

    if let Some(source) = &diagnostic.source {
        text_without_backticks.push_str(&source);
        code_ranges.push(0..source.len());
        text_without_backticks.push_str(": ");
    }

    let mut prev_offset = 0;
    let mut in_code_block = false;
    let has_row_limit = max_message_rows.is_some();
    let mut newline_indices = diagnostic
        .message
        .match_indices('\n')
        .filter(|_| has_row_limit)
        .map(|(ix, _)| ix)
        .fuse()
        .peekable();

    for (quote_ix, _) in diagnostic
        .message
        .match_indices('`')
        .chain([(diagnostic.message.len(), "")])
    {
        let mut first_newline_ix = None;
        let mut last_newline_ix = None;
        while let Some(newline_ix) = newline_indices.peek() {
            if *newline_ix < quote_ix {
                if first_newline_ix.is_none() {
                    first_newline_ix = Some(*newline_ix);
                }
                last_newline_ix = Some(*newline_ix);

                if let Some(rows_left) = &mut max_message_rows {
                    if *rows_left == 0 {
                        break;
                    } else {
                        *rows_left -= 1;
                    }
                }
                let _ = newline_indices.next();
            } else {
                break;
            }
        }
        let prev_len = text_without_backticks.len();
        let new_text = &diagnostic.message[prev_offset..first_newline_ix.unwrap_or(quote_ix)];
        text_without_backticks.push_str(new_text);
        if in_code_block {
            code_ranges.push(prev_len..text_without_backticks.len());
        }
        prev_offset = last_newline_ix.unwrap_or(quote_ix) + 1;
        in_code_block = !in_code_block;
        if first_newline_ix.map_or(false, |newline_ix| newline_ix < quote_ix) {
            text_without_backticks.push_str("...");
            break;
        }
    }

    (text_without_backticks.into(), code_ranges)
}

fn diagnostic_style(severity: DiagnosticSeverity, colors: &StatusColors) -> Hsla {
    match severity {
        DiagnosticSeverity::ERROR => colors.error,
        DiagnosticSeverity::WARNING => colors.warning,
        DiagnosticSeverity::INFORMATION => colors.info,
        DiagnosticSeverity::HINT => colors.info,
        _ => colors.ignored,
    }
}

pub fn styled_runs_for_code_label<'a>(
    label: &'a CodeLabel,
    syntax_theme: &'a theme::SyntaxTheme,
) -> impl 'a + Iterator<Item = (Range<usize>, HighlightStyle)> {
    let fade_out = HighlightStyle {
        fade_out: Some(0.35),
        ..Default::default()
    };

    let mut prev_end = label.filter_range.end;
    label
        .runs
        .iter()
        .enumerate()
        .flat_map(move |(ix, (range, highlight_id))| {
            let style = if let Some(style) = highlight_id.style(syntax_theme) {
                style
            } else {
                return Default::default();
            };
            let mut muted_style = style;
            muted_style.highlight(fade_out);

            let mut runs = SmallVec::<[(Range<usize>, HighlightStyle); 3]>::new();
            if range.start >= label.filter_range.end {
                if range.start > prev_end {
                    runs.push((prev_end..range.start, fade_out));
                }
                runs.push((range.clone(), muted_style));
            } else if range.end <= label.filter_range.end {
                runs.push((range.clone(), style));
            } else {
                runs.push((range.start..label.filter_range.end, style));
                runs.push((label.filter_range.end..range.end, muted_style));
            }
            prev_end = cmp::max(prev_end, range.end);

            if ix + 1 == label.runs.len() && label.text.len() > prev_end {
                runs.push((prev_end..label.text.len(), fade_out));
            }

            runs
        })
}

pub(crate) fn split_words(text: &str) -> impl std::iter::Iterator<Item = &str> + '_ {
    let mut prev_index = 0;
    let mut prev_codepoint: Option<char> = None;
    text.char_indices()
        .chain([(text.len(), '\0')])
        .filter_map(move |(index, codepoint)| {
            let prev_codepoint = prev_codepoint.replace(codepoint)?;
            let is_boundary = index == text.len()
                || !prev_codepoint.is_uppercase() && codepoint.is_uppercase()
                || !prev_codepoint.is_alphanumeric() && codepoint.is_alphanumeric();
            if is_boundary {
                let chunk = &text[prev_index..index];
                prev_index = index;
                Some(chunk)
            } else {
                None
            }
        })
}

pub trait RangeToAnchorExt: Sized {
    fn to_anchors(self, snapshot: &MultiBufferSnapshot) -> Range<Anchor>;

    fn to_display_points(self, snapshot: &EditorSnapshot) -> Range<DisplayPoint> {
        let anchor_range = self.to_anchors(&snapshot.buffer_snapshot);
        anchor_range.start.to_display_point(&snapshot)..anchor_range.end.to_display_point(&snapshot)
    }
}

impl<T: ToOffset> RangeToAnchorExt for Range<T> {
    fn to_anchors(self, snapshot: &MultiBufferSnapshot) -> Range<Anchor> {
        let start_offset = self.start.to_offset(snapshot);
        let end_offset = self.end.to_offset(snapshot);
        if start_offset == end_offset {
            snapshot.anchor_before(start_offset)..snapshot.anchor_before(end_offset)
        } else {
            snapshot.anchor_after(self.start)..snapshot.anchor_before(self.end)
        }
    }
}

pub trait RowExt {
    fn as_f32(&self) -> f32;

    fn next_row(&self) -> Self;

    fn previous_row(&self) -> Self;

    fn minus(&self, other: Self) -> u32;
}

impl RowExt for DisplayRow {
    fn as_f32(&self) -> f32 {
        self.0 as f32
    }

    fn next_row(&self) -> Self {
        Self(self.0 + 1)
    }

    fn previous_row(&self) -> Self {
        Self(self.0.saturating_sub(1))
    }

    fn minus(&self, other: Self) -> u32 {
        self.0 - other.0
    }
}

impl RowExt for MultiBufferRow {
    fn as_f32(&self) -> f32 {
        self.0 as f32
    }

    fn next_row(&self) -> Self {
        Self(self.0 + 1)
    }

    fn previous_row(&self) -> Self {
        Self(self.0.saturating_sub(1))
    }

    fn minus(&self, other: Self) -> u32 {
        self.0 - other.0
    }
}

trait RowRangeExt {
    type Row;

    fn len(&self) -> usize;

    fn iter_rows(&self) -> impl DoubleEndedIterator<Item = Self::Row>;
}

impl RowRangeExt for Range<MultiBufferRow> {
    type Row = MultiBufferRow;

    fn len(&self) -> usize {
        (self.end.0 - self.start.0) as usize
    }

    fn iter_rows(&self) -> impl DoubleEndedIterator<Item = MultiBufferRow> {
        (self.start.0..self.end.0).map(MultiBufferRow)
    }
}

impl RowRangeExt for Range<DisplayRow> {
    type Row = DisplayRow;

    fn len(&self) -> usize {
        (self.end.0 - self.start.0) as usize
    }

    fn iter_rows(&self) -> impl DoubleEndedIterator<Item = DisplayRow> {
        (self.start.0..self.end.0).map(DisplayRow)
    }
}

fn hunk_status(hunk: &DiffHunk<MultiBufferRow>) -> DiffHunkStatus {
    if hunk.diff_base_byte_range.is_empty() {
        DiffHunkStatus::Added
    } else if hunk.associated_range.is_empty() {
        DiffHunkStatus::Removed
    } else {
        DiffHunkStatus::Modified
    }
}
