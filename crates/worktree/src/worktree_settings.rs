use std::path::Path;

use anyhow::Context as _;
use settings::{RegisterSetting, ScanSymlinksSetting, Settings};
use util::{
    ResultExt,
    paths::{PathMatcher, PathStyle},
    rel_path::RelPath,
};

#[derive(Clone, PartialEq, Eq, RegisterSetting)]
pub struct WorktreeSettings {
    /// Whether to prevent this project from being shared in public channels.
    pub prevent_sharing_in_public_channels: bool,
    pub file_scan_exclusions: PathMatcher,
    pub file_scan_inclusions: PathMatcher,
    /// This field contains all ancestors of the `file_scan_inclusions`. It's used to
    /// determine whether to terminate worktree scanning for a given dir.
    pub parent_dir_scan_inclusions: PathMatcher,
    pub scan_symlinks: ScanSymlinksSetting,
    pub file_scan_depth: Option<u32>,
    pub private_files: PathMatcher,
    pub hidden_files: PathMatcher,
    pub read_only_files: PathMatcher,
}

impl WorktreeSettings {
    pub fn is_path_private(&self, path: &RelPath) -> bool {
        path.ancestors()
            .any(|ancestor| self.private_files.is_match(ancestor))
    }

    pub fn is_path_excluded(&self, path: &RelPath) -> bool {
        path.ancestors()
            .any(|ancestor| self.file_scan_exclusions.is_match(ancestor))
    }

    pub fn is_path_always_included(&self, path: &RelPath, is_dir: bool) -> bool {
        if is_dir {
            self.parent_dir_scan_inclusions.is_match(path)
        } else {
            self.file_scan_inclusions.is_match(path)
        }
    }

    pub fn is_path_hidden(&self, path: &RelPath) -> bool {
        path.ancestors()
            .any(|ancestor| self.hidden_files.is_match(ancestor))
    }

    pub fn is_path_read_only(&self, path: &RelPath) -> bool {
        self.read_only_files.is_match(path)
    }

    pub fn is_std_path_read_only(&self, path: &Path) -> bool {
        self.read_only_files.is_match_std_path(path)
    }
}

impl Settings for WorktreeSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let worktree = content.project.worktree.clone();
        let file_scan_exclusions = worktree.file_scan_exclusions.unwrap().0;
        let file_scan_inclusions = worktree.file_scan_inclusions.unwrap().0;
        let private_files = worktree.private_files.unwrap().0;
        let hidden_files = worktree.hidden_files.unwrap().0;
        let read_only_files = worktree.read_only_files.unwrap_or_default().0;
        let scan_symlinks = worktree.scan_symlinks.unwrap();
        let (file_scan_inclusions, parent_dir_scan_inclusions) =
            file_scan_inclusion_matchers(file_scan_inclusions);

        Self {
            prevent_sharing_in_public_channels: worktree.prevent_sharing_in_public_channels,
            file_scan_exclusions: valid_path_matchers(file_scan_exclusions, "file_scan_exclusions"),
            parent_dir_scan_inclusions,
            file_scan_inclusions,
            private_files: path_matchers(private_files, "private_files")
                .log_err()
                .unwrap_or_default(),
            hidden_files: valid_path_matchers(hidden_files, "hidden_files"),
            read_only_files: valid_path_matchers(read_only_files, "read_only_files"),
            scan_symlinks,
            file_scan_depth: worktree.file_scan_depth.filter(|depth| *depth > 0),
        }
    }
}

fn valid_path_matchers(mut values: Vec<String>, context: &'static str) -> PathMatcher {
    values.sort();
    PathMatcher::new_lenient(values, PathStyle::local(), |error| {
        log::error!("Failed to compile patterns in `{context}`: {error}");
    })
}

fn file_scan_inclusion_matchers(values: Vec<String>) -> (PathMatcher, PathMatcher) {
    let mut inclusions = Vec::new();
    let mut parent_inclusions = Vec::new();
    for pattern in values {
        let parents: Vec<String> = Path::new(&pattern)
            .ancestors()
            .skip(1)
            .map(|parent| parent.to_string_lossy().into_owned())
            .filter(|parent| !parent.is_empty())
            .collect();
        // Keep each inclusion and its traversal paths together so a rejected
        // pattern cannot leave behind parent directories that are always scanned
        if PathMatcher::new(
            std::iter::once(&pattern).chain(parents.iter()),
            PathStyle::local(),
        )
        .with_context(|| {
            format!(
                "Ignoring pattern {pattern:?} in `file_scan_inclusions` because it or a parent pattern is invalid"
            )
        })
        .log_err()
        .is_none()
        {
            continue;
        }
        inclusions.push(pattern);
        parent_inclusions.extend(parents);
    }
    (
        path_matchers(inclusions, "file_scan_inclusions")
            .log_err()
            .unwrap_or_default(),
        path_matchers(parent_inclusions, "file_scan_inclusions")
            .log_err()
            .unwrap_or_default(),
    )
}

fn path_matchers(mut values: Vec<String>, context: &'static str) -> anyhow::Result<PathMatcher> {
    values.sort();
    PathMatcher::new(values, PathStyle::local())
        .with_context(|| format!("Failed to parse globs from {}", context))
}
