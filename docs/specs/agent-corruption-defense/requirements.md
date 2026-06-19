# Agent Corruption Defense System - Requirements (v2)

## Context

The Zed agent is a model-driven coding assistant that edits code, runs tools, and
interacts with the user. Like any LLM-powered system, it can suffer from
**context collapse / token-cor enters corrupted model emissaries** — the model
stops producing structured output and emits fragments from unrelated parts of its
context window, training data, or tool outputs. These fragments may include
code symbols, UI strings, multiple languages, news articles, legal text, etc.

The goal is not to prevent the model from ever corrupting (impossible), but to
ensure corrupted output **never reaches the user as an actionable edit**.

## Goals

1. **Detect** model output corruption as early as possible in the pipeline
2. **Block** corrupted output from becoming visible/edible diffs
3. **Recover** gracefully via retry with optional model fallback
4. **Inform** the user clearly when corruption is detected
5. **Maintain** low performance overhead for the 99% case (clean responses)

## Non-Goals

- Preventing model hallucinations that are syntactically valid
- Guaranteeing 100% detection (statistical approach, heuristics-based)
- Rewriting the core agent architecture

## Functional Requirements

### FR-1: Output Quality Scoring (Multiple Detector Votes)
The system shall apply independent corruption detectors to a rolling window of
model output. Each detector reports a `CorruptionSignal` and a confidence. The
system triggers corruption when enough high-confidence signals have fired.

Supported signals:
- **Repetition**: Token loops, excessive repeated n-grams or tokens
- **Script Switching**: Rapid Unicode script transitions (e.g., Latin → Han → Cyrillic)
- **Structure Breakdown**: Invalid JSON bracket balance, malformed JSON structure
- **Semantic Collapse**: Abrupt topic drift within a rolling window
- **Task Irrelevance**: Output unrelated to the current task/filenames
- **Character Class Chaos**: Repeated character-class transitions (code → prose → symbols → code)

### FR-2: Diff-Size Anomaly Detection
The system shall estimate expected edit scope from the user's prompt and the
agent's own plan. If the actual edit exceeds the expected scope by a
configurable margin, flag for review or confirmation.

### FR-3: AST Validation + AST Delta
For edits targeting known languages (Rust, TypeScript, etc.), before
finalizing:
1. Parse the patched file and reject if parsing fails.
2. If an "old" AST exists, compute the AST delta. If a trivial task
   (e.g., rename variable) results in a massive tree mutation, flag as suspicious.

### FR-4: Structured Edit Protocol (attempt_completion tool)
The agent shall enforce a structured protocol where the model must explicitly
signal completion via a tool call (`attempt_completion`). Raw text output after
the final tool call shall trigger a retry.

### FR-5: Retry-on-Corruption + Snapshots
When corruption is detected:
- Discard the corrupted response
- Store a **corruption snapshot** (last 4 KB of output, triggered signals, model, prompt hash)
- Increment retry counter and retry with the same model
- After N corruption-specific retries, optionally fall back to a secondary model
- Surface a user-visible message

## Quality Attributes

| Attribute | Target |
|-----------|--------|
| Detection latency | < 2ms per chunk |
| False positive rate | < 1% |
| Retry success rate | > 80% of corruption events |
| User-visible impact | Clear message, no phantom edits |
| Performance overhead | < 0.1% of total turn time |

## Constraints

- Must integrate with existing `Thread::run_turn_internal` retry logic
- Must work across all supported language model providers
- Must not break existing tool streaming behavior
- Must be configurable (on/off, thresholds) via settings

## Related Systems

- `crates/agent/src/thread.rs` - agent turn loop
- `crates/agent/src/tools/edit_file_tool.rs` - file edit tool
- `crates/agent/src/tools/write_file_tool.rs` - file write tool
- `crates/language_model/` - LLM provider abstractions
- `crates/acp_thread/` - ACP thread and tool call event streaming
