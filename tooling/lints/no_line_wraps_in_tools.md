# `wrapped_doc_comment` results on `crates/agent/src/tools`

Run of the new `wrapped_doc_comment` lint, scoped to the `agent` crate, with all
other lints suppressed. Counts cover only paths under `crates/agent/src/tools`.

## How to reproduce

```
RUSTFLAGS="-A shared_string_from_str_literal -A async_block_without_await \
          -A entity_update_in_render -A len_in_loop_condition \
          -A notify_in_render -A owned_string_into_shared" \
cargo dylint --path tooling/lints -- -p agent --lib
```

## Heuristic

A doc-comment paragraph is a contiguous run of non-empty `///` lines. A line is
a paragraph boundary when it is blank, starts with a list/heading/quote/table
marker (`-`, `*`, `+`, `#`, `>`, `|`), starts with an HTML/XML tag (`<…>`),
opens a fenced code block (` ``` `), or starts a numbered list item (`1.`,
`2)`, …). Inside a paragraph, the second physical line is the one flagged.

## Total

73 warnings under `crates/agent/src/tools`.

## Per file

| count | file |
|------:|------|
| 18 | `crates/agent/src/tools/tool_permissions.rs` |
| 8  | `crates/agent/src/tools/edit_file_tool/streaming_parser.rs` |
| 5  | `crates/agent/src/tools/edit_file_tool.rs` |
| 5  | `crates/agent/src/tools/symbol_locator.rs` |
| 4  | `crates/agent/src/tools/diagnostics_tool.rs` |
| 4  | `crates/agent/src/tools/edit_file_tool/streaming_fuzzy_matcher.rs` |
| 3  | `crates/agent/src/tools/apply_code_action_tool.rs` |
| 3  | `crates/agent/src/tools/copy_path_tool.rs` |
| 3  | `crates/agent/src/tools/get_code_actions_tool.rs` |
| 3  | `crates/agent/src/tools/grep_tool.rs` |
| 2  | `crates/agent/src/tools/edit_file_tool/reindent.rs` |
| 2  | `crates/agent/src/tools/find_references_tool.rs` |
| 2  | `crates/agent/src/tools/go_to_definition_tool.rs` |
| 2  | `crates/agent/src/tools/move_path_tool.rs` |
| 2  | `crates/agent/src/tools/read_file_tool.rs` |
| 2  | `crates/agent/src/tools/rename_tool.rs` |
| 1  | `crates/agent/src/tools.rs` |
| 1  | `crates/agent/src/tools/find_path_tool.rs` |
| 1  | `crates/agent/src/tools/now_tool.rs` |
| 1  | `crates/agent/src/tools/save_file_tool.rs` |
| 1  | `crates/agent/src/tools/web_search_tool.rs` |

## All flagged sites

Each row is the `file:line:column` of the wrap-continuation line, i.e. the
second (or later) physical line of a wrapped doc-comment paragraph.

```
crates/agent/src/tools.rs:39:1
crates/agent/src/tools/apply_code_action_tool.rs:16:1
crates/agent/src/tools/apply_code_action_tool.rs:19:1
crates/agent/src/tools/apply_code_action_tool.rs:23:5
crates/agent/src/tools/copy_path_tool.rs:22:1
crates/agent/src/tools/copy_path_tool.rs:25:1
crates/agent/src/tools/copy_path_tool.rs:29:5
crates/agent/src/tools/diagnostics_tool.rs:19:1
crates/agent/src/tools/diagnostics_tool.rs:23:1
crates/agent/src/tools/diagnostics_tool.rs:28:1
crates/agent/src/tools/diagnostics_tool.rs:40:5
crates/agent/src/tools/edit_file_tool.rs:78:5
crates/agent/src/tools/edit_file_tool.rs:83:5
crates/agent/src/tools/edit_file_tool.rs:100:1
crates/agent/src/tools/edit_file_tool.rs:105:5
crates/agent/src/tools/edit_file_tool.rs:1081:1
crates/agent/src/tools/edit_file_tool/reindent.rs:35:1
crates/agent/src/tools/edit_file_tool/reindent.rs:52:5
crates/agent/src/tools/edit_file_tool/streaming_fuzzy_matcher.rs:9:1
crates/agent/src/tools/edit_file_tool/streaming_fuzzy_matcher.rs:40:5
crates/agent/src/tools/edit_file_tool/streaming_fuzzy_matcher.rs:45:5
crates/agent/src/tools/edit_file_tool/streaming_fuzzy_matcher.rs:71:5
crates/agent/src/tools/edit_file_tool/streaming_parser.rs:41:1
crates/agent/src/tools/edit_file_tool/streaming_parser.rs:47:1
crates/agent/src/tools/edit_file_tool/streaming_parser.rs:63:5
crates/agent/src/tools/edit_file_tool/streaming_parser.rs:133:5
crates/agent/src/tools/edit_file_tool/streaming_parser.rs:148:5
crates/agent/src/tools/edit_file_tool/streaming_parser.rs:152:5
crates/agent/src/tools/edit_file_tool/streaming_parser.rs:211:5
crates/agent/src/tools/edit_file_tool/streaming_parser.rs:250:1
crates/agent/src/tools/find_path_tool.rs:36:5
crates/agent/src/tools/find_references_tool.rs:15:1
crates/agent/src/tools/find_references_tool.rs:18:1
crates/agent/src/tools/get_code_actions_tool.rs:16:1
crates/agent/src/tools/get_code_actions_tool.rs:19:1
crates/agent/src/tools/get_code_actions_tool.rs:22:1
crates/agent/src/tools/go_to_definition_tool.rs:15:1
crates/agent/src/tools/go_to_definition_tool.rs:18:1
crates/agent/src/tools/grep_tool.rs:35:5
crates/agent/src/tools/grep_tool.rs:47:5
crates/agent/src/tools/grep_tool.rs:52:5
crates/agent/src/tools/move_path_tool.rs:41:5
crates/agent/src/tools/move_path_tool.rs:45:5
crates/agent/src/tools/now_tool.rs:23:1
crates/agent/src/tools/read_file_tool.rs:31:1
crates/agent/src/tools/read_file_tool.rs:47:5
crates/agent/src/tools/rename_tool.rs:16:1
crates/agent/src/tools/rename_tool.rs:20:1
crates/agent/src/tools/save_file_tool.rs:27:1
crates/agent/src/tools/symbol_locator.rs:15:1
crates/agent/src/tools/symbol_locator.rs:19:5
crates/agent/src/tools/symbol_locator.rs:23:5
crates/agent/src/tools/symbol_locator.rs:108:1
crates/agent/src/tools/symbol_locator.rs:145:5
crates/agent/src/tools/tool_permissions.rs:27:5
crates/agent/src/tools/tool_permissions.rs:37:1
crates/agent/src/tools/tool_permissions.rs:63:1
crates/agent/src/tools/tool_permissions.rs:66:1
crates/agent/src/tools/tool_permissions.rs:70:1
crates/agent/src/tools/tool_permissions.rs:105:1
crates/agent/src/tools/tool_permissions.rs:134:1
crates/agent/src/tools/tool_permissions.rs:138:1
crates/agent/src/tools/tool_permissions.rs:147:1
crates/agent/src/tools/tool_permissions.rs:233:1
crates/agent/src/tools/tool_permissions.rs:276:1
crates/agent/src/tools/tool_permissions.rs:279:1
crates/agent/src/tools/tool_permissions.rs:312:1
crates/agent/src/tools/tool_permissions.rs:326:1
crates/agent/src/tools/tool_permissions.rs:342:1
crates/agent/src/tools/tool_permissions.rs:346:1
crates/agent/src/tools/tool_permissions.rs:369:1
crates/agent/src/tools/tool_permissions.rs:376:1
crates/agent/src/tools/web_search_tool.rs:19:1
```

## Spot-checked cases

The lint reproduces the canonical pattern from
[zed-industries/zed#56164](https://github.com/zed-industries/zed/pull/56164).
For example, in `crates/agent/src/tools/rename_tool.rs`:

```rust
/// Renames a symbol across the project using the language server.
///
/// This performs a semantic rename, updating all references to the symbol
/// across all files in the project. The language server determines which
/// occurrences to rename based on the symbol's type and scope.
///
/// Before using this tool, use read_file or grep to find the exact symbol
/// name and line number.
```

Lines 16 and 20 are flagged: the two paragraph wrap-continuations that PR
#56164 collapses to single lines.

In `crates/agent/src/tools/apply_code_action_tool.rs`:

```rust
/// Applies a code action previously retrieved by get_code_actions.
///
/// You must call get_code_actions first to get the list of available actions,
/// then use the number from that list to choose which action to apply.
///
/// After applying a code action, the list is cleared. If you want to apply
/// another action, call get_code_actions again.
```

Lines 16 and 19 are flagged for the two body paragraphs.

In `crates/agent/src/tools/edit_file_tool/reindent.rs`:

```rust
/// Synchronous re-indentation adapter. Buffers incomplete lines and applies
/// an `IndentDelta` to each line's leading whitespace before emitting it.
```

Line 35 (a non-tool helper) is also flagged. The lint does not distinguish
between tool-input doc comments and internal helper doc comments; it flags
any wrapped `///` paragraph in the scanned crate.

## Notes

- `tool_permissions.rs` accounts for 18 of the 73 hits. Most of those are on
  internal helpers rather than tool inputs, so reviewers may choose to leave
  them alone.
- The lint flags paragraph wraps; it does not propose a fix and is not
  machine-applicable, in keeping with `LintRULES.md` rules 7 and 8.
