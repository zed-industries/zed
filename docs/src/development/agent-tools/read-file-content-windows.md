# Project Brief: Byte-windowed `read_file` content windows (line-rounded)

## Summary

This project improves the Agent’s ability to read file contents efficiently by adding a byte-window mode to the `read_file` tool. The Agent will be able to request larger, deterministic content windows using byte offsets, and the tool will round returned output to whole line boundaries. The primary goal is to reduce repetitive paging calls (and associated latency/rate limits) while keeping the returned text easy to reason about and safe to splice into subsequent tool calls.

This project is scoped to *paging mechanics* and does not attempt to build a global syntactic map of the codebase.

## Background / Problem

Today, when the Agent needs more context from a medium/large file, it often falls into a pattern like:

- `read_file(path, lines 1-220)`
- `read_file(path, lines 220-520)`
- `read_file(path, lines 520-720)`
- …mixed with `grep` to find landmarks…

This is expensive in:
- tool-call count (more opportunities to hit rate limits),
- latency (many round trips),
- model confusion (the model must pick line ranges and frequently underfetch/overfetch).

A byte-window API provides:
- deterministic forward/back paging by `start_byte`,
- the ability to request a larger chunk up front (`max_bytes`),
- fewer total tool calls, especially on large files.

## Goals

1. **Enable large, deterministic reads**: Allow the Agent to request file content windows using byte offsets and sizes.
2. **Round to clean boundaries**: Returned content must be rounded to whole line boundaries (no partial lines).
3. **Reduce paging calls**: Encourage larger window sizes to reduce repeated reads.
4. **Maintain backwards compatibility**: Existing line-based reads (`start_line`, `end_line`) must continue to work unchanged.
5. **Be safe and bounded**: Enforce a server-side cap to prevent extremely large tool outputs.

## Non-goals

- Providing symbol/outline querying or syntactic navigation of large files.
- Building a global codebase index or “syntactic map”.
- Altering privacy/exclusion behavior for file access.
- Changing how outlines are generated for large files, except where required to support byte windows cleanly.

## Proposed Tool Input Changes

Extend the `read_file` tool input schema with optional byte-window parameters:

- `start_byte: Option<u64>`
  - 0-based byte offset into the file.
  - When omitted, defaults to `0` in byte-window mode.

- `max_bytes: Option<u32>`
  - Requested maximum bytes for the window.
  - When omitted, a default will be applied (see Defaults & Caps).

### Precedence Rules

1. If `start_line` or `end_line` is provided, treat the call as line-range mode (existing behavior).
2. Otherwise, if `start_byte` or `max_bytes` is provided, treat the call as byte-window mode.
3. Otherwise, preserve current behavior (small file full content, large file outline fallback).

## Byte-window Mode Semantics

### Window selection

Given:
- `start = start_byte.unwrap_or(0)`
- `requested_len = max_bytes.unwrap_or(DEFAULT_MAX_BYTES)`
- `end = start + requested_len`

The implementation will clamp to file length and adjust to safe UTF-8 boundaries before converting to internal points/ranges.

### Line rounding

The returned content must be rounded to line boundaries:

- Start is snapped to the beginning of the line containing `start`.
- End is snapped so the output includes only whole lines (typically to the beginning of the next line after `end`, or to end-of-file).

The key property: **no partial lines** are returned.

### Deterministic paging

The Agent should be able to page by setting `start_byte` to the prior returned window end (or a tracked byte offset) and requesting another window.

To support this, the tool may optionally include the effective returned byte range in the output (as a short header) so the Agent can page precisely.

## Defaults & Caps

### Defaults

- `DEFAULT_MAX_BYTES`: A conservative default that is large enough to reduce paging (e.g., 64KiB), but small enough to keep tool outputs manageable.

### Hard cap

- `HARD_MAX_BYTES`: A strict server-side maximum (e.g., 256KiB) applied even if a larger `max_bytes` is requested.

If the Agent requests more than `HARD_MAX_BYTES`, the tool will clamp the request to `HARD_MAX_BYTES`.

## Edge Cases / Special Handling

- **Single extremely long line**: If a single line is longer than `HARD_MAX_BYTES`, rounding to full lines can cause a window to exceed the cap. The implementation must define behavior for this case. Recommended approach:
  - Prefer whole-line output; however, if a single line exceeds the hard cap, allow returning that line truncated with an explicit note, or fall back to returning a bounded slice with clear markers. The chosen behavior must remain deterministic and avoid invalid UTF-8.

- **Files larger than the “auto-outline” threshold**: Byte-window mode should allow reading chunks even for large files (where the default no-args call would return an outline). This enables efficient paging without requiring the Agent to switch to line ranges.

- **UTF-8 safety**: Byte offsets must not split multi-byte characters.

## UX Guidance for the Agent

The tool documentation should recommend:

- Use `max_bytes` generously (up to the hard cap) when you expect to need more context, to reduce paging and rate-limit pressure.
- Prefer byte-window paging for “continue reading” workflows; use line ranges when following up on outline-provided line numbers.

## Telemetry / Observability (optional)

If available, log:
- number of `read_file` calls per task,
- average output size,
- frequency of paging patterns (repeated reads of same file),
- rate-limit related failures.

This will validate that the change reduces tool-call volume in practice.

## Acceptance Criteria

- `read_file` accepts `start_byte` and `max_bytes` and returns content rounded to whole lines.
- Output is safe UTF-8 and does not panic on boundary conditions.
- A single call can return a substantially larger chunk than typical line-based paging patterns, reducing follow-up reads.
- Existing `start_line` / `end_line` behavior remains unchanged.
- Large file default behavior (outline) remains unchanged when byte-window parameters are not provided.

## Out of Scope Follow-up

A separate project will address “global syntactic navigation” (e.g., outline querying, codebase-wide symbol maps, and syntactic retrieval interfaces). This brief intentionally does not cover that work.