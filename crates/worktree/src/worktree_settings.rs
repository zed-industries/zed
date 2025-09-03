use std::path::Path;

use anyhow::Context as _;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources, SettingsUi};
use util::paths::PathMatcher;

#[derive(Clone, PartialEq, Eq)]
pub struct WorktreeSettings {
    pub file_scan_inclusions: PathMatcher,
    pub file_scan_exclusions: PathMatcher,
    pub private_files: PathMatcher,
}

impl WorktreeSettings {
    pub fn is_path_private(&self, path: &Path) -> bool {
        path.ancestors()
            .any(|ancestor| self.private_files.is_match(ancestor))
    }

    pub fn is_path_excluded(&self, path: &Path) -> bool {
        path.ancestors()
            .any(|ancestor| self.file_scan_exclusions.is_match(&ancestor))
    }

    pub fn is_path_always_included(&self, path: &Path) -> bool {
        path.ancestors()
            .any(|ancestor| self.file_scan_inclusions.is_match(&ancestor))
    }
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, SettingsUi)]
pub struct WorktreeSettingsContent {
    /// Completely ignore files matching globs from `file_scan_exclusions`. Overrides
    /// `file_scan_inclusions`.
    ///
    /// Default: [
    ///   "**/.git",
    ///   "**/.svn",
    ///   "**/.hg",
    ///   "**/.jj",
    ///   "**/CVS",
    ///   "**/.DS_Store",
    ///   "**/Thumbs.db",
    ///   "**/.classpath",
    ///   "**/.settings"
    /// ]
    #[serde(default)]
    pub file_scan_exclusions: Option<Vec<String>>,

    /// Always include files that match these globs when scanning for files, even if they're
    /// ignored by git. This setting is overridden by `file_scan_exclusions`.
    /// Default: [
    ///  ".env*",
    ///  "docker-compose.*.yml",
    /// ]
    #[serde(default)]
    pub file_scan_inclusions: Option<Vec<String>>,

    /// Treat the files matching these globs as `.env` files.
    /// Default: [ "**/.env*" ]
    pub private_files: Option<Vec<String>>,
}

impl Settings for WorktreeSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = WorktreeSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> anyhow::Result<Self> {
        let result: WorktreeSettingsContent = sources.json_merge()?;
        let mut file_scan_exclusions = result.file_scan_exclusions.unwrap_or_default();
        let mut private_files = result.private_files.unwrap_or_default();
        let mut parsed_file_scan_inclusions: Vec<String> = result
            .file_scan_inclusions
            .unwrap_or_default()
            .iter()
            .flat_map(|glob| {
                Path::new(glob)
                    .ancestors()
                    .map(|a| a.to_string_lossy().into())
            })
            .filter(|p: &String| !p.is_empty())
            .collect();
        file_scan_exclusions.sort();
        private_files.sort();
        parsed_file_scan_inclusions.sort();
        Ok(Self {
            file_scan_exclusions: path_matchers(&file_scan_exclusions, "file_scan_exclusions")?,
            private_files: path_matchers(&private_files, "private_files")?,
            file_scan_inclusions: path_matchers(
                &parsed_file_scan_inclusions,
                "file_scan_inclusions",
            )?,
        })
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut Self::FileContent) {
        if let Some(inclusions) = vscode
            .read_value("files.watcherInclude")
            .and_then(|v| v.as_array())
            .and_then(|v| v.iter().map(|n| n.as_str().map(str::to_owned)).collect())
        {
            if let Some(old) = current.file_scan_inclusions.as_mut() {
                old.extend(inclusions)
            } else {
                current.file_scan_inclusions = Some(inclusions)
            }
        }
        if let Some(exclusions) = vscode
            .read_value("files.watcherExclude")
            .and_then(|v| v.as_array())
            .and_then(|v| v.iter().map(|n| n.as_str().map(str::to_owned)).collect())
        {
            if let Some(old) = current.file_scan_exclusions.as_mut() {
                old.extend(exclusions)
            } else {
                current.file_scan_exclusions = Some(exclusions)
            }
        }
    }
}

fn path_matchers(values: &[String], context: &'static str) -> anyhow::Result<PathMatcher> {
    PathMatcher::new(values).with_context(|| format!("Failed to parse globs from {}", context))
}
