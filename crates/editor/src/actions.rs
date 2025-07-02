//! This module contains all actions supported by [`Editor`].
use super::*;
use gpui::{Action, actions};
use schemars::JsonSchema;
use util::serde::default_true;

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectNext {
    #[serde(default)]
    pub replace_newest: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectPrevious {
    #[serde(default)]
    pub replace_newest: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MoveToBeginningOfLine {
    #[serde(default = "default_true")]
    pub stop_at_soft_wraps: bool,
    #[serde(default)]
    pub stop_at_indent: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectToBeginningOfLine {
    #[serde(default)]
    pub(super) stop_at_soft_wraps: bool,
    #[serde(default)]
    pub stop_at_indent: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct DeleteToBeginningOfLine {
    #[serde(default)]
    pub(super) stop_at_indent: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MovePageUp {
    #[serde(default)]
    pub(super) center_cursor: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MovePageDown {
    #[serde(default)]
    pub(super) center_cursor: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MoveToEndOfLine {
    #[serde(default = "default_true")]
    pub stop_at_soft_wraps: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectToEndOfLine {
    #[serde(default)]
    pub(super) stop_at_soft_wraps: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ToggleCodeActions {
    // Source from which the action was deployed.
    #[serde(default)]
    #[serde(skip)]
    pub deployed_from: Option<CodeActionSource>,
    // Run first available task if there is only one.
    #[serde(default)]
    #[serde(skip)]
    pub quick_launch: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub enum CodeActionSource {
    Indicator(DisplayRow),
    RunMenu(DisplayRow),
    QuickActionBar,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ConfirmCompletion {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ComposeCompletion {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ConfirmCodeAction {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ToggleComments {
    #[serde(default)]
    pub advance_downwards: bool,
    #[serde(default)]
    pub ignore_indent: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MoveUpByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MoveDownByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectUpByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectDownByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ExpandExcerpts {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ExpandExcerptsUp {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ExpandExcerptsDown {
    #[serde(default)]
    pub(super) lines: u32,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ShowCompletions {
    #[serde(default)]
    pub(super) trigger: Option<String>,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
pub struct HandleInput(pub String);

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct DeleteToNextWordEnd {
    #[serde(default)]
    pub ignore_newlines: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct DeleteToPreviousWordStart {
    #[serde(default)]
    pub ignore_newlines: bool,
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
pub struct FoldAtLevel(pub u32);

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
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

actions!(
    debugger,
    [
        /// Run program execution to the current cursor position
        RunToCursor,
        /// Evaluate the selected text in the debugger context
        EvaluateSelectedText
    ]
);

actions!(
    go_to_line,
    [
        /// Toggle the go to line dialog
        #[action(name = "Toggle")]
        ToggleGoToLine
    ]
);

actions!(
    editor,
    [
        /// Accept the full edit prediction
        AcceptEditPrediction,
        /// Accept a partial Copilot suggestion
        AcceptPartialCopilotSuggestion,
        /// Accept a partial edit prediction
        AcceptPartialEditPrediction,
        /// Add a cursor above the current selection
        AddSelectionAbove,
        /// Add a cursor below the current selection
        AddSelectionBelow,
        /// Apply all diff hunks in the editor
        ApplyAllDiffHunks,
        /// Apply the diff hunk at the current position
        ApplyDiffHunk,
        /// Delete the character before the cursor
        Backspace,
        /// Cancel the current operation
        Cancel,
        /// Cancel the running flycheck operation
        CancelFlycheck,
        /// Cancel pending language server work
        CancelLanguageServerWork,
        /// Clear flycheck results
        ClearFlycheck,
        /// Confirm the rename operation
        ConfirmRename,
        /// Confirm completion by inserting at cursor
        ConfirmCompletionInsert,
        /// Confirm completion by replacing existing text
        ConfirmCompletionReplace,
        /// Navigate to the first item in the context menu
        ContextMenuFirst,
        /// Navigate to the last item in the context menu
        ContextMenuLast,
        /// Navigate to the next item in the context menu
        ContextMenuNext,
        /// Navigate to the previous item in the context menu
        ContextMenuPrevious,
        /// Convert indentation from tabs to spaces
        ConvertIndentationToSpaces,
        /// Convert indentation from spaces to tabs
        ConvertIndentationToTabs,
        /// Convert selected text to kebab-case
        ConvertToKebabCase,
        /// Convert selected text to lowerCamelCase
        ConvertToLowerCamelCase,
        /// Convert selected text to lowercase
        ConvertToLowerCase,
        /// Toggle the case of selected text
        ConvertToOppositeCase,
        /// Convert selected text to snake_case
        ConvertToSnakeCase,
        /// Convert selected text to Title Case
        ConvertToTitleCase,
        /// Convert selected text to UpperCamelCase
        ConvertToUpperCamelCase,
        /// Convert selected text to UPPERCASE
        ConvertToUpperCase,
        /// Apply ROT13 cipher to selected text
        ConvertToRot13,
        /// Apply ROT47 cipher to selected text
        ConvertToRot47,
        /// Copy selected text to the clipboard
        Copy,
        /// Copy selected text to the clipboard with leading/trailing whitespace trimmed
        CopyAndTrim,
        /// Copy the current file location to the clipboard
        CopyFileLocation,
        /// Copy the highlighted text as JSON
        CopyHighlightJson,
        /// Copy the current file name to the clipboard
        CopyFileName,
        /// Copy the file name without extension to the clipboard
        CopyFileNameWithoutExtension,
        /// Copy a permalink to the current line
        CopyPermalinkToLine,
        /// Cut selected text to the clipboard
        Cut,
        /// Cut from cursor to end of line
        CutToEndOfLine,
        /// Delete the character after the cursor
        Delete,
        /// Delete the current line
        DeleteLine,
        /// Delete from cursor to end of line
        DeleteToEndOfLine,
        /// Delete to the end of the next subword
        DeleteToNextSubwordEnd,
        /// Delete to the start of the previous subword
        DeleteToPreviousSubwordStart,
        /// Display names of all active cursors
        DisplayCursorNames,
        /// Duplicate the current line below
        DuplicateLineDown,
        /// Duplicate the current line above
        DuplicateLineUp,
        /// Duplicate the current selection
        DuplicateSelection,
        /// Expand all diff hunks in the editor
        #[action(deprecated_aliases = ["editor::ExpandAllHunkDiffs"])]
        ExpandAllDiffHunks,
        /// Expand macros recursively at cursor position
        ExpandMacroRecursively,
        /// Find all references to the symbol at cursor
        FindAllReferences,
        /// Find the next match in the search
        FindNextMatch,
        /// Find the previous match in the search
        FindPreviousMatch,
        /// Fold the current code block
        Fold,
        /// Fold all foldable regions in the editor
        FoldAll,
        /// Fold all function bodies in the editor
        FoldFunctionBodies,
        /// Fold the current code block and all its children
        FoldRecursive,
        /// Fold the selected ranges
        FoldSelectedRanges,
        /// Toggle folding at the current position
        ToggleFold,
        /// Toggle recursive folding at the current position
        ToggleFoldRecursive,
        /// Format the entire document
        Format,
        /// Format only the selected text
        FormatSelections,
        /// Go to the declaration of the symbol at cursor
        GoToDeclaration,
        /// Go to declaration in a split pane
        GoToDeclarationSplit,
        /// Go to the definition of the symbol at cursor
        GoToDefinition,
        /// Go to definition in a split pane
        GoToDefinitionSplit,
        /// Go to the next diagnostic in the file
        GoToDiagnostic,
        /// Go to the next diff hunk
        GoToHunk,
        /// Go to the previous diff hunk
        GoToPreviousHunk,
        /// Go to the implementation of the symbol at cursor
        GoToImplementation,
        /// Go to implementation in a split pane
        GoToImplementationSplit,
        /// Go to the next change in the file
        GoToNextChange,
        /// Go to the parent module of the current file
        GoToParentModule,
        /// Go to the previous change in the file
        GoToPreviousChange,
        /// Go to the previous diagnostic in the file
        GoToPreviousDiagnostic,
        /// Go to the type definition of the symbol at cursor
        GoToTypeDefinition,
        /// Go to type definition in a split pane
        GoToTypeDefinitionSplit,
        /// Scroll down by half a page
        HalfPageDown,
        /// Scroll up by half a page
        HalfPageUp,
        /// Show hover information for the symbol at cursor
        Hover,
        /// Increase indentation of selected lines
        Indent,
        /// Insert a UUID v4 at cursor position
        InsertUuidV4,
        /// Insert a UUID v7 at cursor position
        InsertUuidV7,
        /// Join the current line with the next line
        JoinLines,
        /// Cut to kill ring (Emacs-style)
        KillRingCut,
        /// Yank from kill ring (Emacs-style)
        KillRingYank,
        /// Move cursor down one line
        LineDown,
        /// Move cursor up one line
        LineUp,
        /// Move cursor down
        MoveDown,
        /// Move cursor left
        MoveLeft,
        /// Move the current line down
        MoveLineDown,
        /// Move the current line up
        MoveLineUp,
        /// Move cursor right
        MoveRight,
        /// Move cursor to the beginning of the document
        MoveToBeginning,
        /// Move cursor to the enclosing bracket
        MoveToEnclosingBracket,
        /// Move cursor to the end of the document
        MoveToEnd,
        /// Move cursor to the end of the paragraph
        MoveToEndOfParagraph,
        /// Move cursor to the end of the next subword
        MoveToNextSubwordEnd,
        /// Move cursor to the end of the next word
        MoveToNextWordEnd,
        /// Move cursor to the start of the previous subword
        MoveToPreviousSubwordStart,
        /// Move cursor to the start of the previous word
        MoveToPreviousWordStart,
        /// Move cursor to the start of the paragraph
        MoveToStartOfParagraph,
        /// Move cursor to the start of the current excerpt
        MoveToStartOfExcerpt,
        /// Move cursor to the start of the next excerpt
        MoveToStartOfNextExcerpt,
        /// Move cursor to the end of the current excerpt
        MoveToEndOfExcerpt,
        /// Move cursor to the end of the previous excerpt
        MoveToEndOfPreviousExcerpt,
        /// Move cursor up
        MoveUp,
        /// Insert a new line and move cursor to it
        Newline,
        /// Insert a new line above the current line
        NewlineAbove,
        /// Insert a new line below the current line
        NewlineBelow,
        /// Navigate to the next edit prediction
        NextEditPrediction,
        /// Scroll to the next screen
        NextScreen,
        /// Open the context menu at cursor position
        OpenContextMenu,
        /// Open excerpts from the current file
        OpenExcerpts,
        /// Open excerpts in a split pane
        OpenExcerptsSplit,
        /// Open the proposed changes editor
        OpenProposedChangesEditor,
        /// Open documentation for the symbol at cursor
        OpenDocs,
        /// Open a permalink to the current line
        OpenPermalinkToLine,
        /// Open the file whose name is selected in the editor
        #[action(deprecated_aliases = ["editor::OpenFile"])]
        OpenSelectedFilename,
        /// Open all selections in a multibuffer
        OpenSelectionsInMultibuffer,
        /// Open the URL at cursor position
        OpenUrl,
        /// Organize import statements
        OrganizeImports,
        /// Decrease indentation of selected lines
        Outdent,
        /// Automatically adjust indentation based on context
        AutoIndent,
        /// Scroll down by one page
        PageDown,
        /// Scroll up by one page
        PageUp,
        /// Paste from clipboard
        Paste,
        /// Navigate to the previous edit prediction
        PreviousEditPrediction,
        /// Redo the last undone edit
        Redo,
        /// Redo the last selection change
        RedoSelection,
        /// Rename the symbol at cursor
        Rename,
        /// Restart the language server for the current file
        RestartLanguageServer,
        /// Reveal the current file in the system file manager
        RevealInFileManager,
        /// Reverse the order of selected lines
        ReverseLines,
        /// Reload the file from disk
        ReloadFile,
        /// Rewrap text to fit within the preferred line length
        Rewrap,
        /// Run flycheck diagnostics
        RunFlycheck,
        /// Scroll the cursor to the bottom of the viewport
        ScrollCursorBottom,
        /// Scroll the cursor to the center of the viewport
        ScrollCursorCenter,
        /// Cycle cursor position between center, top, and bottom
        ScrollCursorCenterTopBottom,
        /// Scroll the cursor to the top of the viewport
        ScrollCursorTop,
        /// Select all text in the editor
        SelectAll,
        /// Select all matches of the current selection
        SelectAllMatches,
        /// Select to the start of the current excerpt
        SelectToStartOfExcerpt,
        /// Select to the start of the next excerpt
        SelectToStartOfNextExcerpt,
        /// Select to the end of the current excerpt
        SelectToEndOfExcerpt,
        /// Select to the end of the previous excerpt
        SelectToEndOfPreviousExcerpt,
        /// Extend selection down
        SelectDown,
        /// Select the enclosing symbol
        SelectEnclosingSymbol,
        /// Select the next larger syntax node
        SelectLargerSyntaxNode,
        /// Extend selection left
        SelectLeft,
        /// Select the current line
        SelectLine,
        /// Extend selection down by one page
        SelectPageDown,
        /// Extend selection up by one page
        SelectPageUp,
        /// Extend selection right
        SelectRight,
        /// Select the next smaller syntax node
        SelectSmallerSyntaxNode,
        /// Select to the beginning of the document
        SelectToBeginning,
        /// Select to the end of the document
        SelectToEnd,
        /// Select to the end of the paragraph
        SelectToEndOfParagraph,
        /// Select to the end of the next subword
        SelectToNextSubwordEnd,
        /// Select to the end of the next word
        SelectToNextWordEnd,
        /// Select to the start of the previous subword
        SelectToPreviousSubwordStart,
        /// Select to the start of the previous word
        SelectToPreviousWordStart,
        /// Select to the start of the paragraph
        SelectToStartOfParagraph,
        /// Extend selection up
        SelectUp,
        /// Show the system character palette
        ShowCharacterPalette,
        /// Show edit prediction at cursor
        ShowEditPrediction,
        /// Show signature help for the current function
        ShowSignatureHelp,
        /// Show word completions
        ShowWordCompletions,
        /// Randomly shuffle selected lines
        ShuffleLines,
        SignatureHelpNext,
        SignatureHelpPrevious,
        /// Sort selected lines case-insensitively
        SortLinesCaseInsensitive,
        /// Sort selected lines case-sensitively
        SortLinesCaseSensitive,
        /// Split selection into individual lines
        SplitSelectionIntoLines,
        /// Stop the language server for the current file
        StopLanguageServer,
        /// Switch between source and header files
        SwitchSourceHeader,
        /// Insert a tab character or indent
        Tab,
        /// Remove a tab character or outdent
        Backtab,
        /// Toggle a breakpoint at the current line
        ToggleBreakpoint,
        /// Toggle the case of selected text
        ToggleCase,
        /// Disable the breakpoint at the current line
        DisableBreakpoint,
        /// Enable the breakpoint at the current line
        EnableBreakpoint,
        /// Edit the log message for a breakpoint
        EditLogBreakpoint,
        /// Toggle automatic signature help
        ToggleAutoSignatureHelp,
        /// Toggle inline git blame display
        ToggleGitBlameInline,
        /// Open the git commit for the blame at cursor
        OpenGitBlameCommit,
        /// Toggle the diagnostics panel
        ToggleDiagnostics,
        /// Toggle indent guides display
        ToggleIndentGuides,
        /// Toggle inlay hints display
        ToggleInlayHints,
        /// Toggle inline values display
        ToggleInlineValues,
        /// Toggle inline diagnostics display
        ToggleInlineDiagnostics,
        /// Toggle edit prediction feature
        ToggleEditPrediction,
        /// Toggle line numbers display
        ToggleLineNumbers,
        /// Toggle the minimap display
        ToggleMinimap,
        /// Swap the start and end of the current selection
        SwapSelectionEnds,
        /// Set a mark at the current position
        SetMark,
        /// Toggle relative line numbers display
        ToggleRelativeLineNumbers,
        /// Toggle diff display for selected hunks
        #[action(deprecated_aliases = ["editor::ToggleHunkDiff"])]
        ToggleSelectedDiffHunks,
        /// Toggle the selection menu
        ToggleSelectionMenu,
        /// Toggle soft wrap mode
        ToggleSoftWrap,
        /// Toggle the tab bar display
        ToggleTabBar,
        /// Transpose characters around cursor
        Transpose,
        /// Undo the last edit
        Undo,
        /// Undo the last selection change
        UndoSelection,
        /// Unfold all folded regions
        UnfoldAll,
        /// Unfold lines at cursor
        UnfoldLines,
        /// Unfold recursively at cursor
        UnfoldRecursive,
        /// Remove duplicate lines (case-insensitive)
        UniqueLinesCaseInsensitive,
        /// Remove duplicate lines (case-sensitive)
        UniqueLinesCaseSensitive,
    ]
);
