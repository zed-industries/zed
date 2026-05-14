use std::{marker::PhantomData, ops::Range, sync::Arc};

use editor::{Anchor, DisplayPoint, MultiBufferOffset};
use language::{CursorShape, SelectionGoal};
use text::{Bias, TransactionId};

pub(crate) type Count = usize;
pub(crate) type RegisterName = char;
pub(crate) type KeyText = Arc<str>;
pub(crate) type CommandName = Arc<str>;
pub(crate) type SearchQuery = Arc<str>;
pub(crate) type BufferEntityId = usize;
pub(crate) type SelectionId = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Dialect {
    Vim,
    Helix,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EngineKind {
    Vim,
    Helix,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Direction {
    Forward,
    Backward,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BoundarySide {
    Start,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BoundaryInclusivity {
    Inclusive,
    Exclusive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WordFlavor {
    Word,
    BigWord,
    Subword,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PunctuationPolicy {
    Include,
    Ignore,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CasePolicy {
    Sensitive,
    Insensitive,
    Smart,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WrapPolicy {
    Stop,
    Wrap,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CountPolicy {
    Exact,
    AtLeastOne,
    Optional,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MotionKind {
    CharacterwiseExclusive,
    CharacterwiseInclusive,
    Linewise,
    Blockwise,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SelectionAffinity {
    Head,
    Tail,
    Start,
    End,
    Newest,
    Primary,
    All,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CursorSemantics {
    InsertionPoint,
    IncludedCharacter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PointCommandSemantics {
    RawSelectionHead,
    DisplayedCursor,
    IncludedCharacter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SelectionStorage {
    HalfOpenRange,
    IncludedCursorRange,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RangeRole {
    MotionRange,
    SelectionRange,
    ObjectRange,
    SearchRange,
    JumpTargetRange,
    EditRange,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RangeExpansion {
    None,
    IncludeCurrentCharacter,
    IncludeLineBreak,
    ExcludeLineBreak,
    ExpandToLine,
    ExpandToFullObject,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AnchorBiasPolicy {
    Before,
    After,
    FromDirection,
    Preserve,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EmptySelectionPolicy {
    Preserve,
    Collapse,
    ExpandToCharacter,
    ExpandToLine,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ReversedSelectionPolicy {
    Preserve,
    Normalize,
    FlipAnchor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LineEndPolicy {
    BeforeNewline,
    AfterLastCharacter,
    OnLastCharacter,
    IncludeNewline,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NewlinePolicy {
    Exclude,
    Include,
    PreserveExisting,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SearchScope {
    Buffer,
    VisibleRange,
    Selection,
    Project,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SearchDirection {
    Next,
    Previous,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TextObjectBoundary {
    Inner,
    Around,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TextObjectSpec {
    Word { ignore_punctuation: bool },
    Subword { ignore_punctuation: bool },
    Sentence,
    Paragraph,
    Quotes,
    BackQuotes,
    AnyQuotes,
    MiniQuotes,
    DoubleQuotes,
    VerticalBars,
    AnyBrackets,
    MiniBrackets,
    Parentheses,
    SquareBrackets,
    CurlyBrackets,
    AngleBrackets,
    Argument,
    IndentObj { include_below: bool },
    Tag,
    Method,
    Class,
    Comment,
    EntireFile,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BracketOpeningPolicy {
    Opening,
    Closing,
    Either,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FindRangeKind {
    SingleLine,
    MultiLine,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DisplayLinePolicy {
    BufferLines,
    DisplayLines,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LineColumnTarget {
    Start,
    Middle,
    End,
    ExplicitColumn,
    FirstNonWhitespace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WindowLineTarget {
    Top,
    Middle,
    Bottom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SectionTarget {
    Start,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum IndentRelation {
    Lesser,
    Greater,
    Same,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum IndentEditDirection {
    In,
    Out,
    Auto,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MethodBoundary {
    Start,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SyntaxNodeDirection {
    Larger,
    Smaller,
    Next,
    Previous,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ObjectSearchPosition {
    Current,
    Next,
    Previous,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SurroundKind {
    Pair,
    Symmetric,
    Tag,
    FunctionCall,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RegisterExpression {
    pub(crate) source: KeyText,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum RegisterScope {
    Explicit(RegisterName),
    Default,
    System,
    BlackHole,
    SmallDelete,
    ReadOnly(ReadOnlyRegister),
    Expression(Option<RegisterExpression>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ReadOnlyRegister {
    CommandLine,
    LastInsertedText,
    AlternateFile,
    CurrentFilePath,
    LastSearch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum UseSystemClipboardPolicy {
    Always,
    OnYank,
    Never,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NumberedRegisterRotationPolicy {
    RotateLinewiseDeletes,
    NeverRotate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EditIntent {
    Insert,
    Append,
    InsertLineAbove,
    InsertLineBelow,
    InsertEmptyLineAbove,
    InsertEmptyLineBelow,
    Change,
    Delete,
    Yank,
    Replace,
    Paste,
    Join,
    Increment,
    Decrement,
    Indent,
    Outdent,
    Format,
    ToggleComment,
    Undo,
    Redo,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PastePlacement {
    Before,
    After,
    Replace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PasteClipboardPolicy {
    PreserveClipboard,
    ReplaceClipboard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum IncrementDirection {
    Increment,
    Decrement,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum IncrementMode {
    Normal,
    Step,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ConvertTarget {
    Lowercase,
    Uppercase,
    OppositeCase,
    Rot13,
    Rot47,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum JoinWhitespacePolicy {
    InsertWhitespace,
    NoWhitespace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum UndoTarget {
    LastChange,
    LastLine,
    Replace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Repeatability {
    DotRepeatable,
    NotDotRepeatable,
    RecordsKeystrokes,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum UiFeedback {
    None,
    Status,
    Error,
    Overlay,
    Highlight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OverlayKind {
    JumpLabels,
    SearchMatches,
    SelectionPreview,
    TextObjectPreview,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum JumpTargetKind {
    WordStart,
    WordRange,
    Character,
    Line,
    SearchMatch,
    TextObject,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum JumpLabelAlphabet {
    Helix,
    HomeRow,
    Custom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum JumpLabelOrdering {
    ForwardThenBackward,
    DistanceAlternating,
    DocumentOrder,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum JumpCompletionPolicy {
    Move,
    Select,
    Extend,
    ApplyAsMotion,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HelixJumpCompletion {
    Move,
    MoveToWordStart,
    Extend,
    ExtendToWordStart,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MacroReplayPolicy {
    RecordedRegister,
    LastReplayedRegister,
    Explicit(RegisterName),
}

/// Mark scope, stored location, and jump target are intentionally not symmetric.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MarkScope {
    Local,
    Buffer,
    Global,
    Path,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum MarkLocation {
    Buffer(BufferEntityId),
    Path(Arc<str>),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum MarkTarget {
    Local(Vec<Anchor>),
    Buffer {
        buffer_id: BufferEntityId,
        anchors: Vec<Anchor>,
    },
    Path {
        path: Arc<str>,
        points: Vec<BufferPoint>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct MarkStoreSnapshot {
    pub(crate) local_marks: Vec<(CommandName, Vec<Anchor>)>,
    pub(crate) buffer_marks: Vec<(BufferEntityId, CommandName, Vec<Anchor>)>,
    pub(crate) global_marks: Vec<(CommandName, MarkLocation)>,
    pub(crate) serialized_marks: Vec<(Arc<str>, CommandName, Vec<BufferPoint>)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ChangeListDirection {
    Older,
    Newer,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ChangeListEntry {
    pub(crate) selections: Vec<Anchor>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ChangeListSnapshot {
    pub(crate) entries: Vec<ChangeListEntry>,
    pub(crate) current_index: Option<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct JumpListEntry {
    pub(crate) selections: Vec<Anchor>,
    pub(crate) source: JumpListSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum JumpListSource {
    VimMotion,
    MarkJump,
    Definition,
    ExplicitSaveLocation,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct JumpListSnapshot {
    pub(crate) entries: Vec<JumpListEntry>,
    pub(crate) current_index: Option<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RegisterContents {
    pub(crate) text: KeyText,
    pub(crate) clipboard_selections: Option<Vec<ClipboardSelectionSnapshot>>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ClipboardSelectionSnapshot {
    pub(crate) len: usize,
    pub(crate) is_entire_line: bool,
    pub(crate) first_line_indent: u32,
    pub(crate) file_path: Option<KeyText>,
    pub(crate) line_range: Option<LineRangeInclusive>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LineRangeInclusive {
    pub(crate) start: u32,
    pub(crate) end: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RegisterStoreSnapshot {
    pub(crate) selected_register: Option<RegisterName>,
    pub(crate) registers: Vec<(RegisterName, RegisterContents)>,
    pub(crate) numbered_rotation: NumberedRegisterRotationPolicy,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ReplayableAction {
    Action {
        action_type: CommandName,
        payload: Option<KeyText>,
    },
    Insertion {
        text: KeyText,
        utf16_range_to_replace: Option<Range<isize>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RecordingState {
    pub(crate) dot_recording: bool,
    pub(crate) dot_replaying: bool,
    pub(crate) stop_recording_after_next_action: bool,
    pub(crate) ignore_current_insertion: bool,
    pub(crate) recording_register: Option<RegisterName>,
    pub(crate) recording_register_for_dot: Option<RegisterName>,
    pub(crate) recorded_register_for_dot: Option<RegisterName>,
    pub(crate) last_recorded_register: Option<RegisterName>,
    pub(crate) last_replayed_register: Option<RegisterName>,
    pub(crate) recording_count: Option<Count>,
    pub(crate) recorded_count: Option<Count>,
    pub(crate) recording_actions: Vec<ReplayableAction>,
    pub(crate) recorded_actions: Vec<ReplayableAction>,
    pub(crate) recordings: Vec<(RegisterName, Vec<ReplayableAction>)>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum RecordedSelection {
    None,
    Visual { rows: u32, cols: u32 },
    SingleLine { cols: u32 },
    VisualBlock { rows: u32, cols: u32 },
    VisualLine { rows: u32 },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ReplacementRecord {
    pub(crate) range: AnchorRange,
    pub(crate) text: KeyText,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TransactionState {
    pub(crate) current_tx: Option<TransactionId>,
    pub(crate) current_anchor: Option<AnchorRange>,
    pub(crate) undo_modes: Vec<(TransactionId, CommandName)>,
    pub(crate) undo_last_line_tx: Option<TransactionId>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SearchStateSnapshot<P> {
    pub(crate) query: Option<SearchQuery>,
    pub(crate) replacement: Option<KeyText>,
    pub(crate) direction: SearchDirection,
    pub(crate) count: Count,
    pub(crate) case_policy: CasePolicy,
    pub(crate) wrap_policy: WrapPolicy,
    pub(crate) include_ignored: bool,
    pub(crate) cmd_f_search: bool,
    pub(crate) prior_selections: Vec<AnchorRange>,
    pub(crate) prior_operator: Option<P>,
    pub(crate) prior_mode: Option<DialectModeSnapshot>,
    pub(crate) helix_select: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DialectModeSnapshot {
    VimNormal,
    VimInsert,
    VimReplace,
    VimVisual,
    VimVisualLine,
    VimVisualBlock,
    VimOperatorPending,
    HelixNormal,
    HelixSelect,
    HelixInsert,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CountState {
    pub(crate) pre_count: Option<Count>,
    pub(crate) post_count: Option<Count>,
    pub(crate) policy: CountPolicy,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectionHistoryEntry {
    pub(crate) selections: SelectionSnapshot,
    pub(crate) transaction_id: Option<TransactionId>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectionHistorySnapshot {
    pub(crate) undo_stack: Vec<SelectionHistoryEntry>,
    pub(crate) redo_stack: Vec<SelectionHistoryEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct EngineMarker<K> {
    _kind: PhantomData<K>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct VimKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct HelixKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct BufferPoint {
    pub(crate) offset: MultiBufferOffset,
    pub(crate) bias: Bias,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DisplayCursorPoint {
    pub(crate) point: DisplayPoint,
    pub(crate) semantics: CursorSemantics,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AnchorRange {
    pub(crate) start: Anchor,
    pub(crate) end: Anchor,
    pub(crate) role: RangeRole,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct OffsetRange {
    pub(crate) start: MultiBufferOffset,
    pub(crate) end: MultiBufferOffset,
    pub(crate) role: RangeRole,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DisplayRange {
    pub(crate) start: DisplayPoint,
    pub(crate) end: DisplayPoint,
    pub(crate) role: RangeRole,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ModalSelection {
    pub(crate) range: AnchorRange,
    pub(crate) goal: SelectionGoal,
    pub(crate) reversed: bool,
    pub(crate) storage: SelectionStorage,
    pub(crate) cursor_semantics: CursorSemantics,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectionSnapshot {
    pub(crate) selections: Vec<ModalSelection>,
    pub(crate) newest_index: usize,
    pub(crate) primary_index: Option<usize>,
    pub(crate) line_mode: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectionTransform {
    pub(crate) empty_policy: EmptySelectionPolicy,
    pub(crate) reversed_policy: ReversedSelectionPolicy,
    pub(crate) expansion: RangeExpansion,
    pub(crate) line_end_policy: LineEndPolicy,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct MotionRequest<M> {
    pub(crate) motion: M,
    pub(crate) count: Option<Count>,
    pub(crate) forced: bool,
    pub(crate) kind: MotionKind,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct MotionOutput {
    pub(crate) head: DisplayPoint,
    pub(crate) tail: DisplayPoint,
    pub(crate) goal: SelectionGoal,
    pub(crate) kind: MotionKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WordBoundaryRequest {
    pub(crate) flavor: WordFlavor,
    pub(crate) side: BoundarySide,
    pub(crate) direction: Direction,
    pub(crate) punctuation: PunctuationPolicy,
    pub(crate) count: Count,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TextObjectRequest {
    pub(crate) object: TextObjectSpec,
    pub(crate) boundary: TextObjectBoundary,
    pub(crate) bracket_policy: BracketOpeningPolicy,
    pub(crate) position: ObjectSearchPosition,
    pub(crate) count: Option<Count>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SurroundRequest {
    pub(crate) kind: SurroundKind,
    pub(crate) target: TextObjectSpec,
    pub(crate) replacement: Option<KeyText>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SearchRequest {
    pub(crate) query: SearchQuery,
    pub(crate) direction: SearchDirection,
    pub(crate) scope: SearchScope,
    pub(crate) case_policy: CasePolicy,
    pub(crate) wrap_policy: WrapPolicy,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RegisterRequest {
    pub(crate) scope: RegisterScope,
    pub(crate) use_system_clipboard: UseSystemClipboardPolicy,
    pub(crate) intent: EditIntent,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct JumpLabel {
    pub(crate) label: [char; 2],
    pub(crate) target_range: AnchorRange,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HelixJumpLabel {
    pub(crate) label: [char; 2],
    pub(crate) range: AnchorRange,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct JumpCandidate {
    pub(crate) word_start: MultiBufferOffset,
    pub(crate) word_end: MultiBufferOffset,
    pub(crate) first_two_end: MultiBufferOffset,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct JumpSkipData {
    pub(crate) points: Vec<MultiBufferOffset>,
    pub(crate) ranges: Vec<Range<MultiBufferOffset>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct JumpLabelFit {
    pub(crate) hide_end_offset: MultiBufferOffset,
    pub(crate) left_shift_px: f32,
    pub(crate) scale_factor: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct JumpLabelFitBudget {
    pub(crate) max_left_shift_px: f32,
    pub(crate) allowed_trailing_hide_end: MultiBufferOffset,
    pub(crate) preserve_full_scale: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct JumpOverlay {
    pub(crate) labels: Vec<JumpLabel>,
    pub(crate) covered_ranges: Vec<AnchorRange>,
    pub(crate) ordering: JumpLabelOrdering,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct JumpRequest {
    pub(crate) target_kind: JumpTargetKind,
    pub(crate) alphabet: JumpLabelAlphabet,
    pub(crate) ordering: JumpLabelOrdering,
    pub(crate) completion: JumpCompletionPolicy,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CommandContext {
    pub(crate) dialect: Dialect,
    pub(crate) count: Option<Count>,
    pub(crate) register: RegisterScope,
    pub(crate) point_semantics: PointCommandSemantics,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PendingInput<I> {
    pub(crate) input: I,
    pub(crate) status: CommandName,
    pub(crate) repeatability: Repeatability,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InsertModeCursorShape {
    Inherit,
    Explicit(CursorShape),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ModalCursorShapes {
    pub(crate) normal: CursorShape,
    pub(crate) insert: InsertModeCursorShape,
    pub(crate) visual: CursorShape,
    pub(crate) replace: CursorShape,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct EngineTransition<M> {
    pub(crate) from: M,
    pub(crate) to: M,
    pub(crate) preserve_selection: bool,
    pub(crate) transform: SelectionTransform,
}
