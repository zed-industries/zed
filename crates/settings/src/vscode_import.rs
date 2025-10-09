use anyhow::{Context as _, Result, anyhow};
use fs::Fs;
use paths::{cursor_settings_file_paths, vscode_settings_file_paths};
use serde_json::{Map, Value};
use std::{path::Path, sync::Arc};

use crate::FontFamilyName;

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
    pub path: Arc<Path>,
    content: Map<String, Value>,
}

impl VsCodeSettings {
    #[cfg(any(test, feature = "test-support"))]
    pub fn from_str(content: &str, source: VsCodeSettingsSource) -> Result<Self> {
        Ok(Self {
            source,
            path: Path::new("/example-path/Code/User/settings.json").into(),
            content: serde_json_lenient::from_str(content)?,
        })
    }

    pub async fn load_user_settings(source: VsCodeSettingsSource, fs: Arc<dyn Fs>) -> Result<Self> {
        let candidate_paths = match source {
            VsCodeSettingsSource::VsCode => vscode_settings_file_paths(),
            VsCodeSettingsSource::Cursor => cursor_settings_file_paths(),
        };
        let mut path = None;
        for candidate_path in candidate_paths.iter() {
            if fs.is_file(candidate_path).await {
                path = Some(candidate_path.clone());
            }
        }
        let Some(path) = path else {
            return Err(anyhow!(
                "No settings file found, expected to find it in one of the following paths:\n{}",
                candidate_paths
                    .into_iter()
                    .map(|path| path.to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("\n")
            ));
        };
        let content = fs.load(&path).await.with_context(|| {
            format!(
                "Error loading {} settings file from {}",
                source,
                path.display()
            )
        })?;
        let content = serde_json_lenient::from_str(&content).with_context(|| {
            format!(
                "Error parsing {} settings file from {}",
                source,
                path.display()
            )
        })?;
        Ok(Self {
            source,
            path: path.into(),
            content,
        })
    }

    pub fn read_value(&self, setting: &str) -> Option<&Value> {
        self.content.get(setting)
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

    pub fn from_f32_setting<T: From<f32>>(&self, key: &str, setting: &mut Option<T>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_f64) {
            *setting = Some(T::from(s as f32))
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

    pub fn read_enum<T>(&self, key: &str, f: impl FnOnce(&str) -> Option<T>) -> Option<T> {
        self.content.get(key).and_then(Value::as_str).and_then(f)
    }

    pub fn font_family_setting(
        &self,
        key: &str,
        font_family: &mut Option<FontFamilyName>,
        font_fallbacks: &mut Option<Vec<FontFamilyName>>,
    ) {
        let Some(css_name) = self.content.get(key).and_then(Value::as_str) else {
            return;
        };

        let mut name_buffer = String::new();
        let mut quote_char: Option<char> = None;
        let mut fonts = Vec::new();
        let mut add_font = |buffer: &mut String| {
            let trimmed = buffer.trim();
            if !trimmed.is_empty() {
                fonts.push(trimmed.to_string().into());
            }

            buffer.clear();
        };

        for ch in css_name.chars() {
            match (ch, quote_char) {
                ('"' | '\'', None) => {
                    quote_char = Some(ch);
                }
                (_, Some(q)) if ch == q => {
                    quote_char = None;
                }
                (',', None) => {
                    add_font(&mut name_buffer);
                }
                _ => {
                    name_buffer.push(ch);
                }
            }
        }

        add_font(&mut name_buffer);

        let mut iter = fonts.into_iter();
        *font_family = iter.next();
        let fallbacks: Vec<_> = iter.collect();
        if !fallbacks.is_empty() {
            *font_fallbacks = Some(fallbacks);
        }
    }
}
