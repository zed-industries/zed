use alacritty_terminal::vte::ansi::{
    CursorShape as AlacCursorShape, CursorStyle as AlacCursorStyle,
};
use collections::HashMap;
use gpui::{
    AbsoluteLength, App, FontFallbacks, FontFeatures, FontWeight, Pixels, SharedString, px,
};
use schemars::{JsonSchema, r#gen::SchemaGenerator, schema::RootSchema};
use serde_derive::{Deserialize, Serialize};
use settings::{SettingsJsonSchemaParams, SettingsSources, add_references_to_properties};
use std::path::PathBuf;
use task::Shell;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TerminalDockPosition {
    Left,
    Bottom,
    Right,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Toolbar {
    pub breadcrumbs: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TerminalSettings {
    pub shell: Shell,
    pub working_directory: WorkingDirectory,
    pub font_size: Option<Pixels>,
    pub font_family: Option<SharedString>,
    pub font_fallbacks: Option<FontFallbacks>,
    pub font_features: Option<FontFeatures>,
    pub font_weight: Option<FontWeight>,
    pub line_height: TerminalLineHeight,
    pub env: HashMap<String, String>,
    pub cursor_shape: Option<CursorShape>,
    pub blinking: TerminalBlink,
    pub alternate_scroll: AlternateScroll,
    pub option_as_meta: bool,
    pub copy_on_select: bool,
    pub button: bool,
    pub dock: TerminalDockPosition,
    pub default_width: Pixels,
    pub default_height: Pixels,
    pub detect_venv: VenvSettings,
    pub max_scroll_history_lines: Option<usize>,
    pub toolbar: Toolbar,
    pub scrollbar: ScrollbarSettings,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarSettings {
    /// When to show the scrollbar in the terminal.
    ///
    /// Default: inherits editor scrollbar settings
    pub show: Option<ShowScrollbar>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarSettingsContent {
    /// When to show the scrollbar in the terminal.
    ///
    /// Default: inherits editor scrollbar settings
    pub show: Option<Option<ShowScrollbar>>,
}

/// When to show the scrollbar in the terminal.
///
/// Default: auto
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ShowScrollbar {
    /// Show the scrollbar if there's important information or
    /// follow the system's configured behavior.
    Auto,
    /// Match the system's configured behavior.
    System,
    /// Always show the scrollbar.
    Always,
    /// Never show the scrollbar.
    Never,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum VenvSettings {
    #[default]
    Off,
    On {
        /// Default directories to search for virtual environments, relative
        /// to the current working directory. We recommend overriding this
        /// in your project's settings, rather than globally.
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
    PowerShell,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct TerminalSettingsContent {
    /// What shell to use when opening a terminal.
    ///
    /// Default: system
    pub shell: Option<Shell>,
    /// What working directory to use when launching the terminal
    ///
    /// Default: current_project_directory
    pub working_directory: Option<WorkingDirectory>,
    /// Sets the terminal's font size.
    ///
    /// If this option is not included,
    /// the terminal will default to matching the buffer's font size.
    pub font_size: Option<f32>,
    /// Sets the terminal's font family.
    ///
    /// If this option is not included,
    /// the terminal will default to matching the buffer's font family.
    pub font_family: Option<String>,

    /// Sets the terminal's font fallbacks.
    ///
    /// If this option is not included,
    /// the terminal will default to matching the buffer's font fallbacks.
    pub font_fallbacks: Option<Vec<String>>,

    /// Sets the terminal's line height.
    ///
    /// Default: comfortable
    pub line_height: Option<TerminalLineHeight>,
    pub font_features: Option<FontFeatures>,
    /// Sets the terminal's font weight in CSS weight units 0-900.
    pub font_weight: Option<f32>,
    /// Any key-value pairs added to this list will be added to the terminal's
    /// environment. Use `:` to separate multiple values.
    ///
    /// Default: {}
    pub env: Option<HashMap<String, String>>,
    /// Default cursor shape for the terminal.
    /// Can be "bar", "block", "underline", or "hollow".
    ///
    /// Default: None
    pub cursor_shape: Option<CursorShape>,
    /// Sets the cursor blinking behavior in the terminal.
    ///
    /// Default: terminal_controlled
    pub blinking: Option<TerminalBlink>,
    /// Sets whether Alternate Scroll mode (code: ?1007) is active by default.
    /// Alternate Scroll mode converts mouse scroll events into up / down key
    /// presses when in the alternate screen (e.g. when running applications
    /// like vim or  less). The terminal can still set and unset this mode.
    ///
    /// Default: on
    pub alternate_scroll: Option<AlternateScroll>,
    /// Sets whether the option key behaves as the meta key.
    ///
    /// Default: false
    pub option_as_meta: Option<bool>,
    /// Whether or not selecting text in the terminal will automatically
    /// copy to the system clipboard.
    ///
    /// Default: false
    pub copy_on_select: Option<bool>,
    /// Whether to show the terminal button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    pub dock: Option<TerminalDockPosition>,
    /// Default width when the terminal is docked to the left or right.
    ///
    /// Default: 640
    pub default_width: Option<f32>,
    /// Default height when the terminal is docked to the bottom.
    ///
    /// Default: 320
    pub default_height: Option<f32>,
    /// Activates the python virtual environment, if one is found, in the
    /// terminal's working directory (as resolved by the working_directory
    /// setting). Set this to "off" to disable this behavior.
    ///
    /// Default: on
    pub detect_venv: Option<VenvSettings>,
    /// The maximum number of lines to keep in the scrollback history.
    /// Maximum allowed value is 100_000, all values above that will be treated as 100_000.
    /// 0 disables the scrolling.
    /// Existing terminals will not pick up this change until they are recreated.
    /// See <a href="https://github.com/alacritty/alacritty/blob/cb3a79dbf6472740daca8440d5166c1d4af5029e/extra/man/alacritty.5.scd?plain=1#L207-L213">Alacritty documentation</a> for more information.
    ///
    /// Default: 10_000
    pub max_scroll_history_lines: Option<usize>,
    /// Toolbar related settings
    pub toolbar: Option<ToolbarContent>,
    /// Scrollbar-related settings
    pub scrollbar: Option<ScrollbarSettingsContent>,
}

impl settings::Settings for TerminalSettings {
    const KEY: Option<&'static str> = Some("terminal");

    type FileContent = TerminalSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> anyhow::Result<Self> {
        sources.json_merge()
    }

    fn json_schema(
        generator: &mut SchemaGenerator,
        params: &SettingsJsonSchemaParams,
        _: &App,
    ) -> RootSchema {
        let mut root_schema = generator.root_schema_for::<Self::FileContent>();
        root_schema.definitions.extend([
            ("FontFamilies".into(), params.font_family_schema()),
            ("FontFallbacks".into(), params.font_fallback_schema()),
        ]);

        add_references_to_properties(
            &mut root_schema,
            &[
                ("font_family", "#/definitions/FontFamilies"),
                ("font_fallbacks", "#/definitions/FontFallbacks"),
            ],
        );

        root_schema
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut Self::FileContent) {
        let name = |s| format!("terminal.integrated.{s}");

        vscode.f32_setting(&name("fontSize"), &mut current.font_size);
        vscode.string_setting(&name("fontFamily"), &mut current.font_family);
        vscode.bool_setting(&name("copyOnSelection"), &mut current.copy_on_select);
        vscode.bool_setting("macOptionIsMeta", &mut current.option_as_meta);
        vscode.usize_setting("scrollback", &mut current.max_scroll_history_lines);
        match vscode.read_bool(&name("cursorBlinking")) {
            Some(true) => current.blinking = Some(TerminalBlink::On),
            Some(false) => current.blinking = Some(TerminalBlink::Off),
            None => {}
        }
        vscode.enum_setting(&name("cursorStyle"), &mut current.cursor_shape, |s| match s {
            "block" => Some(CursorShape::Block),
            "line" => Some(CursorShape::Bar),
            "underline" => Some(CursorShape::Underline),
            _ => None,
        });
        // they also have "none" and "outline" as options but just for the "Inactive" variant
        if let Some(height) = vscode
            .read_value(&name("lineHeight"))
            .and_then(|v| v.as_f64())
        {
            current.line_height = Some(TerminalLineHeight::Custom(height as f32))
        }

        #[cfg(target_os = "windows")]
        let platform = "windows";
        #[cfg(target_os = "linux")]
        let platform = "linux";
        #[cfg(target_os = "macos")]
        let platform = "osx";

        // TODO: handle arguments
        let shell_name = format!("{platform}Exec");
        if let Some(s) = vscode.read_string(&name(&shell_name)) {
            current.shell = Some(Shell::Program(s.to_owned()))
        }

        if let Some(env) = vscode
            .read_value(&name(&format!("env.{platform}")))
            .and_then(|v| v.as_object())
        {
            for (k, v) in env {
                if v.is_null() {
                    if let Some(zed_env) = current.env.as_mut() {
                        zed_env.remove(k);
                    }
                }
                let Some(v) = v.as_str() else { continue };
                if let Some(zed_env) = current.env.as_mut() {
                    zed_env.insert(k.clone(), v.to_owned());
                } else {
                    current.env = Some([(k.clone(), v.to_owned())].into_iter().collect())
                }
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum TerminalLineHeight {
    /// Use a line height that's comfortable for reading, 1.618
    #[default]
    Comfortable,
    /// Use a standard line height, 1.3. This option is useful for TUIs,
    /// particularly if they use box characters
    Standard,
    /// Use a custom line height.
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TerminalBlink {
    /// Never blink the cursor, ignoring the terminal mode.
    Off,
    /// Default the cursor blink to off, but allow the terminal to
    /// set blinking.
    TerminalControlled,
    /// Always blink the cursor, ignoring the terminal mode.
    On,
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
    /// Use the current file's project directory.  Will Fallback to the
    /// first project directory strategy if unsuccessful.
    CurrentProjectDirectory,
    /// Use the first project in this workspace's directory.
    FirstProjectDirectory,
    /// Always use this platform's home directory (if it can be found).
    AlwaysHome,
    /// Always use a specific directory. This value will be shell expanded.
    /// If this path is not a valid directory the terminal will default to
    /// this platform's home directory  (if it can be found).
    Always { directory: String },
}

// Toolbar related settings
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ToolbarContent {
    /// Whether to display the terminal title in breadcrumbs inside the terminal pane.
    /// Only shown if the terminal title is not empty.
    ///
    /// The shell running in the terminal needs to be configured to emit the title.
    /// Example: `echo -e "\e]2;New Title\007";`
    ///
    /// Default: true
    pub breadcrumbs: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CursorShape {
    /// Cursor is a block like `█`.
    #[default]
    Block,
    /// Cursor is an underscore like `_`.
    Underline,
    /// Cursor is a vertical bar like `⎸`.
    Bar,
    /// Cursor is a hollow box like `▯`.
    Hollow,
}

impl From<CursorShape> for AlacCursorShape {
    fn from(value: CursorShape) -> Self {
        match value {
            CursorShape::Block => AlacCursorShape::Block,
            CursorShape::Underline => AlacCursorShape::Underline,
            CursorShape::Bar => AlacCursorShape::Beam,
            CursorShape::Hollow => AlacCursorShape::HollowBlock,
        }
    }
}

impl From<CursorShape> for AlacCursorStyle {
    fn from(value: CursorShape) -> Self {
        AlacCursorStyle {
            shape: value.into(),
            blinking: false,
        }
    }
}
