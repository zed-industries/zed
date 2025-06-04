# Worlds Fair Talk: CI in the Era of AI

This crate contains the materials for Nathan Sobo's talk "CI in the Era of AI: From Unit Tests to Stochastic Evals" presented at the AI Engineer World's Fair.

## Overview

The talk explores how Zed's testing philosophy evolved when integrating language models, using the streaming edits feature as a case study. It demonstrates the shift from purely deterministic testing to embracing statistical methods when working with inherently stochastic systems.

## Structure

The talk is organized as numbered source files with accompanying speaker notes:

### Slides (in `src/`)
- `00_intro.md` - Title slide and introduction
- `01_deterministic_testing_at_zed.rs` - Zed's traditional deterministic testing approach
- `02_stochastic_unit_tests.rs` - Introduction to statistical testing for LLMs
- `03_streaming_edits_overview.md` - Overview of the streaming edits challenge
- `04_deterministic_streaming_tests.rs` - Traditional tests for algorithmic components
- `05_empty_old_text_problem.rs` - First eval failure: empty old_text bug
- `06_tag_mismatch_discovery.rs` - XML tag mismatch issues (5% failure rate)
- `07_the_indentation_discovery.rs` - Indentation problem and algorithmic solution
- `08_escaping_chaos.rs` - Character escaping issues (especially for Gemini)
- `09_lessons_learned.md` - Key takeaways about testing with LLMs

### Speaker Notes (in `notes/`)
Each slide has a corresponding `.md` file with speaker notes in the `notes/` directory.

## Key Concepts

### Streaming Edits Feature
- Allows users to see AI code edits character-by-character as they're generated
- Works around API limitations where tool calling can't stream edit content
- Uses a two-phase approach: tool call for intent, then raw text streaming

### Testing Evolution
1. **Deterministic Tests**: For parsing, algorithms, indentation adjustment
2. **Statistical Tests (Evals)**: For LLM behavior, requiring threshold pass rates
3. **Property-Based Tests**: For comprehensive algorithmic validation

### Major Discoveries
- **Empty old_text**: 0% → 99% pass rate with one prompt line
- **Tag mismatches**: Models mess up XML closing tags, made parser tolerant
- **Indentation**: Built automatic adjustment algorithm
- **Character escaping**: Gemini went from 35% → 86% with one instruction

## Historical Context

The prompt evolution was driven by specific eval failures:
- Commit `ab017129d8` (May 22, 2025) by Oleksiy Syvokon made major improvements:
  - Gemini: 35% → 86%
  - Claude: 96% → 98%
  - GPT-4: 81% → 100%

## Talk Duration

Approximately 15 minutes, designed to move quickly through concrete examples.

## Building the Talk

This crate is not meant to be compiled - the code examples are illustrative and may use simplified types for clarity. The actual implementation lives in `crates/assistant_tools/`.

## Future Work

If continuing this talk:
- Consider adding `test_edit_events` showing real-time event streaming
- The `eval_add_overwrite_test` has surprisingly low pass rates (16-35%) and might reveal interesting failure modes
- More examples of property-based testing could strengthen the deterministic testing section

## Key Message

The core thesis: When building on LLMs, you must embrace empirical methods. You can't reason about their behavior - you can only measure it. This requires:
1. Statistical thresholds instead of binary pass/fail
2. Learning from failure patterns  
3. Accepting imperfection and building resilient systems
4. Layering deterministic and statistical tests appropriately