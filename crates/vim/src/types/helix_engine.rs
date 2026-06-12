use super::shared::*;
use editor::Anchor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HelixMode {
    Normal,
    Select,
    Insert,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HelixSelectionShape {
    Character,
    Line,
    Object,
    SearchMatch,
    SyntaxNode,
    Multiple,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum HelixMovement {
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
    FirstNonWhitespace,
    StartOfLine,
    EndOfLine {
        policy: LineEndPolicy,
        display_lines: bool,
    },
    MiddleOfLine {
        display_lines: bool,
    },
    NextLineStart,
    PreviousLineStart,
    CurrentLine,
    StartOfDocument,
    EndOfDocument,
    GoToPercentage,
    WindowTop,
    WindowMiddle,
    WindowBottom,
    NextWordStart {
        flavor: WordFlavor,
        punctuation: PunctuationPolicy,
    },
    NextWordEnd {
        flavor: WordFlavor,
        punctuation: PunctuationPolicy,
    },
    PreviousWordStart {
        flavor: WordFlavor,
        punctuation: PunctuationPolicy,
    },
    PreviousWordEnd {
        flavor: WordFlavor,
        punctuation: PunctuationPolicy,
    },
    FindForward {
        target: char,
        before: bool,
        multiline: bool,
        smartcase: bool,
    },
    FindBackward {
        target: char,
        after: bool,
        multiline: bool,
        smartcase: bool,
    },
    MatchObject,
    NextObject(TextObjectRequest),
    PreviousObject(TextObjectRequest),
    Section {
        direction: Direction,
        target: SectionTarget,
    },
    Method {
        direction: Direction,
        boundary: MethodBoundary,
    },
    Comment {
        direction: Direction,
    },
    Indent {
        direction: Direction,
        relation: IndentRelation,
    },
    SyntaxNode {
        direction: SyntaxNodeDirection,
    },
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HelixSelectionCommand {
    Collapse,
    FlipAnchor,
    KeepPrimary,
    SplitIntoLines,
    SelectAll,
    SelectRegex,
    SelectLine,
    SelectCurrentObject,
    SelectNextObject,
    SelectPreviousObject,
    DuplicateBelow,
    DuplicateAbove,
    SelectSmallerSyntaxNode,
    SelectLargerSyntaxNode,
    SelectNextSyntaxNode,
    SelectPreviousSyntaxNode,
    SelectAllMatches,
    KeepNewestSelection,
    RemovePrimarySelection,
    AlignSelections,
    TrimSelections,
    SetSearchFromSelection,
    RotateSelectionsForward,
    RotateSelectionsBackward,
    ReverseSelections,
    SelectMatch { direction: SearchDirection },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HelixEditCommand {
    Insert,
    Append,
    InsertAtLineEnd,
    Substitute {
        yank: bool,
    },
    Delete {
        yank: bool,
    },
    Yank,
    Paste {
        before: bool,
    },
    Replace,
    Indent,
    Outdent,
    Format,
    ToggleComment,
    ToggleBlockComment,
    ChangeCase,
    Convert(ConvertTarget),
    Increment {
        direction: IncrementDirection,
        mode: IncrementMode,
    },
    Rewrap {
        line_length: Option<usize>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum HelixPendingState {
    Match,
    Next {
        around: bool,
    },
    Previous {
        around: bool,
    },
    SurroundAdd,
    SurroundReplace {
        replaced_char: Option<char>,
    },
    SurroundDelete,
    Jump {
        behaviour: HelixJumpCompletion,
        request: JumpRequest,
        first_char: Option<char>,
        labels: Vec<HelixJumpLabel>,
    },
    FindForward {
        before: bool,
        multiline: bool,
    },
    FindBackward {
        after: bool,
        multiline: bool,
    },
    Replace,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HelixPendingCommand {
    pub(crate) state: HelixPendingState,
    pub(crate) status: CommandName,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HelixMovementPlan {
    pub(crate) request: MotionRequest<HelixMovement>,
    pub(crate) selection_shape: HelixSelectionShape,
    pub(crate) cursor_semantics: CursorSemantics,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HelixNormalSelectionPolicy {
    pub(crate) cursor_is_selection: bool,
    pub(crate) collapse_to_included_character: bool,
    pub(crate) line_end_policy: LineEndPolicy,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HelixSelectSelectionPolicy {
    pub(crate) preserve_anchor: bool,
    pub(crate) include_head_character: bool,
    pub(crate) line_end_policy: LineEndPolicy,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HelixObjectPolicy {
    pub(crate) object: TextObjectSpec,
    pub(crate) boundary: TextObjectBoundary,
    pub(crate) bracket_policy: BracketOpeningPolicy,
    pub(crate) position: ObjectSearchPosition,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HelixSurroundPolicy {
    pub(crate) literal_pairs_only: bool,
    pub(crate) symmetric_fallback: bool,
    pub(crate) target: SurroundRequest,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HelixJumpPolicy {
    pub(crate) target_kind: JumpTargetKind,
    pub(crate) completion: JumpCompletionPolicy,
    pub(crate) cursor_semantics: CursorSemantics,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HelixRegisterPolicy {
    pub(crate) selected_register: RegisterScope,
    pub(crate) system_clipboard: UseSystemClipboardPolicy,
    pub(crate) selection_order_preserved: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HelixEngineState {
    pub(crate) mode: HelixMode,
    pub(crate) last_mode: HelixMode,
    pub(crate) pending: Option<HelixPendingCommand>,
    pub(crate) register_policy: HelixRegisterPolicy,
    pub(crate) pending_stack: Vec<HelixPendingState>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum HelixCommand {
    SwitchMode(HelixMode),
    Move(HelixMovementPlan),
    Edit(HelixEditCommand),
    Select(HelixSelectionCommand),
    SelectObject(HelixObjectPolicy),
    Surround(HelixSurroundPolicy),
    Jump(HelixJumpPolicy),
    Search(SearchRequest),
    GotoLastModification,
    JumpToWord,
    PushPending(HelixPendingState),
    ClearPending,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HelixEngineOutput {
    pub(crate) mode: HelixMode,
    pub(crate) selections: SelectionSnapshot,
    pub(crate) ui_feedback: UiFeedback,
    pub(crate) point_semantics: PointCommandSemantics,
}
