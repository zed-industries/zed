# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 4.3.2 - 2026-04-26

### Documentation
- 📝 Configure docs.rs to build for all supported targets.

## 4.3.1 - 2026-01-09

### Fixed
- 🐛 add inherent `as_ref()` to owned types.

### Other
- 🤖 release-plz: Fix formatting of CHANGELOG files.
- 🤖 release-plz: Use the default header in changelog.

## 4.3.0 - 2026-01-09

### Added
- ✨ Implement Borrow for Owned* types.

### Changed
- ♻️ Reduce code duplication with `define_name_type_impls!` macro.
- 🎨 Format all files (rust 1.85).
- 🚚 Update name of Github space from dbus2 to z-galaxy.

### Documentation
- 📝 doc typo, Error names have same constraints as *interface* names.

### Fixed
- 🩹 Don't use workspace for local deps.

### Other
- 👽️ Use `std::hint::black_box` in benchmarks code.
- 🧑‍💻 Use workspace dependencies.

### Removed
- ➖ Drop `static_assertions` dep.
