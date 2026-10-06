use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use feature_flags::FeatureFlagAppExt as _;
use fs::{FakeFs, Fs as _};
use futures::FutureExt as _;
use futures::channel::oneshot;
use gpui::{AppContext as _, Entity, TestAppContext};
use parking_lot::Mutex;
use rand::rngs::StdRng;
use rand::{Rng as _, SeedableRng as _};
use serde_json::json;
use settings::SettingsStore;

use crate::classifier::DocumentClassifier;
use crate::engine::{
    ConflictBatch, FirstSyncChoice, FirstSyncPreview, MAX_PUSH_ATTEMPTS, PauseReason,
    PendingDecision, SYNC_DEBOUNCE, SettingsSyncEngine, SettingsSyncEvent, SyncActivity, SyncState,
    SyncedDocument, load_state,
};
use crate::merge::{
    BUILT_IN_EXCLUSIONS, ExclusionPattern, ExclusionSet, PathMap, apply_ops_to_text,
    apply_target_to_text, diff_paths, flatten_doc, merge_three_way, unflatten,
};
use crate::server::{FAKE_ACCOUNT_ID, FakeSettingsSyncBackend, FakeSettingsSyncServer};
use crate::strategy::SettingsJsonStrategy;
use crate::sync_path::SyncPath;

#[test]
fn test_flatten_descends_into_schema_containers() {
    let classifier = DocumentClassifier::for_user_settings();
    let doc = json!({
        "buffer_font_size": 15,
        "macos": { "buffer_font_size": 16 },
        "windows": { "buffer_font_size": 14 },
        "languages": { "Rust": { "tab_size": 4 } },
        "profiles": { "Presenting": { "settings": { "buffer_font_size": 22 } } },
        "preview": { "telemetry": { "metrics": false } },
        "log": { "client": "warn" },
    });
    let paths = flatten_doc(&classifier, &doc);
    let flattened = paths
        .iter()
        .map(|(path, value)| (path.to_string(), value.clone()))
        .collect::<Vec<_>>();
    assert_eq!(
        flattened,
        vec![
            ("/buffer_font_size".to_string(), json!(15)),
            ("/languages/Rust/tab_size".to_string(), json!(4)),
            ("/log/client".to_string(), json!("warn")),
            ("/macos/buffer_font_size".to_string(), json!(16)),
            ("/preview/telemetry/metrics".to_string(), json!(false)),
            (
                "/profiles/Presenting/settings/buffer_font_size".to_string(),
                json!(22)
            ),
            ("/windows/buffer_font_size".to_string(), json!(14)),
        ]
    );
}

#[test]
fn test_flatten_keeps_enum_objects_atomic() {
    let classifier = DocumentClassifier::for_user_settings();
    let doc = json!({
        "languages": {
            "Rust": {
                "formatter": { "external": { "command": "rustfmt", "arguments": [] } }
            }
        }
    });
    let paths = flatten_doc(&classifier, &doc);
    assert_eq!(
        paths.get(&SyncPath::from_segments(["languages", "Rust", "formatter"])),
        Some(&json!({ "external": { "command": "rustfmt", "arguments": [] } }))
    );
}

#[test]
fn test_flatten_keeps_unknown_keys_atomic() {
    let classifier = DocumentClassifier::for_user_settings();
    let doc = json!({
        "some_future_setting": { "nested": { "a": 1 } },
    });
    let paths = flatten_doc(&classifier, &doc);
    assert_eq!(
        paths.get(&SyncPath::from_segments(["some_future_setting"])),
        Some(&json!({ "nested": { "a": 1 } }))
    );
}

#[test]
fn test_flatten_arrays_are_leaves() {
    let classifier = DocumentClassifier::for_user_settings();
    let doc = json!({
        "ui_font_fallbacks": ["Menlo", "Monaco"],
    });
    let paths = flatten_doc(&classifier, &doc);
    assert_eq!(
        paths.get(&SyncPath::from_segments(["ui_font_fallbacks"])),
        Some(&json!(["Menlo", "Monaco"]))
    );
}

#[test]
fn test_unflatten_round_trip() {
    let classifier = DocumentClassifier::for_user_settings();
    let doc = json!({
        "buffer_font_size": 15,
        "macos": { "buffer_font_size": 16 },
        "languages": { "Rust": { "tab_size": 4 } },
    });
    let paths = flatten_doc(&classifier, &doc);
    assert_eq!(unflatten(&paths), doc);
}

#[test]
fn test_merge_three_way_table() {
    let base = path_map(&[
        ("/a", json!(1)),
        ("/b", json!(1)),
        ("/c", json!(1)),
        ("/d", json!(1)),
        ("/e", json!(1)),
        ("/f", json!(1)),
        ("/g", json!(1)),
    ]);
    let local = path_map(&[
        ("/a", json!(1)),
        ("/b", json!(2)),
        ("/c", json!(1)),
        ("/d", json!(3)),
        ("/e", json!(4)),
        ("/g", json!(2)),
        ("/h", json!(9)),
    ]);
    let remote = path_map(&[
        ("/a", json!(1)),
        ("/b", json!(1)),
        ("/c", json!(2)),
        ("/d", json!(3)),
        ("/e", json!(5)),
        ("/f", json!(1)),
        ("/i", json!(8)),
    ]);

    let merge = merge_three_way(&base, &local, &remote);

    assert_eq!(
        merge.merged,
        path_map(&[
            ("/a", json!(1)),
            ("/b", json!(2)),
            ("/c", json!(2)),
            ("/d", json!(3)),
            ("/e", json!(5)),
            ("/h", json!(9)),
            ("/i", json!(8)),
        ])
    );
    let conflict_paths = merge
        .conflicts
        .iter()
        .map(|conflict| conflict.path.to_string())
        .collect::<Vec<_>>();
    assert_eq!(conflict_paths, vec!["/e".to_string(), "/g".to_string()]);
}

#[test]
fn test_merge_delete_vs_change_conflicts() {
    let base = path_map(&[("/a", json!(1)), ("/b", json!(1))]);
    let local = path_map(&[("/b", json!(2))]);
    let remote = path_map(&[("/a", json!(2))]);

    let merge = merge_three_way(&base, &local, &remote);

    assert_eq!(merge.merged, path_map(&[("/a", json!(2))]));
    let conflict_paths = merge
        .conflicts
        .iter()
        .map(|conflict| conflict.path.to_string())
        .collect::<Vec<_>>();
    assert_eq!(conflict_paths, vec!["/a".to_string(), "/b".to_string()]);
}

#[test]
fn test_merge_no_base_unions_disjoint_paths() {
    let local = path_map(&[("/a", json!(1)), ("/shared", json!("local"))]);
    let remote = path_map(&[("/b", json!(2)), ("/shared", json!("remote"))]);

    let merge = merge_three_way(&PathMap::default(), &local, &remote);

    assert_eq!(
        merge.merged,
        path_map(&[
            ("/a", json!(1)),
            ("/b", json!(2)),
            ("/shared", json!("remote")),
        ])
    );
    assert_eq!(merge.conflicts.len(), 1);
    assert_eq!(merge.conflicts[0].path.to_string(), "/shared");
}

#[test]
fn test_merge_is_idempotent_when_idle() {
    let doc = path_map(&[("/a", json!(1)), ("/b", json!({"x": 2}))]);
    let merge = merge_three_way(&doc, &doc, &doc);
    assert_eq!(merge.merged, doc);
    assert_eq!(merge.conflicts, Vec::new());
}

#[test]
fn test_merge_three_way_randomized_properties() {
    let path_pool = ["/a", "/b", "/c", "/d/x", "/d/y", "/e/f/g", "/h", "/i"];
    let value_pool = [
        json!(1),
        json!(2),
        json!("x"),
        json!(true),
        json!(null),
        json!({ "k": 1 }),
        json!([1, 2]),
    ];

    for seed in 0..512 {
        let mut rng = StdRng::seed_from_u64(seed);
        let base = random_path_map(&mut rng, &path_pool, &value_pool);
        let local = mutated_path_map(&mut rng, &base, &path_pool, &value_pool);
        let remote = mutated_path_map(&mut rng, &base, &path_pool, &value_pool);

        let merge = merge_three_way(&base, &local, &remote);
        for pointer in path_pool {
            let path = SyncPath::parse(pointer).unwrap();
            let base_value = base.get(&path);
            let local_value = local.get(&path);
            let remote_value = remote.get(&path);
            let merged_value = merge.merged.get(&path);
            let conflicted = merge.conflicts.iter().any(|conflict| conflict.path == path);

            let expect_conflict = local_value != base_value
                && remote_value != base_value
                && local_value != remote_value;
            assert_eq!(conflicted, expect_conflict, "seed {seed}, path {pointer}");
            if local_value == remote_value {
                assert_eq!(merged_value, local_value, "seed {seed}, path {pointer}");
            } else if local_value == base_value {
                assert_eq!(merged_value, remote_value, "seed {seed}, path {pointer}");
            } else if remote_value == base_value {
                assert_eq!(merged_value, local_value, "seed {seed}, path {pointer}");
            } else {
                assert_eq!(merged_value, remote_value, "seed {seed}, path {pointer}");
            }
        }

        let idle = merge_three_way(&merge.merged, &merge.merged, &merge.merged);
        assert_eq!(idle.merged, merge.merged, "seed {seed}");
        assert_eq!(idle.conflicts, Vec::new(), "seed {seed}");

        let remote_after_race = mutated_path_map(&mut rng, &remote, &path_pool, &value_pool);
        let retry = merge_three_way(&base, &merge.merged, &remote_after_race);
        for pointer in path_pool {
            let path = SyncPath::parse(pointer).unwrap();
            let retry_value = retry.merged.get(&path);
            let merged_value = merge.merged.get(&path);
            let race_value = remote_after_race.get(&path);
            assert!(
                retry_value == merged_value || retry_value == race_value,
                "seed {seed}, path {pointer}: retry produced {retry_value:?}, \
                 expected {merged_value:?} or {race_value:?}"
            );
            if merged_value == base.get(&path) {
                assert_eq!(retry_value, race_value, "seed {seed}, path {pointer}");
            }
        }
    }
}

#[test]
fn test_exclusions_strip_subtrees() {
    let mut exclusions = ExclusionSet::built_in();
    exclusions.insert(ExclusionPattern::parse("/macos/buffer_font_size").unwrap());

    let mut paths = path_map(&[
        ("/buffer_font_size", json!(15)),
        ("/proxy", json!("socks5h://localhost:1080")),
        ("/ssh_connections", json!([{ "host": "example" }])),
        ("/audio/input_audio_device", json!("mic")),
        ("/macos/buffer_font_size", json!(16)),
        ("/macos/proxy", json!("socks5h://mac:1080")),
        ("/settings_sync/enabled", json!(true)),
        ("/settings_sync/exclude", json!(["/macos/buffer_font_size"])),
    ]);
    exclusions.strip(&mut paths);

    assert_eq!(
        paths,
        path_map(&[
            ("/buffer_font_size", json!(15)),
            ("/settings_sync/exclude", json!(["/macos/buffer_font_size"])),
        ])
    );
}

#[test]
fn test_exclusions_strip_ancestor_of_excluded_subpath() {
    let mut exclusions = ExclusionSet::default();
    exclusions.insert(ExclusionPattern::from_segments([
        "audio",
        "input_audio_device",
    ]));

    let local = path_map(&[
        (
            "/audio",
            json!({ "input_audio_device": "mic", "experimental.control_input_volume": true }),
        ),
        ("/theme", json!("One Dark")),
    ]);
    let mut paths = local.clone();
    exclusions.strip(&mut paths);
    assert_eq!(
        paths,
        path_map(&[
            (
                "/audio",
                json!({ "experimental.control_input_volume": true })
            ),
            ("/theme", json!("One Dark")),
        ])
    );

    let merged = path_map(&[
        (
            "/audio",
            json!({ "experimental.control_input_volume": false }),
        ),
        ("/theme", json!("One Light")),
    ]);
    assert_eq!(
        exclusions.restore(&local, &merged),
        path_map(&[
            (
                "/audio",
                json!({ "input_audio_device": "mic", "experimental.control_input_volume": false }),
            ),
            ("/theme", json!("One Light")),
        ])
    );
}

#[test]
fn test_exclusions_from_synced_list() {
    let mut exclusions = ExclusionSet::built_in();
    exclusions.extend_from_doc(&json!({
        "settings_sync": { "exclude": ["/experimental_thing", "/languages/Rust"] },
        "macos": { "settings_sync": { "exclude": ["/from_macos"] } },
        "profiles": {
            "Work": { "settings": { "settings_sync": { "exclude": ["/from_profile"] } } }
        },
    }));

    let excluded = [
        "/experimental_thing",
        "/languages/Rust/tab_size",
        "/languages/Go/tab_size",
        "/from_macos",
        "/from_profile",
        "/language_models/lmstudio/api_key",
        "/language_models/openai/custom_headers",
        "/language_models/openai_compatible/local/custom_headers",
        "/language_models/openai/api_url",
    ]
    .map(|pointer| exclusions.is_excluded(&SyncPath::parse(pointer).unwrap()));
    assert_eq!(
        excluded,
        [true, true, false, true, true, true, true, true, false]
    );
}

#[test]
fn test_wildcard_exclusions() {
    let exclusions = ExclusionSet::built_in();

    assert_eq!(
        exclusions.is_excluded(&SyncPath::parse("/lsp/rust-analyzer/binary/env").unwrap()),
        true
    );
    assert_eq!(
        exclusions
            .is_excluded(&SyncPath::parse("/lsp/rust-analyzer/initialization_options").unwrap()),
        false
    );
    assert_eq!(
        exclusions.is_excluded(&SyncPath::parse("/terminal/env").unwrap()),
        true
    );
    assert_eq!(
        exclusions.is_excluded(&SyncPath::parse("/terminal/font_size").unwrap()),
        false
    );

    let mut paths = path_map(&[
        (
            "/context_servers/some-mcp",
            json!({ "source": "custom", "command": "server", "env": { "API_KEY": "secret" } }),
        ),
        ("/dap/lldb/env", json!({ "TOKEN": "secret" })),
        (
            "/agent_servers/custom-agent/env",
            json!({ "KEY": "secret" }),
        ),
        ("/theme", json!("One Dark")),
    ]);
    exclusions.strip(&mut paths);
    assert_eq!(
        paths,
        path_map(&[
            (
                "/context_servers/some-mcp",
                json!({ "source": "custom", "command": "server" }),
            ),
            ("/theme", json!("One Dark")),
        ])
    );
}

#[test]
fn test_built_in_exclusions_cover_profiles() {
    let exclusions = ExclusionSet::built_in();

    assert_eq!(
        exclusions.is_excluded(&SyncPath::parse("/profiles/Work/settings/proxy").unwrap()),
        true
    );
    assert_eq!(
        exclusions
            .is_excluded(&SyncPath::parse("/profiles/Work/settings/ssh_connections").unwrap()),
        true
    );
    assert_eq!(
        exclusions.is_excluded(&SyncPath::parse("/profiles/Work/settings/terminal/env").unwrap()),
        true
    );
    assert_eq!(
        exclusions.is_excluded(
            &SyncPath::parse("/profiles/Work/settings/lsp/rust-analyzer/binary/env").unwrap()
        ),
        true
    );
    assert_eq!(
        exclusions
            .is_excluded(&SyncPath::parse("/profiles/Work/settings/buffer_font_size").unwrap()),
        false
    );
    assert_eq!(
        exclusions.is_excluded(&SyncPath::parse("/profiles/Work/base").unwrap()),
        false
    );
}

#[test]
fn test_built_in_exclusions_redact_legacy_context_server_secrets() {
    let classifier = DocumentClassifier::for_user_settings();
    let text = r#"{
        "context_servers": {
            "escaped": {
                "comm\u0061nd": { "path": "server", "args": ["--flag"], "env": { "TOKEN": "SECRET" } }
            }
        },
        "macos": {
            "context_servers": {
                "legacy": {
                    "command": { "path": "server", "env": { "TOKEN": "SECRET" } }
                }
            }
        }
    }"#;
    let doc = settings_json::parse_json_with_comments::<serde_json::Value>(text).unwrap();
    let mut paths = flatten_doc(&classifier, &doc);
    ExclusionSet::built_in().strip(&mut paths);

    assert_eq!(
        unflatten(&paths),
        json!({
            "context_servers": {
                "escaped": { "command": { "path": "server", "args": ["--flag"] } }
            },
            "macos": {
                "context_servers": {
                    "legacy": { "command": { "path": "server" } }
                }
            }
        })
    );
}

#[test]
fn test_any_depth_exclusions_never_redact_arrays_they_pass_through() {
    let mut exclusions = ExclusionSet::default();
    exclusions.extend_from_pointers(["/**/**/env"]);
    let mut paths = path_map(&[
        ("/file_scan_exclusions", json!(["**/.git"])),
        ("/terminal/env", json!({ "TOKEN": "SECRET" })),
        (
            "/terminal/shell",
            json!({ "program": "fish", "env": { "A": "B" } }),
        ),
    ]);
    exclusions.strip(&mut paths);

    assert_eq!(
        (paths, exclusions.pointers()),
        (
            path_map(&[
                ("/file_scan_exclusions", json!(["**/.git"])),
                ("/terminal/shell", json!({ "program": "fish" })),
            ]),
            Vec::from(["/**/env".to_string()])
        )
    );
}

#[test]
fn test_changing_built_in_exclusions_requires_schema_epoch_bump() {
    let rendered = BUILT_IN_EXCLUSIONS
        .iter()
        .map(|segments| format!("/{}", segments.join("/")))
        .collect::<Vec<_>>();
    assert_eq!(
        rendered,
        vec![
            "/settings_sync/enabled",
            "/proxy",
            "/server_url",
            "/credentials_url",
            "/ssh_connections",
            "/wsl_connections",
            "/dev_container_connections",
            "/audio/input_audio_device",
            "/audio/output_audio_device",
            "/terminal/env",
            "/lsp/*/binary/env",
            "/dap/*/env",
            "/context_servers/*/**/env",
            "/context_servers/*/headers",
            "/context_servers/*/oauth",
            "/agent_servers/*/env",
            "/language_models/lmstudio/api_key",
            "/language_models/*/custom_headers",
            "/language_models/*/*/custom_headers",
        ]
    );
}

#[test]
fn test_exclusions_cover_arrays_carriers_and_replaced_parents() {
    let classifier = DocumentClassifier::for_user_settings();
    let flatten = |doc: serde_json::Value| flatten_doc(&classifier, &doc);
    let mut exclusions = ExclusionSet::built_in();
    exclusions.extend_from_pointers([
        "/context_servers/x/args/0",
        "/some_future_setting/nested/token",
        "/settings_sync/exclude",
        "/settings_sync",
    ]);
    assert_eq!(
        exclusions.is_excluded(&SyncPath::parse("/settings_sync/exclude").unwrap()),
        false
    );
    assert_eq!(
        exclusions.is_excluded(&SyncPath::parse("/theme").unwrap()),
        false
    );
    let mut everything = ExclusionSet::default();
    everything.extend_from_pointers(["/"]);
    let mut everything_upload = flatten(json!({
        "theme": "One Dark",
        "settings_sync": { "exclude": ["/"] },
        "macos": { "theme": "One Light", "settings_sync": { "exclude": ["/x"] } },
    }));
    everything.strip(&mut everything_upload);
    assert_eq!(
        unflatten(&everything_upload),
        json!({
            "settings_sync": { "exclude": ["/"] },
            "macos": { "settings_sync": { "exclude": ["/x"] } },
        })
    );
    assert_eq!(everything.pointers(), vec!["/".to_string()]);

    let local = flatten(json!({
        "context_servers": { "x": { "command": "server", "args": ["SECRET"] } },
        "terminal": { "font_size": 14, "env": { "TOKEN": "LOCAL" } },
        "some_future_setting": { "nested": { "token": "LOCAL" }, "public": 1 },
        "settings_sync": { "exclude": ["/settings_sync/exclude"] },
    }));
    let mut upload = local.clone();
    exclusions.strip(&mut upload);
    assert_eq!(
        unflatten(&upload),
        json!({
            "context_servers": { "x": { "command": "server" } },
            "terminal": { "font_size": 14 },
            "some_future_setting": { "public": 1 },
            "settings_sync": { "exclude": ["/settings_sync/exclude"] },
        })
    );

    let merged = flatten(json!({
        "context_servers": { "x": { "command": "other" } },
        "terminal": {},
        "some_future_setting": { "public": 2 },
    }));
    assert_eq!(
        exclusions.restore(&local, &merged),
        flatten(json!({
            "context_servers": { "x": { "command": "other", "args": ["SECRET"] } },
            "terminal": { "env": { "TOKEN": "LOCAL" } },
            "some_future_setting": { "nested": { "token": "LOCAL" }, "public": 2 },
        }))
    );

    assert_eq!(
        exclusions.restore(&local, &PathMap::new()),
        flatten(json!({
            "context_servers": { "x": { "args": ["SECRET"] } },
            "terminal": { "env": { "TOKEN": "LOCAL" } },
            "some_future_setting": { "nested": { "token": "LOCAL" } },
        }))
    );
    assert_eq!(
        exclusions.restore(
            &flatten(json!({ "some_future_setting": { "nested": { "token": null } } })),
            &flatten(json!({ "some_future_setting": { "public": 2 } })),
        ),
        flatten(json!({
            "some_future_setting": { "nested": { "token": null }, "public": 2 }
        }))
    );

    let merged = flatten(json!({ "some_future_setting": 3 }));
    let restored = exclusions.restore(&local, &merged);
    assert_eq!(
        restored,
        flatten(json!({
            "context_servers": { "x": { "args": ["SECRET"] } },
            "terminal": { "env": { "TOKEN": "LOCAL" } },
            "some_future_setting": { "nested": { "token": "LOCAL" } },
        }))
    );
    let mut next_upload = restored;
    exclusions.strip(&mut next_upload);
    assert_eq!(next_upload, PathMap::new());
}

#[test]
fn test_merge_resolves_structural_conflicts_for_the_remote() {
    let local = path_map(&[("/telemetry/metrics", json!(false)), ("/ab", json!(1))]);
    let remote = path_map(&[("/telemetry", json!(null))]);
    let merge = merge_three_way(&PathMap::default(), &local, &remote);
    assert_eq!(
        merge.merged,
        path_map(&[("/telemetry", json!(null)), ("/ab", json!(1))])
    );
    let conflicts = merge
        .conflicts
        .iter()
        .map(|conflict| (conflict.path.to_string(), conflict.local.clone()))
        .collect::<Vec<_>>();
    assert_eq!(
        conflicts,
        vec![("/telemetry/metrics".to_string(), Some(json!(false)))]
    );

    let merge = merge_three_way(&PathMap::default(), &remote, &local);
    assert_eq!(
        merge.merged,
        path_map(&[("/telemetry/metrics", json!(false)), ("/ab", json!(1))])
    );
    assert_eq!(
        merge
            .conflicts
            .iter()
            .map(|conflict| conflict.path.to_string())
            .collect::<Vec<_>>(),
        vec!["/telemetry".to_string()]
    );

    let local = path_map(&[("/profiles/Work", json!({}))]);
    let remote = path_map(&[("/profiles/Work/settings/tab_size", json!(4))]);
    let merge = merge_three_way(&PathMap::default(), &local, &remote);
    assert_eq!(merge.merged, remote);
    assert_eq!(merge.conflicts, Vec::new());

    let base = path_map(&[
        ("/telemetry/diagnostics", json!(false)),
        ("/telemetry/metrics", json!(false)),
    ]);
    let local = path_map(&[("/telemetry", json!(null))]);
    let remote = path_map(&[
        ("/telemetry/diagnostics", json!(false)),
        ("/telemetry/metrics", json!(true)),
    ]);
    assert_eq!(merge_three_way(&base, &local, &remote).merged, remote);
}

#[test]
fn test_exclusion_lists_merge_as_sets() {
    let exclude = |items: &[&str]| path_map(&[("/settings_sync/exclude", json!(items))]);
    let base = exclude(&["/a", "/b"]);
    let local = exclude(&["/a", "/b", "/local"]);
    let remote = exclude(&["/b", "/remote"]);
    let merge = merge_three_way(&base, &local, &remote);
    assert_eq!(merge.merged, exclude(&["/b", "/local", "/remote"]));
    assert_eq!(merge.conflicts, Vec::new());

    let merge = merge_three_way(&base, &PathMap::new(), &base);
    assert_eq!(merge.merged, PathMap::new());
}

#[test]
fn test_flatten_merges_pattern_property_maps_and_keeps_empty_containers() {
    let classifier = DocumentClassifier::for_user_settings();
    let flatten = |doc: serde_json::Value| flatten_doc(&classifier, &doc);
    let base = flatten(json!({ "buffer_font_features": { "liga": true, "calt": true } }));
    let local = flatten(json!({ "buffer_font_features": { "liga": false, "calt": true } }));
    let remote = flatten(json!({ "buffer_font_features": { "liga": true, "calt": false } }));
    let merge = merge_three_way(&base, &local, &remote);
    assert_eq!(
        unflatten(&merge.merged),
        json!({ "buffer_font_features": { "liga": false, "calt": false } })
    );
    assert_eq!(merge.conflicts, Vec::new());

    let doc = json!({ "profiles": { "Work": {} }, "languages": {} });
    assert_eq!(
        flatten(doc.clone()),
        path_map(&[("/languages", json!({})), ("/profiles/Work", json!({}))])
    );
    assert_eq!(unflatten(&flatten(doc.clone())), doc);
}

#[test]
fn test_apply_target_edits_literal_keys_prunes_and_verifies() {
    let classifier = DocumentClassifier::for_user_settings();
    let apply = |text: &str, target: serde_json::Value| {
        let current = flatten_doc(
            &classifier,
            &settings_json::parse_json_with_comments::<serde_json::Value>(text).unwrap(),
        );
        apply_target_to_text(
            &classifier,
            text,
            &current,
            &flatten_doc(&classifier, &target),
        )
        .map(|text| settings_json::parse_json_with_comments::<serde_json::Value>(&text).unwrap())
    };

    assert_eq!(
        apply(
            r#"{ "theme": "One Dark", "profiles": { "Work": { "settings": { "tab_size": 4 } } } }"#,
            json!({ "theme": "One Dark" }),
        )
        .unwrap(),
        json!({ "theme": "One Dark" })
    );
    let with_hash_profile = json!({
        "profiles": {
            "Work": { "settings": { "tab_size": 4 } },
            "#0": { "settings": { "tab_size": 8 } }
        }
    });
    assert_eq!(
        apply(
            r#"{ "profiles": { "Work": { "settings": { "tab_size": 4 } } } }"#,
            with_hash_profile.clone(),
        )
        .unwrap(),
        with_hash_profile
    );
    assert_eq!(
        apply(
            r#"{ "tab_size": 2, "tab_size": 4 }"#,
            json!({ "tab_size": 8 })
        )
        .unwrap_err()
        .to_string(),
        "applying synced settings produced unexpected values at /tab_size"
    );

    let text = r#"{ "theme": { /* retained */ "mode": "system", "light": "One Light", "dark": "One Dark", "old": 1 } }"#;
    let current = flatten_doc(
        &classifier,
        &settings_json::parse_json_with_comments::<serde_json::Value>(text).unwrap(),
    );
    let target = flatten_doc(
        &classifier,
        &json!({ "theme": { "mode": "dark", "light": "One Light", "dark": "One Dark" } }),
    );
    assert_eq!(
        apply_target_to_text(&classifier, text, &current, &target).unwrap(),
        r#"{ "theme": { /* retained */ "mode": "dark", "light": "One Light", "dark": "One Dark" } }"#
    );

    assert_eq!(
        apply(
            r#"{ "future": { "nested": { "option": 1 } } }"#,
            json!({ "future": { "nested": {} } }),
        )
        .unwrap(),
        json!({ "future": { "nested": {} } })
    );
    assert_eq!(
        apply("{\n  /*\n  // */\n  \"a\": 1\n}", json!({ "a": 1, "b": 2 })).unwrap(),
        json!({ "a": 1, "b": 2 })
    );
}

#[test]
fn test_apply_target_keeps_comments_of_empty_containers() {
    let classifier = DocumentClassifier::for_user_settings();
    let apply = |text: &str, target: serde_json::Value| {
        let current = flatten_doc(
            &classifier,
            &settings_json::parse_json_with_comments::<serde_json::Value>(text).unwrap(),
        );
        apply_target_to_text(
            &classifier,
            text,
            &current,
            &flatten_doc(&classifier, &target),
        )
        .unwrap()
    };

    assert_eq!(
        apply(
            "{\n  \"telemetry\": {\n    // KEEP\n  }\n}",
            json!({ "telemetry": { "metrics": false } }),
        ),
        "{\n  \"telemetry\": {\n    // KEEP\n    \"metrics\": false\n  }\n}"
    );
    assert_eq!(
        apply(
            "{\n  \"telemetry\": {\n    // KEEP\n    \"metrics\": false\n  }\n}",
            json!({ "telemetry": {} }),
        ),
        "{\n  \"telemetry\": {\n    // KEEP\n    \n  }\n}"
    );
}

#[test]
fn test_first_sync_replacements_keep_both_exclusion_lists() {
    let preview = FirstSyncPreview {
        account_id: FAKE_ACCOUNT_ID,
        group_id: "group".to_string(),
        remote_document_id: Some("document".to_string()),
        remote_version: Some(1),
        local: path_map(&[
            ("/settings_sync/exclude", json!(["/local_secret"])),
            ("/tab_size", json!(4)),
        ]),
        remote: path_map(&[
            ("/settings_sync/exclude", json!(["/remote_secret"])),
            ("/tab_size", json!(8)),
        ]),
        can_replace_remote: true,
    };
    let exclusions = json!(["/local_secret", "/remote_secret"]);
    assert_eq!(
        preview.merge(FirstSyncChoice::ReplaceLocal).merged,
        path_map(&[
            ("/settings_sync/exclude", exclusions.clone()),
            ("/tab_size", json!(8))
        ])
    );
    assert_eq!(
        preview.merge(FirstSyncChoice::ReplaceRemote).merged,
        path_map(&[
            ("/settings_sync/exclude", exclusions),
            ("/tab_size", json!(4))
        ])
    );
}

#[test]
fn test_apply_ops_preserves_comments_and_formatting() {
    let text = indoc::indoc! {r#"
        {
            // The font size everywhere.
            "buffer_font_size": 15,
            "macos": {
                // Bigger on the laptop.
                "buffer_font_size": 16
            },
            // Solarized forever.
            "theme": "Solarized Dark"
        }
    "#};

    let current = path_map(&[
        ("/buffer_font_size", json!(15)),
        ("/macos/buffer_font_size", json!(16)),
        ("/theme", json!("Solarized Dark")),
    ]);
    let target = path_map(&[
        ("/buffer_font_size", json!(17)),
        ("/macos/buffer_font_size", json!(16)),
        ("/theme", json!("Solarized Dark")),
        ("/languages/Rust/tab_size", json!(4)),
    ]);

    let new_text = apply_ops_to_text(text, &diff_paths(&current, &target));

    pretty_assertions::assert_eq!(
        new_text,
        indoc::indoc! {r#"
            {
                "languages": {
                    "Rust": {
                        "tab_size": 4
                    }
                },
                // The font size everywhere.
                "buffer_font_size": 17,
                "macos": {
                    // Bigger on the laptop.
                    "buffer_font_size": 16
                },
                // Solarized forever.
                "theme": "Solarized Dark"
            }
        "#}
    );
}

#[test]
fn test_apply_ops_on_empty_text() {
    let target = path_map(&[
        ("/buffer_font_size", json!(15)),
        ("/languages/Rust/tab_size", json!(4)),
    ]);
    let new_text = apply_ops_to_text("", &diff_paths(&PathMap::default(), &target));
    let value = settings_json::parse_json_with_comments::<serde_json::Value>(&new_text).unwrap();
    assert_eq!(
        value,
        json!({ "buffer_font_size": 15, "languages": { "Rust": { "tab_size": 4 } } })
    );
}

#[test]
fn test_apply_ops_deletes_keys() {
    let text = indoc::indoc! {r#"
        {
            "buffer_font_size": 15,
            "macos": {
                "buffer_font_size": 16
            }
        }
    "#};

    let current = path_map(&[
        ("/buffer_font_size", json!(15)),
        ("/macos/buffer_font_size", json!(16)),
    ]);
    let target = path_map(&[("/buffer_font_size", json!(15))]);

    let new_text = apply_ops_to_text(text, &diff_paths(&current, &target));
    let value = settings_json::parse_json_with_comments::<serde_json::Value>(&new_text).unwrap();
    assert_eq!(value, json!({ "buffer_font_size": 15, "macos": {} }));
}

#[gpui::test]
async fn test_newer_state_file_format_starts_fresh(cx: &mut TestAppContext) {
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/dir",
        json!({
            "state.json": r#"{ "format_version": 999, "group_id": "g", "base": null }"#
        }),
    )
    .await;
    assert_eq!(
        load_state(fs.as_ref(), Path::new("/dir/state.json")).await,
        SyncState::default()
    );

    fs.insert_file("/dir/state.json", b"not json".to_vec())
        .await;
    assert_eq!(
        load_state(fs.as_ref(), Path::new("/dir/state.json")).await,
        SyncState::default()
    );
}

#[gpui::test]
async fn test_two_devices_converge_and_preserve_comments(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device_a = make_device(
        &server,
        "a",
        indoc::indoc! {r#"
            {
                // The font size everywhere.
                "buffer_font_size": 15,
                "settings_sync": {
                    "enabled": true
                }
            }
        "#},
        cx,
    )
    .await;
    settle(cx);

    let doc = server.doc().unwrap();
    assert_eq!(doc.version, 1);
    assert_eq!(doc.doc, json!({ "buffer_font_size": 15 }));

    let device_b = make_device(
        &server,
        "b",
        indoc::indoc! {r#"
            {
                // Solarized forever.
                "theme": "Solarized Dark"
            }
        "#},
        cx,
    )
    .await;
    settle(cx);

    let doc = server.doc().unwrap();
    assert_eq!(doc.version, 2);
    assert_eq!(
        doc.doc,
        json!({
            "buffer_font_size": 15,
            "theme": "Solarized Dark"
        })
    );

    pretty_assertions::assert_eq!(
        device_b.settings_text(cx).await,
        indoc::indoc! {r#"
            {
                "buffer_font_size": 15,
                // Solarized forever.
                "theme": "Solarized Dark"
            }
        "#}
    );

    sync(&device_a, cx);
    pretty_assertions::assert_eq!(
        device_a.settings_text(cx).await,
        indoc::indoc! {r#"
            {
                "theme": "Solarized Dark",
                // The font size everywhere.
                "buffer_font_size": 15,
                "settings_sync": {
                    "enabled": true
                }
            }
        "#}
    );
    assert_eq!(server.doc().unwrap().version, 2);

    let push_count = server.push_count();
    sync(&device_a, cx);
    sync(&device_b, cx);
    assert_eq!(server.push_count(), push_count);
    assert_eq!(server.doc().unwrap().version, 2);
}

#[gpui::test]
async fn test_cas_race_converges_losslessly(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device_a = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);
    assert_eq!(server.doc().unwrap().version, 1);

    device_a
        .fs
        .insert_file(
            &device_a.settings_path,
            br#"{ "buffer_font_size": 16 }"#.to_vec(),
        )
        .await;

    let mut racing_doc = server.doc().unwrap();
    racing_doc.version += 1;
    racing_doc.doc = json!({ "buffer_font_size": 15, "theme": "One Dark" });
    server.queue_racing_doc(racing_doc);

    sync(&device_a, cx);
    cx.executor().advance_clock(Duration::from_secs(5));
    cx.run_until_parked();

    let doc = server.doc().unwrap();
    assert_eq!(doc.version, 3);
    assert_eq!(
        doc.doc,
        json!({ "buffer_font_size": 16, "theme": "One Dark" })
    );
    assert_eq!(server.conflict_count(), 1);
    assert_eq!(
        device_a.settings_json(cx).await,
        json!({ "buffer_font_size": 16, "theme": "One Dark" })
    );
}

#[gpui::test]
async fn test_mid_cycle_edit_defers_base_and_keeps_remote_changes(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);
    assert_eq!(server.doc().unwrap().version, 1);

    let mut remote = server.doc().unwrap();
    remote.version += 1;
    remote.doc = json!({ "buffer_font_size": 15, "theme": "One Dark" });
    server.set_doc(remote);

    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 16 }"#.to_vec(),
        )
        .await;
    let release_push = server.gate_next_push();
    settle(cx);

    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 16, "tab_size": 8 }"#.to_vec(),
        )
        .await;
    cx.run_until_parked();
    release_push.send(()).unwrap();
    settle(cx);
    settle(cx);
    cx.executor().advance_clock(Duration::from_secs(5));
    cx.run_until_parked();

    let doc = server.doc().unwrap();
    assert_eq!(
        doc.doc,
        json!({ "buffer_font_size": 16, "theme": "One Dark", "tab_size": 8 })
    );
    assert_eq!(
        device.settings_json(cx).await,
        json!({ "buffer_font_size": 16, "theme": "One Dark", "tab_size": 8 })
    );
}

#[gpui::test]
async fn test_stale_in_memory_state_defers_to_newer_disk_state(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/shared",
        json!({ "settings.json": r#"{ "buffer_font_size": 15 }"# }),
    )
    .await;
    let settings_path = PathBuf::from("/shared/settings.json");
    let state_path = PathBuf::from("/shared/sync_state.json");
    let process_a = make_device_with(
        server.clone(),
        fs.clone(),
        settings_path.clone(),
        state_path.clone(),
        cx,
    )
    .await;
    let process_b = make_device_with(
        server.clone(),
        fs.clone(),
        settings_path.clone(),
        state_path.clone(),
        cx,
    )
    .await;
    settle(cx);
    settle(cx);
    assert_eq!(server.doc().unwrap().version, 1);

    process_b.engine.update(cx, |engine, _| engine.pause());
    fs.insert_file(&settings_path, br#"{ "buffer_font_size": 16 }"#.to_vec())
        .await;
    settle(cx);
    settle(cx);
    assert_eq!(server.doc().unwrap().version, 2);

    let mut remote = server.doc().unwrap();
    remote.version += 1;
    remote.doc = json!({ "buffer_font_size": 17 });
    server.set_doc(remote);

    let events_b = subscribe_to_events(&process_b.engine, cx);
    process_b.engine.update(cx, |engine, cx| engine.unpause(cx));
    settle(cx);
    settle(cx);

    assert_eq!(
        process_a.settings_json(cx).await,
        json!({ "buffer_font_size": 17 })
    );
    assert_eq!(server.doc().unwrap().version, 3);
    let conflict_event_count = events_b
        .lock()
        .iter()
        .filter(|event| matches!(event, SettingsSyncEvent::ConflictsResolved(_)))
        .count();
    assert_eq!(conflict_event_count, 0);
}

#[gpui::test]
async fn test_groups_are_isolated(cx: &mut TestAppContext) {
    init_test(cx);
    let backend = Arc::new(FakeSettingsSyncBackend::default());
    let server_work = FakeSettingsSyncServer::in_group(backend.clone(), "work");
    let server_home = FakeSettingsSyncServer::in_group(backend.clone(), "home");

    let device_work = make_device(&server_work, "work", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);
    let device_home = make_device(&server_home, "home", r#"{ "theme": "One Dark" }"#, cx).await;
    settle(cx);

    assert_eq!(
        server_work.doc().unwrap().doc,
        json!({ "buffer_font_size": 15 })
    );
    assert_eq!(
        server_home.doc().unwrap().doc,
        json!({ "theme": "One Dark" })
    );
    assert_eq!(
        device_work.settings_json(cx).await,
        json!({ "buffer_font_size": 15 })
    );
    assert_eq!(
        device_home.settings_json(cx).await,
        json!({ "theme": "One Dark" })
    );

    let fetch_count = server_work.fetch_count();
    device_work.engine.update(cx, |engine, cx| {
        engine.handle_remote_changed(
            "work",
            cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS,
            1,
            "document-1",
            cx,
        )
    });
    settle(cx);
    assert_eq!(server_work.fetch_count(), fetch_count);

    device_work.engine.update(cx, |engine, cx| {
        engine.handle_remote_changed(
            "work",
            cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS,
            1,
            "recreated-document",
            cx,
        )
    });
    settle(cx);
    assert_eq!(server_work.fetch_count(), fetch_count + 1);

    let fetch_count = server_work.fetch_count();
    device_work.engine.update(cx, |engine, cx| {
        engine.handle_remote_changed(
            "home",
            cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS,
            99,
            "document-1",
            cx,
        )
    });
    settle(cx);
    assert_eq!(server_work.fetch_count(), fetch_count);

    device_work.engine.update(cx, |engine, cx| {
        engine.handle_remote_changed(
            "work",
            cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS,
            99,
            "document-1",
            cx,
        )
    });
    settle(cx);
    assert_eq!(server_work.fetch_count(), fetch_count + 1);
}

#[gpui::test]
async fn test_excluded_paths_never_upload_and_get_redacted(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();
    server.set_doc(cloud_api_client::SyncedSettings {
        group_id: "fake-group".to_string(),
        document_id: "seeded-document".to_string(),
        kind: cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS.to_string(),
        version: 7,
        schema_epoch: crate::settings_schema_epoch(),
        doc: json!({
            "proxy": "socks5h://leaked:1080",
            "theme": "One Dark"
        }),
        updated_by_system_id: None,
        provenance: BTreeMap::new(),
    });

    let device = make_device(
        &server,
        "a",
        indoc::indoc! {r#"
            {
                "proxy": "socks5h://localhost:1080",
                "ssh_connections": [{ "host": "secret-box" }],
                "audio": { "input_audio_device": "mic" },
                "buffer_font_size": 15,
                "settings_sync": {
                    "enabled": true,
                    "exclude": ["/buffer_line_height"]
                },
                "buffer_line_height": "comfortable"
            }
        "#},
        cx,
    )
    .await;
    settle(cx);

    let doc = server.doc().unwrap();
    assert_eq!(doc.version, 8);
    assert_eq!(
        doc.doc,
        json!({
            "buffer_font_size": 15,
            "theme": "One Dark",
            "settings_sync": {
                "exclude": ["/buffer_line_height"]
            }
        })
    );

    let local = device.settings_json(cx).await;
    assert_eq!(local.get("proxy"), Some(&json!("socks5h://localhost:1080")));
    assert_eq!(
        local.get("ssh_connections"),
        Some(&json!([{ "host": "secret-box" }]))
    );
    assert_eq!(local.get("theme"), Some(&json!("One Dark")));
}

#[gpui::test]
async fn test_exclusions_from_every_settings_scope(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let settings = json!({
        "ui_font_size": 20,
        "buffer_font_size": 15,
        "macos": { "settings_sync": { "exclude": ["/ui_font_size"] } },
    });
    let device = make_device(&server, "a", &settings.to_string(), cx).await;
    settle(cx);

    let doc = server.doc().unwrap();
    assert_eq!(
        doc.doc,
        json!({
            "buffer_font_size": 15,
            "macos": { "settings_sync": { "exclude": ["/ui_font_size"] } },
        })
    );
    assert_eq!(device.settings_json(cx).await, settings);
}

#[gpui::test]
async fn test_only_the_root_setting_enables_sync(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();
    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);
    assert_eq!(server.push_count(), 1);

    cx.update(|cx| {
        SettingsStore::update(cx, |store, cx| {
            store
                .set_user_settings(
                    &json!({
                        "settings_sync": { "enabled": false },
                        "macos": { "settings_sync": { "enabled": true } },
                        "linux": { "settings_sync": { "enabled": true } },
                        "windows": { "settings_sync": { "enabled": true } },
                        "profiles": { "Work": { "settings": { "settings_sync": { "enabled": true } } } },
                    })
                    .to_string(),
                    cx,
                )
                .result()
                .unwrap();
        });
    });
    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 16 }"#.to_vec(),
        )
        .await;
    settle(cx);

    assert_eq!(
        device
            .engine
            .read_with(cx, |engine, cx| engine.activity(cx)),
        SyncActivity::Off
    );
    assert_eq!(server.push_count(), 1);
}

#[gpui::test]
async fn test_synced_writes_queue_behind_settings_file_writes(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();
    let device = make_device(
        &server,
        "a",
        r#"{ "buffer_font_size": 15, "tab_size": 4 }"#,
        cx,
    )
    .await;
    settle(cx);

    let (release_tx, release_rx) = oneshot::channel::<()>();
    cx.update(|cx| {
        let fs = device.fs.clone();
        let settings_path = device.settings_path.clone();
        cx.global::<SettingsStore>()
            .enqueue_settings_file_job(move |_| {
                async move {
                    let text = fs.load(&settings_path).await?;
                    release_rx.await.ok();
                    fs.atomic_write(
                        settings_path,
                        text.replace("\"tab_size\": 4", "\"tab_size\": 8"),
                    )
                    .await
                }
                .boxed_local()
            });
    });
    let mut remote = server.doc().unwrap();
    remote.version += 1;
    remote.doc = json!({ "buffer_font_size": 16, "tab_size": 4 });
    server.set_doc(remote);
    sync(&device, cx);
    release_tx.send(()).unwrap();
    settle(cx);
    settle(cx);

    let expected = json!({ "buffer_font_size": 16, "tab_size": 8 });
    assert_eq!(device.settings_json(cx).await, expected);
    assert_eq!(server.doc().unwrap().doc, expected);
}

#[gpui::test]
async fn test_history_restores_only_into_its_own_group(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();
    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);
    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 16 }"#.to_vec(),
        )
        .await;
    settle(cx);

    let history = device
        .engine
        .update(cx, |engine, cx| engine.list_history(cx))
        .await
        .unwrap();
    assert_eq!(
        (history.account_id, history.group_id.as_deref()),
        (FAKE_ACCOUNT_ID, Some("fake-group"))
    );
    let first_revision = history.revisions.last().unwrap().clone();
    let restore = |group_id: &str, cx: &mut TestAppContext| {
        device.engine.update(cx, |engine, cx| {
            engine.restore_revision(
                first_revision.clone(),
                FAKE_ACCOUNT_ID,
                group_id.to_string(),
                cx,
            )
        })
    };
    let rejection = "this history belongs to another account or sync group, not restoring it";
    assert_eq!(
        restore("another-group", cx).await.unwrap_err().to_string(),
        rejection
    );

    drop(restore("fake-group", cx));
    settle(cx);
    settle(cx);
    assert_eq!(
        device.settings_json(cx).await,
        json!({ "buffer_font_size": 15 })
    );
    assert_eq!(server.doc().unwrap().doc, json!({ "buffer_font_size": 15 }));

    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 18 }"#.to_vec(),
        )
        .await;
    settle(cx);
    cx.update(|cx| {
        SettingsStore::update(cx, |store, cx| {
            store.update_user_settings(cx, |content| {
                content.settings_sync.get_or_insert_default().enabled = Some(false);
            });
        });
    });
    server.set_account(Some(FAKE_ACCOUNT_ID + 1), Some("other-group"));
    sync(&device, cx);
    assert_eq!(
        restore("fake-group", cx).await.unwrap_err().to_string(),
        rejection
    );
    assert_eq!(
        device.settings_json(cx).await,
        json!({ "buffer_font_size": 18 })
    );
}

#[gpui::test]
async fn test_conflict_remote_wins_and_emits_event(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);
    assert_eq!(server.doc().unwrap().version, 1);

    let events = subscribe_to_events(&device.engine, cx);

    let mut remote = server.doc().unwrap();
    remote.version += 1;
    remote.doc = json!({ "buffer_font_size": 16 });
    server.set_doc(remote);

    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 17 }"#.to_vec(),
        )
        .await;
    sync(&device, cx);

    assert_eq!(
        device.settings_json(cx).await,
        json!({ "buffer_font_size": 16 })
    );
    assert_eq!(server.doc().unwrap().version, 2);

    let conflicts = first_conflict_batch(&events).conflicts;
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].path.to_string(), "/buffer_font_size");
    assert_eq!(conflicts[0].local, Some(json!(17)));
    assert_eq!(conflicts[0].remote, Some(json!(16)));
}

#[gpui::test]
async fn test_conflicts_report_original_local_values(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device = make_device(
        &server,
        "a",
        r#"{ "buffer_font_size": 15, "theme": "A" }"#,
        cx,
    )
    .await;
    settle(cx);
    assert_eq!(server.doc().unwrap().version, 1);

    let events = subscribe_to_events(&device.engine, cx);

    let mut remote = server.doc().unwrap();
    remote.version += 1;
    remote.doc = json!({ "buffer_font_size": 18, "theme": "B" });
    server.set_doc(remote.clone());

    let mut racing = remote;
    racing.version += 1;
    racing.doc = json!({ "buffer_font_size": 19, "theme": "C" });
    server.queue_racing_doc(racing);

    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 16, "theme": "A", "tab_size": 8 }"#.to_vec(),
        )
        .await;
    sync(&device, cx);
    cx.executor().advance_clock(Duration::from_secs(5));
    cx.run_until_parked();

    assert_eq!(
        device.settings_json(cx).await,
        json!({ "buffer_font_size": 19, "theme": "C", "tab_size": 8 })
    );
    assert_eq!(
        server.doc().unwrap().doc,
        json!({ "buffer_font_size": 19, "theme": "C", "tab_size": 8 })
    );

    let conflicts = first_conflict_batch(&events).conflicts;
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].path.to_string(), "/buffer_font_size");
    assert_eq!(conflicts[0].local, Some(json!(16)));
    assert_eq!(conflicts[0].remote, Some(json!(19)));
}

#[gpui::test]
async fn test_revert_conflicts_round_trip(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);

    let events = subscribe_to_events(&device.engine, cx);

    let mut remote = server.doc().unwrap();
    remote.version += 1;
    remote.doc = json!({ "buffer_font_size": 16 });
    server.set_doc(remote);
    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 17 }"#.to_vec(),
        )
        .await;
    sync(&device, cx);
    assert_eq!(
        device.settings_json(cx).await,
        json!({ "buffer_font_size": 16 })
    );

    let batch = first_conflict_batch(&events);
    device.engine.update(cx, |engine, cx| {
        engine.revert_conflicts(batch, cx).detach();
    });
    settle(cx);
    settle(cx);

    assert_eq!(
        device.settings_json(cx).await,
        json!({ "buffer_font_size": 17 })
    );
    let doc = server.doc().unwrap();
    assert_eq!(doc.version, 3);
    assert_eq!(doc.doc, json!({ "buffer_font_size": 17 }));

    assert_eq!(
        device
            .engine
            .read_with(cx, |engine, _| engine.recent_conflicts().cloned()),
        None
    );
    let stale_revert = device.engine.update(cx, |engine, cx| {
        engine.revert_conflicts(first_conflict_batch(&events), cx)
    });
    assert_eq!(
        stale_revert.await.unwrap_err().to_string(),
        "/buffer_font_size changed since the conflict, not reverting it"
    );

    let foreign_batch = ConflictBatch {
        group_id: "another-group".to_string(),
        ..first_conflict_batch(&events)
    };
    let foreign_revert = device
        .engine
        .update(cx, |engine, cx| engine.revert_conflicts(foreign_batch, cx));
    assert_eq!(
        foreign_revert.await.unwrap_err().to_string(),
        "these conflicts belong to another account or sync group, not reverting them"
    );
}

#[gpui::test]
async fn test_pauses_after_repeated_push_conflicts(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();
    server.set_always_conflict(true);

    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    let events = subscribe_to_events(&device.engine, cx);
    settle(cx);
    cx.executor().advance_clock(Duration::from_secs(10));
    cx.run_until_parked();

    assert_eq!(server.push_count(), MAX_PUSH_ATTEMPTS);
    assert_eq!(
        device.engine.read_with(cx, |engine, _| engine.is_paused()),
        true
    );
    let paused_event_count = events
        .lock()
        .iter()
        .filter(|event| {
            matches!(
                event,
                SettingsSyncEvent::Paused(PauseReason::RepeatedConflicts)
            )
        })
        .count();
    assert_eq!(paused_event_count, 1);

    server.set_always_conflict(false);
    device.engine.update(cx, |engine, cx| engine.unpause(cx));
    settle(cx);

    assert_eq!(
        device.engine.read_with(cx, |engine, _| engine.is_paused()),
        false
    );
    let doc = server.doc().unwrap();
    assert_eq!(doc.version, 1);
    assert_eq!(doc.doc, json!({ "buffer_font_size": 15 }));
}

#[gpui::test]
async fn test_below_epoch_client_pulls_but_never_pushes(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();
    server.set_doc(cloud_api_client::SyncedSettings {
        group_id: "fake-group".to_string(),
        document_id: "seeded-document".to_string(),
        kind: cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS.to_string(),
        version: 3,
        schema_epoch: crate::settings_schema_epoch() + 1,
        doc: json!({ "theme": "One Dark" }),
        updated_by_system_id: None,
        provenance: BTreeMap::new(),
    });

    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    let events = subscribe_to_events(&device.engine, cx);
    settle(cx);

    assert_eq!(server.push_count(), 0);
    assert_eq!(server.doc().unwrap().version, 3);
    assert_eq!(
        device.settings_json(cx).await,
        json!({ "buffer_font_size": 15, "theme": "One Dark" })
    );
    let update_required_events = events
        .lock()
        .iter()
        .filter(|event| matches!(event, SettingsSyncEvent::UpdateRequired))
        .count();
    assert_eq!(update_required_events, 1);

    sync(&device, cx);
    assert_eq!(server.push_count(), 0);
}

#[gpui::test]
async fn test_disable_mid_cycle_skips_push(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);
    assert_eq!(server.push_count(), 1);

    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 16 }"#.to_vec(),
        )
        .await;
    let release_fetch = server.gate_next_fetch();
    settle(cx);

    cx.update(|cx| {
        SettingsStore::update(cx, |store, cx| {
            store.update_user_settings(cx, |content| {
                content.settings_sync = Some(settings::SettingsSyncContent {
                    enabled: Some(false),
                    exclude: None,
                });
            });
        });
    });
    release_fetch.send(()).unwrap();
    settle(cx);
    settle(cx);

    assert_eq!(server.push_count(), 1);
    assert_eq!(server.doc().unwrap().doc, json!({ "buffer_font_size": 15 }));
}

#[gpui::test]
async fn test_unimplemented_cloud_skips_cycles_until_available(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();
    server.set_unimplemented(true);

    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    let events = subscribe_to_events(&device.engine, cx);
    settle(cx);
    sync(&device, cx);

    assert_eq!(server.doc(), None);
    assert_eq!(server.fetch_count(), 0);
    assert_eq!(server.push_count(), 0);
    assert_eq!(sync_failed_count(&events), 1);

    server.set_unimplemented(false);
    sync(&device, cx);

    let doc = server.doc().unwrap();
    assert_eq!(doc.version, 1);
    assert_eq!(doc.doc, json!({ "buffer_font_size": 15 }));
}

#[gpui::test]
async fn test_malformed_local_json_skips_cycle(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 "#, cx).await;
    let events = subscribe_to_events(&device.engine, cx);
    settle(cx);

    assert_eq!(server.doc(), None);
    assert_eq!(server.push_count(), 0);
    assert_eq!(sync_failed_count(&events), 0);
    assert_eq!(
        device
            .engine
            .read_with(cx, |engine, _| engine.status().last_error.is_some()),
        true
    );
    assert_eq!(
        device.fs.load(&device.settings_path).await.unwrap(),
        r#"{ "buffer_font_size": 15 "#
    );
}

#[gpui::test]
async fn test_cas_retry_keeps_a_concurrent_revert(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);
    assert_eq!(server.doc().unwrap().version, 1);

    let mut remote = server.doc().unwrap();
    remote.version = 2;
    remote.doc = json!({ "buffer_font_size": 16 });
    server.set_doc(remote.clone());
    let mut reverted = remote;
    reverted.version = 3;
    reverted.doc = json!({ "buffer_font_size": 15 });
    server.queue_racing_doc(reverted);

    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 15, "tab_size": 8 }"#.to_vec(),
        )
        .await;
    sync(&device, cx);
    cx.executor().advance_clock(Duration::from_secs(5));
    cx.run_until_parked();

    let doc = server.doc().unwrap();
    assert_eq!(doc.version, 4);
    assert_eq!(doc.doc, json!({ "buffer_font_size": 15, "tab_size": 8 }));
    assert_eq!(
        device.settings_json(cx).await,
        json!({ "buffer_font_size": 15, "tab_size": 8 })
    );
}

#[gpui::test]
async fn test_newer_epoch_seen_on_retry_stops_pushing(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    let events = subscribe_to_events(&device.engine, cx);
    settle(cx);
    assert_eq!(server.push_count(), 1);

    let mut racing_doc = server.doc().unwrap();
    racing_doc.version = 2;
    racing_doc.schema_epoch = crate::settings_schema_epoch() + 1;
    racing_doc.doc = json!({ "buffer_font_size": 15, "theme": "One Dark" });
    server.queue_racing_doc(racing_doc.clone());

    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 15, "tab_size": 8 }"#.to_vec(),
        )
        .await;
    sync(&device, cx);
    cx.executor().advance_clock(Duration::from_secs(5));
    cx.run_until_parked();

    assert_eq!(server.push_count(), 2);
    assert_eq!(server.doc(), Some(racing_doc));
    assert_eq!(
        device.settings_json(cx).await,
        json!({ "buffer_font_size": 15, "tab_size": 8, "theme": "One Dark" })
    );
    let update_required_events = events
        .lock()
        .iter()
        .filter(|event| matches!(event, SettingsSyncEvent::UpdateRequired))
        .count();
    assert_eq!(update_required_events, 1);
}

#[gpui::test]
async fn test_moving_to_another_group_drops_the_stale_base(cx: &mut TestAppContext) {
    init_test(cx);
    let backend = Arc::new(FakeSettingsSyncBackend::default());
    let work = FakeSettingsSyncServer::in_group(backend.clone(), "work");
    let home = FakeSettingsSyncServer::in_group(backend, "home");
    home.set_doc(cloud_api_client::SyncedSettings {
        group_id: "home".to_string(),
        document_id: "seeded-document".to_string(),
        kind: cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS.to_string(),
        version: 1,
        schema_epoch: crate::settings_schema_epoch(),
        doc: json!({ "buffer_font_size": 20 }),
        updated_by_system_id: None,
        provenance: BTreeMap::new(),
    });

    let device = make_device(
        &work,
        "a",
        r#"{ "buffer_font_size": 15, "theme": "One Dark" }"#,
        cx,
    )
    .await;
    settle(cx);
    assert_eq!(
        work.doc().unwrap().doc,
        json!({ "buffer_font_size": 15, "theme": "One Dark" })
    );

    let TestDevice {
        fs,
        engine,
        settings_path,
    } = device;
    drop(engine);
    cx.run_until_parked();
    let moved = make_device_with(
        home.clone(),
        fs,
        settings_path,
        PathBuf::from("/a/sync_state.json"),
        cx,
    )
    .await;
    settle(cx);
    assert_eq!(
        moved
            .engine
            .read_with(cx, |engine, _| engine.pending_decision().cloned()),
        Some(PendingDecision::Consent)
    );
    assert_eq!(home.doc().unwrap().doc, json!({ "buffer_font_size": 20 }));

    moved
        .engine
        .update(cx, |engine, cx| engine.accept_consent(cx))
        .await
        .unwrap();
    settle(cx);
    assert_eq!(
        home.doc().unwrap().doc,
        json!({ "buffer_font_size": 20, "theme": "One Dark" })
    );
    assert_eq!(
        moved.settings_json(cx).await,
        json!({ "buffer_font_size": 20, "theme": "One Dark" })
    );
}

#[gpui::test]
async fn test_feature_flag_arriving_later_starts_syncing(cx: &mut TestAppContext) {
    init_test(cx);
    set_settings_sync_flag_override("off", cx);
    let server = FakeSettingsSyncServer::new();

    let _device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);
    assert_eq!(server.doc(), None);

    set_settings_sync_flag_override("on", cx);
    settle(cx);
    assert_eq!(
        server.doc().map(|doc| doc.doc),
        Some(json!({ "buffer_font_size": 15 }))
    );
}

#[gpui::test]
async fn test_legacy_settings_are_migrated_before_syncing(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device = make_device(
        &server,
        "a",
        r#"{
            "settings_sync": { "exclude": ["/profiles/Work/tab_size", "/context_servers/private/command/args"] },
            "context_servers": {
                "private": {
                    "command": {
                        "path": "server",
                        "args": ["--token=SECRET"]
                    }
                }
            },
            "profiles": { "Work": { "terminal": { "env": { "TOKEN": "secret" } }, "tab_size": 8, "buffer_font_size": 20 } }
        }"#,
        cx,
    )
    .await;
    settle(cx);

    let exclusions = json!([
        "/profiles/Work/tab_size",
        "/context_servers/private/command/args",
        "/context_servers/private/args",
        "/profiles/Work/settings/tab_size"
    ]);
    assert_eq!(
        server.doc().unwrap().doc,
        json!({
            "settings_sync": { "exclude": exclusions },
            "context_servers": { "private": { "command": "server" } },
            "profiles": { "Work": { "settings": { "buffer_font_size": 20 } } }
        })
    );
    assert_eq!(
        device.settings_json(cx).await,
        json!({
            "settings_sync": { "exclude": exclusions },
            "context_servers": { "private": { "command": "server", "args": ["--token=SECRET"] } },
            "profiles": {
                "Work": {
                    "settings": {
                        "terminal": { "env": { "TOKEN": "secret" } },
                        "tab_size": 8,
                        "buffer_font_size": 20
                    }
                }
            }
        })
    );
}

#[gpui::test]
async fn test_account_switch_mid_cycle_never_crosses_accounts(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);
    assert_eq!(server.push_count(), 1);

    let other_account_doc = cloud_api_client::SyncedSettings {
        group_id: "other-group".to_string(),
        document_id: "seeded-document".to_string(),
        kind: cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS.to_string(),
        version: 1,
        schema_epoch: crate::settings_schema_epoch(),
        doc: json!({ "theme": "Other Account" }),
        updated_by_system_id: None,
        provenance: BTreeMap::new(),
    };
    server.set_doc(other_account_doc.clone());
    let mut first_account_doc = server.doc().unwrap();
    first_account_doc.version += 1;
    first_account_doc.doc = json!({ "buffer_font_size": 15, "theme": "First Account" });
    server.set_doc(first_account_doc);

    let release_fetch = server.gate_next_fetch();
    sync(&device, cx);
    server.set_account(Some(FAKE_ACCOUNT_ID + 1), Some("other-group"));
    release_fetch.send(()).unwrap();
    settle(cx);

    assert_eq!(server.push_count(), 1);
    assert_eq!(server.doc(), Some(other_account_doc));
    assert_eq!(
        device.settings_json(cx).await,
        json!({ "buffer_font_size": 15 })
    );

    sync(&device, cx);
    assert_eq!(server.push_count(), 1);
    assert_eq!(
        device
            .engine
            .read_with(cx, |engine, _| engine.pending_decision().cloned()),
        Some(PendingDecision::Consent)
    );
}

#[gpui::test]
async fn test_exclusion_added_mid_cycle_blocks_the_stale_upload(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);

    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 15, "theme": "Private" }"#.to_vec(),
        )
        .await;
    let release_fetch = server.gate_next_fetch();
    settle(cx);
    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 15, "theme": "Private", "settings_sync": { "exclude": ["/theme"] } }"#
                .to_vec(),
        )
        .await;
    release_fetch.send(()).unwrap();
    settle(cx);
    settle(cx);

    let uploaded_docs = crate::SettingsSyncServer::list_history(
        server.as_ref(),
        FAKE_ACCOUNT_ID,
        cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS,
        u32::MAX,
    )
    .await
    .unwrap()
    .revisions
    .into_iter()
    .map(|revision| revision.doc)
    .collect::<Vec<_>>();
    assert_eq!(
        uploaded_docs,
        vec![
            json!({ "buffer_font_size": 15, "settings_sync": { "exclude": ["/theme"] } }),
            json!({ "buffer_font_size": 15 }),
        ]
    );
}

#[gpui::test]
async fn test_decisions_gate_every_upload(cx: &mut TestAppContext) {
    init_test(cx);
    let backend = Arc::new(FakeSettingsSyncBackend::default());
    let existing = FakeSettingsSyncServer::in_group(backend.clone(), "home");
    existing.set_doc(cloud_api_client::SyncedSettings {
        group_id: "home".to_string(),
        document_id: "seeded-document".to_string(),
        kind: cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS.to_string(),
        version: 1,
        schema_epoch: crate::settings_schema_epoch(),
        doc: json!({ "theme": "Home", "buffer_font_size": 20 }),
        updated_by_system_id: None,
        provenance: BTreeMap::new(),
    });
    let server = FakeSettingsSyncServer::unregistered(backend);
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree("/new", json!({ "settings.json": "{}" }))
        .await;
    let device = make_bare_device(
        server.clone(),
        fs,
        PathBuf::from("/new/settings.json"),
        PathBuf::from("/new/sync_state.json"),
        cx,
    );
    let pending = |cx: &mut TestAppContext| {
        device
            .engine
            .read_with(cx, |engine, _| engine.pending_decision().cloned())
    };
    settle(cx);
    assert_eq!(pending(cx), Some(PendingDecision::Consent));
    assert_eq!(server.fetch_count(), 0);

    device
        .engine
        .update(cx, |engine, cx| engine.accept_consent(cx))
        .await
        .unwrap();
    settle(cx);
    let Some(PendingDecision::JoinGroup { account_id, hosts }) = pending(cx) else {
        panic!("expected a group choice, got {:?}", pending(cx));
    };
    assert_eq!(account_id, FAKE_ACCOUNT_ID);
    assert_eq!(
        hosts
            .iter()
            .map(|host| host.group_id.as_str())
            .collect::<Vec<_>>(),
        vec!["home"]
    );

    device
        .engine
        .update(cx, |engine, cx| {
            engine.join_group(
                FAKE_ACCOUNT_ID,
                cloud_api_client::SyncedSettingsGroupTarget::Existing {
                    group_id: "home".to_string(),
                },
                "laptop".to_string(),
                cx,
            )
        })
        .await
        .unwrap();
    settle(cx);
    let Some(PendingDecision::FirstSync(preview)) = pending(cx) else {
        panic!("expected a first sync choice, got {:?}", pending(cx));
    };
    assert_eq!(
        preview.local_changes(FirstSyncChoice::ReplaceLocal),
        diff_paths(
            &PathMap::new(),
            &path_map(&[("/buffer_font_size", json!(20)), ("/theme", json!("Home"))]),
        )
    );
    assert_eq!(
        preview.remote_changes(FirstSyncChoice::ReplaceRemote),
        diff_paths(
            &path_map(&[("/buffer_font_size", json!(20)), ("/theme", json!("Home"))]),
            &PathMap::new(),
        )
    );
    assert_eq!(server.push_count(), 0);

    device.engine.update(cx, |engine, cx| {
        engine.choose_first_sync(preview.clone(), FirstSyncChoice::ReplaceRemote, cx)
    });
    settle(cx);
    assert_eq!(pending(cx), None);
    assert_eq!(server.doc().unwrap().doc, json!({}));
    assert_eq!(device.settings_json(cx).await, json!({}));

    let stale_erase = device.engine.update(cx, |engine, cx| {
        engine.erase_cloud_data(
            cloud_api_client::SyncedSettingsErasureScope::Group,
            FAKE_ACCOUNT_ID,
            Some("work".to_string()),
            cx,
        )
    });
    assert_eq!(
        stale_erase.await.unwrap_err().to_string(),
        "this device moved to another sync group"
    );
    assert_eq!(server.doc().unwrap().doc, json!({}));

    device
        .engine
        .update(cx, |engine, cx| {
            engine.erase_cloud_data(
                cloud_api_client::SyncedSettingsErasureScope::Group,
                FAKE_ACCOUNT_ID,
                Some("home".to_string()),
                cx,
            )
        })
        .await
        .unwrap();
    assert_eq!(server.doc(), None);
    assert_eq!(
        device
            .engine
            .read_with(cx, |engine, _| engine.state().map(|state| state.consented)),
        Some(false)
    );
}

#[gpui::test]
async fn test_erased_cloud_copy_is_not_reuploaded_without_consent(cx: &mut TestAppContext) {
    init_test(cx);
    let backend = Arc::new(FakeSettingsSyncBackend::default());
    let device_server = FakeSettingsSyncServer::in_group(backend.clone(), "home");
    let other_server = FakeSettingsSyncServer::in_group(backend, "home");

    let device = make_device(&device_server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);
    assert_eq!(device_server.push_count(), 1);

    let account_id = FAKE_ACCOUNT_ID;
    crate::SettingsSyncServer::erase(
        other_server.as_ref(),
        account_id,
        cloud_api_client::SyncedSettingsErasureScope::Group,
        Some("home".to_string()),
    )
    .await
    .unwrap();
    sync(&device, cx);

    assert_eq!(device_server.doc(), None);
    assert_eq!(device_server.push_count(), 1);
    assert_eq!(
        device
            .engine
            .read_with(cx, |engine, _| engine.pending_decision().cloned()),
        Some(PendingDecision::Consent)
    );
}

#[gpui::test]
async fn test_group_erase_fences_an_in_flight_initial_upload(cx: &mut TestAppContext) {
    init_test(cx);
    let backend = Arc::new(FakeSettingsSyncBackend::default());
    let device_server = FakeSettingsSyncServer::in_group(backend.clone(), "home");
    let other_server = FakeSettingsSyncServer::in_group(backend, "home");

    let release_push = device_server.gate_next_push();
    let device = make_device(&device_server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);
    crate::SettingsSyncServer::erase(
        other_server.as_ref(),
        FAKE_ACCOUNT_ID,
        cloud_api_client::SyncedSettingsErasureScope::Group,
        Some("home".to_string()),
    )
    .await
    .unwrap();
    release_push.send(()).unwrap();
    settle(cx);
    settle(cx);

    assert_eq!(device_server.doc(), None);
    assert_eq!(
        device
            .engine
            .read_with(cx, |engine, _| engine.pending_decision().cloned()),
        Some(PendingDecision::Consent)
    );
}

#[gpui::test]
async fn test_group_move_discards_an_old_group_apply(cx: &mut TestAppContext) {
    init_test(cx);
    let backend = Arc::new(FakeSettingsSyncBackend::default());
    let server = FakeSettingsSyncServer::in_group(backend.clone(), "home");
    let work = FakeSettingsSyncServer::in_group(backend, "work");
    work.set_doc(cloud_api_client::SyncedSettings {
        group_id: "work".to_string(),
        document_id: "work-document".to_string(),
        kind: cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS.to_string(),
        version: 1,
        schema_epoch: crate::settings_schema_epoch(),
        doc: json!({ "buffer_font_size": 20 }),
        updated_by_system_id: None,
        provenance: BTreeMap::new(),
    });
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/a",
        json!({ "settings.json": r#"{ "buffer_font_size": 15 }"# }),
    )
    .await;
    let device = make_consented_device(
        server.clone(),
        fs,
        PathBuf::from("/a/settings.json"),
        PathBuf::from("/a/sync_state.json"),
        cx,
    )
    .await;
    settle(cx);
    let Some(PendingDecision::FirstSync(upload_preview)) = device
        .engine
        .read_with(cx, |engine, _| engine.pending_decision().cloned())
    else {
        panic!("expected an upload preview for the empty group");
    };
    device.engine.update(cx, |engine, cx| {
        engine.choose_first_sync(upload_preview, FirstSyncChoice::ReplaceRemote, cx)
    });
    settle(cx);
    assert_eq!(server.push_count(), 1);

    let (release_tx, release_rx) = oneshot::channel::<()>();
    cx.update(|cx| {
        cx.global::<SettingsStore>()
            .enqueue_settings_file_job(move |_| {
                async move {
                    release_rx.await.ok();
                    anyhow::Ok(())
                }
                .boxed_local()
            });
    });
    let mut remote = server.doc().unwrap();
    remote.version += 1;
    remote.doc = json!({ "buffer_font_size": 16 });
    server.set_doc(remote);
    sync(&device, cx);
    device
        .engine
        .update(cx, |engine, cx| {
            engine.join_group(
                FAKE_ACCOUNT_ID,
                cloud_api_client::SyncedSettingsGroupTarget::Existing {
                    group_id: "work".to_string(),
                },
                "a".to_string(),
                cx,
            )
        })
        .await
        .unwrap();
    release_tx.send(()).unwrap();
    settle(cx);
    settle(cx);

    assert_eq!(
        device.settings_json(cx).await,
        json!({ "buffer_font_size": 15 })
    );
    let Some(PendingDecision::FirstSync(preview)) = device
        .engine
        .read_with(cx, |engine, _| engine.pending_decision().cloned())
    else {
        panic!("expected a first sync choice for the new group");
    };
    assert_eq!(
        (preview.group_id.as_str(), preview.local),
        ("work", path_map(&[("/buffer_font_size", json!(15))]))
    );
}

#[gpui::test]
async fn test_state_commit_never_overwrites_a_newer_state_from_another_process(
    cx: &mut TestAppContext,
) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();
    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);
    let state_path = PathBuf::from("/a/sync_state.json");

    let release_push = server.gate_next_push();
    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 16 }"#.to_vec(),
        )
        .await;
    settle(cx);
    let other_process_text = serde_json::to_string_pretty(&SyncState {
        format_version: 1,
        user_id: Some(FAKE_ACCOUNT_ID),
        consented: true,
        group_id: Some("fake-group".to_string()),
        base: None,
    })
    .unwrap();
    device
        .fs
        .insert_file(&state_path, other_process_text.clone().into_bytes())
        .await;
    release_push.send(()).unwrap();
    cx.run_until_parked();

    assert_eq!(server.push_count(), 2);
    assert_eq!(
        device.fs.load(&state_path).await.unwrap(),
        other_process_text
    );
}

#[gpui::test]
async fn test_first_upload_to_an_empty_group_waits_for_the_preview(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/a",
        json!({ "settings.json": r#"{ "buffer_font_size": 15 }"# }),
    )
    .await;
    let device = make_consented_device(
        server.clone(),
        fs,
        PathBuf::from("/a/settings.json"),
        PathBuf::from("/a/sync_state.json"),
        cx,
    )
    .await;
    settle(cx);
    let Some(PendingDecision::FirstSync(preview)) = device
        .engine
        .read_with(cx, |engine, _| engine.pending_decision().cloned())
    else {
        panic!("expected an upload preview for the empty group");
    };
    assert_eq!(
        (
            server.push_count(),
            preview.remote_version,
            preview.choices()
        ),
        (0, None, Vec::from([FirstSyncChoice::ReplaceRemote]))
    );

    device.engine.update(cx, |engine, cx| {
        engine.choose_first_sync(preview, FirstSyncChoice::ReplaceRemote, cx)
    });
    settle(cx);
    assert_eq!(server.doc().unwrap().doc, json!({ "buffer_font_size": 15 }));
}

#[gpui::test]
async fn test_host_actions_are_bound_to_the_displayed_account(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();
    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);
    let system_id = crate::SettingsSyncServer::system_id(server.as_ref()).unwrap();

    server.set_account(Some(FAKE_ACCOUNT_ID + 1), Some("fake-group"));
    let rename = device.engine.update(cx, |engine, cx| {
        engine.update_host(
            FAKE_ACCOUNT_ID,
            system_id.clone(),
            cloud_api_client::UpdateSyncedSettingsHostBody {
                label: Some("renamed".to_string()),
                group: None,
            },
            cx,
        )
    });
    let remove = device.engine.update(cx, |engine, cx| {
        engine.remove_host(FAKE_ACCOUNT_ID, system_id.clone(), cx)
    });
    let rejection = "signed into another account, not changing settings sync";
    assert_eq!(rename.await.unwrap_err().to_string(), rejection);
    assert_eq!(remove.await.unwrap_err().to_string(), rejection);

    let hosts = device
        .engine
        .update(cx, |engine, cx| engine.list_hosts(cx))
        .await
        .unwrap();
    assert_eq!(
        (
            hosts.account_id,
            hosts
                .hosts
                .iter()
                .map(|host| (host.system_id.as_str(), host.label.as_str()))
                .collect::<Vec<_>>()
        ),
        (
            FAKE_ACCOUNT_ID + 1,
            Vec::from([(system_id.as_str(), system_id.as_str())])
        )
    );
}

#[gpui::test]
async fn test_unverifiable_apply_pauses_without_writing(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device = make_device(&server, "a", r#"{ "tab_size": 4 }"#, cx).await;
    settle(cx);
    let events = subscribe_to_events(&device.engine, cx);

    let duplicated = r#"{ "tab_size": 2, "tab_size": 4 }"#;
    device
        .fs
        .insert_file(&device.settings_path, duplicated.as_bytes().to_vec())
        .await;
    let mut remote = server.doc().unwrap();
    remote.version += 1;
    remote.doc = json!({ "tab_size": 8 });
    server.set_doc(remote);
    sync(&device, cx);

    assert_eq!(device.settings_text(cx).await, duplicated);
    assert_eq!(
        device.engine.read_with(cx, |engine, _| engine.is_paused()),
        true
    );
    let paused_events = events
        .lock()
        .iter()
        .filter_map(|event| match event {
            SettingsSyncEvent::Paused(reason) => Some(*reason),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(paused_events, vec![PauseReason::ApplyFailed]);
}

#[gpui::test]
async fn test_first_sync_approval_is_bound_to_the_previewed_snapshot(cx: &mut TestAppContext) {
    init_test(cx);
    let backend = Arc::new(FakeSettingsSyncBackend::default());
    let other_device = FakeSettingsSyncServer::in_group(backend.clone(), "home");
    other_device.set_doc(cloud_api_client::SyncedSettings {
        group_id: "home".to_string(),
        document_id: "seeded-document".to_string(),
        kind: cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS.to_string(),
        version: 1,
        schema_epoch: crate::settings_schema_epoch(),
        doc: json!({ "buffer_font_size": 18 }),
        updated_by_system_id: None,
        provenance: BTreeMap::new(),
    });
    let server = FakeSettingsSyncServer::in_group(backend, "home");
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/a",
        json!({ "settings.json": r#"{ "buffer_font_size": 14 }"# }),
    )
    .await;
    let device = make_consented_device(
        server.clone(),
        fs,
        PathBuf::from("/a/settings.json"),
        PathBuf::from("/a/sync_state.json"),
        cx,
    )
    .await;
    let pending_preview = |cx: &mut TestAppContext| match device
        .engine
        .read_with(cx, |engine, _| engine.pending_decision().cloned())
    {
        Some(PendingDecision::FirstSync(preview)) => preview,
        other => panic!("expected a first sync preview, got {other:?}"),
    };
    settle(cx);
    let shown_preview = pending_preview(cx);

    let release_fetch = server.gate_next_fetch();
    device.engine.update(cx, |engine, cx| {
        engine.choose_first_sync(shown_preview.clone(), FirstSyncChoice::ReplaceRemote, cx)
    });
    cx.run_until_parked();
    let mut remote = other_device.doc().unwrap();
    remote.version = 2;
    remote.doc = json!({ "buffer_font_size": 18, "tab_size": 8 });
    other_device.set_doc(remote);
    release_fetch.send(()).unwrap();
    settle(cx);

    assert_eq!(server.push_count(), 0);
    assert_eq!(
        server.doc().unwrap().doc,
        json!({ "buffer_font_size": 18, "tab_size": 8 })
    );
    let fresh_preview = pending_preview(cx);
    assert_eq!(fresh_preview.remote_version, Some(2));
    assert_eq!(
        fresh_preview.remote_changes(FirstSyncChoice::ReplaceRemote),
        diff_paths(
            &path_map(&[("/buffer_font_size", json!(18)), ("/tab_size", json!(8))]),
            &path_map(&[("/buffer_font_size", json!(14))]),
        )
    );

    device.engine.update(cx, |engine, cx| {
        engine.choose_first_sync(fresh_preview, FirstSyncChoice::ReplaceRemote, cx)
    });
    settle(cx);
    assert_eq!(server.doc().unwrap().doc, json!({ "buffer_font_size": 14 }));
}

#[gpui::test]
async fn test_erasure_is_detected_mid_cycle_and_across_recreation(cx: &mut TestAppContext) {
    init_test(cx);
    let backend = Arc::new(FakeSettingsSyncBackend::default());
    let server_a = FakeSettingsSyncServer::in_group(backend.clone(), "home");
    let server_b = FakeSettingsSyncServer::in_group(backend, "home");
    let device_a = make_device(&server_a, "a", r#"{ "buffer_font_size": 14 }"#, cx).await;
    settle(cx);
    let device_b = make_device(&server_b, "b", r#"{ "buffer_font_size": 14 }"#, cx).await;
    settle(cx);
    assert_eq!(server_a.push_count(), 1);
    let pending = |device: &TestDevice, cx: &mut TestAppContext| {
        device
            .engine
            .read_with(cx, |engine, _| engine.pending_decision().cloned())
    };

    let edited = r#"{ "buffer_font_size": 14, "theme": "One Dark" }"#;
    device_a
        .fs
        .insert_file(&device_a.settings_path, edited.as_bytes().to_vec())
        .await;
    let release_push = server_a.gate_next_push();
    settle(cx);
    crate::SettingsSyncServer::erase(
        server_b.as_ref(),
        FAKE_ACCOUNT_ID,
        cloud_api_client::SyncedSettingsErasureScope::Group,
        Some("home".to_string()),
    )
    .await
    .unwrap();
    release_push.send(()).unwrap();
    settle(cx);
    cx.executor().advance_clock(Duration::from_secs(5));
    cx.run_until_parked();

    assert_eq!(server_a.doc(), None);
    assert_eq!(pending(&device_a, cx), Some(PendingDecision::Consent));
    assert_eq!(device_a.settings_text(cx).await, edited);

    crate::SettingsSyncServer::push(
        server_a.as_ref(),
        FAKE_ACCOUNT_ID,
        cloud_api_client::UpdateSyncedSettingsBody {
            kind: cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS.to_string(),
            base_version: None,
            base_document_id: None,
            schema_epoch: crate::settings_schema_epoch(),
            doc: json!({ "theme": "Recreated" }),
            expected_group_id: server_a.group_id().unwrap(),
            exclusions: Vec::new(),
        },
    )
    .await
    .unwrap();
    let recreated = server_b.doc().unwrap();
    assert_eq!(recreated.version, 1);
    sync(&device_b, cx);

    assert_eq!(server_b.doc(), Some(recreated));
    assert_eq!(pending(&device_b, cx), Some(PendingDecision::Consent));
    assert_eq!(
        device_b.settings_json(cx).await,
        json!({ "buffer_font_size": 14 })
    );
}

#[gpui::test]
async fn test_upgraded_client_publishes_its_schema_epoch_when_documents_match(
    cx: &mut TestAppContext,
) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();
    server.set_doc(cloud_api_client::SyncedSettings {
        group_id: "fake-group".to_string(),
        document_id: "seeded-document".to_string(),
        kind: cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS.to_string(),
        version: 7,
        schema_epoch: crate::settings_schema_epoch() - 1,
        doc: json!({
            "edit_predictions": { "copilot": { "enterprise_uri": "https://enterprise.example" } }
        }),
        updated_by_system_id: None,
        provenance: BTreeMap::new(),
    });

    let _device = make_device(
        &server,
        "a",
        r#"{ "copilot": { "enterprise_uri": "https://enterprise.example" } }"#,
        cx,
    )
    .await;
    settle(cx);

    let doc = server.doc().unwrap();
    assert_eq!(
        (doc.version, doc.schema_epoch, doc.doc),
        (
            8,
            crate::settings_schema_epoch(),
            json!({ "copilot": { "enterprise_uri": "https://enterprise.example" } })
        )
    );
}

#[gpui::test]
async fn test_edit_during_a_successful_push_is_kept(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();
    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    settle(cx);
    let mut remote = server.doc().unwrap();
    remote.version += 1;
    remote.doc = json!({ "buffer_font_size": 15, "theme": "One Dark" });
    server.set_doc(remote);

    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 16 }"#.to_vec(),
        )
        .await;
    let release_push = server.gate_next_push();
    settle(cx);
    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 18 }"#.to_vec(),
        )
        .await;
    cx.run_until_parked();
    release_push.send(()).unwrap();
    settle(cx);
    settle(cx);

    let expected = json!({ "buffer_font_size": 18, "theme": "One Dark" });
    assert_eq!(server.doc().unwrap().doc, expected);
    assert_eq!(device.settings_json(cx).await, expected);
}

#[gpui::test]
async fn test_removing_an_exclusion_uploads_the_value(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();
    let device = make_device(
        &server,
        "a",
        r#"{ "buffer_font_size": 15, "settings_sync": { "exclude": ["/buffer_font_size"] } }"#,
        cx,
    )
    .await;
    settle(cx);
    assert_eq!(
        server.doc().unwrap().doc,
        json!({ "settings_sync": { "exclude": ["/buffer_font_size"] } })
    );

    device
        .fs
        .insert_file(
            &device.settings_path,
            br#"{ "buffer_font_size": 15, "settings_sync": { "exclude": [] } }"#.to_vec(),
        )
        .await;
    settle(cx);
    settle(cx);

    assert_eq!(
        server.doc().unwrap().doc,
        json!({ "buffer_font_size": 15, "settings_sync": { "exclude": [] } })
    );
}

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        cx.update_flags(false, vec!["settings-sync".to_string()]);
        SettingsStore::update(cx, |store, cx| {
            store.update_user_settings(cx, |content| {
                content.settings_sync = Some(settings::SettingsSyncContent {
                    enabled: Some(true),
                    exclude: None,
                });
            });
        });
    });
}

fn set_settings_sync_flag_override(value: &str, cx: &mut TestAppContext) {
    cx.update(|cx| {
        SettingsStore::update(cx, |store, cx| {
            store.update_user_settings(cx, |content| {
                content.feature_flags = Some(settings::FeatureFlagsMap(
                    [("settings-sync".to_string(), value.to_string())]
                        .into_iter()
                        .collect(),
                ));
            });
        });
    });
}

struct TestDevice {
    fs: Arc<FakeFs>,
    engine: Entity<SettingsSyncEngine>,
    settings_path: PathBuf,
}

impl TestDevice {
    async fn settings_text(&self, _cx: &mut TestAppContext) -> String {
        self.fs.load(&self.settings_path).await.unwrap()
    }

    async fn settings_json(&self, cx: &mut TestAppContext) -> serde_json::Value {
        settings_json::parse_json_with_comments(&self.settings_text(cx).await).unwrap()
    }
}

async fn make_device(
    server: &Arc<FakeSettingsSyncServer>,
    name: &str,
    settings_text: &str,
    cx: &mut TestAppContext,
) -> TestDevice {
    let fs = FakeFs::new(cx.background_executor.clone());
    let root = PathBuf::from(format!("/{name}"));
    fs.insert_tree(&root, json!({ "settings.json": settings_text }))
        .await;
    make_device_with(
        server.clone(),
        fs,
        root.join("settings.json"),
        root.join("sync_state.json"),
        cx,
    )
    .await
}

async fn make_device_with(
    server: Arc<FakeSettingsSyncServer>,
    fs: Arc<FakeFs>,
    settings_path: PathBuf,
    state_path: PathBuf,
    cx: &mut TestAppContext,
) -> TestDevice {
    let device = make_consented_device(server, fs, settings_path, state_path, cx).await;
    cx.update(|cx| {
        cx.subscribe(&device.engine, |engine, event: &SettingsSyncEvent, cx| {
            if let SettingsSyncEvent::DecisionRequired = event
                && let Some(PendingDecision::FirstSync(preview)) =
                    engine.read(cx).pending_decision().cloned()
            {
                engine.update(cx, |engine, cx| {
                    engine.choose_first_sync(preview, FirstSyncChoice::Merge, cx)
                });
            }
        })
        .detach();
    });
    device
}

async fn make_consented_device(
    server: Arc<FakeSettingsSyncServer>,
    fs: Arc<FakeFs>,
    settings_path: PathBuf,
    state_path: PathBuf,
    cx: &mut TestAppContext,
) -> TestDevice {
    if !fs.is_file(&state_path).await {
        let state = SyncState {
            format_version: 1,
            user_id: Some(FAKE_ACCOUNT_ID),
            consented: true,
            group_id: None,
            base: None,
        };
        fs.insert_file(&state_path, serde_json::to_vec(&state).unwrap())
            .await;
    }
    make_bare_device(server, fs, settings_path, state_path, cx)
}

fn make_bare_device(
    server: Arc<FakeSettingsSyncServer>,
    fs: Arc<FakeFs>,
    settings_path: PathBuf,
    state_path: PathBuf,
    cx: &mut TestAppContext,
) -> TestDevice {
    let engine = cx.new(|cx| {
        SettingsSyncEngine::new(
            server,
            fs.clone(),
            SyncedDocument {
                kind: cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS,
                file_path: settings_path.clone(),
                build_strategy: SettingsJsonStrategy::user_settings,
            },
            state_path,
            cx,
        )
    });
    TestDevice {
        fs,
        engine,
        settings_path,
    }
}

fn sync(device: &TestDevice, cx: &mut TestAppContext) {
    device
        .engine
        .update(cx, |engine, cx| engine.schedule_sync(cx));
    settle(cx);
}

fn settle(cx: &mut TestAppContext) {
    cx.run_until_parked();
    cx.executor().advance_clock(SYNC_DEBOUNCE);
    cx.run_until_parked();
}

fn subscribe_to_events(
    engine: &Entity<SettingsSyncEngine>,
    cx: &mut TestAppContext,
) -> Arc<Mutex<Vec<SettingsSyncEvent>>> {
    let events = Arc::new(Mutex::new(Vec::new()));
    cx.update(|cx| {
        cx.subscribe(engine, {
            let events = events.clone();
            move |_, event: &SettingsSyncEvent, _| events.lock().push(event.clone())
        })
        .detach();
    });
    events
}

fn first_conflict_batch(events: &Mutex<Vec<SettingsSyncEvent>>) -> ConflictBatch {
    events
        .lock()
        .iter()
        .find_map(|event| match event {
            SettingsSyncEvent::ConflictsResolved(batch) => Some(batch.clone()),
            _ => None,
        })
        .unwrap()
}

fn sync_failed_count(events: &Mutex<Vec<SettingsSyncEvent>>) -> usize {
    events
        .lock()
        .iter()
        .filter(|event| matches!(event, SettingsSyncEvent::SyncFailed(_)))
        .count()
}

fn path_map(entries: &[(&str, serde_json::Value)]) -> PathMap {
    entries
        .iter()
        .map(|(pointer, value)| (SyncPath::parse(pointer).unwrap(), value.clone()))
        .collect()
}

fn random_path_map(
    rng: &mut StdRng,
    path_pool: &[&str],
    value_pool: &[serde_json::Value],
) -> PathMap {
    let mut paths = PathMap::new();
    for pointer in path_pool {
        if rng.random_bool(0.5) {
            paths.insert(
                SyncPath::parse(pointer).unwrap(),
                random_value(rng, value_pool),
            );
        }
    }
    paths
}

fn mutated_path_map(
    rng: &mut StdRng,
    base: &PathMap,
    path_pool: &[&str],
    value_pool: &[serde_json::Value],
) -> PathMap {
    let mut paths = base.clone();
    for pointer in path_pool {
        let path = SyncPath::parse(pointer).unwrap();
        match rng.random_range(0..10) {
            0..6 => {}
            6..9 => {
                paths.insert(path, random_value(rng, value_pool));
            }
            _ => {
                paths.remove(&path);
            }
        }
    }
    paths
}

fn random_value(rng: &mut StdRng, value_pool: &[serde_json::Value]) -> serde_json::Value {
    value_pool[rng.random_range(0..value_pool.len())].clone()
}
