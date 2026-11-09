//! A deterministic, very long agent thread in the format the native agent persists.

use serde_json::{Value, json};

use crate::{MARKDOWN_FILE, PYTHON_FILE, RUST_FILE, TYPESCRIPT_FILE, pinned_contents};

/// Conversation turns in the generated thread. Each turn renders as eight thread
/// entries: the user message, three assistant text blocks, a search and a read tool
/// call, an `edit_file` tool call with a diff, and a long closing reply.
pub const AGENT_THREAD_TURNS: usize = 125;
pub const AGENT_THREAD_ENTRIES_PER_TURN: usize = 8;

fn lines(path: &str, start: usize, count: usize) -> String {
    let contents = pinned_contents(path);
    let line_count = contents.lines().count();
    let start = start % line_count.saturating_sub(count).max(1);
    contents
        .lines()
        .skip(start)
        .take(count)
        .collect::<Vec<_>>()
        .join("\n")
}

fn fenced(language: &str, code: &str) -> String {
    format!("```{language}\n{code}\n```")
}

fn tool_use(id: &str, name: &str, input: Value) -> Value {
    json!({
        "ToolUse": {
            "id": id,
            "name": name,
            "raw_input": input.to_string(),
            "input": { "type": "json", "value": input },
            "is_input_complete": true,
            "thought_signature": null
        }
    })
}

fn tool_result(id: &str, name: &str, text: &str, output: Option<Value>) -> Value {
    json!({
        "tool_use_id": id,
        "tool_name": name,
        "is_error": false,
        "content": [{ "Text": text }],
        "output": output
    })
}

fn user_message(turn: usize) -> Value {
    let request = match turn % 4 {
        0 => format!(
            "Can you look at how the editor handles scrolling around line {} and tidy it up? \
             Keep the public API the same.",
            turn * 97
        ),
        1 => format!(
            "The build is failing after the last change. Here's the error:\n\n{}",
            fenced(
                "text",
                &format!(
                    "error[E0308]: mismatched types\n  --> crates/editor/src/editor.rs:{}:17\n   |\n   = note: expected `Option<usize>`, found `usize`",
                    turn * 31
                )
            )
        ),
        2 => "Please also update the TypeScript helper and the Python script so they agree \
              with the new behavior, and explain the settings involved."
            .to_string(),
        _ => format!(
            "Thanks. Summarize what changed in turn {turn}, then continue with the next item."
        ),
    };
    json!({
        "User": {
            "id": format!("bench-user-message-{turn}"),
            "content": [{ "Text": request }]
        }
    })
}

fn long_reply(turn: usize) -> String {
    let rust = lines(RUST_FILE, turn * 211, 40 + turn % 30);
    let typescript = lines(TYPESCRIPT_FILE, turn * 13, 24);
    let python = lines(PYTHON_FILE, turn * 53, 30);
    let prose = lines(MARKDOWN_FILE, turn * 61, 18);
    format!(
        "## Summary of turn {turn}\n\n\
         I made the change in three places. The core of it is in the editor:\n\n{}\n\n\
         The helper needed a matching update:\n\n{}\n\n\
         And the script now does the same thing:\n\n{}\n\n\
         ### Settings\n\n{prose}\n\n\
         | Area | Files | Risk |\n|---|---|---|\n| Editor | `editor.rs` | **medium** |\n\
         | Web | `dangerfile.ts` | low |\n| Scripts | `check_new_issue_for_duplicates.py` | low |\n\n\
         - Run `cargo test -p editor` to check the change.\n\
         - The behavior is unchanged when `scroll_beyond_last_line` is `off`.\n\
         - Follow-ups are listed below.\n\n\
         1. Revisit the autoscroll strategy.\n2. Add a regression test for line {}.\n",
        fenced("rust", &rust),
        fenced("typescript", &typescript),
        fenced("python", &python),
        turn * 97,
    )
}

fn agent_message(turn: usize, project_root: &std::path::Path) -> Value {
    let search_id = format!("toolu_search_{turn}");
    let read_id = format!("toolu_read_{turn}");
    let edit_id = format!("toolu_edit_{turn}");

    let read_excerpt = lines(RUST_FILE, turn * 173, 60);
    let old_text = lines(RUST_FILE, turn * 331, 24);
    let new_text = old_text
        .replace("cx", "context")
        .replace("fn ", "pub(crate) fn ");
    let edited_path = project_root.join(RUST_FILE);
    let edit_input = json!({
        "path": RUST_FILE,
        "edits": [{ "old_text": old_text, "new_text": new_text }]
    });
    let edit_output = json!({
        "input_path": edited_path,
        "old_text": old_text,
        "new_text": new_text,
        "diff": ""
    });

    let search_results = (0..12)
        .map(|index| {
            format!(
                "crates/editor/src/editor.rs:{}: {}",
                turn * 17 + index * 101,
                lines(RUST_FILE, turn * 17 + index * 101, 1).trim()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    json!({
        "Agent": {
            "content": [
                { "Text": format!(
                    "I'll start by finding where this is handled.\n\nLooking for `scroll_position` \
                     and related helpers in turn {turn}."
                ) },
                tool_use(&search_id, "grep", json!({ "regex": "scroll_position", "include_pattern": "**/*.rs" })),
                { "Text": format!(
                    "Found {} matches. The relevant code is in `editor.rs`; let me read it.",
                    12
                ) },
                tool_use(&read_id, "read_file", json!({ "path": RUST_FILE, "start_line": turn * 173, "end_line": turn * 173 + 60 })),
                { "Text": format!(
                    "This is the function to change:\n\n{}\n\nI'll make the edit now.",
                    fenced("rust", &lines(RUST_FILE, turn * 173, 16))
                ) },
                tool_use(&edit_id, "edit_file", edit_input),
                { "Text": long_reply(turn) }
            ],
            "tool_results": {
                search_id.clone(): tool_result(&search_id, "grep", &search_results, None),
                read_id.clone(): tool_result(&read_id, "read_file", &read_excerpt, None),
                edit_id.clone(): tool_result(&edit_id, "edit_file", "Edited crates/editor/src/editor.rs", Some(edit_output))
            },
            "reasoning_details": null
        }
    })
}

/// The thread as `DbThread` JSON (format version 0.3.0), referencing files in
/// `project_root`.
pub fn agent_thread_json(project_root: &std::path::Path) -> Vec<u8> {
    let mut messages = Vec::with_capacity(AGENT_THREAD_TURNS * 2);
    for turn in 0..AGENT_THREAD_TURNS {
        messages.push(user_message(turn));
        messages.push(agent_message(turn, project_root));
    }
    serde_json::to_vec(&json!({
        "version": "0.3.0",
        "title": "Tidy up editor scrolling",
        "messages": messages,
        "updated_at": "2026-01-01T00:00:00Z"
    }))
    .expect("thread JSON serializes")
}
