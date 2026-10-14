//! Record mode: transparent proxy to the real OMP, teeing both directions.

use crate::fixture::{Dir, Placeholders, Record};
use serde_json::Value;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};

pub(crate) fn run(
    real: &Path,
    args: &[String],
    out: &Path,
    cwd: &Path,
    placeholders: &Placeholders,
) -> i32 {
    let file = match File::create(out) {
        Ok(f) => Arc::new(Mutex::new(f)),
        Err(e) => {
            eprintln!("fake-omp record: cannot create {}: {e}", out.display());
            return 2;
        }
    };
    let mut child = match Command::new(real)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("fake-omp record: cannot spawn {}: {e}", real.display());
            return 2;
        }
    };
    let (Some(mut child_in), Some(child_out)) = (child.stdin.take(), child.stdout.take()) else {
        eprintln!("fake-omp record: child pipes missing");
        return 2;
    };

    let tee = {
        let placeholders = placeholders.clone();
        move |file: &Mutex<File>, dir: Dir, line: &str| {
            let Ok(frame) = serde_json::from_str::<Value>(line) else {
                eprintln!("fake-omp record: non-JSON line skipped");
                return;
            };
            if frame.get("type").and_then(Value::as_str) == Some("rpc_chunk") {
                eprintln!("fake-omp record: rpc_chunk recorded raw (payload not redacted)");
            }
            let record = Record {
                dir,
                frame: placeholders.redact_value(&frame),
                check_reason: false,
                check_result: false,
            };
            if let Ok(mut f) = file.lock() {
                let _ = writeln!(f, "{}", record.to_line());
                let _ = f.flush();
            }
        }
    };

    // Host → OMP. Ends when the host closes stdin; detached otherwise.
    {
        let file = Arc::clone(&file);
        let tee = tee.clone();
        std::thread::spawn(move || {
            for line in std::io::stdin().lock().lines() {
                let Ok(line) = line else { break };
                tee(&file, Dir::In, &line);
                if writeln!(child_in, "{line}")
                    .and_then(|()| child_in.flush())
                    .is_err()
                {
                    break;
                }
            }
        });
    }

    // OMP → host, until OMP closes stdout. Around each tool call, the files
    // that tool wrote are recorded as `fs` frames just before its
    // `tool_execution_end` (see `fs_effects`). A tool result that names the
    // whole file it wrote (`edit`, `write`, `ast_edit`) is recorded from the
    // result itself: the snapshot can miss a write that lands outside its
    // window, and the two must not both record the same file.
    let mut stdout = std::io::stdout().lock();
    let mut files = crate::fs_effects::snapshot(cwd);
    for line in BufReader::new(child_out).lines() {
        let Ok(line) = line else { break };
        let kind = serde_json::from_str::<Value>(&line)
            .ok()
            .and_then(|v| v.get("type").and_then(Value::as_str).map(str::to_string));
        match kind.as_deref() {
            Some("tool_execution_start") => files = crate::fs_effects::snapshot(cwd),
            Some("tool_execution_end") => {
                let declared = writes_of(&line, cwd);
                let paths: Vec<&str> = declared
                    .iter()
                    .filter_map(|frame| frame.get("path").and_then(Value::as_str))
                    .collect();
                for frame in &declared {
                    tee(&file, Dir::Fs, &frame.to_string());
                }
                let now = crate::fs_effects::snapshot(cwd);
                for frame in crate::fs_effects::diff(&files, &now) {
                    let named = frame
                        .get("path")
                        .and_then(Value::as_str)
                        .is_some_and(|path| paths.contains(&path));
                    if !named {
                        tee(&file, Dir::Fs, &frame.to_string());
                    }
                }
                files = now;
            }
            _ => {}
        }
        tee(&file, Dir::Out, &line);
        if writeln!(stdout, "{line}")
            .and_then(|()| stdout.flush())
            .is_err()
        {
            break;
        }
    }
    child.wait().ok().and_then(|s| s.code()).unwrap_or(1)
}

/// The file writes one tool result stands for: `edit`, `write` and `ast_edit`
/// report the whole file after the call, per file or in `perFileResults`.
/// A fixture replays frames, and OMP wrote the file itself while recording,
/// so a replayed fixture needs these written where the result is. Paths are
/// relative to the workspace, as the snapshot's are: the replay applies them
/// under its own root and refuses anything that reaches outside it.
fn writes_of(line: &str, cwd: &Path) -> Vec<Value> {
    let Ok(frame) = serde_json::from_str::<Value>(line) else {
        return Vec::new();
    };
    if frame.get("type").and_then(Value::as_str) != Some("tool_execution_end") {
        return Vec::new();
    }
    if !matches!(
        frame.get("toolName").and_then(Value::as_str),
        Some("edit" | "write" | "ast_edit")
    ) {
        return Vec::new();
    }
    let Some(details) = frame.pointer("/result/details") else {
        return Vec::new();
    };
    let files: Vec<&Value> = match details.get("perFileResults").and_then(Value::as_array) {
        Some(per_file) if !per_file.is_empty() => per_file.iter().collect(),
        _ => vec![details],
    };
    files
        .into_iter()
        .filter_map(|file| {
            let path = file.get("path").and_then(Value::as_str)?;
            let content = file.get("newText").and_then(Value::as_str)?;
            let rel = match Path::new(path).strip_prefix(cwd) {
                Ok(inside) => inside.to_path_buf(),
                Err(_) if Path::new(path).is_relative() => PathBuf::from(path),
                Err(_) => return None,
            };
            Some(
                serde_json::json!({"type": "fs_write", "path": rel.to_string_lossy(), "content": content}),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt as _;
    use std::path::PathBuf;

    /// A stub OMP that writes `$1` between a tool call's frames. The frames
    /// go through `printf '%s\n'`, which does not read the backslashes in
    /// the result's `oldText`/`newText` as escapes.
    fn stub(dir: &Path) -> PathBuf {
        let path = dir.join("stub-omp");
        std::fs::write(
            &path,
            "#!/bin/sh\n\
             printf '%s\\n' '{\"type\":\"tool_execution_start\",\"toolCallId\":\"c1\",\"toolName\":\"edit\",\"args\":{\"input\":\"x\"}}'\n\
             printf 'alpha\\nBETA\\ngamma\\n' > \"$1\"\n\
             printf '%s\\n' '{\"type\":\"tool_execution_end\",\"toolCallId\":\"c1\",\"toolName\":\"edit\",\"result\":{\"content\":[{\"type\":\"text\",\"text\":\"[notes.txt]\"}],\"details\":{\"path\":\"notes.txt\",\"oldText\":\"alpha\\nbeta\\ngamma\\n\",\"newText\":\"alpha\\nBETA\\ngamma\\n\"}}}'\n",
        )
        .unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    #[test]
    fn the_write_a_tool_result_stands_for_is_recorded_once() {
        let root = std::env::temp_dir().join(format!("cedian-record-fs-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let ws = root.join("ws");
        std::fs::create_dir_all(&ws).unwrap();
        let notes = ws.join("notes.txt");
        std::fs::write(&notes, "alpha\nbeta\ngamma\n").unwrap();
        let out = root.join("recorded.jsonl");
        let placeholders = Placeholders::new(&root.join("sessions"), &ws, None);
        let real = stub(&root);
        let code = run(
            &real,
            &[notes.display().to_string()],
            &out,
            &ws,
            &placeholders,
        );
        assert_eq!(code, 0);
        let recorded = std::fs::read_to_string(&out).unwrap();
        let writes: Vec<&str> = recorded
            .lines()
            .filter(|l| l.contains("\"type\":\"fs_write\""))
            .collect();
        assert_eq!(
            writes.len(),
            1,
            "the write is recorded once, from the result: {recorded}"
        );
        assert!(writes[0].contains("alpha\\nBETA\\ngamma\\n"), "{recorded}");
        assert!(
            writes[0].contains("\"path\":\"notes.txt\""),
            "relative, as the replay applies it: {recorded}"
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn a_result_that_wrote_nothing_declares_nothing() {
        let cwd = Path::new("/ws");
        let end = |tool: &str, details: Value| {
            serde_json::json!({
                "type": "tool_execution_end",
                "toolName": tool,
                "result": {"details": details},
            })
            .to_string()
        };
        assert!(writes_of(&end("read", serde_json::json!({"path": "/ws/a.txt"})), cwd).is_empty());
        assert!(
            writes_of(&end("edit", serde_json::json!({"path": "/ws/a.txt"})), cwd).is_empty(),
            "no newText, nothing to write"
        );
        let per_file = end(
            "write",
            serde_json::json!({"perFileResults": [
                {"path": "/ws/a.txt", "newText": "one\n"},
                {"path": "/elsewhere/b.txt", "newText": "two\n"}
            ]}),
        );
        let frames = writes_of(&per_file, cwd);
        assert_eq!(
            frames.len(),
            1,
            "only the file inside the workspace: {frames:?}"
        );
        assert_eq!(
            frames[0]["path"], "a.txt",
            "relative to the workspace, as the snapshot's frames are"
        );
        assert_eq!(frames[0]["content"], "one\n");
        assert!(writes_of("{\"type\":\"tool_execution_start\"}", cwd).is_empty());
        assert!(writes_of("not json", cwd).is_empty());
    }
}
