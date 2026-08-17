# ADR 005: Data Flow, State Management & Memory Pressure Control

## Status: Accepted

## Context
High performance in long-running editor sessions and daemon mode demands strict memory bounds, proactive garbage collection, and lock-free/poison-safe synchronization primitives.

## Decision
1. Implement `MemoryPressureMonitor` with 128MB minimum headroom thresholds.
2. Synchronize thread-safe state via poison-resilient `safe_lock` mutex wrappers.
3. Centralize global internationalization in `crates/i18n` with thread-safe `RwLock` dictionaries.

## Consequences
- **Positive**: Zero OOM crashes under high load, non-blocking readers for localized strings, deterministic garbage collection.
- **Negative**: Garbage collection purges inactive undo trees during extreme memory constraints.
