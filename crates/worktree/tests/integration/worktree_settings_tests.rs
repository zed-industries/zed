use serde_json::json;
use settings::{MergeFromTrait as _, RootUserSettings as _, Settings, SettingsContent};
use std::path::Path;
use util::{
    paths::{PathMatcher, PathStyle},
    rel_path::{RelPath, rel_path},
};
use worktree::*;

fn make_settings_with_read_only(patterns: &[&str]) -> WorktreeSettings {
    WorktreeSettings {
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
        scan_symlinks: Default::default(),
        file_scan_depth: None,
    }
}

fn settings_with_patterns(setting: &str, patterns: &[&str]) -> WorktreeSettings {
    zlog::init_test();
    let mut content = SettingsContent::parse_json_with_comments(&settings::default_settings())
        .expect("default settings");
    let inherited: SettingsContent =
        serde_json::from_value(json!({ setting: ["**/.git"] })).expect("inherited settings");
    content.merge_from(&inherited);
    let overrides: SettingsContent =
        serde_json::from_value(json!({ setting: patterns })).expect("override settings");
    content.merge_from(&overrides);
    WorktreeSettings::from_settings(&content)
}

fn assert_file_pattern_settings(setting: &str) {
    for (patterns, expected) in [
        (vec!["**/alpha/**", "zeta/**"], [false, true, true]),
        (vec!["**/alpha/**", "[", "zeta/**"], [false, true, true]),
        (vec!["..."], [true, false, false]),
        (vec!["...", "["], [true, false, false]),
        (
            vec!["**/alpha/**", "...", "[", "zeta/**"],
            [true, true, true],
        ),
        (vec!["[", "{"], [false, false, false]),
        (vec![], [false, false, false]),
    ] {
        let settings = settings_with_patterns(setting, &patterns);
        let matcher = match setting {
            "file_scan_exclusions" => &settings.file_scan_exclusions,
            "file_scan_inclusions" => &settings.file_scan_inclusions,
            "hidden_files" => &settings.hidden_files,
            "read_only_files" => &settings.read_only_files,
            _ => unreachable!(),
        };
        for (path, expected) in [".git", "alpha/file.txt", "zeta/file.txt"]
            .into_iter()
            .zip(expected)
        {
            assert_eq!(
                matcher.is_match(rel_path(path)),
                expected,
                "{setting}: {patterns:?} matching {path}"
            );
        }
        assert!(!matcher.is_match(rel_path("unmatched/file.txt")));
    }
}

#[test]
fn test_invalid_file_scan_exclusions() {
    assert_file_pattern_settings("file_scan_exclusions");
}

#[test]
fn test_invalid_exclusion_addition_preserves_git() {
    let settings = settings_with_patterns("file_scan_exclusions", &["...", "["]);
    assert!(settings.is_path_excluded(rel_path(".git/config")));
    assert!(settings.is_path_excluded(rel_path("nested/.git/config")));
}

#[test]
fn test_invalid_file_scan_inclusions() {
    assert_file_pattern_settings("file_scan_inclusions");
}

#[test]
fn test_invalid_hidden_files() {
    assert_file_pattern_settings("hidden_files");
}

#[test]
fn test_invalid_read_only_files() {
    assert_file_pattern_settings("read_only_files");
}

#[test]
fn test_invalid_direct_inclusions_do_not_include_parents() {
    let settings =
        settings_with_patterns("file_scan_inclusions", &["ignored/[", "valid/nested/**"]);
    assert!(!settings.is_path_always_included(rel_path("ignored"), true));
    assert!(!settings.is_path_always_included(rel_path("ignored/file.txt"), false));
    assert!(settings.is_path_always_included(rel_path("valid"), true));
    assert!(settings.is_path_always_included(rel_path("valid/nested"), true));
    assert!(settings.is_path_always_included(rel_path("valid/nested/file.txt"), false));
}

#[test]
fn test_inclusions_preserve_brace_patterns() {
    for (pattern, files) in [
        (
            "ignored/{one/two,three}/file.rs",
            ["ignored/one/two/file.rs", "ignored/three/file.rs"],
        ),
        (
            "{one/two,three}/file.rs",
            ["one/two/file.rs", "three/file.rs"],
        ),
    ] {
        assert!(PathMatcher::new([pattern], PathStyle::local()).is_ok());
        let settings =
            settings_with_patterns("file_scan_inclusions", &[pattern, "valid/nested/**"]);
        for file in files {
            assert!(settings.is_path_always_included(rel_path(file), false));
            for parent in rel_path(file)
                .ancestors()
                .skip(1)
                .filter(|path| !path.is_empty())
            {
                assert!(
                    settings.is_path_always_included(parent, true),
                    "{pattern}: missing ancestor {parent:?}"
                );
            }
        }
        assert!(settings.is_path_always_included(rel_path("valid"), true));
        assert!(settings.is_path_always_included(rel_path("valid/nested/file.txt"), false));
        assert!(!settings.is_path_always_included(rel_path("unmatched/file.txt"), false));
        assert!(
            !settings.is_path_always_included(rel_path("unmatched"), true),
            "{pattern}: unexpected ancestor"
        );
    }
}

#[test]
fn test_inclusion_parents_close_nested_braces() {
    let settings = settings_with_patterns("file_scan_inclusions", &["{a/{b,c},d}/file.rs"]);
    for directory in ["a", "a/b", "a/c", "d"] {
        assert!(
            settings.is_path_always_included(rel_path(directory), true),
            "missing ancestor {directory}"
        );
    }
    for directory in ["b", "c", "unmatched"] {
        assert!(
            !settings.is_path_always_included(rel_path(directory), true),
            "unexpected ancestor {directory}"
        );
    }
    assert!(settings.is_path_always_included(rel_path("a/b/file.rs"), false));
    assert!(settings.is_path_always_included(rel_path("d/file.rs"), false));
}

#[test]
fn test_inclusion_parents_close_braces_around_character_classes() {
    let settings = settings_with_patterns("file_scan_inclusions", &["{a,[x/y]z}/file.rs"]);
    assert!(settings.is_path_always_included(rel_path("a"), true));
    assert!(settings.is_path_always_included(rel_path("xz"), true));
    assert!(settings.is_path_always_included(rel_path("a/file.rs"), false));
    assert!(settings.is_path_always_included(rel_path("xz/file.rs"), false));
}

#[test]
fn test_inclusion_parents_preserve_literal_braces() {
    let mut patterns = vec!["ignored/[{]one/file.rs"];
    if PathStyle::local().is_posix() {
        patterns.push(r"ignored/\{one/file.rs");
    }
    for pattern in patterns {
        let settings = settings_with_patterns("file_scan_inclusions", &[pattern]);
        assert!(settings.is_path_always_included(rel_path("ignored/{one/file.rs"), false));
        assert!(settings.is_path_always_included(rel_path("ignored/{one"), true));
        assert!(settings.is_path_always_included(rel_path("ignored"), true));
        assert!(!settings.is_path_always_included(rel_path("ignored/other/file.rs"), false));
    }
}

#[cfg(not(windows))]
#[test]
fn test_inclusion_parents_preserve_escaping() {
    let settings = settings_with_patterns("file_scan_inclusions", &[r"{one\/two,three}/file.rs"]);
    assert!(settings.is_path_always_included(rel_path("one/two/file.rs"), false));
    assert!(settings.is_path_always_included(rel_path("one/two"), true));
    assert!(settings.is_path_always_included(rel_path("one"), true));

    let settings = settings_with_patterns("file_scan_inclusions", &[r"ignored\/nested/file.rs"]);
    assert!(settings.is_path_always_included(rel_path("ignored/nested/file.rs"), false));
    assert!(settings.is_path_always_included(rel_path("ignored/nested"), true));
    assert!(settings.is_path_always_included(rel_path("ignored"), true));
}

#[cfg(not(windows))]
#[test]
fn test_inclusion_parents_preserve_character_classes() {
    let settings = settings_with_patterns("file_scan_inclusions", &[r"[a\]cache/file.rs"]);
    assert!(settings.is_path_always_included(rel_path("acache/file.rs"), false));
    assert!(settings.is_path_always_included(rel_path("acache"), true));
    assert!(!settings.is_path_always_included(rel_path("unmatched"), true));
}

#[test]
fn test_private_files_preserves_valid_patterns() {
    let settings = settings_with_patterns("private_files", &["**/private/**"]);
    assert!(settings.is_path_private(rel_path("private/file.txt")));
    assert!(settings.is_path_private(rel_path(".env.local")));

    let settings = settings_with_patterns("private_files", &["**/private/**", "["]);
    assert!(settings.is_path_private(rel_path("private/file.txt")));
    assert!(settings.is_path_private(rel_path(".env.local")));
    assert!(!settings.is_path_private(rel_path("src/main.rs")));
    assert!(PathMatcher::new(["**/private/**", "["], PathStyle::local()).is_err());

    for patterns in [&["["][..], &[][..]] {
        let settings = settings_with_patterns("private_files", patterns);
        assert!(settings.is_path_private(rel_path(".env.local")));
        assert!(!settings.is_path_private(rel_path("private/file.txt")));
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
