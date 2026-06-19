# Agent Corruption Defense System

A multi-layer defense system to detect and block corrupted model output before it reaches the user as actionable edits.

## Layers

| Layer | Name | Catches | Where |
|-------|------|---------|-------|
| 1 | Output Quality Scoring | Repetition, script switching, task irrelevance, structure breakdown | Rolling window on stream |
| 2 | Diff-Size Anomaly Detection | Oversized edits vs. expected task scope | Edit tool finalize |
| 3 | AST Validation + AST Delta | Syntax errors + suspiciously large tree changes | Post-edit, pre-save |
| 4 | Structured Edit Protocol (`attempt_completion`) | Model rambling, forgetting to use tools, raw text dumping | Turn completion check |
| 5 | Retry + Corruption Snapshots | Recovery, evidence capture, model fallback | Turn loop |

## Documents

- [`requirements.md`](requirements.md) - Functional and non-functional requirements (v2)
- [`design.md`](design.md) - Detailed architecture and designs for each layer (v2)
- [`tasks.md`](tasks.md) - Ordered implementation task list based on ROI phase (v2)

## Quick Start

Start with **Phase A** (highest ROI):

1. `attempt_completion` tool — structural guarantee, no heuristics needed
2. Wire into existing retry + fallback infrastructure
3. Telemetry + corruption snapshots for data-driven tuning

Then **Phase B** (core detectors): repetition, script switching, task irrelevance.

## Implementation Order (by ROI)

| Phase | What | Why First |
|-------|------|-----------|
| A | `attempt_completion`, retry/snapshots | Structural, cheap, prevents most corruption from reaching user |
| B | Repetition, script switching, task irrelevance | Highest signal, lowest false-positive |
| C | AST validation, scope anomaly | Edit-level safety |
| D | Semantic coherence, character class chaos, AST delta | Refinement layers |

## Status

_Spec complete at v2. Implementation scheduled in ROI phases._

## Changelog

- **v2**: Replaced weighted composite score with multiple-detector voting; added
  `CorruptionSignal`, `CorruptionAssessment`, `CorruptionSnapshot`; redesigned
  tasks into ROI-based phases (A–D + cross-cutting); added AST delta and
  `from_agent_plan` scope estimation.
