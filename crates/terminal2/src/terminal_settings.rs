use gpui::{px, AbsoluteLength, AppContext, FontFeatures, Pixels};
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TerminalDockPosition {
    Left,
    Bottom,
    Right,
}

#[derive(Deserialize)]
pub struct TerminalSettings {
    pub shell: Shell,
    pub working_directory: WorkingDirectory,
    pub font_size: Option<Pixels>,
    pub font_family: Option<String>,
    pub line_height: TerminalLineHeight,
    pub font_features: Option<FontFeatures>,
    pub env: HashMap<String, String>,
    pub blinking: TerminalBlink,
    pub alternate_scroll: AlternateScroll,
    pub option_as_meta: bool,
    pub copy_on_select: bool,
    pub dock: TerminalDockPosition,
    pub default_width: Pixels,
    pub default_height: Pixels,
    pub detect_venv: VenvSettings,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum VenvSettings {
    #[default]
    Off,
    On {
        activate_script: Option<ActivateScript>,
        directories: Option<Vec<PathBuf>>,
    },
}

pub struct VenvSettingsContent<'a> {
    pub activate_script: ActivateScript,
    pub directories: &'a [PathBuf],
}

impl VenvSettings {
    pub fn as_option(&self) -> Option<VenvSettingsContent> {
        match self {
            VenvSettings::Off => None,
            VenvSettings::On {
                activate_script,
                directories,
            } => Some(VenvSettingsContent {
                activate_script: activate_script.unwrap_or(ActivateScript::Default),
                directories: directories.as_deref().unwrap_or(&[]),
            }),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActivateScript {
    #[default]
    Default,
    Csh,
    Fish,
    Nushell,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct TerminalSettingsContent {
    pub shell: Option<Shell>,
    pub working_directory: Option<WorkingDirectory>,
    pub font_size: Option<f32>,
    pub font_family: Option<String>,
    pub line_height: Option<TerminalLineHeight>,
    pub font_features: Option<FontFeatures>,
    pub env: Option<HashMap<String, String>>,
    pub blinking: Option<TerminalBlink>,
    pub alternate_scroll: Option<AlternateScroll>,
    pub option_as_meta: Option<bool>,
    pub copy_on_select: Option<bool>,
    pub dock: Option<TerminalDockPosition>,
    pub default_width: Option<f32>,
    pub default_height: Option<f32>,
    pub detect_venv: Option<VenvSettings>,
}

impl settings::Settings for TerminalSettings {
    const KEY: Option<&'static str> = Some("terminal");

    type FileContent = TerminalSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum TerminalLineHeight {
    #[default]
    Comfortable,
    Standard,
    Custom(f32),
}

impl TerminalLineHeight {
    pub fn value(&self) -> AbsoluteLength {
        let value = match self {
            TerminalLineHeight::Comfortable => 1.618,
            TerminalLineHeight::Standard => 1.3,
            TerminalLineHeight::Custom(line_height) => f32::max(*line_height, 1.),
        };
        px(value).into()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TerminalBlink {
    Off,
    TerminalControlled,
    On,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Shell {
    System,
    Program(String),
    WithArguments { program: String, args: Vec<String> },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AlternateScroll {
    On,
    Off,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WorkingDirectory {
    CurrentProjectDirectory,
    FirstProjectDirectory,
    AlwaysHome,
    Always { directory: String },
}
