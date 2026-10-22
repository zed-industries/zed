use std::path::PathBuf;

use collections::{HashMap, HashSet};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

use crate::{FontFamilyName, FontFeaturesContent, FontSize, FontWeightContent};

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
    /// Default:
    ///
    /// ```json
    /// {
    ///   "terminal": {
    ///     "path_hyperlink_regexes": [
    ///       // Python-style diagnostics
    ///       "File \"(?<path>[^\"]+)\", line (?<line>[0-9]+)",
    ///       // Common path syntax with optional line, column, description, trailing punctuation, or
    ///       // surrounding symbols or quotes
    ///       [
    ///         "(?x)",
    ///         "(?<path>",
    ///         "    (",
    ///         "        # multi-char path: first char (not opening delimiter, space, or box drawing char)",
    ///         "        [^({\\[<\"'`\\ \\u2500-\\u257F]",
    ///         "        # middle chars: non-space, and colon/paren only if not followed by digit/paren/space",
    ///         "        ([^\\ :(]|[:(][^0-9()\\ ])*",
    ///         "        # last char: not closing delimiter or colon",
    ///         "        [^()}\\]>\"'`.,;:\\ ]",
    ///         "    |",
    ///         "        # single-char path: not delimiter, punctuation, space, or box drawing char",
    ///         "        [^(){}\\[\\]<>\"'`.,;:\\ \\u2500-\\u257F]",
    ///         "    )",
    ///         "    # optional line/column suffix (included in path for PathWithPosition::parse_str)",
    ///         "    (:+[0-9]+(:[0-9]+)?|:?\\([0-9]+([,:]?[0-9]+)?\\))?",
    ///         ")"
    ///       ]
    ///     ]
    ///   }
    /// }
    /// ```
    ///
    /// Use `"..."` to add regexes without repeating Zed's defaults. In project
    /// settings, it extends the user or parent configuration value. Omit
    /// `"..."` to replace the inherited list. Set `[]` to clear it. Omitting this
    /// setting keeps the inherited list.
    ///
    /// ```json
    /// {
    ///   "terminal": {
    ///     "path_hyperlink_regexes": [
    ///       "\\s+(-->|:::|at) (?<link>(?<path>.+?))(:$|$)",
    ///       "\\s+(Compiling|Checking|Documenting) [^(]+\\((?<link>(?<path>.+))\\)",
    ///       "..."
    ///     ]
    ///   }
    /// }
    /// ```
    ///
    /// Inherited regexes are inserted at `"..."`, and duplicates keep their first
    /// occurrence. Regexes are duplicates when their text is identical after
    /// joining multiline entries with newlines.
    ///
    /// Each regex can be a single string or an array of strings joined with
    /// newlines. The marker is recognized only as a top-level string. To use
    /// `...` as a regex matching three characters, write `["..."]` as an entry.
    /// A `"..."` line inside a multiline entry is always regex text.
    ///
    /// The optional named capture `path` selects the hyperlink target. Without
    /// it, the entire match is the target. With `path`, `line` and `column`
    /// specify the position. Without a captured `line`, built-in suffix
    /// processing parses `line:column` and `(line,column)` variants. The optional
    /// `link` capture selects the clickable text, otherwise the entire match is
    /// clickable.
    ///
    /// Processing stops at the first regex that matches the terminal line, even
    /// if the cursor is outside its clickable text. Put more specific regexes
    /// before broader ones. Regexes use Rust's `regex` syntax. Invalid regexes
    /// are logged and ignored. The `path_hyperlink_timeout_ms` setting controls
    /// the discovery timeout. Setting it to `0` disables path hyperlinks.
    pub path_hyperlink_regexes: Option<PathHyperlinkRegexes>,
    /// Timeout for hover and Cmd-click path hyperlink discovery in milliseconds.
    ///
    /// Default: 1
    pub path_hyperlink_timeout_ms: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TerminalSettingsProfiles {
    pub defaults: TerminalSettingsContent,
    pub profiles: Option<Vec<TerminalProfileContent>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TerminalProfileContent {
    pub name: String,
    #[serde(flatten)]
    pub settings: TerminalSettingsContent,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
enum TerminalSettingsFormat {
    Profiles(Vec<TerminalProfileContent>),
    Settings(TerminalSettingsContent),
}

impl<'de> Deserialize<'de> for TerminalSettingsProfiles {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match TerminalSettingsFormat::deserialize(deserializer)? {
            TerminalSettingsFormat::Settings(defaults) => Ok(defaults.into()),
            TerminalSettingsFormat::Profiles(profiles) => {
                let mut names = HashSet::default();
                for profile in &profiles {
                    if profile.name.trim().is_empty() {
                        return Err(serde::de::Error::custom(
                            "terminal profile names must not be empty",
                        ));
                    }
                    if !names.insert(&profile.name) {
                        return Err(serde::de::Error::custom(format!(
                            "duplicate terminal profile name: {}",
                            profile.name
                        )));
                    }
                }
                Ok(Self {
                    defaults: TerminalSettingsContent::default(),
                    profiles: Some(profiles),
                })
            }
        }
    }
}

impl Serialize for TerminalSettingsProfiles {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.profiles {
            Some(profiles) => profiles.serialize(serializer),
            None => self.defaults.serialize(serializer),
        }
    }
}

impl JsonSchema for TerminalSettingsProfiles {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "TerminalSettingsProfiles".into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        TerminalSettingsFormat::json_schema(generator)
    }
}

impl From<TerminalSettingsContent> for TerminalSettingsProfiles {
    fn from(defaults: TerminalSettingsContent) -> Self {
        Self {
            defaults,
            profiles: None,
        }
    }
}

impl crate::merge_from::MergeFrom for TerminalSettingsProfiles {
    fn merge_from(&mut self, other: &Self) {
        self.defaults.merge_from(&other.defaults);
        if let Some(profiles) = &other.profiles {
            self.profiles = Some(profiles.clone());
        } else if let Some(profiles) = &mut self.profiles {
            for profile in profiles {
                profile.settings.merge_from(&other.defaults);
            }
        }
    }
}

// The Settings UI edits the default profile, while the settings resolver keeps
// the shared defaults separate so one profile cannot leak into another.
impl std::ops::Deref for TerminalSettingsProfiles {
    type Target = TerminalSettingsContent;

    fn deref(&self) -> &Self::Target {
        self.profiles
            .as_ref()
            .and_then(|profiles| profiles.first())
            .map_or(&self.defaults, |profile| &profile.settings)
    }
}

impl std::ops::DerefMut for TerminalSettingsProfiles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.profiles.as_ref().is_some_and(Vec::is_empty) {
            self.profiles = None;
        }
        self.profiles
            .as_mut()
            .and_then(|profiles| profiles.first_mut())
            .map_or(&mut self.defaults, |profile| &mut profile.settings)
    }
}

#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Default, Serialize, JsonSchema, MergeFrom)]
pub struct TerminalSettingsContent {
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

crate::fallible_options::flattened_deserialize!(TerminalSettingsContent {
    sections: { project },
    options: {
        font_size, font_family, font_fallbacks, line_height, font_features, font_weight,
        cursor_shape, blinking, alternate_scroll, option_as_meta, copy_on_select,
        keep_selection_on_copy, open_links_in_mouse_mode, button, dock, starts_open, flexible,
        default_width, default_height, max_scroll_history_lines, scroll_multiplier, toolbar,
        scrollbar, minimum_contrast, show_count_badge, bell,
    },
    defaults: {},
});

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
#[serde(rename_all = "snake_case")]
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
    Always { directory: String },
}

#[with_fallible_options]
#[derive(
    Clone, Copy, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq, Default,
)]
pub struct ScrollbarSettingsContent {
    /// When to show the scrollbar in the terminal.
    ///
    /// Default: inherits editor scrollbar settings
    pub show: Option<ShowScrollbar>,
}

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
pub enum AlternateScroll {
    On,
    Off,
}

// Toolbar related settings
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum VenvSettings {
    #[default]
    Off,
    On {
        /// Default directories to search for virtual environments, relative
        /// to the current working directory. We recommend overriding this
        /// in your project's settings, rather than globally.
        activate_script: Option<ActivateScript>,
        venv_name: Option<String>,
        directories: Option<Vec<PathBuf>>,
        /// Preferred Conda manager to use when activating Conda environments.
        ///
        /// Default: auto
        conda_manager: Option<CondaManager>,
    },
}
#[with_fallible_options]
pub struct VenvSettingsContent<'a> {
    pub activate_script: ActivateScript,
    pub venv_name: &'a str,
    pub directories: &'a [PathBuf],
    pub conda_manager: CondaManager,
}

impl VenvSettings {
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

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(transparent)]
pub struct PathHyperlinkRegexes(pub Vec<PathHyperlinkRegex>);

impl crate::merge_from::MergeFrom for PathHyperlinkRegexes {
    fn merge_from(&mut self, other: &Self) {
        let inherited = std::mem::take(&mut self.0);
        let mut seen = HashSet::default();
        self.0 = other
            .0
            .iter()
            .flat_map(|entry| {
                if entry.is_rest() {
                    inherited.as_slice()
                } else {
                    std::slice::from_ref(entry)
                }
            })
            .filter(|entry| {
                let regex = match entry {
                    PathHyperlinkRegex::SingleLine(regex) => regex.clone(),
                    PathHyperlinkRegex::MultiLine(lines) => lines.join("\n"),
                };
                // Project layers can still contain an unresolved marker, which must
                // not collapse with the literal regex written as ["..."]
                seen.insert((entry.is_rest(), regex))
            })
            .cloned()
            .collect();
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
#[serde(untagged)]
pub enum PathHyperlinkRegex {
    SingleLine(String),
    MultiLine(Vec<String>),
}

impl PathHyperlinkRegex {
    fn is_rest(&self) -> bool {
        matches!(self, Self::SingleLine(regex) if regex == crate::SplicingVec::REST)
    }
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
pub enum TerminalDockPosition {
    Left,
    Bottom,
    Right,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum ActivateScript {
    #[default]
    Default,
    Csh,
    Fish,
    Nushell,
    PowerShell,
    Pyenv,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::{PathHyperlinkRegexes, ProjectSettingsContent, Shell, merge_from::MergeFrom};

    #[test]
    fn test_terminal_profiles_round_trip() -> anyhow::Result<()> {
        let input = json!([
            {"name": "PowerShell", "shell": {"with_arguments": {
                "program": "pwsh.exe", "args": ["-NoLogo"]
            }}, "font_size": 16},
            {"name": "Ubuntu", "shell": {"with_arguments": {
                "program": "wsl.exe", "args": ["-d", "Ubuntu"]
            }}}
        ]);
        let mut settings: super::TerminalSettingsProfiles = serde_json::from_value(input)?;
        assert_eq!(settings.profiles.as_ref().map(Vec::len), Some(2));
        assert!(
            matches!(&settings.project.shell, Some(Shell::WithArguments { program, args, .. })
            if program == "pwsh.exe" && args == &["-NoLogo"])
        );
        settings.copy_on_select = Some(true);
        let serialized = serde_json::to_value(&settings)?;
        assert!(serialized.is_array());
        assert_eq!(serialized[0]["name"], "PowerShell");
        assert_eq!(serialized[0]["copy_on_select"], true);
        assert!(serialized[1]["copy_on_select"].is_null());
        assert_eq!(
            serde_json::from_value::<super::TerminalSettingsProfiles>(serialized)?,
            settings
        );
        Ok(())
    }

    #[test]
    fn test_terminal_profiles_accept_legacy_and_empty_settings() -> anyhow::Result<()> {
        let mut legacy: super::TerminalSettingsProfiles = serde_json::from_value(json!({
            "shell": {"program": "cmd.exe"}, "font_size": 14
        }))?;
        assert!(legacy.profiles.is_none());
        legacy.copy_on_select = Some(true);
        assert!(serde_json::to_value(&legacy)?.is_object());
        let mut empty: super::TerminalSettingsProfiles = serde_json::from_value(json!([]))?;
        empty.copy_on_select = Some(true);
        assert_eq!(serde_json::to_value(&empty)?["copy_on_select"], true);
        Ok(())
    }

    #[test]
    fn test_terminal_profiles_validate_names() {
        for input in [
            json!([{"shell": "system"}]),
            json!([{"name": ""}]),
            json!([{"name": "  "}]),
            json!([{"name": "Shell"}, {"name": "Shell"}]),
        ] {
            assert!(serde_json::from_value::<super::TerminalSettingsProfiles>(input).is_err());
        }
    }

    #[test]
    fn test_terminal_profiles_merge_without_sharing_profile_overrides() -> anyhow::Result<()> {
        let mut settings: super::TerminalSettingsProfiles = serde_json::from_value(json!({
            "font_size": 14, "env": {"DEFAULT": "1"}
        }))?;
        settings.merge_from(&serde_json::from_value(json!([
            {"name": "First", "font_size": 20, "env": {"FIRST": "1"}},
            {"name": "Second"}
        ]))?);
        assert_eq!(
            serde_json::to_value(&settings.defaults)?["font_size"].as_f64(),
            Some(14.)
        );
        let profiles = settings
            .profiles
            .as_ref()
            .expect("profiles must be preserved");
        assert!(
            profiles
                .get(1)
                .expect("second profile")
                .settings
                .font_size
                .is_none()
        );
        settings.merge_from(&serde_json::from_value(json!({"font_size": 18}))?);
        assert_eq!(
            serde_json::to_value(&settings)?[0]["font_size"].as_f64(),
            Some(18.)
        );
        assert_eq!(
            serde_json::to_value(&settings)?[1]["font_size"].as_f64(),
            Some(18.)
        );
        settings.merge_from(&serde_json::from_value(json!([{"name": "Replacement"}]))?);
        assert_eq!(settings.profiles.as_ref().map(Vec::len), Some(1));
        assert_eq!(serde_json::to_value(&settings)?[0]["name"], "Replacement");
        Ok(())
    }

    #[test]
    fn test_path_hyperlink_regexes_preserve_entry_forms() -> anyhow::Result<()> {
        let entries = json!([
            "...",
            ["..."],
            ["(?x)", "# comment", "...", "(?<path>file)"],
            "first\nsecond",
            ["first", "second"]
        ]);
        let regexes: PathHyperlinkRegexes = serde_json::from_value(entries.clone())?;
        assert_eq!(serde_json::to_value(regexes)?, entries);
        Ok(())
    }

    #[test]
    fn test_path_hyperlink_regexes_merge_preserves_first_representation() -> anyhow::Result<()> {
        let mut regexes: PathHyperlinkRegexes = serde_json::from_value(json!([
            "...",
            ["..."],
            "first\nsecond",
            ["(?x)", "# comment", "(?<path>file)"]
        ]))?;
        regexes.merge_from(&serde_json::from_value(json!([
            ["first", "second"],
            "...",
            "(?x)\n# comment\n(?<path>file)",
            "..."
        ]))?);
        assert_eq!(
            serde_json::to_value(regexes)?,
            json!([
                ["first", "second"],
                "...",
                ["..."],
                ["(?x)", "# comment", "(?<path>file)"]
            ])
        );
        Ok(())
    }

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
