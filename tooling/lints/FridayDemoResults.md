# `len_in_loop_condition` — Lint Results on Zed Workspace

## What the lint does

It flags `.len()` calls used as loop bounds — in `for` ranges
(`0..v.len()`) and `while` conditions (`while i < v.len()`). Hoisting the
length into a local variable makes the intent clearer and, in `while` loops,
avoids re-evaluating `.len()` on every iteration.

## Command

```
RUSTFLAGS="-A shared_string_from_str_literal \
           -A async_block_without_await \
           -A entity_update_in_render \
           -A notify_in_render \
           -A owned_string_into_shared" \
cargo dylint --path tooling/lints \
  -- --manifest-path /Users/mrg/zed/Cargo.toml --workspace
```

## Results — 23 hits across 11 crates

| # | File | Line | Code |
|---|------|------|------|
| 1 | `crates/zlog/src/zlog.rs` | 244 | `while i + 1 < mod_path_bytes.len()` |
| 2 | `crates/zlog/src/zlog.rs` | 265 | `while i < scopes.len()` |
| 3 | `crates/util/src/shell.rs` | 565 | `while i < chars.len()` |
| 4 | `crates/sum_tree/src/cursor.rs` | 330 | `while entry.index() < child_summaries.len()` |
| 5 | `crates/sum_tree/src/sum_tree.rs` | 276 | `while nodes.len() > 1` |
| 6 | `crates/sum_tree/src/sum_tree.rs` | 346 | `while nodes.len() > 1` |
| 7 | `crates/zeta_prompt/src/multi_region.rs` | 95 | `while i < lines.len()` |
| 8 | `crates/zeta_prompt/src/zeta_prompt.rs` | 2036 | `while offset < model_output.len()` |
| 9 | `crates/zeta_prompt/src/zeta_prompt.rs` | 2061 | `while scan < model_output.len()` |
| 10 | `crates/zeta_prompt/src/zeta_prompt.rs` | 2203 | `while i < original_lines.len()` |
| 11 | `crates/zeta_prompt/src/zeta_prompt.rs` | 3807 | `while suffix_end < old_lines.len()` |
| 12 | `crates/paths/src/paths.rs` | 28 | `while i < APP_NAME.len()` |
| 13 | `crates/text/src/text.rs` | 778 | `while text_offset < visible_text.len()` |
| 14 | `crates/text/src/text.rs` | 1316 | `while text_offset < new_text.len()` |
| 15 | `crates/gpui/src/keymap/context.rs` | 292 | `for i in 0..all_contexts.len()` |
| 16 | `crates/fuzzy/src/matcher.rs` | 159 | `for i in 0..self.query.len()` |
| 17 | `crates/git/src/repository.rs` | 3119 | `while pending_requests.len() < MAX_BATCH_SIZE` |
| 18 | `crates/edit_prediction_metrics/src/kept_rate.rs` | 219 | `while index < tokens.len()` |
| 19 | `crates/edit_prediction_metrics/src/tokenize.rs` | 41 | `while index < characters.len()` |
| 20 | `crates/streaming_diff/src/streaming_diff.rs` | 119 | `for i in 0..=old.len()` |
| 21 | `crates/streaming_diff/src/streaming_diff.rs` | 139 | `for j in self.new_text_ix + 1..=self.new.len()` |
| 22 | `crates/streaming_diff/src/streaming_diff.rs` | 144 | `for i in 1..=self.old.len()` |
| 23 | `crates/streaming_diff/src/streaming_diff.rs` | 166 | `for i in self.old_text_ix..=self.old.len()` |

### By crate

| Crate | Hits |
|-------|------|
| `zeta_prompt` | 5 |
| `streaming_diff` | 4 |
| `sum_tree` | 3 |
| `zlog` | 2 |
| `text` | 2 |
| `edit_prediction_metrics` | 2 |
| `util` | 1 |
| `paths` | 1 |
| `gpui` | 1 |
| `fuzzy` | 1 |
| `git` | 1 |

## Notes

* The `extension` crate failed to compile under `nightly-2026-01-22` due to
  `cfg_select!` being gated behind an unstable feature flag. All other
  workspace crates were checked successfully.
* Hits 5–6 (`sum_tree`) use `while nodes.len() > 1` where the collection
  *is* mutated in the loop body (nodes are drained), so hoisting would
  change semantics. These are likely false positives worth suppressing.
* Hit 17 (`git`) compares `pending_requests.len()` against a constant cap
  while the collection grows inside the loop — another case where mutation
  makes hoisting incorrect.
