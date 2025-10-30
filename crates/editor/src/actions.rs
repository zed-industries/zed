//! This module contains all actions supported by [`Editor`].
use super::*;
use gpui::{Action, actions};
use project::project_settings::GoToDiagnosticSeverityFilter;
use schemars::JsonSchema;
use util::serde::default_true;

/// Selects the next occurrence of the current selection.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectNext {
    #[serde(default)]
    pub replace_newest: bool,
}

/// Selects the previous occurrence of the current selection.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectPrevious {
    #[serde(default)]
    pub replace_newest: bool,
}

/// Moves the cursor to the beginning of the current line.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MoveToBeginningOfLine {
    #[serde(default = "default_true")]
    pub stop_at_soft_wraps: bool,
    #[serde(default)]
    pub stop_at_indent: bool,
}

/// Selects from the cursor to the beginning of the current line.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectToBeginningOfLine {
    #[serde(default)]
    pub(super) stop_at_soft_wraps: bool,
    #[serde(default)]
    pub stop_at_indent: bool,
}

/// Deletes from the cursor to the beginning of the current line.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct DeleteToBeginningOfLine {
    #[serde(default)]
    pub(super) stop_at_indent: bool,
}

/// Moves the cursor up by one page.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MovePageUp {
    #[serde(default)]
    pub(super) center_cursor: bool,
}

/// Moves the cursor down by one page.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MovePageDown {
    #[serde(default)]
    pub(super) center_cursor: bool,
}

/// Moves the cursor to the end of the current line.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MoveToEndOfLine {
    #[serde(default = "default_true")]
    pub stop_at_soft_wraps: bool,
}

/// Selects from the cursor to the end of the current line.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectToEndOfLine {
    #[serde(default)]
    pub(super) stop_at_soft_wraps: bool,
}

/// Toggles the display of available code actions at the cursor position.
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

/// Confirms and accepts the currently selected completion suggestion.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ConfirmCompletion {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

/// Composes multiple completion suggestions into a single completion.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ComposeCompletion {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

/// Confirms and applies the currently selected code action.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ConfirmCodeAction {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

/// Toggles comment markers for the selected lines.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ToggleComments {
    #[serde(default)]
    pub advance_downwards: bool,
    #[serde(default)]
    pub ignore_indent: bool,
}

/// Moves the cursor up by a specified number of lines.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MoveUpByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

/// Moves the cursor down by a specified number of lines.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MoveDownByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

/// Extends selection up by a specified number of lines.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectUpByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

/// Extends selection down by a specified number of lines.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectDownByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

/// Expands all excerpts in the editor.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ExpandExcerpts {
    #[serde(default)]
    pub(super) lines: u32,
}

/// Expands excerpts above the current position.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ExpandExcerptsUp {
    #[serde(default)]
    pub(super) lines: u32,
}

/// Expands excerpts below the current position.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ExpandExcerptsDown {
    #[serde(default)]
    pub(super) lines: u32,
}

/// Shows code completion suggestions at the cursor position.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ShowCompletions {
    #[serde(default)]
    pub(super) trigger: Option<String>,
}

/// Handles text input in the editor.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
pub struct HandleInput(pub String);

/// Deletes from the cursor to the end of the next word.
/// Stops before the end of the next word, if whitespace sequences of length >= 2 are encountered.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct DeleteToNextWordEnd {
    #[serde(default)]
    pub ignore_newlines: bool,
    // Whether to stop before the end of the next word, if language-defined bracket is encountered.
    #[serde(default)]
    pub ignore_brackets: bool,
}

/// Deletes from the cursor to the start of the previous word.
/// Stops before the start of the previous word, if whitespace sequences of length >= 2 are encountered.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct DeleteToPreviousWordStart {
    #[serde(default)]
    pub ignore_newlines: bool,
    // Whether to stop before the start of the previous word, if language-defined bracket is encountered.
    #[serde(default)]
    pub ignore_brackets: bool,
}

/// Cuts from cursor to end of line.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct CutToEndOfLine {
    #[serde(default)]
    pub stop_at_newlines: bool,
}

/// Folds all code blocks at the specified indentation level.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
pub struct FoldAtLevel(pub u32);

/// Spawns the nearest available task from the current cursor position.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SpawnNearestTask {
    #[serde(default)]
    pub reveal: task::RevealStrategy,
}

#[derive(Clone, PartialEq, Action)]
#[action(no_json, no_register)]
pub struct DiffClipboardWithSelectionData {
    pub clipboard_text: String,
    pub editor: Entity<Editor>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Default)]
pub enum UuidVersion {
    #[default]
    V4,
    V7,
}

/// Splits selection into individual lines.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SplitSelectionIntoLines {
    /// Keep the text selected after splitting instead of collapsing to cursors.
    #[serde(default)]
    pub keep_selections: bool,
}

/// Goes to the next diagnostic in the file.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct GoToDiagnostic {
    #[serde(default)]
    pub severity: GoToDiagnosticSeverityFilter,
}

/// Goes to the previous diagnostic in the file.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct GoToPreviousDiagnostic {
    #[serde(default)]
    pub severity: GoToDiagnosticSeverityFilter,
}

/// Adds a cursor above the current selection.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct AddSelectionAbove {
    #[serde(default = "default_true")]
    pub skip_soft_wrap: bool,
}

/// Adds a cursor below the current selection.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct AddSelectionBelow {
    #[serde(default = "default_true")]
    pub skip_soft_wrap: bool,
}

actions!(
    debugger,
    [
        /// Runs program execution to the current cursor position.
        RunToCursor,
        /// Evaluates the selected text in the debugger context.
        EvaluateSelectedText
    ]
);

actions!(
    go_to_line,
    [
        /// Toggles the go to line dialog.
        #[action(name = "Toggle")]
        ToggleGoToLine
    ]
);

actions!(
    editor,
    [
        /// Accepts the full edit prediction.
        AcceptEditPrediction,
        /// Accepts a partial edit prediction.
        #[action(deprecated_aliases = ["editor::AcceptPartialCopilotSuggestion"])]
        AcceptPartialEditPrediction,
        /// Applies all diff hunks in the editor.
        ApplyAllDiffHunks,
        /// Applies the diff hunk at the current position.
        ApplyDiffHunk,
        /// Deletes the character before the cursor.
        Backspace,
        /// Shows git blame information for the current line.
        BlameHover,
        /// Cancels the current operation.
        Cancel,
        /// Cancels the running flycheck operation.
        CancelFlycheck,
        /// Cancels pending language server work.
        CancelLanguageServerWork,
        /// Clears flycheck results.
        ClearFlycheck,
        /// Confirms the rename operation.
        ConfirmRename,
        /// Confirms completion by inserting at cursor.
        ConfirmCompletionInsert,
        /// Confirms completion by replacing existing text.
        ConfirmCompletionReplace,
        /// Navigates to the first item in the context menu.
        ContextMenuFirst,
        /// Navigates to the last item in the context menu.
        ContextMenuLast,
        /// Navigates to the next item in the context menu.
        ContextMenuNext,
        /// Navigates to the previous item in the context menu.
        ContextMenuPrevious,
        /// Converts indentation from tabs to spaces.
        ConvertIndentationToSpaces,
        /// Converts indentation from spaces to tabs.
        ConvertIndentationToTabs,
        /// Converts selected text to kebab-case.
        ConvertToKebabCase,
        /// Converts selected text to lowerCamelCase.
        ConvertToLowerCamelCase,
        /// Converts selected text to lowercase.
        ConvertToLowerCase,
        /// Toggles the case of selected text.
        ConvertToOppositeCase,
        /// Converts selected text to sentence case.
        ConvertToSentenceCase,
        /// Converts selected text to snake_case.
        ConvertToSnakeCase,
        /// Converts selected text to Title Case.
        ConvertToTitleCase,
        /// Converts selected text to UpperCamelCase.
        ConvertToUpperCamelCase,
        /// Converts selected text to UPPERCASE.
        ConvertToUpperCase,
        /// Applies ROT13 cipher to selected text.
        ConvertToRot13,
        /// Applies ROT47 cipher to selected text.
        ConvertToRot47,
        /// Copies selected text to the clipboard.
        Copy,
        /// Copies selected text to the clipboard with leading/trailing whitespace trimmed.
        CopyAndTrim,
        /// Copies the current file location to the clipboard.
        CopyFileLocation,
        /// Copies the highlighted text as JSON.
        CopyHighlightJson,
        /// Copies the current file name to the clipboard.
        CopyFileName,
        /// Copies the file name without extension to the clipboard.
        CopyFileNameWithoutExtension,
        /// Copies a permalink to the current line.
        CopyPermalinkToLine,
        /// Cuts selected text to the clipboard.
        Cut,
        /// Deletes the character after the cursor.
        Delete,
        /// Deletes the current line.
        DeleteLine,
        /// Deletes from cursor to end of line.
        DeleteToEndOfLine,
        /// Deletes to the end of the next subword.
        DeleteToNextSubwordEnd,
        /// Deletes to the start of the previous subword.
        DeleteToPreviousSubwordStart,
        /// Diffs the text stored in the clipboard against the current selection.
        DiffClipboardWithSelection,
        /// Displays names of all active cursors.
        DisplayCursorNames,
        /// Duplicates the current line below.
        DuplicateLineDown,
        /// Duplicates the current line above.
        DuplicateLineUp,
        /// Duplicates the current selection.
        DuplicateSelection,
        /// Expands all diff hunks in the editor.
        #[action(deprecated_aliases = ["editor::ExpandAllHunkDiffs"])]
        ExpandAllDiffHunks,
        /// Collapses all diff hunks in the editor.
        CollapseAllDiffHunks,
        /// Expands macros recursively at cursor position.
        ExpandMacroRecursively,
        /// Finds all references to the symbol at cursor.
        FindAllReferences,
        /// Finds the next match in the search.
        FindNextMatch,
        /// Finds the previous match in the search.
        FindPreviousMatch,
        /// Folds the current code block.
        Fold,
        /// Folds all foldable regions in the editor.
        FoldAll,
        /// Folds all code blocks at indentation level 1.
        #[action(name = "FoldAtLevel_1")]
        FoldAtLevel1,
        /// Folds all code blocks at indentation level 2.
        #[action(name = "FoldAtLevel_2")]
        FoldAtLevel2,
        /// Folds all code blocks at indentation level 3.
        #[action(name = "FoldAtLevel_3")]
        FoldAtLevel3,
        /// Folds all code blocks at indentation level 4.
        #[action(name = "FoldAtLevel_4")]
        FoldAtLevel4,
        /// Folds all code blocks at indentation level 5.
        #[action(name = "FoldAtLevel_5")]
        FoldAtLevel5,
        /// Folds all code blocks at indentation level 6.
        #[action(name = "FoldAtLevel_6")]
        FoldAtLevel6,
        /// Folds all code blocks at indentation level 7.
        #[action(name = "FoldAtLevel_7")]
        FoldAtLevel7,
        /// Folds all code blocks at indentation level 8.
        #[action(name = "FoldAtLevel_8")]
        FoldAtLevel8,
        /// Folds all code blocks at indentation level 9.
        #[action(name = "FoldAtLevel_9")]
        FoldAtLevel9,
        /// Folds all function bodies in the editor.
        FoldFunctionBodies,
        /// Folds the current code block and all its children.
        FoldRecursive,
        /// Folds the selected ranges.
        FoldSelectedRanges,
        /// Toggles focus back to the last active buffer.
        ToggleFocus,
        /// Toggles folding at the current position.
        ToggleFold,
        /// Toggles recursive folding at the current position.
        ToggleFoldRecursive,
        /// Toggles all folds in a buffer or all excerpts in multibuffer.
        ToggleFoldAll,
        /// Formats the entire document.
        Format,
        /// Formats only the selected text.
        FormatSelections,
        /// Goes to the declaration of the symbol at cursor.
        GoToDeclaration,
        /// Goes to declaration in a split pane.
        GoToDeclarationSplit,
        /// Goes to the definition of the symbol at cursor.
        GoToDefinition,
        /// Goes to definition in a split pane.
        GoToDefinitionSplit,
        /// Goes to the next diff hunk.
        GoToHunk,
        /// Goes to the previous diff hunk.
        GoToPreviousHunk,
        /// Goes to the implementation of the symbol at cursor.
        GoToImplementation,
        /// Goes to implementation in a split pane.
        GoToImplementationSplit,
        /// Goes to the next change in the file.
        GoToNextChange,
        /// Goes to the parent module of the current file.
        GoToParentModule,
        /// Goes to the previous change in the file.
        GoToPreviousChange,
        /// Goes to the next reference to the symbol under the cursor.
        GoToNextReference,
        /// Goes to the previous reference to the symbol under the cursor.
        GoToPreviousReference,
        /// Goes to the type definition of the symbol at cursor.
        GoToTypeDefinition,
        /// Goes to type definition in a split pane.
        GoToTypeDefinitionSplit,
        /// Goes to the next document highlight.
        GoToNextDocumentHighlight,
        /// Goes to the previous document highlight.
        GoToPreviousDocumentHighlight,
        /// Scrolls down by half a page.
        HalfPageDown,
        /// Scrolls up by half a page.
        HalfPageUp,
        /// Shows hover information for the symbol at cursor.
        Hover,
        /// Increases indentation of selected lines.
        Indent,
        /// Inserts a UUID v4 at cursor position.
        InsertUuidV4,
        /// Inserts a UUID v7 at cursor position.
        InsertUuidV7,
        /// Joins the current line with the next line.
        JoinLines,
        /// Cuts to kill ring (Emacs-style).
        KillRingCut,
        /// Yanks from kill ring (Emacs-style).
        KillRingYank,
        /// Moves cursor down one line.
        LineDown,
        /// Moves cursor up one line.
        LineUp,
        /// Moves cursor down.
        MoveDown,
        /// Moves cursor left.
        MoveLeft,
        /// Moves the current line down.
        MoveLineDown,
        /// Moves the current line up.
        MoveLineUp,
        /// Moves cursor right.
        MoveRight,
        /// Moves cursor to the beginning of the document.
        MoveToBeginning,
        /// Moves cursor to the enclosing bracket.
        MoveToEnclosingBracket,
        /// Moves cursor to the end of the document.
        MoveToEnd,
        /// Moves cursor to the end of the paragraph.
        MoveToEndOfParagraph,
        /// Moves cursor to the end of the next subword.
        MoveToNextSubwordEnd,
        /// Moves cursor to the end of the next word.
        MoveToNextWordEnd,
        /// Moves cursor to the start of the previous subword.
        MoveToPreviousSubwordStart,
        /// Moves cursor to the start of the previous word.
        MoveToPreviousWordStart,
        /// Moves cursor to the start of the paragraph.
        MoveToStartOfParagraph,
        /// Moves cursor to the start of the current excerpt.
        MoveToStartOfExcerpt,
        /// Moves cursor to the start of the next excerpt.
        MoveToStartOfNextExcerpt,
        /// Moves cursor to the end of the current excerpt.
        MoveToEndOfExcerpt,
        /// Moves cursor to the end of the previous excerpt.
        MoveToEndOfPreviousExcerpt,
        /// Moves cursor up.
        MoveUp,
        /// Inserts a new line and moves cursor to it.
        Newline,
        /// Inserts a new line above the current line.
        NewlineAbove,
        /// Inserts a new line below the current line.
        NewlineBelow,
        /// Navigates to the next edit prediction.
        NextEditPrediction,
        /// Scrolls to the next screen.
        NextScreen,
        /// Goes to the next snippet tabstop if one exists.
        NextSnippetTabstop,
        /// Opens the context menu at cursor position.
        OpenContextMenu,
        /// Opens excerpts from the current file.
        OpenExcerpts,
        /// Opens excerpts in a split pane.
        OpenExcerptsSplit,
        /// Opens the proposed changes editor.
        OpenProposedChangesEditor,
        /// Opens documentation for the symbol at cursor.
        OpenDocs,
        /// Opens a permalink to the current line.
        OpenPermalinkToLine,
        /// Opens the file whose name is selected in the editor.
        #[action(deprecated_aliases = ["editor::OpenFile"])]
        OpenSelectedFilename,
        /// Opens all selections in a multibuffer.
        OpenSelectionsInMultibuffer,
        /// Opens the URL at cursor position.
        OpenUrl,
        /// Organizes import statements.
        OrganizeImports,
        /// Decreases indentation of selected lines.
        Outdent,
        /// Automatically adjusts indentation based on context.
        AutoIndent,
        /// Scrolls down by one page.
        PageDown,
        /// Scrolls up by one page.
        PageUp,
        /// Pastes from clipboard.
        Paste,
        /// Navigates to the previous edit prediction.
        PreviousEditPrediction,
        /// Goes to the previous snippet tabstop if one exists.
        PreviousSnippetTabstop,
        /// Redoes the last undone edit.
        Redo,
        /// Redoes the last selection change.
        RedoSelection,
        /// Renames the symbol at cursor.
        Rename,
        /// Restarts the language server for the current file.
        RestartLanguageServer,
        /// Reveals the current file in the system file manager.
        RevealInFileManager,
        /// Reverses the order of selected lines.
        ReverseLines,
        /// Reloads the file from disk.
        ReloadFile,
        /// Rewraps text to fit within the preferred line length.
        Rewrap,
        /// Runs flycheck diagnostics.
        RunFlycheck,
        /// Scrolls the cursor to the bottom of the viewport.
        ScrollCursorBottom,
        /// Scrolls the cursor to the center of the viewport.
        ScrollCursorCenter,
        /// Cycles cursor position between center, top, and bottom.
        ScrollCursorCenterTopBottom,
        /// Scrolls the cursor to the top of the viewport.
        ScrollCursorTop,
        /// Selects all text in the editor.
        SelectAll,
        /// Selects all matches of the current selection.
        SelectAllMatches,
        /// Selects to the start of the current excerpt.
        SelectToStartOfExcerpt,
        /// Selects to the start of the next excerpt.
        SelectToStartOfNextExcerpt,
        /// Selects to the end of the current excerpt.
        SelectToEndOfExcerpt,
        /// Selects to the end of the previous excerpt.
        SelectToEndOfPreviousExcerpt,
        /// Extends selection down.
        SelectDown,
        /// Selects the enclosing symbol.
        SelectEnclosingSymbol,
        /// Selects the next larger syntax node.
        SelectLargerSyntaxNode,
        /// Selects the next syntax node sibling.
        SelectNextSyntaxNode,
        /// Selects the previous syntax node sibling.
        SelectPreviousSyntaxNode,
        /// Extends selection left.
        SelectLeft,
        /// Selects the current line.
        SelectLine,
        /// Extends selection down by one page.
        SelectPageDown,
        /// Extends selection up by one page.
        SelectPageUp,
        /// Extends selection right.
        SelectRight,
        /// Selects the next smaller syntax node.
        SelectSmallerSyntaxNode,
        /// Selects to the beginning of the document.
        SelectToBeginning,
        /// Selects to the end of the document.
        SelectToEnd,
        /// Selects to the end of the paragraph.
        SelectToEndOfParagraph,
        /// Selects to the end of the next subword.
        SelectToNextSubwordEnd,
        /// Selects to the end of the next word.
        SelectToNextWordEnd,
        /// Selects to the start of the previous subword.
        SelectToPreviousSubwordStart,
        /// Selects to the start of the previous word.
        SelectToPreviousWordStart,
        /// Selects to the start of the paragraph.
        SelectToStartOfParagraph,
        /// Extends selection up.
        SelectUp,
        /// Shows the system character palette.
        ShowCharacterPalette,
        /// Shows edit prediction at cursor.
        ShowEditPrediction,
        /// Shows signature help for the current function.
        ShowSignatureHelp,
        /// Shows word completions.
        ShowWordCompletions,
        /// Randomly shuffles selected lines.
        ShuffleLines,
        /// Navigates to the next signature in the signature help popup.
        SignatureHelpNext,
        /// Navigates to the previous signature in the signature help popup.
        SignatureHelpPrevious,
        /// Sorts selected lines by length.
        SortLinesByLength,
        /// Sorts selected lines case-insensitively.
        SortLinesCaseInsensitive,
        /// Sorts selected lines case-sensitively.
        SortLinesCaseSensitive,
        /// Stops the language server for the current file.
        StopLanguageServer,
        /// Switches between source and header files.
        SwitchSourceHeader,
        /// Inserts a tab character or indents.
        Tab,
        /// Removes a tab character or outdents.
        Backtab,
        /// Toggles a breakpoint at the current line.
        ToggleBreakpoint,
        /// Toggles the case of selected text.
        ToggleCase,
        /// Disables the breakpoint at the current line.
        DisableBreakpoint,
        /// Enables the breakpoint at the current line.
        EnableBreakpoint,
        /// Edits the log message for a breakpoint.
        EditLogBreakpoint,
        /// Toggles automatic signature help.
        ToggleAutoSignatureHelp,
        /// Toggles inline git blame display.
        ToggleGitBlameInline,
        /// Opens the git commit for the blame at cursor.
        OpenGitBlameCommit,
        /// Toggles the diagnostics panel.
        ToggleDiagnostics,
        /// Toggles indent guides display.
        ToggleIndentGuides,
        /// Toggles inlay hints display.
        ToggleInlayHints,
        /// Toggles inline values display.
        ToggleInlineValues,
        /// Toggles inline diagnostics display.
        ToggleInlineDiagnostics,
        /// Toggles edit prediction feature.
        ToggleEditPrediction,
        /// Toggles line numbers display.
        ToggleLineNumbers,
        /// Toggles the minimap display.
        ToggleMinimap,
        /// Swaps the start and end of the current selection.
        SwapSelectionEnds,
        /// Sets a mark at the current position.
        SetMark,
        /// Toggles relative line numbers display.
        ToggleRelativeLineNumbers,
        /// Toggles diff display for selected hunks.
        #[action(deprecated_aliases = ["editor::ToggleHunkDiff"])]
        ToggleSelectedDiffHunks,
        /// Toggles the selection menu.
        ToggleSelectionMenu,
        /// Toggles soft wrap mode.
        ToggleSoftWrap,
        /// Toggles the tab bar display.
        ToggleTabBar,
        /// Transposes characters around cursor.
        Transpose,
        /// Undoes the last edit.
        Undo,
        /// Undoes the last selection change.
        UndoSelection,
        /// Unfolds all folded regions.
        UnfoldAll,
        /// Unfolds lines at cursor.
        UnfoldLines,
        /// Unfolds recursively at cursor.
        UnfoldRecursive,
        /// Removes duplicate lines (case-insensitive).
        UniqueLinesCaseInsensitive,
        /// Removes duplicate lines (case-sensitive).
        UniqueLinesCaseSensitive,
        /// Removes the surrounding syntax node (for example brackets, or closures)
        /// from the current selections.
        UnwrapSyntaxNode,
        /// Wraps selections in tag specified by language.
        WrapSelectionsInTag
    ]
);
