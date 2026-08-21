//! Benchmark for the agent's `edit_file` tool.
//!
//! This crate is split out from `benchmarks` because its harness drives the
//! tool through `TestAppContext`/`FakeFs`/`FakeLspAdapter`/`FakeLanguageModel`
//! and `Project::test`, which pull `test-support` into `agent`, `editor`,
//! `language`, `language_model`, `lsp`, `project`, and `settings`. Cargo's
//! feature unification would otherwise apply those test-only builds to every
//! other benchmark target in the same package, including the ones that render
//! production code paths. Keeping this benchmark in its own package confines
//! that contamination to the one benchmark that actually needs it.
//!
//! This is a staging step, not a destination: the long-term goal is a harness
//! with no `test-support` dependency at all. See `benches/edit_file_tool.rs`
//! for the remaining test-only APIs this benchmark still relies on.
