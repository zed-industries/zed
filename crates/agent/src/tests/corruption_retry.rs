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
/// The fake model sends text that triggers both the repetition and
/// script-switching detectors (periodic Latin + Han + Cyrillic output).
/// After `MAX_CORRUPTION_RETRY_ATTEMPTS` retries the error propagates
/// because no fallback model is configured.
#[gpui::test]
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

    // Periodic text mixing Latin, Han, and Cyrillic scripts.
    // Triggers both the repetition detector (period ≈ 62 bytes, ≥3 full
    // periods in the 512-byte scan window) and the script-switching
    // detector (3 distinct scripts with ≥5 transitions).
    let corrupted_text = "Hello世界Приветworld你好мирtest测试Привет".repeat(20);

    // Each round sends one chunk of corrupted text. The output-quality
    // scorer detects corruption, short-circuits the stream, and the
    // corruption retry path runs. After MAX_CORRUPTION_RETRY_ATTEMPTS
    // retries the error propagates (no fallback configured).
    for _ in 0..=MAX_CORRUPTION_RETRY_ATTEMPTS {
        fake_model.send_last_completion_stream_text_chunk(&corrupted_text);
        fake_model.send_last_completion_stream_event(
            LanguageModelCompletionEvent::Stop(StopReason::EndTurn),
        );
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

    assert_eq!(
        retry_events.len(),
        MAX_CORRUPTION_RETRY_ATTEMPTS as usize,
        "expected {MAX_CORRUPTION_RETRY_ATTEMPTS} corruption retries, got {}",
        retry_events.len()
    );

    // Each retry should report the correct attempt number and max.
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

    // After retries are exhausted the error should propagate.
    assert_eq!(errors.len(), 1, "expected 1 error after retries exhausted");
    let error_msg = errors[0].to_string();
    assert!(
        error_msg.contains("corruption"),
        "error should mention corruption, got: {error_msg}"
    );
}

/// Tests that after `MAX_CORRUPTION_RETRY_ATTEMPTS` corruption retries are
/// exhausted, a fallback model is attempted (if the current model has
/// `refusal_fallback_model_id` configured), or the error propagates otherwise.
///
/// Still marked `#[ignore]` because `FakeLanguageModel` does not yet implement
/// `refusal_fallback_model_id`. Once the fake provider supports a fallback
/// model ID, this test can be enabled to verify:
/// - The fallback model is selected via `set_model`
/// - A final `ThreadEvent::Retry` with `meta` containing the fallback name
/// - On subsequent attempts, the fallback model receives the completion request
#[gpui::test]
#[ignore = "FakeLanguageModel lacks refusal_fallback_model_id support"]
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

    let corrupted_text = "Hello世界Приветworld你好мирtest测试Привет".repeat(20);

    for _ in 0..=MAX_CORRUPTION_RETRY_ATTEMPTS + 1 {
        fake_model.send_last_completion_stream_text_chunk(&corrupted_text);
        fake_model.send_last_completion_stream_event(
            LanguageModelCompletionEvent::Stop(StopReason::EndTurn),
        );
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

    assert!(
        !retry_events.is_empty() || !errors.is_empty(),
        "expected either retry events or a final error"
    );
}

/// Verifies that well-formed, task-relevant output does NOT trigger the
/// output-quality corruption detectors (no false positives).
#[gpui::test]
async fn test_clean_output_does_not_trigger_corruption(cx: &mut TestAppContext) {
    let ThreadTest { thread, model, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    thread.update(cx, |thread, _cx| {
        thread.add_tool(EchoTool);
    });

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.send(
                UserMessageId::new(),
                ["Fix the authentication bug in login.rs"],
                cx,
            )
        })
        .unwrap();
    cx.run_until_parked();

    // A long, well-formed response mentioning task-relevant terms.
    let clean_text = "\
        I've analyzed the authentication function in login.rs and found \
        the bug. The session token expiration was being set to zero seconds \
        which caused immediate expiry. I've updated the token creation \
        logic to use a proper expiration duration of thirty minutes. The \
        authentication handler now correctly validates the session tokens \
        and the login flow should work as expected after this fix. Let me \
        call the completion tool to finish.\
    ";

    fake_model.send_last_completion_stream_text_chunk(clean_text);

    // Model calls attempt_completion to end properly.
    fake_model.send_last_completion_stream_event(
        LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
            id: "ac_1".into(),
            name: "attempt_completion".into(),
            raw_input: "{}".to_string(),
            input: serde_json::json!({}),
            is_input_complete: true,
            thought_signature: None,
        }),
    );
    fake_model.send_last_completion_stream_event(
        LanguageModelCompletionEvent::Stop(StopReason::EndTurn),
    );
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // After tool processing, provide a clean follow-up that ends the turn.
    fake_model.send_last_completion_stream_text_chunk("Done");
    fake_model.send_last_completion_stream_event(
        LanguageModelCompletionEvent::Stop(StopReason::EndTurn),
    );
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
        "clean task-relevant output should not trigger any corruption retries"
    );
}
