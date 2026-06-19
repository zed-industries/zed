# Agent Corruption Defense System - Implementation Tasks (v2)

This is the ordered task list for implementing the 5-layer Agent Corruption
Defense system. Tasks are organized by **ROI phase** (not by layer), because the
review identified that some layers deliver value much faster than others.

---

## Phase A: Ship Immediately (highest ROI)

These three items are cheap and structural. They prevent the most common
 corruption without any detection heuristics at all.

### PA-T1: Implement `attempt_completion` tool + system prompt + enforcement
**Files**: `crates/agent/src/tools/attempt_completion_tool.rs` (new), `crates/agent/src/thread.rs`
**Description**: Create the tool, add it to every agent system prompt,
enforce in turn loop.
**Acceptance Criteria**:
- [ ] `AttemptCompletionTool` with `AgentTool` impl (optional `summary: String` input)
- [ ] System prompt includes the MUST-call instruction
- [ ] Turn loop rejects responses without the tool call
- [ ] Triggers retry path on missing `attempt_completion`
- [ ] Test: missing tool → retry; present tool → success

### PA-T2: Wire corruption errors into existing retry path
**Files**: `crates/agent/src/thread.rs`
**Description**: Reuse the existing `retry_completion_error` and refusal fallback
infrastructure for all corruption-detected retries.
**Acceptance Criteria**:
- [ ] `RetryReason` extended with corruption variants (voting-based, not a catch-all)
- [ ] `run_turn_internal` handles corruption errors as retry triggers
- [ ] Corruption-specific retry count tracked separately from network/retry count
- [ ] After max corruption retries, trigger fallback model (reuse refusal fallback)

### PA-T3: Add telemetry + corruption snapshots
**Files**: `crates/agent/src/thread.rs`, telemetry module
**Description**: When corruption is detected, capture evidence for later analysis.
**Acceptance Criteria**:
- [ ] `CorruptionEvent` struct with layer, model, retry_count, resolved
- [ ] `CorruptionSnapshot` with last 4 KB output, prompt hash, triggered signals
- [ ] Telemetry sent to `telemetry::event!()`
- [ ] Configurable snapshot collection (on/off, retention)
- [ ] Redacted snapshots if needed (PII/data privacy)

---

## Phase B: Core Detectors (catches ~80% of genuine corruption)

These three detectors are the highest-signal, lowest-noise. They catch the exact
failure mode in the screenshot.

### PB-T1: Repetition detector
**Files**: `crates/agent/src/output_quality.rs`
**Description**: Detect token loops and degenerate repeated output.
**Acceptance Criteria**:
- [ ] RLE (run-length encoded) token tracking
- [ ] Detect >64-character repeated substrings
- [ ] Confidence score based on repetition length / window size
- [ ] Test: known-good output scores low; known-gibberish scores high

### PB-T2: Script switching detector
**Files**: `crates/agent/src/output_quality.rs`
**Description**: Rapid Unicode script transitions (Latin → Han → Cyrillic → etc.).
**Acceptance Criteria**:
- [ ] Histogram of Unicode scripts in rolling window
- [ ] Fires when >2 script transitions detected
- [ ] Confidence proportional to transition frequency
- [ ] Test: Latin text → low; Latin+Han+Cyrillic mixed → high

### PB-T3: Task irrelevance detector
**Files**: `crates/agent/src/output_quality.rs`
**Description**: In an editing workflow, output that does not mention files,
symbols, or the current task is a strong corruption signal.
**Acceptance Criteria**:
- [ ] Extract current task context (files, symbols, user prompt)
- [ ] Score output for mentions of task-relevant terms
- [ ] Confidence inversely proportional to task-relevance ratio
- [ ] Test: code-and-file output → low; random news text → high

---

## Phase C: Edit-Level Defense

These catch corruption that passes stream scoring but produces bad edits.

### PC-T1: AST validation (syntax check)
**Files**: `crates/agent/src/ast_validation.rs` (new)
**Description**: After patch, parse file. If parse fails, reject edit.
**Acceptance Criteria**:
- [ ] `AstValidator` trait with tree-sitter backend
- [ ] Per-language opt-out in settings
- [ ] Return `EditSessionResult::Failed` on parse error
- [ ] Test: valid Rust edit passes; invalid Rust edit rejected

### PC-T2: Scope anomaly detection (from prompt + agent plan)
**Files**: `crates/agent/src/anomaly_detection.rs` (new)
**Description**: Compare expected vs actual edit scope. Use both prompt
heuristics and the agent's own plan for better accuracy.
**Acceptance Criteria**:
- [ ] `ExpectedEditScope::from_prompt()` with keyword heuristics
- [ ] `ExpectedEditScope::from_agent_plan()` (if agent plan exists)
- [ ] Compare against actual files changed / lines modified
- [ ] Test: rename producing 7 files → anomaly; rename producing 1 file → OK

---

## Phase D: Refinement Layers

### PD-T1: Structure breakdown detector (JSON bracket balance)
**Files**: `crates/agent/src/output_quality.rs`
**Description**: During tool-call streaming, verify JSON/diff bracket balance.
**Acceptance Criteria**:
- [ ] Track `{`, `[`, `(` and their closes across rolling window
- [ ] Fires on unbalanced structures (not during normal streaming boundaries)
- [ ] Ignore expected boundaries (e.g., inside strings)

### PD-T2: Semantic coherence detector (rolling topic vector)
**Files**: `crates/agent/src/output_quality.rs`
**Description**: Track topic drift. Use a crude TF-IDF approach, not keyword lists.
**Acceptance Criteria**:
- [ ] Extract topic vectors for the last N chunks
- [ ] Track cosine similarity vs. previous window
- [ ] Fires when similarity drops below threshold
- [ ] Test: consistent topic → high similarity; abrupt shift → low

### PD-T3: Character class chaos detector
**Files**: `crates/agent/src/output_quality.rs`
**Description**: Repeated character-class transitions (code → prose → symbols → code).
**Acceptance Criteria**:
- [ ] Classify characters into classes (code, prose, symbol, number, etc.)
- [ ] Track transition frequency
- [ ] Fires on repeated class-switching (the screenshot pattern)
- [ ] Test: clean code → low; "English Code 中文 Symbol English" → high

### PD-T4: AST delta (suspiciously large tree change)
**Files**: `crates/agent/src/ast_validation.rs`
**Description**: After a trivial task (e.g., rename), if the AST changed by >50%,
flag as suspicious.
**Acceptance Criteria**:
- [ ] `AstValidator::compute_delta(before, after)` returns `AstDelta`
- [ ] Flag if `node_change_ratio` exceeds task-appropriate threshold
- [ ] Configurable per-language thresholds

### PD-T5: Model fallback for corruption
**Files**: `crates/agent/src/thread.rs`
**Description**: After N corruption retries, fall back to a configured secondary model.
**Acceptance Criteria**:
- [ ] Configurable `max_corruption_retries` (default: 2)
- [ ] After max retries, attempt fallback model (reuse refusal fallback logic)
- [ ] Telemetry event for corruption fallback
- [ ] Test: corruption → retry → fallback → success/failure

---

## Cross-Cutting Tasks

### CC-T1: Integration tests
**Files**: `crates/agent/src/tests/corruption_defense.rs` (new)
**Description**: End-to-end tests with mock LLMs that simulate corruption.
**Acceptance Criteria**:
- [ ] Mock LLM that returns corrupted content → verify discarded, retried
- [ ] Mock LLM that returns valid content → verify passes
- [ ] Mock LLM that forgets `attempt_completion` → verify retry
- [ ] Full pipeline: corruption → snapshot → retry → fallback → success

### CC-T2: User-facing messages
**Files**: UI strings in `crates/acp_thread/`
**Description**: Clear, non-technical messages when corruption is detected.
**Acceptance Criteria**:
- [ ] On detection: "Model output appeared corrupted. Retrying..."
- [ ] On retry failure: "Model output was corrupted. Please try again."
- [ ] Never show raw corrupted content

### CC-T3: Documentation
**Files**: `docs/` or inline doc comments
**Description**: Document the corruption defense system for developers and users.
**Acceptance Criteria**:
- [ ] Developer doc: how to add a new detector
- [ ] User doc: what happens when "appeared corrupted" is shown
- [ ] Tuning doc: how to adjust thresholds, read telemetry
