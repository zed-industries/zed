use alacritty_terminal::vte::ansi::{
    CursorShape as AlacCursorShape, CursorStyle as AlacCursorStyle,
};
use collections::HashMap;
use gpui::{FontFallbacks, FontFeatures, FontWeight, Pixels, px};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub use settings::AlternateScroll;

use settings::{
    IntoGpui, PathHyperlinkRegex, RegisterSetting, ShowScrollbar, TerminalBlink,
    TerminalDockPosition, TerminalLineHeight, VenvSettings, WorkingDirectory,
    merge_from::MergeFrom,
};
use task::Shell;
use theme::FontFamilyName;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Toolbar {
    pub breadcrumbs: bool,
}

#[derive(Clone, Debug, Deserialize, RegisterSetting)]
pub struct TerminalSettings {
    pub shell: Shell,
    pub working_directory: WorkingDirectory,
    pub font_size: Option<Pixels>, // todo(settings_refactor) can be non-optional...
    pub font_family: Option<FontFamilyName>,
    pub font_fallbacks: Option<FontFallbacks>,
    pub font_features: Option<FontFeatures>,
    pub font_weight: Option<FontWeight>,
    pub line_height: TerminalLineHeight,
    pub env: HashMap<String, String>,
    pub cursor_shape: CursorShape,
    pub blinking: TerminalBlink,
    pub alternate_scroll: AlternateScroll,
    pub option_as_meta: bool,
    pub copy_on_select: bool,
    pub keep_selection_on_copy: bool,
    pub button: bool,
    pub dock: TerminalDockPosition,
    pub default_width: Pixels,
    pub default_height: Pixels,
    pub detect_venv: VenvSettings,
    pub max_scroll_history_lines: Option<usize>,
    pub scroll_multiplier: f32,
    pub toolbar: Toolbar,
    pub scrollbar: ScrollbarSettings,
    pub minimum_contrast: f32,
    pub path_hyperlink_regexes: Vec<String>,
    pub path_hyperlink_timeout_ms: u64,
    pub sandbox: Option<settings::SandboxSettingsContent>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarSettings {
    /// When to show the scrollbar in the terminal.
    ///
    /// Default: inherits editor scrollbar settings
    pub show: Option<ShowScrollbar>,
}

fn settings_shell_to_task_shell(shell: settings::Shell) -> Shell {
    match shell {
        settings::Shell::System => Shell::System,
        settings::Shell::Program(program) => Shell::Program(program),
        settings::Shell::WithArguments {
            program,
            args,
            title_override,
        } => Shell::WithArguments {
            program,
            args,
            title_override,
        },
    }
}

impl settings::Settings for TerminalSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let user_content = content.terminal.clone().unwrap();
        // Note: we allow a subset of "terminal" settings in the project files.
        let mut project_content = user_content.project.clone();
        project_content.merge_from_option(content.project.terminal.as_ref());
        TerminalSettings {
            shell: settings_shell_to_task_shell(project_content.shell.unwrap()),
            working_directory: project_content.working_directory.unwrap(),
            font_size: user_content.font_size.map(|s| s.into_gpui()),
            font_family: user_content.font_family,
            font_fallbacks: user_content.font_fallbacks.map(|fallbacks| {
                FontFallbacks::from_fonts(
                    fallbacks
                        .into_iter()
                        .map(|family| family.0.to_string())
                        .collect(),
                )
            }),
            font_features: user_content.font_features.map(|f| f.into_gpui()),
            font_weight: user_content.font_weight.map(|w| w.into_gpui()),
            line_height: user_content.line_height.unwrap(),
            env: project_content.env.unwrap(),
            cursor_shape: user_content.cursor_shape.unwrap().into(),
            blinking: user_content.blinking.unwrap(),
            alternate_scroll: user_content.alternate_scroll.unwrap(),
            option_as_meta: user_content.option_as_meta.unwrap(),
            copy_on_select: user_content.copy_on_select.unwrap(),
            keep_selection_on_copy: user_content.keep_selection_on_copy.unwrap(),
            button: user_content.button.unwrap(),
            dock: user_content.dock.unwrap(),
            default_width: px(user_content.default_width.unwrap()),
            default_height: px(user_content.default_height.unwrap()),
            detect_venv: project_content.detect_venv.unwrap(),
            scroll_multiplier: user_content.scroll_multiplier.unwrap(),
            max_scroll_history_lines: user_content.max_scroll_history_lines,
            toolbar: Toolbar {
                breadcrumbs: user_content.toolbar.unwrap().breadcrumbs.unwrap(),
            },
            scrollbar: ScrollbarSettings {
                show: user_content.scrollbar.unwrap().show,
            },
            minimum_contrast: user_content.minimum_contrast.unwrap(),
            path_hyperlink_regexes: project_content
                .path_hyperlink_regexes
                .unwrap()
                .into_iter()
                .map(|regex| match regex {
                    PathHyperlinkRegex::SingleLine(regex) => regex,
                    PathHyperlinkRegex::MultiLine(regex) => regex.join("\n"),
                })
                .collect(),
            path_hyperlink_timeout_ms: project_content.path_hyperlink_timeout_ms.unwrap(),
            sandbox: project_content.sandbox,
        }
    }
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

impl From<settings::CursorShapeContent> for CursorShape {
    fn from(value: settings::CursorShapeContent) -> Self {
        match value {
            settings::CursorShapeContent::Block => CursorShape::Block,
            settings::CursorShapeContent::Underline => CursorShape::Underline,
            settings::CursorShapeContent::Bar => CursorShape::Bar,
            settings::CursorShapeContent::Hollow => CursorShape::Hollow,
        }
    }
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

/// Resolved sandbox configuration with all defaults applied.
/// This is the concrete type passed to the terminal spawning code.
#[derive(Clone, Debug)]
pub struct SandboxConfig {
    pub project_dir: PathBuf,
    pub system_paths: ResolvedSystemPaths,
    pub additional_executable_paths: Vec<PathBuf>,
    pub additional_read_only_paths: Vec<PathBuf>,
    pub additional_read_write_paths: Vec<PathBuf>,
    pub allow_network: bool,
    pub allowed_env_vars: Vec<String>,
}

/// Resolved system paths with OS-specific defaults applied.
#[derive(Clone, Debug)]
pub struct ResolvedSystemPaths {
    pub executable: Vec<PathBuf>,
    pub read_only: Vec<PathBuf>,
    pub read_write: Vec<PathBuf>,
}

impl ResolvedSystemPaths {
    pub fn from_settings(settings: &settings::SystemPathsSettingsContent) -> Self {
        Self {
            executable: settings
                .executable
                .clone()
                .map(|v| v.into_iter().map(PathBuf::from).collect())
                .unwrap_or_else(Self::default_executable),
            read_only: settings
                .read_only
                .clone()
                .map(|v| v.into_iter().map(PathBuf::from).collect())
                .unwrap_or_else(Self::default_read_only),
            read_write: settings
                .read_write
                .clone()
                .map(|v| v.into_iter().map(PathBuf::from).collect())
                .unwrap_or_else(Self::default_read_write),
        }
    }

    pub fn with_defaults() -> Self {
        Self {
            executable: Self::default_executable(),
            read_only: Self::default_read_only(),
            read_write: Self::default_read_write(),
        }
    }

    #[cfg(target_os = "macos")]
    fn default_executable() -> Vec<PathBuf> {
        vec![
            "/bin".into(),
            "/usr/bin".into(),
            "/usr/sbin".into(),
            "/sbin".into(),
            "/usr/lib".into(),
            "/usr/libexec".into(),
            "/System/Library/dyld".into(),
            "/System/Cryptexes".into(),
            "/Library/Developer/CommandLineTools/usr/bin".into(),
            "/Library/Developer/CommandLineTools/usr/lib".into(),
            "/Library/Apple/usr/bin".into(),
            "/opt/homebrew/bin".into(),
            "/opt/homebrew/sbin".into(),
            "/opt/homebrew/Cellar".into(),
            "/opt/homebrew/lib".into(),
            "/usr/local/bin".into(),
            "/usr/local/lib".into(),
        ]
    }

    #[cfg(target_os = "linux")]
    fn default_executable() -> Vec<PathBuf> {
        vec![
            "/usr/bin".into(),
            "/usr/sbin".into(),
            "/usr/lib".into(),
            "/usr/lib64".into(),
            "/usr/libexec".into(),
            "/lib".into(),
            "/lib64".into(),
            "/bin".into(),
            "/sbin".into(),
        ]
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    fn default_executable() -> Vec<PathBuf> {
        vec![]
    }

    #[cfg(target_os = "macos")]
    fn default_read_only() -> Vec<PathBuf> {
        vec![
            "/private/etc".into(),
            "/usr/share".into(),
            "/System/Library/Keychains".into(),
            "/Library/Developer/CommandLineTools/SDKs".into(),
            "/Library/Preferences/SystemConfiguration".into(),
            "/opt/homebrew/share".into(),
            "/opt/homebrew/etc".into(),
            "/usr/local/share".into(),
            "/usr/local/etc".into(),
        ]
    }

    #[cfg(target_os = "linux")]
    fn default_read_only() -> Vec<PathBuf> {
        vec![
            "/etc".into(),
            "/usr/share".into(),
            "/usr/include".into(),
            "/usr/lib/locale".into(),
        ]
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    fn default_read_only() -> Vec<PathBuf> {
        vec![]
    }

    #[cfg(target_os = "macos")]
    fn default_read_write() -> Vec<PathBuf> {
        vec![
            "/dev".into(),
            "/private/tmp".into(),
            "/var/folders".into(),
            "/private/var/run/mDNSResponder".into(),
        ]
    }

    #[cfg(target_os = "linux")]
    fn default_read_write() -> Vec<PathBuf> {
        vec![
            "/dev".into(),
            "/tmp".into(),
            "/var/tmp".into(),
            "/dev/shm".into(),
            "/run/user".into(),
        ]
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    fn default_read_write() -> Vec<PathBuf> {
        vec![]
    }
}

impl SandboxConfig {
    /// Default environment variables to pass through to sandboxed terminals.
    pub fn default_allowed_env_vars() -> Vec<String> {
        vec![
            "PATH".into(),
            "HOME".into(),
            "USER".into(),
            "SHELL".into(),
            "LANG".into(),
            "TERM".into(),
            "TERM_PROGRAM".into(),
            "CARGO_HOME".into(),
            "RUSTUP_HOME".into(),
            "GOPATH".into(),
            "EDITOR".into(),
            "VISUAL".into(),
            "XDG_CONFIG_HOME".into(),
            "XDG_DATA_HOME".into(),
            "XDG_RUNTIME_DIR".into(),
            "SSH_AUTH_SOCK".into(),
            "GPG_TTY".into(),
            "COLORTERM".into(),
        ]
    }

    /// Resolve a `SandboxConfig` from settings, applying all defaults.
    pub fn from_settings(
        sandbox_settings: &settings::SandboxSettingsContent,
        project_dir: PathBuf,
    ) -> Self {
        let system_paths = sandbox_settings
            .system_paths
            .as_ref()
            .map(|sp| ResolvedSystemPaths::from_settings(sp))
            .unwrap_or_else(ResolvedSystemPaths::with_defaults);

        let home_dir = std::env::var("HOME").ok().map(PathBuf::from);
        let expand_paths = |paths: &Option<Vec<String>>| -> Vec<PathBuf> {
            paths
                .as_ref()
                .map(|v| {
                    v.iter()
                        .map(|p| {
                            if let Some(rest) = p.strip_prefix("~/") {
                                if let Some(ref home) = home_dir {
                                    return home.join(rest);
                                }
                            }
                            PathBuf::from(p)
                        })
                        .collect()
                })
                .unwrap_or_default()
        };

        Self {
            project_dir,
            system_paths,
            additional_executable_paths: expand_paths(
                &sandbox_settings.additional_executable_paths,
            ),
            additional_read_only_paths: expand_paths(&sandbox_settings.additional_read_only_paths),
            additional_read_write_paths: expand_paths(
                &sandbox_settings.additional_read_write_paths,
            ),
            allow_network: sandbox_settings.allow_network.unwrap_or(true),
            allowed_env_vars: sandbox_settings
                .allowed_env_vars
                .clone()
                .unwrap_or_else(Self::default_allowed_env_vars),
        }
    }

    pub fn canonicalize_paths(&mut self) {
        match std::fs::canonicalize(&self.project_dir) {
            Ok(canonical) => self.project_dir = canonical,
            Err(err) => log::warn!(
                "Failed to canonicalize project dir {:?}: {}",
                self.project_dir,
                err
            ),
        }
        canonicalize_path_list(&mut self.system_paths.executable);
        canonicalize_path_list(&mut self.system_paths.read_only);
        canonicalize_path_list(&mut self.system_paths.read_write);
        canonicalize_path_list(&mut self.additional_executable_paths);
        canonicalize_path_list(&mut self.additional_read_only_paths);
        canonicalize_path_list(&mut self.additional_read_write_paths);
    }
}

fn try_canonicalize(path: &mut PathBuf) {
    if let Ok(canonical) = std::fs::canonicalize(&*path) {
        *path = canonical;
    }
}

fn canonicalize_path_list(paths: &mut Vec<PathBuf>) {
    for path in paths.iter_mut() {
        try_canonicalize(path);
    }
}
