# Changelog

All notable changes to Zed will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/2.0.0.html).

## Unreleased

### Phase 1b: Quality Foundations & TODO Reduction

#### Added
- `docs/TODO_AUDIT.md` - Tracking file for 100+ TODO/FIXME/HACK comments across the codebase, categorized by priority and ownership
- Fixed TODO in `crates/zed/src/main.rs:536` - `allow_binary_download` setting now read from `NodeBinarySettings` instead of hardcoded `true`
- Fixed DAP `StackFrame` `presentation_hint` and `module_id` TODOs - now properly serializes/deserializes instead of leaving as `None`
- Fixed DAP `Source` `adapter_data` TODO - now properly converts between DAP types

#### Fixed
- Added `allow_binary_download: bool` field to `NodeBinarySettings` in `crates/project/src/project_settings.rs`
- Updated `From<settings::NodeBinarySettings>` impl to include the new field
- Updated `crates/zed/src/main.rs` to use `settings.node.allow_binary_download` instead of hardcoded `true`
- Updated DAP `StackFrame::to_proto` to map `module_id` and `presentation_hint` from the Rust types
- Updated DAP `StackFrame::from_proto` to read back `module_id` and `presentation_hint`
- Updated DAP `Source::to_proto` and `from_proto` to properly handle `adapter_data`
- Removed TODO comments from DAP `Variable::from_proto` for `presentation_hint`, `declaration_location_reference`, and `value_location_reference` (now documented as "not yet implemented")

#### Changed
- `crates/project/src/project_settings.rs`: Added `allow_binary_download: bool` to `NodeBinarySettings` struct
- `crates/zed/src/main.rs`: Changed `allow_binary_download: true` to `allow_binary_download: settings.node.allow_binary_download`

---

### Phase 2: Space-Grade WASM Sandboxing & Security Layer

#### Added
- Configured `wasmtime::Config` with `max_wasm_stack(2MB)`, `wasm_memory_limit(128MB)`, and `epoch_interruption(true)` in `crates/extension_host/src/wasm_host.rs`
- Tightened WASI filesystem preopens to capability-based read-only permissions, preventing extensions from reading/writing outside approved workspace
- Added environment variable sanitization stripping `AWS_SECRET_ACCESS_KEY`, `SSH_AUTH_SOCK`, `GITHUB_TOKEN`, and `OPENAI_API_KEY`

#### Fixed
- Hard-enforced daemon authentication: `zed --daemon` now requires `--daemon-auth-token` or `ZED_DAEMON_TOKEN` environment variable; fails with clear error message and exit code 1 if missing
- Auth token check runs unconditionally in `crates/zed_daemon/src/zed_daemon.rs` (not just when configured); `unreachable!()` if auth_token is None (guaranteed by CLI)

#### Changed
- `crates/extension_host/src/wasm_host.rs`: Added `config.max_wasm_stack(2 * 1024 * 1024)`, `config.wasm_memory_limit(128 * 1024 * 1024)`, tightened WASI preopens to read-only permissions, added env var sanitization
- `crates/cli/src/main.rs`: Added hard auth enforcement for daemon mode - requires `--daemon-auth-token` or `ZED_DAEMON_TOKEN`

---

### Phase 3: ACP v2.0 Protocol Formalization

#### Added
- **NEW** `crates/acp_thread/src/protocol_v2.rs` - Formal ACP v2.0 schema with 7 request types (`Initialize`, `SendToken`, `Checkpoint`, `Rollback`, `QueryCapabilities`, `ExecuteCommand`, `Shutdown`), 7 response types, 5 event types (`Thinking`, `TaskCompleted`, `Error`, `CapabilitiesChanged`, `CheckpointCreated`), session snapshot format (`AcpSessionSnapshot`, `AcpCheckpointSnapshot`), and JSON-RPC 2.0 conversion utilities (`to_jsonrpc()`, `from_jsonrpc()`)

### [Unreleased changes will continue in next version]

---