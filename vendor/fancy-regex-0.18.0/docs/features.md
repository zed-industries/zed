# Features

This crate supports several optional features that can be enabled or disabled:

- **`std`** (enabled by default): Enables standard library support. Disable for `no_std` environments.
- **`unicode`** (enabled by default): Enables Unicode support for character classes and word boundaries.
- **`perf`** (enabled by default): Enables performance optimizations in the underlying regex engine.
- **`variable-lookbehinds`** (enabled by default): Enables support for variable-length lookbehind
  assertions (e.g., `(?<=a+)`). Without this feature, only constant-length lookbehinds are supported.
  This feature uses reverse DFA matching from the `regex-automata` crate to efficiently handle
  variable-length patterns that don't use backreferences or other fancy features.
