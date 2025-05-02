use anyhow::Result;
use collections::IndexMap;
use fs::Fs;
use gpui::{AsyncWindowContext, Keystroke, PlatformKeyboardMapper, is_alphabetic_key};
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
                .map(|keystroke| keystroke.to_gpui_style(keyboard_mapper))
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
            // TODO: vscode_shortcut_command_to_zed_action
            let Ok(action) = serde_json_lenient::from_str(&format!(r#""{}""#, command)) else {
                skipped.push((
                    shortcut.to_string(),
                    format!("Unable to parse command: {}", command),
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

fn vscode_shortcut_command_to_zed_action(
    command: &str,
    when: Option<&str>,
) -> Option<(String, Option<String>)> {
    let mut context = None;
    let action = match command {
        "list.focusFirst" | "list.focusAnyFirst" => {
            context = Some("menu".to_string());
            "menu::SelectFirst"
        }
        "list.focusLast" | "list.focusAnyLast" => {
            context = Some("menu".to_string());
            "menu::SelectLast"
        }
        "list.focusUp" | "list.focusAnyUp" => {
            context = Some("menu".to_string());
            "menu::SelectPrevious"
        }
        "list.focusDown" | "list.focusAnyDown" => {
            context = Some("menu".to_string());
            "menu::SelectNext"
        }
        "list.select" => {
            context = Some("menu".to_string());
            "menu::Confirm"
        }
        "list.clear" => {
            context = Some("menu".to_string());
            "menu::Cancel"
        }
        // menu::SecondaryConfirm, Restart
        _ => return None,
    };
    Some((action.to_string(), context))
}

#[cfg(test)]
mod tests {
    use gpui::TestKeyboardMapper;

    use crate::KeymapFile;

    use super::VsCodeShortcuts;

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
            }
        ]
        "#;
        let shortcuts = VsCodeShortcuts::from_str(content).unwrap();
        assert_eq!(shortcuts.content.len(), 4);
        let (keymap, skipped) = shortcuts.parse_shortcuts(&keyboard_mapper);
        let bindings = collect_bindings(&keymap);
        assert_eq!(skipped.len(), 0);
        assert_eq!(
            bindings,
            vec![
                "ctrl-[bracketleft]",
                "shift-[bracketright]",
                "ctrl-alt-_",
                "$"
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
