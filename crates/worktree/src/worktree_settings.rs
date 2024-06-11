use std::{path::Path, sync::Arc};

use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use util::paths::PathMatcher;

#[derive(Clone, PartialEq, Eq)]
pub struct WorktreeSettings {
    pub file_scan_exclusions: Arc<[PathMatcher]>,
    pub private_files: Arc<[PathMatcher]>,
}

impl WorktreeSettings {
    pub fn is_path_private(&self, path: &Path) -> bool {
        path.ancestors().any(|ancestor| {
            self.private_files
                .iter()
                .any(|matcher| matcher.is_match(&ancestor))
        })
    }

    pub fn is_path_excluded(&self, path: &Path) -> bool {
        path.ancestors().any(|ancestor| {
            self.file_scan_exclusions
                .iter()
                .any(|matcher| matcher.is_match(&ancestor))
        })
    }
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
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
    #[serde(default)]
    pub file_scan_exclusions: Option<Vec<String>>,

    /// Treat the files matching these globs as `.env` files.
    /// Default: [ "**/.env*" ]
    pub private_files: Option<Vec<String>>,
}

impl Settings for WorktreeSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = WorktreeSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut AppContext,
    ) -> anyhow::Result<Self> {
        let result: WorktreeSettingsContent = sources.json_merge()?;
        let mut file_scan_exclusions = result.file_scan_exclusions.unwrap_or_default();
        let mut private_files = result.private_files.unwrap_or_default();
        file_scan_exclusions.sort();
        private_files.sort();
        Ok(Self {
            file_scan_exclusions: path_matchers(&file_scan_exclusions, "file_scan_exclusions"),
            private_files: path_matchers(&private_files, "private_files"),
        })
    }
}

fn path_matchers(values: &[String], context: &'static str) -> Arc<[PathMatcher]> {
    values
        .iter()
        .filter_map(|pattern| {
            PathMatcher::new(pattern)
                .map(Some)
                .unwrap_or_else(|e| {
                    log::error!(
                        "Skipping pattern {pattern} in `{}` project settings due to parsing error: {e:#}", context
                    );
                    None
                })
        })
        .collect::<Vec<_>>()
        .into()
}
