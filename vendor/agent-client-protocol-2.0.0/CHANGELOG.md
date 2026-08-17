# Changelog

## [Unreleased]

## [2.0.0](https://github.com/agentclientprotocol/rust-sdk/compare/v1.3.0...v2.0.0) - 2026-07-23

Version 2.0 keeps the stable ACP v1 wire schema unchanged while making coordinated breaking
changes to the Rust SDK APIs and low-level transport boundary.

### Breaking changes

- `Channel` now carries batch-aware `TransportFrame` values, and custom `Channel` or `ConnectTo`
  relays must preserve frames rather than forwarding individual messages. `Lines` and
  `ByteStreams` fields are private; construct those adapters with `new`.
  ([#275](https://github.com/agentclientprotocol/rust-sdk/pull/275),
  [#280](https://github.com/agentclientprotocol/rust-sdk/pull/280))
- JSON-RPC APIs now reflect protocol roles: notifications are never answered,
  `ResponseRouter` uses `route*` methods, typed request IDs are borrowed, and
  `TypeNotification` is role-independent. ([#272](https://github.com/agentclientprotocol/rust-sdk/pull/272),
  [#277](https://github.com/agentclientprotocol/rust-sdk/pull/277))
- Handler and routing APIs now use explicit ownership and terminology:
  `DynamicHandlerGuard` replaces cloneable registrations, background tasks use `with_runner`,
  combined matchers use `if_dispatch*`, and obsolete low-level helpers have been removed.
  ([#277](https://github.com/agentclientprotocol/rust-sdk/pull/277),
  [#280](https://github.com/agentclientprotocol/rust-sdk/pull/280),
  [#283](https://github.com/agentclientprotocol/rust-sdk/pull/283))
- MCP-over-ACP now uses the feature-gated schema-native `McpServer::Acp`, `mcp/connect`,
  `mcp/message`, and request/response `mcp/disconnect` types. The SDK-local wire types,
  `acp:` HTTP declarations, and legacy accessors are removed; MCP and session accessors now
  borrow typed identifiers and connection state. ([#281](https://github.com/agentclientprotocol/rust-sdk/pull/281))
- `AcpAgent` now uses `AcpAgentConfig`, represents JSON environment variables as an object, and
  no longer provides the deprecated Zed constructors or the Gemini convenience constructor.
  ([#276](https://github.com/agentclientprotocol/rust-sdk/pull/276),
  [#280](https://github.com/agentclientprotocol/rust-sdk/pull/280))
- Response callbacks that select ordered consumption before a response is routed during its
  original dispatch now finish before later inbound messages are dispatched. Already-routed
  responses and locally delivered failures do not add this barrier. Framework session setup is
  ordered, while user session work remains concurrent. An ordered callback must not await later
  inbound traffic on the same connection.
  ([#282](https://github.com/agentclientprotocol/rust-sdk/pull/282))

See the [2.0 migration guide] for exact API replacements and migration examples.

### Added

- Accept incoming JSON-RPC batches on stable v1 and draft v2 connections, process entries
  independently, and group replies into one response array. Typed requests and notifications
  are still sent individually. ([#271](https://github.com/agentclientprotocol/rust-sdk/pull/271),
  [#275](https://github.com/agentclientprotocol/rust-sdk/pull/275))
- Add `SentRequest::map` for transforming a response with one-shot closures without requiring the
  mapped output to implement `JsonRpcResponse`. Values consumed with `block_task` may carry
  non-`'static` lifetimes; callback-style consumption remains `'static`.
  ([#278](https://github.com/agentclientprotocol/rust-sdk/pull/278))

### Changed

- Update `agent-client-protocol-schema` to 1.5.0. The stable v1 wire schema is unchanged; the
  opt-in draft v2 API gains semantic newtypes, revised diff and terminal types, and generic
  fallible conversions. ([#273](https://github.com/agentclientprotocol/rust-sdk/pull/273))

### Fixed

- Install framework-owned session and proxy routes before later messages, route response-handler
  failures to the pending local request, and prevent handlers from emitting a second response
  after already responding. ([#280](https://github.com/agentclientprotocol/rust-sdk/pull/280),
  [#282](https://github.com/agentclientprotocol/rust-sdk/pull/282))
- Preserve batch boundaries across adapters, routers, and tracing; retain invalid call entries
  without aborting relays; do not reply to malformed response-shaped input; and complete dropped
  batched responders so sibling replies can flush.
  ([#275](https://github.com/agentclientprotocol/rust-sdk/pull/275),
  [#280](https://github.com/agentclientprotocol/rust-sdk/pull/280))
- During protocol negotiation, ignore response-only traffic before initialization, preserve
  response siblings in mixed initialization batches, flush initialization rejections before
  closing, and drop unused protocol probes before falling back to v1.
  ([#280](https://github.com/agentclientprotocol/rust-sdk/pull/280),
  [#285](https://github.com/agentclientprotocol/rust-sdk/pull/285))
- Preserve `MatchDispatchFrom` retry state across chained matchers.
  ([#274](https://github.com/agentclientprotocol/rust-sdk/pull/274))

[2.0 migration guide]: https://agentclientprotocol.github.io/rust-sdk/migration_v2.0.html

## [1.3.0](https://github.com/agentclientprotocol/rust-sdk/compare/v1.2.0...v1.3.0) - 2026-07-20

### Added

- *(unstable-v2)* Add routers for supporting both v1 and v2 at once ([#253](https://github.com/agentclientprotocol/rust-sdk/pull/253))

### Fixed

- *(acp)* preserve builder auto traits for on_close ([#269](https://github.com/agentclientprotocol/rust-sdk/pull/269))
- *(acp)* Handle  incoming EOF correctly ([#261](https://github.com/agentclientprotocol/rust-sdk/pull/261))
- *(acp)* Bound stderr capture memory ([#255](https://github.com/agentclientprotocol/rust-sdk/pull/255))
- kill the agent's whole process group when ChildGuard drops ([#251](https://github.com/agentclientprotocol/rust-sdk/pull/251))
- *(unstable-v2)* Require matching v2 protocol negotiation ([#252](https://github.com/agentclientprotocol/rust-sdk/pull/252))

## [1.2.0](https://github.com/agentclientprotocol/rust-sdk/compare/v1.1.0...v1.2.0) - 2026-07-07

### Added

- *(acp)* add constructors for renamed adapters ([#247](https://github.com/agentclientprotocol/rust-sdk/pull/247))

## [1.1.0](https://github.com/agentclientprotocol/rust-sdk/compare/v1.0.1...v1.1.0) - 2026-07-06

### Added

- *(deps)* bump schema to 1.4.0 ([#243](https://github.com/agentclientprotocol/rust-sdk/pull/243))
- *(acp)* Make request cancellation stable ([#242](https://github.com/agentclientprotocol/rust-sdk/pull/242))

## [1.0.1](https://github.com/agentclientprotocol/rust-sdk/compare/v1.0.0...v1.0.1) - 2026-06-29

### Fixed

- *(acp)* Ignore all unhandled notifications ([#229](https://github.com/agentclientprotocol/rust-sdk/pull/229))

## [1.0.0](https://github.com/agentclientprotocol/rust-sdk/compare/v0.15.1...v1.0.0) - 2026-06-24

### Added

- *(deps)* bump protocol schema to 1.1.0 ([#224](https://github.com/agentclientprotocol/rust-sdk/pull/224))

### Other

- *(acp)* Handle large future sizes in run_until ([#225](https://github.com/agentclientprotocol/rust-sdk/pull/225))

## [0.15.1](https://github.com/agentclientprotocol/rust-sdk/compare/v0.15.0...v0.15.1) - 2026-06-22

### Fixed

- *(acp)* Hide agent stdio windows on Windows ([#215](https://github.com/agentclientprotocol/rust-sdk/pull/215))

## [0.15.0](https://github.com/agentclientprotocol/rust-sdk/compare/v0.14.0...v0.15.0) - 2026-06-18

### Added

- *(deps)* update schema to 0.14.0 ([#211](https://github.com/agentclientprotocol/rust-sdk/pull/211))
- *(acp)* Update schema crate to 0.13.8 ([#210](https://github.com/agentclientprotocol/rust-sdk/pull/210))
- *(transports)* add HTTP/WebSocket transport support ([#162](https://github.com/agentclientprotocol/rust-sdk/pull/162))
- *(acp)* add unstable request cancellation support ([#179](https://github.com/agentclientprotocol/rust-sdk/pull/179))

### Other

- *(acp)* Replace jsonrpcmsg crate with shared schema types ([#205](https://github.com/agentclientprotocol/rust-sdk/pull/205))
- *(acp)* Remove unused module files ([#204](https://github.com/agentclientprotocol/rust-sdk/pull/204))
- *(deps)* Preserve serde_json object order ([#202](https://github.com/agentclientprotocol/rust-sdk/pull/202))

## [0.14.0](https://github.com/agentclientprotocol/rust-sdk/compare/v0.13.1...v0.14.0) - 2026-06-05

### Added

- Stabilize session/delete, message ids, and context usage ([#199](https://github.com/agentclientprotocol/rust-sdk/pull/199))
- *(acp)* add unstable elicitation support ([#197](https://github.com/agentclientprotocol/rust-sdk/pull/197))

### Fixed

- *(acp)* Serialize proxy metadata as _meta ([#198](https://github.com/agentclientprotocol/rust-sdk/pull/198))

### Other

- Add features to docs.rs ([#190](https://github.com/agentclientprotocol/rust-sdk/pull/190))

### Added

- *(unstable)* Add JSON-RPC support for elicitation requests and notifications.

## [0.13.1](https://github.com/agentclientprotocol/rust-sdk/compare/v0.13.0...v0.13.1) - 2026-06-01

### Added

- *(deps)* bump schema to 0.13.5 ([#188](https://github.com/agentclientprotocol/rust-sdk/pull/188))

## [0.13.0](https://github.com/agentclientprotocol/rust-sdk/compare/v0.12.1...v0.13.0) - 2026-06-01

### Added

- *(acp)* stabilize logout support ([#185](https://github.com/agentclientprotocol/rust-sdk/pull/185))
- *(acp)* Extract all rmcp logic to the rmcp crate ([#180](https://github.com/agentclientprotocol/rust-sdk/pull/180))
- *(acp)* Add unstable (very experimental!) protocol v2 support ([#170](https://github.com/agentclientprotocol/rust-sdk/pull/170))

### Changed

- Move the `rmcp`-backed MCP server builder to `agent-client-protocol-rmcp`, removing `tokio`, `tokio-util`, and `rmcp` from the core crate's normal dependency graph.

## [0.12.1](https://github.com/agentclientprotocol/rust-sdk/compare/v0.12.0...v0.12.1) - 2026-05-17

### Other

- update Cargo.toml dependencies

## [0.12.0](https://github.com/agentclientprotocol/rust-sdk/compare/v0.11.1...v0.12.0) - 2026-05-16

### Added

- *(acp)* add unstable session delete support ([#165](https://github.com/agentclientprotocol/rust-sdk/pull/165))
- extract mcp-over-acp proxy ([#146](https://github.com/agentclientprotocol/rust-sdk/pull/146))
- Stabilize session/close and session/resume ([#147](https://github.com/agentclientprotocol/rust-sdk/pull/147))
- remove direct dependency on tokio  ([#145](https://github.com/agentclientprotocol/rust-sdk/pull/145))

### Fixed

- propagate client connection errors and check capability value truthiness ([#108](https://github.com/agentclientprotocol/rust-sdk/pull/108))

### Other

- Trim dependencies ([#149](https://github.com/agentclientprotocol/rust-sdk/pull/149))
- remove unreachable!() and improve error messages ([#139](https://github.com/agentclientprotocol/rust-sdk/pull/139))

### Breaking Changes

- **Removed `McpAcpTransport`** struct and its `MetaCapability` impl. MCP-over-ACP support is now advertised via `mcpCapabilities.acp` in `InitializeResponse`, not `_meta.symposium.mcp_acp_transport`.
- **Renamed `McpConnectRequest.acp_url` to `acp_id`** to match `McpServerAcp.id` and the MCP-over-ACP RFD.

### Added

- *(unstable)* Add support for `session/delete` method.
- `McpConnectionTo::acp_id()` method.

### Deprecated

- `McpConnectionTo::acp_url()` — use `acp_id()` instead.

## [0.11.1](https://github.com/agentclientprotocol/rust-sdk/compare/v0.11.0...v0.11.1) - 2026-04-21

### Fixed

- *(acp)* remove `boxfnonce` dependency in favor of `Box<dyn FnOnce>` ([#137](https://github.com/agentclientprotocol/rust-sdk/pull/137))

## [0.11.0](https://github.com/agentclientprotocol/rust-sdk/compare/v0.10.4...v0.11.0) - 2026-04-20

### Added

- Migrate to new SDK design ([#117](https://github.com/agentclientprotocol/rust-sdk/pull/117))
- Migration Guide here: https://agentclientprotocol.github.io/rust-sdk/migration_v0.11.x.html

### Fixed

- *(rpc)* log errors when sending response to peer fails ([#101](https://github.com/agentclientprotocol/rust-sdk/pull/101))
- *(rpc)* handle write failures in handle_io loop ([#99](https://github.com/agentclientprotocol/rust-sdk/pull/99))
- *(rpc)* use RawValue::NULL constant instead of from_string().unwrap() ([#96](https://github.com/agentclientprotocol/rust-sdk/pull/96))

### Other

- Cleanup docs still referencing sacp ([#129](https://github.com/agentclientprotocol/rust-sdk/pull/129))
- Add mdbook build ([#120](https://github.com/agentclientprotocol/rust-sdk/pull/120))
- Add migration guide for next release ([#111](https://github.com/agentclientprotocol/rust-sdk/pull/111))
- remove debug code from rpc_tests ([#100](https://github.com/agentclientprotocol/rust-sdk/pull/100))
- *(test)* add conditional compilation ([#98](https://github.com/agentclientprotocol/rust-sdk/pull/98))

## [0.10.4](https://github.com/agentclientprotocol/rust-sdk/compare/v0.10.3...v0.10.4) - 2026-03-31

### Added

- *(schema)* Update schema to 0.11.4 ([#95](https://github.com/agentclientprotocol/rust-sdk/pull/95))

### Fixed

- add warning logs for silent failures in RPC message handling ([#92](https://github.com/agentclientprotocol/rust-sdk/pull/92))
- Clearer error message when connection is broken before messages are sent ([#89](https://github.com/agentclientprotocol/rust-sdk/pull/89))

### Other

- Fix the rpc_test and example use following the new schema api ([#88](https://github.com/agentclientprotocol/rust-sdk/pull/88))

## [0.10.3](https://github.com/agentclientprotocol/rust-sdk/compare/v0.10.2...v0.10.3) - 2026-03-25

### Added

- *(unstable)* Add logout support ([#84](https://github.com/agentclientprotocol/rust-sdk/pull/84))
- *(schema)* Update schema to 0.11.3 ([#82](https://github.com/agentclientprotocol/rust-sdk/pull/82))

## [0.10.2](https://github.com/agentclientprotocol/rust-sdk/compare/v0.10.1...v0.10.2) - 2026-03-11

### Added

- *(unstable)* Add support for session/close methods ([#77](https://github.com/agentclientprotocol/rust-sdk/pull/77))

## [0.10.1](https://github.com/agentclientprotocol/rust-sdk/compare/v0.10.0...v0.10.1) - 2026-03-10

### Added

- Stabilize session_list and session_info_update ([#74](https://github.com/agentclientprotocol/rust-sdk/pull/74))

### Fixed

- Make examples compile again ([#76](https://github.com/agentclientprotocol/rust-sdk/pull/76))

## [0.10.0](https://github.com/agentclientprotocol/rust-sdk/compare/v0.9.5...v0.10.0) - 2026-03-05

### Added

- Add more unstable feature flags from schema ([#71](https://github.com/agentclientprotocol/rust-sdk/pull/71))
- [**breaking**] Update to schema crate v0.11.0 ([#69](https://github.com/agentclientprotocol/rust-sdk/pull/69))

## [0.9.5](https://github.com/agentclientprotocol/rust-sdk/compare/v0.9.4...v0.9.5) - 2026-03-03

### Fixed

- handle escaped forward slashes in JSON-RPC method names ([#65](https://github.com/agentclientprotocol/rust-sdk/pull/65))

## [0.9.4](https://github.com/agentclientprotocol/rust-sdk/compare/v0.9.3...v0.9.4) - 2026-02-04

### Added

- Update to 0.10.8 of the schema ([#51](https://github.com/agentclientprotocol/rust-sdk/pull/51))

## [0.9.3](https://github.com/agentclientprotocol/rust-sdk/compare/v0.9.2...v0.9.3) - 2026-01-09

### Other

- update Cargo.toml dependencies

## [0.9.2](https://github.com/agentclientprotocol/rust-sdk/compare/v0.9.1...v0.9.2) - 2025-12-17

### Added

- *(unstable)* Add initial support for session config options ([#36](https://github.com/agentclientprotocol/rust-sdk/pull/36))

## [0.9.1](https://github.com/agentclientprotocol/rust-sdk/compare/v0.9.0...v0.9.1) - 2025-12-17

### Added

- *(unstable)* Add initial support for resuming sessions ([#34](https://github.com/agentclientprotocol/rust-sdk/pull/34))
- *(unstable)* Add initial support for forking sessions ([#33](https://github.com/agentclientprotocol/rust-sdk/pull/33))
- *(unstable)* Add initial support for listing sessions ([#31](https://github.com/agentclientprotocol/rust-sdk/pull/31))

### Other

- Add test for unstable session info feature ([#35](https://github.com/agentclientprotocol/rust-sdk/pull/35))

## [0.9.0](https://github.com/agentclientprotocol/rust-sdk/compare/v0.8.0...v0.9.0) - 2025-12-08

Update to v0.10.0 of agent-client-protocol-schema

## 0.8.0 (2025-12-01)

The types from the Rust crate, `agent-client-protocol-schema` has major breaking changes. All exported type are now marked as `#[non_exhaustive]`. Since the schema itself is JSON, and we can introduce new fields and variants in a non-breaking way, we wanted to allow for the same behavior in the Rust library.

All enum variants are also tuple variants now, with their own structs. This made it nicer to represent in the JSON Schema, and also made sure we have `_meta` fields on all variants.

This upgrade will likely come with a lot of compilation errors, but ideally upgrading will be more painless in the future.

## 0.7.0 (2025-10-24)

- Add ability for agents and clients to provide information about their implementation
- Fix incorrectly serialized `_meta` field on `SetSessionModeResponse`

## 0.6.0 (2025-10-23)

- Provide missing `_meta` fields on certain enum variants.
- More consistent enum usage. Enums are always either newtype or struct variants within a single enum, not mixed.

## 0.5.0 (2025-10-20)

- Export necessary RPC types. Fixes an issue where certain fields weren't public enough.
- Make id types easier to create and add `PartialEq` and `Eq` impls for as many types as possible.
- Export `acp::Result<T, E = acp::Error>` for easier indication of ACP errors.
- Use `acp::Error`/`acp::Result` instead of `anyhow::Error`/`anyhow::Result` for all return types.

## 0.4.7 (2025-10-13)

- Depend on `agent-client-protocol-schema` for schema types

## 0.4.6 (2025-10-10)

### Rust

- Fix: support all valid JSON-RPC ids (int, string, null)

## 0.4.5 (2025-10-02)

- No changes

## 0.4.4 (2025-09-30)

- Provide default trait implementations for optional capability-based `Agent` and `Client` methods.

## 0.4.3 (2025-09-25)

- impl `Agent` and `Client` for `Rc<T>` and `Arc<T>` where `T` implements either trait.

## 0.4.2 (2025-09-22)

**Unstable** fix missing method for model selection in Rust library.

## 0.4.1 (2025-09-22)

**Unstable** initial support for model selection.

## 0.4.0 (2025-09-17)

- Make `Agent` and `Client` dyn compatible (you'll need to annotate them with `#[async_trait]`) [#97](https://github.com/agentclientprotocol/agent-client-protocol/pull/97)
- `ext_method` and `ext_notification` methods are now more consistent with the other trait methods [#95](https://github.com/agentclientprotocol/agent-client-protocol/pull/95)
  - There are also distinct types for `ExtRequest`, `ExtResponse`, and `ExtNotification`
- Rexport `serde_json::RawValue` for easier use [#95](https://github.com/agentclientprotocol/agent-client-protocol/pull/95)
