use std::path::Path;

use anyhow::Context as _;
use gpui::App;
use settings::{Settings, SettingsContent};
use util::{
    ResultExt,
    paths::{PathMatcher, PathStyle},
    rel_path::RelPath,
};

#[derive(Clone, PartialEq, Eq)]
pub struct WorktreeSettings {
    pub project_name: Option<String>,
    pub file_scan_inclusions: PathMatcher,
    pub file_scan_exclusions: PathMatcher,
    pub private_files: PathMatcher,
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

    pub fn is_path_always_included(&self, path: &RelPath) -> bool {
        path.ancestors()
            .any(|ancestor| self.file_scan_inclusions.is_match(ancestor.as_std_path()))
    }
}

impl Settings for WorktreeSettings {
    fn from_settings(content: &settings::SettingsContent, _cx: &mut App) -> Self {
        let worktree = content.project.worktree.clone();
        let file_scan_exclusions = worktree.file_scan_exclusions.unwrap();
        let file_scan_inclusions = worktree.file_scan_inclusions.unwrap();
        let private_files = worktree.private_files.unwrap().0;
        let parsed_file_scan_inclusions: Vec<String> = file_scan_inclusions
            .iter()
            .flat_map(|glob| {
                Path::new(glob)
                    .ancestors()
                    .map(|a| a.to_string_lossy().into())
            })
            .filter(|p: &String| !p.is_empty())
            .collect();

        Self {
            project_name: worktree.project_name.filter(|p| !p.is_empty()),
            file_scan_exclusions: path_matchers(file_scan_exclusions, "file_scan_exclusions")
                .log_err()
                .unwrap_or_default(),
            file_scan_inclusions: path_matchers(
                parsed_file_scan_inclusions,
                "file_scan_inclusions",
            )
            .unwrap(),
            private_files: path_matchers(private_files, "private_files")
                .log_err()
                .unwrap_or_default(),
        }
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut SettingsContent) {
        if let Some(inclusions) = vscode
            .read_value("files.watcherInclude")
            .and_then(|v| v.as_array())
            .and_then(|v| v.iter().map(|n| n.as_str().map(str::to_owned)).collect())
        {
            if let Some(old) = current.project.worktree.file_scan_inclusions.as_mut() {
                old.extend(inclusions)
            } else {
                current.project.worktree.file_scan_inclusions = Some(inclusions)
            }
        }
        if let Some(exclusions) = vscode
            .read_value("files.watcherExclude")
            .and_then(|v| v.as_array())
            .and_then(|v| v.iter().map(|n| n.as_str().map(str::to_owned)).collect())
        {
            if let Some(old) = current.project.worktree.file_scan_exclusions.as_mut() {
                old.extend(exclusions)
            } else {
                current.project.worktree.file_scan_exclusions = Some(exclusions)
            }
        }
    }
}

fn path_matchers(mut values: Vec<String>, context: &'static str) -> anyhow::Result<PathMatcher> {
    values.sort();
    PathMatcher::new(values, PathStyle::local())
        .with_context(|| format!("Failed to parse globs from {}", context))
}
