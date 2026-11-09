//! Zed doing ordinary things, measured frame by frame.
//!
//! Each benchmark opens a real workspace (see `zed_benchmarks::init_app` and
//! `open_zed_window`) outside the timed interval, then measures sessions of input
//! delivered through the window's normal keyboard, scroll and action dispatch, with
//! every resulting frame drawn by the headless renderer. Assertions on each session's
//! outcome run outside the timed interval.

use std::{
    cell::Cell,
    collections::BTreeSet,
    rc::Rc,
    time::{Duration, Instant},
};

use editor::{Editor, SelectionEffects};
use gpui::{
    Action as _, AnyWindowHandle, App, BenchAppContext, Entity, Focusable as _, ListOffset, Point,
    Window, point, px,
};
use language::Point as BufferPoint;
use project::{LanguageServerStatus, ProgressToken};
use theme::ActiveTheme as _;
use workspace::SaveIntent;
use zed_benchmarks::{
    AGENT_THREAD_ENTRIES_PER_TURN, AGENT_THREAD_TURNS, MARKDOWN_FILE, PYTHON_FILE, RUST_FILE, Step,
    TYPESCRIPT_FILE, ZedWindow, agent_thread_json, bench_sessions, init_app,
    init_app_with_settings, keystroke, open_zed_window, pace_frame, pinned_contents, scroll_wheel,
    wait_for_highlighting, wait_for_task, wait_until,
};

const FPS: u64 = 120;

fn focus_editor(zed: &ZedWindow, cx: &mut BenchAppContext, editor: &Entity<Editor>) {
    zed.update_window(cx, |window, cx| {
        window.focus(&editor.focus_handle(cx), cx);
    });
    cx.run_until_idle();
}

fn editor_center(cx: &mut BenchAppContext, editor: &Entity<Editor>) -> Point<gpui::Pixels> {
    editor.read_with(cx, |editor, _| {
        editor
            .last_bounds()
            .expect("editor was not laid out")
            .center()
    })
}

fn open_highlighted(
    cx: &mut BenchAppContext,
    zed: &ZedWindow,
    path: &str,
    language: &str,
) -> Entity<Editor> {
    let editor = zed.open_file(cx, path);
    wait_for_highlighting(cx, &editor, language);
    focus_editor(zed, cx, &editor);
    editor
}

fn keystrokes(window: &mut Window, cx: &mut App, keys: &str) {
    for key in keys.split(' ') {
        keystroke(window, cx, key);
    }
}

fn typed_keystroke(character: char) -> String {
    match character {
        ' ' => "space".to_string(),
        character if character.is_ascii_uppercase() => {
            format!("shift-{}", character.to_ascii_lowercase())
        }
        character => character.to_string(),
    }
}

fn steps_until(turn: u64, steps: u64) -> zed_benchmarks::Step {
    if turn + 1 < steps {
        Step::Continue
    } else {
        Step::Stop
    }
}

struct ProjectPanelSession {
    started: Instant,
    moves: usize,
    pending: Option<(Option<String>, &'static str, u64)>,
    first_sent_at: u64,
    max_depth: usize,
}

/// 1. Opens the project panel and walks into the generated tree with the arrow
/// keys at 120 Hz: right on a directory expands it and, pressed again, enters it;
/// down moves past files. The panel rebuilds its visible entries on a background
/// thread after an expansion, so a key that hasn't moved the selection yet is
/// pressed again every few frames, like a held key.
#[gpui::bench]
fn project_panel_expand(cx: &mut BenchAppContext) {
    const MOVES: usize = 20;
    const REPEAT_FRAMES: u64 = 3;

    let app_state = init_app(cx);
    let zed = open_zed_window(cx, &app_state);
    zed.update_window(cx, |window, cx| keystroke(window, cx, "cmd-shift-e"));
    cx.run_until_idle();
    let panel = zed.workspace.read_with(cx, |workspace, cx| {
        workspace
            .panel::<project_panel::ProjectPanel>(cx)
            .expect("project panel was not added")
    });
    let selection = move |cx: &App| {
        panel.read(cx).selected_entry(cx).map(|(_, entry)| {
            (
                entry.path.as_unix_str().to_string(),
                entry.path.components().count(),
                entry.is_dir(),
            )
        })
    };

    bench_sessions(
        cx,
        zed.any_window(),
        "project_panel_expand",
        |cx| {
            zed.update_window(cx, |window, cx| {
                window.dispatch_action(project_panel::CollapseAllEntries.boxed_clone(), cx);
                keystroke(window, cx, "home");
            });
            cx.run_until_idle();
            ProjectPanelSession {
                started: Instant::now(),
                moves: 0,
                pending: None,
                first_sent_at: 0,
                max_depth: 0,
            }
        },
        |session, turn, window, cx| {
            if turn == 0 {
                session.started = Instant::now();
            }
            pace_frame(session.started, turn, FPS);
            let current = selection(cx);
            if let Some((before, key, sent_at)) = session.pending.take() {
                if current.as_ref().map(|(path, ..)| path) == before.as_ref() {
                    if turn - sent_at >= REPEAT_FRAMES {
                        // A directory that stays put after expanding has no visible
                        // children to enter; move past it instead.
                        let key = if turn - session.first_sent_at > 10 * REPEAT_FRAMES {
                            "down"
                        } else {
                            key
                        };
                        keystroke(window, cx, key);
                        session.pending = Some((before, key, turn));
                    } else {
                        session.pending = Some((before, key, sent_at));
                    }
                    return Step::Continue;
                }
                session.moves += 1;
                if let Some((_, depth, _)) = &current {
                    session.max_depth = session.max_depth.max(*depth);
                }
            }
            if session.moves >= MOVES {
                return Step::Stop;
            }
            let key = match &current {
                Some((_, _, true)) => "right",
                _ => "down",
            };
            keystroke(window, cx, key);
            session.first_sent_at = turn;
            session.pending = Some((current.map(|(path, ..)| path), key, turn));
            Step::Continue
        },
        |session| {
            assert_eq!(session.moves, MOVES, "the selection should move {MOVES} times");
            assert!(
                session.max_depth >= 4,
                "walking the project panel should reach nested entries, max depth {}",
                session.max_depth
            );
        },
    );
}

enum PaneAction {
    Open(&'static str),
    Keys(&'static str),
}

struct PaneSession {
    next_action: usize,
    pending_open: Option<Rc<Cell<bool>>>,
    pane_count: usize,
    item_count: usize,
}

/// 2. Opens several files, splits the pane right and down with the keyboard, and
/// switches tabs and panes. Each session starts from a workspace with no items.
#[gpui::bench]
fn open_split_switch(cx: &mut BenchAppContext) {
    const SCRIPT: &[PaneAction] = &[
        PaneAction::Open(RUST_FILE),
        PaneAction::Open(TYPESCRIPT_FILE),
        PaneAction::Open(PYTHON_FILE),
        PaneAction::Open(MARKDOWN_FILE),
        PaneAction::Keys("cmd-k right"),
        PaneAction::Open("web/src/components/component_0.ts"),
        PaneAction::Open("README.md"),
        PaneAction::Keys("cmd-k down"),
        PaneAction::Open("crates/generated_03/src/module_1.rs"),
        PaneAction::Keys("cmd-}"),
        PaneAction::Keys("cmd-}"),
        PaneAction::Keys("cmd-k cmd-up"),
        PaneAction::Keys("cmd-}"),
        PaneAction::Keys("cmd-k cmd-left"),
        PaneAction::Keys("cmd-}"),
        PaneAction::Keys("cmd-}"),
        PaneAction::Keys("cmd-}"),
        PaneAction::Keys("cmd-{"),
    ];

    let app_state = init_app(cx);
    let zed = open_zed_window(cx, &app_state);
    let workspace = zed.workspace.clone();

    bench_sessions(
        cx,
        zed.any_window(),
        "open_split_switch",
        |cx| {
            zed.update_window(cx, |window, cx| {
                window.dispatch_action(
                    workspace::CloseAllItemsAndPanes {
                        save_intent: Some(SaveIntent::Skip),
                    }
                    .boxed_clone(),
                    cx,
                );
            });
            wait_until(cx, "an empty workspace", |cx| {
                workspace.read_with(cx, |workspace, cx| {
                    workspace.panes().len() == 1 && workspace.items(cx).next().is_none()
                })
            });
            cx.run_until_idle();
            PaneSession {
                next_action: 0,
                pending_open: None,
                pane_count: 0,
                item_count: 0,
            }
        },
        |session, _turn, window, cx| {
            if let Some(pending_open) = &session.pending_open {
                if !pending_open.get() {
                    return Step::Continue;
                }
                session.pending_open = None;
            }
            let Some(action) = SCRIPT.get(session.next_action) else {
                workspace.read_with(cx, |workspace, cx| {
                    session.pane_count = workspace.panes().len();
                    session.item_count = workspace.items(cx).count();
                });
                return Step::Stop;
            };
            session.next_action += 1;
            match action {
                PaneAction::Open(path) => {
                    let opened = Rc::new(Cell::new(false));
                    let abs_path = zed.abs_path(path);
                    let task = workspace.update(cx, |workspace, cx| {
                        workspace.open_abs_path(
                            abs_path,
                            workspace::OpenOptions {
                                visible: Some(workspace::OpenVisible::None),
                                ..Default::default()
                            },
                            window,
                            cx,
                        )
                    });
                    cx.spawn({
                        let opened = opened.clone();
                        async move |_| {
                            task.await.expect("failed to open fixture file");
                            opened.set(true);
                        }
                    })
                    .detach();
                    session.pending_open = Some(opened);
                }
                PaneAction::Keys(keys) => keystrokes(window, cx, keys),
            }
            Step::Continue
        },
        |session| {
            assert_eq!(
                session.pane_count, 3,
                "expected three panes after splitting"
            );
            assert!(
                session.item_count >= 9,
                "expected at least nine open items, found {}",
                session.item_count
            );
        },
    );
}

struct ScrollFixture {
    name: &'static str,
    path: &'static str,
    language: &'static str,
}

impl std::fmt::Display for ScrollFixture {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.name)
    }
}

fn scroll_fixtures() -> [ScrollFixture; 4] {
    [
        ScrollFixture {
            name: "rust-editor-rs",
            path: RUST_FILE,
            language: "Rust",
        },
        ScrollFixture {
            name: "typescript",
            path: TYPESCRIPT_FILE,
            language: "TypeScript",
        },
        ScrollFixture {
            name: "python",
            path: PYTHON_FILE,
            language: "Python",
        },
        ScrollFixture {
            name: "markdown",
            path: MARKDOWN_FILE,
            language: "Markdown",
        },
    ]
}

struct ScrollSession {
    start_row: f64,
    end_row: f64,
    delta: f32,
}

/// 3. Scrolls a fully highlighted file with trackpad-sized wheel events, one frame
/// per event. Sessions start at different points so a run covers the whole file.
#[gpui::bench(
    inputs = scroll_fixtures(),
    input_name = "file",
    group = "editor_scroll",
    sample_size = 10
)]
fn editor_scroll(fixture: &ScrollFixture, cx: &mut BenchAppContext) {
    const STEPS: u64 = 24;
    const DELTA: f32 = 96.;

    let app_state = init_app(cx);
    let zed = open_zed_window(cx, &app_state);
    let editor = open_highlighted(cx, &zed, fixture.path, fixture.language);
    let center = editor_center(cx, &editor);
    let line_count = pinned_contents(fixture.path).lines().count() as f64;
    let minimum_rows = (STEPS as f64 * DELTA as f64 / 40.).min(line_count / 3.);
    let mut session_index = 0u64;

    bench_sessions(
        cx,
        zed.any_window(),
        &format!("editor_scroll/{}", fixture.name),
        |cx| {
            let max_start = (line_count - 160.).max(1.);
            let start_row = ((session_index * 997) as f64) % max_start;
            session_index += 1;
            let delta = if start_row < line_count / 2. {
                -DELTA
            } else {
                DELTA
            };
            zed.update_window(cx, |window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.set_scroll_position(point(0., start_row), window, cx);
                });
            });
            cx.run_until_idle();
            ScrollSession {
                start_row,
                end_row: start_row,
                delta,
            }
        },
        |session, turn, window, cx| {
            scroll_wheel(window, cx, center, session.delta);
            if turn + 1 < STEPS {
                return Step::Continue;
            }
            session.end_row = editor.update(cx, |editor, cx| editor.scroll_position(cx).y);
            Step::Stop
        },
        move |session| {
            let scrolled_rows = (session.end_row - session.start_row).abs();
            assert!(
                scrolled_rows >= minimum_rows,
                "expected each scroll session to move at least {minimum_rows} rows, moved \
                 {scrolled_rows} (from {} to {})",
                session.start_row,
                session.end_row
            );
        },
    );
}

struct TypingSession {
    original_len: usize,
    final_len: usize,
    peak_len: usize,
}

/// 4. Types a line into `editor.rs` one key at a time through the keymap and input
/// handler, then undoes it with cmd-z. Every keystroke reparses the buffer.
#[gpui::bench]
fn editor_type_and_undo(cx: &mut BenchAppContext) {
    const TYPED: &str = "let total = values.iter().map(|value| value.len()).sum::<usize>();";
    const UNDOS: usize = 3;

    let app_state = init_app(cx);
    let zed = open_zed_window(cx, &app_state);
    let editor = open_highlighted(cx, &zed, RUST_FILE, "Rust");
    let buffer = editor.read_with(cx, |editor, cx| {
        editor
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("fixture editors edit one buffer")
    });
    let original_len = buffer.read_with(cx, |buffer, _| buffer.len());
    // Inside a function body in the middle of the file.
    let target_row = pinned_contents(RUST_FILE)
        .lines()
        .enumerate()
        .skip(6000)
        .find(|(_, line)| line.trim_start().starts_with("let ") && line.ends_with(';'))
        .map(|(row, _)| row as u32)
        .expect("editor.rs has a let statement after line 6000");
    zed.update_window(cx, |window, cx| {
        editor.update(cx, |editor, cx| {
            editor.change_selections(SelectionEffects::default(), window, cx, |selections| {
                let end = BufferPoint::new(target_row, u32::MAX);
                selections.select_ranges([end..end]);
            });
        });
    });
    cx.run_until_idle();

    let mut keys: Vec<String> = vec!["enter".to_string()];
    keys.extend(TYPED.chars().map(typed_keystroke));
    keys.extend(std::iter::repeat_n("cmd-z".to_string(), UNDOS));

    bench_sessions(
        cx,
        zed.any_window(),
        "editor_type_and_undo",
        |cx| {
            wait_for_highlighting(cx, &editor, "Rust");
            TypingSession {
                original_len,
                final_len: 0,
                peak_len: 0,
            }
        },
        |session, turn, window, cx| {
            keystroke(window, cx, &keys[turn as usize]);
            let len = buffer.read(cx).len();
            session.peak_len = session.peak_len.max(len);
            if (turn as usize) + 1 < keys.len() {
                return Step::Continue;
            }
            session.final_len = len;
            Step::Stop
        },
        |session| {
            assert!(
                session.peak_len > session.original_len + TYPED.len() - 1,
                "typing did not insert the line ({} -> {})",
                session.original_len,
                session.peak_len
            );
            assert_eq!(
                session.final_len, session.original_len,
                "undo did not restore the buffer"
            );
        },
    );
}

struct SearchSession {
    found_match: bool,
    selected_rows: BTreeSet<u32>,
    selected_text_matches: bool,
}

fn selected_text(editor: &Entity<Editor>, cx: &mut App) -> (u32, String) {
    editor.update(cx, |editor, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let selection = editor
            .selections
            .newest::<BufferPoint>(&editor.display_snapshot(cx));
        let text = snapshot
            .text_for_range(selection.start..selection.end)
            .collect::<String>();
        (selection.start.row, text)
    })
}

/// 5. Deploys buffer search with cmd-f, types a query, steps through matches with
/// enter and dismisses the bar with escape.
#[gpui::bench]
fn buffer_search(cx: &mut BenchAppContext) {
    const QUERY: &str = "cx.notify()";
    const NEXT_MATCHES: usize = 12;

    let expected_matches = pinned_contents(RUST_FILE).matches(QUERY).count();
    assert!(
        expected_matches > NEXT_MATCHES,
        "the pinned editor.rs should contain more than {NEXT_MATCHES} matches for {QUERY:?}"
    );

    let app_state = init_app(cx);
    let zed = open_zed_window(cx, &app_state);
    let editor = open_highlighted(cx, &zed, RUST_FILE, "Rust");
    let search_bar = zed.workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_pane()
            .read(cx)
            .toolbar()
            .read(cx)
            .item_of_type::<search::BufferSearchBar>()
            .expect("panes have a buffer search bar")
    });

    let mut keys: Vec<String> = vec!["cmd-f".to_string()];
    keys.extend(QUERY.chars().map(typed_keystroke));
    let typed_keys = keys.len();

    bench_sessions(
        cx,
        zed.any_window(),
        "buffer_search",
        |cx| {
            focus_editor(&zed, cx, &editor);
            zed.update_window(cx, |window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.change_selections(
                        SelectionEffects::default(),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges([BufferPoint::zero()..BufferPoint::zero()]);
                        },
                    );
                });
            });
            cx.run_until_idle();
            (
                0usize,
                SearchSession {
                    found_match: false,
                    selected_rows: BTreeSet::new(),
                    selected_text_matches: true,
                },
            )
        },
        |(next_key, session), _turn, window, cx| {
            if *next_key < typed_keys {
                keystroke(window, cx, &keys[*next_key]);
                *next_key += 1;
                return Step::Continue;
            }
            // The search runs in the background; frames keep drawing while it does.
            if !session.found_match {
                session.found_match = search_bar.read(cx).query(cx) == QUERY
                    && search_bar.read(cx).has_active_match()
                    && editor
                        .read(cx)
                        .has_background_highlights(editor::HighlightKey::BufferSearchHighlights);
                return Step::Continue;
            }
            if session.selected_rows.len() < NEXT_MATCHES {
                keystroke(window, cx, "enter");
                let (row, text) = selected_text(&editor, cx);
                session.selected_text_matches &= text == QUERY;
                session.selected_rows.insert(row);
                return Step::Continue;
            }
            keystroke(window, cx, "escape");
            Step::Stop
        },
        |(_, session)| {
            assert!(session.found_match, "buffer search found no match");
            assert!(
                session.selected_text_matches,
                "stepping through matches should select the query text"
            );
            assert_eq!(
                session.selected_rows.len(),
                NEXT_MATCHES,
                "each enter should select a match on a different row"
            );
        },
    );

    // Count the highlighted matches outside the measurement.
    focus_editor(&zed, cx, &editor);
    zed.update_window(cx, |window, cx| {
        keystroke(window, cx, "cmd-f");
        for character in QUERY.chars() {
            keystroke(window, cx, &typed_keystroke(character));
        }
    });
    wait_until(cx, "search results", |cx| {
        let query = search_bar.read_with(cx, |search_bar, cx| search_bar.query(cx));
        query == QUERY
            && editor.read_with(cx, |editor, _| {
                editor.has_background_highlights(editor::HighlightKey::BufferSearchHighlights)
            })
    });
    // Distinct highlighted ranges whose text is the query; selection-occurrence
    // highlights of the active match cover the same ranges and are deduplicated.
    let highlighted = zed.update_window(cx, |window, cx| {
        editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(window, cx);
            let buffer_snapshot = snapshot.buffer_snapshot();
            let range = buffer_snapshot.anchor_before(BufferPoint::zero())
                ..buffer_snapshot.anchor_after(buffer_snapshot.max_point());
            editor
                .background_highlights_in_range(range, &snapshot.display_snapshot, cx.theme())
                .into_iter()
                .filter_map(|(range, _)| {
                    let start = range.start.to_point(&snapshot.display_snapshot);
                    let end = range.end.to_point(&snapshot.display_snapshot);
                    let text = buffer_snapshot.text_for_range(start..end).collect::<String>();
                    (text == QUERY).then_some((start, end))
                })
                .collect::<BTreeSet<_>>()
                .len()
        })
    });
    assert_eq!(
        highlighted, expected_matches,
        "buffer search should highlight every match of {QUERY:?}"
    );
}

const FAKE_SERVER_ID: lsp::LanguageServerId = lsp::LanguageServerId(10_000);
const FAKE_PROGRESS_TOKEN: &str = "zed-benchmarks/indexing";

fn progress_notification(value: lsp::WorkDoneProgress) -> lsp::ProgressParams {
    lsp::ProgressParams {
        token: lsp::NumberOrString::String(FAKE_PROGRESS_TOKEN.to_string()),
        value: lsp::ProgressParamsValue::WorkDone(value),
    }
}

struct SpinnerSession {
    started: Instant,
    began: bool,
    ended: bool,
    saw_pending_work: bool,
    pending_after_end: bool,
}

/// 6a. Idle editor while a language server reports work that starts and finishes
/// on a fixed schedule, so the status bar's activity indicator spins and then
/// disappears. Frames are paced at 120 Hz.
#[gpui::bench]
fn idle_language_server_spinner(cx: &mut BenchAppContext) {
    const WORK_STARTS: Duration = Duration::from_millis(20);
    const WORK_ENDS: Duration = Duration::from_millis(170);
    const SESSION: Duration = Duration::from_millis(250);

    let app_state = init_app(cx);
    let zed = open_zed_window(cx, &app_state);
    open_highlighted(cx, &zed, PYTHON_FILE, "Python");
    let lsp_store = zed
        .workspace
        .read_with(cx, |workspace, cx| workspace.project().read(cx).lsp_store());

    bench_sessions(
        cx,
        zed.any_window(),
        "idle_language_server_spinner",
        |cx| {
            // What a running server's status looks like once it has asked to report
            // progress with `window/workDoneProgress/create`.
            lsp_store.update(cx, |lsp_store, _| {
                lsp_store.language_server_statuses.insert(
                    FAKE_SERVER_ID,
                    LanguageServerStatus {
                        name: lsp::LanguageServerName("bench-language-server".into()),
                        language_name: Some("Python".into()),
                        server_version: None,
                        server_readable_version: None,
                        pending_work: Default::default(),
                        has_pending_diagnostic_updates: false,
                        progress_tokens: [ProgressToken::String(FAKE_PROGRESS_TOKEN.into())]
                            .into_iter()
                            .collect(),
                        worktree: None,
                        binary: None,
                        configuration: None,
                        workspace_folders: Default::default(),
                        process_id: None,
                    },
                );
            });
            cx.run_until_idle();
            SpinnerSession {
                started: Instant::now(),
                began: false,
                ended: false,
                saw_pending_work: false,
                pending_after_end: false,
            }
        },
        |session, turn, _window, cx| {
            if turn == 0 {
                session.started = Instant::now();
            }
            pace_frame(session.started, turn, FPS);
            let elapsed = session.started.elapsed();
            if !session.began && elapsed >= WORK_STARTS {
                session.began = true;
                lsp_store.update(cx, |lsp_store, cx| {
                    lsp_store.bench_on_lsp_progress(
                        progress_notification(lsp::WorkDoneProgress::Begin(
                            lsp::WorkDoneProgressBegin {
                                title: "Indexing".to_string(),
                                cancellable: Some(false),
                                message: Some("bench-crate".to_string()),
                                percentage: None,
                            },
                        )),
                        FAKE_SERVER_ID,
                        cx,
                    );
                });
            }
            let pending = lsp_store.read_with(cx, |lsp_store, _| {
                lsp_store
                    .language_server_statuses
                    .get(&FAKE_SERVER_ID)
                    .is_some_and(|status| !status.pending_work.is_empty())
            });
            session.saw_pending_work |= pending;
            if session.began && !session.ended && elapsed >= WORK_ENDS {
                session.ended = true;
                lsp_store.update(cx, |lsp_store, cx| {
                    lsp_store.bench_on_lsp_progress(
                        progress_notification(lsp::WorkDoneProgress::End(
                            lsp::WorkDoneProgressEnd { message: None },
                        )),
                        FAKE_SERVER_ID,
                        cx,
                    );
                });
            } else if session.ended {
                session.pending_after_end |= pending;
            }
            if elapsed >= SESSION {
                Step::Stop
            } else {
                Step::Continue
            }
        },
        |session| {
            assert!(
                session.saw_pending_work,
                "the language server work never started"
            );
            assert!(session.ended, "the language server work never ended");
            assert!(
                !session.pending_after_end,
                "the language server work was still pending after it ended"
            );
        },
    );
}

/// Runs an idle session of `duration` paced at 120 Hz and returns its frame stats.
fn idle_sessions(
    cx: &mut BenchAppContext,
    window: AnyWindowHandle,
    name: &str,
    duration: Duration,
) -> zed_benchmarks::FrameStats {
    bench_sessions(
        cx,
        window,
        name,
        |cx| {
            cx.run_until_idle();
            Instant::now()
        },
        move |started, turn, _window, _cx| {
            if turn == 0 {
                *started = Instant::now();
            }
            pace_frame(*started, turn, FPS);
            if started.elapsed() >= duration {
                Step::Stop
            } else {
                Step::Continue
            }
        },
        |_| {},
    )
}

/// 6b. A focused editor with nothing happening except the blinking cursor.
#[gpui::bench]
fn idle_cursor_blink(cx: &mut BenchAppContext) {
    let app_state = init_app(cx);
    let zed = open_zed_window(cx, &app_state);
    open_highlighted(cx, &zed, RUST_FILE, "Rust");
    let stats = idle_sessions(
        cx,
        zed.any_window(),
        "idle_cursor_blink",
        Duration::from_millis(560),
    );
    assert!(
        stats.draws >= stats.sessions,
        "the cursor should blink at least once per session ({} draws in {} sessions)",
        stats.draws,
        stats.sessions
    );
}

/// 6c. A focused editor with cursor blinking disabled and nothing else going on.
/// GPUI should not draw at all; any foreground work still shows up in the report.
#[gpui::bench]
fn idle_nothing(cx: &mut BenchAppContext) {
    let app_state = init_app_with_settings(cx, r#"{ "cursor_blink": false }"#);
    let zed = open_zed_window(cx, &app_state);
    open_highlighted(cx, &zed, RUST_FILE, "Rust");
    let stats = idle_sessions(
        cx,
        zed.any_window(),
        "idle_nothing",
        Duration::from_millis(250),
    );
    assert_eq!(
        stats.draws, 0,
        "an idle window should not draw ({} draws in {} sessions)",
        stats.draws, stats.sessions
    );
}

struct AgentScrollSession {
    start: ListOffset,
    end: ListOffset,
    delta: f32,
}

/// 7. The agent panel showing a restored thread of about a thousand entries
/// (markdown replies with large code blocks, search and read tool calls and
/// `edit_file` calls with diffs), scrolled with the trackpad.
#[gpui::bench]
fn agent_panel_long_thread(cx: &mut BenchAppContext) {
    const STEPS: u64 = 24;
    const DELTA: f32 = 120.;
    let expected_entries = AGENT_THREAD_TURNS * AGENT_THREAD_ENTRIES_PER_TURN;

    let app_state = init_app(cx);
    let zed = open_zed_window(cx, &app_state);
    let session_id = agent_client_protocol::v2::schema::SessionId::new("zed-benchmarks-thread");
    let thread = agent::DbThread::from_json(&agent_thread_json(&zed.project_root))
        .expect("generated thread must parse as a saved thread");
    let work_dirs = util::path_list::PathList::new(&[zed.project_root.clone()]);
    let save = cx.update(|cx| {
        agent::ThreadStore::global(cx).update(cx, |store, cx| {
            store.save_thread(session_id.clone(), thread, work_dirs.clone(), cx)
        })
    });
    wait_for_task(cx, save).expect("failed to save the generated thread");

    zed.update(cx, |workspace, window, cx| {
        workspace.focus_panel::<agent_ui::AgentPanel>(window, cx);
    });
    cx.run_until_idle();
    let panel = zed.workspace.read_with(cx, |workspace, cx| {
        workspace
            .panel::<agent_ui::AgentPanel>(cx)
            .expect("agent panel was not added")
    });
    zed.update_window(cx, |window, cx| {
        panel.update(cx, |panel, cx| {
            panel.open_thread(session_id.clone(), Some(work_dirs), None, window, cx);
        });
    });
    wait_until(cx, "the restored thread", |cx| {
        panel.read_with(cx, |panel, cx| {
            panel.active_thread_view(cx).is_some_and(|thread_view| {
                thread_view.read(cx).thread.read(cx).entries().len() >= expected_entries
            })
        })
    });
    cx.run_until_idle();
    let thread_view = panel.read_with(cx, |panel, cx| {
        panel.active_thread_view(cx).expect("the thread is open")
    });
    let entry_count = thread_view.read_with(cx, |thread_view, cx| {
        thread_view.thread.read(cx).entries().len()
    });
    assert_eq!(
        entry_count, expected_entries,
        "unexpected thread entry count"
    );
    let diff_count = thread_view.read_with(cx, |thread_view, cx| {
        thread_view
            .thread
            .read(cx)
            .entries()
            .iter()
            .filter(|entry| entry.diffs().next().is_some())
            .count()
    });
    assert_eq!(
        diff_count, AGENT_THREAD_TURNS,
        "every edit_file call should show a diff"
    );
    let list_state = thread_view.read_with(cx, |thread_view, _| thread_view.list_state.clone());
    wait_until(cx, "the thread list layout", |_| {
        list_state.viewport_bounds().size.height > px(0.)
    });
    let viewport = list_state.viewport_bounds();
    assert!(
        viewport.size.height > px(200.),
        "the thread list was not laid out: {viewport:?}"
    );
    let center = viewport.center();
    let mut session_index = 0usize;

    bench_sessions(
        cx,
        zed.any_window(),
        "agent_panel_long_thread",
        |cx| {
            let start_item = (session_index * 89) % (entry_count - 40);
            session_index += 1;
            let start = ListOffset {
                item_ix: start_item,
                offset_in_item: px(0.),
            };
            list_state.scroll_to(start);
            zed.update_window(cx, |window, _| window.refresh());
            cx.run_until_idle();
            AgentScrollSession {
                start: list_state.logical_scroll_top(),
                end: start,
                delta: if session_index % 2 == 0 {
                    DELTA
                } else {
                    -DELTA
                },
            }
        },
        |session, turn, window, cx| {
            scroll_wheel(window, cx, center, session.delta);
            if turn + 1 < STEPS {
                return Step::Continue;
            }
            session.end = list_state.logical_scroll_top();
            Step::Stop
        },
        |session| {
            assert_ne!(
                (session.start.item_ix, session.start.offset_in_item),
                (session.end.item_ix, session.end.offset_in_item),
                "scrolling the thread should move the list"
            );
        },
    );
}

/// 8. The keymap editor (a `uniform_list` table of every binding), scrolled with
/// the trackpad and the keyboard.
#[gpui::bench]
fn keymap_editor_scroll(cx: &mut BenchAppContext) {
    const STEPS: u64 = 30;
    const DELTA: f32 = 120.;

    let app_state = init_app(cx);
    let zed = open_zed_window(cx, &app_state);
    let binding_count = cx.read(|cx| cx.key_bindings().borrow().bindings().count());
    assert!(binding_count > 500, "the default keymap was not loaded");
    zed.update_window(cx, |window, cx| keystrokes(window, cx, "cmd-k cmd-s"));
    wait_until(cx, "the keymap editor", |cx| {
        zed.workspace.read_with(cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .is_some_and(|item| item.tab_content_text(0, cx).contains("Keymap"))
        })
    });
    cx.run_until_idle();
    let content_center = zed.update_window(cx, |window, _| {
        let size = window.viewport_size();
        point(size.width / 2., size.height / 2.)
    });
    let mut session_index = 0u64;

    bench_sessions(
        cx,
        zed.any_window(),
        "keymap_editor_scroll",
        |cx| {
            let downward = session_index % 2 == 0;
            session_index += 1;
            if downward {
                zed.update_window(cx, |window, cx| {
                    window.dispatch_action(menu::SelectFirst.boxed_clone(), cx)
                });
            }
            cx.run_until_idle();
            downward
        },
        |downward, turn, window, cx| {
            if turn % 6 == 5 {
                keystroke(window, cx, if *downward { "down" } else { "up" });
            } else {
                scroll_wheel(
                    window,
                    cx,
                    content_center,
                    if *downward { -DELTA } else { DELTA },
                );
            }
            steps_until(turn, STEPS)
        },
        |_| {},
    );
}

/// 9. The settings window, switching pages through settings links and scrolling
/// each page with the trackpad.
#[gpui::bench]
fn settings_ui_navigate(cx: &mut BenchAppContext) {
    const SETTINGS: &[&str] = &[
        "buffer_font_size",
        "relative_line_numbers",
        "project_panel.indent_guides.show",
        "terminal.font_size",
    ];
    const STEPS_PER_PAGE: u64 = 8;
    const DELTA: f32 = 120.;

    let app_state = init_app(cx);
    let zed = open_zed_window(cx, &app_state);
    zed.update_window(cx, |window, cx| keystroke(window, cx, "cmd-,"));
    wait_until(cx, "the settings window", |cx| {
        cx.read(|cx| {
            cx.windows()
                .iter()
                .any(|window| window.downcast::<settings_ui::SettingsWindow>().is_some())
        })
    });
    let settings_window = cx.read(|cx| {
        cx.windows()
            .into_iter()
            .find_map(|window| window.downcast::<settings_ui::SettingsWindow>())
            .expect("settings window is open")
    });
    settings_window
        .update(cx, |_, window, _| window.activate_window())
        .expect("settings window was closed");
    cx.run_until_idle();
    let (settings_view, content_point) = settings_window
        .update(cx, |_, window, cx| {
            let size = window.viewport_size();
            (cx.entity(), point(size.width * 0.65, size.height / 2.))
        })
        .expect("settings window was closed");

    bench_sessions(
        cx,
        settings_window.into(),
        "settings_ui_navigate",
        |cx| {
            cx.run_until_idle();
            0usize
        },
        |navigated, turn, window, cx| {
            if turn % STEPS_PER_PAGE == 0 {
                let setting = SETTINGS[(turn / STEPS_PER_PAGE) as usize % SETTINGS.len()];
                let found = settings_view
                    .update(cx, |view, cx| view.navigate_to_setting(setting, window, cx));
                assert!(found, "settings UI has no {setting} setting");
                *navigated += 1;
            } else {
                scroll_wheel(window, cx, content_point, -DELTA);
            }
            steps_until(turn, STEPS_PER_PAGE * SETTINGS.len() as u64)
        },
        |navigated| {
            assert_eq!(
                *navigated,
                SETTINGS.len(),
                "every settings page should be visited"
            );
        },
    );
}

gpui::bench_group!(
    name = benches;
    config = criterion::Criterion::default()
        .sample_size(10)
        .warm_up_time(Duration::from_millis(100))
        .measurement_time(Duration::from_millis(500));
    targets =
        project_panel_expand,
        open_split_switch,
        editor_scroll,
        editor_type_and_undo,
        buffer_search,
        idle_language_server_spinner,
        idle_cursor_blink,
        idle_nothing,
        agent_panel_long_thread,
        keymap_editor_scroll,
        settings_ui_navigate,
);
gpui::bench_main!(benches);
