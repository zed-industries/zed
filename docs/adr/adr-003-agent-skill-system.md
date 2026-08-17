# ADR 003: Agent Skill System & Atomic Checkpoint-Rollback Protocol

## Status: Accepted

## Context
AI agents modifying large codebases require structured sandboxing, deterministic tool execution, and the ability to roll back speculative changes if compile checks or test passes fail.

## Decision
1. Standardize ACP thread connections and JSON-RPC tool-use methods (`agent/checkpoint`, `agent/rollback`, `agent/prompt`).
2. Provide in-memory buffer snapshot checkpointing allowing agents to save clean states before refactoring and restore immediately upon errors.
3. Enforce execution environment token sanitization to prevent accidental API key leaks.

## Consequences
- **Positive**: Resilient, self-healing agent refactoring workflows.
- **Negative**: In-memory snapshots consume RAM proportional to modified buffer sizes (managed by `MemoryPressureMonitor`).
