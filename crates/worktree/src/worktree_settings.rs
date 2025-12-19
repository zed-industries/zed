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
        let file_scan_exclusions = worktree.file_scan_exclusions.unwrap();
        let file_scan_inclusions = worktree.file_scan_inclusions.unwrap();
        let private_files = worktree.private_files.unwrap().0;
        let hidden_files = worktree.hidden_files.unwrap();
        let read_only_files = worktree.read_only_files.unwrap_or_default();
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
            read_only_files: path_matchers(read_only_files, "read_only_files")
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn make_settings_with_read_only(patterns: &[&str]) -> WorktreeSettings {
        WorktreeSettings {
            project_name: None,
            prevent_sharing_in_public_channels: false,
            file_scan_exclusions: PathMatcher::default(),
            file_scan_inclusions: PathMatcher::default(),
            parent_dir_scan_inclusions: PathMatcher::default(),
            private_files: PathMatcher::default(),
            hidden_files: PathMatcher::default(),
            read_only_files: PathMatcher::new(
                patterns.iter().map(|s| s.to_string()),
                PathStyle::local(),
            )
            .unwrap(),
        }
    }

    #[test]
    fn test_is_path_read_only_with_glob_patterns() {
        let settings = make_settings_with_read_only(&["**/generated/**", "**/*.gen.rs"]);

        let generated_file =
            RelPath::new(Path::new("src/generated/schema.rs"), PathStyle::local()).unwrap();
        assert!(
            settings.is_path_read_only(&generated_file),
            "Files in generated directory should be read-only"
        );

        let gen_rs_file = RelPath::new(Path::new("src/types.gen.rs"), PathStyle::local()).unwrap();
        assert!(
            settings.is_path_read_only(&gen_rs_file),
            "Files with .gen.rs extension should be read-only"
        );

        let regular_file = RelPath::new(Path::new("src/main.rs"), PathStyle::local()).unwrap();
        assert!(
            !settings.is_path_read_only(&regular_file),
            "Regular files should not be read-only"
        );

        let similar_name = RelPath::new(Path::new("src/generator.rs"), PathStyle::local()).unwrap();
        assert!(
            !settings.is_path_read_only(&similar_name),
            "Files with 'generator' in name but not in generated dir should not be read-only"
        );
    }

    #[test]
    fn test_is_path_read_only_with_specific_paths() {
        let settings = make_settings_with_read_only(&["vendor/**", "node_modules/**"]);

        let vendor_file =
            RelPath::new(Path::new("vendor/lib/package.js"), PathStyle::local()).unwrap();
        assert!(
            settings.is_path_read_only(&vendor_file),
            "Files in vendor directory should be read-only"
        );

        let node_modules_file = RelPath::new(
            Path::new("node_modules/lodash/index.js"),
            PathStyle::local(),
        )
        .unwrap();
        assert!(
            settings.is_path_read_only(&node_modules_file),
            "Files in node_modules should be read-only"
        );

        let src_file = RelPath::new(Path::new("src/app.js"), PathStyle::local()).unwrap();
        assert!(
            !settings.is_path_read_only(&src_file),
            "Files in src should not be read-only"
        );
    }

    #[test]
    fn test_is_path_read_only_empty_patterns() {
        let settings = make_settings_with_read_only(&[]);

        let any_file = RelPath::new(Path::new("src/main.rs"), PathStyle::local()).unwrap();
        assert!(
            !settings.is_path_read_only(&any_file),
            "No files should be read-only when patterns are empty"
        );
    }

    #[test]
    fn test_is_path_read_only_with_extension_pattern() {
        let settings = make_settings_with_read_only(&["**/*.lock", "**/*.min.js"]);

        let lock_file = RelPath::new(Path::new("Cargo.lock"), PathStyle::local()).unwrap();
        assert!(
            settings.is_path_read_only(&lock_file),
            "Lock files should be read-only"
        );

        let nested_lock =
            RelPath::new(Path::new("packages/app/yarn.lock"), PathStyle::local()).unwrap();
        assert!(
            settings.is_path_read_only(&nested_lock),
            "Nested lock files should be read-only"
        );

        let minified_js =
            RelPath::new(Path::new("dist/bundle.min.js"), PathStyle::local()).unwrap();
        assert!(
            settings.is_path_read_only(&minified_js),
            "Minified JS files should be read-only"
        );

        let regular_js = RelPath::new(Path::new("src/app.js"), PathStyle::local()).unwrap();
        assert!(
            !settings.is_path_read_only(&regular_js),
            "Regular JS files should not be read-only"
        );
    }
}
