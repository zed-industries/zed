# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2025-08-12

### Removed

- `build.rs` and inline literal parsing logic [#16](https://github.com/AS1100K/pastey/pull/16)

## [0.1.0] - 2025-03-12

### Added

- Raw Mode in `paste!` macro [#8](https://github.com/AS1100K/pastey/pull/8)

## [v0.0.1]

### Added

- `lower_camel` case conversion modifier [#4](https://github.com/AS1100K/pastey/issues/4)
- `upper_camel` case conversion modifier, similar to `camel`
- `camel_edge` case coversion modifer [#3](https://github.com/AS1100K/pastey/issues/3)
- Internal crate `paste-compat` for testing behaviour against paste crate
