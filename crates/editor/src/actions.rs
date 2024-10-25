//! This module contains all actions supported by [`Editor`].
use super::*;
use gpui::action_as;
use util::serde::default_true;

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct SelectNext {
    #[serde(default)]
    pub replace_newest: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct SelectPrevious {
    #[serde(default)]
    pub replace_newest: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct MoveToBeginningOfLine {
    #[serde(default = "default_true")]
    pub stop_at_soft_wraps: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct SelectToBeginningOfLine {
    #[serde(default)]
    pub(super) stop_at_soft_wraps: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct MovePageUp {
    #[serde(default)]
    pub(super) center_cursor: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct MovePageDown {
    #[serde(default)]
    pub(super) center_cursor: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct MoveToEndOfLine {
    #[serde(default = "default_true")]
    pub stop_at_soft_wraps: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct SelectToEndOfLine {
    #[serde(default)]
    pub(super) stop_at_soft_wraps: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct ToggleCodeActions {
    // Display row from which the action was deployed.
    #[serde(default)]
    pub deployed_from_indicator: Option<DisplayRow>,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct ConfirmCompletion {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct ComposeCompletion {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct ConfirmCodeAction {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct ToggleComments {
    #[serde(default)]
    pub advance_downwards: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct FoldAt {
    pub buffer_row: MultiBufferRow,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct UnfoldAt {
    pub buffer_row: MultiBufferRow,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct MoveUpByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct MoveDownByLines {
    #[serde(default)]
    pub(super) lines: u32,
}
#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct SelectUpByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct SelectDownByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct ExpandExcerpts {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct ExpandExcerptsUp {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct ExpandExcerptsDown {
    #[serde(default)]
    pub(super) lines: u32,
}
#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct ShowCompletions {
    #[serde(default)]
    pub(super) trigger: Option<String>,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct HandleInput(pub String);

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct DeleteToNextWordEnd {
    #[serde(default)]
    pub ignore_newlines: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct DeleteToPreviousWordStart {
    #[serde(default)]
    pub ignore_newlines: bool,
}

impl_actions!(
    editor,
    [
        ComposeCompletion,
        ConfirmCodeAction,
        ConfirmCompletion,
        DeleteToNextWordEnd,
        DeleteToPreviousWordStart,
        ExpandExcerpts,
        ExpandExcerptsDown,
        ExpandExcerptsUp,
        FoldAt,
        HandleInput,
        MoveDownByLines,
        MovePageDown,
        MovePageUp,
        MoveToBeginningOfLine,
        MoveToEndOfLine,
        MoveUpByLines,
        SelectDownByLines,
        SelectNext,
        SelectPrevious,
        SelectToBeginningOfLine,
        SelectToEndOfLine,
        SelectUpByLines,
        ShowCompletions,
        ToggleCodeActions,
        ToggleComments,
        UnfoldAt,
    ]
);

gpui::actions!(
    editor,
    [
        AcceptInlineCompletion,
        AcceptPartialCopilotSuggestion,
        AcceptPartialInlineCompletion,
        AddSelectionAbove,
        AddSelectionBelow,
        ApplyDiffHunk,
        Backspace,
        Cancel,
        CancelLanguageServerWork,
        ConfirmRename,
        ContextMenuFirst,
        ContextMenuLast,
        ContextMenuNext,
        ContextMenuPrev,
        ConvertToKebabCase,
        ConvertToLowerCamelCase,
        ConvertToLowerCase,
        ConvertToOppositeCase,
        ConvertToSnakeCase,
        ConvertToTitleCase,
        ConvertToUpperCamelCase,
        ConvertToUpperCase,
        Copy,
        CopyFileLocation,
        CopyHighlightJson,
        CopyPath,
        CopyPermalinkToLine,
        CopyRelativePath,
        Cut,
        CutToEndOfLine,
        Delete,
        DeleteLine,
        DeleteToBeginningOfLine,
        DeleteToEndOfLine,
        DeleteToNextSubwordEnd,
        DeleteToPreviousSubwordStart,
        DisplayCursorNames,
        DuplicateLineDown,
        DuplicateLineUp,
        ExpandAllHunkDiffs,
        ExpandMacroRecursively,
        FindAllReferences,
        Fold,
        FoldAll,
        FoldRecursive,
        FoldSelectedRanges,
        ToggleFold,
        ToggleFoldRecursive,
        Format,
        FormatSelections,
        GoToDeclaration,
        GoToDeclarationSplit,
        GoToDefinition,
        GoToDefinitionSplit,
        GoToDiagnostic,
        GoToHunk,
        GoToImplementation,
        GoToImplementationSplit,
        GoToPrevDiagnostic,
        GoToPrevHunk,
        GoToTypeDefinition,
        GoToTypeDefinitionSplit,
        HalfPageDown,
        HalfPageUp,
        Hover,
        Indent,
        JoinLines,
        LineDown,
        LineUp,
        MoveDown,
        MoveLeft,
        MoveLineDown,
        MoveLineUp,
        MoveRight,
        MoveToBeginning,
        MoveToEnclosingBracket,
        MoveToEnd,
        MoveToEndOfParagraph,
        MoveToNextSubwordEnd,
        MoveToNextWordEnd,
        MoveToPreviousSubwordStart,
        MoveToPreviousWordStart,
        MoveToStartOfParagraph,
        MoveUp,
        Newline,
        NewlineAbove,
        NewlineBelow,
        NextInlineCompletion,
        NextScreen,
        OpenExcerpts,
        OpenExcerptsSplit,
        OpenProposedChangesEditor,
        OpenFile,
        OpenPermalinkToLine,
        OpenUrl,
        Outdent,
        PageDown,
        PageUp,
        Paste,
        PreviousInlineCompletion,
        Redo,
        RedoSelection,
        Rename,
        RestartLanguageServer,
        RevealInFileManager,
        ReverseLines,
        RevertFile,
        ReloadFile,
        RevertSelectedHunks,
        Rewrap,
        ScrollCursorBottom,
        ScrollCursorCenter,
        ScrollCursorCenterTopBottom,
        ScrollCursorTop,
        SelectAll,
        SelectAllMatches,
        SelectDown,
        SelectEnclosingSymbol,
        SelectLargerSyntaxNode,
        SelectLeft,
        SelectLine,
        SelectPageDown,
        SelectPageUp,
        SelectRight,
        SelectSmallerSyntaxNode,
        SelectToBeginning,
        SelectToEnd,
        SelectToEndOfParagraph,
        SelectToNextSubwordEnd,
        SelectToNextWordEnd,
        SelectToPreviousSubwordStart,
        SelectToPreviousWordStart,
        SelectToStartOfParagraph,
        SelectUp,
        ShowCharacterPalette,
        ShowInlineCompletion,
        ShowSignatureHelp,
        ShuffleLines,
        SortLinesCaseInsensitive,
        SortLinesCaseSensitive,
        SplitSelectionIntoLines,
        SwitchSourceHeader,
        Tab,
        TabPrev,
        ToggleAutoSignatureHelp,
        ToggleGitBlame,
        ToggleGitBlameInline,
        ToggleHunkDiff,
        ToggleIndentGuides,
        ToggleInlayHints,
        ToggleInlineCompletions,
        ToggleLineNumbers,
        ToggleRelativeLineNumbers,
        ToggleSelectionMenu,
        ToggleSoftWrap,
        ToggleTabBar,
        Transpose,
        Undo,
        UndoSelection,
        UnfoldAll,
        UnfoldLines,
        UnfoldRecursive,
        UniqueLinesCaseInsensitive,
        UniqueLinesCaseSensitive,
    ]
);

action_as!(outline, ToggleOutline as Toggle);

action_as!(go_to_line, ToggleGoToLine as Toggle);
