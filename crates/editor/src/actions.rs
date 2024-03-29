//! This module contains all actions supported by [`Editor`].
use super::*;

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
pub struct SelectToEndOfLine {
    #[serde(default)]
    pub(super) stop_at_soft_wraps: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct ToggleCodeActions {
    #[serde(default)]
    pub deployed_from_indicator: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct ConfirmCompletion {
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
    pub buffer_row: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct UnfoldAt {
    pub buffer_row: u32,
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

impl_actions!(
    editor,
    [
        SelectNext,
        SelectPrevious,
        SelectToBeginningOfLine,
        MovePageUp,
        MovePageDown,
        SelectToEndOfLine,
        ToggleCodeActions,
        ConfirmCompletion,
        ConfirmCodeAction,
        ToggleComments,
        FoldAt,
        UnfoldAt,
        MoveUpByLines,
        MoveDownByLines,
        SelectUpByLines,
        SelectDownByLines,
    ]
);

gpui::actions!(
    editor,
    [
        AcceptPartialCopilotSuggestion,
        AcceptPartialInlineCompletion,
        AddSelectionAbove,
        AddSelectionBelow,
        Backspace,
        Cancel,
        ConfirmRename,
        ContextMenuFirst,
        ContextMenuLast,
        ContextMenuNext,
        ContextMenuPrev,
        ConvertToKebabCase,
        ConvertToLowerCamelCase,
        ConvertToLowerCase,
        ConvertToSnakeCase,
        ConvertToTitleCase,
        ConvertToUpperCamelCase,
        ConvertToUpperCase,
        Copy,
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
        DeleteToNextWordEnd,
        DeleteToPreviousSubwordStart,
        DeleteToPreviousWordStart,
        DisplayCursorNames,
        DuplicateLineUp,
        DuplicateLineDown,
        ExpandMacroRecursively,
        FindAllReferences,
        Fold,
        FoldSelectedRanges,
        Format,
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
        MoveToBeginningOfLine,
        MoveToEnclosingBracket,
        MoveToEnd,
        MoveToEndOfLine,
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
        RevealInFinder,
        ReverseLines,
        RevertSelectedHunks,
        ScrollCursorBottom,
        ScrollCursorCenter,
        ScrollCursorTop,
        SelectAll,
        SelectAllMatches,
        SelectDown,
        SelectLargerSyntaxNode,
        SelectLeft,
        SelectLine,
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
        ShowCompletions,
        ShowInlineCompletion,
        ShuffleLines,
        SortLinesCaseInsensitive,
        SortLinesCaseSensitive,
        SplitSelectionIntoLines,
        Tab,
        TabPrev,
        ToggleGitBlame,
        ToggleInlayHints,
        ToggleLineNumbers,
        ToggleSoftWrap,
        Transpose,
        Undo,
        UndoSelection,
        UnfoldLines,
        UniqueLinesCaseSensitive,
        UniqueLinesCaseInsensitive
    ]
);
