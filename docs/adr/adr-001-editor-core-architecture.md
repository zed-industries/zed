# ADR 001: Editor Core Architecture & Headless Decoupling

## Status: Accepted

## Context
Zed required complete architectural decoupling of text editing, transaction application, and buffer management from GPU window creation to support headless CI/CD pipelines, background daemon operation, and programmatic AI agent integration.

## Decision
1. Extract core buffer manipulation and rope operations into `crates/zed_core_lib` and `crates/libzed_core`.
2. Publish semantic version-stable abstractions in `crates/zed_api` (`EditorCore`, `EditorBackend`, `ZedApiError`).
3. Implement `crates/zed_daemon` providing multi-transport JSON-RPC 2.0 servers over stdio and TCP sockets.

## Consequences
- **Positive**: Enables zero-GPU headless operation, programmatic JSON-RPC 2.0 buffer edits, deterministic testing, and multi-language AI agent control.
- **Negative**: Requires maintaining version-stable API compatibility across releases.

## Related
- Implemented in: `crates/zed_daemon`, `crates/zed_core_lib`, `crates/zed_api`, `crates/zed/src/main.rs`.
