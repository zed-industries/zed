use collections::{HashMap, IndexMap};
use gpui::{FontFallbacks, FontFeatures, FontWeight, Pixels};
use parking_lot::Mutex as ParkingMutex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::LazyLock};
use util::shell_detection::DetectedShell;

pub use settings::AlternateScroll;

use settings::{
    IntoGpui, PathHyperlinkRegex, RegisterSetting, ShowScrollbar, TerminalBell, TerminalBlink,
    TerminalDockPosition, TerminalLineHeight, TerminalProfile, VenvSettings, WorkingDirectory,
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
    pub profiles: collections::IndexMap<String, TerminalProfile>,
    pub default_profile: Option<String>,
    /// Cached output of [`validate_configured_profiles`], so consumers
    /// don't re-run the validator on every menu open.
    #[serde(skip)]
    pub profile_warnings: Vec<ProfileWarning>,
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

/// Convert a `TerminalProfile` into the runtime `task::Shell` for the
/// spawn path. The profile name becomes the tab title when
/// `title_override` is unset.
pub fn profile_to_task_shell(name: &str, profile: &TerminalProfile) -> Shell {
    Shell::WithArguments {
        program: expand_tilde(&profile.program),
        args: profile.args.clone().unwrap_or_default(),
        title_override: profile.title_override.clone().or(Some(name.to_string())),
    }
}

/// Expand a leading `~` to the user's home directory. Validation, menu
/// shadowing, and spawn all resolve the same expanded program.
fn expand_tilde(program: &str) -> String {
    if program == "~" {
        return util::paths::home_dir().to_string_lossy().into_owned();
    }
    if let Some(rest) = program.strip_prefix("~/") {
        return util::paths::home_dir()
            .join(rest)
            .to_string_lossy()
            .into_owned();
    }
    program.to_string()
}

/// A configured profile or detected shell entry for the "+" menu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergedShellEntry {
    Configured {
        name: String,
        profile: TerminalProfile,
    },
    Detected(DetectedShell),
}

/// Combine configured profiles with detected shells, dropping detected
/// entries whose program collides with a configured profile's program.
/// Path-bearing configured programs only shadow via exact path equality —
/// otherwise `program: "/bin/bash"` would shadow every bash on the system.
pub fn merge_with_configured_profiles(
    detected: Vec<DetectedShell>,
    profiles: &IndexMap<String, TerminalProfile>,
) -> Vec<MergedShellEntry> {
    let configured_programs: Vec<String> = profiles.values().map(|p| p.program.clone()).collect();

    let mut entries: Vec<MergedShellEntry> = profiles
        .iter()
        .map(|(name, profile)| MergedShellEntry::Configured {
            name: name.clone(),
            profile: profile.clone(),
        })
        .collect();

    for shell in detected {
        let shadowed = detected_program_is_shadowed(
            shell.program.as_ref(),
            &configured_programs,
            cfg!(windows),
        );
        if !shadowed {
            entries.push(MergedShellEntry::Detected(shell));
        }
    }

    entries
}

/// Whether a detected shell's program collides with a configured profile.
/// With `windows_extension_matching`, the detected program's file stem
/// (`pwsh` from `pwsh.exe`) also matches configured bare names, and all
/// comparisons are case-insensitive (Windows paths are).
fn detected_program_is_shadowed(
    program: &std::path::Path,
    configured_programs: &[String],
    windows_extension_matching: bool,
) -> bool {
    let fold = |key: &str| {
        if windows_extension_matching {
            // Windows accepts both separators, so normalize before
            // comparing case-insensitively.
            key.replace("/", "\\").to_lowercase()
        } else {
            key.to_string()
        }
    };
    let configured_paths: std::collections::HashSet<String> =
        configured_programs.iter().map(|p| fold(p)).collect();
    if configured_paths.contains(&fold(&program.to_string_lossy())) {
        return true;
    }
    let configured_stems: std::collections::HashSet<String> = configured_programs
        .iter()
        .filter(|p| {
            !p.contains(std::path::MAIN_SEPARATOR)
                && !(windows_extension_matching && p.contains('/'))
        })
        .map(|p| fold(p))
        .collect();
    if let Some(name) = program.file_name()
        && configured_stems.contains(&fold(&name.to_string_lossy()))
    {
        return true;
    }
    windows_extension_matching
        && program
            .file_stem()
            .is_some_and(|stem| configured_stems.contains(&fold(&stem.to_string_lossy())))
}

/// Single menu entry for the "+" PopoverMenu, produced by
/// [`build_menu_entries`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuEntry {
    Configured {
        name: String,
    },
    Detected {
        label: String,
        program: String,
        args: Vec<String>,
    },
}

pub const MENU_ESCALATION_THRESHOLD: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuEntryPlan {
    Inline { entries: Vec<MenuEntry> },
    Collapsed { entries: Vec<MenuEntry> },
}

/// Pure, fs-free computation of the "+" menu contents. Profiles named in
/// `warnings` are hidden from the menu.
pub fn build_menu_entries(
    profiles: &IndexMap<String, TerminalProfile>,
    detected: Vec<DetectedShell>,
    warnings: &[ProfileWarning],
) -> MenuEntryPlan {
    let hidden: std::collections::HashSet<&str> =
        warnings.iter().map(|w| w.profile_name.as_str()).collect();

    let visible_profiles: IndexMap<String, TerminalProfile> = profiles
        .iter()
        .filter(|(name, _)| !hidden.contains(name.as_str()))
        .map(|(name, profile)| (name.clone(), profile.clone()))
        .collect();

    let merged = merge_with_configured_profiles(detected, &visible_profiles);

    let entries: Vec<MenuEntry> = merged
        .into_iter()
        .map(|entry| match entry {
            MergedShellEntry::Configured { name, .. } => MenuEntry::Configured { name },
            MergedShellEntry::Detected(shell) => MenuEntry::Detected {
                label: shell.label,
                program: shell.program.to_string_lossy().into_owned(),
                args: shell.args,
            },
        })
        .collect();

    if entries.len() > MENU_ESCALATION_THRESHOLD {
        MenuEntryPlan::Collapsed { entries }
    } else {
        MenuEntryPlan::Inline { entries }
    }
}

/// An advisory warning about a configured profile whose `program` is
/// neither an existing absolute path nor resolvable on `PATH`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileWarning {
    pub profile_name: String,
    pub program: String,
    pub reason: String,
}

fn validate_configured_profiles_with(
    profiles: &IndexMap<String, TerminalProfile>,
    is_file: &dyn Fn(&Path) -> bool,
    resolve_on_path: &dyn Fn(&str) -> Option<std::path::PathBuf>,
) -> Vec<ProfileWarning> {
    let mut warnings = Vec::new();
    for (name, profile) in profiles {
        let program = profile.program.as_str();
        if program.is_empty() {
            warnings.push(ProfileWarning {
                profile_name: name.clone(),
                program: program.to_string(),
                reason: "profile program is empty".to_string(),
            });
            continue;
        }
        let program = expand_tilde(program);
        let path = Path::new(&program);
        let absolute_exists = path.is_absolute() && is_file(path);
        let on_path = !path.is_absolute() && resolve_on_path(&program).is_some();
        if !absolute_exists && !on_path {
            warnings.push(ProfileWarning {
                profile_name: name.clone(),
                program: program.to_string(),
                reason: format!(
                    "profile program '{program}' is not an existing file and is not found on PATH"
                ),
            });
        }
    }
    warnings
}

/// Programs already warned about, so an invalid profile logs once per
/// process instead of on every settings recompute.
static WARNED_PROFILE_PROGRAMS: LazyLock<ParkingMutex<std::collections::HashSet<String>>> =
    LazyLock::new(|| ParkingMutex::new(std::collections::HashSet::new()));

/// Return one [`ProfileWarning`] per profile whose `program` is neither an
/// existing absolute path nor resolvable on `PATH`. Advisory only: actual
/// spawn failures surface through the regular `TerminalError` path.
pub fn validate_configured_profiles(
    profiles: &IndexMap<String, TerminalProfile>,
) -> Vec<ProfileWarning> {
    validate_configured_profiles_with(profiles, &|p| p.is_file(), &|program| {
        which::which(program).ok()
    })
}

impl settings::Settings for TerminalSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let user_content = content.terminal.clone().unwrap();
        // Note: we allow a subset of "terminal" settings in the project files.
        let mut project_content = user_content.project.clone();
        project_content.merge_from_option(content.project.terminal.as_ref());
        let profiles = project_content.profiles.clone().unwrap_or_default();
        let default_profile = project_content.default_profile.clone();
        let profile_warnings = validate_configured_profiles(&profiles);
        let mut warned_programs = WARNED_PROFILE_PROGRAMS.lock();
        for warning in &profile_warnings {
            if warned_programs.insert(warning.program.clone()) {
                log::warn!(
                    "terminal.profiles.{}: {}",
                    warning.profile_name,
                    warning.reason
                );
            }
        }
        let shell = if let Some(name) = default_profile.as_deref() {
            match profiles.get(name) {
                Some(profile) => profile_to_task_shell(name, profile),
                None => {
                    log::warn!(
                        "terminal.default_profile references unknown profile '{name}'; \
                         falling back to terminal.shell"
                    );
                    settings_shell_to_task_shell(project_content.shell.unwrap())
                }
            }
        } else {
            settings_shell_to_task_shell(project_content.shell.unwrap())
        };
        TerminalSettings {
            shell,
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
            profiles,
            default_profile,
            profile_warnings,
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

#[cfg(test)]
mod tests {
    use super::*;
    use settings::{
        LocalSettingsKind, LocalSettingsPath, SettingsLocation, SettingsStore, TerminalProfile,
        WorktreeId,
    };
    use util::rel_path::rel_path;

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

    fn profile(program: &str, args: &[&str], title: Option<&str>) -> TerminalProfile {
        TerminalProfile {
            program: program.to_string(),
            args: Some(args.iter().map(|a| a.to_string()).collect()),
            title_override: title.map(|t| t.to_string()),
        }
    }

    #[test]
    fn profile_to_shell_promotes_title_to_profile_name_when_unset() {
        let p = profile("/bin/zsh", &["-l"], None);
        let shell = profile_to_task_shell("Zsh", &p);
        match shell {
            Shell::WithArguments {
                program,
                args,
                title_override,
            } => {
                assert_eq!(program, "/bin/zsh");
                assert_eq!(args, vec!["-l".to_string()]);
                assert_eq!(title_override, Some("Zsh".to_string()));
            }
            other => panic!("expected WithArguments, got {other:?}"),
        }
    }

    #[test]
    fn profile_to_shell_preserves_explicit_title_override() {
        let p = profile("/bin/zsh", &[], Some("My Shell"));
        let shell = profile_to_task_shell("Zsh", &p);
        match shell {
            Shell::WithArguments { title_override, .. } => {
                assert_eq!(title_override, Some("My Shell".to_string()));
            }
            other => panic!("expected WithArguments, got {other:?}"),
        }
    }

    #[test]
    fn profile_to_shell_defaults_empty_args() {
        let p = TerminalProfile {
            program: "pwsh".to_string(),
            args: None,
            title_override: None,
        };
        let shell = profile_to_task_shell("PowerShell", &p);
        match shell {
            Shell::WithArguments { args, .. } => assert!(args.is_empty()),
            other => panic!("expected WithArguments, got {other:?}"),
        }
    }

    fn profile_map(entries: &[(&str, &str)]) -> collections::IndexMap<String, TerminalProfile> {
        entries
            .iter()
            .map(|(name, program)| {
                (
                    name.to_string(),
                    TerminalProfile {
                        program: program.to_string(),
                        args: None,
                        title_override: None,
                    },
                )
            })
            .collect()
    }

    #[test]
    fn merge_places_configured_first_then_detected() {
        let profiles = profile_map(&[("Zsh", "/bin/zsh"), ("Fish", "/usr/bin/fish")]);
        let detected = vec![
            detected_shell("bash", "/bin/bash"),
            detected_shell("python", "/usr/bin/python"),
        ];
        let merged = merge_with_configured_profiles(detected, &profiles);
        let mut labels = merged.iter().map(|entry| match entry {
            MergedShellEntry::Configured { name, .. } => name.clone(),
            MergedShellEntry::Detected(shell) => shell.label.clone(),
        });
        assert_eq!(labels.next().as_deref(), Some("Zsh"));
        assert_eq!(labels.next().as_deref(), Some("Fish"));
        assert_eq!(labels.next().as_deref(), Some("bash"));
        assert_eq!(labels.next().as_deref(), Some("python"));
        assert_eq!(labels.next(), None);
    }

    #[test]
    fn merge_shadows_detected_when_program_path_matches_exactly() {
        let profiles = profile_map(&[("Zsh", "/bin/zsh")]);
        let detected = vec![
            detected_shell("zsh", "/bin/zsh"),
            detected_shell("bash", "/bin/bash"),
        ];
        let merged = merge_with_configured_profiles(detected, &profiles);
        let labels: Vec<String> = merged
            .iter()
            .map(|entry| match entry {
                MergedShellEntry::Configured { name, .. } => name.clone(),
                MergedShellEntry::Detected(shell) => shell.label.clone(),
            })
            .collect();
        assert_eq!(labels, vec!["Zsh".to_string(), "bash".to_string()]);
    }

    #[test]
    fn merge_shadows_detected_when_configured_is_basename_match() {
        let profiles = profile_map(&[("MyZsh", "zsh")]);
        let detected = vec![detected_shell("zsh", "/bin/zsh")];
        let merged = merge_with_configured_profiles(detected, &profiles);
        assert_eq!(merged.len(), 1);
        match &merged[0] {
            MergedShellEntry::Configured { name, .. } => assert_eq!(name, "MyZsh"),
            MergedShellEntry::Detected(_) => panic!("basename match should shadow detected"),
        }
    }

    #[test]
    fn merge_shadows_detected_exe_by_configured_stem_on_windows() {
        let profiles = profile_map(&[("PowerShell", "pwsh")]);
        let detected = vec![detected_shell("pwsh", r"C:\Windows\System32\pwsh.exe")];
        let merged = merge_with_configured_profiles(detected, &profiles);
        match &merged[0] {
            MergedShellEntry::Configured { name, .. } => assert_eq!(name, "PowerShell"),
            MergedShellEntry::Detected(_) => panic!("basename match should shadow detected"),
        }
        if cfg!(windows) {
            assert_eq!(merged.len(), 1, "detected pwsh.exe should be shadowed");
        }

        let configured_programs = vec!["pwsh".to_string()];
        // Case-insensitive on Windows: a detected "PWSH.exe" shadows
        // against configured "pwsh"; case-sensitive elsewhere.
        let program = std::path::Path::new("/windows/system32/PWSH.exe");
        assert!(detected_program_is_shadowed(
            program,
            &configured_programs,
            true,
        ));
        assert!(!detected_program_is_shadowed(
            program,
            &configured_programs,
            false,
        ));
    }

    #[test]
    fn merge_does_not_shadow_by_basename_when_configured_is_path() {
        let profiles = profile_map(&[("Bash", "/bin/bash")]);
        let detected = vec![detected_shell("bash", "/usr/bin/bash")];
        let merged = merge_with_configured_profiles(detected, &profiles);
        assert_eq!(
            merged.len(),
            2,
            "path-bearing configured program must NOT shadow detected shells at different paths by basename"
        );
        assert!(matches!(merged[0], MergedShellEntry::Configured { .. }));
        assert!(matches!(merged[1], MergedShellEntry::Detected(_)));
    }

    #[test]
    fn merge_keeps_detected_when_program_differs() {
        let profiles = profile_map(&[("Zsh", "/bin/zsh")]);
        let detected = vec![
            detected_shell("bash", "/bin/bash"),
            detected_shell("fish", "/usr/bin/fish"),
        ];
        let merged = merge_with_configured_profiles(detected, &profiles);
        assert_eq!(merged.len(), 3);
    }

    #[test]
    fn validate_returns_no_warnings_when_all_programs_resolve() {
        let profiles = profile_map(&[
            ("Zsh", "/bin/zsh"),
            ("Bash", "bash"),
            ("Fish", "/usr/bin/fish"),
        ]);
        let warnings = validate_configured_profiles_with(
            &profiles,
            &|p| matches!(p.to_string_lossy().as_ref(), "/bin/zsh" | "/usr/bin/fish"),
            &|program| match program {
                "bash" => Some(std::path::PathBuf::from("/usr/bin/bash")),
                _ => None,
            },
        );
        assert!(
            warnings.is_empty(),
            "all programs resolvable -> no warnings, got {warnings:?}"
        );
    }

    #[test]
    fn validate_warns_when_absolute_program_does_not_exist() {
        let profiles = profile_map(&[("Broken", "/definitely/not/a/real/shell")]);
        let warnings = validate_configured_profiles_with(&profiles, &|_| false, &|_| None);
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].profile_name, "Broken");
        assert!(warnings[0].reason.contains("not an existing file"));
    }

    #[test]
    fn validate_warns_when_relative_program_not_on_path() {
        let profiles = profile_map(&[("Nope", "definitely-not-a-real-shell-xyz")]);
        let warnings = validate_configured_profiles_with(&profiles, &|_| false, &|_| None);
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].profile_name, "Nope");
        assert!(warnings[0].reason.contains("not found on PATH"));
    }

    #[test]
    fn validate_warns_on_empty_program() {
        let profiles = profile_map(&[("Empty", "")]);
        let warnings = validate_configured_profiles_with(&profiles, &|_| false, &|_| None);
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].profile_name, "Empty");
        assert!(warnings[0].reason.contains("empty"));
    }

    #[test]
    fn validate_does_not_block_on_partial_failure() {
        let profiles = profile_map(&[
            ("Good", "/bin/zsh"),
            ("Bad", "totally-bogus-program"),
            ("AlsoGood", "bash"),
        ]);
        let warnings = validate_configured_profiles_with(
            &profiles,
            &|p| p.to_string_lossy() == "/bin/zsh",
            &|program| match program {
                "bash" => Some(std::path::PathBuf::from("/usr/bin/bash")),
                _ => None,
            },
        );
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].profile_name, "Bad");
    }

    #[test]
    fn validate_handles_empty_profile_map() {
        let profiles: collections::IndexMap<String, TerminalProfile> =
            collections::IndexMap::default();
        let warnings = validate_configured_profiles_with(&profiles, &|_| false, &|_| None);
        assert!(warnings.is_empty());
    }

    fn detected_shell(label: &str, program: &str) -> DetectedShell {
        use util::shell_detection::ShellSource;
        DetectedShell {
            label: label.to_string(),
            program: std::path::PathBuf::from(program),
            args: Vec::new(),
            source: ShellSource::EtcShells,
        }
    }

    fn build_plan(
        profiles: &[(&str, &str)],
        detected: Vec<DetectedShell>,
        warning_names: &[&str],
    ) -> MenuEntryPlan {
        let profile_map: collections::IndexMap<String, TerminalProfile> = profiles
            .iter()
            .map(|(name, program)| {
                (
                    name.to_string(),
                    TerminalProfile {
                        program: program.to_string(),
                        args: None,
                        title_override: None,
                    },
                )
            })
            .collect();
        let warnings: Vec<ProfileWarning> = warning_names
            .iter()
            .map(|name| ProfileWarning {
                profile_name: name.to_string(),
                program: String::new(),
                reason: "test stub".to_string(),
            })
            .collect();
        build_menu_entries(&profile_map, detected, &warnings)
    }

    fn entry_labels(plan: &MenuEntryPlan) -> Vec<String> {
        let entries = match plan {
            MenuEntryPlan::Inline { entries } => entries,
            MenuEntryPlan::Collapsed { entries } => entries,
        };
        entries
            .iter()
            .map(|e| match e {
                MenuEntry::Configured { name } => name.clone(),
                MenuEntry::Detected { label, .. } => label.clone(),
            })
            .collect()
    }

    #[test]
    fn menu_inline_when_at_threshold() {
        let names: Vec<String> = (1..=MENU_ESCALATION_THRESHOLD)
            .map(|i| format!("n{i}"))
            .collect();
        let programs: Vec<String> = (1..=MENU_ESCALATION_THRESHOLD)
            .map(|i| format!("/bin/n{i}"))
            .collect();
        let profiles: Vec<(&str, &str)> = names
            .iter()
            .zip(&programs)
            .map(|(name, program)| (name.as_str(), program.as_str()))
            .collect();
        let plan = build_plan(&profiles, Vec::new(), &[]);
        assert!(
            matches!(plan, MenuEntryPlan::Inline { .. }),
            "exactly at threshold should stay inline"
        );
    }

    #[test]
    fn menu_collapses_above_threshold() {
        let names: Vec<String> = (1..=9).map(|i| format!("n{i}")).collect();
        let programs: Vec<String> = (1..=9).map(|i| format!("/bin/n{i}")).collect();
        let profiles: Vec<(&str, &str)> = names
            .iter()
            .zip(&programs)
            .map(|(name, program)| (name.as_str(), program.as_str()))
            .collect();
        let plan = build_plan(&profiles, Vec::new(), &[]);
        match plan {
            MenuEntryPlan::Collapsed { entries } => {
                assert_eq!(entries.len(), 9, "all entries preserved in submenu");
            }
            _ => panic!("9 entries should collapse to submenu"),
        }
    }

    #[test]
    fn menu_hides_invalid_profiles() {
        let plan = build_plan(
            &[("Good", "/bin/zsh"), ("Bad", "/nonexistent")],
            vec![],
            &["Bad"],
        );
        let labels = entry_labels(&plan);
        assert!(labels.contains(&"Good".to_string()));
        assert!(
            !labels.contains(&"Bad".to_string()),
            "profile named in warnings should be hidden from menu"
        );
    }

    #[test]
    fn menu_lets_detected_surface_when_configured_invalid() {
        let plan = build_plan(
            &[("Broken", "/bin/zsh")],
            vec![detected_shell("zsh", "/bin/zsh")],
            &["Broken"],
        );
        let labels = entry_labels(&plan);
        assert!(
            labels.contains(&"zsh".to_string()),
            "detected zsh should surface after the broken configured profile is hidden"
        );
        assert!(!labels.contains(&"Broken".to_string()));
    }

    #[test]
    fn menu_handles_empty_inputs() {
        let plan = build_plan(&[], Vec::new(), &[]);
        match plan {
            MenuEntryPlan::Inline { entries } => assert!(entries.is_empty()),
            _ => panic!("empty input should be inline-empty, not collapsed"),
        }
    }

    #[test]
    fn menu_detected_entry_carries_program_and_args() {
        let mut detected = detected_shell("wsl-ubuntu", "wsl.exe");
        detected.args = vec!["-d".to_string(), "Ubuntu".to_string()];
        let plan = build_plan(&[], vec![detected], &[]);
        match plan {
            MenuEntryPlan::Inline { entries } => match &entries[0] {
                MenuEntry::Detected {
                    label,
                    program,
                    args,
                } => {
                    assert_eq!(label, "wsl-ubuntu");
                    assert_eq!(program, "wsl.exe");
                    assert_eq!(args, &vec!["-d".to_string(), "Ubuntu".to_string()]);
                }
                _ => panic!("expected Detected entry"),
            },
            _ => panic!("expected Inline"),
        }
    }

    #[test]
    fn menu_escalation_counts_after_filtering() {
        let names: Vec<String> = (1..=10).map(|i| format!("n{i}")).collect();
        let programs: Vec<String> = (1..=10).map(|i| format!("/bin/n{i}")).collect();
        let profiles: Vec<(&str, &str)> = names
            .iter()
            .zip(&programs)
            .map(|(name, program)| (name.as_str(), program.as_str()))
            .collect();
        let plan = build_plan(&profiles, Vec::new(), &["n1", "n2", "n3"]);
        match plan {
            MenuEntryPlan::Inline { entries } => {
                assert_eq!(entries.len(), 7, "filtered count should drive threshold");
            }
            _ => panic!("7 visible entries should stay inline"),
        }
    }
}
