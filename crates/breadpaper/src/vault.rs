use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::notes::NoteKind;

pub const VAULT_MARKER_DIR: &str = ".breadpaper";
pub const VAULT_CONFIG_FILE: &str = "config.toml";
pub const WELCOME_FILE: &str = "welcome.md";

pub const DEFAULT_CONFIG_TOML: &str = r#"schema = 1

[daily]
dir      = "daily"           # daily notes dir, relative to vault root
filename = "YYYY-MM-DD"      # moment-style date format; ".md" is appended
template = "templates/daily.md"

[weekly]
dir      = "weekly"          # weekly notes dir, relative to vault root
filename = "GGGG-[W]WW"      # ISO week year + week number, e.g. 2026-W30
template = "templates/weekly.md"

[[areas.installed]]
id      = "timeline"
enabled = true
version = 1
"#;

pub const DEFAULT_DAILY_TEMPLATE: &str = r#"# {{date:dddd, MMMM D, YYYY}}

## Journal

## Day planner

## Personal
"#;

pub const DEFAULT_WEEKLY_TEMPLATE: &str = r#"# Week {{date:W}}, {{date:GGGG}}

## Goals

## Notes

## Week review
"#;

pub const DEFAULT_WELCOME: &str = r#"# Welcome to BreadPaper

This folder is your **vault** — a plain folder of Markdown files that belongs to you.

## The Timeline

Open the **Timeline** panel in the left sidebar and click an entry:

- **Today** / **Yesterday** open daily notes, created from `templates/daily.md`.
- **This Week** / **Last Week** open weekly notes, created from `templates/weekly.md`.

The same entries (plus **Tomorrow**) are available from the command palette as
`breadpaper: open today`, `breadpaper: open tomorrow`, and friends.

Notes live in `daily/` and `weekly/`, one file per day or week. Existing notes
are only ever opened — never overwritten.

## Make it yours

Everything is a plain file you can edit:

- `templates/daily.md` and `templates/weekly.md` — the templates for new notes.
  Tokens like `{{date:dddd, MMMM D, YYYY}}`, `{{time}}`, and `{{title}}` are
  filled in when a note is created.
- `.breadpaper/config.toml` — where notes go and how they are named.

This file is just a note, too. Edit it, or delete it once you've found your feet.
"#;

/// The parsed shape of `config.toml`; every field is optional so partially
/// specified sections fall back to defaults. Also `Serialize` so the areas
/// registry can rewrite the file: only fields the user actually set are
/// re-emitted (comments are not preserved).
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
struct VaultConfigContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<u32>,
    #[serde(skip_serializing_if = "NotesConfigContent::is_unset")]
    daily: NotesConfigContent,
    #[serde(skip_serializing_if = "NotesConfigContent::is_unset")]
    weekly: NotesConfigContent,
    #[serde(skip_serializing_if = "HistoryConfigContent::is_unset")]
    history: HistoryConfigContent,
    #[serde(skip_serializing_if = "AreasConfigContent::is_unset")]
    areas: AreasConfigContent,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
struct NotesConfigContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template: Option<String>,
}

impl NotesConfigContent {
    fn resolve(self, defaults: NotesConfig) -> NotesConfig {
        NotesConfig {
            dir: self.dir.unwrap_or(defaults.dir),
            filename: self.filename.unwrap_or(defaults.filename),
            template: self.template.unwrap_or(defaults.template),
        }
    }

    fn is_unset(&self) -> bool {
        self.dir.is_none() && self.filename.is_none() && self.template.is_none()
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
struct HistoryConfigContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_debounce_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heartbeat_minutes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_file_bytes: Option<u64>,
}

impl HistoryConfigContent {
    fn resolve(self) -> HistoryConfig {
        HistoryConfig {
            enabled: self.enabled.unwrap_or(true),
            idle_debounce: Duration::from_secs(self.idle_debounce_seconds.unwrap_or(20)),
            heartbeat: Duration::from_secs(self.heartbeat_minutes.unwrap_or(5) * 60),
            max_file_bytes: self.max_file_bytes.unwrap_or(2_000_000),
        }
    }

    fn is_unset(&self) -> bool {
        self.enabled.is_none()
            && self.idle_debounce_seconds.is_none()
            && self.heartbeat_minutes.is_none()
            && self.max_file_bytes.is_none()
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
struct AreasConfigContent {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    installed: Vec<InstalledAreaContent>,
}

impl AreasConfigContent {
    fn resolve(self) -> AreasConfig {
        AreasConfig {
            installed: self
                .installed
                .into_iter()
                .map(InstalledAreaContent::resolve)
                .collect(),
        }
    }

    fn is_unset(&self) -> bool {
        self.installed.is_empty()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct InstalledAreaContent {
    id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<u32>,
}

impl InstalledAreaContent {
    fn resolve(self) -> InstalledArea {
        InstalledArea {
            id: self.id,
            enabled: self.enabled.unwrap_or(true),
            version: self.version.unwrap_or(1),
        }
    }
}

/// The `[[areas.installed]]` registry (the V3 Areas spec §5.4). Array order is
/// display order in the panel's Areas section.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AreasConfig {
    pub installed: Vec<InstalledArea>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InstalledArea {
    pub id: String,
    pub enabled: bool,
    pub version: u32,
}

impl InstalledArea {
    fn into_content(self) -> InstalledAreaContent {
        InstalledAreaContent {
            id: self.id,
            enabled: Some(self.enabled),
            version: Some(self.version),
        }
    }
}

/// Settings for the invisible checkpoint history (the `[history]` table).
/// Every field has a default, so the table may be entirely absent.
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryConfig {
    pub enabled: bool,
    pub idle_debounce: Duration,
    pub heartbeat: Duration,
    pub max_file_bytes: u64,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        HistoryConfigContent::default().resolve()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VaultConfig {
    pub schema: u32,
    pub daily: NotesConfig,
    pub weekly: NotesConfig,
    pub history: HistoryConfig,
    pub areas: AreasConfig,
}

impl Default for VaultConfig {
    fn default() -> Self {
        VaultConfigContent::default().resolve()
    }
}

impl VaultConfigContent {
    fn resolve(self) -> VaultConfig {
        VaultConfig {
            schema: self.schema.unwrap_or(1),
            daily: self.daily.resolve(NotesConfig::daily_default()),
            weekly: self.weekly.resolve(NotesConfig::weekly_default()),
            history: self.history.resolve(),
            areas: self.areas.resolve(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NotesConfig {
    pub dir: String,
    pub filename: String,
    pub template: String,
}

impl NotesConfig {
    fn daily_default() -> Self {
        Self {
            dir: "daily".to_string(),
            filename: "YYYY-MM-DD".to_string(),
            template: "templates/daily.md".to_string(),
        }
    }

    fn weekly_default() -> Self {
        Self {
            dir: "weekly".to_string(),
            filename: "GGGG-[W]WW".to_string(),
            template: "templates/weekly.md".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vault {
    pub root: PathBuf,
    pub config: VaultConfig,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VaultStatus {
    /// The folder has no `.breadpaper/` marker.
    NotAVault,
    /// The marker exists but `config.toml` could not be read or parsed.
    Invalid { error: String },
    Valid(Vault),
}

impl Vault {
    /// Determines whether `root` is a BreadPaper vault and loads its config.
    pub fn detect(root: &Path) -> VaultStatus {
        let config_path = root.join(VAULT_MARKER_DIR).join(VAULT_CONFIG_FILE);
        if !config_path.is_file() {
            return VaultStatus::NotAVault;
        }
        match fs::read_to_string(&config_path) {
            Ok(contents) => match toml::from_str::<VaultConfigContent>(&contents) {
                Ok(content) => VaultStatus::Valid(Vault {
                    root: root.to_path_buf(),
                    config: content.resolve(),
                }),
                Err(error) => VaultStatus::Invalid {
                    error: format!("failed to parse {}: {}", config_path.display(), error),
                },
            },
            Err(error) => VaultStatus::Invalid {
                error: format!("failed to read {}: {}", config_path.display(), error),
            },
        }
    }

    pub fn notes_config(&self, kind: NoteKind) -> &NotesConfig {
        match kind {
            NoteKind::Daily => &self.config.daily,
            NoteKind::Weekly => &self.config.weekly,
        }
    }

    pub fn note_path(&self, kind: NoteKind, date: chrono::NaiveDate) -> PathBuf {
        let config = self.notes_config(kind);
        let stem = crate::notes::format_date(date, &config.filename);
        self.root.join(&config.dir).join(format!("{stem}.md"))
    }

    pub fn template_path(&self, kind: NoteKind) -> PathBuf {
        self.root.join(&self.notes_config(kind).template)
    }
}

/// Creates `path`'s parent directories and writes `contents`, unless the file
/// already exists — scaffolding and Area materialization never clobber user
/// data.
pub(crate) fn write_if_missing(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }
    if !path.exists() {
        fs::write(path, contents).with_context(|| format!("writing {}", path.display()))?;
    }
    Ok(())
}

/// Rewrites `.breadpaper/config.toml` with `mutate` applied to the
/// `[[areas.installed]]` registry. Re-serializes the known config schema, so
/// only fields present in the file are kept and comments are dropped.
/// Blocking I/O — call from a background thread.
pub fn update_areas_registry(
    root: &Path,
    mutate: impl FnOnce(&mut Vec<InstalledArea>),
) -> Result<()> {
    let config_path = root.join(VAULT_MARKER_DIR).join(VAULT_CONFIG_FILE);
    let raw = fs::read_to_string(&config_path)
        .with_context(|| format!("reading {}", config_path.display()))?;
    let mut content: VaultConfigContent = toml::from_str(&raw)
        .with_context(|| format!("parsing {}", config_path.display()))?;

    let mut installed: Vec<InstalledArea> = content
        .areas
        .installed
        .drain(..)
        .map(InstalledAreaContent::resolve)
        .collect();
    mutate(&mut installed);
    content.areas.installed = installed
        .into_iter()
        .map(InstalledArea::into_content)
        .collect();

    let serialized =
        toml::to_string_pretty(&content).context("serializing vault config")?;
    fs::write(&config_path, serialized)
        .with_context(|| format!("writing {}", config_path.display()))?;
    Ok(())
}

/// Writes the default vault structure into `root`, creating it if needed.
///
/// Only files that don't exist yet are written, so scaffolding into a non-empty
/// folder (the "Create vault here" action) never clobbers user data.
pub fn scaffold_vault(root: &Path) -> Result<()> {
    let config_path = root.join(VAULT_MARKER_DIR).join(VAULT_CONFIG_FILE);
    // The Timeline Area ships pre-installed, but only when this scaffold is
    // creating the vault: an existing config.toml wouldn't register the Area,
    // and materializing unregistered files would clutter an existing vault.
    let install_default_areas = !config_path.exists();
    write_if_missing(&config_path, DEFAULT_CONFIG_TOML)?;
    fs::create_dir_all(root.join("daily")).context("creating daily dir")?;
    fs::create_dir_all(root.join("weekly")).context("creating weekly dir")?;
    write_if_missing(&root.join("templates").join("daily.md"), DEFAULT_DAILY_TEMPLATE)?;
    write_if_missing(
        &root.join("templates").join("weekly.md"),
        DEFAULT_WEEKLY_TEMPLATE,
    )?;
    write_if_missing(&root.join(WELCOME_FILE), DEFAULT_WELCOME)?;
    if install_default_areas {
        let timeline = crate::areas::catalog_area(crate::areas::TIMELINE_AREA_ID)?
            .context("the bundled Timeline Area is missing from the catalog")?;
        crate::areas::materialize_area(root, &timeline)?;
    }
    Ok(())
}

/// The default location for the vault scaffolded on first run.
pub fn default_vault_path() -> PathBuf {
    util::paths::home_dir().join("BreadPaper")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn detect_non_vault() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(Vault::detect(dir.path()), VaultStatus::NotAVault);
    }

    #[test]
    fn detect_invalid_config() {
        let dir = tempfile::tempdir().unwrap();
        let marker = dir.path().join(VAULT_MARKER_DIR);
        fs::create_dir_all(&marker).unwrap();
        fs::write(marker.join(VAULT_CONFIG_FILE), "not [valid toml").unwrap();
        match Vault::detect(dir.path()) {
            VaultStatus::Invalid { .. } => {}
            other => panic!("expected invalid, got {other:?}"),
        }
    }

    #[test]
    fn scaffold_then_detect_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        scaffold_vault(dir.path()).unwrap();
        let vault = match Vault::detect(dir.path()) {
            VaultStatus::Valid(vault) => vault,
            other => panic!("expected valid vault, got {other:?}"),
        };
        assert_eq!(vault.config.daily, VaultConfig::default().daily);
        assert_eq!(vault.config.weekly, VaultConfig::default().weekly);
        assert_eq!(vault.config.history, VaultConfig::default().history);
        assert_eq!(
            vault.config.areas.installed,
            vec![InstalledArea {
                id: "timeline".to_string(),
                enabled: true,
                version: 1,
            }]
        );
        assert!(dir.path().join("daily").is_dir());
        assert!(dir.path().join("weekly").is_dir());
        assert!(dir.path().join("templates/daily.md").is_file());
        assert!(dir.path().join("templates/weekly.md").is_file());
        assert!(dir.path().join(WELCOME_FILE).is_file());

        let date = NaiveDate::from_ymd_opt(2026, 7, 20).unwrap();
        assert_eq!(
            vault.note_path(NoteKind::Daily, date),
            dir.path().join("daily/2026-07-20.md")
        );
        assert_eq!(
            vault.note_path(NoteKind::Weekly, date),
            dir.path().join("weekly/2026-W30.md")
        );
    }

    #[test]
    fn scaffold_preserves_existing_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join(WELCOME_FILE), "my own welcome").unwrap();
        scaffold_vault(dir.path()).unwrap();
        assert_eq!(
            fs::read_to_string(dir.path().join(WELCOME_FILE)).unwrap(),
            "my own welcome"
        );
    }

    #[test]
    fn history_config_parsing() {
        let dir = tempfile::tempdir().unwrap();
        let marker = dir.path().join(VAULT_MARKER_DIR);
        fs::create_dir_all(&marker).unwrap();
        fs::write(
            marker.join(VAULT_CONFIG_FILE),
            "schema = 1\n[history]\nenabled = false\nidle_debounce_seconds = 5\n",
        )
        .unwrap();
        match Vault::detect(dir.path()) {
            VaultStatus::Valid(vault) => {
                assert!(!vault.config.history.enabled);
                assert_eq!(vault.config.history.idle_debounce, Duration::from_secs(5));
                // Unspecified keys keep their defaults.
                assert_eq!(vault.config.history.heartbeat, Duration::from_secs(300));
                assert_eq!(vault.config.history.max_file_bytes, 2_000_000);
            }
            other => panic!("expected valid vault, got {other:?}"),
        }
    }

    #[test]
    fn update_areas_registry_preserves_other_config() {
        let dir = tempfile::tempdir().unwrap();
        let marker = dir.path().join(VAULT_MARKER_DIR);
        fs::create_dir_all(&marker).unwrap();
        fs::write(
            marker.join(VAULT_CONFIG_FILE),
            "schema = 1\n\n[daily]\ndir = \"notes/daily\"\n\n[history]\nenabled = false\n",
        )
        .unwrap();

        update_areas_registry(dir.path(), |installed| {
            installed.push(InstalledArea {
                id: "timeline".to_string(),
                enabled: true,
                version: 1,
            });
        })
        .unwrap();
        update_areas_registry(dir.path(), |installed| {
            if let Some(entry) = installed.first_mut() {
                entry.enabled = false;
            }
        })
        .unwrap();

        match Vault::detect(dir.path()) {
            VaultStatus::Valid(vault) => {
                assert_eq!(vault.config.daily.dir, "notes/daily");
                // Unset fields must stay unset, not be materialized as defaults.
                assert_eq!(vault.config.daily.filename, "YYYY-MM-DD");
                assert!(!vault.config.history.enabled);
                assert_eq!(vault.config.weekly, NotesConfig::weekly_default());
                assert_eq!(
                    vault.config.areas.installed,
                    vec![InstalledArea {
                        id: "timeline".to_string(),
                        enabled: false,
                        version: 1,
                    }]
                );
            }
            other => panic!("expected valid vault, got {other:?}"),
        }
        let raw = fs::read_to_string(marker.join(VAULT_CONFIG_FILE)).unwrap();
        assert!(!raw.contains("[weekly]"), "unset section reappeared: {raw}");
    }

    #[test]
    fn partial_config_uses_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let marker = dir.path().join(VAULT_MARKER_DIR);
        fs::create_dir_all(&marker).unwrap();
        fs::write(
            marker.join(VAULT_CONFIG_FILE),
            "schema = 1\n[daily]\ndir = \"notes/daily\"\n",
        )
        .unwrap();
        match Vault::detect(dir.path()) {
            VaultStatus::Valid(vault) => {
                assert_eq!(vault.config.daily.dir, "notes/daily");
                assert_eq!(vault.config.daily.filename, "YYYY-MM-DD");
                // A config written before weekly notes existed still works.
                assert_eq!(vault.config.weekly, NotesConfig::weekly_default());
            }
            other => panic!("expected valid vault, got {other:?}"),
        }
    }
}
