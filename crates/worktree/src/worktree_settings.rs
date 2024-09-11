use std::path::Path;

use anyhow::Context;
use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use util::paths::PathMatcher;

#[derive(Clone, PartialEq, Eq)]
pub struct WorktreeSettings {
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
            .any(|ancestor| self.file_scan_exclusions.is_match(ancestor))
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct WorktreeSettingsContent {
    /// Completely ignore files matching globs from `file_scan_exclusions`
    ///
    /// Default: [
    ///   "**/.git",
    ///   "**/.svn",
    ///   "**/.hg",
    ///   "**/CVS",
    ///   "**/.DS_Store",
    ///   "**/Thumbs.db",
    ///   "**/.classpath",
    ///   "**/.settings"
    /// ]
    pub file_scan_exclusions: Vec<String>,

    /// Treat the files matching these globs as `.env` files.
    /// Default: [ "**/.env*" ]
    pub private_files: Vec<String>,
}

impl Default for WorktreeSettingsContent {
    fn default() -> Self {
        Self {
            private_files: [
                "**/.env*",
                "**/*.pem",
                "**/*.key",
                "**/*.cert",
                "**/*.crt",
                "**/secrets.yml",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            file_scan_exclusions: [
                "**/.git",
                "**/.svn",
                "**/.hg",
                "**/CVS",
                "**/.DS_Store",
                "**/Thumbs.db",
                "**/.classpath",
                "**/.settings",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        }
    }
}

impl Settings for WorktreeSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = WorktreeSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut AppContext,
    ) -> anyhow::Result<Self> {
        let result: WorktreeSettingsContent = sources.json_merge()?;
        let mut file_scan_exclusions = result.file_scan_exclusions;
        let mut private_files = result.private_files;
        file_scan_exclusions.sort();
        private_files.sort();
        Ok(Self {
            file_scan_exclusions: path_matchers(&file_scan_exclusions, "file_scan_exclusions")?,
            private_files: path_matchers(&private_files, "private_files")?,
        })
    }
}

fn path_matchers(values: &[String], context: &'static str) -> anyhow::Result<PathMatcher> {
    PathMatcher::new(values).with_context(|| format!("Failed to parse globs from {}", context))
}
