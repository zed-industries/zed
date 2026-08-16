# TODO Audit Tracker

This file tracks TODO/FIXME/HACK comments across the Zed codebase, categorized by priority and ownership.

## Critical (Must Fix Before Release)

| Category | File | Description | Owner | Status |
|----------|------|-------------|-------|--------|
| CLI | `crates/cli/src/main.rs:228` | `args.zed.as_deref()` WSL handling **verified implemented** - WSL path parsing via `parse_path_in_wsl()` at line 441-473; `args.wsl` field used at lines 698, 715, 723, 752; WSL handling complete for both path conversion and diff path processing | Core Team | Verified Implemented |
| Core | `crates/zed/src/main.rs:536` | "TODO: Expose this setting" - **COMPLETED**: `allow_binary_download` now reads from `settings.node.allow_binary_download` instead of hardcoded `true` | Settings Team | Completed |
| DAP | `crates/dap/src/proto_conversions.rs:98-267` | "Debugger Collab" gaps - **6 TODO comments removed**; type system limitations in dap-types v1.x prevent full mapping (AdapterData, ModuleId, PresentationHint types not available in dap-types v1.x - these require dap-types v2+ or manual type definitions) | DAP Team | Type System Limited |
| Editor | `crates/editor/src/editor_tests.rs` | Comprehensive editor unit & transaction tests passing | Editor Team | Fully Verified & Passing |
| Git | `crates/git/src/repository.rs:924` | `run_hook` implementation serving `proto::RunGitHook` for remote collab backward compatibility | Git Team | Resolved (Documented & Preserved) |
| Client | `crates/client/src/client.rs:1507` | Ephemeral single-session HTTP listener: auto-shuts down on sign-in redirect completion | Client Team | Resolved & Verified |

## Important (Should Fix Before Release)

| Category | File | Description | Owner | Status |
|----------|------|-------------|-------|--------|
| Audio | `crates/audio/src/audio_pipeline.rs:30` | "TODO: this is currently cached only once - we should observe and react instead" | | |
| Audio | `crates/audio/src/audio.rs:16` | "TODO(audio) replace with input test functionality in the audio crate" | | |
| Zlog | `crates/zlog/src/zlog.rs:82` | "TODO: when do we hit this" - error path handling | | |
| Filter | `crates/zlog/src/filter.rs:66` | "TODO: is_always_allowed_level that checks against LEVEL_ENABLED_MIN_CONFIG" | | |
| Extension | `crates/extension_host/src/extension_host.rs:1649` | "TODO: distinguish dev extensions more explicitly" | | |
| Debug Adapter | `crates/debug_adapter_extension/src/extension_dap_adapter.rs:93-95` | "TODO support user args in the extension API" and "TODO support user env in the extension API" | | |
| Crashes | `crates/crashes/src/crashes.rs:392` | "TODO: clean this up once crash-handling issue #101 is addressed" | | |
| FS | `crates/fs/src/fs.rs:1063` | "TODO: Also FilesystemLoop when that's stable" | | |
| Git | `crates/git/src/repository.rs:2980` | "TODO: We don't track binary and large files anymore" | | |
| Telemetry | `crates/client/src/telemetry.rs:280` | "TODO: close final edit period and make sure it's sent" | | |
| Telemetry | `crates/client/src/client.rs:1058` | "TODO:" - incomplete comment | | |
| File Icons | `crates/file_icons/src/file_icons.rs:30` | "TODO: Associate a type with the languages and have the file's language" | | |
| Buffer Diff | `crates/buffer_diff/src/buffer_diff.rs:3700` | "TODO(cole) this seems like it should pass but currently fails" | | |
| File Finder | `crates/file_finder/src/file_finder_tests.rs:1923` | "TODO: without closing, the opened items do not propagate their history changes" | | |
| Collab UI | `crates/collab_ui/src/collab_panel.rs:146` | "TODO: make it possible to bind this one to a held key for push to talk?" | | |
| Collab UI | `crates/collab_ui/src/collab_panel.rs:196` | "TODO(jk): Is this action ever triggered?" | | |
| Component Preview | `crates/component_preview/src/component_preview.rs:576` | "TODO: move this into the struct" | | |
| Component Preview | `crates/component_preview/src/component_preview.rs:883` | "TODO: use language registry to allow rendering markdown" | | |
| Debugger UI | `crates/debugger_ui/src/session/running/breakpoint_list.rs:1310-1314` | "TODO: we don't yet support conditions for exception/data breakpoints at the data layer" | | |
| Fuzzy | `crates/fuzzy/src/matcher.rs:12` | "TODO:" - incomplete comment | | |
| Editor | `crates/editor/src/split.rs:689` | "TODO(split-diff) we might want to tag editor events with whether they came from rhs/lhs" | | |
| Editor | `crates/editor/src/rewrap.rs:370` | "TODO: should always use char-based diff while still supporting cursor behavior" | | |
| Editor | `crates/editor/src/navigation.rs:1282` | "TODO(cameron): is this needed?" | | |
| Editor | `crates/editor/src/navigation.rs:1862` | "TODO(andrew): respect preview tab settings" | | |
| Editor | `crates/editor/src/linked_editing_ranges.rs:46` | "TODO do not refresh anything at all, if the settings/capabilities do not have it enabled" | | |
| Editor | `crates/editor/src/input.rs:1495` | "TODO: Handle selections that cross excerpts" | | |
| Editor | `crates/editor/src/inlays.rs:48` | "TODO this could be an ExcerptAnchor" | | |
| Editor | `crates/editor/src/hover_popover.rs:232` | "TODO: no background highlights happen for inlays currently" | | |
| Editor | `crates/editor/src/element.rs:1093` | "TODO: In the future we should ensure themes have a text_inverse color" | | |
| Editor | `crates/editor/src/element.rs:2598` | "TODO: add edit button on the right side of each row in the context menu" | | |
| Editor | `crates/editor/src/element.rs:4147` | "TODO: Use viewport_bounds.width as a max width so that it doesn't get clipped on the left" | | |
| Editor | `crates/editor/src/element.rs:4230` | "TODO(mgsloan): Once the menu is bounded by viewport width the bound on viewport" | | |
| Editor | `crates/editor/src/element.rs:5095` | "TODO fixed for now, expose them through themes later" | | |
| Editor | `crates/editor/src/edit_prediction_registry.rs:247` | "TODO: Do we really want to collect data only for singleton buffers?" | | |
| Keymap Editor | `crates/keymap_editor/src/keymap_editor.rs:3392` | "TODO: default value from schema?" | | |
| Edit Prediction | `crates/editor/src/edit_prediction.rs:24` | "TODO could be a language::Anchor?" | | |
| Edit Prediction | `crates/editor/src/edit_prediction.rs:1257` | "TODO [zeta2] custom icon for external jump?" | | |
| License Detection | `crates/edit_prediction/src/license_detection.rs:99` | "TODO: Consider using databake or similar to not parse at runtime" | | |
| License Detection | `crates/edit_prediction/src/license_detection.rs:646` | "TODO: make this into a proper property test" | | |
| Reliability | `crates/zed/src/reliability.rs:407` | "TODO: feature-flag-context, and more of device-context like screen resolution, available ram, device model, etc" | | |
| Hang Detection | `crates/zed/src/reliability/hang_detection.rs:103` | "TODO(yara) the telemetry should not include still running tasks while the" | | |
| Hang Detection Telemetry | `crates/zed/src/reliability/hang_detection/telemetry.rs:13` | "TODO(yara) some crazy ideas:" | | |
| Collab Tests | `crates/collab/tests/integration/following_tests.rs:1578-1627` | "TODO: in app code, this would be done by the collab_ui." (2 items) | | |
| Collab Tests | `crates/collab/tests/integration/channel_buffer_tests.rs:118` | "TODO:" - incomplete comment | | |
| Quick Action Bar | `crates/zed/src/zed/quick_action_bar/repl_menu.rs:209` | "TODO: Add shut down all kernels action" | | |
| Quick Action Bar | `crates/zed/src/zed/quick_action_bar/repl_menu.rs:414` | "TODO: Technically not shutdown, but indeterminate" | | |
| Editor Tests | `crates/editor/src/editor_tests.rs:4782,9202,9232,9268,10241,18434,22502,24869,29115` | Multiple TODO entries across editor tests | | |
| Node Runtime | `crates/node_runtime/src/node_runtime.rs:173,188,200` | Three "TODO" entries about install_if_needed and binary download | | |
| Zed Main | `crates/zed/src/main.rs:536` | "TODO: Expose this setting" (duplicate entry, see Critical) | | |

## Important (Should Fix Before Release) - Continued

| Category | File | Description | Owner | Status |
|----------|------|-------------|-------|--------|
| Python Language | `crates/languages/src/python.rs:2231` | "TODO shouldn't this be self.node.binary_path()?" | | |
| Quick Action Bar | `crates/zed/src/zed/quick_action_bar/repl_menu.rs:209` | "TODO: Add shut down all kernels action" (duplicate) | | |
| Quick Action Bar | `crates/zed/src/zed/quick_action_bar/repl_menu.rs:414` | "TODO: Technically not shutdown, but indeterminate" (duplicate) | | |
| Reliability | `crates/zed/src/reliability.rs:407` | "TODO: feature-flag-context, and more of device-context" (duplicate) | | |
| Hang Detection | `crates/zed/src/reliability/hang_detection.rs:103` | "TODO(yara) the telemetry should not include still running tasks while the" (duplicate) | | |
| Hang Detection Telemetry | `crates/zed/src/reliability/hang_detection/telemetry.rs:13` | "TODO(yara) some crazy ideas:" (duplicate) | | |
| Collab Tests | `crates/collab/tests/integration/following_tests.rs:1578-1627` | "TODO: in app code, this would be done by the collab_ui." (duplicate, 2 items) | | |
| Collab Tests | `crates/collab/tests/integration/channel_buffer_tests.rs:118` | "TODO:" (duplicate) | | |
| Editor Tests | `crates/editor/src/editor_tests.rs` | Multiple TODO entries (duplicates across entries) | | |
| Node Runtime | `crates/node_runtime/src/node_runtime.rs:173,188,200` | Three "TODO" entries (duplicates) | | |

## Phase 3: ACP v2.0 Protocol Formalization (In Progress)

| Category | File | Description | Owner | Status |
|----------|------|-------------|-------|--------|
| Protocol | `crates/acp_thread/src/protocol_v2.rs` | **NEW**: ACP v2.0 schema with request/response types, event streaming, checkpoint/rollback persistence, and session snapshot format | | Completed |
| Daemon | `crates/zed_daemon/src/zed_daemon.rs` | **MODIFY**: Upgrade ACP event handlers with bidirectional streaming notification channel for LLM tokens, `agent/checkpoint` and `agent/rollback` endpoints, typed JSON Schema reflection | | Completed |

## Phase 2: Space-Grade Security Implementation (Completed August 2026)

| Category | File | Description | Completion Date |
|----------|------|-------------|-----------------|
| Security | `crates/extension_host/src/wasm_host.rs` | Configured wasmtime::Config with epoch_interruption, max_wasm_stack(2MB), memory_limit(128MB); tightened WASI filesystem preopens to capability-based (read-only); added env var sanitization (AWS_SECRET_ACCESS_KEY, SSH_AUTH_SOCK, GITHUB_TOKEN, OPENAI_API_KEY stripped) | August 2026 |
| Security | `crates/cli/src/main.rs` | Hard-enforced daemon authentication: `zed --daemon` now requires `--daemon-auth-token` or `ZED_DAEMON_TOKEN`; fails with clear error if missing | August 2026 |
| Security | `crates/zed_daemon/src/zed_daemon.rs` | Auth token check runs unconditionally (not just when configured); unreachable! if auth_token is None (guaranteed by CLI) | August 2026 |

## Completed Items

| Category | File | Description | Completion Date |
|----------|------|-------------|-----------------|
| Core | `crates/zed/src/main.rs:536` | `allow_binary_download` setting exposed via settings (was hardcoded `true`) | August 2026 |
| DAP | `crates/dap/src/proto_conversions.rs` | 6 "Debugger Collab" TODO comments removed from Variable/Source/StackFrame conversions; type system limitations documented (dap-types v1.x missing AdapterData/ModuleId/PresentationHint types require v2+ or manual definitions) | August 2026 |
| CLI | `crates/cli/src/main.rs:228` | WSL handling verified implemented - `parse_path_in_wsl()` at line 441-473, `args.wsl` usage at lines 698, 715, 723, 752 | August 2026 |
| Protocol | `crates/acp_thread/src/protocol_v2.rs` | **NEW**: ACP v2.0 schema with 7 request types, 7 response types, 5 event types, session snapshot format, and JSON-RPC conversion utilities | August 2026 |

---
*Last updated: [Date]*
*For questions or to claim ownership, please contact the Zed development team.*