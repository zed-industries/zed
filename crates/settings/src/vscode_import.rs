use anyhow::Result;
use collections::HashMap;
use fs::Fs;
use gpui::{Keystroke, PlatformKeyboardMapper};
use serde_json::{Map, Value};

use std::sync::Arc;

pub struct VsCodeSettings {
    content: Map<String, Value>,
}

impl VsCodeSettings {
    pub fn from_str(content: &str) -> Result<Self> {
        Ok(Self {
            content: serde_json_lenient::from_str(content)?,
        })
    }

    pub async fn load_user_settings(fs: Arc<dyn Fs>) -> Result<Self> {
        let content = fs.load(paths::vscode_settings_file()).await?;
        Ok(Self {
            content: serde_json_lenient::from_str(&content)?,
        })
    }

    pub fn read_value(&self, setting: &str) -> Option<&Value> {
        if let Some(value) = self.content.get(setting) {
            return Some(value);
        }
        // TODO: maybe check if it's in [platform] settings for current platform as a fallback
        // TODO: deal with language specific settings
        None
    }

    pub fn read_string(&self, setting: &str) -> Option<&str> {
        self.read_value(setting).and_then(|v| v.as_str())
    }

    pub fn read_bool(&self, setting: &str) -> Option<bool> {
        self.read_value(setting).and_then(|v| v.as_bool())
    }

    pub fn string_setting(&self, key: &str, setting: &mut Option<String>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_str) {
            *setting = Some(s.to_owned())
        }
    }

    pub fn bool_setting(&self, key: &str, setting: &mut Option<bool>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_bool) {
            *setting = Some(s)
        }
    }

    pub fn u32_setting(&self, key: &str, setting: &mut Option<u32>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_u64) {
            *setting = Some(s as u32)
        }
    }

    pub fn u64_setting(&self, key: &str, setting: &mut Option<u64>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_u64) {
            *setting = Some(s)
        }
    }

    pub fn usize_setting(&self, key: &str, setting: &mut Option<usize>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_u64) {
            *setting = Some(s.try_into().unwrap())
        }
    }

    pub fn f32_setting(&self, key: &str, setting: &mut Option<f32>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_f64) {
            *setting = Some(s as f32)
        }
    }

    pub fn enum_setting<T>(
        &self,
        key: &str,
        setting: &mut Option<T>,
        f: impl FnOnce(&str) -> Option<T>,
    ) {
        if let Some(s) = self.content.get(key).and_then(Value::as_str).and_then(f) {
            *setting = Some(s)
        }
    }
}

#[derive(Debug, Clone)]
pub struct VsCodeShortcuts {
    content: Vec<Map<String, Value>>,
}

impl VsCodeShortcuts {
    pub fn from_str(content: &str) -> Result<Self> {
        Ok(Self {
            content: serde_json_lenient::from_str(content)?,
        })
    }

    pub async fn load_user_shortcuts(fs: Arc<dyn Fs>) -> Result<Self> {
        let content = fs.load(paths::vscode_shortcuts_file()).await?;

        Ok(Self {
            content: serde_json_lenient::from_str(&content)?,
        })
    }

    pub fn parse_shortcuts(&self, keyboard_mapper: &dyn PlatformKeyboardMapper) -> String {
        let mut normal_bindings = Vec::new();
        let mut other_bindings = HashMap::default();
        let mut skipped = Vec::new();
        for content in self.content.iter() {
            let Some(shortcut) = content.get("key").and_then(|key| key.as_str()) else {
                continue;
            };
            let vscode_raw_input =
                serde_json::to_string_pretty(content).unwrap_or(shortcut.to_string());
            let Some(keystroke) = Keystroke::parse_keystroke_components(shortcut, '+')
                .ok()
                .map(|keystroke| keystroke.into_gpui_style(keyboard_mapper))
            else {
                skipped.push((vscode_raw_input, "Unable to parse keystroke".to_string()));
                continue;
            };
            let Some(command) = content.get("command").and_then(|command| command.as_str()) else {
                continue;
            };
            let when = content.get("when").and_then(|when| when.as_str());
            let args = content.get("args").and_then(|args| args.as_str());
            let Some((action, context)) =
                vscode_shortcut_command_to_zed_action(command, when, args)
            else {
                skipped.push((
                    vscode_raw_input,
                    format!("Unable to parse command: {}", command),
                ));
                continue;
            };
            if let Some(context) = context {
                other_bindings
                    .entry(context)
                    .or_insert_with(Vec::new)
                    .push(ZedBindingContent {
                        binding: keystroke.unparse(),
                        action,
                        comments: vscode_raw_input,
                    });
            } else {
                normal_bindings.push(ZedBindingContent {
                    binding: keystroke.unparse(),
                    action,
                    comments: vscode_raw_input,
                });
            }
        }
        serialize_all(normal_bindings, other_bindings, skipped)
    }
}

struct ZedBindingContent {
    binding: String,
    action: String,
    comments: String,
}

fn vscode_shortcut_command_to_zed_action(
    command: &str,
    when: Option<&str>,
    args: Option<&str>,
) -> Option<(String, Option<String>)> {
    match command {
        // crates/menu/src/menu.rs
        // Missing:
        // SecondaryConfirm, Restart, EndSlot
        "list.focusFirst" | "list.focusAnyFirst" => {
            Some(("menu::SelectFirst".into(), Some("menu".into())))
        }
        "list.focusLast" | "list.focusAnyLast" => {
            Some(("menu::SelectLast".into(), Some("menu".into())))
        }
        "list.focusUp" | "list.focusAnyUp" => {
            Some(("menu::SelectPrevious".into(), Some("menu".into())))
        }
        "list.focusDown" | "list.focusAnyDown" => {
            Some(("menu::SelectNext".into(), Some("menu".into())))
        }
        "list.select" => {
            Some(("menu::Confirm".into(), Some("menu".into())))
        }
        "list.clear" => {
            Some(("menu::Cancel".into(), Some("menu".into())))
        }
        // crates/picker/src/picker.rs
        // Missing:
        // ConfirmCompletion, ConfirmInput. What's the secondary setting?

        // crates/workspace/src/workspace.rs
        // Missing:
        // ActivateNextPane, ActivatePreviousPane, ActivateNextWindow, ActivatePreviousWindow, ClearAllNotifications, CloseAllDocks,
        // Feedback, FollowNextCollaborator, MoveFocusedPanelToNextPosition, NewCenterTerminal, NewFileSplitVertical, NewFileSplitHorizontal,
        // OpenInTerminal, OpenComponentPreview, ReloadActiveItem, ShutdownDebugAdapters, ToggleCenteredLayout, ToggleZoom, Unfollow, Welcome,
        // RestoreBanner, CloseInactiveTabsAndPanes, MoveItemToPane, MoveItemToPaneInDirection, OpenTerminal, Reload, SendKeystrokes,
        // SwapPaneLeft, SwapPaneRight, SwapPaneUp, SwapPaneDown
        "addRootFolder" => {
            Some(("workspace::AddFolderToProject".into(), None))
        }
        "workbench.action.closeWindow" => {
            Some(("workspace::CloseWindow".into(), None))
        }
        "workbench.action.files.newUntitledFile" => {
            Some(("workspace::NewFile".into(), None))
        }
        "workbench.view.search" => {
            Some(("workspace::NewSearch".into(), None))
        }
        "workbench.action.terminal.new" => {
            Some(("workspace::NewTerminal".into(), None))
        }
        "workbench.action.newWindow" => {
            Some(("workspace::NewWindow".into(), None))
        }
        "workbench.action.files.openFolder" => {
            Some(("workspace::Open".into(), None))
        }
        "workbench.action.files.openFile" => {
            Some(("workspace::OpenFiles".into(), None))
        }
        "workbench.action.files.saveAs" => {
            Some(("workspace::SaveAs".into(), None))
        }
        "workbench.action.files.saveWithoutFormatting" => {
            Some(("workspace::SaveWithoutFormat".into(), None))
        }
        "workbench.action.togglePanel" => {
            Some(("workspace::ToggleBottomDock".into(), Some("Workspace".into())))
        }
        "workbench.action.toggleSidebarVisibility" => {
            Some(("workspace::ToggleLeftDock".into(), Some("Workspace".into())))
        }
        "workbench.action.toggleAuxiliaryBar" => {
            Some(("workspace::ToggleRightDock".into(), Some("Workspace".into())))
        }
        "workbench.action.focusFirstEditorGroup" => {
            Some((r#"["workspace::ActivatePane", 0]"#.into(), Some("Workspace".into())))
        }
        "workbench.action.focusSecondEditorGroup" => {
            Some((r#"["workspace::ActivatePane", 1]"#.into(), Some("Workspace".into())))
        }
        "workbench.action.focusThirdEditorGroup" => {
            Some((r#"["workspace::ActivatePane", 2]"#.into(), Some("Workspace".into())))
        }
        "workbench.action.focusFourthEditorGroup" => {
            Some((r#"["workspace::ActivatePane", 3]"#.into(), Some("Workspace".into())))
        }
        "workbench.action.focusFifthEditorGroup" => {
            Some((r#"["workspace::ActivatePane", 4]"#.into(), Some("Workspace".into())))
        }
        "workbench.action.focusSixthEditorGroup" => {
            Some((r#"["workspace::ActivatePane", 5]"#.into(), Some("Workspace".into())))
        }
        "workbench.action.focusSeventhEditorGroup" => {
            Some((r#"["workspace::ActivatePane", 6]"#.into(), Some("Workspace".into())))
        }
        "workbench.action.focusEighthEditorGroup" => {
            Some((r#"["workspace::ActivatePane", 7]"#.into(), Some("Workspace".into())))
        }
        "workbench.action.closeAllEditors" => {
            Some(("workspace::CloseAllItemsAndPanes".into(), Some("Pane".into())))
        }
        "workbench.action.files.save" => {
            Some(("workspace::Save".into(), Some("Workspace".into())))
        }
        "saveAll" => {
            Some(("workspace::SaveAll".into(), Some("Workspace".into())))
        }
        "workbench.action.focusLeftGroup" => {
            Some(("workspace::ActivatePaneLeft".into(), Some("Workspace".into())))
        }
        "workbench.action.focusRightGroup" => {
            Some(("workspace::ActivatePaneRight".into(), Some("Workspace".into())))
        }
        "workbench.action.focusAboveGroup" => {
            Some(("workspace::ActivatePaneUp".into(), Some("Workspace".into())))
        }
        "workbench.action.focusBelowGroup" => {
            Some(("workspace::ActivatePaneDown".into(), Some("Workspace".into())))
        }
        "workbench.action.showAllSymbols" => {
            Some(("project_symbols::Toggle".into(), Some("Workspace".into())))
        }
        "workbench.action.quickOpen" => {
            Some(("file_finder::Toggle".into(), Some("Workspace".into())))
        }
        // crates/zed_actions/src/lib.rs
        // Missing:
        // OpenBrowser, OpenZedUrl, OpenAccountSettings, OpenServerSettings, About, OpenLicenses, OpenTelemetryLog, DecreaseUiFontSize,
        // IncreaseUiFontSize, ResetUiFontSize, workspace::CopyPath, workspace::CopyRelativePath, workspace::CopyFileName, git::*,
        // feadback::*, icon_theme_selector::Toggle, agent::OpenConfiguration, assistant::*, assistant::InlineAssist, projects::OpenRemote,
        // task::*, outline::*, zed_predict_onboarding::*, git_onboarding::*,
        "workbench.action.openSettings" => {
            Some(("zed::OpenSettings".into(), None))
        }
        "workbench.action.openDefaultKeybindingsFile" => {
            Some(("zed::OpenDefaultKeymap".into(), None))
        }
        "workbench.action.quit" => {
            Some(("zed::Quit".into(), None))
        }
        "workbench.action.openGlobalKeybindings" => {
            Some(("zed::OpenKeymap".into(), Some("Workspace".into())))
        }
        "workbench.view.extensions" => {
            Some(("zed::OpenExtensions".into(), Some("Workspace".into())))
        }
        "workbench.action.zoomOut" => {
            Some((r#"["zed::DecreaseBufferFontSize", { "persist": false }]"#.into(), None))
        }
        "workbench.action.zoomIn" => {
            Some((r#"["zed::IncreaseBufferFontSize", { "persist": false }]"#.into(), None))
        }
        "workbench.action.zoomReset" => {
            Some((r#"["zed::ResetBufferFontSize", { "persist": false }]"#.into(), None))
        }
        "workbench.action.showCommands" => {
            Some(("command_palette::Toggle".into(), Some("Workspace".into())))
        }
        "workbench.action.selectTheme" => {
            Some(("theme_selector::Toggle".into(), Some("Workspace".into())))
        }
        "workbench.action.openRecent" => {
            Some(("projects::OpenRecent".into(), Some("Workspace".into())))
        }
        // crates/debugger_ui/src/debugger_ui.rs
        // Missing:
        // ToggleIgnoreBreakpoints, ClearAllBreakpoints, CreateDebuggingSession, FocusConsole, FocusVariables, FocusBreakpointList,
        // FocusFrames, FocusModules, FocusLoadedSources, FocusTerminal,
        "workbench.action.debug.start" => {
            Some(("debugger::Start".into(), None))
        }
        "workbench.action.debug.continue" => {
            Some(("debugger::Continue".into(), None))
        }
        "workbench.action.debug.disconnect" => {
            Some(("debugger::Disconnect".into(), None))
        }
        "workbench.action.debug.pause" => {
            Some(("debugger::Pause".into(), None))
        }
        "workbench.action.debug.restart" => {
            Some(("debugger::Restart".into(), None))
        }
        "workbench.action.debug.stepInto" => {
            Some(("debugger::StepInto".into(), None))
        }
        "workbench.action.debug.stepOver" => {
            Some(("debugger::StepOver".into(), None))
        }
        "workbench.action.debug.stepOut" => {
            Some(("debugger::StepOut".into(), None))
        }
        "workbench.action.debug.stepBack" => {
            Some(("debugger::StepBack".into(), None))
        }
        "workbench.action.debug.stop" => {
            Some(("debugger::Stop".into(), None))
        }
        // crates/zed/src/zed.rs
        // Missing:
        // DebugElements, Hide, HideOthers, Minimize, OpenDefaultSettings, OpenProjectSettings, OpenProjectTasks, OpenProjectDebugTasks,
        // OpenTasks, OpenDebugTasks, ResetDatabase, ShowAll, Zoom, TestPanic,
        "workbench.action.toggleFullScreen" => {
            Some(("zed::ToggleFullScreen".into(), None))
        }
        // crates/zeta/src/init.rs
        // Missing: All

        // crates/editor/src/actions.rs
        // Missing:
        // ComposeCompletion, ConfirmCompletion, DeleteToBeginningOfLine, ExpandExcerpts, ExpandExcerptsDown, ExpandExcerptsUp, HandleInput,
        // MoveUpByLines, SelectDownByLines, SelectUpByLines, SpawnNearestTask, AcceptEditPrediction, AcceptPartialCopilotSuggestion,
        // AcceptPartialEditPrediction, ApplyAllDiffHunks, ApplyDiffHunk, Cancel, CancelLanguageServerWork, ConfirmCompletionInsert,
        // ConfirmCompletionReplace, ContextMenuFirst, ContextMenuLast, ContextMenuNext, ContextMenuPrevious, ConvertToKebabCase,
        // ConvertToLowerCamelCase, ConvertToLowerCase, ConvertToOppositeCase, ConvertToSnakeCase, ConvertToTitleCase, ConvertToUpperCamelCase,
        // ConvertToUpperCase, ConvertToRot13, ConvertToRot47, CopyAndTrim, CopyFileLocation, CopyHighlightJson, CopyFileName,
        // CopyFileNameWithoutExtension, CopyPermalinkToLine, CutToEndOfLine, DeleteToEndOfLine, DeleteToNextSubwordEnd,
        // DeleteToPreviousSubwordStart, DisplayCursorNames, DuplicateSelection, ExpandMacroRecursively, FindNextMatch, FindPreviousMatch,
        // FoldFunctionBodies, FoldSelectedRanges, ToggleFoldRecursive, FormatSelections, GoToDeclarationSplit, GoToDiagnostic, GoToHunk,
        // GoToPreviousHunk, GoToImplementationSplit, GoToNextChange, GoToPreviousChange, GoToPreviousDiagnostic, GoToTypeDefinitionSplit,
        // HalfPageDown, HalfPageUp, InsertUuidV4, InsertUuidV7, KillRingCut, KillRingYank, MoveToEndOfParagraph, MoveToStartOfParagraph,
        // MoveToStartOfExcerpt, MoveToStartOfNextExcerpt, MoveToEndOfExcerpt, MoveToEndOfPreviousExcerpt, Newline, NextEditPrediction,
        // NextScreen, OpenContextMenu, OpenExcerpts, OpenExcerptsSplit, OpenProposedChangesEditor, OpenDocs, OpenPermalinkToLine,
        // OpenSelectionsInMultibuffer, OpenUrl, AutoIndent, PreviousEditPrediction, RedoSelection, RestartLanguageServer, ReverseLines,
        // GoToTypeDefinition, RevertFile, ReloadFile, Rewrap, ScrollCursorBottom, ScrollCursorCenter, ScrollCursorCenterTopBottom,
        // ScrollCursorTop, SelectAllMatches, SelectToStartOfExcerpt, SelectToStartOfNextExcerpt, SelectToEndOfExcerpt,
        // SelectToEndOfPreviousExcerpt, SelectEnclosingSymbol, SelectToEndOfParagraph, SelectToStartOfParagraph, ShowCharacterPalette,
        // ShowEditPrediction, ShowSignatureHelp, ShowWordCompletions, ShuffleLines, SortLinesCaseInsensitive, SortLinesCaseSensitive,
        // SplitSelectionIntoLines, StopLanguageServer, SwitchSourceHeader, ToggleCase, DisableBreakpoint, EnableBreakpoint, EditLogBreakpoint,
        // DebuggerRunToCursor, DebuggerEvaluateSelectedText, ToggleAutoSignatureHelp, ToggleGitBlameInline, OpenGitBlameCommit, ToggleIndentGuides,
        // ToggleInlayHints, ToggleInlineValues, ToggleInlineDiagnostics, ToggleEditPrediction, ToggleLineNumbers, SwapSelectionEnds,
        // SetMark, ToggleRelativeLineNumbers, ToggleSelectionMenu, ToggleSoftWrap, ToggleTabBar, UniqueLinesCaseInsensitive,
        // UniqueLinesCaseSensitive, ToggleGoToLine, OpenSelectedFilename, ToggleSelectedDiffHunks, ExpandAllDiffHunks
        "acceptSelectedCodeAction" => {
            Some(("editor::ConfirmCodeAction".into(), Some("Editor && showing_code_actions".into())))
        }
        "deleteWordLeft" => {
            Some(("editor::DeleteToPreviousWordStart".into(), Some("Editor".into())))
        }
        "deleteWordRight" => {
            Some(("editor::DeleteToNextWordEnd".into(), Some("Editor".into())))
        }
        "cursorPageDown" => {
            Some(("editor::MovePageDown".into(), Some("Editor".into())))
        }
        "cursorPageUp" => {
            Some(("editor::MovePageUp".into(), Some("Editor".into())))
        }
        "cursorHome" => {
            Some((r#"["editor::MoveToBeginningOfLine", { "stop_at_soft_wraps": true, "stop_at_indent": true }]"#.into(), Some("Editor".into())))
        }
        "cursorEnd" => {
            Some((r#"["editor::MoveToEndOfLine", { "stop_at_soft_wraps": true }]"#.into(), Some("Editor".into())))
        }
        "editor.action.addSelectionToNextFindMatch" => {
            Some((r#"["editor::SelectNext", { "replace_newest": false }]"#.into(), Some("Editor".into())))
        }
        "editor.action.moveSelectionToNextFindMatch" => {
            Some((r#"["editor::SelectNext", { "replace_newest": true }]"#.into(), Some("Editor".into())))
        }
        "editor.action.addSelectionToPreviousFindMatch" => {
            Some((r#"["editor::SelectPrevious", { "replace_newest": false }]"#.into(), Some("Editor".into())))
        }
        "editor.action.moveSelectionToPreviousFindMatch" => {
            Some((r#"["editor::SelectPrevious", { "replace_newest": true }]"#.into(), Some("Editor".into())))
        }
        "cursorHomeSelect" => {
            Some((r#"["editor::SelectToBeginningOfLine", { "stop_at_soft_wraps": true, "stop_at_indent": true }]"#.into(), Some("Editor".into())))
        }
        "cursorEndSelect" => {
            Some((r#"["editor::SelectToEndOfLine", { "stop_at_soft_wraps": true }]"#.into(), Some("Editor".into())))
        }
        "editor.action.triggerSuggest" => {
            Some(("editor::ShowCompletions".into(), Some("Editor".into())))
        }
        "editor.action.quickFix" => {
            Some(("editor::ToggleCodeActions".into(), Some("Editor".into())))
        }
        "editor.action.commentLine" => {
            Some(("editor::ToggleComments".into(), Some("Editor".into())))
        }
        "editor.foldLevel1" => {
            Some((r#"["editor::FoldAtLevel", 1]"#.into(), Some("Editor".into())))
        }
        "editor.foldLevel2" => {
            Some((r#"["editor::FoldAtLevel", 2]"#.into(), Some("Editor".into())))
        }
        "editor.foldLevel3" => {
            Some((r#"["editor::FoldAtLevel", 3]"#.into(), Some("Editor".into())))
        }
        "editor.foldLevel4" => {
            Some((r#"["editor::FoldAtLevel", 4]"#.into(), Some("Editor".into())))
        }
        "editor.foldLevel5" => {
            Some((r#"["editor::FoldAtLevel", 5]"#.into(), Some("Editor".into())))
        }
        "editor.foldLevel6" => {
            Some((r#"["editor::FoldAtLevel", 6]"#.into(), Some("Editor".into())))
        }
        "editor.foldLevel7" => {
            Some((r#"["editor::FoldAtLevel", 7]"#.into(), Some("Editor".into())))
        }
        "editor.action.insertCursorAbove" => {
            Some(("editor::AddSelectionAbove".into(), Some("Editor".into())))
        }
        "editor.action.insertCursorBelow" => {
            Some(("editor::AddSelectionBelow".into(), Some("Editor".into())))
        }
        "deleteLeft" => {
            Some(("editor::Backspace".into(), Some("Editor".into())))
        }
        "acceptRenameInput" => {
            Some(("editor::ConfirmRename".into(), Some("Editor && renaming".into())))
        }
        "editor.action.clipboardCopyAction" => {
            Some(("editor::Copy".into(), Some("Editor".into())))
        }
        "editor.action.clipboardCutAction" => {
            Some(("editor::Cut".into(), Some("Editor".into())))
        }
        "deleteRight" => {
            Some(("editor::Delete".into(), Some("Editor".into())))
        }
        "editor.action.deleteLines" => {
            Some(("editor::DeleteLine".into(), Some("Editor".into())))
        }
        "editor.action.copyLinesDownAction" => {
            Some(("editor::DuplicateLineDown".into(), Some("Editor".into())))
        }
        "editor.action.copyLinesUpAction" => {
            Some(("editor::DuplicateLineUp".into(), Some("Editor".into())))
        }
        "references-view.findReferences" => {
            Some(("editor::FindAllReferences".into(), Some("Editor".into())))
        }
        "editor.fold" => {
            Some(("editor::Fold".into(), Some("Editor".into())))
        }
        "editor.foldAll" => {
            Some(("editor::FoldAll".into(), Some("Editor".into())))
        }
        "editor.foldRecursively" => {
            Some(("editor::FoldRecursive".into(), Some("Editor".into())))
        }
        "editor.toggleFold" => {
            Some(("editor::ToggleFold".into(), Some("Editor".into())))
        }
        "editor.action.formatDocument" => {
            Some(("editor::Format".into(), Some("Editor".into())))
        }
        "editor.action.goToDeclaration" => {
            Some(("editor::GoToDeclaration".into(), Some("Editor && !menu".into())))
        }
        "editor.action.revealDefinition" => {
            Some(("editor::GoToDefinition".into(), Some("Editor".into())))
        }
        "editor.action.peekDefinition" => {
            Some(("editor::GoToDefinitionSplit".into(), Some("Editor".into())))
        }
        "editor.action.goToImplementation" => {
            Some(("editor::GoToImplementation".into(), Some("Editor".into())))
        }
        "editor.action.showHover" => {
            Some(("editor::Hover".into(), Some("Editor".into())))
        }
        "editor.action.indentLines" => {
            Some(("editor::Indent".into(), Some("Editor".into())))
        }
        "editor.action.joinLines" => {
            Some(("editor::JoinLines".into(), Some("Editor".into())))
        }
        "deleteAllRight" => {
            Some(("editor::KillRingCut".into(), Some("Editor".into())))
        }
        "scrollLineDown" => {
            Some(("editor::LineDown".into(), Some("Editor".into())))
        }
        "scrollLineUp" => {
            Some(("editor::LineUp".into(), Some("Editor".into())))
        }
        "cursorDown" => {
            Some(("editor::MoveDown".into(), Some("Editor".into())))
        }
        "cursorUp" => {
            Some(("editor::MoveUp".into(), Some("Editor".into())))
        }
        "cursorLeft" => {
            Some(("editor::MoveLeft".into(), Some("Editor".into())))
        }
        "cursorRight" => {
            Some(("editor::MoveRight".into(), Some("Editor".into())))
        }
        "cursorTop" => {
            Some(("editor::MoveToBeginning".into(), Some("Editor".into())))
        }
        "editor.action.jumpToBracket" => {
            Some(("editor::MoveToEnclosingBracket".into(), Some("Editor".into())))
        }
        "cursorBottom" => {
            Some(("editor::MoveToEnd".into(), Some("Editor".into())))
        }
        "cursorWordPartRight" => {
            Some(("editor::MoveToNextSubwordEnd".into(), Some("Editor".into())))
        }
        "cursorWordEndRight" => {
            Some(("editor::MoveToNextWordEnd".into(), Some("Editor".into())))
        }
        "cursorWordPartLeft" => {
            Some(("editor::MoveToPreviousSubwordStart".into(), Some("Editor".into())))
        }
        "cursorWordLeft" => {
            Some(("editor::MoveToPreviousWordStart".into(), Some("Editor".into())))
        }
        "editor.action.insertLineBefore" => {
            Some(("editor::NewlineAbove".into(), Some("Editor && mode == full".into())))
        }
        "editor.action.insertLineAfter" => {
            Some(("editor::NewlineBelow".into(), Some("Editor && mode == full".into())))
        }
        "editor.action.organizeImports" => {
            Some(("editor::OrganizeImports".into(), Some("Editor".into())))
        }
        "editor.action.outdentLines" => {
            Some(("editor::Outdent".into(), Some("Editor".into())))
        }
        "scrollPageDown" => {
            Some(("editor::PageDown".into(), Some("Editor".into())))
        }
        "scrollPageUp" => {
            Some(("editor::PageUp".into(), Some("Editor".into())))
        }
        "editor.action.clipboardPasteAction" => {
            Some(("editor::Paste".into(), Some("Editor".into())))
        }
        "redo" => {
            Some(("editor::Redo".into(), Some("Editor".into())))
        }
        "editor.action.rename" => {
            Some(("editor::Rename".into(), Some("Editor".into())))
        }
        "workbench.action.files.revealActiveFileInWindows" => {
            Some(("editor::RevealInFileManager".into(), Some("Editor".into())))
        }
        "editor.action.selectAll" => {
            Some(("editor::SelectAll".into(), Some("Editor".into())))
        }
        "cursorDownSelect" => {
            Some(("editor::SelectDown".into(), Some("Editor".into())))
        }
        "editor.action.smartSelect.expand" => {
            Some(("editor::SelectLargerSyntaxNode".into(), Some("Editor".into())))
        }
        "cursorLeftSelect" => {
            Some(("editor::SelectLeft".into(), Some("Editor".into())))
        }
        "expandLineSelection" => {
            Some(("editor::SelectLine".into(), Some("Editor".into())))
        }
        "cursorPageDownSelect" => {
            Some(("editor::SelectPageDown".into(), Some("Editor".into())))
        }
        "cursorPageUpSelect" => {
            Some(("editor::SelectPageUp".into(), Some("Editor".into())))
        }
        "cursorRightSelect" => {
            Some(("editor::SelectRight".into(), Some("Editor".into())))
        }
        "editor.action.smartSelect.shrink" => {
            Some(("editor::SelectSmallerSyntaxNode".into(), Some("Editor".into())))
        }
        "cursorTopSelect" => {
            Some(("editor::SelectToBeginning".into(), Some("Editor".into())))
        }
        "cursorBottomSelect" => {
            Some(("editor::SelectToEnd".into(), Some("Editor".into())))
        }
        "cursorWordPartRightSelect" => {
            Some(("editor::SelectToNextSubwordEnd".into(), Some("Editor".into())))
        }
        "cursorWordEndRightSelect" => {
            Some(("editor::SelectToNextWordEnd".into(), Some("Editor".into())))
        }
        "cursorWordPartLeftSelect" => {
            Some(("editor::SelectToPreviousSubwordStart".into(), Some("Editor".into())))
        }
        "cursorWordLeftSelect" => {
            Some(("editor::SelectToPreviousWordStart".into(), Some("Editor".into())))
        }
        "cursorUpSelect" => {
            Some(("editor::SelectUp".into(), Some("Editor".into())))
        }
        "tab" => {
            Some(("editor::Tab".into(), Some("Editor".into())))
        }
        "outdent" => {
            Some(("editor::Backtab".into(), Some("Editor".into())))
        }
        "editor.debug.action.toggleBreakpoint" => {
            Some(("editor::ToggleBreakpoint".into(), Some("Editor".into())))
        }
        "editor.action.transposeLetters" => {
            Some(("editor::Transpose".into(), Some("Editor".into())))
        }
        "undo" => {
            Some(("editor::Undo".into(), Some("Editor".into())))
        }
        "cursorUndo" => {
            Some(("editor::UndoSelection".into(), Some("Editor".into())))
        }
        "editor.unfoldAll" => {
            Some(("editor::UnfoldAll".into(), Some("Editor".into())))
        }
        "editor.unfold" => {
            Some(("editor::UnfoldLines".into(), Some("Editor".into())))
        }
        "editor.unfoldRecursively" => {
            Some(("editor::UnfoldRecursive".into(), Some("Editor".into())))
        }
        // crates/search/src/buffer_search.rs
        // Missing:
        // Dismiss, FocusEditor
        "actions.find" => {
            Some(("buffer_search::Deploy".into(), Some("Editor && mode == full".into())))
        }
        "editor.action.startFindReplaceAction" => {
            Some(("buffer_search::DeployReplace".into(), Some("Editor && mode == full".into())))
        }
        // crates/assistant_context_editor/src/context_editor.rs
        // Missing:
        // Assist, ConfirmCommand, CopyCode, CycleMessageRole, Edit, InsertIntoEditor, QuoteSelection, Split

        // crates/markdown/src/markdown.rs
        // Missing:
        // Copy, CopyAsMarkdown

        // crates/repl/src/repl_sessions_ui.rs
        // Missing:
        // RunInPlace, ClearOutputs, Sessions, Interrupt, Shutdown, RefreshKernelspecs
        "jupyter.runAndDebugCell" => {
            Some(("repl::Run".into(), Some("Editor && jupyter && !ContextEditor".into())))
        }
        // crates/git/src/git.rs
        // Missing:
        // ToggleStaged, StageAndNext, UnstageAndNext, StageFile, StageAll, UnstageAll, RestoreTrackedFiles, TrashUntrackedFiles,
        // Uncommit, Push, ForcePush, Pull, Fetch, Commit, Amend, Cancel, ExpandCommitEditor, GenerateCommitMessage, Init, RestoreFile,
        // Restore, Blame

        // crates/agent/src/assistant.rs
        // Missing:
        // NewTextThread, ToggleContextPicker, ToggleNavigationMenu, ToggleOptionsMenu, DeleteRecentlyOpenThread, ToggleProfileSelector,
        // RemoveAllContext, ExpandMessageEditor, OpenHistory, AddContextServer, RemoveSelectedThread, Chat, ChatMode, CycleNextInlineAssist,
        // CyclePreviousInlineAssist, FocusUp, FocusDown, FocusLeft, FocusRight, RemoveFocusedContext, AcceptSuggestedContext,
        // OpenActiveThreadAsMarkdown, OpenAgentDiff, Keep, Reject, RejectAll, KeepAll

        // crates/search/src/search.rs
        // Missing:
        // FocusSearch, ToggleIncludeIgnored, ToggleReplace, NextHistoryQuery, PreviousHistoryQuery, SplitLeft, SplitUp,
        // SplitRight, SplitDown, SplitHorizontal, SplitVertical, SwapItemLeft, SwapItemRight, TogglePreviewTab
        "toggleFindWholeWord" | "toggleSearchWholeWord" | "toggleSearchEditorWholeWord" => {
            Some(("search::ToggleWholeWord".into(), Some("Pane".into())))
        }
        "toggleFindCaseSensitive" | "toggleSearchCaseSensitive" | "toggleSearchEditorCaseSensitive" => {
            Some(("search::ToggleCaseSensitive".into(), Some("Pane".into())))
        }
        "toggleFindInSelection" => {
            Some(("search::ToggleSelection".into(), Some("BufferSearchBar".into())))
        }
        "toggleFindRegex" => {
            Some(("search::ToggleRegex".into(), Some("BufferSearchBar".into())))
        }
        "editor.action.nextMatchFindAction" => {
            if let Some(when) = when {
                match when {
                    "editorFocus" => {
                        Some(("search::SelectNextMatch".into(), Some("Pane".into())))
                    }
                    "editorFocus && findInputFocussed" => {
                        Some(("search::SelectNextMatch".into(), Some("BufferSearchBar".into())))
                    }
                    _ => None,
                }
            } else {
                Some(("search::SelectNextMatch".into(), None))
            }
        }
        "editor.action.previousMatchFindAction" => {
            if let Some(when) = when {
                match when {
                    "editorFocus" => {
                        Some(("search::SelectPreviousMatch".into(), Some("Pane".into())))
                    }
                    "editorFocus && findInputFocussed" => {
                        Some(("search::SelectPreviousMatch".into(), Some("BufferSearchBar".into())))
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        "editor.action.selectAllMatches" => {
            if when.is_some_and(|when| when == "editorFocus && findWidgetVisible") {
                Some(("search::SelectAllMatches".into(), Some("BufferSearchBar".into())))
            } else {
                Some(("search::SelectAllMatches".into(), Some("Pane".into())))
            }
        }
        "editor.action.replaceAll" => {
            if let Some(when) = when {
                match when {
                    "editorFocus && findWidgetVisible" => 
                        Some(("search::ReplaceAll".into(), Some("BufferSearchBar".into()))),
                    "editorFocus && findWidgetVisible && replaceInputFocussed" => {
                        Some(("search::ReplaceAll".into(), Some("BufferSearchBar && in_replace > Editor".into())))
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        "editor.action.replaceOne" => {
            if let Some(when) = when {
                match when {
                    "editorFocus && findWidgetVisible" => 
                        Some(("search::Replace".into(), Some("BufferSearchBar".into()))),
                    "editorFocus && findWidgetVisible && replaceInputFocussed" => {
                        Some(("search::Replace".into(), Some("BufferSearchBar && in_replace > Editor".into())))
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        // crates/language_model_selector/src/language_model_selector.rs
        // Missing:
        // ToggleModelSelector

        // crates/rules_library/src/rules_library.rs
        // Missing:
        // NewRule, DeleteRule, DuplicateRule, ToggleDefaultRule

        // crates/search/src/project_search.rs
        // Missing:
        // SearchInNew, ToggleFocus, NextField, ToggleFilters

        // crates/workspace/src/pane.rs
        // Missing:
        // DeploySearch, AlternateFile, JoinIntoNext, JoinAll, RevealInProjectPanel
        "workbench.action.closeEditorsInGroup" => {
            Some((r#"["pane::CloseAllItems", { "close_pinned": false }]"#.into(), Some("Pane".into())))
        }
        "workbench.action.closeActiveEditor" => {
            Some((r#"["pane::CloseActiveItem", { "close_pinned": false }]"#.into(), Some("Pane".into())))
        }
        "workbench.action.closeUnmodifiedEditors" => {
            Some((r#"["pane::CloseCleanItems", { "close_pinned": false }]"#.into(), Some("Pane".into())))
        }
        "workbench.action.closeEditorsToTheLeft" => {
            Some((r#"["pane::CloseItemsToTheLeft", { "close_pinned": false }]"#.into(), Some("Pane".into())))
        }
        "workbench.action.closeEditorsToTheRight" => {
            Some((r#"["pane::CloseItemsToTheRight", { "close_pinned": false }]"#.into(), Some("Pane".into())))
        }
        "workbench.action.closeOtherEditors" => {
            Some((r#"["pane::CloseInactiveItems", { "close_pinned": false }]"#.into(), Some("Pane".into())))
        }
        "workbench.action.openEditorAtIndex1" => {
            Some((r#"["pane::ActivateItem", 0]"#.into(), Some("Pane".into())))
        }
        "workbench.action.openEditorAtIndex2" => {
            Some((r#"["pane::ActivateItem", 1]"#.into(), Some("Pane".into())))
        }
        "workbench.action.openEditorAtIndex3" => {
            Some((r#"["pane::ActivateItem", 2]"#.into(), Some("Pane".into())))
        }
        "workbench.action.openEditorAtIndex4" => {
            Some((r#"["pane::ActivateItem", 3]"#.into(), Some("Pane".into())))
        }
        "workbench.action.openEditorAtIndex5" => {
            Some((r#"["pane::ActivateItem", 4]"#.into(), Some("Pane".into())))
        }
        "workbench.action.openEditorAtIndex6" => {
            Some((r#"["pane::ActivateItem", 5]"#.into(), Some("Pane".into())))
        }
        "workbench.action.openEditorAtIndex7" => {
            Some((r#"["pane::ActivateItem", 6]"#.into(), Some("Pane".into())))
        }
        "workbench.action.openEditorAtIndex8" => {
            Some((r#"["pane::ActivateItem", 7]"#.into(), Some("Pane".into())))
        }
        "workbench.action.openEditorAtIndex9" => {
            Some((r#"["pane::ActivateItem", 8]"#.into(), Some("Pane".into())))
        }
        "workbench.action.previousEditor" => {
            Some(("pane::ActivatePreviousItem".into(), Some("Pane".into())))
        }
        "workbench.action.nextEditor" => {
            Some(("pane::ActivateNextItem".into(), Some("Pane".into())))
        }
        "workbench.action.lastEditorInGroup" => {
            Some(("pane::ActivateLastItem".into(), Some("Pane".into())))
        }
        "workbench.action.navigateBack" => {
            Some(("pane::GoBack".into(), Some("Pane".into())))
        }
        "workbench.action.navigateForward" => {
            Some(("pane::GoForward".into(), Some("Pane".into())))
        }
        "workbench.action.reopenClosedEditor" => {
            Some(("pane::ReopenClosedItem".into(), Some("Workspace".into())))
        }
        "workbench.action.pinEditor" | "workbench.action.unpinEditor" => {
            Some(("pane::TogglePinTab".into(), Some("Pane".into())))
        }
        "markdown.showPreviewToSide" => {
            Some(("markdown::OpenPreviewToTheSide".into(), Some("Editor".into())))
        }
        "markdown.showPreview" => {
            Some(("markdown::OpenPreview".into(), Some("Editor".into())))
        }
        // go_to_line
        "workbench.action.gotoLine" => {
            Some(("go_to_line::Toggle".into(), Some("Editor && mode == full".into())))
        }
        // crates/workspace/src/toast_layer.rs
        // Missing:
        // RunAction

        // crates/title_bar/src/application_menu.rs
        // Missing:
        // OpenApplicationMenu, ActivateMenuRight, ActivateMenuLeft

        // crates/tab_switcher/src/tab_switcher.rs
        // Missing:
        // CloseSelectedItem, ToggleAll
        "workbench.action.quickOpenNavigateNextInEditorPicker" => {
            Some(("tab_switcher::Toggle".into(), Some("Workspace".into())))
        }
        "workbench.action.quickOpenNavigatePreviousInEditorPicker" => {
            Some((r#"["tab_switcher::Toggle", { "select_last": true }]"#.into(), Some("Workspace".into())))
        }
        // crates/project_panel/src/project_panel.rs
        // Missing:
        // CollapseAllEntries, NewDirectory, NewFile, Duplicate, RemoveFromProject, OpenWithSystem, Rename, Open, OpenPermanent,
        // ToggleHideGitIgnore, NewSearchInDirectory, UnfoldDirectory, FoldDirectory, SelectParent, SelectNextGitEntry, SelectPrevGitEntry,
        // SelectNextDiagnostic, SelectPrevDiagnostic, SelectNextDirectory, SelectPrevDirectory, ToggleFocus
        "deleteFile" => {
            Some((r#"["project_panel::Delete", { "skip_prompt": false }]"#.into(), Some("ProjectPanel".into())))
        }
        "moveFileToTrash" => {
            Some((r#"["project_panel::Trash", { "skip_prompt": true }]"#.into(), Some("ProjectPanel".into())))
        }
        "list.expand" => {
            Some(("project_panel::ExpandSelectedEntry".into(), Some("ProjectPanel".into())))
        }
        "list.collapse" => {
            Some(("project_panel::CollapseSelectedEntry".into(), Some("ProjectPanel".into())))
        }
        "filesExplorer.copy" => {
            Some(("project_panel::Copy".into(), Some("ProjectPanel".into())))
        }
        "revealFileInOS" => {
            Some(("project_panel::RevealInFileManager".into(), Some("ProjectPanel".into())))
        }
        "filesExplorer.cut" => {
            Some(("project_panel::Cut".into(), Some("ProjectPanel".into())))
        }
        "filesExplorer.paste" => {
            Some(("project_panel::Paste".into(), Some("ProjectPanel".into())))
        }
        "workbench.view.explorer" => {
            Some(("project_panel::ToggleFocus".into(), Some("Workspace".into())))
        }
        // crates/git_ui/src/git_panel.rs
        // Missing:
        // Close, OpenMenu, FocusEditor, FocusChanges, ToggleFillCoAuthors, GenerateCommitMessage
        "workbench.view.scm" => {
            Some(("git_panel::ToggleFocus".into(), Some("Workspace".into())))
        }
        // crates/collab_ui/src/collab_panel.rs
        // Missing:
        // ToggleFocus, Remove, Secondary, CollapseSelectedChannel, ExpandSelectedChannel, StartMoveChannel, MoveSelected, InsertSpace,

        // crates/outline_panel/src/outline_panel.rs
        // Missing:
        // CollapseAllEntries, CollapseSelectedEntry, ExpandAllEntries, ExpandSelectedEntry, FoldDirectory, OpenSelectedEntry,
        // RevealInFileManager, SelectParent, ToggleActiveEditorPin, ToggleFocus, UnfoldDirectory

        // crates/terminal/src/terminal.rs
        // Missing:
        // ShowCharacterPalette, SearchTest, ToggleViMode
        "workbench.action.terminal.clear" => {
            Some(("terminal::Clear".into(), Some("Terminal".into())))
        }
        "workbench.action.terminal.copySelection" => {
            Some(("terminal::Copy".into(), Some("Terminal".into())))
        }
        "workbench.action.terminal.paste" => {
            Some(("terminal::Paste".into(), Some("Terminal".into())))
        }
        "workbench.action.terminal.scrollUp" => {
            Some(("terminal::ScrollLineUp".into(), Some("Terminal".into())))
        }
        "workbench.action.terminal.scrollDown" => {
            Some(("terminal::ScrollLineDown".into(), Some("Terminal".into())))
        }
        "workbench.action.terminal.scrollUpPage" => {
            Some(("terminal::ScrollPageUp".into(), Some("Terminal".into())))
        }
        "workbench.action.terminal.scrollDownPage" => {
            Some(("terminal::ScrollPageDown".into(), Some("Terminal".into())))
        }
        "workbench.action.terminal.scrollToTop" => {
            Some(("terminal::ScrollToTop".into(), Some("Terminal".into())))
        }
        "workbench.action.terminal.scrollToBottom" => {
            Some(("terminal::ScrollToBottom".into(), Some("Terminal".into())))
        }
        // crates/terminal_view/src/terminal_view.rs
        // Missing:
        // SendKeystroke
        "workbench.action.terminal.sendSequence" => {
            if let Some(args) = args {
                Some((format!(r#"["terminal::SendText", "{}"]"#, args), Some("Terminal".into())))
            } else {
                None
            }
        }
        _ => None,
    }
}

struct KeymapSerializer {
    result: String,
    indent_cache: Vec<String>,
    current_indent: usize,
}

enum IndentAction {
    Increase,
    Decrease,
}

impl KeymapSerializer {
    const KEYMAP_HEADER: &[&str] = &[
        "// Zed keymap",
        "//",
        "// For information on binding keys, see the Zed",
        "// documentation: https://zed.dev/docs/key-bindings",
        "//",
        "// To see the default key bindings run `zed: open default keymap`",
        "// from the command palette.",
        "//",
        "// NOTE: This file is auto-generated by Zed.",
        "//",
    ];

    fn new() -> Self {
        let mut this = KeymapSerializer {
            result: String::new(),
            indent_cache: (0..5).map(|n| "  ".repeat(n)).collect(),
            current_indent: 0,
        };
        for line in Self::KEYMAP_HEADER {
            this.append_line(line);
        }
        this
    }

    fn append_line(&mut self, content: &str) -> &mut Self {
        // We should not have more than 5 levels of indentation in theory, but just in case we do,
        if let Some(indent) = self.indent_cache.get(self.current_indent) {
            self.result.push_str(indent);
        } else {
            self.indent_cache.push("  ".repeat(self.current_indent));
            self.result.push_str(self.indent_cache.last().unwrap());
        }
        self.result.push_str(content);
        #[cfg(target_os = "windows")]
        self.result.push_str("\r\n");
        #[cfg(not(target_os = "windows"))]
        self.result.push('\n');
        self
    }

    fn append_line_with_indent(&mut self, content: &str, indent_action: IndentAction) -> &mut Self {
        match indent_action {
            IndentAction::Increase => {
                self.append_line(content);
                self.current_indent += 1;
            }
            IndentAction::Decrease => {
                self.current_indent -= 1;
                self.append_line(content);
            }
        }
        self
    }
}

fn serialize_actions(
    serializer: &mut KeymapSerializer,
    context: Option<String>,
    actions: &[ZedBindingContent],
    has_more: bool,
) {
    serializer.append_line_with_indent("{", IndentAction::Increase);
    if let Some(ref context) = context {
        serializer.append_line(&format!(r#""context": "{}","#, context));
    }
    serializer.append_line_with_indent(r#""bindings": {"#, IndentAction::Increase);
    for (action_idx, action) in actions.iter().enumerate() {
        let is_last_action = action_idx == actions.len() - 1;

        for comment in action.comments.lines() {
            serializer.append_line(&format!("// {}", comment));
        }
        if is_last_action {
            serializer.append_line(&format!(r#""{}": "{}""#, action.binding, action.action));
        } else {
            serializer.append_line(&format!(r#""{}": "{}","#, action.binding, action.action));
        }
    }
    serializer.append_line_with_indent("}", IndentAction::Decrease);
    if has_more {
        serializer.append_line_with_indent("},", IndentAction::Decrease);
    } else {
        serializer.append_line_with_indent("}", IndentAction::Decrease);
    }
}

fn serialize_all(
    normal_bindings: Vec<ZedBindingContent>,
    other_bindings: HashMap<String, Vec<ZedBindingContent>>,
    skipped: Vec<(String, String)>,
) -> String {
    let mut serializer = KeymapSerializer::new();
    if !skipped.is_empty() {
        serializer
            .append_line("// The following bindings are skipped:")
            .append_line("//");
        for (action, reason) in skipped {
            serializer.append_line("// Skipped shortcut:");
            for line in action.lines() {
                serializer.append_line(&format!("// {}", line));
            }
            serializer
                .append_line(&format!("// Skipped reason: {}", reason))
                .append_line("//");
        }
    }

    serializer.append_line_with_indent("[", IndentAction::Increase);

    if !normal_bindings.is_empty() {
        serialize_actions(
            &mut serializer,
            None,
            &normal_bindings,
            !other_bindings.is_empty(),
        );
    }
    if !other_bindings.is_empty() {
        let last_idx = other_bindings.len() - 1;
        for (idx, (context, actions)) in other_bindings.into_iter().enumerate() {
            let has_more = idx != last_idx;
            serialize_actions(&mut serializer, Some(context), &actions, has_more);
        }
    }
    serializer.append_line_with_indent("]", IndentAction::Decrease);
    serializer.result
}

#[cfg(test)]
mod tests {
    use collections::HashMap;

    use crate::vscode_import::serialize_all;

    use super::{VsCodeShortcuts, ZedBindingContent};

    fn check_serialization_result(result: String, expected: &str) {
        let mut new_result = String::new();
        for line in result.lines() {
            new_result.push_str(line.trim());
        }
        let mut new_expected = String::new();
        for line in expected.lines() {
            new_expected.push_str(line.trim());
        }
        assert_eq!(new_result, new_expected);
    }

    #[test]
    fn test_serialization() {
        let normal_bindings = vec![
            ZedBindingContent {
                binding: "ctrl+shift+f".to_string(),
                action: "editor::Find".to_string(),
                comments: "Find in editor".to_string(),
            },
            ZedBindingContent {
                binding: "ctrl+shift+r".to_string(),
                action: "editor::Replace".to_string(),
                comments: "Replace in editor\r\nHello".to_string(),
            },
        ];
        let mut other_bindings = HashMap::default();
        other_bindings.insert(
            "Editor".to_string(),
            vec![
                ZedBindingContent {
                    binding: "ctrl+shift+f".to_string(),
                    action: "editor::Find".to_string(),
                    comments: "Find in editor".to_string(),
                },
                ZedBindingContent {
                    binding: "ctrl+shift+r".to_string(),
                    action: "editor::Replace".to_string(),
                    comments: "Replace in editor\r\nHello".to_string(),
                },
            ],
        );
        other_bindings.insert(
            "Workspace".to_string(),
            vec![
                ZedBindingContent {
                    binding: "ctrl+shift+f".to_string(),
                    action: "editor::Find".to_string(),
                    comments: "Find in editor".to_string(),
                },
                ZedBindingContent {
                    binding: "ctrl+shift+r".to_string(),
                    action: "editor::Replace".to_string(),
                    comments: "Replace in editor\r\nHello".to_string(),
                },
            ],
        );
        let skipped = vec![
            (
                "\"key\": \"ctrl+shift+alt+i\"\n\"command\": \"editor.action.inspectTMScopes\""
                    .to_string(),
                "Unable to parse command".to_string(),
            ),
            (
                "\"key\": \"ctrl+shift+alt+alt\"\n\"command\": \"workbench.view.scm\"".to_string(),
                "Unable to parse keystroke".to_string(),
            ),
        ];
        let result = serialize_all(normal_bindings, other_bindings, skipped);
        check_serialization_result(
            result,
            r#"// Zed keymap
            //
            // For information on binding keys, see the Zed
            // documentation: https://zed.dev/docs/key-bindings
            //
            // To see the default key bindings run `zed: open default keymap`
            // from the command palette.
            //
            // NOTE: This file is auto-generated by Zed.
            //
            // The following bindings are skipped:
            //
            // Skipped shortcut:
            // "key": "ctrl+shift+alt+i"
            // "command": "editor.action.inspectTMScopes"
            // Skipped reason: Unable to parse command
            //
            // Skipped shortcut:
            // "key": "ctrl+shift+alt+alt"
            // "command": "workbench.view.scm"
            // Skipped reason: Unable to parse keystroke
            //
            [
                {
                    "bindings": {
                        // Find in editor
                        "ctrl+shift+f": "editor::Find",
                        // Replace in editor
                        // Hello
                        "ctrl+shift+r": "editor::Replace"
                    }
                },
                {
                    "context": "Editor",
                    "bindings": {
                        // Find in editor
                        "ctrl+shift+f": "editor::Find",
                        // Replace in editor
                        // Hello
                        "ctrl+shift+r": "editor::Replace"
                    }
                },
                {
                    "context": "Workspace",
                    "bindings": {
                        // Find in editor
                        "ctrl+shift+f": "editor::Find",
                        // Replace in editor
                        // Hello
                        "ctrl+shift+r": "editor::Replace"
                    }
                }
            ]
            "#,
        );
    }

    #[test]
    fn test_load_vscode_shortcuts() {
        use gpui::TestKeyboardMapper;

        let keyboard_mapper = TestKeyboardMapper::new();
        let content = r#"
        [
            {
                "key": "ctrl+[BracketLeft]",
                "command": "list.focusFirst",
            },
            {
                "key": "shift+[BracketRight]",
                "command": "list.focusFirst",
            },
            {
                "key": "ctrl+shift+alt+-",
                "command": "list.focusFirst",
            },
            {
                "key": "shift+4",
                "command": "list.focusFirst",
            },
            {
                "key": "shift+oem_3",
                "command": "workbench.action.openEditorAtIndex1",
            }
        ]
        "#;
        let shortcuts = VsCodeShortcuts::from_str(content).unwrap();
        assert_eq!(shortcuts.content.len(), 5);
        let result = shortcuts.parse_shortcuts(&keyboard_mapper);
        check_serialization_result(
            result,
            r#"// Zed keymap
            //
            // For information on binding keys, see the Zed
            // documentation: https://zed.dev/docs/key-bindings
            //
            // To see the default key bindings run `zed: open default keymap`
            // from the command palette.
            //
            // NOTE: This file is auto-generated by Zed.
            //
            [
                {
                    "context": "Pane",
                    "bindings": {
                        // {
                        //   "key": "shift+oem_3",
                        //   "command": "workbench.action.openEditorAtIndex1"
                        // }
                        "shift-oem_3": "["pane::ActivateItem", 0]"
                    }
                },
                {
                    "context": "menu",
                    "bindings": {
                        // {
                        //   "key": "ctrl+[BracketLeft]",
                        //   "command": "list.focusFirst"
                        // }
                        "ctrl-[bracketleft]": "menu::SelectFirst",
                        // {
                        //   "key": "shift+[BracketRight]",
                        //   "command": "list.focusFirst"
                        // }
                        "shift-[bracketright]": "menu::SelectFirst",
                        // {
                        //   "key": "ctrl+shift+alt+-",
                        //   "command": "list.focusFirst"
                        // }
                        "ctrl-alt-_": "menu::SelectFirst",
                        // {
                        //   "key": "shift+4",
                        //   "command": "list.focusFirst"
                        // }
                        "$": "menu::SelectFirst"
                    }
                }
            ]
            "#,
        );

        let content = r#"
        [
            {
                "key": "ctrl+shift+a",
                "command": "list.focusFirst",
            },
            {
                "key": "ctrl+shift+=",
                "command": "junkui::张小白", // Test invalid command
            }
        ]
        "#;
        let shortcuts = VsCodeShortcuts::from_str(content).unwrap();
        assert_eq!(shortcuts.content.len(), 2);
        let result = shortcuts.parse_shortcuts(&keyboard_mapper);
        check_serialization_result(
            result,
            r#"// Zed keymap
            //
            // For information on binding keys, see the Zed
            // documentation: https://zed.dev/docs/key-bindings
            //
            // To see the default key bindings run `zed: open default keymap`
            // from the command palette.
            //
            // NOTE: This file is auto-generated by Zed.
            //
            // The following bindings are skipped:
            //
            // Skipped shortcut:
            // {
            //   "key": "ctrl+shift+=",
            //   "command": "junkui::张小白"
            // }
            // Skipped reason: Unable to parse command: junkui::张小白
            //
            [
                {
                    "context": "menu",
                    "bindings": {
                        // {
                        //   "key": "ctrl+shift+a",
                        //   "command": "list.focusFirst"
                        // }
                        "ctrl-shift-a": "menu::SelectFirst"
                    }
                }
            ]
            "#,
        );
    }
}
