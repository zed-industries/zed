use std::path::Path;

use settings::{RegisterSetting, ScanSymlinksSetting, Settings};
use util::{
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
            private_files: valid_path_matchers(private_files, "private_files"),
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

fn file_scan_inclusion_matchers(mut values: Vec<String>) -> (PathMatcher, PathMatcher) {
    values.sort();
    let mut errors = Vec::new();
    let inclusions = PathMatcher::new_lenient(&values, PathStyle::local(), |error| {
        errors.push(error);
    });
    let parent_inclusions = values
        .iter()
        .filter(|pattern| {
            !errors.iter().any(|error| {
                error
                    .glob()
                    .is_none_or(|invalid| invalid == pattern.as_str())
            })
        })
        .flat_map(|pattern| inclusion_parent_patterns(pattern, PathStyle::local()))
        .collect();
    for error in errors {
        log::error!("Failed to compile patterns in `file_scan_inclusions`: {error}");
    }
    (
        inclusions,
        valid_path_matchers(parent_inclusions, "file_scan_inclusions"),
    )
}

fn inclusion_parent_patterns(pattern: &str, path_style: PathStyle) -> Vec<String> {
    let mut parents = Vec::new();
    let mut open_braces = 0_usize;
    let mut class_start = None;
    let mut escaped = false;
    for (index, character) in pattern.char_indices() {
        let is_separator = path_style.separators_ch().contains(&character);
        let mut split_index = index;
        if class_start.is_none() {
            if escaped {
                escaped = false;
                if !is_separator {
                    continue;
                }
                split_index -= 1;
            } else if character == '\\' && path_style.is_posix() {
                escaped = true;
                continue;
            }
        }
        if let Some(start) = class_start {
            let first_content = start
                + 1
                + usize::from(matches!(
                    pattern.as_bytes().get(start + 1),
                    Some(b'!' | b'^')
                ));
            if character == ']' && index > first_content {
                class_start = None;
            }
        } else {
            match character {
                '[' => class_start = Some(index),
                '{' => open_braces += 1,
                '}' => open_braces = open_braces.saturating_sub(1),
                _ => {}
            }
        }
        if is_separator {
            let closing_braces = "}".repeat(open_braces);
            if let Some(start) = class_start {
                parents.push(format!("{}*{closing_braces}", &pattern[..start]));
            } else if open_braces > 0 {
                parents.push(format!("{}{closing_braces}", &pattern[..split_index]));
            } else if split_index > 0 {
                parents.push(pattern[..split_index].to_string());
            }
        }
    }
    parents.sort();
    parents.dedup();
    parents
}
