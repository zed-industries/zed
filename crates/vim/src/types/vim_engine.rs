use super::shared::*;
use editor::Anchor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum VimMode {
    Normal,
    Insert,
    Replace,
    Visual,
    VisualLine,
    VisualBlock,
    OperatorPending,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum VimVisualKind {
    Characterwise,
    Linewise,
    Blockwise,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum VimTrueOperator {
    Change,
    Delete,
    Yank,
    Replace,
    Object {
        around: bool,
    },
    AddSurrounds {
        target: Option<SurroundTarget>,
    },
    ChangeSurrounds {
        target: Option<TextObjectSpec>,
        opening: bool,
        bracket_anchors: Vec<Option<(Anchor, Anchor)>>,
    },
    DeleteSurrounds,
    Indent,
    Outdent,
    AutoIndent,
    Rewrap,
    ShellCommand,
    Lowercase,
    Uppercase,
    OppositeCase,
    Rot13,
    Rot47,
    ToggleComments,
    ToggleBlockComments,
    ReplaceWithRegister,
    Exchange,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SurroundTarget {
    Motion(VimMotion),
    Object(TextObjectRequest),
    Character(char),
    Pair { open: char, close: char },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum VimMotion {
    Left,
    WrappingLeft,
    Down {
        display_lines: bool,
    },
    Up {
        display_lines: bool,
    },
    Right,
    WrappingRight,
    NextWordStart {
        ignore_punctuation: bool,
    },
    NextWordEnd {
        ignore_punctuation: bool,
    },
    PreviousWordStart {
        ignore_punctuation: bool,
    },
    PreviousWordEnd {
        ignore_punctuation: bool,
    },
    NextSubwordStart {
        ignore_punctuation: bool,
    },
    NextSubwordEnd {
        ignore_punctuation: bool,
    },
    PreviousSubwordStart {
        ignore_punctuation: bool,
    },
    PreviousSubwordEnd {
        ignore_punctuation: bool,
    },
    FirstNonWhitespace {
        display_lines: bool,
    },
    CurrentLine,
    StartOfLine {
        display_lines: bool,
    },
    MiddleOfLine {
        display_lines: bool,
    },
    EndOfLine {
        display_lines: bool,
    },
    SentenceBackward,
    SentenceForward,
    StartOfParagraph,
    EndOfParagraph,
    StartOfDocument,
    EndOfDocument,
    Matching {
        match_quotes: bool,
    },
    GoToPercentage,
    UnmatchedForward {
        character: char,
    },
    UnmatchedBackward {
        character: char,
    },
    FindForward {
        target: char,
        before: bool,
        range: FindRangeKind,
        smartcase: bool,
    },
    FindBackward {
        target: char,
        after: bool,
        range: FindRangeKind,
        smartcase: bool,
    },
    Sneak {
        first_char: char,
        second_char: char,
        smartcase: bool,
    },
    SneakBackward {
        first_char: char,
        second_char: char,
        smartcase: bool,
    },
    RepeatFind {
        last_find: Box<VimMotion>,
    },
    RepeatFindReversed {
        last_find: Box<VimMotion>,
    },
    NextLineStart,
    PreviousLineStart,
    StartOfLineDownward,
    EndOfLineDownward,
    GoToColumn,
    WindowTop,
    WindowMiddle,
    WindowBottom,
    NextSectionStart,
    NextSectionEnd,
    PreviousSectionStart,
    PreviousSectionEnd,
    NextMethodStart,
    NextMethodEnd,
    PreviousMethodStart,
    PreviousMethodEnd,
    NextComment,
    PreviousComment,
    PreviousLesserIndent,
    PreviousGreaterIndent,
    PreviousSameIndent,
    NextLesserIndent,
    NextGreaterIndent,
    NextSameIndent,
    ZedSearchResult {
        prior_selections: Vec<AnchorRange>,
        new_selections: Vec<AnchorRange>,
    },
    Jump {
        anchor: Anchor,
        line: bool,
    },
    SearchMatch {
        direction: SearchDirection,
    },
    TextObject(TextObjectRequest),
    Mark {
        name: RegisterName,
        linewise: bool,
    },
    Paragraph {
        direction: Direction,
    },
    Sentence {
        direction: Direction,
    },
    Section {
        direction: Direction,
        side: BoundarySide,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VimOperator {
    pub(crate) operator: VimTrueOperator,
    pub(crate) count: Option<Count>,
    pub(crate) register: RegisterScope,
    pub(crate) forced_motion: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum VimPendingState {
    Operator(VimOperator),
    FindForward {
        before: bool,
        multiline: bool,
        operator: Option<VimOperator>,
    },
    FindBackward {
        after: bool,
        multiline: bool,
        operator: Option<VimOperator>,
    },
    SneakForward {
        first_char: Option<char>,
        operator: Option<VimOperator>,
    },
    SneakBackward {
        first_char: Option<char>,
        operator: Option<VimOperator>,
    },
    TextObject {
        boundary: TextObjectBoundary,
        operator: Option<VimOperator>,
    },
    Mark {
        linewise: bool,
    },
    JumpToMark {
        linewise: bool,
    },
    Register {
        intent: VimRegisterIntent,
    },
    Digraph {
        first_char: Option<char>,
    },
    Literal {
        prefix: Option<KeyText>,
    },
    Surround {
        kind: VimSurroundPendingKind,
        operator: Option<VimOperator>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum VimRegisterIntent {
    Select,
    ReplaceWithRegister,
    RecordMacro,
    ReplayMacro,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum VimSurroundPendingKind {
    Add,
    Change,
    Delete,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VimPendingCommand {
    pub(crate) state: VimPendingState,
    pub(crate) status: CommandName,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VimMotionPlan {
    pub(crate) request: MotionRequest<VimMotion>,
    pub(crate) operator: Option<VimOperator>,
    pub(crate) visual_kind: Option<VimVisualKind>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VimNormalSelectionPolicy {
    pub(crate) collapse_after_motion: bool,
    pub(crate) clip_at_line_ends: bool,
    pub(crate) operator_range_kind: MotionKind,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VimVisualSelectionPolicy {
    pub(crate) visual_kind: VimVisualKind,
    pub(crate) preserve_anchor: bool,
    pub(crate) inclusive_selection: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VimOperatorPolicy {
    pub(crate) operator: VimTrueOperator,
    pub(crate) accepts_linewise_motion: bool,
    pub(crate) accepts_forced_motion: bool,
    pub(crate) repeatability: Repeatability,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VimRegisterPolicy {
    pub(crate) selected_register: RegisterScope,
    pub(crate) system_clipboard: UseSystemClipboardPolicy,
    pub(crate) yank_linewise: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VimDotRepeatState {
    pub(crate) action: Option<CommandName>,
    pub(crate) count: Option<Count>,
    pub(crate) register: RegisterScope,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VimEngineState {
    pub(crate) mode: VimMode,
    pub(crate) last_mode: VimMode,
    pub(crate) pending: Option<VimPendingCommand>,
    pub(crate) register_policy: VimRegisterPolicy,
    pub(crate) dot_repeat: VimDotRepeatState,
    pub(crate) temp_mode: bool,
    pub(crate) exit_temporary_mode: bool,
    pub(crate) operator_stack: Vec<VimTrueOperator>,
    pub(crate) stored_visual_mode: Option<(VimMode, Vec<bool>)>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum NormalAction {
    InsertAfter,
    InsertBefore,
    InsertFirstNonWhitespace,
    InsertEndOfLine,
    InsertLineAbove,
    InsertLineBelow,
    InsertEmptyLineAbove,
    InsertEmptyLineBelow,
    InsertAtPrevious,
    JoinLines {
        whitespace: JoinWhitespacePolicy,
    },
    DeleteLeft,
    DeleteRight,
    HelixDelete,
    HelixCollapseSelection,
    ChangeToEndOfLine,
    DeleteToEndOfLine,
    Yank,
    YankLine,
    YankToEndOfLine,
    ChangeCase,
    Convert(ConvertTarget),
    ToggleComments,
    ToggleBlockComments,
    ShowLocation,
    Undo(UndoTarget),
    Redo,
    GoToTab,
    GoToPreviousTab,
    GoToPreviousReference,
    GoToNextReference,
    Increment {
        direction: IncrementDirection,
        mode: IncrementMode,
    },
    Substitute {
        line_mode: bool,
    },
    Paste(PasteSpec),
    Rewrap {
        line_length: Option<usize>,
    },
    Indent(IndentEditDirection),
    Scroll(ScrollAction),
    Repeat(RepeatAction),
    Search(SearchAction),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum InsertAction {
    NormalBefore,
    TemporaryNormal,
    InsertFromAbove,
    InsertFromBelow,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ReplaceAction {
    ToggleReplace,
    UndoReplace { count: Option<Count> },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum VisualAction {
    ToggleVisual,
    ToggleVisualLine,
    ToggleVisualBlock,
    VisualDelete { line_mode: bool },
    VisualYank { line_mode: bool },
    OtherEnd,
    OtherEndRowAware,
    SelectNext,
    SelectPrevious,
    SelectNextMatch,
    SelectPreviousMatch,
    SelectSyntaxNode { direction: SyntaxNodeDirection },
    RestoreVisualSelection,
    VisualInsertEndOfLine,
    VisualInsertFirstNonWhitespace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScrollAction {
    LineUp,
    LineDown,
    ColumnRight,
    ColumnLeft,
    ScrollUp,
    ScrollDown,
    PageUp,
    PageDown,
    HalfPageRight,
    HalfPageLeft,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RepeatAction {
    Repeat,
    EndRepeat,
    ToggleRecord,
    ReplayLastRecording,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SearchAction {
    MoveToNext {
        case_sensitive: bool,
        partial_word: bool,
        regex: bool,
    },
    MoveToPrevious {
        case_sensitive: bool,
        partial_word: bool,
        regex: bool,
    },
    SearchUnderCursor {
        backwards: bool,
        case_sensitive: bool,
        partial_word: bool,
        regex: bool,
    },
    Search {
        backwards: bool,
        regex: bool,
    },
    FindCommand {
        query: SearchQuery,
        backwards: bool,
    },
    ReplaceCommand {
        range: CommandRange,
        replacement: ReplacementSpec,
    },
    SearchSubmit,
    MoveToNextMatch,
    MoveToPreviousMatch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum IndentAction {
    Indent,
    Outdent,
    AutoIndent,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PasteSpec {
    pub(crate) placement: PastePlacement,
    pub(crate) clipboard_policy: PasteClipboardPolicy,
    pub(crate) preserve_clipboard: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ReplacementSpec {
    pub(crate) search: SearchQuery,
    pub(crate) replacement: KeyText,
    pub(crate) case_sensitive: Option<bool>,
    pub(crate) flag_n: bool,
    pub(crate) flag_g: bool,
    pub(crate) flag_c: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CommandPosition {
    CurrentLine { offset: i32 },
    LastLine { offset: i32 },
    Line { row: u32, offset: i32 },
    Mark { name: RegisterName, offset: i32 },
    SearchForward,
    SearchBackward,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CommandRange {
    pub(crate) start: CommandPosition,
    pub(crate) end: Option<CommandPosition>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum VimOption {
    Wrap(bool),
    Number(bool),
    RelativeNumber(bool),
    IgnoreCase(bool),
    GDefault(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum DeleteMarks {
    Marks(KeyText),
    AllLocal,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ExCommand {
    GoToLine {
        range: CommandRange,
    },
    Yank {
        range: CommandRange,
    },
    WithRange {
        restore_selection: bool,
        range: CommandRange,
        action: CommandName,
    },
    WithCount {
        count: u32,
        action: CommandName,
    },
    VimSet {
        options: Vec<VimOption>,
    },
    VimSave {
        range: Option<CommandRange>,
        save_intent: Option<SaveIntent>,
        filename: KeyText,
    },
    VimSplit {
        vertical: bool,
        filename: KeyText,
    },
    DeleteMarks(DeleteMarks),
    VimEdit {
        filename: KeyText,
    },
    VimRead {
        range: Option<CommandRange>,
        filename: KeyText,
    },
    VimNorm {
        range: Option<CommandRange>,
        command: KeyText,
        override_rows: Option<Vec<u32>>,
    },
    OnMatchingLines(OnMatchingLines),
    ShellExec(ShellExec),
    VisualCommand,
    CountCommand,
    ShellCommand,
    ArgumentRequired,
    Generated {
        short: CommandName,
        suffix: CommandName,
        action: CommandName,
        bang_action: Option<CommandName>,
        accepts_args: bool,
        accepts_range: bool,
        accepts_count: bool,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SaveIntent {
    Save,
    FormatAndSave,
    SaveWithoutFormat,
    SaveAll,
    SaveAs,
    Close,
    Overwrite,
    Skip,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct OnMatchingLines {
    pub(crate) range: CommandRange,
    pub(crate) pattern: SearchQuery,
    pub(crate) action: CommandName,
    pub(crate) invert: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ShellExec {
    pub(crate) command: KeyText,
    pub(crate) range: Option<CommandRange>,
    pub(crate) is_read: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum VimCommand {
    SwitchMode(VimMode),
    PushOperator(VimOperator),
    PushOperatorState(VimTrueOperator),
    PushPending(VimPendingState),
    ApplyMotion(VimMotionPlan),
    ApplyTextObject(TextObjectRequest),
    Normal(NormalAction),
    Insert(InsertAction),
    Replace(ReplaceAction),
    Visual(VisualAction),
    Ex(ExCommand),
    SelectRegister(RegisterScope),
    BeginRecording(RegisterScope),
    ReplayRegister(RegisterScope),
    EnterInsert,
    Append,
    Paste(PasteSpec),
    Search(SearchRequest),
    ChangeList(ChangeListDirection),
    JumpToMark(RegisterName),
    ClearPending,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VimEngineOutput {
    pub(crate) mode: VimMode,
    pub(crate) selections: SelectionSnapshot,
    pub(crate) ui_feedback: UiFeedback,
    pub(crate) repeatability: Repeatability,
}
