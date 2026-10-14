//! Replay mode: play the server side of a fixture against a live host.
//!
//! Server (`out`) frames are emitted in recorded order. At each recorded host
//! (`in`) frame, replay blocks for the host's next line and checks its `type`
//! matches, and a dialog answer (and a prompt's text and recorded images,
//! a steer's, follow-up's or subagent command's id and text, a host URI
//! read's content, an opted-in host tool refusal's reason, and an opted-in
//! host tool result's text) matches; the host's fresh request id
//! is mapped onto the recorded one so the recorded `response` frames
//! correlate.
//! Any divergence exits with [`crate::EXIT_DIVERGED`] — the host sees a
//! closed transport, never a hang.

use crate::fixture::{Dir, Placeholders, Record};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    io::{BufRead, Write},
    path::Path,
};

pub(crate) fn run(fixture: &Path, cwd: &Path, placeholders: &Placeholders) -> i32 {
    let records = match load(fixture) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("fake-omp replay: {e}");
            return 2;
        }
    };
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut stdout = std::io::stdout().lock();
    // recorded request id → this run's request id.
    let mut ids: HashMap<String, String> = HashMap::new();
    // Where the host asked `export_html` to write, until OMP's answer.
    let mut export: Option<String> = None;

    for (n, record) in records.iter().enumerate() {
        match record.dir {
            Dir::Fs => {
                let frame = placeholders.expand_value(&record.frame);
                match frame.get("type").and_then(Value::as_str) {
                    Some("fs_hold") => {
                        hold(cwd, &frame);
                        continue;
                    }
                    // A slow OMP: the next frame comes this much later.
                    Some("fs_sleep") => {
                        let ms = frame.get("ms").and_then(Value::as_u64).unwrap_or(0);
                        std::thread::sleep(std::time::Duration::from_millis(ms));
                        continue;
                    }
                    _ => {}
                }
                if let Err(e) = crate::fs_effects::apply(cwd, &frame) {
                    eprintln!("fake-omp replay: record {n}: {e}");
                    return crate::EXIT_DIVERGED;
                }
            }
            Dir::Out => {
                let mut frame = placeholders.expand_value(&record.frame);
                // A prompt's result names the prompt's request id too.
                if matches!(
                    frame.get("type").and_then(Value::as_str),
                    Some("response" | "prompt_result")
                ) {
                    remap_id(&mut frame, &ids);
                }
                if frame.get("command").and_then(Value::as_str) == Some("export_html") {
                    if let Some(path) = export.take() {
                        if let Err(e) = write_export(&path) {
                            eprintln!("fake-omp replay: record {n}: export to {path}: {e}");
                            return crate::EXIT_DIVERGED;
                        }
                        frame["data"] = json!({ "path": path });
                    }
                }
                if writeln!(stdout, "{frame}")
                    .and_then(|()| stdout.flush())
                    .is_err()
                {
                    return 0; // host went away
                }
            }
            Dir::In => {
                let Some(Ok(line)) = lines.next() else {
                    return 0; // host closed stdin early: normal shutdown
                };
                let got: Value = match serde_json::from_str(&line) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("fake-omp replay: host sent non-JSON at record {n}: {e}");
                        return crate::EXIT_DIVERGED;
                    }
                };
                let got_type = got.get("type").and_then(Value::as_str);
                if got_type == Some("export_html") {
                    export = got
                        .get("outputPath")
                        .and_then(Value::as_str)
                        .map(str::to_string);
                }
                if got_type != record.frame_type() {
                    eprintln!(
                        "fake-omp replay: divergence at record {n}: expected {:?}, host sent {:?}",
                        record.frame_type(),
                        got_type
                    );
                    return crate::EXIT_DIVERGED;
                }
                if got_type == Some("extension_ui_response") {
                    let expected = placeholders.expand_value(&record.frame);
                    if got != expected {
                        eprintln!(
                            "fake-omp replay: divergence at record {n}: expected answer {expected}, host sent {got}"
                        );
                        return crate::EXIT_DIVERGED;
                    }
                }
                // A prompt must be the recorded one, not merely a prompt.
                let expected = placeholders.expand_value(&record.frame);
                if got_type == Some("prompt") && got.get("message") != expected.get("message") {
                    eprintln!(
                        "fake-omp replay: divergence at record {n}: expected prompt {:?}, host sent {:?}",
                        expected.get("message"),
                        got.get("message")
                    );
                    return crate::EXIT_DIVERGED;
                }
                // A steer, follow-up, queue removal or subagent command must
                // name the recorded target and text.
                if matches!(
                    got_type,
                    Some(
                        "steer"
                            | "follow_up"
                            | "remove_queued_message"
                            | "steer_subagent"
                            | "cancel_subagent"
                    )
                ) {
                    for field in ["subagentId", "message", "queue"] {
                        if got.get(field) != expected.get(field) {
                            eprintln!(
                                "fake-omp replay: divergence at record {n}: expected {field} {:?}, host sent {:?}",
                                expected.get(field),
                                got.get(field)
                            );
                            return crate::EXIT_DIVERGED;
                        }
                    }
                }
                // A parity-core command must be the recorded one, field by
                // field (its id aside).
                if got_type.is_some_and(|t| WHOLE_FRAME.contains(&t)) {
                    let strip = |v: &Value| {
                        let mut v = v.clone();
                        if let Some(o) = v.as_object_mut() {
                            o.remove("id");
                        }
                        v
                    };
                    if strip(&got) != strip(&expected) {
                        eprintln!(
                            "fake-omp replay: divergence at record {n}: expected {}, host sent {}",
                            strip(&expected),
                            strip(&got)
                        );
                        return crate::EXIT_DIVERGED;
                    }
                }
                // A host URI read must answer what the recording answered.
                let fields: &[&str] = match got_type {
                    Some("host_uri_result") => &["content", "isError", "error"],
                    Some("host_tool_result") => &["isError"],
                    _ => &[],
                };
                for field in fields {
                    if got.get(field) != expected.get(field) {
                        let json = |v: Option<&Value>| v.cloned().unwrap_or(Value::Null);
                        eprintln!(
                            "fake-omp replay: divergence at record {n}: expected {field} {}, host sent {} {:?}",
                            json(expected.get(field)),
                            json(got.get(field)),
                            texts(&got)
                        );
                        return crate::EXIT_DIVERGED;
                    }
                }
                // A host tool call succeeds or fails as recorded. A record
                // that opts in is refused for the same reason (its first
                // line); others keep what OMP echoed when it was recorded.
                if record.check_reason
                    && got_type == Some("host_tool_result")
                    && expected.get("isError") == Some(&json!(true))
                    && first_lines(&got) != first_lines(&expected)
                {
                    eprintln!(
                        "fake-omp replay: divergence at record {n}: expected refusal {:?}, host sent {:?}",
                        first_lines(&expected),
                        first_lines(&got)
                    );
                    return crate::EXIT_DIVERGED;
                }
                // An opted-in host tool result must be the recorded text.
                if record.check_result
                    && got_type == Some("host_tool_result")
                    && texts(&got) != texts(&expected)
                {
                    eprintln!(
                        "fake-omp replay: divergence at record {n}: expected result {:?}, host sent {:?}",
                        texts(&expected),
                        texts(&got)
                    );
                    return crate::EXIT_DIVERGED;
                }
                // Images the recording prompted with must reach OMP too.
                let expected = record.frame.get("images");
                if expected.is_some() && got.get("images") != expected {
                    eprintln!(
                        "fake-omp replay: divergence at record {n}: expected images {expected:?}, host sent {:?}",
                        got.get("images")
                    );
                    return crate::EXIT_DIVERGED;
                }
                if let (Some(rec), Some(now)) = (
                    record.frame.get("id").and_then(Value::as_str),
                    got.get("id").and_then(Value::as_str),
                ) {
                    ids.insert(rec.to_string(), now.to_string());
                }
            }
        }
    }

    // Fixture exhausted: refuse further commands explicitly until EOF.
    for line in lines {
        let Ok(line) = line else { break };
        let Ok(got) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        let Some(id) = got.get("id").and_then(Value::as_str) else {
            continue;
        };
        let reply = json!({
            "type": "response",
            "id": id,
            "command": got.get("type").cloned().unwrap_or(Value::Null),
            "success": false,
            "error": "fake-omp: fixture exhausted",
        });
        if writeln!(stdout, "{reply}")
            .and_then(|()| stdout.flush())
            .is_err()
        {
            break;
        }
    }
    0
}

/// `fs_hold`: a child in this process's group keeps `path` open for writing
/// and ignores SIGTERM, as an OMP slow to exit holds its session until the
/// host's SIGKILL.
fn hold(cwd: &Path, frame: &Value) {
    let Some(path) = frame.get("path").and_then(Value::as_str) else {
        return;
    };
    let spawned = std::process::Command::new("/bin/sh")
        .arg("-c")
        .arg("trap '' TERM; exec 3>>\"$0\"; exec sleep 60")
        .arg(cwd.join(path))
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
    if let Err(e) = spawned {
        eprintln!("fake-omp replay: fs_hold: {e}");
    }
}

/// OMP writes the export where the host asked; fake-omp writes a stand-in
/// page there, so the host finds a file at the path OMP answers.
fn write_export(path: &str) -> std::io::Result<()> {
    let path = Path::new(path);
    if !path.is_absolute() {
        return Err(std::io::Error::other("not an absolute path"));
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(path, "<!doctype html><title>fake-omp export</title>\n")
}

/// The first line of each text item of a host tool result.
fn texts(frame: &Value) -> Vec<&str> {
    frame
        .pointer("/result/content")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|item| item.get("text").and_then(Value::as_str))
        .collect()
}

fn first_lines(frame: &Value) -> Vec<&str> {
    frame
        .pointer("/result/content")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|item| item.get("text").and_then(Value::as_str))
        .map(|text| text.lines().next().unwrap_or(""))
        .collect()
}

/// Commands replay checks whole (S9 U11, ADR-0057 decision 3; S10 U1).
const WHOLE_FRAME: &[&str] = &[
    "get_available_models",
    "set_model",
    "cycle_model",
    "get_available_thinking_levels",
    "set_thinking_level",
    "cycle_thinking_level",
    "abort_and_prompt",
    "promote_queued_message",
    "set_steering_mode",
    "set_follow_up_mode",
    "set_interrupt_mode",
    "compact",
    "set_auto_compaction",
    "set_auto_retry",
    "abort_retry",
    "switch_session",
    "set_session_name",
    "get_messages_page",
    "handoff",
];

fn remap_id(frame: &mut Value, ids: &HashMap<String, String>) {
    let Some(id) = frame.get("id").and_then(Value::as_str) else {
        return;
    };
    if let Some(now) = ids.get(id) {
        frame["id"] = Value::String(now.clone());
    }
}

fn load(path: &Path) -> Result<Vec<Record>, String> {
    let raw = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    raw.lines()
        .filter(|l| !l.trim().is_empty())
        .map(Record::parse)
        .collect()
}
