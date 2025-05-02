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
