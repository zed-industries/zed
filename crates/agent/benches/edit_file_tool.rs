use std::{
    any::Any,
    future::Future,
    path::Path,
    sync::Arc,
    task::{Context, Poll},
};

use action_log::ActionLog;
use agent::{
    AgentTool, ContextServerRegistry, EditFileTool, EditFileToolInput, EditFileToolOutput,
    Templates, Thread, ToolCallEventStream, ToolInput,
};
use agent_settings::{AgentSettings, ToolRules};
use criterion::{
    BatchSize, BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main,
};
use editor::{Editor, EditorStyle};
use futures::{StreamExt as _, pin_mut, task::noop_waker};
use gpui::{
    AnyWindowHandle, AppContext as _, BackgroundExecutor, Entity, Focusable as _, TestAppContext,
    UpdateGlobal as _,
};
use language::{FakeLspAdapter, rust_lang};
use language_model::fake_provider::FakeLanguageModel;
use project::{FakeFs, Project};
use prompt_store::ProjectContext;
use rand::{Rng as _, SeedableRng as _, rngs::StdRng};
use serde_json::{Value, json};
use settings::{Settings as _, SettingsStore};
use ui::IntoElement as _;

const SEED: u64 = 0x5EED_5EED;
const OLD_TEXT_CHUNK_SIZE: usize = 512;
const NEW_TEXT_CHUNK_SIZE: usize = 512;

const FILE_PROJECT_PATH: &str = "root/src/workspace_snapshot.rs";
const FILE_ABS_PATH: &str = "/root/src/workspace_snapshot.rs";

#[derive(Clone)]
struct EditOp {
    old_text: String,
    new_text: String,
}

#[derive(Clone)]
struct EditFixture {
    name: &'static str,
    old_file_text: String,
    expected_file_text: String,
    edits: Vec<EditOp>,
}

struct BenchmarkHarness {
    cx: Option<TestAppContext>,
    edit_tool: Option<Arc<EditFileTool>>,
    thread: Option<Entity<Thread>>,
    partial_payloads: Vec<Value>,
    final_payload: Value,
    expected_file_text: String,
    editor: Option<Entity<Editor>>,
    window: Option<AnyWindowHandle>,
    // Keeps the LSP buffer-registration handle and the fake language server alive
    // for the lifetime of the benchmark so `didChange`/diagnostics keep flowing
    // while edits are applied.
    keep_alive: Vec<Box<dyn Any>>,
}

impl Drop for BenchmarkHarness {
    fn drop(&mut self) {
        // Release our handles to the entities first.
        self.edit_tool.take();
        self.thread.take();
        self.editor.take();
        self.keep_alive.clear();

        if let Some(mut cx) = self.cx.take() {
            // Close the editor window so the editor entity and the buffer handles
            // it holds are released, then pump the executor so cancelled editor /
            // action-log background tasks drop their captured handles before the
            // leak detector runs on `TestAppContext` drop.
            if let Some(window) = self.window.take() {
                cx.update_window(window, |_, window, _| window.remove_window())
                    .ok();
            }
            cx.update(|_| {});
            cx.executor().run_until_parked();
            cx.quit();
        }
    }
}

fn edit_file_tool_streaming(c: &mut Criterion) {
    let fixtures = fixtures();
    let mut group = c.benchmark_group("edit_file_tool_streaming");
    group.sample_size(10);

    for fixture in fixtures {
        let new_bytes: usize = fixture.edits.iter().map(|edit| edit.new_text.len()).sum();
        group.throughput(Throughput::Bytes(new_bytes as u64));
        group.bench_with_input(
            BenchmarkId::new(fixture.name, fixture.old_file_text.len()),
            &fixture,
            |bench, fixture| {
                bench.iter_batched(
                    || setup_harness(fixture.clone()),
                    |mut harness| {
                        let output = run_streamed_edit(&mut harness);
                        let EditFileToolOutput::Success { new_text, .. } = &output else {
                            panic!("expected edit_file tool to succeed");
                        };
                        assert_eq!(new_text, &harness.expected_file_text);
                        // Return the harness as part of the output so its teardown (which has
                        // to pump the executor to release `Entity<Buffer>` handles captured by
                        // background tasks) runs in criterion's drop phase after the timer has
                        // stopped, rather than inside the timed region.
                        (black_box(output), harness)
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

fn setup_harness(fixture: EditFixture) -> BenchmarkHarness {
    let mut cx = init_context();
    let executor = cx.executor();
    let parts = block_on_executor(
        &executor,
        setup_editor_and_tool(&mut cx, fixture.old_file_text.clone()),
    );
    // Let the LSP handshake, initial parse, and first layout settle before timing.
    cx.executor().run_until_parked();

    let partial_payloads = streamed_partial_payloads(&fixture.edits);
    let final_payload = json!({
        "path": FILE_PROJECT_PATH,
        "edits": fixture
            .edits
            .iter()
            .map(|edit| json!({ "old_text": edit.old_text, "new_text": edit.new_text }))
            .collect::<Vec<_>>(),
    });

    BenchmarkHarness {
        cx: Some(cx),
        edit_tool: Some(parts.edit_tool),
        thread: Some(parts.thread),
        partial_payloads,
        final_payload,
        expected_file_text: fixture.expected_file_text,
        editor: Some(parts.editor),
        window: Some(parts.window),
        keep_alive: parts.keep_alive,
    }
}

struct HarnessParts {
    edit_tool: Arc<EditFileTool>,
    thread: Entity<Thread>,
    editor: Entity<Editor>,
    window: AnyWindowHandle,
    keep_alive: Vec<Box<dyn Any>>,
}

/// Builds a project + edit tool, opens the target buffer in an editor view inside
/// a window, and attaches a fake Rust language server. This mirrors the real app:
/// the edited file is open in a pane with a language server, so each buffer edit
/// drives the editor's observer cascade (matching brackets, code actions, outline,
/// bracket colorization), a tree-sitter reparse, and an LSP `didChange` +
/// diagnostics round-trip — the costs that dominate a real agent edit.
async fn setup_editor_and_tool(cx: &mut TestAppContext, file_text: String) -> HarnessParts {
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "src": {
                "workspace_snapshot.rs": file_text,
            },
        }),
    )
    .await;

    let project = Project::test(fs, [Path::new("/root")], cx).await;
    let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                text_document_sync: Some(lsp::TextDocumentSyncCapability::Kind(
                    lsp::TextDocumentSyncKind::INCREMENTAL,
                )),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
    let model = Arc::new(FakeLanguageModel::default());
    let thread = cx.new(|cx| {
        Thread::new(
            project.clone(),
            cx.new(|_cx| ProjectContext::default()),
            context_server_registry,
            Templates::new(),
            Some(model),
            cx,
        )
    });
    let action_log: Entity<ActionLog> =
        thread.read_with(cx, |thread, _cx| thread.action_log().clone());
    let edit_tool = Arc::new(EditFileTool::new(
        project.clone(),
        thread.downgrade(),
        action_log,
        language_registry,
    ));

    // Open the same buffer the tool will edit and register it with the language
    // servers so edits produce `didChange` notifications.
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(FILE_ABS_PATH, cx)
        })
        .await
        .expect("failed to open buffer");
    let lsp_handle = project.update(cx, |project, cx| {
        project.register_buffer_with_language_servers(&buffer, cx)
    });

    let fake_server = fake_servers
        .next()
        .await
        .expect("fake language server should start");
    // Publish diagnostics on every edit, mirroring a real server reacting to
    // `didChange`, so the editor's diagnostics path runs per edit.
    let server = fake_server.clone();
    fake_server.handle_notification::<lsp::notification::DidChangeTextDocument, _>(
        move |params, _cx| {
            server.notify::<lsp::notification::PublishDiagnostics>(lsp::PublishDiagnosticsParams {
                uri: params.text_document.uri.clone(),
                version: Some(params.text_document.version),
                diagnostics: vec![lsp::Diagnostic {
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 1)),
                    severity: Some(lsp::DiagnosticSeverity::WARNING),
                    message: "bench diagnostic".to_string(),
                    ..Default::default()
                }],
            });
        },
    );

    // Attach an editor view in a window and lay it out once so the viewport-gated
    // observers (bracket colorization, selection highlights) have a visible range.
    let window = cx.add_window(|window, cx| {
        let mut editor = Editor::for_buffer(buffer.clone(), Some(project.clone()), window, cx);
        editor.set_style(EditorStyle::default(), window, cx);
        window.focus(&editor.focus_handle(cx), cx);
        editor
    });
    let editor = window.root(cx).expect("window should have an editor root");
    let window: AnyWindowHandle = window.into();
    // Lay out and paint a real frame so the editor establishes a viewport (this
    // is what makes the viewport-gated observers like bracket colorization run).
    {
        let mut visual_cx = gpui::VisualTestContext::from_window(window, &*cx);
        visual_cx.draw(
            gpui::point(gpui::px(0.0), gpui::px(0.0)),
            gpui::size(gpui::px(1024.0), gpui::px(768.0)),
            |_, _| editor.clone().into_any_element(),
        );
    }

    let keep_alive: Vec<Box<dyn Any>> = vec![
        Box::new(lsp_handle),
        Box::new(fake_server),
        Box::new(fake_servers),
        Box::new(buffer),
    ];

    HarnessParts {
        edit_tool,
        thread,
        editor,
        window,
        keep_alive,
    }
}

fn init_context() -> TestAppContext {
    let cx = TestAppContext::single();
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        assets::Assets.load_test_fonts(cx);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
        SettingsStore::update_global(cx, |store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |settings| {
                settings
                    .project
                    .all_languages
                    .defaults
                    .ensure_final_newline_on_save = Some(false);
                settings.project.all_languages.defaults.colorize_brackets = Some(true);
            });
        });

        let mut agent_settings = AgentSettings::get_global(cx).clone();
        agent_settings.tool_permissions.tools.insert(
            EditFileTool::NAME.into(),
            ToolRules {
                default: Some(settings::ToolPermissionMode::Allow),
                always_allow: vec![],
                always_deny: vec![],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        AgentSettings::override_global(agent_settings, cx);
    });
    cx
}

fn run_streamed_edit(harness: &mut BenchmarkHarness) -> EditFileToolOutput {
    let (mut sender, input): (_, ToolInput<EditFileToolInput>) = ToolInput::test();
    for payload in &harness.partial_payloads {
        sender.send_partial(payload.clone());
    }
    sender.send_full(harness.final_payload.clone());

    let (event_stream, _event_rx) = ToolCallEventStream::test();
    let cx = harness
        .cx
        .as_ref()
        .expect("benchmark harness should have a cx");
    let task = cx.update(|cx| {
        harness
            .edit_tool
            .as_ref()
            .expect("benchmark harness should have an edit tool")
            .clone()
            .run(input, event_stream, cx)
    });

    let executor = harness
        .cx
        .as_ref()
        .expect("benchmark harness should have a cx")
        .executor();
    block_on_executor(&executor, task).unwrap()
}

fn block_on_executor<R>(executor: &BackgroundExecutor, future: impl Future<Output = R>) -> R {
    pin_mut!(future);
    let waker = noop_waker();
    let mut task_context = Context::from_waker(&waker);

    for _ in 0..10_000 {
        if let Poll::Ready(output) = future.as_mut().poll(&mut task_context) {
            return output;
        }
        executor.run_until_parked();
    }

    panic!("future did not complete while running edit_file_tool benchmark");
}

/// Builds the streamed partial payloads for a (possibly multi-edit) session,
/// mirroring how the agent reveals one edit at a time: earlier edits stay
/// complete in the array while the current edit streams its `old_text` then its
/// `new_text` in chunks.
fn streamed_partial_payloads(edits: &[EditOp]) -> Vec<Value> {
    let path = FILE_PROJECT_PATH;
    let mut payloads = vec![json!({ "path": path }), json!({ "path": path })];

    for index in 0..edits.len() {
        let completed: Vec<Value> = edits[..index]
            .iter()
            .map(|edit| json!({ "old_text": edit.old_text, "new_text": edit.new_text }))
            .collect();
        let edit = &edits[index];

        for old_end in chunk_ends(&edit.old_text, OLD_TEXT_CHUNK_SIZE) {
            let mut arr = completed.clone();
            arr.push(json!({ "old_text": &edit.old_text[..old_end] }));
            payloads.push(json!({ "path": path, "edits": arr }));
        }

        let mut arr = completed.clone();
        arr.push(json!({ "old_text": edit.old_text, "new_text": "" }));
        payloads.push(json!({ "path": path, "edits": arr }));

        for new_end in chunk_ends(&edit.new_text, NEW_TEXT_CHUNK_SIZE) {
            let mut arr = completed.clone();
            arr.push(json!({ "old_text": edit.old_text, "new_text": &edit.new_text[..new_end] }));
            payloads.push(json!({ "path": path, "edits": arr }));
        }
    }

    payloads
}

fn chunk_ends(text: &str, chunk_size: usize) -> impl Iterator<Item = usize> + '_ {
    let mut end = 0;
    std::iter::from_fn(move || {
        if end == text.len() {
            return None;
        }

        end = (end + chunk_size).min(text.len());
        while !text.is_char_boundary(end) {
            end -= 1;
        }
        Some(end)
    })
}

fn fixtures() -> Vec<EditFixture> {
    vec![
        make_fixture(
            "tiny_function_rewrite",
            2,
            EditPattern::LocalizedRewrite {
                start_line: 12,
                line_count: 6,
            },
            SEED,
        ),
        make_fixture(
            "small_function_rewrite",
            5,
            EditPattern::LocalizedRewrite {
                start_line: 22,
                line_count: 12,
            },
            SEED + 1,
        ),
        make_fixture(
            "medium_many_small_changes",
            8,
            EditPattern::ManySmallChanges { every_nth_line: 7 },
            SEED + 2,
        ),
        make_fixture(
            "medium_insertions",
            8,
            EditPattern::InsertHelperBlocks { every_nth_line: 9 },
            SEED + 3,
        ),
        make_large_multi_edit_fixture("large_multi_edit", 80, 16, SEED + 4),
    ]
}

enum EditPattern {
    LocalizedRewrite {
        start_line: usize,
        line_count: usize,
    },
    ManySmallChanges {
        every_nth_line: usize,
    },
    InsertHelperBlocks {
        every_nth_line: usize,
    },
}

fn make_fixture(
    name: &'static str,
    function_count: usize,
    pattern: EditPattern,
    seed: u64,
) -> EditFixture {
    let mut rng = StdRng::seed_from_u64(seed);
    let old_lines = random_rust_module(&mut rng, function_count);
    let edit_range = edit_range(&old_lines, &pattern);
    let old_text = old_lines[edit_range.clone()].join("\n");
    let mut new_lines = old_lines.clone();

    match pattern {
        EditPattern::LocalizedRewrite { .. } => {
            rewrite_local_block(&mut new_lines[edit_range.clone()], &mut rng)
        }
        EditPattern::ManySmallChanges { every_nth_line } => {
            rewrite_many_small_lines(&mut new_lines[edit_range.clone()], every_nth_line, &mut rng)
        }
        EditPattern::InsertHelperBlocks { every_nth_line } => {
            insert_helper_blocks(&mut new_lines, edit_range.clone(), every_nth_line, &mut rng)
        }
    }

    let new_text_end = edit_range.end + new_lines.len().saturating_sub(old_lines.len());
    let old_file_text = old_lines.join("\n");
    let expected_file_text = new_lines.join("\n");
    let new_text = new_lines[edit_range.start..new_text_end].join("\n");

    EditFixture {
        name,
        old_file_text,
        expected_file_text,
        edits: vec![EditOp { old_text, new_text }],
    }
}

fn make_large_multi_edit_fixture(
    name: &'static str,
    function_count: usize,
    edit_count: usize,
    seed: u64,
) -> EditFixture {
    const HEADER_LINES: usize = 10;
    const FUNCTION_LINES: usize = 12;
    const FUNCTION_BODY_LINES: usize = 11;

    let mut rng = StdRng::seed_from_u64(seed);
    let old_lines = random_rust_module(&mut rng, function_count);
    let old_file_text = old_lines.join("\n");

    let step = (function_count / edit_count).max(1);
    let mut picks: Vec<usize> = (0..edit_count)
        .map(|k| (k * step).min(function_count - 1))
        .collect();
    picks.dedup();

    let replacements: Vec<(usize, Vec<String>)> = picks
        .iter()
        .map(|&function_index| {
            (
                function_index,
                large_function_lines(&mut rng, function_index),
            )
        })
        .collect();

    let edits = replacements
        .iter()
        .map(|(function_index, new_function)| {
            let start = HEADER_LINES + function_index * FUNCTION_LINES;
            let end = start + FUNCTION_BODY_LINES;
            EditOp {
                old_text: old_lines[start..end].join("\n"),
                new_text: new_function.join("\n"),
            }
        })
        .collect();

    let mut new_lines = old_lines;
    for (function_index, new_function) in replacements.iter().rev() {
        let start = HEADER_LINES + function_index * FUNCTION_LINES;
        let end = start + FUNCTION_BODY_LINES;
        new_lines.splice(start..end, new_function.iter().cloned());
    }
    let expected_file_text = new_lines.join("\n");

    EditFixture {
        name,
        old_file_text,
        expected_file_text,
        edits,
    }
}

fn large_function_lines(rng: &mut StdRng, index: usize) -> Vec<String> {
    let function_name = identifier(rng, index + 40_000);
    let argument_name = identifier(rng, index + 41_000);

    let mut lines = vec![
        format!(
            "    pub fn {function_name}(&mut self, {argument_name}: usize) -> Result<usize> {{"
        ),
        format!("        let mut accumulator = {argument_name};"),
    ];

    let body_lines = rng.random_range(30..42);
    for body_index in 0..body_lines {
        let local_name = identifier(rng, index + 50_000 + body_index);
        let multiplier = rng.random_range(2..19);
        let offset = rng.random_range(1..256);
        match body_index % 4 {
            0 => lines.push(format!(
                "        let {local_name} = accumulator.saturating_mul({multiplier}).saturating_add({offset});"
            )),
            1 => lines.push(format!(
                "        accumulator = {local_name}.saturating_sub(self.version % {offset}.max(1));"
            )),
            2 => lines.push(format!(
                "        if {local_name} % {multiplier} == 0 {{ accumulator = accumulator.saturating_add({local_name}); }}"
            )),
            _ => lines.push(format!(
                "        self.buffers.insert(\"{local_name}\".to_string(), accumulator);"
            )),
        }
    }

    lines.push("        self.version = self.version.saturating_add(accumulator);".to_string());
    lines.push("        Ok(accumulator)".to_string());
    lines.push("    }".to_string());
    lines
}

fn edit_range(lines: &[String], pattern: &EditPattern) -> std::ops::Range<usize> {
    let mut range = match pattern {
        EditPattern::LocalizedRewrite {
            start_line,
            line_count,
        } => *start_line..(*start_line + *line_count).min(lines.len()),
        EditPattern::ManySmallChanges { .. } | EditPattern::InsertHelperBlocks { .. } => {
            10..lines.len().saturating_sub(5)
        }
    };

    while range.end > range.start && lines[range.end - 1].is_empty() {
        range.end -= 1;
    }

    range
}

fn random_rust_module(rng: &mut StdRng, function_count: usize) -> Vec<String> {
    let mut lines = vec![
        "use anyhow::{Context as _, Result};".to_string(),
        "use collections::HashMap;".to_string(),
        "".to_string(),
        "#[derive(Clone, Debug)]".to_string(),
        "pub struct WorkspaceSnapshot {".to_string(),
        "    buffers: HashMap<String, usize>,".to_string(),
        "    version: usize,".to_string(),
        "}".to_string(),
        "".to_string(),
        "impl WorkspaceSnapshot {".to_string(),
    ];

    for function_index in 0..function_count {
        let function_name = identifier(rng, function_index);
        let argument_name = identifier(rng, function_index + 1_000);
        let local_name = identifier(rng, function_index + 2_000);
        let branch_name = identifier(rng, function_index + 3_000);
        let multiplier = rng.random_range(2..17);
        let offset = rng.random_range(1..128);

        lines.extend([
            format!(
                "    pub fn {function_name}(&mut self, {argument_name}: usize) -> Result<usize> {{"
            ),
            format!("        let mut {local_name} = {argument_name}.saturating_mul({multiplier});"),
            format!("        if {local_name} % 2 == 0 {{"),
            format!(
                "            {local_name} = {local_name}.saturating_add(self.version + {offset});"
            ),
            "        } else {".to_string(),
            format!("            {local_name} = {local_name}.saturating_sub({offset});"),
            "        }".to_string(),
            format!("        let {branch_name} = self.buffers.len().saturating_add({local_name});"),
            format!("        self.version = self.version.saturating_add({branch_name});"),
            format!("        Ok({branch_name})"),
            "    }".to_string(),
            "".to_string(),
        ]);
    }

    lines.push("}".to_string());
    lines.push("".to_string());
    lines.push("pub fn normalize_path(path: &str) -> String {".to_string());
    lines.push("    path.replace('\\\\', \"/\")".to_string());
    lines.push("}".to_string());
    lines
}

fn rewrite_local_block(lines: &mut [String], rng: &mut StdRng) {
    for (line_index, line) in lines.iter_mut().enumerate() {
        let suffix = identifier(rng, line_index + 10_000);
        if line.contains("saturating_add") {
            *line = format!(
                "        let {suffix} = self.version.checked_add({line_index}).context(\"version overflow\")?;"
            );
        } else if line.contains("saturating_sub") {
            *line = format!(
                "            {suffix}.saturating_sub({});",
                rng.random_range(8..256)
            );
        } else if line.trim().is_empty() {
            *line =
                format!("        tracing::trace!(target: \"agent_bench\", value = {line_index});");
        } else {
            *line = format!("{line} // updated {suffix}");
        }
    }
}

fn rewrite_many_small_lines(lines: &mut [String], every_nth_line: usize, rng: &mut StdRng) {
    for (line_index, line) in lines.iter_mut().enumerate() {
        if line_index.is_multiple_of(every_nth_line) || line.trim().is_empty() {
            continue;
        }

        let suffix = identifier(rng, line_index + 20_000);
        *line = format!("{line} // audited {suffix}");
    }
}

fn insert_helper_blocks(
    lines: &mut Vec<String>,
    range: std::ops::Range<usize>,
    every_nth_line: usize,
    rng: &mut StdRng,
) {
    let mut line_index = range.start;
    while line_index < range.end.min(lines.len()) {
        if line_index.is_multiple_of(every_nth_line) && !lines[line_index].trim().is_empty() {
            let suffix = identifier(rng, line_index + 30_000);
            lines.splice(
                line_index..line_index,
                [
                    format!("        let {suffix}_before = self.version;"),
                    format!("        tracing::debug!(version = {suffix}_before);"),
                ],
            );
            line_index += 2;
        }
        line_index += 1;
    }
}

fn identifier(rng: &mut StdRng, salt: usize) -> String {
    const PARTS: &[&str] = &[
        "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "theta", "lambda", "sigma", "omega",
    ];
    format!(
        "{}_{}_{}",
        PARTS[rng.random_range(0..PARTS.len())],
        salt,
        rng.random_range(0..10_000)
    )
}

criterion_group!(benches, edit_file_tool_streaming);
criterion_main!(benches);
