use alacritty_terminal::vte::ansi::{
    CursorShape as AlacCursorShape, CursorStyle as AlacCursorStyle,
};
use collections::HashMap;
use gpui::{FontFallbacks, FontFeatures, FontWeight, Pixels, px};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use settings::AlternateScroll;
use settings::{
    CursorShapeContent, SettingsContent, ShowScrollbar, TerminalBlink, TerminalDockPosition,
    TerminalLineHeight, TerminalSettingsContent, VenvSettings, WorkingDirectory,
    merge_from::MergeFrom,
};
use task::Shell;
use theme::FontFamilyName;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Toolbar {
    pub breadcrumbs: bool,
}

#[derive(Clone, Debug, Deserialize)]
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
    pub toolbar: Toolbar,
    pub scrollbar: ScrollbarSettings,
    pub minimum_contrast: f32,
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
            font_size: user_content.font_size.map(px),
            font_family: user_content.font_family,
            font_fallbacks: user_content.font_fallbacks.map(|fallbacks| {
                FontFallbacks::from_fonts(
                    fallbacks
                        .into_iter()
                        .map(|family| family.0.to_string())
                        .collect(),
                )
            }),
            font_features: user_content.font_features,
            font_weight: user_content.font_weight.map(FontWeight),
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
            max_scroll_history_lines: user_content.max_scroll_history_lines,
            toolbar: Toolbar {
                breadcrumbs: user_content.toolbar.unwrap().breadcrumbs.unwrap(),
            },
            scrollbar: ScrollbarSettings {
                show: user_content.scrollbar.unwrap().show,
            },
            minimum_contrast: user_content.minimum_contrast.unwrap(),
        }
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, content: &mut SettingsContent) {
        let mut default = TerminalSettingsContent::default();
        let current = content.terminal.as_mut().unwrap_or(&mut default);
        let name = |s| format!("terminal.integrated.{s}");

        vscode.f32_setting(&name("fontSize"), &mut current.font_size);
        vscode.font_family_setting(
            &name("fontFamily"),
            &mut current.font_family,
            &mut current.font_fallbacks,
        );
        vscode.bool_setting(&name("copyOnSelection"), &mut current.copy_on_select);
        vscode.bool_setting("macOptionIsMeta", &mut current.option_as_meta);
        vscode.usize_setting("scrollback", &mut current.max_scroll_history_lines);
        match vscode.read_bool(&name("cursorBlinking")) {
            Some(true) => current.blinking = Some(TerminalBlink::On),
            Some(false) => current.blinking = Some(TerminalBlink::Off),
            None => {}
        }
        vscode.enum_setting(
            &name("cursorStyle"),
            &mut current.cursor_shape,
            |s| match s {
                "block" => Some(CursorShapeContent::Block),
                "line" => Some(CursorShapeContent::Bar),
                "underline" => Some(CursorShapeContent::Underline),
                _ => None,
            },
        );
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
        #[cfg(target_os = "freebsd")]
        let platform = "freebsd";

        // TODO: handle arguments
        let shell_name = format!("{platform}Exec");
        if let Some(s) = vscode.read_string(&name(&shell_name)) {
            current.project.shell = Some(settings::Shell::Program(s.to_owned()))
        }

        if let Some(env) = vscode
            .read_value(&name(&format!("env.{platform}")))
            .and_then(|v| v.as_object())
        {
            for (k, v) in env {
                if v.is_null()
                    && let Some(zed_env) = current.project.env.as_mut()
                {
                    zed_env.remove(k);
                }
                let Some(v) = v.as_str() else { continue };
                if let Some(zed_env) = current.project.env.as_mut() {
                    zed_env.insert(k.clone(), v.to_owned());
                } else {
                    current.project.env = Some([(k.clone(), v.to_owned())].into_iter().collect())
                }
            }
        }
        if content.terminal.is_none() && default != TerminalSettingsContent::default() {
            content.terminal = Some(default)
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
