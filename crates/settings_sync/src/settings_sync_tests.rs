use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use fs::{FakeFs, Fs as _};
use gpui::{AppContext as _, Entity, TestAppContext};
use parking_lot::Mutex;
use serde_json::json;
use settings::SettingsStore;

use crate::classifier::DocumentClassifier;
use crate::engine::{SettingsSyncEngine, SettingsSyncEvent};
use crate::merge::{
    ExclusionSet, PathMap, apply_ops_to_text, diff_paths, flatten_doc, merge_three_way, unflatten,
};
use crate::server::FakeSettingsSyncServer;
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
fn test_exclusions_strip_subtrees() {
    let mut exclusions = ExclusionSet::built_in();
    exclusions.insert(SyncPath::parse("/macos/buffer_font_size").unwrap());

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
                // The font size everywhere.
                "languages": {
                    "Rust": {
                        "tab_size": 4
                    }
                },
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
    cx.run_until_parked();

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
    cx.run_until_parked();

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
                // Solarized forever.
                "buffer_font_size": 15,
                "theme": "Solarized Dark"
            }
        "#}
    );

    sync(&device_a, cx);
    pretty_assertions::assert_eq!(
        device_a.settings_text(cx).await,
        indoc::indoc! {r#"
            {
                // The font size everywhere.
                "theme": "Solarized Dark",
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
    cx.run_until_parked();
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
    cx.run_until_parked();

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
async fn test_conflict_remote_wins_and_emits_event(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 }"#, cx).await;
    cx.run_until_parked();
    assert_eq!(server.doc().unwrap().version, 1);

    let events = Arc::new(Mutex::new(Vec::new()));
    cx.update(|cx| {
        cx.subscribe(&device.engine, {
            let events = events.clone();
            move |_, event: &SettingsSyncEvent, _| events.lock().push(event.clone())
        })
        .detach();
    });

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

    let events = events.lock().clone();
    let conflicts = events
        .iter()
        .find_map(|event| match event {
            SettingsSyncEvent::ConflictsResolved(conflicts) => Some(conflicts.clone()),
            _ => None,
        })
        .unwrap();
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].path.to_string(), "/buffer_font_size");
    assert_eq!(conflicts[0].local, Some(json!(17)));
    assert_eq!(conflicts[0].remote, Some(json!(16)));
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
    let events = Arc::new(Mutex::new(Vec::new()));
    cx.update(|cx| {
        cx.subscribe(&device.engine, {
            let events = events.clone();
            move |_, event: &SettingsSyncEvent, _| events.lock().push(event.clone())
        })
        .detach();
    });
    cx.run_until_parked();

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
async fn test_malformed_local_json_skips_cycle(cx: &mut TestAppContext) {
    init_test(cx);
    let server = FakeSettingsSyncServer::new();

    let device = make_device(&server, "a", r#"{ "buffer_font_size": 15 "#, cx).await;
    cx.run_until_parked();

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
    let settings_path = root.join("settings.json");
    let engine = cx.new(|cx| {
        SettingsSyncEngine::new(
            server.clone(),
            fs.clone(),
            settings_path.clone(),
            root.join("sync_state.json"),
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
    cx.run_until_parked();
}

fn path_map(entries: &[(&str, serde_json::Value)]) -> PathMap {
    entries
        .iter()
        .map(|(pointer, value)| (SyncPath::parse(pointer).unwrap(), value.clone()))
        .collect()
}
