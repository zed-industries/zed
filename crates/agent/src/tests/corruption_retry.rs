use super::*;
use crate::thread::{BASE_RETRY_DELAY, MAX_CORRUPTION_RETRY_ATTEMPTS};
use language_model::{LanguageModelCompletionEvent, LanguageModelToolUse, StopReason};
use pretty_assertions::assert_eq;

/// When the model ends a turn without calling `attempt_completion`,
/// a MissingCompletionTool error should be injected and a retry triggered.
#[gpui::test]
async fn test_missing_completion_tool_triggers_retry(cx: &mut TestAppContext) {
    let ThreadTest { thread, model, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    thread.update(cx, |thread, _cx| {
        thread.add_tool(EchoTool);
    });

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Do something"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    // First attempt: model ends turn with text but no attempt_completion call.
    // This should trigger a MissingCompletionTool error and a retry.
    fake_model.send_last_completion_stream_text_chunk("I'm done!");
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();

    // Advance past the corruption retry delay
    cx.executor().advance_clock(BASE_RETRY_DELAY);
    cx.run_until_parked();

    // Second attempt: model properly calls attempt_completion
    let attempt_completion_tool_use = LanguageModelToolUse {
        id: "ac_1".into(),
        name: "attempt_completion".into(),
        raw_input: "{}".to_string(),
        input: serde_json::json!({}),
        is_input_complete: true,
        thought_signature: None,
    };
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        attempt_completion_tool_use,
    ));
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // Collect events: should see one retry from the MissingCompletionTool error
    let mut retry_events = Vec::new();
    while let Some(Ok(event)) = events.next().await {
        match event {
            ThreadEvent::Retry(retry_status) => {
                retry_events.push(retry_status);
            }
            ThreadEvent::Stop(..) => break,
            _ => {}
        }
    }

    assert_eq!(retry_events.len(), 1);
    assert!(
        retry_events[0]
            .last_error
            .to_string()
            .contains("attempt_completion"),
        "retry error should mention attempt_completion, got: {}",
        retry_events[0].last_error
    );
}

/// After MAX_CORRUPTION_RETRY_ATTEMPTS retries without attempt_completion,
/// the error should propagate (no more retries).
#[gpui::test]
async fn test_missing_completion_tool_retries_exhausted(cx: &mut TestAppContext) {
    let ThreadTest { thread, model, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    thread.update(cx, |thread, _cx| {
        thread.add_tool(EchoTool);
    });

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Do something"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    // Simulate more rounds of missing attempt_completion than MAX_CORRUPTION_RETRY_ATTEMPTS.
    // Each round: model ends turn without attempt_completion → retry.
    // After MAX_CORRUPTION_RETRY_ATTEMPTS + 1, the error should propagate.
    for _ in 0..=MAX_CORRUPTION_RETRY_ATTEMPTS {
        fake_model.send_last_completion_stream_text_chunk("I'm done!");
        fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(
            StopReason::EndTurn,
        ));
        fake_model.end_last_completion_stream();

        cx.executor().advance_clock(BASE_RETRY_DELAY);
        cx.run_until_parked();
    }

    let mut retry_events = Vec::new();
    let mut stop_events = Vec::new();
    let mut errors = Vec::new();
    while let Some(event) = events.next().await {
        match event {
            Ok(ThreadEvent::Retry(retry_status)) => {
                retry_events.push(retry_status);
            }
            Ok(ThreadEvent::Stop(reason)) => {
                stop_events.push(reason);
                break;
            }
            Err(error) => {
                // The error message should mention attempt_completion
                let msg = error.to_string();
                assert!(
                    msg.contains("attempt_completion"),
                    "error should mention attempt_completion, got: {msg}"
                );
                errors.push(error);
                break;
            }
            _ => {}
        }
    }

    // Should have exactly MAX_CORRUPTION_RETRY_ATTEMPTS retry events before failing
    assert_eq!(
        retry_events.len(),
        MAX_CORRUPTION_RETRY_ATTEMPTS as usize,
        "expected {} retry events, got {}",
        MAX_CORRUPTION_RETRY_ATTEMPTS,
        retry_events.len()
    );

    // Should have an error after retries exhausted
    assert!(
        !errors.is_empty(),
        "expected an error event after retries exhausted"
    );
}

/// When the model DOES call attempt_completion, no MissingCompletionTool retry should occur.
#[gpui::test]
async fn test_attempt_completion_present_no_retry(cx: &mut TestAppContext) {
    let ThreadTest { thread, model, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    thread.update(cx, |thread, _cx| {
        thread.add_tool(EchoTool);
    });

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Do something"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    // Model calls attempt_completion (as a ToolUse). Send ToolUse then
    // EndTurn. The tool use will be processed first (since tool_results
    // won't be empty), and on the next iteration the completion will end
    // normally (the attempt_completion tool use is in the message history).
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "ac_1".into(),
            name: "attempt_completion".into(),
            raw_input: "{}".to_string(),
            input: serde_json::json!({}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // After tool processing, on the next loop iteration the model sends
    // a brief text and ends the turn. Since attempt_completion was already
    // called in a prior message, no retry should be triggered.
    fake_model.send_last_completion_stream_text_chunk("Done");
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let mut retry_events = Vec::new();
    while let Some(Ok(event)) = events.next().await {
        match event {
            ThreadEvent::Retry(retry_status) => {
                retry_events.push(retry_status);
            }
            ThreadEvent::Stop(..) => break,
            _ => {}
        }
    }

    assert_eq!(
        retry_events.len(),
        0,
        "no retry should occur when attempt_completion is called"
    );
}

/// Corruption retry attempt counts are tracked correctly: each retry
/// increases the attempt counter within the retry status.
#[gpui::test]
async fn test_corruption_retry_attempt_counts(cx: &mut TestAppContext) {
    let ThreadTest { thread, model, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    thread.update(cx, |thread, _cx| {
        thread.add_tool(EchoTool);
    });

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Do something"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    // Simulate MAX_CORRUPTION_RETRY_ATTEMPTS rounds of missing attempt_completion
    for _ in 0..MAX_CORRUPTION_RETRY_ATTEMPTS {
        fake_model.send_last_completion_stream_text_chunk("I'm done!");
        fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(
            StopReason::EndTurn,
        ));
        fake_model.end_last_completion_stream();

        cx.executor().advance_clock(BASE_RETRY_DELAY);
        cx.run_until_parked();
    }

    // Now provide attempt_completion on the next attempt
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "ac_final".into(),
            name: "attempt_completion".into(),
            raw_input: "{}".to_string(),
            input: serde_json::json!({}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let mut retry_events = Vec::new();
    while let Some(Ok(event)) = events.next().await {
        match event {
            ThreadEvent::Retry(retry_status) => {
                retry_events.push(retry_status);
            }
            ThreadEvent::Stop(..) => break,
            _ => {}
        }
    }

    assert_eq!(
        retry_events.len(),
        MAX_CORRUPTION_RETRY_ATTEMPTS as usize,
        "expected exactly {} retry events",
        MAX_CORRUPTION_RETRY_ATTEMPTS
    );

    // Verify attempt counts increment
    for (i, retry) in retry_events.iter().enumerate() {
        assert_eq!(
            retry.attempt,
            i + 1,
            "retry attempt should be {}, got {}",
            i + 1,
            retry.attempt
        );
        assert_eq!(
            retry.max_attempts, MAX_CORRUPTION_RETRY_ATTEMPTS as usize,
            "max_attempts should be {}",
            MAX_CORRUPTION_RETRY_ATTEMPTS
        );
    }
}

/// Tests that `CompletionError::Corrupted(CorruptionDetail)` errors go through
/// the corruption retry path with bounded retries.
///
/// Currently marked `#[ignore]` because `CompletionError::Corrupted` is only
/// produced by the output quality detector (Phase B). Once the detector is
/// integrated into `handle_text_event`, this test can be enabled.
///
/// When enabled, a fake model helper should inject a corrupted completion that
/// triggers the detector, then verify:
/// - `ThreadEvent::Retry` events with `max_attempts == MAX_CORRUPTION_RETRY_ATTEMPTS`
/// - After `MAX_CORRUPTION_RETRY_ATTEMPTS` retries, the fallback model is attempted
/// - The corruption retry counter increments correctly
#[gpui::test]
#[ignore = "requires output quality detector from Phase B"]
async fn test_corrupted_output_triggers_retry(cx: &mut TestAppContext) {
    let ThreadTest { thread, model, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    thread.update(cx, |thread, _cx| {
        thread.add_tool(EchoTool);
    });

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Do something"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    // TODO(Phase B): Wire the output quality detector so that sending high-entropy
    // or script-switching text causes a `CompletionError::Corrupted(CorruptionDetail)`
    // to be constructed inside `handle_text_event`. Then simulate repeated
    // corrupted completions and verify the retry counter.

    // Placeholder: advance through the expected retry cycles
    for _ in 0..=MAX_CORRUPTION_RETRY_ATTEMPTS {
        fake_model.send_last_completion_stream_text_chunk("corrupted output");
        fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(
            StopReason::EndTurn,
        ));
        fake_model.end_last_completion_stream();

        cx.executor().advance_clock(BASE_RETRY_DELAY);
        cx.run_until_parked();
    }

    let mut retry_events = Vec::new();
    while let Some(Ok(event)) = events.next().await {
        match event {
            ThreadEvent::Retry(retry_status) => {
                retry_events.push(retry_status);
            }
            ThreadEvent::Stop(..) => break,
            _ => {}
        }
    }

    // TODO(Phase B): Once the detector is active, assert that
    // retry_events.len() == MAX_CORRUPTION_RETRY_ATTEMPTS and each
    // retry_status.last_error contains "output corruption detected".
    assert_eq!(
        retry_events.len(),
        MAX_CORRUPTION_RETRY_ATTEMPTS as usize,
        "expected {MAX_CORRUPTION_RETRY_ATTEMPTS} corruption retries, got {}",
        retry_events.len()
    );
}

/// Tests that after `MAX_CORRUPTION_RETRY_ATTEMPTS` corruption retries are
/// exhausted, a fallback model is attempted (if the current model has
/// `refusal_fallback_model_id` configured), or the error propagates otherwise.
///
/// Currently marked `#[ignore]` because `CompletionError::Corrupted` is only
/// produced by the output quality detector (Phase B). Once the detector is
/// integrated, this test can be enabled.
///
/// When enabled, configure a fake model with a `refusal_fallback_model_id`,
/// simulate corruption exhaustions, and verify:
/// - The fallback model is selected via `set_model`
/// - A final `ThreadEvent::Retry` with `meta` containing the fallback name
/// - On subsequent attempts, the fallback model receives the completion request
#[gpui::test]
#[ignore = "requires output quality detector from Phase B + fallback model"]
async fn test_corruption_fallback_model_on_exhaustion(cx: &mut TestAppContext) {
    let ThreadTest { thread, model, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    thread.update(cx, |thread, _cx| {
        thread.add_tool(EchoTool);
    });

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Do something"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    // TODO(Phase B): Simulate MAX_CORRUPTION_RETRY_ATTEMPTS + 1 corrupted
    // completions from the primary model. On the final attempt, the thread
    // should attempt a fallback model (if configured) or propagate the error.
    for _ in 0..=MAX_CORRUPTION_RETRY_ATTEMPTS + 1 {
        fake_model.send_last_completion_stream_text_chunk("corrupted output");
        fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(
            StopReason::EndTurn,
        ));
        fake_model.end_last_completion_stream();

        cx.executor().advance_clock(BASE_RETRY_DELAY);
        cx.run_until_parked();
    }

    let mut retry_events = Vec::new();
    let mut errors = Vec::new();
    while let Some(event) = events.next().await {
        match event {
            Ok(ThreadEvent::Retry(retry_status)) => {
                retry_events.push(retry_status);
            }
            Ok(ThreadEvent::Stop(..)) => break,
            Err(error) => {
                errors.push(error);
                break;
            }
            _ => {}
        }
    }

    // TODO(Phase B): Assert that after MAX_CORRUPTION_RETRY_ATTEMPTS retries,
    // either a fallback model retry event was emitted or the error propagated.
    // The fallback retry should have meta containing the fallback model name.
    assert!(
        !retry_events.is_empty() || !errors.is_empty(),
        "expected either retry events or a final error"
    );
}
