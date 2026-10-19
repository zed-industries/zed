#![cfg(unix)]

use crate::{Content, ProviderCommand, ProviderConnection, ProviderError, ProviderNotification};
use futures::future::{Either, select};
use gpui::TestAppContext;
use serde_json::{Value, json};
use std::{
    ffi::OsString,
    future::Future,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};

struct Fixture {
    connection: Arc<ProviderConnection>,
    directory: tempfile::TempDir,
    events: PathBuf,
}

fn fixture(scenario: &str, cx: &TestAppContext) -> Fixture {
    cx.executor().allow_parking();
    let directory = tempfile::tempdir().expect("fixture directory");
    let events = directory.path().join("events.jsonl");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixture_provider.py");
    let command = ProviderCommand {
        path: std::env::var_os("VCS_FIXTURE_PYTHON")
            .unwrap_or_else(|| OsString::from("python3"))
            .into(),
        arguments: vec![
            script.into_os_string(),
            "--scenario".into(),
            scenario.into(),
            "--events".into(),
            events.clone().into_os_string(),
        ],
        env: Default::default(),
    };
    let connection =
        ProviderConnection::start(command, directory.path(), cx.background_executor.clone())
            .expect("start fixture provider");
    Fixture {
        connection,
        directory,
        events,
    }
}

async fn bounded<T>(future: impl Future<Output = T>, cx: &TestAppContext) -> T {
    let timeout = cx.background_executor.timer(Duration::from_secs(5));
    futures::pin_mut!(future, timeout);
    match select(future, timeout).await {
        Either::Left((result, _)) => result,
        Either::Right(_) => panic!("timed out during VCS fixture exchange"),
    }
}

fn events(fixture: &Fixture) -> Vec<Value> {
    read_events(&fixture.events)
}

fn read_events(path: &Path) -> Vec<Value> {
    let contents = std::fs::read_to_string(path).expect("fixture event log");
    let complete = contents.rfind('\n').map_or("", |end| &contents[..end]);
    complete
        .lines()
        .map(|line| serde_json::from_str(line).expect("fixture event"))
        .collect()
}

async fn provider_event(path: PathBuf, event: &'static str) {
    // The fixture writes this receipt outside GPUI's simulated clock.
    smol::unblock(move || {
        let deadline = Instant::now() + Duration::from_secs(5);
        while !read_events(&path)
            .iter()
            .any(|record| record["event"] == event)
        {
            assert!(Instant::now() < deadline, "fixture event {event} timed out");
            std::thread::sleep(Duration::from_millis(5));
        }
    })
    .await;
}

async fn shutdown_child_taken(connection: Arc<ProviderConnection>) {
    smol::unblock(move || {
        let deadline = Instant::now() + Duration::from_secs(5);
        while !connection.has_taken_shutdown_child() {
            assert!(Instant::now() < deadline, "shutdown did not take its child");
            std::thread::sleep(Duration::from_millis(5));
        }
    })
    .await;
}

#[gpui::test]
async fn stdio_roundtrip_uses_opaque_revisions_and_explicit_content_kinds(cx: &mut TestAppContext) {
    let fixture = fixture("normal", cx);
    let initialized = bounded(fixture.connection.initialize(), cx)
        .await
        .expect("initialize");
    assert!(initialized.capabilities.read_only);
    let scope = url::Url::from_directory_path(fixture.directory.path())
        .expect("scope URI")
        .to_string();
    let repositories = bounded(fixture.connection.discover(scope), cx)
        .await
        .expect("discover");
    let repository = repositories.first().expect("fixture repository");
    let snapshot = bounded(
        fixture
            .connection
            .snapshot(repository.id.clone(), "submitted".into()),
        cx,
    )
    .await
    .expect("snapshot");
    assert_eq!(snapshot.revision_id, "r42");
    assert_eq!(
        snapshot.changes.first().expect("change").new_mode,
        Some(0o100644)
    );
    for (path, expected) in [
        (
            "src/example.txt",
            Content::Text {
                text: "baseline — r42\n".into(),
            },
        ),
        ("absent.txt", Content::Absent),
        ("binary.bin", Content::Binary),
        (
            "link",
            Content::Symlink {
                target: "../target".into(),
            },
        ),
    ] {
        let content = bounded(
            fixture.connection.read_content(
                repository.id.clone(),
                snapshot.id.clone(),
                "submitted".into(),
                path.into(),
            ),
            cx,
        )
        .await
        .expect("baseline content");
        assert_eq!(content, expected);
    }
    bounded(fixture.connection.shutdown(), cx)
        .await
        .expect("shutdown");
    assert!(
        events(&fixture)
            .iter()
            .any(|event| event["event"] == "exit-received")
    );
    assert!(!fixture.directory.path().join(".git").exists());
}

#[gpui::test]
async fn completed_reply_survives_immediate_provider_eof(cx: &mut TestAppContext) {
    let fixture = fixture("reply-eof", cx);
    bounded(fixture.connection.initialize(), cx)
        .await
        .expect("initialize");
    let notifications = fixture.connection.notifications();
    let content = bounded(
        fixture.connection.read_content(
            "fixture-repository".into(),
            "snapshot:7".into(),
            "submitted".into(),
            "src/example.txt".into(),
        ),
        cx,
    )
    .await
    .expect("complete reply before EOF");
    assert_eq!(
        content,
        Content::Text {
            text: "baseline — r42\n".repeat(8192)
        }
    );
    assert!(matches!(
        bounded(notifications.recv(), cx)
            .await
            .expect("exit notification"),
        ProviderNotification::Exited { .. }
    ));
    assert!(!fixture.connection.is_alive());
}

#[gpui::test]
async fn provider_crash_rejects_all_pending_requests(cx: &mut TestAppContext) {
    let fixture = fixture("crash", cx);
    bounded(fixture.connection.initialize(), cx)
        .await
        .expect("initialize");
    let requests = async {
        futures::join!(
            fixture
                .connection
                .snapshot("fixture-repository".into(), "submitted".into()),
            fixture
                .connection
                .snapshot("fixture-repository".into(), "submitted".into()),
        )
    };
    let (first, second) = bounded(requests, cx).await;
    assert!(first.is_err());
    assert!(second.is_err());
    assert!(!fixture.connection.is_alive());
}

#[gpui::test]
async fn typed_provider_errors_retain_their_data(cx: &mut TestAppContext) {
    let fixture = fixture("error", cx);
    bounded(fixture.connection.initialize(), cx)
        .await
        .expect("initialize");
    let error = bounded(
        fixture
            .connection
            .snapshot("fixture-repository".into(), "submitted".into()),
        cx,
    )
    .await
    .expect_err("typed provider rejection");
    let error = error
        .downcast_ref::<ProviderError>()
        .expect("preserved protocol error");
    assert_eq!(error.code, -32044);
    assert_eq!(error.message, "Snapshot needs refresh");
    assert_eq!(
        error.data,
        Some(json!({"kind":"stale","revision":"r42","retryable":true}))
    );
    assert!(fixture.connection.is_alive());
    bounded(fixture.connection.shutdown(), cx)
        .await
        .expect("shutdown");
}

#[gpui::test]
async fn dropping_a_request_emits_cancellation_without_closing_the_connection(
    cx: &mut TestAppContext,
) {
    let fixture = fixture("pending", cx);
    bounded(fixture.connection.initialize(), cx)
        .await
        .expect("initialize");
    let notifications = fixture.connection.notifications();
    let request = cx.background_executor.spawn({
        let connection = fixture.connection.clone();
        async move {
            connection
                .snapshot("fixture-repository".into(), "submitted".into())
                .await
        }
    });
    assert!(matches!(
        bounded(notifications.recv(), cx)
            .await
            .expect("request received"),
        ProviderNotification::DidChange { .. }
    ));
    drop(request);
    assert!(matches!(
        bounded(notifications.recv(), cx)
            .await
            .expect("cancel received"),
        ProviderNotification::DidChange { .. }
    ));
    let recorded = events(&fixture);
    let pending = recorded
        .iter()
        .find(|event| event["event"] == "request-pending")
        .expect("pending request");
    let cancelled = recorded
        .iter()
        .find(|event| event["event"] == "cancel-received")
        .expect("cancel notification");
    assert_eq!(pending["id"], cancelled["id"]);
    assert!(fixture.connection.is_alive());
    bounded(fixture.connection.shutdown(), cx)
        .await
        .expect("shutdown");
}

#[gpui::test]
async fn invalid_or_oversized_frames_fail_the_connection(cx: &mut TestAppContext) {
    for scenario in ["malformed", "oversized"] {
        let fixture = fixture(scenario, cx);
        bounded(fixture.connection.initialize(), cx)
            .await
            .expect("initialize");
        assert!(
            bounded(
                fixture
                    .connection
                    .snapshot("fixture-repository".into(), "submitted".into()),
                cx
            )
            .await
            .is_err()
        );
        assert!(!fixture.connection.is_alive());
    }
}

#[gpui::test]
async fn stderr_flood_is_drained_and_retains_only_a_bounded_utf8_tail(cx: &mut TestAppContext) {
    let fixture = fixture("stderr-flood", cx);
    bounded(fixture.connection.initialize(), cx)
        .await
        .expect("initialize despite stderr flood");
    bounded(
        async {
            while !fixture
                .connection
                .diagnostics()
                .ends_with("fixture-stderr-tail\n")
            {
                cx.background_executor
                    .timer(Duration::from_millis(10))
                    .await;
            }
        },
        cx,
    )
    .await;
    let diagnostics = fixture.connection.diagnostics();
    assert!(diagnostics.len() <= 64 * 1024);
    assert!(!diagnostics.contains("discarded-prefix"));
    bounded(fixture.connection.shutdown(), cx)
        .await
        .expect("shutdown");
}

async fn process_is_running(pid: u64) -> bool {
    let output = util::command::new_command("ps")
        .args(["-p", &pid.to_string(), "-o", "stat="])
        .output()
        .await
        .expect("inspect fixture process");
    assert!(output.stderr.is_empty(), "process status query failed");
    let state = String::from_utf8_lossy(&output.stdout);
    !state.trim().is_empty() && !state.trim().starts_with('Z')
}

fn fixture_process_ids(fixture: &Fixture) -> Vec<u64> {
    let recorded = events(fixture);
    ["provider-started", "descendant-started"]
        .into_iter()
        .map(|name| {
            recorded
                .iter()
                .find(|event| event["event"] == name)
                .and_then(|event| event["pid"].as_u64())
                .expect("fixture process id")
        })
        .collect()
}

async fn processes_exit(ids: Vec<u64>) {
    smol::unblock(move || {
        let deadline = Instant::now() + Duration::from_secs(5);
        while smol::block_on(futures::future::join_all(
            ids.iter().map(|pid| process_is_running(*pid)),
        ))
        .into_iter()
        .any(|running| running)
        {
            assert!(Instant::now() < deadline, "fixture processes did not exit");
            std::thread::sleep(Duration::from_millis(5));
        }
    })
    .await;
}

#[gpui::test]
async fn dropping_connection_stops_provider_and_its_descendant(cx: &mut TestAppContext) {
    let fixture = fixture("descendant", cx);
    bounded(fixture.connection.initialize(), cx)
        .await
        .expect("initialize descendant fixture");
    let ids = fixture_process_ids(&fixture);
    for pid in &ids {
        assert!(process_is_running(*pid).await);
    }
    drop(fixture.connection);
    processes_exit(ids).await;
}

#[gpui::test]
async fn cancelled_shutdown_waiter_cleans_the_taken_child_and_its_group(cx: &mut TestAppContext) {
    let fixture = fixture("hang-after-exit", cx);
    bounded(fixture.connection.initialize(), cx)
        .await
        .expect("initialize shutdown fixture");
    let ids = fixture_process_ids(&fixture);
    let shutdown = cx.background_executor.spawn({
        let connection = fixture.connection.clone();
        async move { connection.shutdown().await }
    });
    provider_event(fixture.events.clone(), "exit-received").await;
    shutdown_child_taken(fixture.connection.clone()).await;
    assert!(
        events(&fixture)
            .iter()
            .any(|event| event["event"] == "shutdown-received")
    );
    for pid in &ids {
        assert!(process_is_running(*pid).await);
    }
    drop(shutdown);
    drop(fixture.connection);
    processes_exit(ids).await;
}

#[gpui::test]
async fn natural_parent_exit_still_cleans_descendants_holding_stdout(cx: &mut TestAppContext) {
    let fixture = fixture("exit-leaves-descendant", cx);
    bounded(fixture.connection.initialize(), cx)
        .await
        .expect("initialize surviving-descendant fixture");
    let ids = fixture_process_ids(&fixture);
    for pid in &ids {
        assert!(process_is_running(*pid).await);
    }
    bounded(fixture.connection.shutdown(), cx)
        .await
        .expect("shutdown cleans the group after its leader exits");
    assert!(
        events(&fixture)
            .iter()
            .any(|event| event["event"] == "exit-received")
    );
    processes_exit(ids).await;
}

#[gpui::test]
async fn exit_before_request_registration_never_leaves_initialization_pending(
    cx: &mut TestAppContext,
) {
    for _ in 0..4 {
        let fixture = fixture("early-exit", cx);
        assert!(bounded(fixture.connection.initialize(), cx).await.is_err());
        assert!(!fixture.connection.is_alive());
    }
}
