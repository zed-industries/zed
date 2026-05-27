use std::{
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
use futures::{pin_mut, task::noop_waker};
use gpui::{AppContext as _, BackgroundExecutor, Entity, TestAppContext, UpdateGlobal as _};
use language_model::fake_provider::FakeLanguageModel;
use project::{FakeFs, Project};
use prompt_store::ProjectContext;
use rand::{Rng as _, SeedableRng as _, rngs::StdRng};
use serde_json::{Value, json};
use settings::{Settings as _, SettingsStore};

const SEED: u64 = 0x5EED_5EED;
const OLD_TEXT_CHUNK_SIZE: usize = 512;
const NEW_TEXT_CHUNK_SIZE: usize = 512;

#[derive(Clone)]
struct EditFixture {
    name: &'static str,
    old_file_text: String,
    expected_file_text: String,
    old_text: String,
    new_text: String,
}

struct BenchmarkHarness {
    cx: Option<TestAppContext>,
    edit_tool: Option<Arc<EditFileTool>>,
    thread: Option<Entity<Thread>>,
    partial_payloads: Vec<Value>,
    final_payload: Value,
    expected_file_text: String,
}

impl Drop for BenchmarkHarness {
    fn drop(&mut self) {
        // Release our handles to the entities first.
        self.edit_tool.take();
        self.thread.take();

        if let Some(cx) = self.cx.take() {
            // `ActionLog` holds buffers strongly via `tracked_buffers`, and spawns a background
            // diff-maintenance task that also captures a strong `Entity<Buffer>`. Releasing the
            // last handle to the action log only marks its entity for deferred release; the
            // entity's value (and the buffer handles inside) is not actually dropped until
            // `flush_effects` runs `release_dropped_entities`. Even then, the cancelled task's
            // captured handle does not drop until the executor pumps the cancellation through.
            //
            // Without this two-step teardown, GPUI's test leak detector panics on
            // `TestAppContext` drop because the buffer still appears alive. See
            // `ActionLog::track_buffer_internal` and `LeakDetector::drop` in
            // `crates/gpui/src/app/entity_map.rs`.
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
        group.throughput(Throughput::Bytes(fixture.new_text.len() as u64));
        group.bench_with_input(
            BenchmarkId::new(fixture.name, fixture.old_text.len()),
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
    let (edit_tool, thread) = block_on_executor(
        &executor,
        setup_edit_tool(&mut cx, fixture.old_file_text.clone()),
    );
    let partial_payloads = streamed_partial_payloads(&fixture.old_text, &fixture.new_text);
    let final_payload = json!({
        "path": "root/src/workspace_snapshot.rs",
        "edits": [{
            "old_text": fixture.old_text,
            "new_text": fixture.new_text,
        }],
    });

    BenchmarkHarness {
        cx: Some(cx),
        edit_tool: Some(edit_tool),
        thread: Some(thread),
        partial_payloads,
        final_payload,
        expected_file_text: fixture.expected_file_text,
    }
}

fn init_context() -> TestAppContext {
    let cx = TestAppContext::single();
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        SettingsStore::update_global(cx, |store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |settings| {
                settings
                    .project
                    .all_languages
                    .defaults
                    .ensure_final_newline_on_save = Some(false);
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

async fn setup_edit_tool(
    cx: &mut TestAppContext,
    file_text: String,
) -> (Arc<EditFileTool>, Entity<Thread>) {
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
        project,
        thread.downgrade(),
        action_log,
        language_registry,
    ));
    (edit_tool, thread)
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

fn streamed_partial_payloads(old_text: &str, new_text: &str) -> Vec<Value> {
    let path = "root/src/workspace_snapshot.rs";
    let mut payloads = Vec::new();

    payloads.push(json!({ "path": path }));
    payloads.push(json!({ "path": path }));

    for old_end in chunk_ends(old_text, OLD_TEXT_CHUNK_SIZE) {
        payloads.push(json!({
            "path": path,
            "edits": [{ "old_text": &old_text[..old_end] }],
        }));
    }

    payloads.push(json!({
        "path": path,
        "edits": [{ "old_text": old_text, "new_text": "" }],
    }));

    for new_end in chunk_ends(new_text, NEW_TEXT_CHUNK_SIZE) {
        payloads.push(json!({
            "path": path,
            "edits": [{
                "old_text": old_text,
                "new_text": &new_text[..new_end],
            }],
        }));
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
        old_text,
        new_text,
    }
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
