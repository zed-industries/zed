use collections::{HashMap, IndexMap};
use gpui::{FontFallbacks, FontFeatures, FontWeight, Pixels, px};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::Path;
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
    /// Cached output of [`validate_configured_profiles`] for the current
    /// settings, so consumers (e.g. the "+"-menu builder) don't re-run the
    /// validator (and its `which::which` / `Path::is_file` probes) on every
    /// menu open.
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

/// Convert a `TerminalProfile` (settings twin) into the runtime `task::Shell`
/// used by the spawn path. If the profile does not specify a
/// `title_override`, the profile name is used so the tab is titled after
/// the profile (matching VSCode's `overrideName` default for generated
/// profiles). The result is always `Shell::WithArguments`, since
/// `title_override` is only expressible on that variant.
pub fn profile_to_task_shell(name: &str, profile: &TerminalProfile) -> Shell {
    Shell::WithArguments {
        program: profile.program.clone(),
        args: profile.args.clone().unwrap_or_default(),
        title_override: profile.title_override.clone().or(Some(name.to_string())),
    }
}

/// A view over the configured-vs-detected shell entries used by the "+"
/// menu. Configured profiles shadow detected shells with the same resolved
/// program path; order is configured-first (preserving IndexMap insertion
/// order), then detected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergedShellEntry {
    /// User-configured profile from `terminal.profiles`.
    Configured {
        name: String,
        profile: TerminalProfile,
    },
    /// Detected shell that is not shadowed by a configured profile.
    Detected(DetectedShell),
}

/// Combine configured profiles with detected shells, dropping any detected
/// entry whose program path collides with a configured profile's program
/// (configured wins the slot).
///
/// Comparison is by the profile's program string resolved against the
/// detected entry's `program` both ways:
/// * exact string match (covers the common "configured `/bin/zsh`,
///   detected `/bin/zsh`" case), and
/// * bare-basename match when the configured program is a bare name like
///   `zsh` (no path separator). This covers "configured `zsh`, detected
///   `/bin/zsh`". Path-bearing configured programs (`/bin/bash`,
///   `./local/shell`) only shadow via exact path equality — otherwise a
///   single `program: "/bin/bash"` profile would shadow every bash on the
///   system, and a future `program: "wsl.exe"` profile would shadow all
///   WSL distros (which differ only by `args`).
pub fn merge_with_configured_profiles(
    detected: Vec<DetectedShell>,
    profiles: &IndexMap<String, TerminalProfile>,
) -> Vec<MergedShellEntry> {
    let configured_paths: std::collections::HashSet<String> =
        profiles.values().map(|p| p.program.clone()).collect();
    let configured_stems: std::collections::HashSet<String> = profiles
        .values()
        .filter_map(|p| {
            // Only bare basenames (no path separator) participate in
            // stem-matching. Path-bearing programs only shadow via exact
            // path equality above.
            if p.program.contains(std::path::MAIN_SEPARATOR) {
                return None;
            }
            // On Windows also treat '/' as a separator for cross-platform
            // configs (a user might write `C:/Program Files/...`).
            if cfg!(windows) && p.program.contains('/') {
                return None;
            }
            Some(p.program.clone())
        })
        .collect();

    let mut entries: Vec<MergedShellEntry> = profiles
        .iter()
        .map(|(name, profile)| MergedShellEntry::Configured {
            name: name.clone(),
            profile: profile.clone(),
        })
        .collect();

    for shell in detected {
        let program_string = shell.program.to_string_lossy().into_owned();
        let stem = shell
            .program
            .file_name()
            .map(|s| s.to_string_lossy().into_owned());
        let shadowed = configured_paths.contains(&program_string)
            || stem.as_ref().is_some_and(|s| configured_stems.contains(s));
        if !shadowed {
            entries.push(MergedShellEntry::Detected(shell));
        }
    }

    entries
}

/// Single menu entry for the "+" PopoverMenu. Produced by
/// [`build_menu_entries`], which filters invalid profiles and applies the
/// escalation threshold. The view layer converts each `MenuEntry` into a
/// `ContextMenuItem` (`action` for configured, `entry` callback for
/// detected).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuEntry {
    /// A configured profile that passed validation. Selecting it dispatches
    /// `workspace::NewTerminal { profile: Some(name) }`.
    Configured { name: String },
    /// A detected shell not shadowed by any configured profile. Selecting it
    /// constructs a `task::Shell::WithArguments` with the detected label as
    /// `title_override` and calls `add_terminal_shell_with`.
    Detected {
        label: String,
        program: String,
        args: Vec<String>,
    },
}

/// Above this many entries, the menu collapses into a single "Select Shell…"
/// submenu entry (keeps the menu manageable when long).
pub const MENU_ESCALATION_THRESHOLD: usize = 8;

/// Outcome of [`build_menu_entries`]: either render entries inline, or
/// collapse into a single "Select Shell…" entry whose submenu holds the
/// same list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuEntryPlan {
    /// `entries.len() <= MENU_ESCALATION_THRESHOLD` — render each entry
    /// directly under a separator.
    Inline { entries: Vec<MenuEntry> },
    /// `entries.len() > MENU_ESCALATION_THRESHOLD` — render a single
    /// "Select Shell…" entry; the same `entries` move into its submenu.
    Collapsed { entries: Vec<MenuEntry> },
}

/// Pure, fs-free computation of the menu contents. Used both by the menu
/// UI (terminal_panel.rs) and by unit tests; the latter assert structure
/// without rendering GPUI elements.
///
/// * `profiles` — the user's `terminal.profiles` map (insertion order
///   preserved).
/// * `detected` — output of `util::shell_detection::detect_available_shells`.
/// * `warnings` — output of `validate_configured_profiles`. Any profile
///   named here is **hidden** from the menu (the user can still spawn it
///   via keymap with a warning).
pub fn build_menu_entries(
    profiles: &IndexMap<String, TerminalProfile>,
    detected: Vec<DetectedShell>,
    warnings: &[ProfileWarning],
) -> MenuEntryPlan {
    let hidden: std::collections::HashSet<&str> =
        warnings.iter().map(|w| w.profile_name.as_str()).collect();

    // Hide invalid configured profiles by filtering the IndexMap before
    // merging with detected shells. This also prevents an invalid profile
    // from shadowing a detected shell at the same path.
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

/// A non-blocking warning about a configured profile whose `program` is
/// neither an existing absolute path nor resolvable on `PATH`. Returned by
/// [`validate_configured_profiles`]; the caller decides how to surface the
/// warning (log line, toast, settings diagnostic).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileWarning {
    pub profile_name: String,
    pub program: String,
    pub reason: String,
}

/// Test-friendly inner validator. The `is_file` and `resolve_on_path`
/// callbacks abstract `Path::is_file()` and `which::which()` so unit tests
/// can run without touching the real filesystem.
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
        let path = Path::new(program);
        let absolute_exists = path.is_absolute() && is_file(path);
        let on_path = !path.is_absolute() && resolve_on_path(program).is_some();
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

/// Validate the configured terminal profiles, returning one
/// [`ProfileWarning`] per profile whose `program` is neither an existing
/// absolute path nor resolvable on `PATH`.
///
/// Never blocks terminal spawn — the existing `TerminalError` notification
/// path handles actual spawn failures. This is purely advisory: invalid
/// programs surface a warning, selection still routes through the regular
/// spawn-error path.
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
        // Surface a warning per profile whose `program` is
        // neither an existing absolute path nor resolvable on `PATH`. This
        // is advisory only — spawn still goes through the normal
        // `TerminalError` path if the program is genuinely missing. The
        // result is cached on `TerminalSettings.profile_warnings` so the
        // "+"-menu builder doesn't have to re-run the validator on every
        // menu open.
        let profile_warnings = validate_configured_profiles(&profiles);
        for warning in &profile_warnings {
            log::warn!(
                "terminal.profiles.{}: {}",
                warning.profile_name,
                warning.reason
            );
        }
        // Precedence: default_profile (if it resolves) > terminal.shell > System.
        // Unknown default_profile name falls through to terminal.shell with a warning,
        // never silently to System.
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
    use settings::TerminalProfile;

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

    fn detected(label: &str, program: &str) -> DetectedShell {
        use util::shell_detection::ShellSource;
        DetectedShell {
            label: label.to_string(),
            program: std::path::PathBuf::from(program),
            args: Vec::new(),
            source: ShellSource::EtcShells,
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
            detected("bash", "/bin/bash"),
            detected("python", "/usr/bin/python"),
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
        let detected = vec![detected("zsh", "/bin/zsh"), detected("bash", "/bin/bash")];
        let merged = merge_with_configured_profiles(detected, &profiles);
        let labels: Vec<String> = merged
            .iter()
            .map(|entry| match entry {
                MergedShellEntry::Configured { name, .. } => name.clone(),
                MergedShellEntry::Detected(shell) => shell.label.clone(),
            })
            .collect();
        // Configured Zsh wins; detected zsh is dropped; detected bash survives.
        assert_eq!(labels, vec!["Zsh".to_string(), "bash".to_string()]);
    }

    #[test]
    fn merge_shadows_detected_when_configured_is_basename_match() {
        // Configured profile uses bare "zsh"; detected shell is at
        // "/bin/zsh". The basename match should shadow the detected entry.
        let profiles = profile_map(&[("MyZsh", "zsh")]);
        let detected = vec![detected("zsh", "/bin/zsh")];
        let merged = merge_with_configured_profiles(detected, &profiles);
        assert_eq!(merged.len(), 1);
        match &merged[0] {
            MergedShellEntry::Configured { name, .. } => assert_eq!(name, "MyZsh"),
            MergedShellEntry::Detected(_) => panic!("basename match should shadow detected"),
        }
    }

    #[test]
    fn merge_does_not_shadow_by_basename_when_configured_is_path() {
        // Configured profile uses an absolute path "/bin/bash"; detected
        // shell is at "/usr/bin/bash" — same stem, different path. The
        // path-bearing configured program should only shadow via exact
        // match (it doesn't here), so both entries survive.
        let profiles = profile_map(&[("Bash", "/bin/bash")]);
        let detected = vec![detected("bash", "/usr/bin/bash")];
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
    fn merge_shadows_detected_when_configured_path_matches_exactly() {
        // Path-bearing configured programs still shadow via exact path
        // equality (the configured program string equals the detected
        // program string).
        let profiles = profile_map(&[("Bash", "/bin/bash")]);
        let detected = vec![detected("bash", "/bin/bash")];
        let merged = merge_with_configured_profiles(detected, &profiles);
        assert_eq!(merged.len(), 1);
        assert!(matches!(merged[0], MergedShellEntry::Configured { .. }));
    }

    #[test]
    fn merge_keeps_detected_when_program_differs() {
        let profiles = profile_map(&[("Zsh", "/bin/zsh")]);
        let detected = vec![
            detected("bash", "/bin/bash"),
            detected("fish", "/usr/bin/fish"),
        ];
        let merged = merge_with_configured_profiles(detected, &profiles);
        // 1 configured + 2 detected = 3 entries.
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
        // Multiple profiles, only one broken — should still return only the
        // one warning, never panic.
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
        // Distinct programs so dedup doesn't collapse them.
        let profiles: Vec<(&str, &str)> = (1..=MENU_ESCALATION_THRESHOLD)
            .map(|i| {
                let n: &'static str = Box::leak(format!("n{i}").into_boxed_str());
                let p: &'static str = Box::leak(format!("/bin/n{i}").into_boxed_str());
                (n, p)
            })
            .collect();
        let plan = build_plan(&profiles, Vec::new(), &[]);
        assert!(
            matches!(plan, MenuEntryPlan::Inline { .. }),
            "exactly at threshold should stay inline"
        );
    }

    #[test]
    fn menu_collapses_above_threshold() {
        let profiles: Vec<(&str, &str)> = (1..=9)
            .map(|i| {
                let n: &'static str = Box::leak(format!("n{i}").into_boxed_str());
                let p: &'static str = Box::leak(format!("/bin/n{i}").into_boxed_str());
                (n, p)
            })
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
    fn menu_places_configured_first_then_detected() {
        let plan = build_plan(
            &[("Zsh", "/bin/zsh"), ("Fish", "/usr/bin/fish")],
            vec![detected_shell("bash", "/bin/bash")],
            &[],
        );
        assert_eq!(entry_labels(&plan), vec!["Zsh", "Fish", "bash"]);
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
        // When a configured profile fails validation, it's filtered out
        // BEFORE merging — so a detected shell at the same path can surface
        // in its place (rather than being shadowed by the broken entry).
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
        // 10 profiles, but 3 are invalid -> 7 visible -> stays inline.
        let profiles: Vec<(&str, &str)> = (1..=10)
            .map(|i| {
                let n: &'static str = Box::leak(format!("n{i}").into_boxed_str());
                let p: &'static str = Box::leak(format!("/bin/n{i}").into_boxed_str());
                (n, p)
            })
            .collect();
        let plan = build_plan(&profiles, Vec::new(), &["n1", "n2", "n3"]);
        match plan {
            MenuEntryPlan::Inline { entries } => {
                assert_eq!(entries.len(), 7, "filtered count should drive threshold");
            }
            _ => panic!("7 visible entries should stay inline"),
        }
    }

    /// `TerminalSettings.profile_warnings` is the cached
    /// validator output. The cache is populated in `from_settings`, but
    /// because constructing a full `SettingsContent` here is heavy, this
    /// test verifies the equivalent contract — that the field exists,
    /// accepts `validate_configured_profiles` output directly, and that
    /// `build_menu_entries` (which the menu now drives off
    /// `settings.profile_warnings`) treats the cached value identically to
    /// a freshly-computed one.
    #[test]
    fn profile_warnings_cache_is_equivalent_to_direct_call() {
        let profiles = profile_map(&[
            ("Good", "/bin/zsh"),
            ("Bad", "/totally/not/real"),
            ("BashOnPath", "bash"),
        ]);
        // Direct call (the value `from_settings` caches).
        let direct = validate_configured_profiles_with(
            &profiles,
            &|p| p.as_os_str() == "/bin/zsh",
            &|program| match program {
                "bash" => Some(std::path::PathBuf::from("/usr/bin/bash")),
                _ => None,
            },
        );
        // Exactly one warning — the bad profile.
        assert_eq!(direct.len(), 1);
        assert_eq!(direct[0].profile_name, "Bad");

        // The cache field is `Vec<ProfileWarning>`; assigning the direct
        // call's output and feeding it to build_menu_entries must hide the
        // bad profile exactly as the live validator did.
        let cached: Vec<ProfileWarning> = direct.clone();
        let plan_cached = build_menu_entries(&profiles, Vec::new(), &cached);
        let plan_direct = build_menu_entries(&profiles, Vec::new(), &direct);
        assert_eq!(
            entry_labels(&plan_cached),
            entry_labels(&plan_direct),
            "cached and direct warnings must drive identical menu plans"
        );
        let labels = entry_labels(&plan_cached);
        assert!(labels.contains(&"Good".to_string()));
        assert!(labels.contains(&"BashOnPath".to_string()));
        assert!(!labels.contains(&"Bad".to_string()));
    }

    #[test]
    fn empty_profile_warnings_cache_yields_all_profiles() {
        let profiles = profile_map(&[("A", "/bin/a"), ("B", "/bin/b")]);
        let cached: Vec<ProfileWarning> = Vec::new();
        let plan = build_menu_entries(&profiles, Vec::new(), &cached);
        assert_eq!(entry_labels(&plan), vec!["A".to_string(), "B".to_string()]);
    }
}
