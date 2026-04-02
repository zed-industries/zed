use std::path::Path;
use util::{
    paths::{PathMatcher, PathStyle},
    rel_path::RelPath,
};
use worktree::*;

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

    let vendor_file = RelPath::new(Path::new("vendor/lib/package.js"), PathStyle::local()).unwrap();
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

    let minified_js = RelPath::new(Path::new("dist/bundle.min.js"), PathStyle::local()).unwrap();
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
