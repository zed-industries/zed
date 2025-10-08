use super::*;
use crate::tools::{ListHistoryTool, MemoryTool};
use anyhow::{Context as _, Result};
use gpui::TestAppContext;
use serde_json::json;
use std::sync::Arc;

/// These integration-style tests exercise the `memory` and `list_history`
/// tools directly against an in‑memory `Thread` constructed via the existing
/// `setup` harness. They avoid model completions where unnecessary by
/// inserting only user messages. This keeps the surface minimal while
/// validating durable archive round‑trips and history enumeration.
///
/// Rationale:
/// * We construct tool input structs via serde_json because their fields
///   are intentionally non-public; this mirrors how real tool invocations
///   are deserialized from JSON arguments.
/// * We assert on stable textual markers in the markdown output (e.g.,
///   "Handle:", "Archived Memories") instead of attempting to parse full
///   tables, minimizing brittleness while still guaranteeing correctness.
/// * We do not add extensive helper abstractions to keep the test logic
///   straightforward and explicit.
///
/// NOTE: If this file is added and tests do not compile, ensure
/// `mod memory_history_integration_tests;` is declared inside
/// `crates/agent2/src/tests/mod.rs`.

/// Helper: run the list_history tool and return its raw markdown output.
async fn run_list_history(
    cx: &mut TestAppContext,
    thread: &Entity<Thread>,
    start: Option<usize>,
    limit: Option<usize>,
    max_chars: Option<usize>,
    include_full: Option<bool>,
) -> Result<String> {
    let mut obj = serde_json::Map::new();
    if let Some(s) = start {
        obj.insert("start".into(), s.into());
    }
    if let Some(l) = limit {
        obj.insert("limit".into(), l.into());
    }
    if let Some(m) = max_chars {
        obj.insert("max_chars_per_message".into(), m.into());
    }
    if let Some(f) = include_full {
        obj.insert("include_full_markdown".into(), f.into());
    }
    let input_json = serde_json::Value::Object(obj);
    let input: crate::tools::ListHistoryToolInput =
        serde_json::from_value(input_json).context("deserialize ListHistoryToolInput")?;

    let tool = Arc::new(ListHistoryTool::new(thread.downgrade()));
    let task = thread.update(cx, |_, thread_cx| {
        let (event_stream, _rx) = crate::ToolCallEventStream::test();
        tool.clone().run(input, event_stream, thread_cx)
    });
    task.await.context("run list_history tool")
}

/// Helper: run memory tool with arbitrary JSON input.
async fn run_memory(
    cx: &mut TestAppContext,
    thread: &Entity<Thread>,
    input_json: serde_json::Value,
) -> Result<String> {
    let input: crate::tools::MemoryToolInput =
        serde_json::from_value(input_json).context("deserialize MemoryToolInput")?;
    let tool = Arc::new(MemoryTool::new(thread.downgrade()));
    let task = thread.update(cx, |_, thread_cx| {
        let (event_stream, _rx) = crate::ToolCallEventStream::test();
        tool.clone().run(input, event_stream, thread_cx)
    });
    task.await.context("run memory tool")
}

/// Extract archive handle from a memory store markdown output.
fn extract_handle(store_output: &str) -> Option<String> {
    store_output
        .lines()
        .find_map(|l| l.strip_prefix("Handle: "))
        .map(|s| s.trim().to_string())
}

/// Basic history enumeration + truncation sanity check.
#[gpui::test]
async fn list_history_basic_and_truncation(cx: &mut TestAppContext) -> Result<()> {
    let ThreadTest { thread, .. } = setup(cx, TestModel::Fake).await;

    for i in 0..6 {
        let content = format!("User message number {i} with some extra descriptive text");
        let _ = thread.update(cx, |thread, cx| {
            thread.send(acp_thread::UserMessageId::new(), [content.as_str()], cx)
        })?;
    }
    cx.run_until_parked();

    let output_default = run_list_history(cx, &thread, None, Some(5), None, None).await?;
    assert!(
        output_default.contains("0") && output_default.contains("4"),
        "Expected indices 0..4 in default output; got:\n{output_default}"
    );

    let output_trunc =
        run_list_history(cx, &thread, Some(2), Some(3), Some(32), Some(false)).await?;
    assert!(
        output_trunc.contains("2") && output_trunc.contains("4"),
        "Expected indices 2..4 in truncated output; got:\n{output_trunc}"
    );
    Ok(())
}

/// Round‑trip archive lifecycle: store -> list -> load -> restore (remove placeholder) -> prune.
#[gpui::test]
async fn memory_store_list_load_restore_prune_roundtrip(cx: &mut TestAppContext) -> Result<()> {
    let ThreadTest { thread, .. } = setup(cx, TestModel::Fake).await;

    for i in 0..5 {
        let content = format!("Seed user message {i}");
        let _ = thread.update(cx, |thread, cx| {
            thread.send(acp_thread::UserMessageId::new(), [content.as_str()], cx)
        })?;
    }
    cx.run_until_parked();

    let store_out = run_memory(
        cx,
        &thread,
        json!({
            "operation": "store",
            "start_index": 0,
            "end_index": 2,
            "summary": "Archive of first three messages"
        }),
    )
    .await?;
    let handle =
        extract_handle(&store_out).context("failed to parse archive handle from store output")?;
    assert!(
        store_out.contains("Range: 0..=2"),
        "Store output missing expected range line:\n{store_out}"
    );

    let list_out = run_memory(cx, &thread, json!({"operation":"list"})).await?;
    assert!(
        list_out.contains(&handle),
        "List output did not contain handle {handle}:\n{list_out}"
    );
    assert!(
        list_out.contains("Archived Memories"),
        "List output missing heading:\n{list_out}"
    );

    let load_out = run_memory(
        cx,
        &thread,
        json!({
            "operation":"load",
            "memory_handle": handle
        }),
    )
    .await?;
    assert!(
        load_out.contains("Summary:"),
        "Load output missing summary:\n{load_out}"
    );

    let restore_out = run_memory(
        cx,
        &thread,
        json!({
            "operation":"restore",
            "memory_handle": handle,
            "remove_placeholder": true,
            "restore_insert_index": 5
        }),
    )
    .await?;
    assert!(
        restore_out.contains("Placeholder: removed"),
        "Restore output did not record placeholder removal:\n{restore_out}"
    );

    let prune_out = run_memory(cx, &thread, json!({"operation":"prune"})).await?;
    if !prune_out.contains(&handle) {
        let list_after = run_memory(cx, &thread, json!({"operation":"list"})).await?;
        assert!(
            !list_after.contains(&handle),
            "Archive still listed after prune:\n{list_after}"
        );
    }

    Ok(())
}

/// Negative test: store without required indices must error.
#[gpui::test]
async fn memory_store_missing_indices_errors(cx: &mut TestAppContext) -> Result<()> {
    let ThreadTest { thread, .. } = setup(cx, TestModel::Fake).await;

    let _ = thread.update(cx, |thread, cx| {
        thread.send(acp_thread::UserMessageId::new(), ["One message"], cx)
    })?;
    cx.run_until_parked();

    let result = run_memory(
        cx,
        &thread,
        json!({
            "operation":"store",
            "summary":"Should fail because indices missing"
        }),
    )
    .await;

    assert!(
        result.is_err(),
        "Store without indices unexpectedly succeeded:\n{result:?}"
    );
    let err = format!("{result:?}");
    assert!(
        err.contains("store requires start_index") || err.contains("start_index"),
        "Error did not mention missing start_index: {err}"
    );
    Ok(())
}

/// Restore with placeholder replacement test.
#[gpui::test]
async fn memory_restore_with_placeholder_replacement(cx: &mut TestAppContext) -> Result<()> {
    let ThreadTest { thread, .. } = setup(cx, TestModel::Fake).await;
    for i in 0..3 {
        let _ = thread.update(cx, |thread, cx| {
            thread.send(
                acp_thread::UserMessageId::new(),
                [format!("Msg {i}").as_str()],
                cx,
            )
        })?;
    }
    cx.run_until_parked();

    let store_out = run_memory(
        cx,
        &thread,
        serde_json::json!({
            "operation":"store",
            "start_index":0,
            "end_index":1,
            "summary":"Two messages to be replaced"
        }),
    )
    .await?;
    let handle = extract_handle(&store_out).context("missing handle")?;

    let restore_out = run_memory(
        cx,
        &thread,
        serde_json::json!({
            "operation":"restore",
            "memory_handle": handle,
            "replace_placeholder_with":"[Replaced Placeholder Marker]"
        }),
    )
    .await?;
    assert!(
        restore_out.contains("Placeholder: replaced"),
        "Restore output did not indicate replacement:\n{restore_out}"
    );

    let list_out = run_memory(cx, &thread, serde_json::json!({"operation":"list"})).await?;
    assert!(
        list_out.contains(&handle),
        "Archive handle missing after replacement:\n{list_out}"
    );
    Ok(())
}

/// Auto-summary generation test (auto:true, no explicit summary)
#[gpui::test]
async fn memory_store_auto_summary_generation(cx: &mut TestAppContext) -> Result<()> {
    let ThreadTest { thread, .. } = setup(cx, TestModel::Fake).await;
    for i in 0..4 {
        let _ = thread.update(cx, |thread, cx| {
            thread.send(
                acp_thread::UserMessageId::new(),
                [format!("Auto seed message {i}").as_str()],
                cx,
            )
        })?;
    }
    cx.run_until_parked();

    let store_out = run_memory(
        cx,
        &thread,
        serde_json::json!({
            "operation":"store",
            "start_index":1,
            "end_index":3,
            "auto": true
        }),
    )
    .await?;
    let handle = extract_handle(&store_out).context("missing handle")?;

    let load_out = run_memory(
        cx,
        &thread,
        serde_json::json!({
            "operation":"load",
            "memory_handle": handle,
            "max_preview_chars":160
        }),
    )
    .await?;
    assert!(
        !load_out.contains("No summary provided"),
        "Auto summary fallback was not generated:\n{load_out}"
    );
    assert!(
        load_out.contains("Summary:"),
        "Load output missing summary field:\n{load_out}"
    );
    Ok(())
}
