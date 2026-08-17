# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.0](https://github.com/tailcallhq/gh-workflow/compare/gh-workflow-v0.7.0...gh-workflow-v0.8.0) - 2025-09-08

### Fixed

- make `event.schedule` as vector ([#170](https://github.com/tailcallhq/gh-workflow/pull/170))

## [0.7.0](https://github.com/tailcallhq/gh-workflow/compare/gh-workflow-v0.6.1...gh-workflow-v0.7.0) - 2025-09-02

### Other

- improve API ergonomics with builder pattern and method chaining ([#165](https://github.com/tailcallhq/gh-workflow/pull/165))

## [0.6.1](https://github.com/tailcallhq/gh-workflow/compare/gh-workflow-v0.6.0...gh-workflow-v0.6.1) - 2025-08-16

### Other

- *(deps)* update actions/checkout action to v5 ([#162](https://github.com/tailcallhq/gh-workflow/pull/162))

## [0.6.0](https://github.com/tailcallhq/gh-workflow/compare/gh-workflow-v0.5.10...gh-workflow-v0.6.0) - 2025-05-06

### Added

- *(event)* add tags filter to Push struct for event handling ([#157](https://github.com/tailcallhq/gh-workflow/pull/157))

## [0.5.10](https://github.com/tailcallhq/gh-workflow/compare/gh-workflow-v0.5.9...gh-workflow-v0.5.10) - 2025-01-29

### Added

- add get method to Jobs struct for retrieving jobs by key (#134)

## [0.5.9](https://github.com/tailcallhq/gh-workflow/compare/gh-workflow-v0.5.8...gh-workflow-v0.5.9) - 2025-01-19

### Fixed

- correct cargo install command in CI workflow and standard workflow

## [0.5.8](https://github.com/tailcallhq/gh-workflow/compare/gh-workflow-v0.5.7...gh-workflow-v0.5.8) - 2025-01-13

### Added

- add caching support for Cargo toolchain in CI workflows

## [0.5.7](https://github.com/tailcallhq/gh-workflow/compare/gh-workflow-v0.5.6...gh-workflow-v0.5.7) - 2024-12-29

### Fixed

- *(deps)* update rust crate serde_json to v1.0.134 (#111)

## [0.5.6](https://github.com/tailcallhq/gh-workflow/compare/gh-workflow-v0.5.5...gh-workflow-v0.5.6) - 2024-12-11

### Other

- unset release_always

## [0.5.5](https://github.com/tailcallhq/gh-workflow/compare/gh-workflow-v0.5.4...gh-workflow-v0.5.5) - 2024-12-05

### Other

- add readme

## [0.5.4](https://github.com/tailcallhq/gh-workflow/compare/gh-workflow-v0.5.3...gh-workflow-v0.5.4) - 2024-12-02

### Fixed

- jobs dependency id generator ([#94](https://github.com/tailcallhq/gh-workflow/pull/94))

## [0.5.3](https://github.com/tailcallhq/gh-workflow/compare/gh-workflow-v0.5.2...gh-workflow-v0.5.3) - 2024-11-29

### Fixed

- drop `v` prefix from `uses` API

## [0.5.2](https://github.com/tailcallhq/gh-workflow/compare/gh-workflow-v0.5.1...gh-workflow-v0.5.2) - 2024-11-29

### Other

- use gh-workflow-tailcall in the test

## [0.5.1](https://github.com/tailcallhq/gh-workflow/compare/gh-workflow-v0.5.0...gh-workflow-v0.5.1) - 2024-11-29

### Other

- *(gh-workflow-macros)* release v0.5.0 ([#86](https://github.com/tailcallhq/gh-workflow/pull/86))

## [0.5.0](https://github.com/tailcallhq/gh-workflow/compare/v0.4.1...v0.5.0) - 2024-11-28

### Other

- reset release flags
