# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 5.11.0 - 2026-05-03

### Added
- ✨ Support nested dictionaries in *Dict derives. #312

## 5.10.1 - 2026-04-26

### Documentation
- 📝 Configure docs.rs to build for all supported targets.

## 5.10.0 - 2026-02-22

- New release to go with zvariant 5.10.0

## 5.9.1 - 2026-01-10

### Fixed
- 🐛 Fix hard-coded zvariant path in signature generation.

### Other
- 🤖 release-plz: Fix formatting of CHANGELOG files.
- 🤖 release-plz: Use the default header in changelog.

### Testing
- ✅ Add tests for `signature_to_tokens_with_crate`.

## 5.9.0 - 2026-01-09

### Added
- ✨ zvariant_derive: Add crate attribute for custom crate paths.
- ✨ Add signature! macro for compile-time validation. #984

### Changed
- 🎨 Format all files (rust 1.85).

### Fixed
- 🐛 zvariant_derive shouldn't set features on zvariant.
