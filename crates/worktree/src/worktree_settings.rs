use std::path::Path;

use anyhow::Context as _;
use settings::{RegisterSetting, Settings};
use util::{
    ResultExt,
    paths::{PathMatcher, PathStyle},
    rel_path::RelPath,
};

#[derive(Clone, PartialEq, Eq, RegisterSetting)]
pub struct WorktreeSettings {
    pub project_name: Option<String>,
    /// Whether to prevent this project from being shared in public channels.
    pub prevent_sharing_in_public_channels: bool,
    pub file_scan_exclusions: PathMatcher,
    pub file_scan_inclusions: PathMatcher,
    /// This field contains all ancestors of the `file_scan_inclusions`. It's used to
    /// determine whether to terminate worktree scanning for a given dir.
    pub parent_dir_scan_inclusions: PathMatcher,
    pub private_files: PathMatcher,
    pub hidden_files: PathMatcher,
}

impl WorktreeSettings {
    pub fn is_path_private(&self, path: &RelPath) -> bool {
        path.ancestors()
            .any(|ancestor| self.private_files.is_match(ancestor.as_std_path()))
    }

    pub fn is_path_excluded(&self, path: &RelPath) -> bool {
        path.ancestors()
            .any(|ancestor| self.file_scan_exclusions.is_match(ancestor.as_std_path()))
    }

    pub fn is_path_always_included(&self, path: &RelPath, is_dir: bool) -> bool {
        if is_dir {
            self.parent_dir_scan_inclusions.is_match(path.as_std_path())
        } else {
            self.file_scan_inclusions.is_match(path.as_std_path())
        }
    }

    pub fn is_path_hidden(&self, path: &RelPath) -> bool {
        path.ancestors()
            .any(|ancestor| self.hidden_files.is_match(ancestor.as_std_path()))
    }
}

impl Settings for WorktreeSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let worktree = content.project.worktree.clone();
        let file_scan_exclusions = worktree.file_scan_exclusions.unwrap();
        let file_scan_inclusions = worktree.file_scan_inclusions.unwrap();
        let private_files = worktree.private_files.unwrap().0;
        let hidden_files = worktree.hidden_files.unwrap();
        let parsed_file_scan_inclusions: Vec<String> = file_scan_inclusions
            .iter()
            .flat_map(|glob| {
                Path::new(glob)
                    .ancestors()
                    .skip(1)
                    .map(|a| a.to_string_lossy().into())
            })
            .filter(|p: &String| !p.is_empty())
            .collect();

        Self {
            project_name: worktree.project_name,
            prevent_sharing_in_public_channels: worktree.prevent_sharing_in_public_channels,
            file_scan_exclusions: path_matchers(file_scan_exclusions, "file_scan_exclusions")
                .log_err()
                .unwrap_or_default(),
            parent_dir_scan_inclusions: path_matchers(
                parsed_file_scan_inclusions,
                "file_scan_inclusions",
            )
            .unwrap(),
            file_scan_inclusions: path_matchers(file_scan_inclusions, "file_scan_inclusions")
                .unwrap(),
            private_files: path_matchers(private_files, "private_files")
                .log_err()
                .unwrap_or_default(),
            hidden_files: path_matchers(hidden_files, "hidden_files")
                .log_err()
                .unwrap_or_default(),
        }
    }
}

fn path_matchers(mut values: Vec<String>, context: &'static str) -> anyhow::Result<PathMatcher> {
    values.sort();
    PathMatcher::new(values, PathStyle::local())
        .with_context(|| format!("Failed to parse globs from {}", context))
}
