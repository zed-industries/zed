# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 5.11.0 - 2026-05-03

### Added
- ✨ Support nested dictionaries in *Dict derives. #312

### Fixed
- 🐛 Accept ObjectPath/Signature as map identifier keys.

## 5.10.1 - 2026-04-26

### Documentation
- 📝 Configure docs.rs to build for all supported targets.

## 5.10.0 - 2026-02-22

### Added
- ✨ Implement Basic for more types. #1681

### Changed
- 🚚 Rename an internal macro.

### Dependencies
- ⬆️ Update libfuzzer-sys to v0.4.12 (#1709).

### Fixed
- 🐛 Encode bool as single byte in GVariant.

### Testing
- ✅ Add test case for bool encoding.

## 5.9.2 - 2026-01-18

### Other
- ⏪️ Revert "🐛 zv: Don't impl Type for dicts with non-basic keys".

## 5.9.1 - 2026-01-10

### Other
- 🤖 release-plz: Fix formatting of CHANGELOG files.
- 🤖 release-plz: Use the default header in changelog.

## 5.9.0 - 2026-01-09

### Added
- ✨ Implement `TryFrom<&Value>` for tuples.
- ✨ Add signature! macro for compile-time validation. #984

### Changed
- 🎨 Format all files (rust 1.85).
- ♻️ Use signature! macro in tests.

### Dependencies
- ⬆️ Update endi to v1.1.1 (#1583).

### Fixed
- 🐛 Don't impl Type for dicts with non-basic keys. #1637

### Other
- 🧱 Fix all clippy warnings (rust 1.85).
- 🧑‍💻 Bump rust version to 1.85.
- 🚸 Implement `to_string_lossy` for `FilePath`.

### Testing
- ✅ Remove unused imports from tests.
