use anyhow::Result;
use collections::IndexMap;
use fs::Fs;
use gpui::{AsyncWindowContext, Keystroke, PlatformKeyboardMapper};
use serde_json::{Map, Value};
use util::ResultExt;

use std::sync::Arc;

use crate::{KeymapFile, keymap_file::KeymapSection};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VsCodeSettingsSource {
    VsCode,
    Cursor,
}

impl std::fmt::Display for VsCodeSettingsSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VsCodeSettingsSource::VsCode => write!(f, "VS Code"),
            VsCodeSettingsSource::Cursor => write!(f, "Cursor"),
        }
    }
}

pub struct VsCodeSettings {
    pub source: VsCodeSettingsSource,
    content: Map<String, Value>,
}

impl VsCodeSettings {
    pub fn from_str(content: &str, source: VsCodeSettingsSource) -> Result<Self> {
        Ok(Self {
            source,
            content: serde_json_lenient::from_str(content)?,
        })
    }

    pub async fn load_user_settings(source: VsCodeSettingsSource, fs: Arc<dyn Fs>) -> Result<Self> {
        let path = match source {
            VsCodeSettingsSource::VsCode => paths::vscode_settings_file(),
            VsCodeSettingsSource::Cursor => paths::cursor_settings_file(),
        };
        let content = fs.load(path).await?;
        Ok(Self {
            source,
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
        let content = r#"
        [
            {
                "key": "ctrl+shift+a",
                "command": "list.focusFirst",
            },
            {
                "key": "ctrl+shift+=",
                "command": "menu::SelectFirst",
            },
            {
                "key": "ctrl+shift+[BracketLeft]",
                "command": "menu::SelectFirst",
            },
            {
                "key": "ctrl+shift+oem_3",
                "command": "menu::SelectFirst",
            }
        ]
        "#;
        // let content = fs.load(paths::vscode_shortcuts_file()).await?;
        println!("Loaded shortcuts: {}", content);

        Ok(Self {
            content: serde_json_lenient::from_str(&content)?,
        })
    }

    pub fn parse_shortcuts(
        &self,
        keyboard_mapper: &dyn PlatformKeyboardMapper,
    ) -> (KeymapFile, Vec<(String, String)>) {
        let mut result = KeymapFile::default();
        let mut skipped = Vec::new();
        for content in self.content.iter() {
            let Some(shortcut) = content.get("key").and_then(|key| key.as_str()) else {
                continue;
            };
            let Some(keystroke) = Keystroke::parse_keystroke_components(shortcut, '+')
                .ok()
                .map(|keystroke| keystroke.into_gpui_style(keyboard_mapper))
            else {
                skipped.push((
                    shortcut.to_string(),
                    "Unable to parse keystroke".to_string(),
                ));
                continue;
            };
            let Some(command) = content.get("command").and_then(|command| command.as_str()) else {
                continue;
            };
            let context = content
                .get("when")
                .and_then(|when| when.as_str())
                .unwrap_or_default()
                .to_string();
            let (action, _) = vscode_shortcut_command_to_zed_action(command, Some(&context))
                .unwrap_or((ActionType::String(command), None));
            let Ok(action) = serde_json_lenient::from_str(&action.to_string()) else {
                skipped.push((
                    shortcut.to_string(),
                    format!("Unable to parse command: {}, action: {:?}", command, action),
                ));
                continue;
            };
            result.insert_keystroke(context, keystroke, action);
        }
        println!("=> result: {:#?}", result);
        println!("=> skipped: {:#?}", skipped);
        (result, skipped)
    }
}

#[derive(Debug)]
enum ActionType<'t> {
    String(&'t str),
    Other(&'t str),
}

impl ActionType<'_> {
    fn to_string(&self) -> String {
        match self {
            ActionType::String(s) => format!("\"{}\"", s),
            ActionType::Other(s) => s.to_string(),
        }
    }
}

fn vscode_shortcut_command_to_zed_action<'t, 's>(
    command: &'t str,
    when: Option<&'s str>,
) -> Option<(ActionType<'t>, Option<&'s str>)> {
    let action;
    let mut context = None;
    match command {
        // crates/menu/src/menu.rs
        // Missing:
        // SecondaryConfirm, Restart, EndSlot
        "list.focusFirst" | "list.focusAnyFirst" => {
            action = ActionType::String("menu::SelectFirst");
            context = Some("menu");
        }
        "list.focusLast" | "list.focusAnyLast" => {
            action = ActionType::String("menu::SelectLast");
            context = Some("menu");
        }
        "list.focusUp" | "list.focusAnyUp" => {
            action = ActionType::String("menu::SelectPrevious");
            context = Some("menu");
        }
        "list.focusDown" | "list.focusAnyDown" => {
            action = ActionType::String("menu::SelectNext");
            context = Some("menu");
        }
        "list.select" => {
            action = ActionType::String("menu::Confirm");
            context = Some("menu");
        }
        "list.clear" => {
            action = ActionType::String("menu::Cancel");
            context = Some("menu");
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
            // https://github.com/microsoft/vscode/blob/e9daa2e0f3dd86459eea57aac3f5f181d065c06d/src/vs/workbench/browser/actions/workspaceCommands.ts#L28
            action = ActionType::String("workspace::AddFolderToProject");
        }
        "workbench.action.closeWindow" => {
            action = ActionType::String("workspace::CloseWindow");
        }
        "workbench.action.files.newUntitledFile" => {
            action = ActionType::String("workspace::NewFile");
        }
        "workbench.view.search" => {
            action = ActionType::String("workspace::NewSearch");
        }
        "workbench.action.terminal.new" => {
            action = ActionType::String("workspace::NewTerminal");
        }
        "workbench.action.newWindow" => {
            action = ActionType::String("workspace::NewWindow");
        }
        "workbench.action.files.openFolder" => {
            action = ActionType::String("workspace::Open");
        }
        "workbench.action.files.openFile" => {
            action = ActionType::String("workspace::OpenFiles");
        }
        "workbench.action.files.saveAs" => {
            action = ActionType::String("workspace::SaveAs");
        }
        "workbench.action.files.saveWithoutFormatting" => {
            action = ActionType::String("workspace::SaveWithoutFormat");
        }
        "workbench.action.togglePanel" => {
            action = ActionType::String("workspace::ToggleBottomDock");
            context = Some("Workspace");
        }
        "workbench.action.toggleSidebarVisibility" => {
            action = ActionType::String("workspace::ToggleLeftDock");
            context = Some("Workspace");
        }
        "workbench.action.toggleAuxiliaryBar" => {
            action = ActionType::String("workspace::ToggleRightDock");
            context = Some("Workspace");
        }
        "workbench.action.openEditorAtIndex1" => {
            action = ActionType::Other("[\"workspace::ActivatePane\", 0]");
            context = Some("Workspace");
        }
        "workbench.action.openEditorAtIndex2" => {
            action = ActionType::Other("[\"workspace::ActivatePane\", 1]");
            context = Some("Workspace");
        }
        "workbench.action.openEditorAtIndex3" => {
            action = ActionType::Other("[\"workspace::ActivatePane\", 2]");
            context = Some("Workspace");
        }
        "workbench.action.openEditorAtIndex4" => {
            action = ActionType::Other("[\"workspace::ActivatePane\", 3]");
            context = Some("Workspace");
        }
        "workbench.action.openEditorAtIndex5" => {
            action = ActionType::Other("[\"workspace::ActivatePane\", 4]");
            context = Some("Workspace");
        }
        "workbench.action.openEditorAtIndex6" => {
            action = ActionType::Other("[\"workspace::ActivatePane\", 5]");
            context = Some("Workspace");
        }
        "workbench.action.openEditorAtIndex7" => {
            action = ActionType::Other("[\"workspace::ActivatePane\", 6]");
            context = Some("Workspace");
        }
        "workbench.action.openEditorAtIndex8" => {
            action = ActionType::Other("[\"workspace::ActivatePane\", 7]");
            context = Some("Workspace");
        }
        "workbench.action.openEditorAtIndex9" => {
            action = ActionType::Other("[\"workspace::ActivatePane\", 8]");
            context = Some("Workspace");
        }
        "workbench.action.closeAllEditors" => {
            action = ActionType::String("workspace::CloseAllItemsAndPanes");
            context = Some("Pane");
        }
        "workbench.action.files.save" => {
            action = ActionType::String("workspace::Save");
            context = Some("Workspace");
        }
        "saveAll" => {
            action = ActionType::String("workspace::SaveAll");
            context = Some("Workspace");
        }
        "workbench.action.focusLeftGroup" => {
            action = ActionType::String("workspace::ActivatePaneLeft");
            context = Some("Workspace");
        }
        "workbench.action.focusRightGroup" => {
            action = ActionType::String("workspace::ActivatePaneRight");
            context = Some("Workspace");
        }
        "workbench.action.focusAboveGroup" => {
            action = ActionType::String("workspace::ActivatePaneUp");
            context = Some("Workspace");
        }
        "workbench.action.focusBelowGroup" => {
            action = ActionType::String("workspace::ActivatePaneDown");
            context = Some("Workspace");
        }
        // crates/zed_actions/src/lib.rs
        // Missing:
        // OpenBrowser, OpenZedUrl, OpenAccountSettings, OpenServerSettings, About, OpenLicenses, OpenTelemetryLog, DecreaseUiFontSize,
        // IncreaseUiFontSize, ResetUiFontSize, workspace::CopyPath, workspace::CopyRelativePath, workspace::CopyFileName, git::*,
        // feadback::*, icon_theme_selector::Toggle, agent::OpenConfiguration, assistant::*, assistant::InlineAssist, projects::OpenRemote,
        // task::*, outline::*, zed_predict_onboarding::*, git_onboarding::*,
        "workbench.action.openSettings" => {
            action = ActionType::String("zed::OpenSettings");
        }
        "workbench.action.openDefaultKeybindingsFile" => {
            action = ActionType::String("zed::OpenDefaultKeymap");
        }
        "workbench.action.quit" => {
            action = ActionType::String("zed::Quit");
        }
        "workbench.action.openGlobalKeybindings" => {
            action = ActionType::String("zed::OpenKeymap");
            context = Some("Workspace");
        }
        "workbench.view.extensions" => {
            action = ActionType::String("zed::OpenExtensions");
            context = Some("Workspace");
        }
        "workbench.action.zoomOut" => {
            action = ActionType::Other(r#"["zed::DecreaseBufferFontSize", { "persist": false }]"#);
        }
        "workbench.action.zoomIn" => {
            action = ActionType::Other(r#"["zed::IncreaseBufferFontSize", { "persist": false }]"#);
        }
        "workbench.action.zoomReset" => {
            action = ActionType::Other(r#"["zed::ResetBufferFontSize", { "persist": false }]"#);
        }
        "workbench.action.showCommands" => {
            action = ActionType::String("command_palette::Toggle");
            context = Some("Workspace");
        }
        "workbench.action.selectTheme" => {
            action = ActionType::String("theme_selector::Toggle");
            context = Some("Workspace");
        }
        "workbench.action.openRecent" => {
            action = ActionType::String("projects::OpenRecent");
            context = Some("Workspace");
        }
        // crates/debugger_ui/src/debugger_ui.rs
        // Missing:
        // ToggleIgnoreBreakpoints, ClearAllBreakpoints, CreateDebuggingSession, FocusConsole, FocusVariables, FocusBreakpointList,
        // FocusFrames, FocusModules, FocusLoadedSources, FocusTerminal,
        "workbench.action.debug.start" => {
            action = ActionType::String("debugger::Start");
        }
        "workbench.action.debug.continue" => {
            action = ActionType::String("debugger::Continue");
        }
        "workbench.action.debug.disconnect" => {
            action = ActionType::String("debugger::Disconnect");
        }
        "workbench.action.debug.pause" => {
            action = ActionType::String("debugger::Pause");
        }
        "workbench.action.debug.restart" => {
            action = ActionType::String("debugger::Restart");
        }
        "workbench.action.debug.stepInto" => {
            action = ActionType::String("debugger::StepInto");
        }
        "workbench.action.debug.stepOver" => {
            action = ActionType::String("debugger::StepOver");
        }
        "workbench.action.debug.stepOut" => {
            action = ActionType::String("debugger::StepOut");
        }
        "workbench.action.debug.stepBack" => {
            action = ActionType::String("debugger::StepBack");
        }
        "workbench.action.debug.stop" => {
            action = ActionType::String("debugger::Stop");
        }
        // crates/zed/src/zed.rs
        // Missing:
        // DebugElements, Hide, HideOthers, Minimize, OpenDefaultSettings, OpenProjectSettings, OpenProjectTasks, OpenProjectDebugTasks,
        // OpenTasks, OpenDebugTasks, ResetDatabase, ShowAll, Zoom, TestPanic,
        "workbench.action.toggleFullScreen" => {
            action = ActionType::String("zed::ToggleFullScreen");
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
            // TODO: is this the right action?
            action = ActionType::String("editor::ConfirmCodeAction");
            context = Some("Editor && showing_code_actions");
        }
        "deleteWordLeft" => {
            action = ActionType::String("editor::DeleteToPreviousWordStart");
            context = Some("Editor");
        }
        "deleteWordRight" => {
            action = ActionType::String("editor::DeleteToNextWordEnd");
            context = Some("Editor");
        }
        "cursorPageDown" => {
            action = ActionType::String("editor::MovePageDown");
            context = Some("Editor");
        }
        "cursorPageUp" => {
            action = ActionType::String("editor::MovePageUp");
            context = Some("Editor");
        }
        "cursorHome" => {
            action = ActionType::Other(
                r#"["editor::MoveToBeginningOfLine", { "stop_at_soft_wraps": true, "stop_at_indent": true }]"#,
            );
            context = Some("Editor");
        }
        "cursorEnd" => {
            action =
                ActionType::Other(r#"["editor::MoveToEndOfLine", { "stop_at_soft_wraps": true }]"#);
            context = Some("Editor");
        }
        "editor.action.addSelectionToNextFindMatch" => {
            action = ActionType::Other(r#"["editor::SelectNext", { "replace_newest": false }]"#);
            context = Some("Editor");
        }
        "editor.action.moveSelectionToNextFindMatch" => {
            action = ActionType::Other(r#"["editor::SelectNext", { "replace_newest": true }]"#);
            context = Some("Editor");
        }
        "editor.action.addSelectionToPreviousFindMatch" => {
            action =
                ActionType::Other(r#"["editor::SelectPrevious", { "replace_newest": false }]"#);
            context = Some("Editor");
        }
        "editor.action.moveSelectionToPreviousFindMatch" => {
            action = ActionType::Other(r#"["editor::SelectPrevious", { "replace_newest": true }]"#);
            context = Some("Editor");
        }
        "cursorHomeSelect" => {
            action = ActionType::Other(
                r#"["editor::SelectToBeginningOfLine", { "stop_at_soft_wraps": true, "stop_at_indent": true }]"#,
            );
            context = Some("Editor");
        }
        "cursorEndSelect" => {
            action = ActionType::Other(
                r#"["editor::SelectToEndOfLine", { "stop_at_soft_wraps": true }]"#,
            );
            context = Some("Editor");
        }
        "editor.action.triggerSuggest" => {
            action = ActionType::String("editor::ShowCompletions");
            context = Some("Editor");
        }
        "editor.action.quickFix" => {
            action = ActionType::String("editor::ToggleCodeActions");
            context = Some("Editor");
        }
        "editor.action.commentLine" => {
            action = ActionType::String("editor::ToggleComments");
            context = Some("Editor");
        }
        "editor.foldLevel1" => {
            action = ActionType::Other(r#"["editor::FoldAtLevel", 1]"#);
            context = Some("Editor");
        }
        "editor.foldLevel2" => {
            action = ActionType::Other(r#"["editor::FoldAtLevel", 2]"#);
            context = Some("Editor");
        }
        "editor.foldLevel3" => {
            action = ActionType::Other(r#"["editor::FoldAtLevel", 3]"#);
            context = Some("Editor");
        }
        "editor.foldLevel4" => {
            action = ActionType::Other(r#"["editor::FoldAtLevel", 4]"#);
            context = Some("Editor");
        }
        "editor.foldLevel5" => {
            action = ActionType::Other(r#"["editor::FoldAtLevel", 5]"#);
            context = Some("Editor");
        }
        "editor.foldLevel6" => {
            action = ActionType::Other(r#"["editor::FoldAtLevel", 6]"#);
            context = Some("Editor");
        }
        "editor.foldLevel7" => {
            action = ActionType::Other(r#"["editor::FoldAtLevel", 7]"#);
            context = Some("Editor");
        }
        "editor.action.insertCursorAbove" => {
            action = ActionType::String("editor::AddSelectionAbove");
            context = Some("Editor");
        }
        "editor.action.insertCursorBelow" => {
            action = ActionType::String("editor::AddSelectionBelow");
            context = Some("Editor");
        }
        "deleteLeft" => {
            action = ActionType::String("editor::Backspace");
            context = Some("Editor");
        }
        "acceptRenameInput" => {
            action = ActionType::String("editor::ConfirmRename");
            context = Some("Editor && renaming");
        }
        "editor.action.clipboardCopyAction" => {
            action = ActionType::String("editor::Copy");
            context = Some("Editor");
        }
        "editor.action.clipboardCutAction" => {
            action = ActionType::String("editor::Cut");
            context = Some("Editor");
        }
        "deleteRight" => {
            action = ActionType::String("editor::Delete");
            context = Some("Editor");
        }
        "editor.action.deleteLines" => {
            action = ActionType::String("editor::DeleteLine");
            context = Some("Editor");
        }
        "editor.action.copyLinesDownAction" => {
            action = ActionType::String("editor::DuplicateLineDown");
            context = Some("Editor");
        }
        "editor.action.copyLinesUpAction" => {
            action = ActionType::String("editor::DuplicateLineUp");
            context = Some("Editor");
        }
        "references-view.findReferences" => {
            action = ActionType::String("editor::FindAllReferences");
            context = Some("Editor");
        }
        "editor.fold" => {
            action = ActionType::String("editor::Fold");
            context = Some("Editor");
        }
        "editor.foldAll" => {
            action = ActionType::String("editor::FoldAll");
            context = Some("Editor");
        }
        "editor.foldRecursively" => {
            action = ActionType::String("editor::FoldRecursive");
            context = Some("Editor");
        }
        "editor.toggleFold" => {
            action = ActionType::String("editor::ToggleFold");
            context = Some("Editor");
        }
        "editor.action.formatDocument" => {
            action = ActionType::String("editor::Format");
            context = Some("Editor");
        }
        "editor.action.goToDeclaration" => {
            action = ActionType::String("editor::GoToDeclaration");
            context = Some("Editor && !menu");
        }
        "editor.action.revealDefinition" => {
            action = ActionType::String("editor::GoToDefinition");
            context = Some("Editor");
        }
        "editor.action.peekDefinition" => {
            action = ActionType::String("editor::GoToDefinitionSplit");
            context = Some("Editor");
        }
        "editor.action.goToImplementation" => {
            action = ActionType::String("editor::GoToImplementation");
            context = Some("Editor");
        }
        "editor.action.showHover" => {
            action = ActionType::String("editor::Hover");
            context = Some("Editor");
        }
        "editor.action.indentLines" => {
            action = ActionType::String("editor::Indent");
            context = Some("Editor");
        }
        "editor.action.joinLines" => {
            action = ActionType::String("editor::JoinLines");
            context = Some("Editor");
        }
        "deleteAllRight" => {
            action = ActionType::String("editor::KillRingCut");
            context = Some("Editor");
        }
        "scrollLineDown" => {
            action = ActionType::String("editor::LineDown");
            context = Some("Editor");
        }
        "scrollLineUp" => {
            action = ActionType::String("editor::LineUp");
            context = Some("Editor");
        }
        "cursorDown" => {
            action = ActionType::String("editor::MoveDown");
            context = Some("Editor");
        }
        "cursorUp" => {
            action = ActionType::String("editor::MoveUp");
            context = Some("Editor");
        }
        "cursorLeft" => {
            action = ActionType::String("editor::MoveLeft");
            context = Some("Editor");
        }
        "cursorRight" => {
            action = ActionType::String("editor::MoveRight");
            context = Some("Editor");
        }
        "cursorTop" => {
            action = ActionType::String("editor::MoveToBeginning");
            context = Some("Editor");
        }
        "editor.action.jumpToBracket" => {
            action = ActionType::String("editor::MoveToEnclosingBracket");
            context = Some("Editor");
        }
        "cursorBottom" => {
            action = ActionType::String("editor::MoveToEnd");
            context = Some("Editor");
        }
        "cursorWordPartRight" => {
            action = ActionType::String("editor::MoveToNextSubwordEnd");
            context = Some("Editor");
        }
        "cursorWordEndRight" => {
            action = ActionType::String("editor::MoveToNextWordEnd");
            context = Some("Editor");
        }
        "cursorWordPartLeft" => {
            action = ActionType::String("editor::MoveToPreviousSubwordStart");
            context = Some("Editor");
        }
        "cursorWordLeft" => {
            action = ActionType::String("editor::MoveToPreviousWordStart");
            context = Some("Editor");
        }
        "editor.action.insertLineBefore" => {
            action = ActionType::String("editor::NewlineAbove");
            context = Some("Editor && mode == full");
        }
        "editor.action.insertLineAfter" => {
            action = ActionType::String("editor::NewlineBelow");
            context = Some("Editor && mode == full");
        }
        "editor.action.organizeImports" => {
            action = ActionType::String("editor::OrganizeImports");
            context = Some("Editor");
        }
        "editor.action.outdentLines" => {
            action = ActionType::String("editor::Outdent");
            context = Some("Editor");
        }
        "scrollPageDown" => {
            action = ActionType::String("editor::PageDown");
            context = Some("Editor");
        }
        "scrollPageUp" => {
            action = ActionType::String("editor::PageUp");
            context = Some("Editor");
        }
        "editor.action.clipboardPasteAction" => {
            action = ActionType::String("editor::Paste");
            context = Some("Editor");
        }
        "redo" => {
            action = ActionType::String("editor::Redo");
            context = Some("Editor");
        }
        "editor.action.rename" => {
            action = ActionType::String("editor::Rename");
            context = Some("Editor");
        }
        "workbench.action.files.revealActiveFileInWindows" => {
            action = ActionType::String("editor::RevealInFileManager");
            context = Some("Editor");
        }
        "editor.action.selectAll" => {
            action = ActionType::String("editor::SelectAll");
            context = Some("Editor");
        }
        "cursorDownSelect" => {
            action = ActionType::String("editor::SelectDown");
            context = Some("Editor");
        }
        "editor.action.smartSelect.expand" => {
            action = ActionType::String("editor::SelectLargerSyntaxNode");
            context = Some("Editor");
        }
        "cursorLeftSelect" => {
            action = ActionType::String("editor::SelectLeft");
            context = Some("Editor");
        }
        "expandLineSelection" => {
            action = ActionType::String("editor::SelectLine");
            context = Some("Editor");
        }
        "cursorPageDownSelect" => {
            action = ActionType::String("editor::SelectPageDown");
            context = Some("Editor");
        }
        "cursorPageUpSelect" => {
            action = ActionType::String("editor::SelectPageUp");
            context = Some("Editor");
        }
        "cursorRightSelect" => {
            action = ActionType::String("editor::SelectRight");
            context = Some("Editor");
        }
        "editor.action.smartSelect.shrink" => {
            action = ActionType::String("editor::SelectSmallerSyntaxNode");
            context = Some("Editor");
        }
        "cursorTopSelect" => {
            action = ActionType::String("editor::SelectToBeginning");
            context = Some("Editor");
        }
        "cursorBottomSelect" => {
            action = ActionType::String("editor::SelectToEnd");
            context = Some("Editor");
        }
        "cursorWordPartRightSelect" => {
            action = ActionType::String("editor::SelectToNextSubwordEnd");
            context = Some("Editor");
        }
        "cursorWordEndRightSelect" => {
            action = ActionType::String("editor::SelectToNextWordEnd");
            context = Some("Editor");
        }
        "cursorWordPartLeftSelect" => {
            action = ActionType::String("editor::SelectToPreviousSubwordStart");
            context = Some("Editor");
        }
        "cursorWordLeftSelect" => {
            action = ActionType::String("editor::SelectToPreviousWordStart");
            context = Some("Editor");
        }
        "cursorUpSelect" => {
            action = ActionType::String("editor::SelectUp");
            context = Some("Editor");
        }
        "tab" => {
            action = ActionType::String("editor::Tab");
            context = Some("Editor");
        }
        "outdent" => {
            action = ActionType::String("editor::Backtab");
            context = Some("Editor");
        }
        "editor.debug.action.toggleBreakpoint" => {
            action = ActionType::String("editor::ToggleBreakpoint");
            context = Some("Editor");
        }
        "editor.action.transposeLetters" => {
            action = ActionType::String("editor::Transpose");
            context = Some("Editor");
        }
        "undo" => {
            action = ActionType::String("editor::Undo");
            context = Some("Editor");
        }
        "cursorUndo" => {
            action = ActionType::String("editor::UndoSelection");
            context = Some("Editor");
        }
        "editor.unfoldAll" => {
            action = ActionType::String("editor::UnfoldAll");
            context = Some("Editor");
        }
        "editor.unfold" => {
            action = ActionType::String("editor::UnfoldLines");
            context = Some("Editor");
        }
        "editor.unfoldRecursively" => {
            action = ActionType::String("editor::UnfoldRecursive");
            context = Some("Editor");
        }

        _ => return None,
    }
    Some((action, context))
}

#[cfg(test)]
mod tests {
    use gpui::TestKeyboardMapper;

    use crate::KeymapFile;

    use super::{VsCodeShortcuts, vscode_shortcut_command_to_zed_action};

    fn collect_bindings(keymap: &KeymapFile) -> Vec<String> {
        let mut result = Vec::new();
        for section in keymap.sections() {
            for binding in section.bindings() {
                result.push(binding.0.clone());
            }
        }
        result
    }

    #[test]
    fn test_load_vscode_shortcuts() {
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
        let (keymap, skipped) = shortcuts.parse_shortcuts(&keyboard_mapper);
        let bindings = collect_bindings(&keymap);
        assert_eq!(skipped.len(), 0);
        assert_eq!(
            bindings,
            vec![
                "ctrl-[bracketleft]",
                "shift-[bracketright]",
                "ctrl-alt-_",
                "$",
                "shift-oem_3"
            ]
        );
        // assert_eq!(
        //     skipped[0],
        //     (
        //         "ctrl+shift+alt+[张小白]".to_string(),
        //         "Unable to parse keystroke".to_string()
        //     )
        // );

        let content = r#"
        [
            {
                "key": "ctrl+shift+a",
                "command": "list.focusFirst", // we are unable to check whether this is a valid command
            },
            {
                "key": "ctrl+shift+=",
                "command": "menu::SelectFirst",
            }
        ]
        "#;
        let shortcuts = VsCodeShortcuts::from_str(content).unwrap();
        assert_eq!(shortcuts.content.len(), 2);
        let (keymap, skipped) = shortcuts.parse_shortcuts(&keyboard_mapper);
        assert_eq!(skipped.len(), 0);
    }
}
