//! Stress benchmark for the agent panel.
//!
//! Builds a workspace with an `AgentPanel` whose active thread contains
//! hundreds of entries — long markdown responses, thinking blocks, tool calls
//! with diffs and terminal-style output, and sub-agent invocations — then
//! measures frame latency while scrolling through the conversation. The
//! workload is intentionally heavy enough to drop frames so regressions (and
//! improvements) in agent panel rendering are visible in the frame report.

use std::{cell::RefCell, collections::VecDeque, future::Future, rc::Rc, time::Duration};

use acp_thread::{
    AgentThreadEntry, AssistantMessageChunk, SUBAGENT_SESSION_INFO_META_KEY, StubAgentConnection,
    SubagentSessionInfo, TerminalProviderEvent,
};
use agent_client_protocol::schema as acp;
use agent_ui::AgentPanel;
use agent_ui::test_support::{StubAgentServer, init_test_app};
use clock::FakeSystemClock;
use fs::{FakeFs, Fs};
use gpui::{AppContext as _, BenchAppContext, FollowMode, UpdateGlobal as _, px, size};
use language::{LanguageRegistry, rust_lang};
use node_runtime::NodeRuntime;
use project::Project;
use serde_json::json;
use settings::SettingsStore;
use std::sync::Arc;
use util::path;
use util::paths::PathStyle;
use workspace::MultiWorkspace;

/// Conversation turns to fabricate. Each turn adds a user message, a thinking
/// block, a long markdown response, and several tool calls.
const TURNS: usize = 40;
/// Spawn a sub-agent tool call every N turns.
const SUBAGENT_EVERY: usize = 8;
/// Entries to fabricate inside each sub-agent session.
const SUBAGENT_ENTRIES: usize = 8;
/// Pixels scrolled per frame during the measured loop (fast flick at 120fps).
const SCROLL_STEP: f32 = 360.0;
/// Frames spent scrolling between cold jumps. Each cycle streams new content
/// into a huge tool call, jumps to it (scrollbar-drag style), and scrolls
/// through the surrounding entries.
const FRAMES_PER_CYCLE: usize = 30;
/// Line count for the giant tool outputs (e.g. a full `cargo build` log).
const HUGE_TOOL_OUTPUT_LINES: usize = 8000;
/// Every Nth turn gets a giant tool output entry.
const HUGE_OUTPUT_EVERY: usize = 4;
/// Every Nth turn embeds a real (display-only) terminal view in a tool call.
const TERMINAL_VIEW_EVERY: usize = 5;
/// Lines of ANSI-colored output initially written into each embedded terminal.
const TERMINAL_VIEW_LINES: usize = 600;
/// Number of external frame inputs to synthesize before measurement starts.
const PRECOMPUTED_FRAME_INPUTS: usize = 512;

/// Drives a `'static` future to completion by spawning it on the benchmark's
/// foreground executor and pumping the dispatcher until it resolves.
fn block_on<R: 'static>(cx: &mut BenchAppContext, future: impl Future<Output = R> + 'static) -> R {
    let result = Rc::new(RefCell::new(None));
    cx.foreground_executor()
        .spawn({
            let result = result.clone();
            async move {
                *result.borrow_mut() = Some(future.await);
            }
        })
        .detach();
    let started = std::time::Instant::now();
    let deadline = started + Duration::from_secs(60);
    let mut last_report = started;
    loop {
        cx.run_until_idle();
        if let Some(value) = result.borrow_mut().take() {
            return value;
        }
        if last_report.elapsed() > Duration::from_secs(5) {
            last_report = std::time::Instant::now();
            if let Some(dispatcher) = cx.background_executor().dispatcher().as_bench() {
                eprintln!(
                    "[bench setup] waiting {:?}: {}",
                    started.elapsed(),
                    dispatcher.debug_state()
                );
            }
        }
        assert!(
            std::time::Instant::now() < deadline,
            "timed out waiting for benchmark setup future"
        );
        // Let real-time timers (e.g. debounces) come due before re-pumping.
        std::thread::sleep(Duration::from_millis(1));
    }
}

fn file_path(turn: usize) -> String {
    format!("/project/src/file_{turn}.rs")
}

/// A bracket-dense, deeply nested line of Rust. Generic-heavy signatures
/// maximize tree-sitter bracket pairs (`<>` also exercises the bogus-match
/// repair pass), nesting deeper than the 13 theme accents spreads highlights
/// across every `ColorizeBracket` key, and the length defeats soft wrap.
fn bracket_soup(turn: usize, line: usize) -> String {
    format!(
        "fn original_{turn}_{line}<T: Iterator<Item = Result<HashMap<String, \
         Vec<Option<Box<dyn Fn(usize) -> Result<Vec<(usize, [u8; 4])>, Error>>>>>, Error>>>(\
         value: T) -> Vec<Vec<(usize, usize)>> {{ \
         vec![vec![(({line}, {turn}))], (0..{line}).map(|index| ((index, index * {turn}))).collect()] }}\n"
    )
}

fn old_file_text(turn: usize) -> String {
    let mut text = String::new();
    for line in 0..240 {
        text.push_str(&bracket_soup(turn, line));
    }
    text
}

fn new_file_text(turn: usize) -> String {
    churned_file_text(turn, 7)
}

/// Like [`old_file_text`] with every `period`th line rewritten. Six-plus
/// unchanged lines between changes keeps each hunk a distinct excerpt
/// (excerpt context is 2 lines), so hunk count drives excerpt count.
fn churned_file_text(turn: usize, period: usize) -> String {
    let mut text = String::new();
    for line in 0..240 {
        if line % period == 0 {
            text.push_str(&format!(
                "fn rewritten_{turn}_{line}(value: usize) -> Vec<Vec<usize>> {{ \
                 vec![vec![value * {line} + 1]] }}\n"
            ));
        } else {
            text.push_str(&bracket_soup(turn, line));
        }
    }
    text
}

fn assistant_markdown(turn: usize) -> String {
    let mut text = format!(
        "## Investigating issue #{turn}\n\n\
         I looked at the **frame scheduling** path and found a few things worth\n\
         calling out. The `dispatch_after` timers were firing on the *main*\n\
         thread, which interacts badly with `run_until_idle`:\n\n"
    );
    for item in 0..6 {
        text.push_str(&format!(
            "{}. `module_{turn}::function_{item}` re-entrantly locks the state mutex \
             while holding the inflight guard, see [the docs](https://zed.dev/docs/{item})\n",
            item + 1
        ));
    }
    // A large `rust` fence: the markdown renderer re-runs a full tree-sitter
    // parse + highlight of this block on every frame it's laid out.
    text.push_str("\n```rust\n");
    for line in 0..40 {
        text.push_str(&format!(
            "    let nested_{line}: Vec<HashMap<String, Vec<Option<Box<dyn Fn(usize) -> \
             Result<Vec<(usize, usize)>, Error>>>>>> = vec![(0..{line}).map(|value| \
             ((value, [value; 4]), vec![Some((value, {turn}))])).collect()];\n"
        ));
    }
    text.push_str("```\n\nThe fix is to drain the queue *before* taking the lock:\n\n");
    for item in 0..4 {
        text.push_str(&format!(
            "- **Step {item}**: split `take_bencher` so the `Rc<RefCell<_>>` borrow ends \
             before `bencher.iter` runs, avoiding the double-borrow panic\n"
        ));
    }
    text
}

fn thought_markdown(turn: usize) -> String {
    format!(
        "I need to check whether the list state re-measures items when entry {turn} \
         updates mid-scroll. The interaction between `splice_focusable` and \
         `remeasure_items` could explain the dropped frames.\n\n\
         Let me look at:\n- the invalidation path\n- the measurement cache\n- \
         whether `FollowMode::Tail` snaps during streaming\n"
    )
}

fn terminal_output(turn: usize) -> String {
    terminal_output_sized(turn, 40)
}

/// A giant single entry, like streaming `cargo build` output. Content inside
/// one entry is not virtualized by the list, so these dominate frame cost
/// whenever one is visible.
fn huge_terminal_output(turn: usize) -> String {
    terminal_output_sized(turn, HUGE_TOOL_OUTPUT_LINES)
}

fn terminal_output_sized(turn: usize, lines: usize) -> String {
    let mut text = format!("```\n$ cargo test -p crate_{turn} --lib\n");
    for line in 0..lines {
        text.push_str(&format!(
            "test module_{turn}::tests::case_{line:05} ... ok ({line}.{turn:02}ms) \
             [worker {} | retries 0 | mem {}KB]\n",
            line % 16,
            1024 + line * 7 % 4096,
        ));
    }
    text.push_str(&format!("test result: ok. {lines} passed; 0 failed\n```\n"));
    text
}

fn text_content(text: String) -> acp::ToolCallContent {
    acp::ToolCallContent::Content(acp::Content::new(acp::ContentBlock::Text(
        acp::TextContent::new(text),
    )))
}

/// ANSI-colored compiler-style output, making the embedded terminal's
/// alacritty grid parse escape sequences like real `cargo build` output.
fn ansi_terminal_output(seed: usize, lines: usize) -> Vec<u8> {
    let mut output = Vec::new();
    for line in 0..lines {
        let color = 31 + (line + seed) % 6;
        output.extend_from_slice(
            format!(
                "\x1b[1;32m   Compiling\x1b[0m crate_{seed} v0.{line}.0 \
                 (\x1b[{color}m/project/src/file_{line}.rs\x1b[0m): \
                 \x1b[1mwarning\x1b[0m: unused variable `value_{line}`\r\n"
            )
            .as_bytes(),
        );
    }
    output
}

fn plan_update(completed_through: usize) -> acp::SessionUpdate {
    let entries = (0..24)
        .map(|item| {
            let status = if item < completed_through {
                acp::PlanEntryStatus::Completed
            } else if item == completed_through {
                acp::PlanEntryStatus::InProgress
            } else {
                acp::PlanEntryStatus::Pending
            };
            acp::PlanEntry::new(
                format!(
                    "**Step {item}**: refactor `module_{item}::render` to cache \
                     highlighted chunks across frames (see `crates/editor`)"
                ),
                acp::PlanEntryPriority::Medium,
                status,
            )
        })
        .collect();
    acp::SessionUpdate::Plan(acp::Plan::new(entries))
}

struct HugeEntry {
    item_ix: usize,
    tool_call_id: String,
    turn: usize,
}

struct FrameInput {
    cycle_frame: usize,
    huge_entry_ix: usize,
    streaming_terminal_id: acp::TerminalId,
    streaming_terminal_output: Vec<u8>,
    session_updates: Vec<acp::SessionUpdate>,
}

fn frame_input(
    frame_ix: usize,
    huge_entries: &[HugeEntry],
    terminal_ids: &[acp::TerminalId],
) -> FrameInput {
    let cycle = frame_ix / FRAMES_PER_CYCLE;
    let cycle_frame = frame_ix % FRAMES_PER_CYCLE;
    let payload_seed = frame_ix + 1;

    let huge_entry = &huge_entries[cycle % huge_entries.len()];
    let mut session_updates = Vec::with_capacity(if cycle_frame == 3 { 2 } else { 1 });
    if cycle_frame == 3 {
        session_updates.push(plan_update(cycle % 24));
    }

    if cycle_frame == 2 {
        let churn_period = if cycle % 2 == 0 { 5 } else { 7 };
        session_updates.push(acp::SessionUpdate::ToolCallUpdate(
            acp::ToolCallUpdate::new(
                format!("edit-{}", huge_entry.turn),
                acp::ToolCallUpdateFields::new()
                    .status(acp::ToolCallStatus::InProgress)
                    .content(vec![acp::ToolCallContent::Diff(
                        acp::Diff::new(
                            file_path(huge_entry.turn),
                            churned_file_text(huge_entry.turn, churn_period),
                        )
                        .old_text(old_file_text(huge_entry.turn)),
                    )]),
            ),
        ));
    } else if cycle_frame == 0 {
        session_updates.push(acp::SessionUpdate::ToolCallUpdate(
            acp::ToolCallUpdate::new(
                huge_entry.tool_call_id.clone(),
                acp::ToolCallUpdateFields::new()
                    .status(acp::ToolCallStatus::InProgress)
                    .content(vec![text_content(huge_terminal_output(payload_seed))]),
            ),
        ));
    } else {
        session_updates.push(acp::SessionUpdate::ToolCallUpdate(
            acp::ToolCallUpdate::new(
                format!("read-{}", payload_seed % TURNS),
                acp::ToolCallUpdateFields::new()
                    .status(acp::ToolCallStatus::Completed)
                    .content(vec![text_content(terminal_output(payload_seed))]),
            ),
        ));
    }

    FrameInput {
        cycle_frame,
        huge_entry_ix: huge_entry.item_ix,
        streaming_terminal_id: terminal_ids[cycle % terminal_ids.len()].clone(),
        streaming_terminal_output: ansi_terminal_output(payload_seed, 6),
        session_updates,
    }
}

/// Builds the session updates for one conversation turn.
fn turn_updates(turn: usize) -> Vec<acp::SessionUpdate> {
    let mut updates = vec![
        acp::SessionUpdate::UserMessageChunk(acp::ContentChunk::new(
            format!(
                "Please investigate the frame drops in `crates/module_{turn}` and fix \
                 the scheduling bug we discussed. Remember to run the tests!"
            )
            .into(),
        )),
        acp::SessionUpdate::AgentThoughtChunk(acp::ContentChunk::new(
            thought_markdown(turn).into(),
        )),
        // Stream the response in several chunks like a real model would; each
        // chunk re-parses the accumulated markdown.
        acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
            assistant_markdown(turn).into(),
        )),
        acp::SessionUpdate::ToolCall(
            acp::ToolCall::new(format!("read-{turn}"), format!("Read `src/file_{turn}.rs`"))
                .kind(acp::ToolKind::Read)
                .status(acp::ToolCallStatus::Completed)
                .raw_input(json!({ "path": file_path(turn), "start_line": 1 }))
                .content(vec![text_content(format!(
                    "```rust\n{}```",
                    old_file_text(turn)
                ))]),
        ),
        acp::SessionUpdate::ToolCall(
            acp::ToolCall::new(format!("terminal-{turn}"), "Run `cargo test`")
                .kind(acp::ToolKind::Execute)
                .status(acp::ToolCallStatus::Completed)
                .raw_input(json!({ "command": format!("cargo test -p crate_{turn}") }))
                .content(vec![text_content(if turn % HUGE_OUTPUT_EVERY == 0 {
                    huge_terminal_output(turn)
                } else {
                    terminal_output(turn)
                })]),
        ),
        acp::SessionUpdate::ToolCall(
            acp::ToolCall::new(format!("edit-{turn}"), format!("Edit `src/file_{turn}.rs`"))
                .kind(acp::ToolKind::Edit)
                .status(acp::ToolCallStatus::Completed)
                .locations(vec![acp::ToolCallLocation::new(file_path(turn))])
                .content(vec![acp::ToolCallContent::Diff(
                    acp::Diff::new(file_path(turn), new_file_text(turn))
                        .old_text(old_file_text(turn)),
                )]),
        ),
    ];
    if turn % 3 == 0 {
        updates.push(acp::SessionUpdate::ToolCall(
            acp::ToolCall::new(format!("fetch-{turn}"), "Fetch `https://zed.dev/docs/gpui`")
                .kind(acp::ToolKind::Fetch)
                .status(acp::ToolCallStatus::Completed)
                .content(vec![text_content(format!(
                    "# GPUI docs (turn {turn})\n\nFetched **2.3KB** of documentation \
                     covering `ListState`, `FollowMode`, and frame scheduling."
                ))]),
        ));
    }
    updates
}

#[gpui::bench]
fn agent_panel_scroll_heavy_thread(cx: &mut BenchAppContext) {
    // === Global init ===
    cx.update(|cx| {
        init_test_app(cx);
        assets::Assets.load_test_fonts(cx);
        agent::ThreadStore::init_global(cx);
        agent_ui::thread_metadata_store::ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
        // Bracket colorization shows up in production frame-drop profiles;
        // it must be opted into, and only runs for buffers with a language.
        SettingsStore::update_global(cx, |store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.all_languages.defaults.colorize_brackets = Some(true);
            });
        });
    });

    // === FakeFs project with files for the diff tool calls to resolve ===
    let fs = FakeFs::new(cx.background_executor().clone());
    let mut tree = serde_json::Map::new();
    for turn in 0..TURNS {
        tree.insert(format!("file_{turn}.rs"), json!(old_file_text(turn)));
    }
    block_on(cx, {
        let fs = fs.clone();
        async move {
            fs.insert_tree(path!("/project"), json!({ "src": tree }))
                .await
        }
    });
    cx.update(|cx| <dyn Fs>::set_global(fs.clone(), cx));

    // === Project (mirrors `Project::test`, which needs a TestAppContext) ===
    let project = cx.update(|cx| {
        let languages = LanguageRegistry::test(cx.background_executor().clone());
        // A real Rust grammar (with brackets.scm + highlights.scm) so diff
        // editors and markdown code fences run tree-sitter like production.
        languages.add(rust_lang());
        let clock = Arc::new(FakeSystemClock::new());
        let http_client = http_client::FakeHttpClient::with_404_response();
        let client = client::Client::new(clock, http_client, cx);
        let user_store = cx.new(|cx| client::UserStore::new(client.clone(), cx));
        Project::local(
            client,
            NodeRuntime::unavailable(),
            user_store,
            Arc::new(languages),
            fs.clone(),
            None,
            project::LocalProjectFlags::default(),
            cx,
        )
    });
    let worktree_task = cx.update(|cx| {
        project.update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/project"), true, cx)
        })
    });
    let (worktree, _) = block_on(cx, worktree_task).expect("failed to create worktree");
    let scan = cx.read(|cx| {
        worktree
            .read(cx)
            .as_local()
            .expect("worktree should be local")
            .scan_complete()
    });
    block_on(cx, scan);

    // === Window with a workspace root and a visible agent panel ===
    let mut window = cx.add_empty_window();
    // Sized like the logical resolution of a 16" MacBook Pro (the test window
    // reports a 2.0 scale factor, so the device size is 3456x2234).
    window.update(|window, cx| {
        window.resize(size(px(1728.0), px(1117.0)));
        window.bounds_changed(cx);
    });
    let multi_workspace = window.update(|window, cx| {
        window.replace_root(cx, |window, cx| {
            MultiWorkspace::test_new(project.clone(), window, cx)
        })
    });
    let workspace = window.update(|_, cx| multi_workspace.read(cx).workspace().clone());
    let panel = window.update(|window, cx| {
        workspace.update(cx, |workspace, cx| {
            let panel = cx.new(|cx| AgentPanel::test_new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            workspace.focus_panel::<AgentPanel>(window, cx);
            panel
        })
    });
    cx.run_until_idle();

    // Open real editors in the workspace center, like a user working while
    // the agent runs: the window renders the full app (tabs, gutters, syntax
    // highlighted buffers) every frame, not just the panel, and buffer/project
    // events flow through the same main thread.
    let worktree_id = cx.read(|cx| worktree.read(cx).id());
    for file in ["src/file_1.rs", "src/file_2.rs"] {
        let open_task = window.update(|window, cx| {
            workspace.update(cx, |workspace, cx| {
                workspace.open_path(
                    project::ProjectPath {
                        worktree_id,
                        path: util::rel_path::rel_path(file).into(),
                    },
                    None,
                    true,
                    window,
                    cx,
                )
            })
        });
        block_on(cx, open_task).expect("failed to open editor in workspace");
    }
    cx.run_until_idle();

    // === Open a thread backed by the stub connection ===
    let connection = StubAgentConnection::new().with_supports_load_session(true);
    window.update(|window, cx| {
        panel.update(cx, |panel, cx| {
            panel.open_external_thread_with_server(
                Rc::new(StubAgentServer::new(connection.clone())),
                window,
                cx,
            );
        })
    });
    cx.run_until_idle();

    let thread = cx.read(|cx| {
        panel
            .read(cx)
            .active_agent_thread(cx)
            .expect("panel should have an active agent thread")
    });
    let session_id = cx.read(|cx| thread.read(cx).session_id().clone());

    // === Fabricate the heavy conversation ===
    // One `cx.update` per session update: entry view syncing assumes each
    // `NewEntry` event is observed before the next entry lands, which holds in
    // production because updates arrive as individual messages.
    let mut terminal_ids = Vec::new();
    for turn in 0..TURNS {
        for update in turn_updates(turn) {
            cx.update(|cx| connection.send_update(session_id.clone(), update, cx));
        }

        // A live plan that mutates as turns complete; its in-progress entry
        // renders with a rotating animation like production.
        cx.update(|cx| connection.send_update(session_id.clone(), plan_update(turn % 24), cx));

        if turn % TERMINAL_VIEW_EVERY == 0 {
            // Embed a real terminal view (display-only: a full alacritty grid
            // without a PTY) and fill it with ANSI-colored output.
            let terminal_id = acp::TerminalId::new(format!("term-view-{turn}"));
            terminal_ids.push(terminal_id.clone());
            cx.update(|cx| {
                let lower = cx.new(|cx| {
                    terminal::TerminalBuilder::new_display_only(
                        terminal::terminal_settings::CursorShape::default(),
                        terminal::terminal_settings::AlternateScroll::On,
                        None,
                        0,
                        cx.background_executor(),
                        PathStyle::local(),
                    )
                    .subscribe(cx)
                });
                thread.update(cx, |thread, cx| {
                    thread.on_terminal_provider_event(
                        TerminalProviderEvent::Created {
                            terminal_id: terminal_id.clone(),
                            label: format!("cargo build --turn {turn}"),
                            cwd: None,
                            output_byte_limit: None,
                            terminal: lower,
                        },
                        cx,
                    );
                    thread.on_terminal_provider_event(
                        TerminalProviderEvent::Output {
                            terminal_id: terminal_id.clone(),
                            data: ansi_terminal_output(turn, TERMINAL_VIEW_LINES),
                        },
                        cx,
                    );
                });
                connection.send_update(
                    session_id.clone(),
                    acp::SessionUpdate::ToolCall(
                        acp::ToolCall::new(
                            format!("term-view-call-{turn}"),
                            "Run `cargo build` in terminal",
                        )
                        .kind(acp::ToolKind::Execute)
                        .status(acp::ToolCallStatus::InProgress)
                        .content(vec![acp::ToolCallContent::Terminal(acp::Terminal::new(
                            terminal_id.clone(),
                        ))]),
                    ),
                    cx,
                );
            });
        }

        if turn % SUBAGENT_EVERY == 0 {
            let subagent_id = acp::SessionId::new(format!("subagent-{turn}"));
            cx.update(|cx| {
                connection.send_update(
                    session_id.clone(),
                    acp::SessionUpdate::ToolCall(
                        acp::ToolCall::new(
                            format!("spawn-agent-{turn}"),
                            format!("Investigate flaky test {turn}"),
                        )
                        .kind(acp::ToolKind::Think)
                        .status(acp::ToolCallStatus::InProgress)
                        .meta(acp::Meta::from_iter([(
                            SUBAGENT_SESSION_INFO_META_KEY.into(),
                            json!(SubagentSessionInfo {
                                session_id: subagent_id.clone(),
                                message_start_index: 0,
                                message_end_index: None,
                            }),
                        )])),
                    ),
                    cx,
                );
                thread.update(cx, |thread, cx| {
                    thread.subagent_spawned(subagent_id.clone(), cx);
                });
            });
            // Let the conversation view load the sub-agent session before
            // streaming content into it.
            cx.run_until_idle();
            for entry in 0..SUBAGENT_ENTRIES {
                let update = if entry % 2 == 0 {
                    acp::SessionUpdate::AgentThoughtChunk(acp::ContentChunk::new(
                        thought_markdown(entry).into(),
                    ))
                } else {
                    acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
                        assistant_markdown(entry).into(),
                    ))
                };
                cx.update(|cx| connection.send_update(subagent_id.clone(), update, cx));
            }
            // Sub-agents edit files too.
            cx.update(|cx| {
                connection.send_update(
                    subagent_id.clone(),
                    acp::SessionUpdate::ToolCall(
                        acp::ToolCall::new(
                            format!("subagent-edit-{turn}"),
                            format!("Edit `src/file_{turn}.rs`"),
                        )
                        .kind(acp::ToolKind::Edit)
                        .status(acp::ToolCallStatus::Completed)
                        .content(vec![acp::ToolCallContent::Diff(
                            acp::Diff::new(file_path(turn), churned_file_text(turn, 11))
                                .old_text(old_file_text(turn)),
                        )]),
                    ),
                    cx,
                );
            });
        }

        // Drain entry-view syncing and diff buffer loads as we go, like
        // streaming would in production.
        cx.run_until_idle();
    }
    cx.run_until_idle();

    // === Sanity-check that the fabricated content actually rendered ===
    let thread_view = cx.read(|cx| {
        panel
            .read(cx)
            .active_thread_view(cx)
            .expect("panel should have an active thread view")
    });
    let (entry_count, item_count) = cx.read(|cx| {
        let view = thread_view.read(cx);
        (
            view.thread.read(cx).entries().len(),
            view.list_state.item_count(),
        )
    });
    // Each turn yields at least: user message, assistant message (thought and
    // text chunks merge into one entry), and three tool calls.
    assert!(
        entry_count >= TURNS * 5,
        "expected a heavy thread, got only {entry_count} entries"
    );
    assert_eq!(
        item_count, entry_count,
        "list items should mirror thread entries"
    );

    // === Expand all cards, like a user reviewing the work ===
    // Collapsed tool calls don't render their content at all, so without this
    // the heavy markdown/diff/terminal output would never hit the renderer.
    window.update(|_, cx| {
        let entries: Vec<_> = thread
            .read(cx)
            .entries()
            .iter()
            .enumerate()
            .map(|(entry_ix, entry)| match entry {
                AgentThreadEntry::ToolCall(tool_call) => (Some(tool_call.id.clone()), entry_ix, 0),
                AgentThreadEntry::AssistantMessage(message) => {
                    let thoughts = message
                        .chunks
                        .iter()
                        .filter(|chunk| matches!(chunk, AssistantMessageChunk::Thought { .. }))
                        .count();
                    (None, entry_ix, thoughts)
                }
                _ => (None, entry_ix, 0),
            })
            .collect();
        thread_view.update(cx, |view, cx| {
            for (tool_call_id, entry_ix, thought_chunks) in entries {
                if let Some(id) = tool_call_id {
                    view.expanded_tool_calls.insert(id.clone());
                    // Raw inputs render as a JSON markdown block when expanded.
                    view.expanded_tool_call_raw_inputs.insert(id);
                }
                for chunk_ix in 0..thought_chunks {
                    view.expanded_thinking_blocks.insert((entry_ix, chunk_ix));
                }
            }
            cx.notify();
        });
    });
    cx.run_until_idle();

    // Sanity-check that expansion actually renders the heavy content: scroll
    // to a huge terminal entry, draw, and verify it measures very tall.
    let huge_entry_ix = cx.read(|cx| {
        thread
            .read(cx)
            .entries()
            .iter()
            .position(|entry| {
                matches!(entry, AgentThreadEntry::ToolCall(tool_call) if tool_call.id.0.as_ref() == "terminal-0")
            })
            .expect("the first huge terminal tool call should exist")
    });
    window.update(|_, cx| {
        thread_view.update(cx, |view, cx| {
            view.list_state.scroll_to(gpui::ListOffset {
                item_ix: huge_entry_ix,
                offset_in_item: px(0.0),
            });
            cx.notify();
        });
    });
    cx.run_until_idle();
    let huge_entry_height = cx.read(|cx| {
        thread_view
            .read(cx)
            .list_state
            .bounds_for_item(huge_entry_ix)
            .map(|bounds| bounds.size.height)
    });
    assert!(
        huge_entry_height.is_some_and(|height| height > px(2000.0)),
        "expanded tool output should render tall; expansion may be broken: {huge_entry_height:?}"
    );

    // === Measure scrolling while the agent keeps streaming ===
    //
    // This is the scenario where the agent panel drops frames in production:
    // the user scrolls through a long conversation while tool calls are still
    // streaming output. Each cycle:
    //   1. streams a fresh giant log into one of the huge tool calls (which
    //      invalidates its measured height),
    //   2. jumps the scroll position there, scrollbar-drag style (forcing the
    //      list to re-measure the dirtied ~150k px entry),
    //   3. replaces a nearby edit tool call's diff, which rebuilds the diff
    //      from scratch like a streaming `edit_file_tool` update (new buffer,
    //      tree-sitter parse, background re-diff, brand-new editor), and
    //   4. flick-scrolls through the surrounding entries while smaller tool
    //      call updates keep arriving.
    // In parallel, every frame streams ANSI output into one of the embedded
    //   terminals, and each cycle advances the live plan.
    // Content alternates rather than accumulating so the workload stays in a
    // steady state across Criterion samples.
    let huge_entries: Vec<HugeEntry> = cx.read(|cx| {
        thread
            .read(cx)
            .entries()
            .iter()
            .enumerate()
            .filter_map(|(entry_ix, entry)| match entry {
                AgentThreadEntry::ToolCall(tool_call)
                    if tool_call.id.0.starts_with("terminal-") =>
                {
                    let turn = tool_call
                        .id
                        .0
                        .trim_start_matches("terminal-")
                        .parse::<usize>()
                        .ok()?;
                    if turn % HUGE_OUTPUT_EVERY == 0 {
                        Some(HugeEntry {
                            item_ix: entry_ix,
                            tool_call_id: tool_call.id.0.to_string(),
                            turn,
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    });
    assert!(!huge_entries.is_empty());

    cx.read(|cx| {
        let list_state = &thread_view.read(cx).list_state;
        list_state.set_follow_mode(FollowMode::Normal);
        list_state.scroll_to_end();
    });

    let mut frame_inputs: VecDeque<_> = (0..PRECOMPUTED_FRAME_INPUTS)
        .map(|frame_ix| frame_input(frame_ix, &huge_entries, &terminal_ids))
        .collect();
    let thread = thread.clone();
    let mut direction = -1.0f32;
    cx.bench_iter(move |cx| {
        let frame_input = frame_inputs.pop_front().expect(
            "agent panel benchmark consumed all precomputed frame inputs; increase PRECOMPUTED_FRAME_INPUTS",
        );
        window.update(|_, cx| {
            // Terminal output streams continuously, like `cargo build`
            // running in an embedded terminal card.
            thread.update(cx, |thread, cx| {
                thread.on_terminal_provider_event(
                    TerminalProviderEvent::Output {
                        terminal_id: frame_input.streaming_terminal_id,
                        data: frame_input.streaming_terminal_output,
                    },
                    cx,
                );
            });

            for update in frame_input.session_updates {
                connection.send_update(session_id.clone(), update, cx);
            }

            thread_view.update(cx, |view, cx| {
                if frame_input.cycle_frame == 1 {
                    // Scrollbar-drag jump to the dirtied giant entry.
                    view.list_state.scroll_to(gpui::ListOffset {
                        item_ix: frame_input.huge_entry_ix,
                        offset_in_item: px(0.0),
                    });
                    direction = 1.0;
                } else {
                    view.list_state.scroll_by(px(SCROLL_STEP * direction));
                    if frame_input.cycle_frame == FRAMES_PER_CYCLE / 2 {
                        direction = -direction;
                    }
                }
                cx.notify();
            });
        });

        // Apply queued async work (markdown parses, diff buffer updates) the
        // way the platform run loop would between frames.
        cx.run_until_idle();
        window.update(|window, _| window.present_if_needed());
    });
}

gpui::bench_group!(benches, agent_panel_scroll_heavy_thread);
gpui::bench_main!(benches);
