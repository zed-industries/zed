# TESTING_MEMORY_AND_HISTORY.md

Comprehensive test suite specification for validating the behavior and robustness of the `memory` and `list_history` tools via an LLM. This document is intended to be executed mentally or by a harness that:
1. Preloads a deterministic conversation (seed history).
2. Executes tool calls in a defined order.
3. Asserts structural and semantic correctness of responses and state transitions.

---

## 1. Scope & Goals

Covers:
- All `memory` operations: store, load, list, prune, restore.
- Every argument of `memory` (including edge combinations).
- All `list_history` arguments and behaviors (pagination, truncation, full markdown inclusion).
- State transition validation (before / after store, prune, restore).
- Negative and edge cases (empty summaries, overlapping prune indices, invalid handles).
- Repeatability guidelines and acceptance criteria patterns.

Not Covered (out of scope):
- Underlying persistence engine durability guarantees.
- Performance, latency benchmarks.
- Cross-session memory carryover.

---

## 2. Seed Conversation History (Precondition)

The following conversation MUST exist (indexes are implicit in chronological order). Message roles (User / Assistant / System) are included so an evaluator can map them if the environment categorizes messages.

Seed messages (indexes 0..14):

| Idx | Role      | Content (abridged description) |
|-----|-----------|--------------------------------|
| 0   | System    | "You are a coding assistant specializing in Rust and clarity." |
| 1   | User      | "Hi, I want help building a GPUI view. Prefer no unwraps." |
| 2   | Assistant | Provides GPUI intro, emphasizes error propagation with '?' |
| 3   | User      | "Track my preferences: Rust clarity, avoid panics, descriptive names." |
| 4   | Assistant | Acknowledges preferences; suggests entity patterns. |
| 5   | User      | "What about concurrency? Spawn vs background_spawn examples please." |
| 6   | Assistant | Explains spawn on foreground vs background_spawn for heavy work. |
| 7   | User      | "Add: I also prefer minimal allocations. Include that in memory." |
| 8   | Assistant | Confirms adding 'minimal allocations' to preferences. |
| 9   | User      | "Show an entity update pattern with shadow cloning." |
| 10  | Assistant | Provides example using variable shadowing. |
| 11  | User      | "Later I might switch to performance focus." |
| 12  | Assistant | Notes potential future shift to performance. |
| 13  | User      | "Summarize my current constraints." |
| 14  | Assistant | Summarizes: clarity > performance (for now), no unwraps, minimal allocations, descriptive names. |

All tests assume this conversation is present when `list_history` initial calls occur unless a test explicitly manipulates indexes via pruning operations (which do NOT actually change the historic transcript, only memory summaries).

---

## 3. Tool Argument Reference

### 3.1 memory

| Argument                 | Type       | Required | Notes |
|--------------------------|------------|---------|-------|
| operation                | enum       | Yes     | store | load | list | restore | prune |
| summary                  | string     | store   | Required when operation=store (non-empty recommended) |
| memory_handle            | string     | prune/restore optional | Identifier of previously stored entry |
| auto                     | bool       | Optional| Hint for auto summarization (if supported) |
| start_index              | int/null   | Optional| For range-based prune (inclusive) |
| end_index                | int/null   | Optional| For range-based prune (exclusive or inclusive depending on implementation; assume exclusive if unspecified) |
| restore_insert_index     | int/null   | Optional| Target position when restoring |
| max_preview_chars        | int        | Optional| Affects list preview truncation |
| remove_placeholder       | bool       | Optional| When restoring after placeholder usage |
| replace_placeholder_with | string     | Optional| Replacement text for placeholder |
| summary (empty)          | string     | Bad case| Should yield validation or no-op test |
| auto + summary           | both       | Allowed | Tests precedence rules |

### 3.2 list_history

| Argument              | Type   | Required | Effect |
|-----------------------|--------|---------|--------|
| start                 | int    | No      | Default 0 |
| limit                 | int    | No      | Default 40 |
| max_chars_per_message | int    | No      | Truncation of each message |
| include_full_markdown | bool   | No      | Append full text after table-style listing |

---

## 4. Test Case Matrix (High-Level)

| ID  | Category          | Purpose |
|-----|-------------------|---------|
| H01 | list_history basic | Baseline retrieval |
| H02 | list_history pagination | Non-zero start offset |
| H03 | list_history truncation | Verify `max_chars_per_message` |
| H04 | list_history full markdown | Verify full markdown inclusion |
| M01 | memory store basic | Store first canonical summary |
| M02 | memory list after store | Confirm handle & truncated preview |
| M03 | memory load basic | Retrieve stored summary |
| M04 | memory store second | Store an updated/refined summary |
| M05 | memory list multiple | Confirm ordering and multiple handles |
| M06 | memory prune by handle | Remove one summary by handle |
| M07 | memory restore by handle | Restore pruned summary at default position |
| M08 | memory prune by index range | Range removal when multiple entries exist |
| M09 | memory restore with insert index | Insert at explicit position |
| M10 | memory store with auto=true | Validate coexistence of manual + auto |
| M11 | memory list with max_preview_chars | Request small preview size |
| M12 | memory prune non-existent handle | Negative: expect error or empty effect |
| M13 | memory store empty summary | Negative: expect rejection |
| M14 | memory restore with placeholder replacement | Validate replace & remove flags |
| M15 | memory prune partial indices overlapping | Edge overlap behavior |
| M16 | memory load after sequence | Final integrity snapshot |

---

## 5. Detailed Test Specifications

Each test includes: Preconditions, Invocation, Expected Structural Output Pattern, Postconditions.

### H01: list_history basic
Preconditions: Seed conversation loaded.
Invocation:
```/dev/null/h01_list_history.json#L1-6
{
  "name": "list_history",
  "arguments": {
    "limit": 5
  }
}
```
Assertions:
- Returns 5 messages (indexes 0..4).
- Column metadata includes indices stabilizing future pagination.
Postconditions: None.

### H02: list_history pagination
Invocation:
```/dev/null/h02_list_history.json#L1-7
{
  "name": "list_history",
  "arguments": {
    "start": 5,
    "limit": 4
  }
}
```
Assertions:
- Messages 5..8 included.
- Indices match seed mapping.

### H03: list_history truncation
Invocation:
```/dev/null/h03_list_history.json#L1-8
{
  "name": "list_history",
  "arguments": {
    "limit": 3,
    "max_chars_per_message": 30
  }
}
```
Assertions:
- Each preview length ≤ 30 chars (no mid-grapheme split if system supports).
- Ellipsis or truncation indicator allowed.

### H04: list_history full markdown
Invocation:
```/dev/null/h04_list_history.json#L1-7
{
  "name": "list_history",
  "arguments": {
    "limit": 2,
    "include_full_markdown": true
  }
}
```
Assertions:
- Table or structured list first, followed by full raw content for those 2 messages.

---

### M01: memory store basic
Preconditions: No memory entries yet.
Invocation:
```/dev/null/m01_memory_store.json#L1-9
{
  "name": "memory",
  "arguments": {
    "operation": "store",
    "summary": "User prefers: Rust clarity > performance for now; avoid unwrap; descriptive names; minimal allocations emerging."
  }
}
```
Assertions:
- Response includes generated memory_handle (string, non-empty).
- No errors.
Postconditions: Memory contains 1 entry.

### M02: memory list after store
Invocation:
```/dev/null/m02_memory_list.json#L1-6
{
  "name": "memory",
  "arguments": { "operation": "list" }
}
```
Assertions:
- Entry count = 1.
- Preview matches prefix of stored summary.
- Fields possibly: handle, created_at/updated_at (if system includes metadata).
Capture handle as H1 for future tests.

### M03: memory load basic
Invocation:
```/dev/null/m03_memory_load.json#L1-6
{
  "name": "memory",
  "arguments": { "operation": "load" }
}
```
Assertions:
- Full text includes "Rust clarity > performance".
- No duplication artifacts.

### M04: memory store second (refined)
Invocation:
```/dev/null/m04_memory_store_second.json#L1-11
{
  "name": "memory",
  "arguments": {
    "operation": "store",
    "summary": "Consolidated: Clarity-first; avoid panics; minimal allocations; descriptive variable names; may later shift performance focus."
  }
}
```
Postconditions: Memory now has 2 entries (H1 original, H2 new). Capture handle H2.

### M05: memory list multiple
Invocation:
```/dev/null/m05_memory_list.json#L1-6
{
  "name": "memory",
  "arguments": { "operation": "list" }
}
```
Assertions:
- Count = 2.
- Ordering: Implementation-defined (document expectation). EXPECTATION: chronological (H1 then H2).
- Previews truncated appropriately (default truncation length if any).

### M06: memory prune by handle
Invocation (prune H1):
```/dev/null/m06_memory_prune_handle.json#L1-9
{
  "name": "memory",
  "arguments": {
    "operation": "prune",
    "memory_handle": "H1"
  }
}
```
Assertions:
- Confirmation of removal (boolean true or absence in subsequent list).
Postconditions: Only H2 remains.

### M07: memory restore by handle
Invocation:
```/dev/null/m07_memory_restore_handle.json#L1-10
{
  "name": "memory",
  "arguments": {
    "operation": "restore",
    "memory_handle": "H1"
  }
}
```
Assertions:
- H1 reappears.
- If ordering modified, document expectation (EXPECTATION: appended at end unless restore_insert_index used).
Postconditions: Entries H2 + restored H1' (maybe new handle or same). Capture new handle as H1R if changed.

### M08: memory prune by index range
Preconditions: At least 2 entries (H2, H1R).
Invocation (remove the first of the current ordered list):
```/dev/null/m08_memory_prune_range.json#L1-11
{
  "name": "memory",
  "arguments": {
    "operation": "prune",
    "start_index": 0,
    "end_index": 1
  }
}
```
Assertions:
- Entry count decremented by 1.
- The targeted index removed.
Postconditions: 1 entry remains.

### M09: memory restore with explicit insert index
Restore previously removed H2 (assuming handle H2 still valid).
Invocation:
```/dev/null/m09_memory_restore_insert.json#L1-12
{
  "name": "memory",
  "arguments": {
    "operation": "restore",
    "memory_handle": "H2",
    "restore_insert_index": 0
  }
}
```
Assertions:
- H2 appears at index 0 in subsequent list.
- Another entry shifts to index 1.

### M10: memory store with auto=true
Invocation:
```/dev/null/m10_memory_store_auto.json#L1-13
{
  "name": "memory",
  "arguments": {
    "operation": "store",
    "auto": true,
    "summary": "Auto-assisted merge: clarity priority, error propagation, minimal allocations, possible later perf shift."
  }
}
```
Assertions:
- New handle H3.
- Auto flag does not discard manual summary.

### M11: memory list with max_preview_chars
Invocation:
```/dev/null/m11_memory_list_preview.json#L1-10
{
  "name": "memory",
  "arguments": {
    "operation": "list",
    "max_preview_chars": 25
  }
}
```
Assertions:
- Each preview length ≤ 25.
- No multi-byte character corruption.

### M12: memory prune non-existent handle
Invocation:
```/dev/null/m12_memory_prune_invalid.json#L1-10
{
  "name": "memory",
  "arguments": {
    "operation": "prune",
    "memory_handle": "NON_EXISTENT_HANDLE"
  }
}
```
Assertions:
- Expected error OR no-op with explicit status (must not silently succeed without status).
- Entry count unchanged.

### M13: memory store empty summary (negative)
Invocation:
```/dev/null/m13_memory_store_empty.json#L1-9
{
  "name": "memory",
  "arguments": {
    "operation": "store",
    "summary": ""
  }
}
```
Assertions:
- Expect validation failure (error message).
- No new entry added.

### M14: memory restore with placeholder replacement
Pre-step: Prune H3 producing a placeholder (if system uses placeholders).
Invocation:
```/dev/null/m14_memory_restore_placeholder.json#L1-15
{
  "name": "memory",
  "arguments": {
    "operation": "restore",
    "memory_handle": "H3",
    "remove_placeholder": true,
    "replace_placeholder_with": "Merged clarity & future performance watch."
  }
}
```
Assertions:
- Entry restored with replacement text included OR appended metadata.
- No residual placeholder artifacts.

### M15: memory prune overlapping indices
Precondition: At least 3 entries (if not, re-store entries).
Invocation:
```/dev/null/m15_memory_prune_overlap.json#L1-13
{
  "name": "memory",
  "arguments": {
    "operation": "prune",
    "start_index": 1,
    "end_index": 10
  }
}
```
Assertions:
- All entries from index 1 onward removed.
- Only index 0 remains.
- No out-of-range panic; gracefully bounds check.

### M16: memory load after sequence
Invocation:
```/dev/null/m16_memory_load_final.json#L1-6
{
  "name": "memory",
  "arguments": { "operation": "load" }
}
```
Assertions:
- Returned composite summary (if system merges) OR latest active entry.
- No orphaned placeholders.

---

## 6. Expected Output Patterns

Because actual schema may vary, define pattern expectations rather than exact JSON:

Common pattern for list:
```
Entries: [
  {
    "handle": "<NON-EMPTY STRING>",
    "preview": "<<= max_preview_chars or full>",
    "length": <int?>,
    "created_at": "<ISO8601?>"
  },
  ...
]
Total: <int>
```

For load:
```
{
  "summaries": [
     {"handle": "Hx", "text": "..."},
     ...
  ]
}
```
OR if singular:
```
{"summary": "..."}
```

For errors:
```
{
  "error": {
    "message": "Description",
    "code": "<validation|not_found|...>"
  }
}
```

---

## 7. Edge & Negative Considerations

| Scenario | Expectation |
|----------|-------------|
| Duplicate store with identical summary | Allowed; distinct handle |
| Prune with both memory_handle and start/end | Prefer explicit handle; ignore range (documentedly) OR error |
| Restore inserting out-of-bounds index | Clamp to end |
| max_preview_chars < 5 | Still honored; truncated safely |
| Large summary ( > 10k chars ) | Should either store or return size error; test if environment allows |

(Optional Stress Test)
```/dev/null/stress_store_large.json#L1-9
{
  "name": "memory",
  "arguments": {
    "operation": "store",
    "summary": "<REPEAT 'A' x 12000>"
  }
}
```

---

## 8. Recommended Execution Order

1. H01–H04 (history baseline)
2. M01–M05 (initial population)
3. M06–M07 (handle prune/restore)
4. M08–M11 (range + previews + auto)
5. M12–M15 (negative & edge)
6. M16 (final verification)

---

## 9. Sample Consolidated Acceptance Script (Logical Narrative)

Script (human/harness pseudo-flow):

1. Confirm history indexes stable (H01).
2. Store first summary (M01) -> record handle H1.
3. List -> verify H1 (M02).
4. Load -> verify content (M03).
5. Store refined summary -> H2 (M04).
6. List -> two entries (M05).
7. Prune H1 (M06).
8. Restore H1 (M07).
9. Prune by range removing first entry (M08).
10. Restore H2 at index 0 (M09).
11. Store auto summary (M10) -> H3.
12. List with preview cap (M11).
13. Attempt prune invalid handle (M12).
14. Attempt store empty summary (M13).
15. Prune H3 then restore with placeholder replacement (M14).
16. Prune overlapping indices (M15).
17. Load final state (M16).

---

## 10. Example Narrative Summaries to Use

Initial manual summary candidate:
"User prioritizes clarity, safe error handling (no unwrap), descriptive names."

Refined summary:
"Clarity-first; avoid panics; minimal allocations; descriptive naming; performance shift anticipated."

Auto-assisted summary:
"Consolidated: clarity > perf (current), propagate errors with '?', minimize allocations, potential future perf optimization."

Placeholder replacement (if used):
"Merged clarity & future performance watch."

---

## 11. Validation Checklist

| Check | Must Hold |
|-------|-----------|
| Handle uniqueness | All stored entries have unique handles until pruned; restored may reuse or alias (document) |
| Idempotence of list | List does not mutate state |
| Prune accuracy | Only specified entries removed |
| Restore insertion | Entry position matches requested index or spec fallback |
| Preview safety | No multi-byte truncation corruption |
| Negative handling | Invalid operations produce explicit error envelope |
| Final load coherence | No references to pruned-only (non-restored) handles |

---

## 12. Reporting Format (For Harness)

For each test produce:
```
[TEST_ID] RESULT: PASS|FAIL
Request: <serialized invocation>
Response: <raw response>
Assertions:
 - <assertion1>: PASS|FAIL
 - <assertion2>: PASS|FAIL
State After: <list summary handles ordered>
```

---

## 13. Maintenance Notes

- If memory backend changes ordering semantics (e.g. MRU vs chronological), adjust tests M05, M07, M09 accordingly.
- If range prune semantics are clarified (inclusive vs exclusive end), update M08 / M15.
- If automatic summarization (`auto=true`) starts generating internal text ignoring provided summary, adapt M10 acceptance to allow system-generated content with a stability token.

---

## 14. Quick Start Minimal Subset (Smoke)

If only smoke testing:
1. H01
2. M01
3. M02
4. M03
5. M06
6. M07
7. M11
8. M16

---

End of specification.