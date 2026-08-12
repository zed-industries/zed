use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use feature_flags::FeatureFlagAppExt as _;
use fs::{FakeFs, Fs as _};
use gpui::{AppContext as _, Entity, TestAppContext};
use parking_lot::Mutex;
use rand::rngs::StdRng;
use rand::{Rng as _, SeedableRng as _};
use serde_json::json;
use settings::SettingsStore;

use crate::classifier::DocumentClassifier;
use crate::engine::{
    MAX_PUSH_ATTEMPTS, SYNC_DEBOUNCE, SettingsSyncEngine, SettingsSyncEvent, SyncState,
    SyncedDocument, load_state,
};
use crate::merge::{
    BUILT_IN_EXCLUSIONS, Conflict, ExclusionPattern, ExclusionSet, PathMap, apply_ops_to_text,
    diff_paths, drop_prefix_overlaps, flatten_doc, merge_three_way, unflatten,
};
use crate::server::{FakeSettingsSyncBackend, FakeSettingsSyncServer};
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

    let mut paths = path_map(&[
        (
            "/audio",
            json!({ "input_audio_device": "mic", "experimental.control_input_volume": true }),
        ),
        ("/theme", json!("One Dark")),
    ]);
    exclusions.strip(&mut paths);

    assert_eq!(paths, path_map(&[("/theme", json!("One Dark"))]));
}

#[test]
fn test_exclusions_from_synced_list() {
    let mut exclusions = ExclusionSet::built_in();
    let paths = path_map(&[(
        "/settings_sync/exclude",
        json!(["/experimental_thing", "/languages/Rust"]),
    )]);
    exclusions.extend_from_flattened(&paths);

    assert_eq!(
        exclusions.is_excluded(&SyncPath::parse("/experimental_thing").unwrap()),
        true
    );
    assert_eq!(
        exclusions.is_excluded(&SyncPath::parse("/languages/Rust/tab_size").unwrap()),
        true
    );
    assert_eq!(
        exclusions.is_excluded(&SyncPath::parse("/languages/Go/tab_size").unwrap()),
        false
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
    assert_eq!(paths, path_map(&[("/theme", json!("One Dark"))]));
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
            "/context_servers/*/env",
            "/context_servers/*/headers",
            "/context_servers/*/oauth",
            "/agent_servers/*/env",
        ]
    );
}

#[test]
fn test_drop_prefix_overlaps() {
    let mut paths = path_map(&[
        ("/a", json!(1)),
        ("/a/b", json!(2)),
        ("/a/b/c", json!(3)),
        ("/ab", json!(4)),
    ]);
    drop_prefix_overlaps(&mut paths);
    assert_eq!(paths, path_map(&[("/a/b/c", json!(3)), ("/ab", json!(4))]));
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
    );
    let process_b = make_device_with(
        server.clone(),
        fs.clone(),
        settings_path.clone(),
        state_path.clone(),
        cx,
    );
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
            "home",
            cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS,
            99,
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
        kind: cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS.to_string(),
        version: 7,
        schema_epoch: crate::settings_schema_epoch(),
        doc: json!({
            "proxy": "socks5h://leaked:1080",
            "theme": "One Dark"
        }),
        updated_by_system_id: None,
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
async fn test_exclusions_from_merged_settings(cx: &mut TestAppContext) {
    init_test(cx);
    cx.update(|cx| {
        SettingsStore::update(cx, |store, cx| {
            store.update_user_settings(cx, |content| {
                content.settings_sync = Some(settings::SettingsSyncContent {
                    enabled: Some(true),
                    exclude: Some(vec!["/ui_font_size".to_string()]),
                });
            });
        });
    });
    let server = FakeSettingsSyncServer::new();

    let device = make_device(
        &server,
        "a",
        r#"{ "ui_font_size": 20, "buffer_font_size": 15 }"#,
        cx,
    )
    .await;
    settle(cx);

    let doc = server.doc().unwrap();
    assert_eq!(doc.doc, json!({ "buffer_font_size": 15 }));
    assert_eq!(
        device.settings_json(cx).await,
        json!({ "ui_font_size": 20, "buffer_font_size": 15 })
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

    let conflicts = first_conflict_batch(&events);
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

    let conflicts = first_conflict_batch(&events);
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

    let conflicts = first_conflict_batch(&events);
    device.engine.update(cx, |engine, cx| {
        engine.revert_conflicts(conflicts, cx).detach();
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
        .filter(|event| matches!(event, SettingsSyncEvent::Paused))
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
        kind: cloud_api_client::SYNCED_SETTINGS_KIND_SETTINGS.to_string(),
        version: 3,
        schema_epoch: crate::settings_schema_epoch() + 1,
        doc: json!({ "theme": "One Dark" }),
        updated_by_system_id: None,
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
    settle(cx);
    sync(&device, cx);

    assert_eq!(server.doc(), None);
    assert_eq!(server.fetch_count(), 0);
    assert_eq!(server.push_count(), 0);

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
    settle(cx);

    assert_eq!(server.doc(), None);
    assert_eq!(server.push_count(), 0);
    assert_eq!(
        device.fs.load(&device.settings_path).await.unwrap(),
        r#"{ "buffer_font_size": 15 "#
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
}

fn make_device_with(
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
                build_classifier: DocumentClassifier::for_user_settings,
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

fn first_conflict_batch(events: &Mutex<Vec<SettingsSyncEvent>>) -> Vec<Conflict> {
    events
        .lock()
        .iter()
        .find_map(|event| match event {
            SettingsSyncEvent::ConflictsResolved(conflicts) => Some(conflicts.clone()),
            _ => None,
        })
        .unwrap()
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
