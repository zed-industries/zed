# Changelog

All notable changes to `jupyter-protocol` will be documented in this file.

## [Unreleased]

## [1.4.0] - 2026-02-26

### Changed

- `Header.date` now defaults to UNIX_EPOCH when missing instead of failing deserialization. Some kernels (e.g., Almond) omit this field. (#284)
- `KernelInfoReply.status` defaults to `Ok` when missing. At least 4 kernels don't set this field despite the spec requiring it. (#284)

### Fixed

- `KernelInfoReply` fields `implementation`, `implementation_version`, and `banner` now tolerate missing values via `#[serde(default)]`. Discovered via Ark kernel. (#282)
- `InspectReply.data` and `InspectReply.metadata` now handle `null` values (Ark returns `null` instead of `{}`). (#282)

## [1.3.0] - 2026-02-25

### Fixed

- Added `#[serde(default)]` to several message types where kernels may omit fields: `KernelInfoReply.help_links`, `CommInfoReply.comms`, `InputReply.value`/`status`, `InspectReply.found`/`data`/`metadata`, `CompleteReply.matches`/`cursor_start`/`cursor_end`/`metadata`. (#281)

## [1.2.2] - 2026-02-25

### Fixed

- `IsCompleteReply.indent` now has `#[serde(default)]` — kernels like ipykernel omit this field when status is not `incomplete`. (#279)

## [1.2.1] - 2026-02-20

### Fixed

- Trailing newline bug in `MultilineString` and media serialization. (#271)

## [1.2.0] - 2026-02-11

### Added

- Stdin-aware connection API and `with_channel()` builder for `JupyterMessage`. (#250)

## [1.1.1] - 2026-02-09

### Fixed

- Allow `execution_count` to be omitted by kernels. (#248)
- Handle empty parent header for sidecar compatibility. (#249)

## [1.1.0] - 2026-02-09

### Changed

- Verify messaging types against JupyterLab repo. (#195)
- Use `thiserror` for `JupyterError`. (#191)
- Add `Other` variant to `ExecutionState`. (#193)
- Add missing `starting`/`restarting` execution states. (#188)

### Fixed

- Fix deserialization of empty `parent_header`. (#185)

## [1.0.0] - 2026-01-07

### Changed

- Stable release of the Jupyter messaging protocol implementation.
- Optimized serde for media types. (#204)
- Include `Channel` in unknown messages. (#209)
- Add missing `name` and `profile` to `TabularDataResource`. (#207)
- Properly handle all JSON types with fallback for undefined types. (#206)

## [0.11.0] - 2025-12-17

Last pre-1.0 release. See git history for earlier changes.
