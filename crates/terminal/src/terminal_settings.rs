use anyhow::{Context as _, Result, ensure};
use collections::{HashMap, IndexMap};
use gpui::{FontFallbacks, FontFeatures, FontWeight, Pixels};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use settings::AlternateScroll;

use settings::{
    IntoGpui, PathHyperlinkRegex, RegisterSetting, ShowScrollbar, TerminalBell, TerminalBlink,
    TerminalDockPosition, TerminalLineHeight, VenvSettings, WorkingDirectory,
    merge_from::MergeFrom,
};
use task::Shell;
use theme_settings::FontFamilyName;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Toolbar {
    pub breadcrumbs: bool,
}

#[derive(Clone, Debug, Deserialize, RegisterSetting)]
pub struct TerminalSettings {
    #[serde(default)]
    pub profiles: IndexMap<String, settings::TerminalProfileContent>,
    #[serde(default)]
    pub default_profile: Option<String>,
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
    pub open_links_in_mouse_mode: bool,
    pub button: bool,
    pub dock: TerminalDockPosition,
    pub starts_open: bool,
    pub flexible: bool,
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
    pub show_count_badge: bool,
    pub bell: TerminalBell,
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
            profiles: project_content.profiles.unwrap_or_default(),
            default_profile: project_content.default_profile,
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
            open_links_in_mouse_mode: user_content.open_links_in_mouse_mode.unwrap(),
            button: user_content.button.unwrap(),
            dock: user_content.dock.unwrap(),
            starts_open: user_content.starts_open.unwrap(),
            default_width: user_content.default_width.unwrap().into_gpui(),
            default_height: user_content.default_height.unwrap().into_gpui(),
            flexible: user_content.flexible.unwrap(),
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
                .0
                .into_iter()
                .map(|regex| match regex {
                    PathHyperlinkRegex::SingleLine(regex) => regex,
                    PathHyperlinkRegex::MultiLine(regex) => regex.join("\n"),
                })
                .collect(),
            path_hyperlink_timeout_ms: project_content.path_hyperlink_timeout_ms.unwrap(),
            show_count_badge: user_content.show_count_badge.unwrap(),
            bell: user_content.bell.unwrap(),
        }
    }
}

impl TerminalSettings {
    pub fn resolve_profile(&self, name: Option<&str>) -> Result<(Option<String>, Self)> {
        let mut settings = self.clone();
        let Some(name) = name.or(self
            .default_profile
            .as_deref()
            .filter(|name| !name.trim().is_empty()))
        else {
            return Ok((None, settings));
        };
        ensure!(
            !name.trim().is_empty(),
            "Terminal profile names must not be blank"
        );
        let profile = self
            .profiles
            .get(name)
            .with_context(|| format!("Terminal profile {name:?} is not configured"))?;
        if let Some(shell) = profile.shell.clone() {
            settings.shell = settings_shell_to_task_shell(shell);
        }
        if let Some(environment) = profile.env.as_ref() {
            settings.env.extend(environment.clone());
        }
        if let Some(working_directory) = profile.working_directory.clone() {
            settings.working_directory = working_directory;
        }
        if let Some(detect_venv) = profile.detect_venv.clone() {
            settings.detect_venv = detect_venv;
        }
        Ok((Some(name.to_owned()), settings))
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

#[cfg(test)]
mod tests {
    use super::*;
    use settings::{
        LocalSettingsKind, LocalSettingsPath, SettingsLocation, SettingsStore, WorktreeId,
    };
    use util::rel_path::rel_path;

    #[gpui::test]
    fn test_terminal_profiles_resolve_without_changing_defaults(cx: &mut gpui::App) {
        let result = (|| -> Result<()> {
            let mut store = SettingsStore::new(cx, &settings::default_settings());
            store
                .set_user_settings(
                    r#"{"terminal": {
            "shell": {"program": "bash"},
            "env": {"SHARED": "default"},
            "default_profile": "Bash",
            "profiles": {
                "Bash": {},
                "JavaScript": {
                    "shell": {"with_arguments": {"program": "node", "args": ["--interactive"]}},
                    "env": {"SHARED": "javascript", "NODE_PROFILE": "1"},
                    "working_directory": "always_home"
                }
            }
        }}"#,
                    cx,
                )
                .result()?;
            let settings = store.get::<TerminalSettings>(None);
            let (name, javascript) = settings.resolve_profile(Some("JavaScript"))?;
            assert_eq!(name.as_deref(), Some("JavaScript"));
            assert!(
                matches!(&javascript.shell, Shell::WithArguments { program, args, .. }
            if program == "node" && args == &["--interactive"])
            );
            assert_eq!(
                javascript.env.get("SHARED").map(String::as_str),
                Some("javascript")
            );
            assert_eq!(javascript.working_directory, WorkingDirectory::AlwaysHome);
            let (name, default) = settings.resolve_profile(None)?;
            assert_eq!(name.as_deref(), Some("Bash"));
            assert_eq!(default.shell.program(), "bash");
            assert_eq!(
                default.env.get("SHARED").map(String::as_str),
                Some("default")
            );
            assert!(!default.env.contains_key("NODE_PROFILE"));
            assert!(settings.resolve_profile(Some("missing")).is_err());
            assert!(settings.resolve_profile(Some(" ")).is_err());
            assert_eq!(settings.shell.program(), "bash");
            Ok(())
        })();
        assert!(result.is_ok(), "{result:?}");
    }

    #[gpui::test]
    fn test_terminal_profiles_inherit_project_settings(cx: &mut gpui::App) {
        let result = (|| -> Result<()> {
            let mut store = SettingsStore::new(cx, &settings::default_settings());
            store
                .set_user_settings(
                    r#"{"terminal": {
            "profiles": {"Bash": {"shell": {"program": "bash"}, "env": {"USER_PROFILE": "1"}}}
        }}"#,
                    cx,
                )
                .result()?;
            store.set_local_settings(
                WorktreeId::from_usize(1),
                LocalSettingsPath::InWorktree(rel_path("project").into()),
                LocalSettingsKind::Settings,
                Some(
                    r#"{"terminal": {
                "default_profile": "Bash",
                "env": {"PROJECT_SHARED": "1"},
                "profiles": {"Bash": {"env": {"PROJECT_PROFILE": "1"}}}
            }}"#,
                ),
                cx,
            )?;
            let settings = store.get::<TerminalSettings>(Some(SettingsLocation {
                worktree_id: WorktreeId::from_usize(1),
                path: rel_path("project"),
            }));
            let (_, profile) = settings.resolve_profile(None)?;
            for variable in ["USER_PROFILE", "PROJECT_SHARED", "PROJECT_PROFILE"] {
                assert_eq!(profile.env.get(variable).map(String::as_str), Some("1"));
            }
            assert_eq!(profile.shell.program(), "bash");
            let (_, legacy) = store.get::<TerminalSettings>(None).resolve_profile(None)?;
            assert!(!legacy.env.contains_key("PROJECT_SHARED"));
            Ok(())
        })();
        assert!(result.is_ok(), "{result:?}");
    }

    fn path_regexes<'a>(store: &'a SettingsStore, path: Option<&str>) -> &'a [String] {
        &store
            .get::<TerminalSettings>(path.map(|path| SettingsLocation {
                worktree_id: WorktreeId::from_usize(1),
                path: rel_path(path),
            }))
            .path_hyperlink_regexes
    }

    #[gpui::test]
    fn test_path_hyperlink_regexes_inherit_across_settings_layers(cx: &mut gpui::App) {
        let mut store = SettingsStore::new(cx, &settings::default_settings());
        let defaults = path_regexes(&store, None).to_vec();
        store
            .set_user_settings(
                r##"{"terminal": {"path_hyperlink_regexes": [
                    ["(?x)", "# Keep this comment", "(?<path>user)"],
                    "...",
                    "(?<path>tail)",
                    "..."
                ]}}"##,
                cx,
            )
            .expect("user terminal settings must parse");
        let user_regex = "(?x)\n# Keep this comment\n(?<path>user)".to_owned();
        let mut user = vec![user_regex.clone()];
        user.extend(defaults.clone());
        user.push("(?<path>tail)".into());
        assert_eq!(path_regexes(&store, None), user);

        for (path, content) in [
            (
                "project",
                r#"{"terminal": {"path_hyperlink_regexes": [
                    "(?<path>project)",
                    "...",
                    ["..."],
                    "(?x)\n# Keep this comment\n(?<path>user)"
                ]}}"#,
            ),
            (
                "project/child",
                r##"{"terminal": {"path_hyperlink_regexes": [
                    ["(?x)", "# Keep this comment", "(?<path>user)"],
                    "...",
                    ["..."],
                    ["(?x)", "# Keep this too", "(?<path>child)", "..."]
                ]}}"##,
            ),
        ] {
            store
                .set_local_settings(
                    WorktreeId::from_usize(1),
                    LocalSettingsPath::InWorktree(rel_path(path).into()),
                    LocalSettingsKind::Settings,
                    Some(content),
                    cx,
                )
                .expect("project terminal settings must parse");
        }

        let mut project = vec!["(?<path>project)".to_owned()];
        project.extend(user.clone());
        project.push("...".into());
        assert_eq!(path_regexes(&store, Some("project/file")), project);

        let mut child = vec![user_regex, "(?<path>project)".to_owned()];
        child.extend(defaults);
        child.extend([
            "(?<path>tail)".into(),
            "...".into(),
            "(?x)\n# Keep this too\n(?<path>child)\n...".into(),
        ]);
        assert_eq!(path_regexes(&store, Some("project/child/file")), child);
        assert_eq!(path_regexes(&store, Some("elsewhere/file")), user);
    }

    #[gpui::test]
    fn test_path_hyperlink_regexes_replace_clear_and_omit(cx: &mut gpui::App) {
        let mut store = SettingsStore::new(cx, &settings::default_settings());
        let defaults = path_regexes(&store, None).to_vec();
        store
            .set_user_settings(
                r#"{"terminal": {"path_hyperlink_regexes": ["(?<path>user)"]}}"#,
                cx,
            )
            .expect("user terminal settings must parse");
        assert_eq!(path_regexes(&store, None), ["(?<path>user)"]);

        for (path, content) in [
            (
                "project",
                r#"{"terminal": {"path_hyperlink_regexes": ["(?<path>project)"]}}"#,
            ),
            (
                "project/child",
                r#"{"terminal": {"path_hyperlink_regexes": []}}"#,
            ),
            (
                "project/child/nested",
                r#"{"terminal": {"path_hyperlink_regexes": [
                    "...", ["(?x)", "...", "(?<path>nested)"]
                ]}}"#,
            ),
        ] {
            store
                .set_local_settings(
                    WorktreeId::from_usize(1),
                    LocalSettingsPath::InWorktree(rel_path(path).into()),
                    LocalSettingsKind::Settings,
                    Some(content),
                    cx,
                )
                .expect("project terminal settings must parse");
        }
        assert_eq!(
            path_regexes(&store, Some("project/file")),
            ["(?<path>project)"]
        );
        assert!(path_regexes(&store, Some("project/child/file")).is_empty());
        assert_eq!(
            path_regexes(&store, Some("project/child/nested/file")),
            ["(?x)\n...\n(?<path>nested)"]
        );
        assert_eq!(path_regexes(&store, None), ["(?<path>user)"]);

        store
            .set_local_settings(
                WorktreeId::from_usize(1),
                LocalSettingsPath::InWorktree(rel_path("project").into()),
                LocalSettingsKind::Settings,
                Some(r#"{"terminal": {"path_hyperlink_timeout_ms": 17}}"#),
                cx,
            )
            .expect("project terminal settings must parse");
        assert_eq!(
            path_regexes(&store, Some("project/file")),
            ["(?<path>user)"]
        );
        store
            .set_user_settings(r#"{"terminal": {"path_hyperlink_regexes": []}}"#, cx)
            .expect("user terminal settings must parse");
        assert!(path_regexes(&store, None).is_empty());
        assert!(path_regexes(&store, Some("project/file")).is_empty());
        store
            .set_user_settings("{}", cx)
            .expect("empty user settings must parse");
        assert_eq!(path_regexes(&store, None), defaults);
        assert_eq!(path_regexes(&store, Some("project/file")), defaults);
        assert!(path_regexes(&store, Some("project/child/file")).is_empty());
    }
}
