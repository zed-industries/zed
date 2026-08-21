use std::path::PathBuf;

use collections::HashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

use crate::{FontFamilyName, FontFeaturesContent, FontSize, FontWeightContent};

/// Terminal settings that can be configured per project.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ProjectTerminalSettingsContent {
    /// What shell to use when opening a terminal.
    ///
    /// Default: system
    pub shell: Option<Shell>,
    /// What working directory to use when launching the terminal
    ///
    /// Default: current_project_directory
    pub working_directory: Option<WorkingDirectory>,
    /// Any key-value pairs added to this list will be added to the terminal's
    /// environment. Use `:` to separate multiple values.
    ///
    /// Default: {}
    pub env: Option<HashMap<String, String>>,
    /// Activates the python virtual environment, if one is found, in the
    /// terminal's working directory (as resolved by the working_directory
    /// setting). Set this to "off" to disable this behavior.
    ///
    /// Default: on
    pub detect_venv: Option<VenvSettings>,
    /// Regexes used to identify paths for hyperlink navigation.
    ///
    /// Default: [
    ///   // Python-style diagnostics
    ///   "File \"(?<path>[^\"]+)\", line (?<line>[0-9]+)",
    ///   // Common path syntax with optional line, column, description, trailing punctuation, or
    ///   // surrounding symbols or quotes
    ///   [
    ///     "(?x)",
    ///     "# optionally starts with 0-2 opening prefix symbols",
    ///     "[({\\[<]{0,2}",
    ///     "# which may be followed by an opening quote",
    ///     "(?<quote>[\"'`])?",
    ///     "# `path` is the shortest sequence of any non-space character",
    ///     "(?<link>(?<path>[^ ]+?",
    ///     "    # which may end with a line and optionally a column,",
    ///     "    (?<line_column>:+[0-9]+(:[0-9]+)?|:?\\([0-9]+([,:][0-9]+)?\\))?",
    ///     "))",
    ///     "# which must be followed by a matching quote",
    ///     "(?(<quote>)\\k<quote>)",
    ///     "# and optionally a single closing symbol",
    ///     "[)}\\]>]?",
    ///     "# if line/column matched, may be followed by a description",
    ///     "(?(<line_column>):[^ 0-9][^ ]*)?",
    ///     "# which may be followed by trailing punctuation",
    ///     "[.,:)}\\]>]*",
    ///     "# and always includes trailing whitespace or end of line",
    ///     "([ ]+|$)"
    ///   ]
    /// ]
    pub path_hyperlink_regexes: Option<Vec<PathHyperlinkRegex>>,
    /// Timeout for hover and Cmd-click path hyperlink discovery in milliseconds.
    ///
    /// Default: 1
    pub path_hyperlink_timeout_ms: Option<u64>,
}

/// Configuration for the terminal.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct TerminalSettingsContent {
    /// Terminal settings that can be configured per project.
    #[serde(flatten)]
    pub project: ProjectTerminalSettingsContent,
    /// Sets the terminal's font size.
    ///
    /// If this option is not included,
    /// the terminal will default to matching the buffer's font size.
    pub font_size: Option<FontSize>,
    /// Sets the terminal's font family.
    ///
    /// If this option is not included,
    /// the terminal will default to matching the buffer's font family.
    pub font_family: Option<FontFamilyName>,

    /// Sets the terminal's font fallbacks.
    ///
    /// If this option is not included,
    /// the terminal will default to matching the buffer's font fallbacks.
    #[schemars(extend("uniqueItems" = true))]
    pub font_fallbacks: Option<Vec<FontFamilyName>>,

    /// Sets the terminal's line height.
    ///
    /// Default: comfortable
    pub line_height: Option<TerminalLineHeight>,
    /// What font features to use for the terminal.
    /// When not set, defaults to matching the editor's font features.
    pub font_features: Option<FontFeaturesContent>,
    /// Sets the terminal's font weight in CSS weight units 0-900.
    pub font_weight: Option<FontWeightContent>,
    /// Default cursor shape for the terminal.
    /// Can be "bar", "block", "underline", or "hollow".
    ///
    /// Default: "block"
    pub cursor_shape: Option<CursorShapeContent>,
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
    /// Whether to keep the text selection after copying it to the clipboard.
    ///
    /// Default: true
    pub keep_selection_on_copy: Option<bool>,
    /// Whether cmd-click (ctrl-click on Linux and Windows) opens hyperlinks even
    /// when the terminal application has enabled mouse reporting (e.g. vim with
    /// mouse=a, htop). When false, these clicks are forwarded to the application
    /// instead, and hyperlinks can still be opened with shift-cmd-click
    /// (shift-ctrl-click).
    ///
    /// Default: true
    pub open_links_in_mouse_mode: Option<bool>,
    /// Whether to show the terminal button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Where to dock terminals panel. Can be `left`, `right`, `bottom`.
    ///
    /// Default: bottom
    pub dock: Option<TerminalDockPosition>,
    /// Whether the terminal panel should open on startup.
    ///
    /// Default: false
    pub starts_open: Option<bool>,
    /// Whether the terminal panel should use flexible (proportional) sizing.
    ///
    /// Default: true
    pub flexible: Option<bool>,
    /// Default width when the terminal is docked to the left or right.
    ///
    /// Default: 640
    pub default_width: Option<crate::PixelSetting>,
    /// Default height when the terminal is docked to the bottom.
    ///
    /// Default: 320
    pub default_height: Option<crate::PixelSetting>,
    /// The maximum number of lines to keep in the scrollback history.
    /// Maximum allowed value is 100_000, all values above that will be treated as 100_000.
    /// 0 disables the scrolling.
    /// Existing terminals will not pick up this change until they are recreated.
    /// See <a href="https://github.com/alacritty/alacritty/blob/cb3a79dbf6472740daca8440d5166c1d4af5029e/extra/man/alacritty.5.scd?plain=1#L207-L213">Alacritty documentation</a> for more information.
    ///
    /// Default: 10_000
    pub max_scroll_history_lines: Option<usize>,
    /// The multiplier for scrolling with the mouse wheel.
    ///
    /// Default: 1.0
    pub scroll_multiplier: Option<f32>,
    /// Toolbar related settings
    pub toolbar: Option<TerminalToolbarContent>,
    /// Scrollbar-related settings
    pub scrollbar: Option<ScrollbarSettingsContent>,
    /// The minimum APCA perceptual contrast between foreground and background colors.
    ///
    /// APCA (Accessible Perceptual Contrast Algorithm) is more accurate than WCAG 2.x,
    /// especially for dark mode. Values range from 0 to 106.
    ///
    /// Based on APCA Readability Criterion (ARC) Bronze Simple Mode:
    /// https://readtech.org/ARC/tests/bronze-simple-mode/
    /// - 0: No contrast adjustment
    /// - 45: Minimum for large fluent text (36px+)
    /// - 60: Minimum for other content text
    /// - 75: Minimum for body text
    /// - 90: Preferred for body text
    ///
    /// Default: 45
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub minimum_contrast: Option<f32>,
    /// Whether to show a badge on the terminal panel icon with the count of open terminals.
    ///
    /// Default: false
    pub show_count_badge: Option<bool>,
    /// What to do when the `BEL` character (`\a`) is printed to terminal.
    ///
    /// Default: "system"
    pub bell: Option<TerminalBell>,
}

/// Shell configuration to open the terminal with.
#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::EnumDiscriminants,
)]
#[strum_discriminants(derive(strum::VariantArray, strum::VariantNames, strum::FromRepr))]
#[strum_discriminants(doc = "The kind of a shell configuration.")]
#[serde(rename_all = "snake_case")]
pub enum Shell {
    /// Use the system's default terminal configuration in /etc/passwd
    #[default]
    System,
    /// Use a specific program with no arguments.
    Program(String),
    /// Use a specific program with arguments.
    WithArguments {
        /// The program to run.
        program: String,
        /// The arguments to pass to the program.
        args: Vec<String>,
        /// An optional string to override the title of the terminal tab
        title_override: Option<String>,
    },
}

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::EnumDiscriminants,
)]
#[strum_discriminants(derive(strum::VariantArray, strum::VariantNames, strum::FromRepr))]
#[strum_discriminants(doc = "The kind of a working directory strategy.")]
#[serde(rename_all = "snake_case")]
/// What working directory to use when launching the terminal.
pub enum WorkingDirectory {
    /// Use the current file's directory, falling back to the project directory,
    /// then the first project in the workspace.
    CurrentFileDirectory,
    /// Use the current file's project directory. Fallback to the
    /// first project directory strategy if unsuccessful.
    CurrentProjectDirectory,
    /// Use the first project in this workspace's directory. Fallback to using
    /// this platform's home directory.
    FirstProjectDirectory,
    /// Always use this platform's home directory (if it can be found).
    AlwaysHome,
    /// Always use a specific directory. This value will be shell expanded.
    /// If this path is not a valid directory the terminal will default to
    /// this platform's home directory  (if it can be found).
    Always {
        /// The directory to use.
        directory: String,
    },
}

#[with_fallible_options]
#[derive(
    Clone, Copy, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq, Default,
)]
/// Scrollbar-related settings for the terminal.
pub struct ScrollbarSettingsContent {
    /// When to show the scrollbar in the terminal.
    ///
    /// Default: inherits editor scrollbar settings
    pub show: Option<ShowScrollbar>,
}

/// The terminal's line height.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom, Default)]
#[serde(rename_all = "snake_case")]
pub enum TerminalLineHeight {
    /// Use a line height that's comfortable for reading, 1.618
    #[default]
    Comfortable,
    /// Use a standard line height, 1.3. This option is useful for TUIs,
    /// particularly if they use box characters
    Standard,
    /// Use a custom line height.
    Custom(#[serde(serialize_with = "crate::serialize_f32_with_two_decimal_places")] f32),
}

impl TerminalLineHeight {
    /// Returns the line height as a multiplier of the font size.
    pub fn value(&self) -> f32 {
        match self {
            TerminalLineHeight::Comfortable => 1.618,
            TerminalLineHeight::Standard => 1.3,
            TerminalLineHeight::Custom(line_height) => f32::max(*line_height, 1.),
        }
    }
}

/// When to show the scrollbar.
///
/// Default: auto
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum ShowScrollbar {
    /// Show the scrollbar if there's important information or
    /// follow the system's configured behavior.
    #[default]
    Auto,
    /// Match the system's configured behavior.
    System,
    /// Always show the scrollbar.
    Always,
    /// Never show the scrollbar.
    Never,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
// todo() -> combine with CursorShape
/// The shape of the terminal cursor.
pub enum CursorShapeContent {
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

#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
/// The cursor blinking behavior in the terminal.
pub enum TerminalBlink {
    /// Never blink the cursor, ignoring the terminal mode.
    Off,
    /// Default the cursor blink to off, but allow the terminal to
    /// set blinking.
    TerminalControlled,
    /// Always blink the cursor, ignoring the terminal mode.
    On,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
/// Whether Alternate Scroll mode (code: ?1007) is active by default.
pub enum AlternateScroll {
    /// Default alternate scroll mode to on.
    On,
    /// Default alternate scroll mode to off.
    Off,
}

/// Toolbar related settings
#[with_fallible_options]
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq)]
pub struct TerminalToolbarContent {
    /// Whether to display the terminal title in breadcrumbs inside the terminal pane.
    /// Only shown if the terminal title is not empty.
    ///
    /// The shell running in the terminal needs to be configured to emit the title.
    /// Example: `echo -e "\e]2;New Title\007";`
    ///
    /// Default: true
    pub breadcrumbs: Option<bool>,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
/// What to do when the `BEL` character (`\a`) is printed to terminal.
pub enum TerminalBell {
    /// Play an OS-specific alert sound.
    #[default]
    System,
    /// Do not play any sound.
    Off,
}

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom,
)]
#[serde(rename_all = "snake_case")]
/// The Conda manager to use when activating Conda environments.
pub enum CondaManager {
    /// Automatically detect the conda manager
    #[default]
    Auto,
    /// Use conda
    Conda,
    /// Use mamba
    Mamba,
    /// Use micromamba
    Micromamba,
}

/// Whether to detect and activate Python virtual environments in the
/// terminal's working directory.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum VenvSettings {
    /// Do not detect virtual environments.
    #[default]
    Off,
    /// Detect virtual environments and activate them in the terminal.
    On {
        /// The activation script flavor to use when activating the virtual
        /// environment. Can also be `csh`, `fish`, `nushell` and `power_shell`.
        ///
        /// Default: default
        activate_script: Option<ActivateScript>,
        /// The name of the virtual environment.
        venv_name: Option<String>,
        /// Default directories to search for virtual environments, relative
        /// to the current working directory. We recommend overriding this
        /// in your project's settings, rather than globally.
        directories: Option<Vec<PathBuf>>,
        /// Preferred Conda manager to use when activating Conda environments.
        ///
        /// Default: auto
        conda_manager: Option<CondaManager>,
    },
}
/// Resolved virtual environment detection settings, with defaults applied.
#[with_fallible_options]
pub struct VenvSettingsContent<'a> {
    /// The activation script flavor to use.
    pub activate_script: ActivateScript,
    /// The name of the virtual environment.
    pub venv_name: &'a str,
    /// Directories to search for virtual environments.
    pub directories: &'a [PathBuf],
    /// The Conda manager to use when activating Conda environments.
    pub conda_manager: CondaManager,
}

impl VenvSettings {
    /// Returns the resolved settings when detection is enabled, or `None` when off.
    pub fn as_option(&self) -> Option<VenvSettingsContent<'_>> {
        match self {
            VenvSettings::Off => None,
            VenvSettings::On {
                activate_script,
                venv_name,
                directories,
                conda_manager,
            } => Some(VenvSettingsContent {
                activate_script: activate_script.unwrap_or(ActivateScript::Default),
                venv_name: venv_name.as_deref().unwrap_or(""),
                directories: directories.as_deref().unwrap_or(&[]),
                conda_manager: conda_manager.unwrap_or(CondaManager::Auto),
            }),
        }
    }
}

/// A regex used to identify path hyperlinks. The regexes can be specified in
/// two forms - a single regex string, or an array of strings (which will be
/// collected into a single multi-line regex string).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
#[serde(untagged)]
pub enum PathHyperlinkRegex {
    /// A single regex string.
    SingleLine(String),
    /// An array of strings which will be collected into a single multi-line
    /// regex string.
    MultiLine(Vec<String>),
}

#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
/// Where to dock the terminal panel.
pub enum TerminalDockPosition {
    /// Dock the terminal panel to the left.
    Left,
    /// Dock the terminal panel to the bottom.
    Bottom,
    /// Dock the terminal panel to the right.
    Right,
}

/// The activation script flavor to use when activating a virtual environment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum ActivateScript {
    /// Use the default activation script.
    #[default]
    Default,
    /// Use the activation script for the C shell.
    Csh,
    /// Use the activation script for the Fish shell.
    Fish,
    /// Use the activation script for Nushell.
    Nushell,
    /// Use the activation script for PowerShell.
    PowerShell,
    /// Use the activation script for pyenv.
    Pyenv,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::{ProjectSettingsContent, Shell};

    #[test]
    #[ignore]
    fn test_project_settings() {
        let project_content =
            json!({"terminal": {"shell": {"program": "/bin/project"}}, "option_as_meta": true});

        let _user_content =
            json!({"terminal": {"shell": {"program": "/bin/user"}}, "option_as_meta": false});

        let project_settings =
            serde_json::from_value::<ProjectSettingsContent>(project_content).unwrap();

        assert_eq!(
            project_settings.terminal.unwrap().shell,
            Some(Shell::Program("/bin/project".to_owned()))
        );
    }
}

impl ProjectTerminalSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            shell: Some(Shell::System),
            working_directory: Some(WorkingDirectory::CurrentProjectDirectory),
            env: Some(HashMap::default()),
            detect_venv: Some(VenvSettings::On {
                activate_script: Some(ActivateScript::Default),
                venv_name: None,
                directories: Some(vec![
                    PathBuf::from(".env"),
                    PathBuf::from("env"),
                    PathBuf::from(".venv"),
                    PathBuf::from("venv"),
                ]),
                conda_manager: Some(CondaManager::Auto),
            }),
            path_hyperlink_regexes: Some(vec![
                PathHyperlinkRegex::SingleLine(String::from(
                    r#"File "(?<path>[^"]+)", line (?<line>[0-9]+)"#,
                )),
                PathHyperlinkRegex::MultiLine(vec![
                    String::from("(?x)"),
                    String::from("(?<path>"),
                    String::from("    ("),
                    String::from(
                        "        # multi-char path: first char (not opening delimiter, space, or box drawing char)",
                    ),
                    String::from(r#"        [^({\[<"'`\ \u2500-\u257F]"#),
                    String::from(
                        "        # middle chars: non-space, and colon/paren only if not followed by digit/paren/space",
                    ),
                    String::from(r"        ([^\ :(]|[:(][^0-9()\ ])*"),
                    String::from("        # last char: not closing delimiter or colon"),
                    String::from(r#"        [^()}\]>"'`.,;:\ ]"#),
                    String::from("    |"),
                    String::from(
                        "        # single-char path: not delimiter, punctuation, space, or box drawing char",
                    ),
                    String::from(r#"        [^(){}\[\]<>"'`.,;:\ \u2500-\u257F]"#),
                    String::from("    )"),
                    String::from(
                        "    # optional line/column suffix (included in path for PathWithPosition::parse_str)",
                    ),
                    String::from(r"    (:+[0-9]+(:[0-9]+)?|:?\([0-9]+([,:]?[0-9]+)?\))?"),
                    String::from(")"),
                ]),
            ]),
            path_hyperlink_timeout_ms: Some(1),
        }
    }
}

impl TerminalToolbarContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            breadcrumbs: Some(false),
        }
    }
}

impl ScrollbarSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self { show: None }
    }
}

impl TerminalSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            project: ProjectTerminalSettingsContent::defaults(),
            font_size: None,
            font_family: None,
            font_fallbacks: None,
            line_height: Some(TerminalLineHeight::Standard),
            font_features: None,
            font_weight: Some(FontWeightContent(400.0)),
            cursor_shape: Some(CursorShapeContent::Block),
            blinking: Some(TerminalBlink::TerminalControlled),
            alternate_scroll: Some(AlternateScroll::On),
            option_as_meta: Some(false),
            copy_on_select: Some(false),
            keep_selection_on_copy: Some(true),
            open_links_in_mouse_mode: Some(true),
            button: Some(true),
            dock: Some(TerminalDockPosition::Bottom),
            starts_open: Some(false),
            flexible: Some(true),
            default_width: Some(crate::PixelSetting(640.0)),
            default_height: Some(crate::PixelSetting(320.0)),
            max_scroll_history_lines: Some(10000),
            scroll_multiplier: Some(1.0),
            toolbar: Some(TerminalToolbarContent::defaults()),
            scrollbar: Some(ScrollbarSettingsContent::defaults()),
            minimum_contrast: Some(45.0),
            show_count_badge: Some(false),
            bell: Some(TerminalBell::Off),
        }
    }
}
