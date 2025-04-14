//! This module contains all actions supported by [`Editor`].
use super::*;
use gpui::{action_as, action_with_deprecated_aliases, actions};
use schemars::JsonSchema;
use util::serde::default_true;

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SelectNext {
    #[serde(default)]
    pub replace_newest: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SelectPrevious {
    #[serde(default)]
    pub replace_newest: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MoveToBeginningOfLine {
    #[serde(default = "default_true")]
    pub stop_at_soft_wraps: bool,
    #[serde(default)]
    pub stop_at_indent: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SelectToBeginningOfLine {
    #[serde(default)]
    pub(super) stop_at_soft_wraps: bool,
    #[serde(default)]
    pub stop_at_indent: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeleteToBeginningOfLine {
    #[serde(default)]
    pub(super) stop_at_indent: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MovePageUp {
    #[serde(default)]
    pub(super) center_cursor: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MovePageDown {
    #[serde(default)]
    pub(super) center_cursor: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MoveToEndOfLine {
    #[serde(default = "default_true")]
    pub stop_at_soft_wraps: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SelectToEndOfLine {
    #[serde(default)]
    pub(super) stop_at_soft_wraps: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ToggleCodeActions {
    // Display row from which the action was deployed.
    #[serde(default)]
    #[serde(skip)]
    pub deployed_from_indicator: Option<DisplayRow>,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConfirmCompletion {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ComposeCompletion {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConfirmCodeAction {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ToggleComments {
    #[serde(default)]
    pub advance_downwards: bool,
    #[serde(default)]
    pub ignore_indent: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MoveUpByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MoveDownByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SelectUpByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SelectDownByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExpandExcerpts {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExpandExcerptsUp {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExpandExcerptsDown {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ShowCompletions {
    #[serde(default)]
    pub(super) trigger: Option<String>,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
pub struct HandleInput(pub String);

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeleteToNextWordEnd {
    #[serde(default)]
    pub ignore_newlines: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeleteToPreviousWordStart {
    #[serde(default)]
    pub ignore_newlines: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
pub struct FoldAtLevel(pub u32);

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SpawnNearestTask {
    #[serde(default)]
    pub reveal: task::RevealStrategy,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Default)]
pub enum UuidVersion {
    #[default]
    V4,
    V7,
}

impl_actions!(
    editor,
    [
        ComposeCompletion,
        ConfirmCodeAction,
        ConfirmCompletion,
        DeleteToBeginningOfLine,
        DeleteToNextWordEnd,
        DeleteToPreviousWordStart,
        ExpandExcerpts,
        ExpandExcerptsDown,
        ExpandExcerptsUp,
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
        SpawnNearestTask,
        ShowCompletions,
        ToggleCodeActions,
        ToggleComments,
        FoldAtLevel,
    ]
);

actions!(
    editor,
    [
        AcceptEditPrediction,
        AcceptPartialCopilotSuggestion,
        AcceptPartialEditPrediction,
        AddSelectionAbove,
        AddSelectionBelow,
        ApplyAllDiffHunks,
        ApplyDiffHunk,
        Backspace,
        Cancel,
        CancelLanguageServerWork,
        ConfirmRename,
        ConfirmCompletionInsert,
        ConfirmCompletionReplace,
        ContextMenuFirst,
        ContextMenuLast,
        ContextMenuNext,
        ContextMenuPrevious,
        ConvertToKebabCase,
        ConvertToLowerCamelCase,
        ConvertToLowerCase,
        ConvertToOppositeCase,
        ConvertToSnakeCase,
        ConvertToTitleCase,
        ConvertToUpperCamelCase,
        ConvertToUpperCase,
        ConvertToRot13,
        ConvertToRot47,
        Copy,
        CopyAndTrim,
        CopyFileLocation,
        CopyHighlightJson,
        CopyFileName,
        CopyFileNameWithoutExtension,
        CopyPermalinkToLine,
        Cut,
        CutToEndOfLine,
        Delete,
        DeleteLine,
        DeleteToEndOfLine,
        DeleteToNextSubwordEnd,
        DeleteToPreviousSubwordStart,
        DisplayCursorNames,
        DuplicateLineDown,
        DuplicateLineUp,
        DuplicateSelection,
        ExpandMacroRecursively,
        FindAllReferences,
        FindNextMatch,
        FindPreviousMatch,
        Fold,
        FoldAll,
        FoldFunctionBodies,
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
        GoToPreviousHunk,
        GoToImplementation,
        GoToImplementationSplit,
        GoToPreviousDiagnostic,
        GoToTypeDefinition,
        GoToTypeDefinitionSplit,
        HalfPageDown,
        HalfPageUp,
        Hover,
        Indent,
        InsertUuidV4,
        InsertUuidV7,
        JoinLines,
        KillRingCut,
        KillRingYank,
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
        MoveToStartOfExcerpt,
        MoveToStartOfNextExcerpt,
        MoveToEndOfExcerpt,
        MoveToEndOfPreviousExcerpt,
        MoveUp,
        Newline,
        NewlineAbove,
        NewlineBelow,
        NextEditPrediction,
        NextScreen,
        OpenContextMenu,
        OpenExcerpts,
        OpenExcerptsSplit,
        OpenProposedChangesEditor,
        OpenDocs,
        OpenPermalinkToLine,
        OpenSelectionsInMultibuffer,
        OpenUrl,
        OrganizeImports,
        Outdent,
        AutoIndent,
        PageDown,
        PageUp,
        Paste,
        PreviousEditPrediction,
        Redo,
        RedoSelection,
        Rename,
        RestartLanguageServer,
        RevealInFileManager,
        ReverseLines,
        RevertFile,
        ReloadFile,
        Rewrap,
        ScrollCursorBottom,
        ScrollCursorCenter,
        ScrollCursorCenterTopBottom,
        ScrollCursorTop,
        SelectAll,
        SelectAllMatches,
        SelectToStartOfExcerpt,
        SelectToStartOfNextExcerpt,
        SelectToEndOfExcerpt,
        SelectToEndOfPreviousExcerpt,
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
        ShowEditPrediction,
        ShowSignatureHelp,
        ShowWordCompletions,
        ShuffleLines,
        SortLinesCaseInsensitive,
        SortLinesCaseSensitive,
        SplitSelectionIntoLines,
        StopLanguageServer,
        SwitchSourceHeader,
        Tab,
        Backtab,
        ToggleBreakpoint,
        ToggleCase,
        DisableBreakpoint,
        EnableBreakpoint,
        EditLogBreakpoint,
        DebuggerRunToCursor,
        DebuggerEvaluateSelectedText,
        ToggleAutoSignatureHelp,
        ToggleGitBlameInline,
        OpenGitBlameCommit,
        ToggleIndentGuides,
        ToggleInlayHints,
        ToggleInlineDiagnostics,
        ToggleEditPrediction,
        ToggleLineNumbers,
        SwapSelectionEnds,
        SetMark,
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

action_as!(go_to_line, ToggleGoToLine as Toggle);

action_with_deprecated_aliases!(editor, OpenSelectedFilename, ["editor::OpenFile"]);
action_with_deprecated_aliases!(editor, ToggleSelectedDiffHunks, ["editor::ToggleHunkDiff"]);
action_with_deprecated_aliases!(editor, ExpandAllDiffHunks, ["editor::ExpandAllHunkDiffs"]);
