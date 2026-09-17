use chrono::{DateTime, Local, Utc};
use editor::{Editor, MultiBufferOffset};
use fs::fs_watcher::{WatchDiagnosticEvent, WatchRecording, WatchSnapshot};
use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, Render, Task, TitlebarOptions,
    Window, WindowBounds, WindowOptions, actions, px, size,
};
use language::{LineEnding, language_settings::SoftWrap};
use release_channel::AppVersion;
use serde::Serialize;
use std::{
    collections::{HashSet, VecDeque},
    sync::Arc,
    time::Duration,
};
use ui::{Tab, TabBar, TabPosition, prelude::*};
use util::ResultExt;
use workspace::AppState;

actions!(
    dev,
    [
        /// Open an app-wide recording of raw local filesystem watcher events.
        DebugFilesystemWatching
    ]
);

pub fn init(app_state: Arc<AppState>, cx: &mut App) {
    cx.on_action(move |_: &DebugFilesystemWatching, cx| {
        if let Some(existing) = cx
            .windows()
            .into_iter()
            .find_map(|window| window.downcast::<WatcherDebug>())
        {
            existing
                .update(cx, |view, window, cx| {
                    window.activate_window();
                    view.focus_handle(cx).focus(window, cx);
                })
                .log_err();
            return;
        }

        let app_state = app_state.clone();
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions::default()),
                window_bounds: Some(WindowBounds::centered(size(px(1000.), px(700.)), cx)),
                ..Default::default()
            },
            |window, cx| {
                window.set_window_title("Debug Filesystem Watching");
                let view = cx.new(|cx| WatcherDebug::new(app_state, window, cx));
                window.activate_window();
                view.focus_handle(cx).focus(window, cx);
                view
            },
        )
        .log_err();
    });
}

#[derive(Serialize)]
struct WorktreeExclusions {
    root: String,
    file_scan_exclusions: Vec<String>,
}

#[derive(Serialize)]
struct Export {
    zed_version: String,
    os_name: String,
    os_version: String,
    watcher: WatchSnapshot,
    worktree_scan_exclusions: Vec<WorktreeExclusions>,
    exclusion_scope: &'static str,
}

const EXCLUSION_SCOPE: &str = "Patterns Zed skips when scanning your open local projects. Excluded files may still produce watcher events.";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum WatcherTab {
    RawEvents,
    WatchRoots,
    ScanExclusions,
}

impl WatcherTab {
    const ALL: [Self; 3] = [Self::RawEvents, Self::WatchRoots, Self::ScanExclusions];

    fn label(self) -> &'static str {
        match self {
            Self::RawEvents => "Raw Events",
            Self::WatchRoots => "Watch Roots",
            Self::ScanExclusions => "Scan Exclusions",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::RawEvents => {
                "Raw watcher notifications, oldest first. Times are shown in your local time zone."
            }
            Self::WatchRoots => "Live native and polling watch roots across the app.",
            Self::ScanExclusions => EXCLUSION_SCOPE,
        }
    }

    fn empty_message(self) -> &'static str {
        match self {
            Self::RawEvents => "Waiting for filesystem watcher events…",
            Self::WatchRoots => "No watch roots.",
            Self::ScanExclusions => "No local projects are open.",
        }
    }
}

fn format_event(event: &WatchDiagnosticEvent) -> SharedString {
    let time = i64::try_from(event.timestamp_unix_millis)
        .ok()
        .and_then(DateTime::<Utc>::from_timestamp_millis)
        .map(|time| {
            time.with_timezone(&Local)
                .format("%H:%M:%S%.3f")
                .to_string()
        })
        .unwrap_or_else(|| "--:--:--.---".into());
    let name = if event.rescan {
        "Rescan"
    } else {
        event.event_kind.as_deref().unwrap_or(&event.operation)
    };
    let name = name
        .split_once('(')
        .map_or(name, |(name, _)| name)
        .to_lowercase();
    let mut row = format!("{time}  {name}");
    for path in &event.paths {
        row.push_str("  ");
        row.push_str(path);
    }
    row.into()
}

fn create_editor(
    tab: WatcherTab,
    text: String,
    window: &mut Window,
    cx: &mut App,
) -> Entity<Editor> {
    let buffer = cx.new(|cx| language::Buffer::local(text, cx));
    cx.new(|cx| {
        let mut editor = Editor::for_buffer(buffer, None, window, cx);
        editor.set_read_only(true);
        editor.set_input_enabled(false);
        editor.set_use_modal_editing(false);
        editor.set_show_gutter(false, cx);
        editor.hide_minimap_by_default(window, cx);
        editor.set_show_edit_predictions(Some(false), window, cx);
        editor.set_soft_wrap_mode(SoftWrap::None, cx);
        editor.set_placeholder_text(tab.empty_message(), window, cx);
        editor
    })
}

fn update_editor_text(editor: &Entity<Editor>, mut text: String, cx: &mut App) -> usize {
    LineEnding::normalize(&mut text);
    editor.update(cx, |editor, cx| {
        let previous = editor.text(cx);
        if previous == text {
            return 0;
        }
        let edits = language::text_diff(&previous, &text);
        let edited_bytes = edits
            .iter()
            .map(|(range, text)| range.len() + text.len())
            .sum();
        let edits = edits.into_iter().map(|(range, text)| {
            (
                MultiBufferOffset(range.start)..MultiBufferOffset(range.end),
                text,
            )
        });
        editor
            .buffer()
            .update(cx, |buffer, cx| buffer.edit(edits, None, cx));
        edited_bytes
    })
}

fn worktree_exclusions(app_state: &AppState, cx: &App) -> Vec<WorktreeExclusions> {
    let mut seen = HashSet::new();
    let mut exclusions = Vec::new();
    for workspace in app_state.workspace_store.read(cx).workspaces() {
        let Some(workspace) = workspace.upgrade() else {
            continue;
        };
        for worktree in workspace.read(cx).worktrees(cx) {
            if !seen.insert(worktree.entity_id()) {
                continue;
            }
            let Some(local) = worktree.read(cx).as_local() else {
                continue;
            };
            exclusions.push(WorktreeExclusions {
                root: local.abs_path().to_string_lossy().into_owned(),
                file_scan_exclusions: local
                    .settings()
                    .file_scan_exclusions
                    .sources()
                    .map(str::to_owned)
                    .collect(),
            });
        }
    }
    exclusions.sort_by(|left, right| left.root.cmp(&right.root));
    exclusions
}

struct WatcherDebug {
    app_state: Arc<AppState>,
    recording: Option<WatchRecording>,
    snapshot: Option<WatchSnapshot>,
    active_tab: WatcherTab,
    editors: [Entity<Editor>; 3],
    editor_edit_bytes: [usize; 3],
    event_lengths: VecDeque<usize>,
    save_error: Option<SharedString>,
    saving: bool,
    _poll_task: Task<()>,
}

impl WatcherDebug {
    fn new(app_state: Arc<AppState>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let editors = WatcherTab::ALL.map(|tab| create_editor(tab, String::new(), window, cx));
        let recording = app_state.fs.record_watcher_diagnostics();
        let poll_task = cx.spawn_in(window, async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(250))
                    .await;
                if this
                    .update_in(cx, |this, window, cx| {
                        this.refresh(cx);
                        this.compact_editors(window, cx);
                        cx.notify();
                    })
                    .is_err()
                {
                    break;
                }
            }
        });
        let mut view = Self {
            app_state,
            recording,
            snapshot: None,
            active_tab: WatcherTab::RawEvents,
            editors,
            editor_edit_bytes: [0; 3],
            event_lengths: VecDeque::new(),
            save_error: None,
            saving: false,
            _poll_task: poll_task,
        };
        view.refresh(cx);
        view
    }

    fn refresh(&mut self, cx: &mut Context<Self>) {
        let snapshot = self.recording.as_ref().map(WatchRecording::snapshot);
        let mut watch_roots = String::new();
        if let Some(snapshot) = &snapshot {
            self.refresh_events(snapshot, cx);
            for watcher in &snapshot.watchers {
                watch_roots.push_str(&format!(
                    "{:?}: recursive={}, cooldown remaining={:?} ms\n",
                    watcher.backend, watcher.recursive, watcher.cooldown_remaining_millis
                ));
                for root in &watcher.roots {
                    watch_roots.push_str(&format!(
                        "  {} ({} registrations)\n",
                        root.path, root.registrations
                    ));
                }
            }
        }
        self.snapshot = snapshot;
        self.editor_edit_bytes[WatcherTab::WatchRoots as usize] +=
            update_editor_text(self.editor(WatcherTab::WatchRoots), watch_roots, cx);
        let mut scan_exclusions = String::new();
        for worktree in worktree_exclusions(&self.app_state, cx) {
            scan_exclusions.push_str(&worktree.root);
            scan_exclusions.push('\n');
            if worktree.file_scan_exclusions.is_empty() {
                scan_exclusions.push_str("  (none)\n");
            }
            for pattern in worktree.file_scan_exclusions {
                scan_exclusions.push_str(&format!("  {pattern}\n"));
            }
        }
        self.editor_edit_bytes[WatcherTab::ScanExclusions as usize] +=
            update_editor_text(self.editor(WatcherTab::ScanExclusions), scan_exclusions, cx);
    }

    fn refresh_events(&mut self, snapshot: &WatchSnapshot, cx: &mut Context<Self>) {
        let previous_dropped = self
            .snapshot
            .as_ref()
            .map_or(0, |snapshot| snapshot.dropped_events);
        let removed = usize::try_from(snapshot.dropped_events.saturating_sub(previous_dropped))
            .unwrap_or(usize::MAX)
            .min(self.event_lengths.len());
        let removed_bytes: usize = self.event_lengths.drain(..removed).sum();
        let retained = self.event_lengths.len();
        let mut appended = String::new();
        for event in snapshot.events.iter().skip(retained) {
            let mut line = format!("{}\n", format_event(event));
            LineEnding::normalize(&mut line);
            self.event_lengths.push_back(line.len());
            appended.push_str(&line);
        }
        if removed_bytes == 0 && appended.is_empty() {
            return;
        }
        self.editor_edit_bytes[WatcherTab::RawEvents as usize] += removed_bytes + appended.len();
        self.editor(WatcherTab::RawEvents).update(cx, |editor, cx| {
            editor.buffer().update(cx, |buffer, cx| {
                let length = buffer.len(cx);
                // Keep anchors in retained events intact when the recording rolls over.
                buffer.edit(
                    [
                        (
                            MultiBufferOffset(0)..MultiBufferOffset(removed_bytes),
                            String::new(),
                        ),
                        (length..length, appended),
                    ],
                    None,
                    cx,
                );
            });
        });
    }

    fn compact_editors(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for tab in WatcherTab::ALL {
            let index = tab as usize;
            let length = self.editors[index].read(cx).buffer().read(cx).len(cx).0;
            if self.editor_edit_bytes[index] < length.saturating_mul(2).max(64 * 1024) {
                continue;
            }

            // Deletions leave CRDT tombstones and undo history behind. Replacing
            // the editor releases that storage without invalidating live selections.
            let (text, selections, scroll_position, focused) =
                self.editors[index].update(cx, |editor, cx| {
                    let snapshot = editor.snapshot(window, cx);
                    (
                        editor.text(cx),
                        editor.selections.all::<MultiBufferOffset>(&snapshot),
                        editor.scroll_position(cx),
                        editor.focus_handle(cx).is_focused(window),
                    )
                });
            let editor = create_editor(tab, text, window, cx);
            editor.update(cx, |editor, cx| {
                editor.change_selections(
                    editor::SelectionEffects::no_scroll(),
                    window,
                    cx,
                    |selection| {
                        selection.select(selections);
                    },
                );
                editor.set_scroll_position(scroll_position, window, cx);
                if focused {
                    editor.focus_handle(cx).focus(window, cx);
                }
            });
            self.editors[index] = editor;
            self.editor_edit_bytes[index] = 0;
            cx.notify();
        }
    }

    fn editor(&self, tab: WatcherTab) -> &Entity<Editor> {
        &self.editors[tab as usize]
    }

    fn select_tab(&mut self, tab: WatcherTab, window: &mut Window, cx: &mut Context<Self>) {
        self.active_tab = tab;
        self.focus_handle(cx).focus(window, cx);
        cx.notify();
    }

    fn save(&mut self, cx: &mut Context<Self>) {
        let Some(recording) = &self.recording else {
            return;
        };
        let watcher = recording.snapshot();
        let worktree_scan_exclusions = worktree_exclusions(&self.app_state, cx);
        let zed_version = AppVersion::global(cx).to_string();
        let os_name = client::telemetry::os_name();
        let directory = std::env::home_dir().unwrap_or_default();
        let path = cx.prompt_for_new_path(&directory, Some("filesystem-watcher.json"));
        let fs = self.app_state.fs.clone();
        self.saving = true;
        self.save_error = None;
        cx.notify();
        cx.spawn(async move |this, cx| {
            let result = async {
                let Some(path) = path.await?? else {
                    return anyhow::Ok(());
                };
                cx.background_spawn(async move {
                    let export = Export {
                        zed_version,
                        os_name,
                        os_version: client::telemetry::os_version(),
                        watcher,
                        worktree_scan_exclusions,
                        exclusion_scope: EXCLUSION_SCOPE,
                    };
                    let json = serde_json::to_vec_pretty(&export)?;
                    fs.write(&path, &json).await
                })
                .await
            }
            .await;
            this.update(cx, |this, cx| {
                this.saving = false;
                this.save_error = match result {
                    Ok(()) => None,
                    Err(error) => Some(format!("Save failed: {error:#}").into()),
                };
                cx.notify();
            })
            .log_err();
        })
        .detach();
    }
}

impl Focusable for WatcherDebug {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor(self.active_tab).focus_handle(cx)
    }
}

impl Render for WatcherDebug {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_hidden()
            .bg(cx.theme().colors().editor_background)
            .text_color(cx.theme().colors().text)
            .child(
                TabBar::new("watcher-tabs").children(WatcherTab::ALL.map(|tab| {
                    Tab::new(tab.label())
                        .position(match tab {
                            WatcherTab::RawEvents => TabPosition::First,
                            WatcherTab::WatchRoots => {
                                TabPosition::Middle(tab.cmp(&self.active_tab))
                            }
                            WatcherTab::ScanExclusions => TabPosition::Last,
                        })
                        .toggle_state(self.active_tab == tab)
                        .child(Label::new(tab.label()))
                        .on_click(
                            cx.listener(move |this, _, window, cx| {
                                this.select_tab(tab, window, cx)
                            }),
                        )
                })),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_h_0()
                    .overflow_hidden()
                    .p_3()
                    .gap_2()
                    .child(
                        Label::new(self.active_tab.description())
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .when_some(self.save_error.clone(), |content, error| {
                        content.child(Label::new(error).size(LabelSize::Small).color(Color::Error))
                    })
                    .child(
                        div()
                            .flex_1()
                            .min_h_0()
                            .w_full()
                            .child(self.editor(self.active_tab).clone()),
                    ),
            )
            .child(
                h_flex()
                    .debug_selector(|| "watcher-footer".into())
                    .flex_none()
                    .justify_end()
                    .bg(cx.theme().colors().elevated_surface_background)
                    .p(DynamicSpacing::Base04.rems(cx))
                    .child(
                        Button::new("save-watcher-json", "Export as JSON")
                            .disabled(self.saving || self.recording.is_none())
                            .on_click(cx.listener(|this, _, _, cx| this.save(cx))),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Timelike};
    use fs::fs_watcher::OsWatcherKind;

    #[test]
    fn formats_local_times_event_names_and_plain_paths() {
        let time = Local
            .with_ymd_and_hms(2026, 9, 10, 16, 25, 49)
            .single()
            .unwrap()
            .with_nanosecond(123_000_000)
            .unwrap();
        let event = WatchDiagnosticEvent {
            timestamp_unix_millis: time.timestamp_millis().try_into().unwrap(),
            backend: OsWatcherKind::Native,
            operation: "event".into(),
            event_kind: Some("Modify(Name(Both))".into()),
            paths: vec!["/project/old name.rs".into(), "/project/new name.rs".into()],
            detail: "Extra raw backend attributes".into(),
            rescan: false,
        };
        assert_eq!(
            format_event(&event).as_ref(),
            "16:25:49.123  modify  /project/old name.rs  /project/new name.rs"
        );
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["detail"], "Extra raw backend attributes");
        assert_eq!(json["timestamp_unix_millis"], time.timestamp_millis());
        assert_eq!(json["paths"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn formats_pathless_rescans_and_errors() {
        let mut event = WatchDiagnosticEvent {
            timestamp_unix_millis: u128::MAX,
            backend: OsWatcherKind::Poll,
            operation: "event".into(),
            event_kind: Some("Other".into()),
            paths: vec![],
            detail: "Raw rescan attributes".into(),
            rescan: true,
        };
        assert_eq!(format_event(&event).as_ref(), "--:--:--.---  rescan");
        event.rescan = false;
        event.event_kind = None;
        event.operation = "watch_error".into();
        event.detail = "Permission denied".into();
        assert_eq!(format_event(&event).as_ref(), "--:--:--.---  watch_error");
    }

    #[gpui::test]
    fn editors_allow_copying_and_preserve_selections_on_updates(cx: &mut gpui::TestAppContext) {
        let app_state = cx.update(AppState::test);
        let window = cx.add_window(|window, cx| WatcherDebug::new(app_state, window, cx));
        let mut visual = gpui::VisualTestContext::from_window(window.into(), cx);
        for tab in WatcherTab::ALL {
            window
                .update(cx, |view, window, cx| {
                    let editor = view.editor(tab).clone();
                    update_editor_text(&editor, "prefix\nselected path\nsuffix\n".into(), cx);
                    editor.update(cx, |editor, cx| {
                        editor.change_selections(
                            editor::SelectionEffects::no_scroll(),
                            window,
                            cx,
                            |selections| {
                                selections
                                    .select_ranges([MultiBufferOffset(7)..MultiBufferOffset(20)]);
                            },
                        );
                    });
                    view.select_tab(tab, window, cx);
                })
                .unwrap();
            visual.run_until_parked();
            visual.dispatch_action(editor::actions::Copy);
            visual.run_until_parked();
            assert_eq!(
                cx.read_from_clipboard()
                    .and_then(|item| item.text())
                    .as_deref(),
                Some("selected path")
            );
            visual.simulate_input("should not edit");
            window
                .update(cx, |view, _, cx| {
                    let editor = view.editor(tab);
                    assert_eq!(editor.read(cx).text(cx), "prefix\nselected path\nsuffix\n");
                    update_editor_text(
                        editor,
                        "added\nprefix\nselected path\nsuffix\ntail\n".into(),
                        cx,
                    );
                })
                .unwrap();
            visual.run_until_parked();
            visual.dispatch_action(editor::actions::Copy);
            visual.run_until_parked();
            assert_eq!(
                cx.read_from_clipboard()
                    .and_then(|item| item.text())
                    .as_deref(),
                Some("selected path")
            );
        }
        window
            .update(cx, |_, window, _| window.remove_window())
            .unwrap();
    }

    #[gpui::test]
    fn event_rollover_preserves_retained_selection(cx: &mut gpui::TestAppContext) {
        let app_state = cx.update(AppState::test);
        let window = cx.add_window(|window, cx| WatcherDebug::new(app_state, window, cx));
        let snapshot = |dropped_events, paths: &[&str]| WatchSnapshot {
            started_at_unix_millis: 0,
            captured_at_unix_millis: 0,
            capacity: 2,
            dropped_events,
            watchers: vec![],
            events: paths
                .iter()
                .map(|path| {
                    Arc::new(WatchDiagnosticEvent {
                        timestamp_unix_millis: 0,
                        backend: OsWatcherKind::Native,
                        operation: "event".into(),
                        event_kind: Some("Create(File)".into()),
                        paths: vec![(*path).into()],
                        detail: String::new(),
                        rescan: false,
                    })
                })
                .collect(),
        };
        window
            .update(cx, |view, window, cx| {
                let first = snapshot(0, &["/old\r\nfile", "/kept"]);
                view.refresh_events(&first, cx);
                view.snapshot = Some(first);
                view.editor(WatcherTab::RawEvents).update(cx, |editor, cx| {
                    let start = editor.text(cx).find("/kept").unwrap();
                    editor.change_selections(
                        editor::SelectionEffects::no_scroll(),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges([
                                MultiBufferOffset(start)..MultiBufferOffset(start + 5)
                            ]);
                        },
                    );
                });
                let next = snapshot(1, &["/kept", "/new"]);
                view.refresh_events(&next, cx);
                view.snapshot = Some(next);
                let text = view.editor(WatcherTab::RawEvents).read(cx).text(cx);
                assert!(!text.contains("/old"));
                assert!(text.contains("/new"));
                view.select_tab(WatcherTab::RawEvents, window, cx);
            })
            .unwrap();
        let mut visual = gpui::VisualTestContext::from_window(window.into(), cx);
        visual.run_until_parked();
        visual.dispatch_action(editor::actions::Copy);
        visual.run_until_parked();
        assert_eq!(
            cx.read_from_clipboard()
                .and_then(|item| item.text())
                .as_deref(),
            Some("/kept")
        );
        let old_editor = window
            .update(cx, |view, window, cx| {
                let old_editor = view.editor(WatcherTab::RawEvents).downgrade();
                view.editor_edit_bytes[WatcherTab::RawEvents as usize] = 64 * 1024;
                view.compact_editors(window, cx);
                old_editor
            })
            .unwrap();
        visual.run_until_parked();
        assert!(old_editor.upgrade().is_none());
        visual.dispatch_action(editor::actions::Copy);
        visual.run_until_parked();
        assert_eq!(
            cx.read_from_clipboard()
                .and_then(|item| item.text())
                .as_deref(),
            Some("/kept")
        );
        window
            .update(cx, |view, window, cx| {
                let next = snapshot(10, &["/entirely", "/different"]);
                view.refresh_events(&next, cx);
                view.snapshot = Some(next);
                let text = view.editor(WatcherTab::RawEvents).read(cx).text(cx);
                assert!(!text.contains("/kept"));
                assert!(!text.contains("/new"));
                assert!(text.contains("/entirely"));
                assert_eq!(view.event_lengths.len(), 2);
                let mut compactions = 0;
                for cycle in 0..40 {
                    let path = format!("/{cycle}/{}", "x".repeat(4096));
                    let next = snapshot(12 + cycle * 2, &[&path, &path]);
                    view.refresh_events(&next, cx);
                    view.snapshot = Some(next);
                    let previous_editor = view.editor(WatcherTab::RawEvents).entity_id();
                    view.compact_editors(window, cx);
                    let editor = view.editor(WatcherTab::RawEvents);
                    if previous_editor != editor.entity_id() {
                        compactions += 1;
                    }
                    let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);
                    let buffer = buffer.as_singleton().unwrap();
                    assert!(buffer.deleted_text().len() < 64 * 1024);
                    assert_eq!(buffer.text().lines().count(), 2);
                }
                assert!(compactions > 1);
                window.remove_window();
            })
            .unwrap();
    }

    #[gpui::test]
    async fn tabs_separate_rows_and_keep_footer_visible(cx: &mut gpui::TestAppContext) {
        let app_state = cx.update(AppState::test);
        let fs = app_state.fs.clone();
        let root = std::path::Path::new(util::path!("/watched"));
        fs.create_dir(root).await.unwrap();
        let (_events, _watcher) = fs.watch(root, Duration::ZERO).await;
        cx.update(|cx| {
            init(app_state, cx);
            cx.dispatch_action(&DebugFilesystemWatching);
        });
        let window = cx.update(|cx| {
            cx.windows()
                .into_iter()
                .find_map(|window| window.downcast::<WatcherDebug>())
                .unwrap()
        });
        fs.write(&root.join("event.txt"), b"first").await.unwrap();
        cx.executor().advance_clock(Duration::from_millis(250));
        cx.run_until_parked();
        window
            .update(cx, |view, _, cx| {
                assert_eq!(view.active_tab, WatcherTab::RawEvents);
                assert!(
                    view.editor(WatcherTab::RawEvents)
                        .read(cx)
                        .text(cx)
                        .contains("event.txt")
                );
            })
            .unwrap();

        let mut visual = gpui::VisualTestContext::from_window(window.into(), cx);
        assert_eq!(
            visual.window_title().as_deref(),
            Some("Debug Filesystem Watching")
        );
        visual.run_until_parked();
        let footer = visual.debug_bounds("watcher-footer").unwrap();
        let status_bar_height = visual.update(|window, cx| {
            ButtonSize::Default.rems().to_pixels(window.rem_size())
                + DynamicSpacing::Base04.rems(cx).to_pixels(window.rem_size()) * 2.
        });
        assert_eq!(footer.size.height, status_bar_height);
        for (selector, tab) in [
            ("TAB-Watch Roots", WatcherTab::WatchRoots),
            ("TAB-Scan Exclusions", WatcherTab::ScanExclusions),
            ("TAB-Raw Events", WatcherTab::RawEvents),
        ] {
            let bounds = visual.debug_bounds(selector).unwrap();
            visual.simulate_click(bounds.center(), gpui::Modifiers::default());
            visual.run_until_parked();
            assert_eq!(visual.debug_bounds("watcher-footer"), Some(footer));
            window
                .update(cx, |view, _, cx| {
                    assert_eq!(view.active_tab, tab);
                    let editor = view.editor(tab).read(cx);
                    assert!(editor.read_only(cx));
                    let text = editor.text(cx);
                    match tab {
                        WatcherTab::RawEvents => {
                            assert!(text.contains("event.txt"))
                        }
                        WatcherTab::WatchRoots => {
                            assert!(text.contains("Native"));
                            assert!(text.contains(root.to_str().unwrap()));
                        }
                        WatcherTab::ScanExclusions => {
                            assert!(text.is_empty());
                        }
                    }
                })
                .unwrap();
        }
        window
            .update(cx, |_, window, _| window.remove_window())
            .unwrap();
    }

    #[gpui::test]
    fn singleton_releases_recording_view_on_close(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let app_state = AppState::test(cx);
            init(app_state, cx);
            cx.dispatch_action(&DebugFilesystemWatching);
        });
        let window = cx.update(|cx| {
            cx.windows()
                .into_iter()
                .find_map(|window| window.downcast::<WatcherDebug>())
                .expect("diagnostics window should open")
        });
        let view = window
            .update(cx, |view, _, cx| {
                assert!(view.recording.is_some());
                cx.weak_entity()
            })
            .expect("diagnostics window should exist");
        cx.update(|cx| cx.dispatch_action(&DebugFilesystemWatching));
        cx.update(|cx| {
            assert_eq!(
                cx.windows()
                    .into_iter()
                    .filter(|window| window.downcast::<WatcherDebug>().is_some())
                    .count(),
                1
            );
        });
        window
            .update(cx, |_, window, _| window.remove_window())
            .expect("diagnostics window should close");
        cx.run_until_parked();
        assert!(view.upgrade().is_none());
    }

    #[gpui::test]
    async fn saves_json_handles_cancellation_and_reports_errors(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| release_channel::init("1.2.3+dev.test".parse().unwrap(), cx));
        let app_state = cx.update(AppState::test);
        let fs = app_state.fs.clone();
        let root = std::path::Path::new(util::path!("/exports"));
        fs.create_dir(root).await.unwrap();
        let (_events, _watcher) = fs.watch(root, Duration::ZERO).await;
        cx.update(|cx| {
            init(app_state, cx);
            cx.dispatch_action(&DebugFilesystemWatching);
        });
        let window = cx.update(|cx| {
            cx.windows()
                .into_iter()
                .find_map(|window| window.downcast::<WatcherDebug>())
                .unwrap()
        });

        window.update(cx, |view, _, cx| view.save(cx)).unwrap();
        assert!(cx.did_prompt_for_new_path());
        cx.simulate_new_path_selection(|_| None);
        cx.run_until_parked();
        window
            .update(cx, |view, _, _| {
                assert!(!view.saving);
                assert!(view.save_error.is_none());
            })
            .unwrap();

        let path = root.join("watcher.json");
        window.update(cx, |view, _, cx| view.save(cx)).unwrap();
        cx.simulate_new_path_selection(|_| Some(path.clone()));
        cx.run_until_parked();
        let json: serde_json::Value = serde_json::from_str(&fs.load(&path).await.unwrap()).unwrap();
        assert_eq!(json["zed_version"], "1.2.3+dev.test");
        assert_eq!(json["os_name"], client::telemetry::os_name());
        let os_version = cx
            .background_executor
            .spawn(async { client::telemetry::os_version() })
            .await;
        assert_eq!(json["os_version"], os_version);
        assert!(!os_version.is_empty());
        assert!(json.get("schema_version").is_none());
        assert_eq!(json["watcher"]["capacity"], 10_000);
        assert_eq!(
            json["watcher"]["watchers"][0]["roots"][0]["path"],
            root.to_string_lossy().as_ref()
        );
        assert_eq!(json["exclusion_scope"], EXCLUSION_SCOPE);
        window
            .update(cx, |view, _, _| {
                assert!(!view.saving);
                assert!(view.save_error.is_none());
            })
            .unwrap();

        window.update(cx, |view, _, cx| view.save(cx)).unwrap();
        cx.simulate_new_path_selection(|_| Some(root.to_owned()));
        cx.run_until_parked();
        window
            .update(cx, |view, window, _| {
                assert!(!view.saving);
                assert!(
                    view.save_error
                        .as_ref()
                        .unwrap()
                        .starts_with("Save failed:")
                );
                window.remove_window();
            })
            .unwrap();
    }
}
