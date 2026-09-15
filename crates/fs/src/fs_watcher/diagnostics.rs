use super::{OsWatcher, OsWatcherKind};
use parking_lot::Mutex;
use serde::Serialize;
use std::{
    cell::LazyCell,
    collections::VecDeque,
    path::PathBuf,
    sync::{Arc, Weak},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

const EVENT_CAPACITY: usize = 10_000;

/// An opt-in recording. Watchers hold only weak references to its buffer, so
/// dropping this value stops collection without changing any watches.
pub struct WatchRecording {
    state: Arc<Mutex<RecordingState>>,
    watchers: Vec<Arc<OsWatcher>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct WatchSnapshot {
    pub started_at_unix_millis: u128,
    pub captured_at_unix_millis: u128,
    pub capacity: usize,
    pub dropped_events: u64,
    pub watchers: Vec<WatcherSnapshot>,
    pub events: Vec<Arc<WatchDiagnosticEvent>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct WatcherSnapshot {
    pub backend: OsWatcherKind,
    pub recursive: bool,
    pub roots: Vec<WatchRoot>,
    pub cooldown_remaining_millis: Option<u128>,
}

#[derive(Clone, Debug, Serialize)]
pub struct WatchRoot {
    pub path: String,
    pub registrations: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct WatchDiagnosticEvent {
    pub timestamp_unix_millis: u128,
    pub backend: OsWatcherKind,
    pub operation: String,
    pub event_kind: Option<String>,
    /// Paths are displayed lossily so a non-UTF-8 filename cannot prevent export.
    pub paths: Vec<String>,
    pub detail: String,
    pub rescan: bool,
}

impl WatchDiagnosticEvent {
    pub(super) fn new(
        backend: OsWatcherKind,
        operation: &str,
        paths: &[PathBuf],
        detail: String,
        rescan: bool,
    ) -> Self {
        Self {
            timestamp_unix_millis: unix_millis(),
            backend,
            operation: operation.into(),
            event_kind: None,
            paths: paths
                .iter()
                .map(|path| path.to_string_lossy().into_owned())
                .collect(),
            detail,
            rescan,
        }
    }

    pub(super) fn from_notify_event(backend: OsWatcherKind, event: &notify::Event) -> Self {
        Self {
            event_kind: Some(format!("{:?}", event.kind)),
            ..Self::new(
                backend,
                "event",
                &event.paths,
                format!("{:?} {:?}", event.kind, event.attrs),
                event.need_rescan(),
            )
        }
    }
}

struct RecordingState {
    started_at_unix_millis: u128,
    dropped_events: u64,
    events: VecDeque<Arc<WatchDiagnosticEvent>>,
}

#[derive(Default)]
pub(super) struct DiagnosticRecorder {
    recordings: Mutex<Vec<Weak<Mutex<RecordingState>>>>,
}

impl DiagnosticRecorder {
    pub(super) fn record(&self, event: impl FnOnce() -> WatchDiagnosticEvent) {
        let mut recordings = self.recordings.lock();
        // Formatting raw events and allocating paths must not happen when
        // there is no debug window recording them.
        let event = LazyCell::new(|| Arc::new(event()));
        recordings.retain(|recording| {
            let Some(recording) = recording.upgrade() else {
                return false;
            };
            let mut recording = recording.lock();
            if recording.events.len() == EVENT_CAPACITY {
                recording.events.pop_front();
                recording.dropped_events += 1;
            }
            recording.events.push_back((*event).clone());
            true
        });
    }
}

impl WatchRecording {
    pub(crate) fn new(watchers: impl IntoIterator<Item = Arc<OsWatcher>>) -> Self {
        let state = Arc::new(Mutex::new(RecordingState {
            started_at_unix_millis: unix_millis(),
            dropped_events: 0,
            events: VecDeque::new(),
        }));
        let watchers: Vec<_> = watchers.into_iter().collect();
        for watcher in &watchers {
            let mut recordings = watcher.diagnostics.recordings.lock();
            recordings.retain(|recording| recording.strong_count() > 0);
            recordings.push(Arc::downgrade(&state));
        }
        Self { state, watchers }
    }

    pub fn snapshot(&self) -> WatchSnapshot {
        let (started_at_unix_millis, dropped_events, events) = {
            let state = self.state.lock();
            (
                state.started_at_unix_millis,
                state.dropped_events,
                state.events.iter().cloned().collect(),
            )
        };
        let watchers = self
            .watchers
            .iter()
            .map(|watcher| {
                let state = watcher.state.lock();
                let mut roots: Vec<_> = state
                    .paths
                    .0
                    .values()
                    .filter(|path| path.has_os_watcher)
                    .filter_map(|path| {
                        let registration = state.watchers.get(path.watcher_ids.first()?)?;
                        Some(WatchRoot {
                            path: registration.path.as_path().to_string_lossy().into_owned(),
                            registrations: path.watcher_ids.len(),
                        })
                    })
                    .collect();
                roots.sort_by(|left, right| left.path.cmp(&right.path));
                WatcherSnapshot {
                    backend: watcher.kind,
                    recursive: watcher.recursive,
                    roots,
                    cooldown_remaining_millis: state
                        .cooldown_until
                        .and_then(|until| until.checked_duration_since(Instant::now()))
                        .map(|remaining| remaining.as_millis()),
                }
            })
            .collect();
        WatchSnapshot {
            started_at_unix_millis,
            captured_at_unix_millis: unix_millis(),
            capacity: EVENT_CAPACITY,
            dropped_events,
            watchers,
            events,
        }
    }
}

fn unix_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs_watcher::WatchBackend;
    use gpui::TestAppContext;
    use notify::{Event, EventKind, event::Flag};
    use std::path::Path;

    struct Backend;

    impl WatchBackend for Backend {
        fn watch(&mut self, path: &Path, _: notify::RecursiveMode) -> notify::Result<()> {
            if path.ends_with("fail") {
                Err(notify::Error::generic("watch failed").add_path(path.to_owned()))
            } else {
                Ok(())
            }
        }

        fn unwatch(&mut self, path: &Path) -> notify::Result<()> {
            Err(notify::Error::generic("unwatch failed").add_path(path.to_owned()))
        }
    }

    fn watcher(kind: OsWatcherKind, cx: &TestAppContext) -> Arc<OsWatcher> {
        OsWatcher::with_backend(
            kind,
            cx.background_executor.clone(),
            Some(Box::new(Backend)),
        )
    }

    #[gpui::test]
    fn recording_is_opt_in_and_bounded(cx: &TestAppContext) {
        let watcher = watcher(OsWatcherKind::Native, cx);
        watcher.diagnostics.record(|| panic!("not recording"));
        let recording = WatchRecording::new([watcher.clone()]);
        let state = Arc::downgrade(&recording.state);
        for index in 0..EVENT_CAPACITY + 3 {
            watcher.diagnostics.record(|| {
                WatchDiagnosticEvent::new(
                    OsWatcherKind::Native,
                    "event",
                    &[],
                    index.to_string(),
                    false,
                )
            });
        }
        let snapshot = recording.snapshot();
        assert_eq!(snapshot.events.len(), EVENT_CAPACITY);
        assert_eq!(snapshot.dropped_events, 3);
        assert_eq!(snapshot.events.first().unwrap().detail, "3");
        assert_eq!(
            snapshot.events.last().unwrap().detail,
            (EVENT_CAPACITY + 2).to_string()
        );

        drop(recording);
        assert!(state.upgrade().is_none());
        watcher
            .diagnostics
            .record(|| panic!("recording was dropped"));
        let recording = WatchRecording::new([watcher]);
        assert!(recording.snapshot().events.is_empty());
    }

    #[gpui::test]
    fn recording_keeps_raw_events_errors_and_rescans(cx: &TestAppContext) {
        let native = watcher(OsWatcherKind::Native, cx);
        let poll = watcher(OsWatcherKind::Poll, cx);
        let recording = WatchRecording::new([native.clone(), poll.clone()]);
        let native_sink = native.event_sink();
        let poll_sink = poll.event_sink();

        // Access events and duplicate overflows are filtered on their way to
        // worktrees, but diagnostics must preserve what the backend reported.
        native_sink(Ok(Event::new(EventKind::Access(
            notify::event::AccessKind::Any,
        ))));
        for _ in 0..2 {
            native_sink(Ok(Event::new(EventKind::Other).set_flag(Flag::Rescan)));
        }
        poll_sink(Err(
            notify::Error::generic("read failed").add_path(util::path!("/root/file").into())
        ));

        let snapshot = recording.snapshot();
        assert_eq!(snapshot.events.len(), 4);
        assert!(snapshot.events[0].detail.contains("Access"));
        assert_eq!(
            snapshot.events[0].event_kind.as_deref(),
            Some("Access(Any)")
        );
        assert!(snapshot.events[1].rescan);
        assert!(snapshot.events[2].rescan);
        assert_eq!(snapshot.events[3].backend, OsWatcherKind::Poll);
        assert_eq!(snapshot.events[3].operation, "error");
        assert_eq!(snapshot.events[3].event_kind, None);
        assert!(snapshot.events[3].detail.contains("read failed"));
        assert_eq!(snapshot.events[3].paths, [util::path!("/root/file")]);
        let json = serde_json::to_value(snapshot).unwrap();
        assert_eq!(json["events"][2]["rescan"], true);
        assert_eq!(json["events"][3]["paths"][0], util::path!("/root/file"));
    }

    #[gpui::test]
    fn recording_snapshots_existing_roots_and_registration_errors(cx: &TestAppContext) {
        let watcher = watcher(OsWatcherKind::Native, cx);
        let root = util::path!("/root");
        let registration = watcher
            .add(Path::new(root).into(), false, |_| {})
            .unwrap()
            .unwrap();
        let duplicate = watcher
            .add(Path::new(root).into(), false, |_| {})
            .unwrap()
            .unwrap();
        let recording = WatchRecording::new([watcher.clone()]);
        let snapshot = recording.snapshot();
        assert!(snapshot.events.is_empty());
        assert_eq!(snapshot.watchers[0].roots.len(), 1);
        assert_eq!(snapshot.watchers[0].roots[0].path, root);
        assert_eq!(snapshot.watchers[0].roots[0].registrations, 2);

        assert!(
            watcher
                .add(Path::new(util::path!("/fail")).into(), false, |_| {})
                .is_err()
        );
        watcher.remove(duplicate);
        assert_eq!(recording.snapshot().watchers[0].roots[0].registrations, 1);
        watcher.remove(registration);
        let snapshot = recording.snapshot();
        assert!(snapshot.watchers[0].roots.is_empty());
        assert_eq!(snapshot.events[0].operation, "watch_error");
        assert!(snapshot.events[0].detail.contains("watch failed"));
        assert_eq!(snapshot.events[1].operation, "unwatch_error");
        assert!(snapshot.events[1].detail.contains("unwatch failed"));
    }
}
